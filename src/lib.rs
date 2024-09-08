use crate::players::{Player, PlayerClass};
use crate::armory::{Bag, Weapon};

pub mod players;
pub mod armory;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut player1 = Player::new("Player 1".to_string(), "goldengofer".to_string());
        let mut player2 = Player::new("Player 2".to_string(), "serpentmage".to_string());
        player1.attack(&mut player2);
        assert!(player2.health < 800);
    }

    #[test]
    fn test_bag() {
        let bag = Bag::new();
        let x: bool = bag.capacity <= 15;
        assert!(x);
    }

    #[test]
    fn test_weapon_iterator() {
        let mut bag = Bag::new();
        let weapon = Weapon::new("Mindtrick".to_string(), "mindtrick".to_string());
        bag.add_item(weapon);
        let mut iter = bag.weapons.iter();
        assert_eq!(iter.next().unwrap().1.name, "Mindtrick");
    }
}
