use crate::default_structures::{pokemon, attacks};

pub enum Action {
    Swap (pokemon::Pokemon),
    Attack (attacks::Attack),
    Picking
}

#[derive(Copy, Clone)]
pub struct Battlemon {
    pub pokemon: pokemon::Pokemon,
    pub current_health: u32,
    pub evasion: u32,
    pub accuracy: u32,
    //effect
    //TODO add sprites, sounds, etc
}

pub struct Battle {
    own_team: [Battlemon; 6],
    enemy_team: [Battlemon; 6],
    pub p1: Battlemon,
    pub p2: Battlemon,
    pub a1: Action,
    pub a2: Action
}

impl Battle {
    pub fn new(own: [Battlemon; 6], enemy: [Battlemon; 6]) -> Battle {
        Battle {
            own_team: own,
            enemy_team: enemy,
            p1: own[0],
            p2: enemy[0],
            a1: Action::Picking,
            a2: Action::Picking
        }
    }

    pub fn pick_phase(&mut self) {
        let own_picking = std::thread::spawn(|| {
           //TODO implement picking algorithm 
           Action::Attack(attacks::dummy())
        });

        let enemy_picking = std::thread::spawn(|| {
            //TODO implement picking algorith (network or AI)
            Action::Attack(attacks::dummy())
        });

        self.a1 = own_picking.join().unwrap();
        self.a2 = enemy_picking.join().unwrap();
    }

    pub fn battle_phase(&mut self) {
        //TODO implement battle logic

        //swap 

        //attack

        //effects (burn, sleep, etc.)

        self.a1 = Action::Picking;
        self.a2 = Action::Picking;
    }

    pub fn swap_pokemon() {} //TODO implement

    pub fn attacks(attack: attacks::Attack, target: Battlemon) {
        //let multiplier; -> look in hashmap of type for pokemon types and set it accordingly

        //damage calculation
    }

    pub fn effect() {
        //TODO implement -> apply changes based on effect
    } 
}