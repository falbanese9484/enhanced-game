use rand::Rng;
use std::fmt::Display;
use crate::armory::Bag;

pub struct Player {
    pub name: String,
    pub power: i32,
    pub health: i32,
    pub defense: i32,
    pub class: PlayerClass,
    pub bag: Option<Bag>,
}

#[derive(Debug)]
pub enum PlayerClass {
    SerpentMage,
    GoldenGofer,
    Rustacean,
    JavaGenie
}


impl Display for Player {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
       write!(f, "Player: {} \nHealth: {} \nPower: {} \nDefense: {} \nClass: {:?}", self.name(), self.health(), self.power(), self.defense(), self.class())
   } 
}



impl From<String> for PlayerClass {
    fn from(s: String) -> Self {
        let subs = s.to_lowercase();
        match subs.as_str() {
            "serpentmage" => Self::SerpentMage,
            "goldengofer" => Self::GoldenGofer,
            "rustacean" => Self::Rustacean,
            "javagenie" => Self::JavaGenie,
            _ => panic!("Invalid Class Name"),
        }
    }
}

impl Player {
    pub fn new(name: String, class: String) -> Player {
        Player {
            name,
            health: rand::thread_rng().gen_range(500..800),
            power: rand::thread_rng().gen_range(40..80),
            defense: rand::thread_rng().gen_range(200..300),
            class: PlayerClass::from(class),
            bag: None,
        }
    }

    pub fn attack(&self, other_player: &mut Player) {
        let player1_dice: i32 = rand::random();
        let player2_dice: i32 = rand::random();
        if player2_dice > player1_dice {
            let attack_percent = other_player.defense / self.health;
            let total_attack = self.power * attack_percent;
            other_player.health -= total_attack;
            println!("Attack was defended. Total damage was {}", total_attack);
        } else {
            println!("Attacker delivered delicious blow of {}", self.power);
            other_player.health -= self.power
        }
        println!("Defending players health is {}", other_player.health);
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn health(&self) -> &i32 {
        &self.health
    }

    fn power(&self) -> &i32 {
        &self.power
    }

    fn defense(&self) -> &i32 {
        &self.defense
    }

    fn class(&self) -> String {
        match self.class {
            PlayerClass::Rustacean => "Rustacean".to_string(),
            PlayerClass::JavaGenie => "JavaGenie".to_string(),
            PlayerClass::SerpentMage => "SerpentMage".to_string(),
            PlayerClass::GoldenGofer => "GoldenGofer".to_string()
        }
    }

//   pub fn find_item(&mut self, item: String) {
//        if let Some(bag) = &mut self.bag {
//            bag.find_item(item);
//        } else {
//            println!("Player has no bag to search for items");
//        }
//    }
//
//    pub fn add_item(&mut self, item: String) {
//        if let Some(bag) = &mut self.bag {
//            bag.add_item(item);
//        } else {
//            println!("Player has no bag to add items");
//        }
//    }
//
//    pub fn remove_item(&mut self, item: String) {
//        if let Some(bag) = &mut self.bag {
//            bag.remove_item(item);
//        } else {
//            println!("Player has no bag to remove items");
//        }
//    }
}

    

