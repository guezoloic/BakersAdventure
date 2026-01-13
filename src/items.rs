use crate::character::Character;

pub trait Item {
    fn apply(&mut self, char: &mut Character) -> ();
    fn get_name(&self) -> &'static str;
}

pub struct Weapon {
    name: &'static str,
    damage: u8,
    durability: u8,
}

// #[warn(dead_code)]
impl Weapon {
    pub fn new(name: &'static str, damage: u8, durability: u8) -> Self {
        Weapon {
            name,
            damage,
            durability,
        }
    }

    fn reduce_durability(&mut self) -> () {
        self.durability = self.durability.saturating_sub(self.damage / 2);
    }

    pub fn is_broken(&self) -> bool {
        self.durability == 0
    }
}

impl Item for Weapon {
    fn apply(&mut self, char: &mut Character) -> () {
        if !self.is_broken() {
            char.life = char.life.saturating_sub(self.damage);
            self.reduce_durability();
        }
    }

    fn get_name(&self) -> &'static str {
        self.name
    }
}

pub struct Potion {
    name: &'static str,
    heal: u8,
}

impl Potion {
    pub fn new(name: &'static str, heal: u8) -> Self {
        Potion { name, heal }
    }
}

impl Item for Potion {
    fn apply(&mut self, char: &mut Character) -> () {
        char.life += self.heal;
    }

    fn get_name(&self) -> &'static str {
        self.name
    }
}

#[cfg(test)]
mod tests {
    mod weapon {
        use super::super::*;

        #[test]
        fn build() {
            let w = Weapon::new("test", 10, 100);
            assert_eq!(w.damage, 10);
            assert_eq!(w.durability, 100);
            assert_eq!(w.get_name(), w.name);
        }
    }
}
