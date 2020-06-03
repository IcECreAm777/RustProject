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
    
        //flags/other stuff cuz Rust is weird
        let mut swap1: bool = false;
        let mut swap2: bool = false;
        let mut to_switch: Battlemon = Battlemon {
                                            pokemon: pokemon::dummy_pokemon(),
                                            current_health: 0,
                                            evasion: 0,
                                            accuracy: 0,
                                        };
        let mut atk1: bool = false;
        let mut atk2: bool = false;
        let mut to_atk1: attacks::Attack = attacks::dummy();
        let mut to_atk2: attacks::Attack = attacks::dummy();
        let mut dead = 0;

        //match and if swap do so immediately
        match self.a1 {
            Action::Swap (pok) => {swap1 = true; to_switch = pok;},
            Action::Attack (atk) => {atk1 = true; to_atk1 = atk;}
            _ => {},
        };
        if swap1 == true {
            Battle::swap_pokemon(&mut self.p1, &mut to_switch);
            //swap1 = false;
        }

        match self.a2 {
            Action::Swap (pok) => {swap2 = true; to_switch = pok;},
            Action::Attack (atk) => {atk2 = true; to_atk2 = atk;}
            _ => {},
        };
        if swap2 == true {
            Battle::swap_pokemon(&mut self.p2, &mut to_switch);
            //swap2 = false;
        }

        //attack

        if atk1 == true {
            Battle::attacks(&to_atk1, &mut self.p1, &mut self.p2);
            //atk1 = false;
            for i in self.own_team.iter() {         // check if either team dead
                if i.current_health == 0 {
                    dead += 1;
                }
            }
            if dead == 6 {return}
            else {dead = 0;}
            for i in self.enemy_team.iter() {
                    if i.current_health == 0 {
                        dead += 1;
                    }
            }
            if dead == 6 {return}
            else {dead = 0;}
        }

        if atk2 == true {
            Battle::attacks(&to_atk2, &mut self.p2, &mut self.p1);
            //atk2 = false;
            for i in self.own_team.iter() {         // again check if either team dead
                if i.current_health == 0 {
                    dead += 1;
                }
            }
            if dead == 6 {return}
            else {dead = 0;}
            for i in self.enemy_team.iter() {
                if i.current_health == 0 {
                    dead += 1;
                }
            }
            if dead == 6 {return}
        }

        // TODO: implement check to return if one team is completely dead after each atk

        //effects (burn, sleep, etc.)

        // TODO: again check if either team dead after effects

        self.a1 = Action::Picking;
        self.a2 = Action::Picking;
    }

    pub fn swap_pokemon(current: &mut Battlemon, pok: &Battlemon) {
        *current = *pok;
    } //TODO implement

    pub fn attacks(attack: &attacks::Attack, user: &Battlemon, target: &mut Battlemon) {
        // for now: just basic damage calculation
        // TODO effect checks like constant damage/status
        // TODO differentiate physical/special, include status etc.
        target.current_health -= 42*attack.strength*(user.pokemon.atk/(50*target.pokemon.def))+2;  //TODO F1, crit, stab, effectiveness
        // TODO check for effects after dmg calc
    }

    pub fn effect() {
        //TODO implement -> apply changes based on effect
    } 
}