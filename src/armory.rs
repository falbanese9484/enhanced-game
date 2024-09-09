use rand::Rng;
use std::fmt;
use std::convert::TryInto;
use std::ops::Index;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Weapon {
    pub name: String,
    pub weapon_type: WeaponType,
    pub attack_bonus: i32,
    pub defense_bonus: i32,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum WeaponType {
    MindTrick,
    Concurrent,
    BruteForce,
    Hacky,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct WeaponId(pub usize);

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

impl fmt::Display for Bag {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bag: {:#?}", self.weapons)
    } 
}

impl Bag {
    pub fn new() -> Self {
        Bag {
            capacity: rand::thread_rng().gen_range(2..8),
            counter: 0,
            weapons: BTreeMap::new(),
        }
    }
    
    pub fn add_item(&mut self, item: Weapon) {
        if self.weapons.len() < self.capacity.try_into().unwrap() {
            self.weapons.insert(WeaponId(self.counter as usize), item);
            self.counter += 1;
        } else {
            println!("Cannot add item, bag is full..");
        }
    }

    pub fn get_item(&self, id: WeaponId) -> &Weapon {
        &self.weapons[&id]
    }

    pub fn remove_item(&mut self, id: WeaponId) -> Weapon {
        self.weapons.remove(&id).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weapon_creation() {
        let weapon = Weapon::new("Sword".to_string(), "MindTrick".to_string());
        assert_eq!(weapon.name, "Sword");
        assert_eq!(weapon.weapon_type, WeaponType::MindTrick);
    }

    #[test]
    fn test_bag_creation() {
        let bag = Bag::new();
        assert_eq!((bag.capacity <= 8 && bag.capacity >= 2), true);
    }

    #[test]
    fn test_add_item() {
        let mut bag = Bag::new();
        let weapon = Weapon::new("Sword".to_string(), "MindTrick".to_string());
        bag.add_item(weapon);
        assert_eq!(bag.weapons.len(), 1);
    }

    #[test]
    fn test_remove_item() {
        let mut bag = Bag::new();
        let weapon = Weapon::new("Sword".to_string(), "MindTrick".to_string());
        bag.add_item(weapon);
        let removed_weapon = bag.remove_item(WeaponId(0));
        assert_eq!(removed_weapon.name, "Sword");
    }

    #[test]
    fn test_get_item() {
        let mut bag = Bag::new();
        let weapon = Weapon::new("Sword".to_string(), "MindTrick".to_string());
        bag.add_item(weapon);
        let item = bag.get_item(WeaponId(0));
        assert_eq!(item.name, "Sword");
    }
}
