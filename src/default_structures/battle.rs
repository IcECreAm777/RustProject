use crate::default_structures::{pokemon, attacks};

pub enum Action {
    Swap (Battlemon),//(pokemon::Pokemon), // change swap to index in array
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
        let mut swap1: bool = false;
        let mut swap2: bool = false;
        let mut to_switch: Battlemon = Battlemon {
                                            pokemon: pokemon::dummy_pokemon(),
                                            current_health: 0,
                                            evasion: 0,
                                            accuracy: 0,
                                        };
        match self.a1 {
            Action::Swap (pok) => {swap1 = true; to_switch = pok;},
            _ => {},
        };
        if swap1 == true {
            Battle::swap_pokemon(&mut self.p1, &mut to_switch);
        }

        match self.a2 {
            Action::Swap (pok) => {swap2 = true; to_switch = pok;},
            _ => {},
        };
        if swap2 == true {
            Battle::swap_pokemon(&mut self.p2, &mut to_switch);
        }

        //attack

        //effects (burn, sleep, etc.)

        self.a1 = Action::Picking;
        self.a2 = Action::Picking;
    }

    pub fn swap_pokemon(current: &mut Battlemon, pok: &Battlemon) {
        *current = *pok;
    } //TODO implement

//    pub fn swap_enemy_pokemon(&mut self, pok: Battlemon) {
//        self.p2 = pok;
//    } //TODO implement

    pub fn attacks(attack: attacks::Attack, target: Battlemon) {
        //let multiplier; -> look in hashmap of type for pokemon types and set it accordingly

        //damage calculation
    }

    pub fn effect() {
        //TODO implement -> apply changes based on effect
    } 
}