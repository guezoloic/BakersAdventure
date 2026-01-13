use bakersadventure::items::{Item, Weapon};

fn main() {
    let mut list: [Box<dyn Item>; 1] = [Box::new(Weapon::new("name", 1, 1))];
    
}
