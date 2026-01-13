pub struct Weapon {
    name: &'static str,
    damage: u8,
    durability: u8,
}

impl Weapon {
    pub fn new(name: &'static str, damage: u8, durability: u8) -> Self {
        Weapon {
            name,
            damage,
            durability,
        }
    }

    pub fn consume(&mut self) -> &mut Self {
        self.durability -= (self.damage / 2) as u8;
        self
    }

    fn is_broken(&self) -> bool {
        self.durability <= 0
    }

    pub fn if_not_broken<F, T>(&mut self, fun: F) -> Option<T>
    where
        F: FnOnce(u8) -> T,
    {
        if !self.is_broken() {
            Some(fun(self.damage))
        } else {
            None
        }
    }

    pub fn get_name(&self) -> &'static str {
        self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weapon_build() {
        let w = Weapon::new("test", 10, 100);
        assert_eq!(w.damage, 10);
        assert_eq!(w.durability, 100);
        assert_eq!(w.get_name(), w.name);
    }

    #[test]
    fn weapon_consumption() {
        let mut w = Weapon::new("test", 10, 100);
        w.consume();
        assert_eq!(w.durability, 95);
    }

    #[test]
    fn weapon_if_not_broken() {
        let mut w = Weapon::new("test", 10, 100);
        let val: u8 = 50;
        assert_eq!(w.if_not_broken(|d: u8| { val - d }).unwrap(), 40);
    }
}
