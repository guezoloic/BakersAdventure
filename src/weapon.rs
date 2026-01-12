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

    pub fn if_not_broken<T>(&mut self, fun: impl FnOnce(u8) -> T) -> Option<T> {
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
