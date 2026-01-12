pub struct Character {
    life: u8,
    damage: u8,
    mana: u8,
    name: &'static str,
}

impl Character {
    pub fn new(life: u8, damage: u8, mana: u8, name: &'static str) -> Self {
        Character {
            life,
            damage,
            mana,
            name,
        }
    }

    pub fn attack(&mut self, other: &mut Self) -> &mut Self {
        other.life -= self.damage;
        self
    }

    pub fn consume_mana(&mut self, consumption: u8) -> &mut Self {
        self.mana -= consumption;
        self
    }

    pub fn get_name(&self) -> &str {
        self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn character_build() {
        let c = Character::new(10, 5, 3, "test1");
        assert_eq!(c.life, 10);
        assert_eq!(c.damage, 5);
        assert_eq!(c.mana, 3);
        assert_eq!(c.name, c.get_name());
    }

    #[test]
    fn character_attack() {
        let mut c1 = Character::new(10, 3, 5, "test1");
        let mut c2 = Character::new(10, 2, 5, "test2");

        c1.attack(&mut c2);
        assert_eq!(c2.life, 7);
        Character::attack(&mut c2, &mut c1);
        assert_eq!(c1.life, 8);
    }

    #[test]
    fn character_mana() {
        let mut c = Character::new(10, 3, 15, "test1");
        c.consume_mana(10);
        assert_eq!(c.mana, 5);
    }

    #[test]
    fn character_chaining() {
        let mut c1 = Character::new(10, 3, 15, "test1");
        let mut c2 = Character::new(10, 2, 15, "test2");

        c1.attack(&mut c2).consume_mana(10).attack(&mut c2);
        assert_eq!(c2.life, 4);
        assert_eq!(c1.mana, 5);
    }
}
