use crate::default_structures::{pokemon, attacks, Type, self};
use crate::game_assets::PokemonAssets;
use rand::prelude::*;

#[derive(PartialEq)]
pub enum Action {
    Swap (usize),
    Attack (attacks::Attack),
    Picking,
    None, //temporary?
}

#[derive(Clone)]
pub struct Battlemon {
    pub pokemon: pokemon::Pokemon,
    //pub assets: PokemonAssets,
    pub current_health: u32,
    pub evasion: u32,
    pub accuracy: u32,
    pub status: attacks::Status,
    pub flinch: bool
}

impl Battlemon {
    pub fn dummy() -> Battlemon {
        Battlemon {
            pokemon: default_structures::pokemon::dummy_pokemon(),
            current_health: pokemon::dummy_pokemon().health,
            evasion: 0,
            accuracy: 0,
            status: attacks::Status::None,
            flinch: false,
        }
    }

    pub fn name(&self) -> &'static str {
        self.pokemon.name
    }

    pub fn hp_fract(&self) -> f32 {
        (self.current_health as f32)/(self.pokemon.health as f32)
    }

    pub fn dead(&self) -> bool {
        self.current_health == 0
    }

    pub fn ftype(&self) -> default_structures::Type {
        self.pokemon.ftype
    }

    pub fn stype(&self) -> default_structures::Type {
        self.pokemon.stype
    }
}
#[derive(Clone, PartialEq)]
pub enum State {
    Picking,
    PickAtk,
    PickSlot,
    Between,
    A1,
    A2,
    After,
    E1,
    E2,
    SelfReplace,
    EnemyReplace,
    Fin,
}

pub struct Battle {
    pub own_team: [Battlemon; 6],
    pub enemy_team: [Battlemon; 6],
    pub p1: usize,
    pub p2: usize,
    pub a1: Action,
    pub a2: Action,
    pub text: String,
    pub state: State,
    pub dmg: u32,
    pub timer: u32,
    pub user: bool,
}

impl Battle {
    pub fn new(own: [Battlemon; 6], enemy: [Battlemon; 6]) -> Battle {
        Battle {
            own_team: own,
            enemy_team: enemy,
            p1: 0,
            p2: 0,
            a1: Action::Picking,
            a2: Action::Swap(2),
            text: "What will you do?".to_string(),
            state: State::Picking,
            dmg: 0,
            timer: 0,
            user: true,
        }
    }

    // check if the current chosen attack (true for own, false for enemy) has the Prio(rity) effect
    pub fn has_prio(&self, place: bool) -> bool {    // true for own_team, false for enemy_team
        if place {
            match self.a1 {
                Action::Attack(attack) => (attack.effect_1 == attacks::Effect::Prio),
                _ => false,
            };
        }
        else {
            match self.a2 {
                Action::Attack(attack) => (attack.effect_1 == attacks::Effect::Prio),
                _ => false,
            };
        }
        false
    }

    // determine upon picking of both actions, which to perform first
    pub fn prio(&self) -> State {
        match self.a1 {
            Action::Swap(_) => return State::A1,
            _ => {},
        };
        match self.a2 {
            Action::Swap(_) => return State::A2,
            _ => {},
        };
        let bool1 = self.has_prio(true);
        let bool2 = self.has_prio(false);
        let state = if bool1 == bool2 {if self.speed_test() {State::A1} else {State::A2}}
                    else {if bool1 {State::A1}
                            else {State::A2}};
        state
    }

    // function to compare the speed values of two Battlemon
    pub fn speed_test(&self) -> bool {
        self.own_team[self.p1].pokemon.init >= self.enemy_team[self.p2].pokemon.init
    }

    // function to find out what action needs to be performed next
    pub fn between(&self) -> State {
        if self.a1 == Action::Picking && self.a2 == Action::Picking {State::After} 
        else {if self.a2 == Action::Picking {State::A1}
                else {State::A2}
        }
    }

    // basic swap
    pub fn swap(&mut self, slot: usize, which: bool) {
        if which {self.p1 = slot;}
        else {self.p2 = slot;}
        self.timer = 60;
        self.text = String::new();
        if which {self.text.push_str("You sent out ");
                    self.text.push_str(self.own_team[slot].name());}
        else {self.text.push_str("Opponent sent out ");
                self.text.push_str(self.enemy_team[slot].name());}
    }

    // basic normal atk calc
    pub fn attack(&mut self, atk: attacks::Attack, target: bool) {
        let user = !(target);
        if user {
            let mult = effective(&atk.etype, &self.enemy_team[self.p2].pokemon.ftype)*effective(&atk.etype, &self.enemy_team[self.p2].pokemon.stype);
            let brt: f32= if self.own_team[self.p1].status == attacks::Status::Burn && atk.atype == attacks::AttackType::Physical {2.0} else {1.0};
        
            let basic: f32 = if atk.atype == attacks::AttackType::Physical 
                                    {(((42*atk.strength*(self.own_team[self.p1].pokemon.atk))as f32)/((50*self.enemy_team[self.p2].pokemon.def)as f32))/(brt as f32)+2.0}
                             else {((42*atk.strength*(self.own_team[self.p1].pokemon.sp_atk))as f32)/((50*self.enemy_team[self.p2].pokemon.sp_def)as f32)/(brt)+2.0};
            let stab: f32 = stab(&atk.etype, &self.own_team[self.p1].pokemon);
            let z = rand::thread_rng().gen_range(84,101) as f32;
            let damage: f32 = basic*mult*stab*(z/100.0); //TODO: Crit
            self.dmg = damage as u32;
            self.timer = damage as u32 + 30;
            self.user = false;
            }
        else {
            let mult = effective(&atk.etype, &self.own_team[self.p1].pokemon.ftype)*effective(&atk.etype, &self.own_team[self.p1].pokemon.stype);
            let brt: f32 = if self.enemy_team[self.p2].status == attacks::Status::Burn && atk.atype == attacks::AttackType::Physical {2.0} else {1.0};
        
            let basic: f32 = if atk.atype == attacks::AttackType::Physical
                                    {(((42*atk.strength*(self.enemy_team[self.p2].pokemon.atk))as f32)/((50*self.own_team[self.p1].pokemon.def)as f32))/(brt)+2.0}
                            else {((42*atk.strength*(self.enemy_team[self.p2].pokemon.sp_atk))as f32)/((50*self.own_team[self.p1].pokemon.sp_def)as f32)/(brt)+2.0};
            let stab: f32 = stab(&atk.etype, &self.enemy_team[self.p2].pokemon);
            let z = rand::thread_rng().gen_range(84,101) as f32;
            let damage: f32 = basic*mult*stab*(z/100.0); //TODO: Crit
            self.dmg = damage as u32;
            self.text = String::new();
            if user {self.text.push_str(self.own_team[self.p1].name()); self.text.push_str(" used "); self.text.push_str(atk.name); self.text.push_str("!");}
            else {self.text.push_str(self.enemy_team[self.p2].name()); self.text.push_str(" used "); self.text.push_str(atk.name); self.text.push_str("!");}
            self.timer = damage as u32 + 30;
            self.user = true;
        }


    }

    pub fn enemy_swap(&mut self) {
        let mut slot: usize = 6;
        for i in 0..6 {
            if self.enemy_team[i as usize].current_health == 0 {continue;}
            if slot == 6 {slot = i as usize; continue;}
            if 
                effective(&self.enemy_team[i as usize].ftype(), &self.own_team[self.p1].ftype())
                *effective(&self.enemy_team[i as usize].ftype(), &self.own_team[self.p1].stype())
                *effective(&self.enemy_team[i as usize].stype(), &self.own_team[self.p1].ftype())
                *effective(&self.enemy_team[i as usize].stype(), &self.own_team[self.p1].stype())
                >effective(&self.enemy_team[slot].ftype(), &self.own_team[self.p1].ftype())
                *effective(&self.enemy_team[slot].ftype(), &self.own_team[self.p1].stype())
                *effective(&self.enemy_team[slot].stype(), &self.own_team[self.p1].ftype())
                *effective(&self.enemy_team[slot].stype(), &self.own_team[self.p1].stype())
                {
                    slot = i as usize;
                }
        }
        self.swap(slot, false);
    }

    pub fn check_swap(&mut self, slot: usize) {
        if self.own_team[slot].dead() {
            self.text = String::new();
            self.text.push_str(self.own_team[slot].name());
            self.text.push_str(" has already fainted");
        } 
        else {self.swap(slot, true)}
    }

    pub fn stat_eff(&mut self, who: bool) {
        if who {
            match self.own_team[self.p1].status {
                attacks::Status::Burn | attacks::Status::Poison => {
                    self.user = true;
                    self.dmg = self.own_team[self.p1].pokemon.health/16;
                    self.text = String::new();
                    self.text.push_str(self.own_team[self.p1].name());
                    self.text.push_str(" took ");
                    self.text.push_str(self.own_team[self.p1].status.name());
                    self.text.push_str(" damage");
                    self.timer = self.dmg + 30;
                },
                _ => {},
            };
        }
        else {
            match self.enemy_team[self.p2].status {
                attacks::Status::Burn | attacks::Status::Poison => {
                    self.user = false;
                    self.dmg = self.enemy_team[self.p2].pokemon.health/16;
                    self.text = String::new();
                    self.text.push_str(self.enemy_team[self.p2].name());
                    self.text.push_str(" took ");
                    self.text.push_str(self.enemy_team[self.p2].status.name());
                    self.text.push_str(" damage");
                    self.timer = self.dmg + 30;
                },
                attacks::Status::Sleep(val) => {
                    self.enemy_team[self.p2].status = attacks::Status::Sleep(if val == 0 {0} else {val-1});
                }
                attacks::Status::Freeze(val) => {
                    self.enemy_team[self.p2].status = attacks::Status::Freeze(if val == 0 {0} else {val-1});
                }
                _ => {},
            };
        }
    }
    /*pub fn pick_phase(&mut self) {
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
        let mut to_switch: usize = 0; 
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
            Battle::swap_pokemon(&mut self.p1, &mut self.own_team[to_switch]);
            //swap1 = false;
        }

        match self.a2 {
            Action::Swap (pok) => {swap2 = true; to_switch = pok;},
            Action::Attack (atk) => {atk2 = true; to_atk2 = atk;}
            _ => {},
        };
        if swap2 == true {
            Battle::swap_pokemon(&mut self.p2, &mut self.enemy_team[to_switch]);
            //swap2 = false;
        }
        if swap1 && swap2 {return}

        //attack
        
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
                    self.p1.flinch = false;
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
                    self.p2.flinch = false;
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
            attacks::Status::Sleep(value) => {if value <= 1 {self.p1.status = attacks::Status::Sleep(0);}
                                            else {self.p1.status = attacks::Status::Sleep(value-1);}},
            attacks::Status::Freeze(value) => {if value <=1 {self.p1.status = attacks::Status::Freeze(0);}
                                            else {self.p1.status = attacks::Status::Freeze(value-1)}}, 
            _ => {},
        };  // Poison 1/8 in gen 2+

        match self.p2.status {
            attacks::Status::Burn | attacks::Status::Poison => self.p2.current_health -= self.p2.pokemon.health/16,
            attacks::Status::Sleep(value) => {if value <= 1 {self.p2.status = attacks::Status::Sleep(0);}
                                            else {self.p2.status = attacks::Status::Sleep(value-1);}},
            attacks::Status::Freeze(value) => {if value <=1 {self.p2.status = attacks::Status::Freeze(0);}
                                            else {self.p2.status = attacks::Status::Freeze(value-1)}}, 
            _ => {},
        };
        // TODO: again check if either team dead after effects
        // actually not necessarily? Battle phase just exits

        self.a1 = Action::Picking;
        self.a2 = Action::Picking;
    }

    pub fn swap_pokemon(current: &mut Battlemon, pok: &mut Battlemon) {
        //current = &pok;
        return
    } //TODO implement

    pub fn attacks(attack: &attacks::Attack, user: &mut Battlemon, target: &mut Battlemon) {
        if attack.atype != attacks::AttackType::Status {
            let mult = effective(&attack.etype, &target.pokemon.ftype)*effective(&attack.etype, &target.pokemon.stype);
            let brt: u32 = if user.status == attacks::Status::Burn && attack.atype == attacks::AttackType::Physical {2} else {1};
        // TODO effect checks like constant damage/status
            let basic: f32 = if attack.atype == attacks::AttackType::Physical {(42*attack.strength*(user.pokemon.atk/(50*target.pokemon.def))/(brt)+2) as f32}
                             else {(42*attack.strength*(user.pokemon.sp_atk/(50*target.pokemon.sp_def))/(brt)+2) as f32};
            let stab: f32 = stab(&attack.etype, &user.pokemon);
            let z = rand::thread_rng().gen_range(84,101) as f32;
            let damage: u32 = (basic*mult*stab*(z/100.0)) as u32; //TODO: Crit
            let curr = target.current_health;
            if damage >= target.current_health {target.current_health = 0;}
            else {target.current_health -= damage;}
            let done = curr - target.current_health;

            match attack.effect_1 {
                attacks::Effect::Flinch10 => if rand::thread_rng().gen_range(0,100) <= 10 {target.flinch = true}, //randomness einbauen
                attacks::Effect::Flinch33 => if rand::thread_rng().gen_range(0,100) <= 33 {target.flinch = true},
                attacks::Effect::Absorb => if user.current_health + done/2 >= user.pokemon.health {user.current_health = user.pokemon.health;}
                                           else {user.current_health += done/2;},
                attacks::Effect::Recoil => if user.current_health - done/4 <= 0 {user.current_health = 0;}
                                           else {user.current_health -= done/4;},
                _ => {},
            };
        }

        }
        // TODO check for effects after dmg calc
    */}

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
        Type::Normal => {if default_structures::normap().contains_key(&type2) {
                    return default_structures::normap().get(&type2).unwrap().clone();}
                    else {return 1.0;}
            },
        Type::Fire => {if default_structures::fimap().contains_key(&type2) {
                    return *default_structures::fimap().get(&type2).unwrap();}
                    else {return 1.0;}
            },
        Type::Water => {if default_structures::wamap().contains_key(&type2) {
                    return *default_structures::wamap().get(&type2).unwrap();}
                    else {return 1.0;}
            },
        Type::Electric => {if default_structures::elmap().contains_key(&type2) {
                    return *default_structures::elmap().get(&type2).unwrap();}
                    else {return 1.0;}
            },
        Type::Grass => {if default_structures::gramap().contains_key(&type2) {
                    return *default_structures::gramap().get(&type2).unwrap();}
                    else {return 1.0;}
            },
        Type::Ice => {if default_structures::icemap().contains_key(&type2) {
                    return *default_structures::icemap().get(&type2).unwrap();}
                    else {return 1.0;}
            },
        Type::Fighting => {if default_structures::figmap().contains_key(&type2) {
                    return *default_structures::figmap().get(&type2).unwrap();}
                    else {return 1.0;}
            },
        Type::Poison => {if default_structures::poimap().contains_key(&type2) {
                    return *default_structures::poimap().get(&type2).unwrap();}
                    else {return 1.0;}
            },
        Type::Ground => {if default_structures::gromap().contains_key(&type2) {
                    return *default_structures::gromap().get(&type2).unwrap();}
                    else {return 1.0;}
            },
        Type::Flying => {if default_structures::flymap().contains_key(&type2) {
                    return *default_structures::flymap().get(&type2).unwrap();}
                    else {return 1.0;}
            },
        Type::Psychic => {if default_structures::psymap().contains_key(&type2) {
                    return *default_structures::psymap().get(&type2).unwrap();}
                    else {return 1.0;}
            },
        Type::Bug => {if default_structures::bugmap().contains_key(&type2) {
                    return *default_structures::bugmap().get(&type2).unwrap();}
                    else {return 1.0;}
            },
        Type::Rock => {if default_structures::rockmap().contains_key(&type2) {
                    return *default_structures::rockmap().get(&type2).unwrap();}
                    else {return 1.0;}
            },
        Type::Ghost => {if default_structures::ghomap().contains_key(&type2) {
                    return *default_structures::ghomap().get(&type2).unwrap();}
                    else {return 1.0;}
            },
        Type::Dragon => {if default_structures::dramap().contains_key(&type2) {
                    return *default_structures::dramap().get(&type2).unwrap();}
                    else {return 1.0;}
            },
        Type::None => return 1.0,
    };
}