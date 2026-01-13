use std::f64::consts::E;

use crate::items::Item;

/// # Character
///
/// Originally, there were two python classes: `Heroes`
/// and `Monster`. However there were extremely similar
/// and not very useful, so they were merged into a
/// single struct nammed `Character`.
///
/// It consists of:
/// - `life`    as unsigned 8-bit integer
/// - `weapon`  as an Option heap weapon struct
/// - `name`    as rodata str (string stored in
/// read-only memory)
///  
/// It also provides the following methods:
/// - `new(u8, &'static str, Option<Box<Weapon>>)`
/// function is used to be a static constructor of
/// `Character` that return the struct initialized.
/// - `new_no_weapon(u8, &'static str)` is simply the
/// same function as new, but sets weapon to None
/// - `attack(&mut self, &mut self)` function removes
/// other's life based on self damage and return self
/// for method chaining.
/// - `get_name` is only an accessor of `name`
pub struct Character {
    pub life: u8,
    name: &'static str,
    item: Option<Box<dyn Item>>,
}

impl Character {
    /// Creates a new `Character` with the given attributes.
    /// Acts as a "static constructor" returning the
    /// initialized struct.
    pub fn new(name: &'static str, life: u8, item: Option<Box<dyn Item>>) -> Self {
        Character { life, name, item }
    }

    pub fn new_no_weapon(name: &'static str, life: u8) -> Self {
        Character {
            life,
            name,
            item: None,
        }
    }

    pub fn use_item(&mut self, other: Option<&mut Character>) -> () {
        let mut item: Option<Box<dyn Item>> = self.item.take(); // pop item and replace to None
        if let Some(i) = &mut item {
            i.apply(other.unwrap_or(self));
        }
        self.item = item;
    }

    /// Returns the character's name.
    pub fn get_name(&self) -> &str {
        self.name
    }

    pub fn set_item(&mut self, item: Option<Box<dyn Item>>) -> &mut Self {
        self.item = item;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::items::{Potion, Weapon};

    #[test]
    /// Tests that a character is correctly initialized.
    fn character_build() {
        let c = Character::new_no_weapon("test", 100);
        assert_eq!(c.life, 100);
        assert_eq!(c.name, c.get_name());
    }

    #[test]
    /// Tests that attacking reduces the target's life correctly.
    fn character_item() {
        let w = Box::new(Weapon::new("testw", 10, 100));
        let p = Box::new(Potion::new("testp", 50));

        let mut c: Character = Character::new_no_weapon("testc", 100);
        c.set_item(Some(p)).use_item(None);

        assert_eq!(c.life, 150);

        c.set_item(Some(w)).use_item(None);

        assert_eq!(c.life, 140);
    }
}
