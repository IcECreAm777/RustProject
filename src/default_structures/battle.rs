use crate::default_structures::{pokemon, attacks, Type, self};


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
    pub status: attacks::Status,
    pub flinch: bool,
    //TODO add sprites, sounds, etc
}

pub struct Battle {
    own_team: [Battlemon; 6],
    enemy_team: [Battlemon; 6],
    pub p1: Battlemon,
    pub p2: Battlemon,
    pub a1: Action,
    pub a2: Action,
}

impl Battle {
    pub fn new(own: [Battlemon; 6], enemy: [Battlemon; 6]) -> Battle {
        Battle {
            own_team: own,
            enemy_team: enemy,
            p1: own[0],
            p2: enemy[0],
            a1: Action::Picking,
            a2: Action::Picking,
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
                                            status: attacks::Status::None,
                                            flinch: false,
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
            Battle::swap_pokemon(&mut self.p1, &to_switch);
            //swap1 = false;
        }

        match self.a2 {
            Action::Swap (pok) => {swap2 = true; to_switch = pok;},
            Action::Attack (atk) => {atk2 = true; to_atk2 = atk;}
            _ => {},
        };
        if swap2 == true {
            Battle::swap_pokemon(&mut self.p2, &to_switch);
            //swap2 = false;
        }
        if swap1 && swap2 {return}

        //attack
        //decide who moves first
        let prio: bool;
        if to_atk1.effect_1 == attacks::Effect::Prio || to_atk1.effect_2 == attacks::Effect::Prio {
            if to_atk2.effect_2 == attacks::Effect::Prio || to_atk2.effect_2 == attacks::Effect::Prio {
                prio = self.p1.pokemon.init >= self.p2.pokemon.init;
            }
            else {prio = true;}
        }
        else {
            if to_atk2.effect_2 == attacks::Effect::Prio || to_atk2.effect_2 == attacks::Effect::Prio {prio = false;}
            else {prio = self.p1.pokemon.init >= self.p2.pokemon.init;}
        }

        if prio {       // TODO: Add freeze/paralysis/sleep check
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
                if self.p2.flinch == true {self.p2.flinch = false}
                else {
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
            }

        else {
            if atk2 == true {
                Battle::attacks(&to_atk2, &mut self.p2, &mut self.p1);
                //atk2 = false;
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

            if atk1 == true {
                if self.p1.flinch == true {self.p1.flinch = false;}
                else {
                    Battle::attacks(&to_atk1, &mut self.p1, &mut self.p2);
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
            }
        }
    }

        // TODO: implement check to return if one team is completely dead after each atk

        //effects (burn, sleep, etc.)
        match self.p1.status {
            attacks::Status::Burn | attacks::Status::Poison => self.p1.current_health -= self.p1.pokemon.health/16,
            _ => {},
        };  // Poison 1/8 in gen 2+
        // TODO: again check if either team dead after effects

        self.a1 = Action::Picking;
        self.a2 = Action::Picking;
    }

    pub fn swap_pokemon(current: &mut Battlemon, pok: &Battlemon) {
        *current = *pok;
    } //TODO implement

    pub fn attacks(attack: &attacks::Attack, user: &mut Battlemon, target: &mut Battlemon) {
        // for now: just basic damage calculation
        let mult = effective(&attack.etype, &target.pokemon.ftype)*effective(&attack.etype, &target.pokemon.stype);
        let brt: u32 = if user.status == attacks::Status::Burn && attack.atype == attacks::AttackType::Physical {2} else {1};
        // TODO effect checks like constant damage/status
        // TODO differentiate physical/special, include status etc.
        let basic: f32 = (42*attack.strength*(user.pokemon.atk/(50*target.pokemon.def))/(brt)+2) as f32;
        let stab: f32 = stab(&attack.etype, &user.pokemon);
        //let z: f32 = 100-random0bis15/100
        let damage: u32 = (basic*mult*stab) as u32;
        if damage >= target.current_health {target.current_health = 0; return}
        target.current_health -= damage;  //TODO crit, Z

        match attack.effect_1 {
            attacks::Effect::Flinch10 => if 5/(11) == 0 {target.flinch = true}, //randomness einbauen
            attacks::Effect::Flinch33 => if 5/(34) == 0 {target.flinch = true},
            _ => {},
        };

        }
        // TODO check for effects after dmg calc
    }

    pub fn effect() {
        //TODO implement -> apply changes based on effect
    } 


pub fn stab(atk_type: &Type, pok: &pokemon::Pokemon) -> f32 {
    if pok.ftype == *atk_type || pok.stype == *atk_type {
        return 1.5;
    }
    else {return 1.0;}

}

pub fn effective(type1: &Type, type2: &Type) -> f32 {
    match type1 {
        Type::Normal => {if default_structures::normap().contains_key(type2) {
                    return *default_structures::normap().get(type2).unwrap();}
                    else {return 1.0;}
            },
        Type::Fire => {if default_structures::fimap().contains_key(type2) {
                    return *default_structures::fimap().get(type2).unwrap();}
                    else {return 1.0;}
            },
        Type::Water => {if default_structures::wamap().contains_key(type2) {
                    return *default_structures::wamap().get(type2).unwrap();}
                    else {return 1.0;}
            },
        Type::Electric => {if default_structures::elmap().contains_key(type2) {
                    return *default_structures::elmap().get(type2).unwrap();}
                    else {return 1.0;}
            },
        Type::Grass => {if default_structures::gramap().contains_key(type2) {
                    return *default_structures::gramap().get(type2).unwrap();}
                    else {return 1.0;}
            },
        Type::Ice => {if default_structures::icemap().contains_key(type2) {
                    return *default_structures::icemap().get(type2).unwrap();}
                    else {return 1.0;}
            },
        Type::Fighting => {if default_structures::figmap().contains_key(type2) {
                    return *default_structures::figmap().get(type2).unwrap();}
                    else {return 1.0;}
            },
        Type::Poison => {if default_structures::poimap().contains_key(type2) {
                    return *default_structures::poimap().get(type2).unwrap();}
                    else {return 1.0;}
            },
        Type::Ground => {if default_structures::gromap().contains_key(type2) {
                    return *default_structures::gromap().get(type2).unwrap();}
                    else {return 1.0;}
            },
        Type::Flying => {if default_structures::flymap().contains_key(type2) {
                    return *default_structures::flymap().get(type2).unwrap();}
                    else {return 1.0;}
            },
        Type::Psychic => {if default_structures::psymap().contains_key(type2) {
                    return *default_structures::psymap().get(type2).unwrap();}
                    else {return 1.0;}
            },
        Type::Bug => {if default_structures::bugmap().contains_key(type2) {
                    return *default_structures::bugmap().get(type2).unwrap();}
                    else {return 1.0;}
            },
        Type::Rock => {if default_structures::rockmap().contains_key(type2) {
                    return *default_structures::rockmap().get(type2).unwrap();}
                    else {return 1.0;}
            },
        Type::Ghost => {if default_structures::ghomap().contains_key(type2) {
                    return *default_structures::ghomap().get(type2).unwrap();}
                    else {return 1.0;}
            },
        Type::Dragon => {if default_structures::dramap().contains_key(type2) {
                    return *default_structures::dramap().get(type2).unwrap();}
                    else {return 1.0;}
            },
        Type::None => return 1.0,
    };
}