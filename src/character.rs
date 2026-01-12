/// # Character
///
/// Originally, there were two python classes: `Heroes`
/// and `Monster`. However there were extremely similar
/// and not very useful, so they were merged into a
/// single struct nammed `Character`.
///
/// It consists of:
/// - `life`    as unsigned 8-bit integer
/// - `damage`  as unsigned 8-bit integer
/// - `mana`    as unsigned 8-bit integer
/// - `name`    as rodata str (string stored in
/// read-only memory)
///  
/// It also provides the following methods:
/// - `new(u8, u8, u8, &'static str)` function is used
/// to be a static constructor of `Character` that return
/// the struct initialized.
/// - `attack(&mut self, &mut self)` function removes
/// other's life based on self damage and return self
/// for method chaining.
/// - `comsume_mana(&mut self, u8)` function reduces mana
/// with the `u8` second parameter.
/// - `get_name` is only an accessor of `name`
pub struct Character {
    life: u8,
    damage: u8,
    mana: u8,
    name: &'static str,
}

impl Character {
    /// Creates a new `Character` with the given attributes.
    /// Acts as a "static constructor" returning the
    /// initialized struct.
    pub fn new(life: u8, damage: u8, mana: u8, name: &'static str) -> Self {
        Character {
            life,
            damage,
            mana,
            name,
        }
    }

    /// Attacks another character, reducing their `life` by
    /// self.damage. Returns `self` to allow method chaining.
    pub fn attack(&mut self, other: &mut Self) -> &mut Self {
        other.life -= self.damage;
        self
    }

    /// Consumes `consumption` amount of mana.
    /// Returns `self` to allow method chaining.
    pub fn consume_mana(&mut self, consumption: u8) -> &mut Self {
        self.mana -= consumption;
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
        let c = Character::new(10, 5, 3, "test1");
        assert_eq!(c.life, 10);
        assert_eq!(c.damage, 5);
        assert_eq!(c.mana, 3);
        assert_eq!(c.name, c.get_name());
    }

    #[test]
    /// Tests that attacking reduces the target's life correctly.
    fn character_attack() {
        let mut c1 = Character::new(10, 3, 5, "test1");
        let mut c2 = Character::new(10, 2, 5, "test2");

        c1.attack(&mut c2);
        assert_eq!(c2.life, 7);
        Character::attack(&mut c2, &mut c1);
        assert_eq!(c1.life, 8);
    }

    #[test]
    /// Tests that consuming mana reduces it correctly.
    fn character_mana() {
        let mut c = Character::new(10, 3, 15, "test1");
        c.consume_mana(10);
        assert_eq!(c.mana, 5);
    }

    #[test]
    /// Tests method chaining: attack -> consume mana 
    /// -> attack.
    fn character_chaining() {
        let mut c1 = Character::new(10, 3, 15, "test1");
        let mut c2 = Character::new(10, 2, 15, "test2");

        c1.attack(&mut c2).consume_mana(10).attack(&mut c2);
        assert_eq!(c2.life, 4);
        assert_eq!(c1.mana, 5);
    }
}
