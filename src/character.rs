use crate::weapon::Weapon;

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
    life: u8,
    name: &'static str,
    weapon: Option<Box<Weapon>>,
}

impl Character {
    /// Creates a new `Character` with the given attributes.
    /// Acts as a "static constructor" returning the
    /// initialized struct.
    pub fn new(life: u8, name: &'static str, weapon: Option<Box<Weapon>>) -> Self {
        Character { life, name, weapon }
    }

    pub fn new_no_weapon(life: u8, name: &'static str) -> Self {
        Character {
            life,
            name,
            weapon: None,
        }
    }

    /// Attacks another character, reducing their `life` by
    /// self.damage. Returns `self` to allow method chaining.
    pub fn attack(&mut self, other: &mut Self) -> &mut Self {
        if let Some(w) = &mut self.weapon {
            other.life = w
                .if_not_broken(|damage: u8| other.life - damage)
                .unwrap_or(0);
            w.consume();
        }
        self
    }

    /// Returns the character's name.
    pub fn get_name(&self) -> &str {
        self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Tests that a character is correctly initialized.
    fn character_build() {
        let c = Character::new_no_weapon(100, "test");
        assert_eq!(c.life, 100);
        assert_eq!(c.name, c.get_name());
    }

    #[test]
    /// Tests that attacking reduces the target's life correctly.
    fn character_attack() {
        let w = Box::new(Weapon::new("test", 10, 100));
        let mut c1 = Character::new(100, "test1", Option::Some(w));
        let mut c2 = Character::new_no_weapon(200, "test2");

        c1.attack(&mut c2);
        assert_eq!(c2.life, 190);
        Character::attack(&mut c2, &mut c1);
        assert_eq!(c1.life, 100);
    }
}
