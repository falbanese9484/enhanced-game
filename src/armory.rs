use rand::Rng;
use std::convert::TryInto;
use std::ops::Index;
use std::collections::BTreeMap;

pub struct Weapon {
    pub name: String,
    pub weapon_type: WeaponType,
    pub attack_bonus: i32,
    pub defense_bonus: i32,
}

pub enum WeaponType {
    MindTrick,
    Concurrent,
    BruteForce,
    Hacky,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct WeaponId(usize);

pub struct Bag {
    pub capacity: i32,
    counter: i32,
    pub weapons: BTreeMap<WeaponId, Weapon>
}

impl From<String> for WeaponType {
    fn from(weapon: String) -> Self {
        let w = weapon.to_lowercase();
        match w.as_str() {
            "mindtrick" => WeaponType::MindTrick,
            "concurrent" => WeaponType::Concurrent,
            "bruteforce" => WeaponType::BruteForce,
            "hacky" => WeaponType::Hacky,
            _ => panic!("Not a vaild weapon type.")
        }
    }
}

impl Index<WeaponId> for Bag {
    type Output = Weapon;
    fn index(&self, index: WeaponId) -> &Self::Output {
        self.weapons.get(&index).unwrap()
    }
}

impl Weapon {
    pub fn new(name: String, weapon_type: String) -> Weapon {
        Weapon {
            name,
            weapon_type: WeaponType::from(weapon_type),
            attack_bonus: rand::thread_rng().gen_range(0..15),
            defense_bonus: rand::thread_rng().gen_range(0..15)
        }
    }
}

impl Bag {
    pub fn new() -> Self {
        Bag {
            capacity: rand::thread_rng().gen_range(0..5),
            counter: 0,
            weapons: BTreeMap::new(),
        }
    }
    
    pub fn add_item(&mut self, item: Weapon) {
        if self.weapons.len() < self.capacity.try_into().unwrap() {
            self.weapons.insert(WeaponId(self.counter as usize), item);
            self.counter += 1;
        } else {
            panic!("Cannot add item, bag is full..");
        }
    }

    pub fn get_item(&self, id: WeaponId) -> &Weapon {
        &self.weapons[&id]
    }
}
