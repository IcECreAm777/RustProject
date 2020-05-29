use crate::default_structures::{pokemon, attacks};

pub enum Action {
    Swap (pokemon::Pokemon),
    Attack (attacks::Attack),
    Picking
}

pub struct Battle {
    ownTeam: [pokemon::Pokemon; 6],
    enemyTeam: [pokemon::Pokemon; 6],
    pub p1: pokemon::Pokemon,
    pub p2: pokemon::Pokemon,
    pub a1: Action,
    pub a2: Action
}

impl Battle {
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

        self.a1 = Action::Picking;
        self.a2 = Action::Picking;
    }

    pub fn swap_Pokemon() {} //TODO implement
    pub fn attacks() {} //TODO implement
}