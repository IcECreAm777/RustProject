use crate::default_structures::{pokemon, attacks, Type, self};
use rand::prelude::*;
use ggez::Context;
use ggez::graphics;
use ggez::audio::{self, SoundSource};
use std::time::Duration;

#[derive(PartialEq)]
pub enum Action {
    Swap (usize),
    Attack (attacks::Attack),
    Picking,
    //None, //temporary?
}

pub struct BattleAssets {
    pub bg1: graphics::Image,
    pub healthbar: graphics::Image,
    pub healthbar2: graphics::Image,
    pub ball: graphics::Image,
    pub botbox: graphics::Image,
    pub indic: graphics::Image,
    pub brn: graphics::Image,
    pub frz: graphics::Image,
    pub par: graphics::Image,
    pub slp: graphics::Image,
    pub psn: graphics::Image,
    pub gen1: audio::Source,
    pub gen1win: audio::Source,
    pub gen3: audio::Source,
    pub gen3win: audio::Source,
    pub gen4: audio::Source,
    pub gen4win: audio::Source,
    pub select: audio::Source,
    pub pick: audio::Source,
    pub denied: audio::Source,
    pub death: audio::Source,
    pub low: audio::Source,
    pub up: audio::Source,
    pub down: audio::Source,
    pub send: audio::Source,
    pub weak: audio::Source,
    pub normal: audio::Source,
    pub strong: audio::Source,
    pub absorb: audio::Source,
}

impl BattleAssets {
    fn new(ctx: &mut Context) -> BattleAssets {
        let bg1 = graphics::Image::new(ctx, "/bg1.png");
        let health = graphics::Image::new(ctx, "/healthbar.png");
        let health2 = graphics::Image::new(ctx, "/healthbar2.png");
        let ball = graphics::Image::new(ctx, "/ball.png");
        let botbox = graphics::Image::new(ctx, "/botbox.png");
        let indic = graphics::Image::new(ctx, "/indicator.png");
        let brn = graphics::Image::new(ctx, "/stati/brn.png");
        let frz = graphics::Image::new(ctx, "/stati/frz.png");
        let par = graphics::Image::new(ctx, "/stati/par.png");
        let slp = graphics::Image::new(ctx, "/stati/slp.png");
        let psn = graphics::Image::new(ctx, "/stati/psn.png");
        let mut gen1 = audio::Source::new(ctx, "/sounds/gen1.wav").unwrap();
        gen1.set_volume(0.1);
        gen1.set_repeat(true);
        let mut gen1win = audio::Source::new(ctx, "/sounds/gen1win.wav").unwrap();
        gen1win.set_volume(0.1);
        let mut gen3 = audio::Source::new(ctx, "/sounds/gen3.wav").unwrap();
        gen3.set_volume(0.1);
        gen3.set_repeat(true);
        let mut gen3win = audio::Source::new(ctx, "/sounds/gen3win.wav").unwrap();
        gen3win.set_volume(0.1);
        let mut gen4 = audio::Source::new(ctx, "/sounds/gen4.wav").unwrap();
        gen4.set_volume(0.1);
        gen4.set_repeat(true);
        let mut gen4win = audio::Source::new(ctx, "/sounds/gen4win.wav").unwrap();
        gen4win.set_volume(0.1);
        let mut select = audio::Source::new(ctx, "/sounds/select.wav").unwrap();
        select.set_volume(0.1);
        let mut pick = audio::Source::new(ctx, "/sounds/pick.wav").unwrap();
        pick.set_volume(0.25);
        let mut denied = audio::Source::new(ctx, "/sounds/denied.wav").unwrap();
        denied.set_volume(0.25);
        let mut death = audio::Source::new(ctx, "/sounds/death.wav").unwrap();
        death.set_volume(0.25);
        let mut low = audio::Source::new(ctx, "/sounds/low.wav").unwrap();
        low.set_volume(0.25);
        let mut up = audio::Source::new(ctx, "/sounds/up.wav").unwrap();
        up.set_volume(0.25);
        let mut down = audio::Source::new(ctx, "/sounds/down.wav").unwrap();
        down.set_volume(0.25);
        let mut send = audio::Source::new(ctx, "/sounds/send.wav").unwrap();
        send.set_volume(0.25);
        let mut weak = audio::Source::new(ctx, "/sounds/weak.wav").unwrap();
        weak.set_volume(0.25);
        let mut normal = audio::Source::new(ctx, "/sounds/normal.wav").unwrap();
        normal.set_volume(0.25);
        let mut strong = audio::Source::new(ctx, "/sounds/strong.wav").unwrap();
        strong.set_volume(0.25);
        let mut absorb = audio::Source::new(ctx, "/sounds/absorb.wav").unwrap();
        absorb.set_volume(0.25);
        BattleAssets{
            bg1: bg1.unwrap(),
            healthbar: health.unwrap(),
            healthbar2: health2.unwrap(),
            ball: ball.unwrap(),
            botbox: botbox.unwrap(),
            indic: indic.unwrap(),
            brn: brn.unwrap(),
            frz: frz.unwrap(),
            par: par.unwrap(),
            slp: slp.unwrap(),
            psn: psn.unwrap(),
            gen1: gen1,
            gen1win: gen1win,
            gen3: gen3,
            gen3win: gen3win,
            gen4: gen4,
            gen4win: gen4win,
            select: select,
            pick: pick,
            denied: denied,
            death: death,
            low: low,
            send: send,
            up: up,
            down: down,
            weak: weak,
            normal: normal,
            strong: strong,
            absorb: absorb,
        }
    }
}

#[derive(Clone)]
pub struct Battlemon {
    pub pokemon: pokemon::Pokemon,
    //pub assets: PokemonAssets,
    pub stats: [i32; 7], //atk, spatk, def, spdef, init, acc, eva
    pub current_health: u32,
    pub evasion: u32,
    pub accuracy: u32,
    pub status: attacks::Status,
    pub flinch: bool,
    pub offset: u16,
    pub died: bool
}

impl Battlemon {
    //TODO delete later
    pub fn dummy(ctx: &mut Context) -> Battlemon {
        Battlemon {
            pokemon: default_structures::pokemon::dummy_pokemon(ctx),
            stats: [0; 7],
            current_health: pokemon::dummy_pokemon(ctx).health,
            evasion: 0,
            accuracy: 0,
            status: attacks::Status::None,
            flinch: false,
            offset: 0,
            died: false,
        }
    }

    pub fn new(pok: pokemon::Pokemon) -> Battlemon {
        let hp = pok.health;
        Battlemon {
            pokemon: pok,
            stats: [0; 7],
            current_health: hp,
            evasion: 0,
            accuracy: 0,
            status: attacks::Status::None,
            flinch: false,
            offset: 0,
            died: false,
        }
    }

    pub fn stat_reset(&mut self) {
        for i in 0..7 {
            self.stats[i] = 0;
        }
    }

    pub fn offset(&mut self) {
        self.offset += if self.dead() && !self.died && self.offset != 30 {1} else {0};
    }
    pub fn name(&self) -> &'static str {
        self.pokemon.name
    }

    pub fn health(&self) -> u32 {
        self.pokemon.health
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
    Pre,
    Start,
    Picking,
    PickAtk,
    PickSlot,
    Between,
    A1,
    A2,
    After1,
    After2,
    E1,
    E2,
    E3,
    E4,
    Cycle,
    SelfReplace,
    EnemyReplace,
    Fin,
    None,
}

pub struct Battle {
    pub assets: BattleAssets,
    pub theme: u8,
    pub own_team: [Battlemon; 6],
    pub enemy_team: [Battlemon; 6],
    pub p1: usize,
    pub p2: usize,
    pub a1: Action,
    pub a2: Action,
    pub text: String,
    pub textcount: usize,
    pub state: State,
    pub dmg: i32,
    pub timer: u32,
    pub user: bool,
    pub ret: State,
    pub own_sounds: [audio::Source; 6],
    pub enemy_sounds: [audio::Source; 6],
    pub own_sent: bool,
    pub enemy_sent: bool,
    pub selected: u8,
    pub e1: attacks::Effect,
    pub e2: attacks::Effect,
    pub e3: attacks::Effect,
    pub e4: attacks::Effect,
    pub finished: bool,
    pub won: bool,
}

impl Battle {
    pub fn new(own: [Battlemon; 6], enemy: [Battlemon; 6], ctx: &mut Context) -> Battle {
        let mut soundsa: [audio::Source; 6] = [
            audio::Source::from_data(ctx, own[0].pokemon.assets.battle_cry.clone()).unwrap(),
            audio::Source::from_data(ctx, own[1].pokemon.assets.battle_cry.clone()).unwrap(),
            audio::Source::from_data(ctx, own[2].pokemon.assets.battle_cry.clone()).unwrap(),
            audio::Source::from_data(ctx, own[3].pokemon.assets.battle_cry.clone()).unwrap(),
            audio::Source::from_data(ctx, own[4].pokemon.assets.battle_cry.clone()).unwrap(),
            audio::Source::from_data(ctx, own[5].pokemon.assets.battle_cry.clone()).unwrap(),
        ];
        let mut soundsb: [audio::Source; 6] = [
            audio::Source::from_data(ctx, enemy[0].pokemon.assets.battle_cry.clone()).unwrap(),
            audio::Source::from_data(ctx, enemy[1].pokemon.assets.battle_cry.clone()).unwrap(),
            audio::Source::from_data(ctx, enemy[2].pokemon.assets.battle_cry.clone()).unwrap(),
            audio::Source::from_data(ctx, enemy[3].pokemon.assets.battle_cry.clone()).unwrap(),
            audio::Source::from_data(ctx, enemy[4].pokemon.assets.battle_cry.clone()).unwrap(),
            audio::Source::from_data(ctx, enemy[5].pokemon.assets.battle_cry.clone()).unwrap(),
        ];

        for i in 0..6 {
            soundsa[i as usize].set_volume(0.25);
            soundsa[i as usize].set_fade_in(Duration::new(1,0));
            soundsb[i as usize].set_volume(0.25);
            soundsb[i as usize].set_fade_in(Duration::new(1,0));
        }
        
        Battle {
            assets: BattleAssets::new(ctx),
            theme: 4,
            own_team: own,
            enemy_team: enemy,
            p1: 0,
            p2: 0,
            a1: Action::Picking,
            a2: Action::Picking,
            text: "".to_string(),
            textcount: 0,
            state: State::Pre,
            dmg: 0,
            timer: 0,
            user: true,
            ret: State::None,
            own_sounds: soundsa,
            enemy_sounds: soundsb,
            own_sent: false,
            enemy_sent: false,
            selected: 0,
            e1: attacks::Effect::None,
            e2: attacks::Effect::None,
            e3: attacks::Effect::None,
            e4: attacks::Effect::None,
            finished: false,
            won: false,
        }
    }

    pub fn init_cries(&mut self, ctx: &mut Context) {
        let mut soundsa: [audio::Source; 6] = [
            audio::Source::from_data(ctx, self.own_team[0].pokemon.assets.battle_cry.clone()).unwrap(),
            audio::Source::from_data(ctx, self.own_team[1].pokemon.assets.battle_cry.clone()).unwrap(),
            audio::Source::from_data(ctx, self.own_team[2].pokemon.assets.battle_cry.clone()).unwrap(),
            audio::Source::from_data(ctx, self.own_team[3].pokemon.assets.battle_cry.clone()).unwrap(),
            audio::Source::from_data(ctx, self.own_team[4].pokemon.assets.battle_cry.clone()).unwrap(),
            audio::Source::from_data(ctx, self.own_team[5].pokemon.assets.battle_cry.clone()).unwrap(),
        ];
        let mut soundsb: [audio::Source; 6] = [
            audio::Source::from_data(ctx, self.enemy_team[0].pokemon.assets.battle_cry.clone()).unwrap(),
            audio::Source::from_data(ctx, self.enemy_team[1].pokemon.assets.battle_cry.clone()).unwrap(),
            audio::Source::from_data(ctx, self.enemy_team[2].pokemon.assets.battle_cry.clone()).unwrap(),
            audio::Source::from_data(ctx, self.enemy_team[3].pokemon.assets.battle_cry.clone()).unwrap(),
            audio::Source::from_data(ctx, self.enemy_team[4].pokemon.assets.battle_cry.clone()).unwrap(),
            audio::Source::from_data(ctx, self.enemy_team[5].pokemon.assets.battle_cry.clone()).unwrap(),
        ];

        for i in 0..6 {
            soundsa[i as usize].set_volume(0.25);
            soundsa[i as usize].set_fade_in(Duration::new(1,0));
            soundsb[i as usize].set_volume(0.25);
            soundsb[i as usize].set_fade_in(Duration::new(1,0));
        }

        self.own_sounds = soundsa;
        self.enemy_sounds = soundsb;
    }

    pub fn theme1(&mut self) {
        let _ = self.assets.gen1.play();
    }
    
    pub fn win1(&mut self) {
        let _ = self.assets.gen1win.play();
    }

    pub fn theme3(&mut self) {
        let _ = self.assets.gen3.play();
    }
    
    pub fn win3(&mut self) {
        let _ = self.assets.gen3win.play();
    }

    pub fn theme4(&mut self) {
        let _ = self.assets.gen4.play();
    }

    pub fn win4(&mut self) {
        let _ = self.assets.gen4win.play();
    }

    pub fn pick(&mut self) {
        let _ = self.assets.pick.play();
    }

    pub fn select(&mut self) {
        let _ = self.assets.select.play();
    }

    pub fn denied(&mut self) {
        let _ = self.assets.denied.play();
    }

    pub fn death(&mut self) {
        let _ = self.assets.death.play();
    }

    pub fn low(&mut self) {
        let _ = self.assets.low.play();
    }

    pub fn send(&mut self) {
        let _ = self.assets.send.play();
    }

    pub fn up(&mut self) {
        let _ = self.assets.up.play();
    }

    pub fn down(&mut self) {
        let _ = self.assets.down.play();
    }

    pub fn weak(&mut self) {
        let _ = self.assets.weak.play();
    }
    
    pub fn normal(&mut self) {
        let _ = self.assets.normal.play();
    }
    
    pub fn strong(&mut self) {
        let _ = self.assets.strong.play();
    }

    pub fn absorb(&mut self) {
        let _ = self.assets.absorb.play();
    }

    pub fn ret_state(&mut self) -> State {
        match self.ret {
            State::After2 => State::After2,
            State::Between => State::Between,
            State::E1 => State::E1,
            State::E2 => State::E2,
            State::E3 => State::E3,
            State::E4 => State::E4,
            State::Picking => {self.text = "What will you do?".to_string(); self.textcount = 0; State::Picking},
            _ => State::None,
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
    pub fn prio(&mut self) -> State {
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
    pub fn speed_test(&mut self) -> bool {
        (self.numerator(true,5)*self.own_team[self.p1].pokemon.init/ self.denominator(true,5)) as f32 *
        (if self.own_team[self.p1].status == attacks::Status::Paralysis {0.5} else {1.0}) >= 
        (self.numerator(false,5)*self.enemy_team[self.p2].pokemon.init/ self.denominator(false,5)) as f32 *
        (if self.enemy_team[self.p2].status == attacks::Status::Paralysis {0.5} else {1.0})
    }

    // function to find out what action needs to be performed next
    pub fn between(&self) -> State {
        if self.a1 == Action::Picking && self.a2 == Action::Picking {State::After1} 
        else {if self.a2 == Action::Picking {State::A1}
                else {State::A2}
        }
    }

    pub fn atk_init(&mut self, atk: attacks::Attack, target: bool) {
        if !target {
            if self.own_team[self.p1].flinch {
                self.text = format!("{} flinched!", self.own_team[self.p1].name());
                self.textcount = 0;
                self.timer = 90;
                return;
            }
            
            match self.own_team[self.p1].status {
                attacks::Status::Sleep(_val) => {
                    self.text = format!("{} is fast asleep!", self.own_team[self.p1].name());
                    self.textcount = 0;
                    self.timer = 90;
                    return;
                },
                attacks::Status::Freeze(_val) => {
                    self.text = format!("{} is frozen solid!", self.own_team[self.p1].name());
                    self.textcount = 0;
                    self.timer = 90;
                    return;
                }
                attacks::Status::Paralysis => {
                    if rand::thread_rng().gen_range(1,101) <= 25 {
                        self.text = format!("{} is paralysed and could not move!", self.own_team[self.p1].name());
                        self.textcount = 0;
                        self.timer = 90;
                        return;
                    }
                },
                _ => {},
            };
            //for now: just default attack, no differentiation yet
            match atk.atype {
                attacks::AttackType::Status => self.status_attack(atk, target),
                _ => self.dmg_attack(atk, target), 
            };
        }
        else {
            if self.enemy_team[self.p2].flinch {
                self.text = format!("Enemy {} flinched!", self.enemy_team[self.p2].name());
                self.textcount = 0;
                self.timer = 90;
                return;
            }
            
            match self.enemy_team[self.p2].status {
                attacks::Status::Sleep(_val) => {
                    self.text = format!("Enemy {} is fast asleep!", self.enemy_team[self.p2].name());
                    self.textcount = 0;
                    self.timer = 90;
                    return;
                },
                attacks::Status::Freeze(_val) => {
                    self.text = format!("Enemy {} is frozen solid!", self.enemy_team[self.p2].name());
                    self.textcount = 0;
                    self.timer = 90;
                    return;
                }
                attacks::Status::Paralysis => {
                    if rand::thread_rng().gen_range(1,101) <= 25 {
                        self.text = format!("Enemy {} is paralysed and could not move!", self.enemy_team[self.p2].name());
                        self.textcount = 0;
                        self.timer = 90;
                        return;
                    }
                },
                _ => {},
            };

            match atk.atype {
                attacks::AttackType::Status => self.status_attack(atk, target),
                _ => self.dmg_attack(atk, target), 
            };
        }
    }

    pub fn status_attack(&mut self, atk: attacks::Attack, target: bool) {
        let user = !target;
        let miss = rand::thread_rng().gen_range(1,101);

        if user {
            if miss <= atk.acc {self.set_effects(&atk, user);}
            self.text = format!("{} used {}!", self.own_team[self.p1].name(), atk.name());
            if miss > atk.acc {
                self.text.push_str(" It missed!");
            }
            self.textcount = 0;
        }
        else {
            if miss <= atk.acc {self.set_effects(&atk, user);}
            self.text = format!("Enemy {} used {}!", self.enemy_team[self.p2].name(), atk.name());
            if miss > atk.acc {
                self.text.push_str(" It missed!");
            }
            self.textcount = 0;
        }
        self.timer = 90;
    }

    pub fn numerator(&mut self, user: bool, slot: usize) -> i32 {
        let val: i32;
        if user {
            val = match self.own_team[self.p1].stats[slot] {
                1 => 3,
                2 => 4,
                3 => 5,
                4 => 6,
                5 => 7,
                6 => 8,
                _ => 2
            };
        }
        else {
            val = match self.enemy_team[self.p2].stats[slot] {
                1 => 3,
                2 => 4,
                3 => 5,
                4 => 6,
                5 => 7,
                6 => 8,
                _ => 2
            };
        }
        val
    }

    pub fn denominator(&mut self, user: bool, slot: usize) -> i32 {
        let val: i32;
        if user {
            val = match self.own_team[self.p1].stats[slot] {
                -1 => 3,
                -2 => 4,
                -3 => 5,
                -4 => 6,
                -5 => 7,
                -6 => 8,
                _ => 2,
            };
        }
        else {
            val = match self.enemy_team[self.p2].stats[slot] {
                -1 => 3,
                -2 => 4,
                -3 => 5,
                -4 => 6,
                -5 => 7,
                -6 => 8,
                _ => 2,
            };
        }
        val
    }

    // basic normal atk calc
    pub fn dmg_attack(&mut self, atk: attacks::Attack, target: bool) {
        let user = !(target);
        let mut _mult: f32 = 5.0;
        let miss = rand::thread_rng().gen_range(1,101);
        let crit = rand::thread_rng().gen_range(1,10001) <= 625;
        if user && miss <= atk.acc {
            _mult = effective(&atk.etype, &self.enemy_team[self.p2].pokemon.ftype)*effective(&atk.etype, &self.enemy_team[self.p2].pokemon.stype);
            let brt: f32= if self.own_team[self.p1].status == attacks::Status::Burn && atk.atype == attacks::AttackType::Physical {2.0} else {1.0};
        
            let basic: f32 = if atk.atype == attacks::AttackType::Physical 
                                    {(((42*atk.strength*(self.numerator(true,1)*self.own_team[self.p1].pokemon.atk/self.denominator(true,1)))as f32)/
                                            ((50*(self.numerator(false,3)*self.enemy_team[self.p2].pokemon.def/self.denominator(false,3)))as f32))/
                                        brt+2.0
                                    }
                             else   {(((42*atk.strength*(self.numerator(true,2)*self.own_team[self.p1].pokemon.sp_atk/self.denominator(true,2)))as f32)/
                                            ((50*(self.numerator(false,4)*self.enemy_team[self.p2].pokemon.sp_def/self.denominator(false,4)))as f32))/
                                        brt+2.0};
            let stab: f32 = stab(&atk.etype, &self.own_team[self.p1].pokemon);
            let z = rand::thread_rng().gen_range(85,101) as f32;
            let mut damage: f32 = basic*_mult*stab*(z/100.0);
            if crit {damage = damage*2.0;}
            self.dmg = damage as i32;
            self.user = false;
            self.set_effects(&atk, user);

        }
        else { if miss <= atk.acc {
            _mult = effective(&atk.etype, &self.own_team[self.p1].pokemon.ftype)*effective(&atk.etype, &self.own_team[self.p1].pokemon.stype);
            let brt: f32 = if self.enemy_team[self.p2].status == attacks::Status::Burn && atk.atype == attacks::AttackType::Physical {2.0} else {1.0};

            let basic: f32 = if atk.atype == attacks::AttackType::Physical
                                    {(((42*atk.strength*(self.numerator(false,1)*self.enemy_team[self.p2].pokemon.atk/self.denominator(false,1)))as f32)/
                                            ((50*(self.numerator(true,3)*self.own_team[self.p1].pokemon.def/self.denominator(true,3)))as f32))/
                                        brt+2.0}
                            else    {(((42*atk.strength*(self.numerator(false,2)*self.enemy_team[self.p2].pokemon.sp_atk/self.denominator(false,2)))as f32)/
                                            ((50*(self.numerator(true,4)*self.own_team[self.p1].pokemon.sp_def/self.denominator(true,4)))as f32))/
                                    brt+2.0};
            let stab: f32 = stab(&atk.etype, &self.enemy_team[self.p2].pokemon);
            let z = rand::thread_rng().gen_range(85,101) as f32;
            let mut damage: f32 = basic*_mult*stab*(z/100.0);
            if crit {damage = damage*2.0;}
            self.dmg = damage as i32;
            self.user = true;
            self.set_effects(&atk, user);
        }}

        if user {
            self.text = format!("{} used {}!", self.own_team[self.p1].name(), atk.name);
            match (_mult*4.0) as u8 {
                4 => self.normal(),
                2 | 1 => {
                    self.text.push_str(" It is not very effective!");
                    self.weak();
                },
                8 | 16 => {
                    self.text.push_str(" It is very effective!");
                    self.strong();
                },
                0 => self.text.push_str(" It has no effect!"),
                20 => self.text.push_str(" It missed!"),
                _ => {}
            }; 
            self.textcount = 0;
        }
        else {
            self.text = format!("Enemy {} used {}!", self.enemy_team[self.p2].name(), atk.name);
            match (_mult*4.0) as u8 {
                4 => self.normal(),
                2 | 1 => {
                    self.text.push_str(" It is not very effective!");
                    self.weak();
                },
                8 | 16 => {
                    self.text.push_str(" It is very effective!");
                    self.strong();
                },
                0 => self.text.push_str(" It has no effect!"),
                20 => self.text.push_str(" It missed!"),
                _ => {}
            };
            self.textcount = 0;
        }
        if crit && _mult != 0.0 {
            self.text.push_str("A critical hit!");
        }
        self.timer = self.dmg as u32 + 90;
    }

    pub fn set_effects(&mut self, atk: &attacks::Attack, user: bool) {
        if user {
            let to_set1 = match atk.effect_1 {
                attacks::Effect::Status(status, val) => 
                    if rand::thread_rng().gen_range(1,101) <= val {attacks::Effect::Status(status, val)} else {attacks::Effect::None},
                    attacks::Effect::Flinch10 => 
                    if rand::thread_rng().gen_range(1,101) <= 10 {attacks::Effect::Flinch10} else {attacks::Effect::None},
                attacks::Effect::Flinch33 => 
                    if rand::thread_rng().gen_range(1,101) <= 33 {attacks::Effect::Flinch33} else {attacks::Effect::None},
                attacks::Effect::Recoil(_) => attacks::Effect::Recoil(self.dmg/4),
                attacks::Effect::Absorb(_) => attacks::Effect::Absorb(self.dmg/2),
                attacks::Effect::StatusChange1(slot, value, chance) => {
                    if rand::thread_rng().gen_range(1,101) <= chance {attacks::Effect::StatusChange1(slot, value, chance)}
                    else {attacks::Effect::None}
                },
                _ => attacks::Effect::None,
            };
            self.e1 = to_set1;
            let to_set2 = match atk.effect_2 {
                attacks::Effect::Status(status, val) => 
                    if rand::thread_rng().gen_range(1,101) <= val {attacks::Effect::Status(status, val)} else {attacks::Effect::None},
                    attacks::Effect::Flinch10 => 
                    if rand::thread_rng().gen_range(1,101) <= 10 {attacks::Effect::Flinch10} else {attacks::Effect::None},
                attacks::Effect::Flinch33 => 
                    if rand::thread_rng().gen_range(1,101) <= 33 {attacks::Effect::Flinch33} else {attacks::Effect::None},
                attacks::Effect::StatusChange1(slot, value, chance) => {
                    if rand::thread_rng().gen_range(1,101) <= chance {attacks::Effect::StatusChange1(slot, value, chance)}
                    else {attacks::Effect::None}
                },    
                attacks::Effect::Recoil(_) => attacks::Effect::Recoil(self.dmg/4),
                attacks::Effect::Absorb(_) => attacks::Effect::Absorb(self.dmg/2),
                _ => attacks::Effect::None,
            };
            self.e2 = to_set2;
        }
        else {
            let to_set1 = match atk.effect_1 {
                attacks::Effect::Status(status, val) => 
                    if rand::thread_rng().gen_range(1,101) <= val {attacks::Effect::Status(status, val)} else {attacks::Effect::None},
                    attacks::Effect::Flinch10 => 
                    if rand::thread_rng().gen_range(1,101) <= 10 {attacks::Effect::Flinch10} else {attacks::Effect::None},
                attacks::Effect::Flinch33 => 
                    if rand::thread_rng().gen_range(1,101) <= 33 {attacks::Effect::Flinch33} else {attacks::Effect::None},
                attacks::Effect::StatusChange1(slot, value, chance) => {
                    if rand::thread_rng().gen_range(1,101) <= chance {attacks::Effect::StatusChange1(slot, value, chance)}
                    else {attacks::Effect::None}
                },    
                attacks::Effect::Recoil(_) => attacks::Effect::Recoil(self.dmg/4),
                attacks::Effect::Absorb(_) => attacks::Effect::Absorb(self.dmg/2),
                _ => attacks::Effect::None,
            };
            self.e3 = to_set1;
            let to_set2 = match atk.effect_2 {
                attacks::Effect::Status(status, val) => 
                    if rand::thread_rng().gen_range(1,101) <= val {attacks::Effect::Status(status, val)} else {attacks::Effect::None},
                    attacks::Effect::Flinch10 => 
                    if rand::thread_rng().gen_range(1,101) <= 10 {attacks::Effect::Flinch10} else {attacks::Effect::None},
                attacks::Effect::Flinch33 => 
                    if rand::thread_rng().gen_range(1,101) <= 33 {attacks::Effect::Flinch33} else {attacks::Effect::None},
                attacks::Effect::StatusChange1(slot, value, chance) => {
                    if rand::thread_rng().gen_range(1,101) <= chance {attacks::Effect::StatusChange1(slot, value, chance)}
                    else {attacks::Effect::None}
                },
                attacks::Effect::Recoil(_) => attacks::Effect::Recoil(self.dmg/4),
                attacks::Effect::Absorb(_) => attacks::Effect::Absorb(self.dmg/2),
                _ => attacks::Effect::None,
            };
            self.e4 = to_set2;

        }
    }

    pub fn apply_effect(&mut self, effect: attacks::Effect, user: bool) {
        if user {
            match effect {
                attacks::Effect::Flinch10 | attacks::Effect::Flinch33 => self.enemy_team[self.p2].flinch = true,
                attacks::Effect::Status(status, _) => {
                    if self.enemy_team[self.p2].status == attacks::Status::None {
                        self.enemy_team[self.p2].status = status;
                        match status {
                            attacks::Status::Paralysis => {
                                self.text = format!("Enemy {} was paralyzed!", self.enemy_team[self.p2].name());
                                self.textcount = 0;
                                self.timer = 90;
                            },

                            attacks::Status::Burn => {
                                self.text = format!("Enemy {} caught fire!", self.enemy_team[self.p2].name());
                                self.textcount = 0;
                                self.timer = 90;
                            },

                            attacks::Status::Sleep(_) => {
                                self.text = format!("Enemy {} fell asleep!", self.enemy_team[self.p2].name());
                                self.textcount = 0;
                                self.timer = 90;
                            },
                            attacks::Status::Freeze(_) => {
                                self.text = format!("Enemy {} froze!", self.enemy_team[self.p2].name());
                                self.textcount = 0;
                                self.timer = 90;
                            },

                            attacks::Status::Poison => {
                                self.text = format!("Enemy {} was poisoned!", self.enemy_team[self.p2].name());
                                self.textcount = 0;
                                self.timer = 90;
                            },
                            _ => {},
                        };
                    } 
                },

                attacks::Effect::StatusChange1(slot, value, _) => {
                    let stat: &str = match slot {
                        1 => "Attack",
                        2 => "Special Attack",
                        3 => "Defense",
                        4 => "Special Defense",
                        5 => "Speed",
                        6 => "Accuracy",
                        7 => "Evasion",
                        _ => "",
                    };
                    if value > 0 {
                        if self.own_team[self.p1].stats[slot] < 6 {
                            self.own_team[self.p1].stats[slot] += value;
                            if self.own_team[self.p1].stats[slot] > 6 {self.own_team[self.p1].stats[slot] = 6;}
                            let much: &str = match value {
                                1 => "rose!",
                                2 => "rose sharply!",
                                _ => "", 
                            };
                            self.text = format!("{}'s {} {}", self.own_team[self.p1].name(), stat, much);
                            self.textcount = 0;
                            self.up();
                        } 
                        else {
                            self.text = format!("{}'s {} won't go any higher!", self.own_team[self.p1].name(), stat);
                            self.textcount = 0;
                        }
                    }
                    else {
                        if self.enemy_team[self.p2].stats[slot] > -6 {
                            self.enemy_team[self.p2].stats[slot] += value;
                            if self.enemy_team[self.p2].stats[slot] < -6 {self.enemy_team[self.p2].stats[slot] = -6;}
                            let much: &str = match value {
                                -1 => "fell!",
                                -2 => "harshly fell!",
                                _ => "",
                            };
                            self.text = format!("Enemy {}'s {} {}", self.enemy_team[self.p2].name(), stat, much);
                            self.textcount = 0;
                            self.down();
                        }
                        else {
                            self.text = format!("Enemy {}'s {} won't go any lower!", self.enemy_team[self.p2].name(), stat);
                            self.textcount = 0;
                        }
                    }
                    self.timer = 120;
                },

                attacks::Effect::Recoil(val) => {
                    self.user = true;
                    self.dmg = val;
                    self.text = format!("{} was hit by recoil!", self.own_team[self.p1].name());
                    self.textcount = 0;
                    self.timer = val as u32 + 90;
                },

                attacks::Effect::Absorb(val) => {
                    self.user = true;
                    self.dmg = -val;
                    self.text = format!("{} absorbed health from its enemy!", self.own_team[self.p1].name());
                    self.textcount = 0;
                    self.absorb();
                    self.timer = val as u32 + 120;
                }
                _ => {},
            };
        }
        else {
            match effect {
                attacks::Effect::Flinch10 | attacks::Effect::Flinch33 => self.own_team[self.p1].flinch = true,
                attacks::Effect::Status(status, _) => {
                    if self.own_team[self.p1].status == attacks::Status::None {
                        self.own_team[self.p1].status = status;
                        match status {
                            attacks::Status::Paralysis => {
                                self.text = format!("{} was paralyzed!", self.own_team[self.p1].name());
                                self.textcount = 0;
                                self.timer = 90;
                            },

                            attacks::Status::Burn => {
                                self.text = format!("{} caught fire!", self.own_team[self.p1].name());
                                self.textcount = 0;
                                self.timer = 90;
                            },

                            attacks::Status::Sleep(_) => {
                                self.text = format!("{} fell asleep!", self.own_team[self.p1].name());
                                self.textcount = 0;
                                self.timer = 90;
                            },
                            attacks::Status::Freeze(_) => {
                                self.text = format!("{} froze!", self.own_team[self.p1].name());
                                self.textcount = 0;
                                self.timer = 90;
                            },

                            attacks::Status::Poison => {
                                self.text = format!("{} was poisoned!", self.own_team[self.p1].name());
                                self.textcount = 0;
                                self.timer = 90;
                            },
                            _ => {},
                        };
                    } 
                },

                attacks::Effect::StatusChange1(slot, value, _) => {
                    let stat: &str = match slot {
                        1 => "Attack",
                        2 => "Special Attack",
                        3 => "Defense",
                        4 => "Special Defense",
                        5 => "Speed",
                        6 => "Accuracy",
                        7 => "Evasion",
                        _ => "",
                    };
                    if value > 0 {
                        if self.enemy_team[self.p2].stats[slot] < 6 {
                            self.enemy_team[self.p2].stats[slot] += value;
                            if self.enemy_team[self.p2].stats[slot] > 6 {self.enemy_team[self.p2].stats[slot] = 6;}
                            let much: &str = match value {
                                1 => "rose!",
                                2 => "rose sharply!",
                                _ => "", 
                            };
                            self.text = format!("Enemy {}'s {} {}", self.enemy_team[self.p2].name(), stat, much);
                            self.textcount = 0;
                            self.timer = 90;
                            self.up();
                        } 
                        else {
                            //self.own_team[self.p1].stats[slot] = 6;
                            self.text = format!("Enemy {}'s {} won't go any higher!", self.enemy_team[self.p2].name(), stat);
                            self.textcount = 0;
                            self.timer = 90;
                        }
                    }
                    else {
                        if self.own_team[self.p1].stats[slot] > -6 {
                            self.own_team[self.p1].stats[slot] += value;
                            if self.own_team[self.p1].stats[slot] < -6 {self.own_team[self.p1].stats[slot] = -6;}
                            let much: &str = match value {
                                -1 => "fell!",
                                -2 => "harshly fell!",
                                _ => "",
                            };
                            self.text = format!("{}'s {} {}", self.own_team[self.p1].name(), stat, much);
                            self.textcount = 0;
                            self.timer = 90;
                            self.down();
                        }
                        else {
                            self.text = format!("{}'s {} won't go any lower!", self.own_team[self.p1].name(), stat);
                            self.textcount = 0;
                            self.timer = 90;
                        }
                    }
                },

                attacks::Effect::Recoil(val) => {
                    self.user = false;
                    self.dmg = val;
                    self.text = format!("Enemy {} was hit by recoil!", self.enemy_team[self.p2].name());
                    self.textcount = 0;
                    self.timer = val as u32 + 90;
                },

                attacks::Effect::Absorb(val) => {
                    self.user = false;
                    self.dmg = -val;
                    self.text = format!("Enemy {} absorbed health from your pokemon!", self.enemy_team[self.p2].name());
                    self.textcount = 0;
                    self.absorb();
                    self.timer = val as u32 + 120;
                }
                _ => {},
            };
        }
    }

    pub fn clear_effects(&mut self, user: bool) {
        if user {
            match self.e3 {
                attacks::Effect::Flinch10 | attacks::Effect::Flinch33 => self.e3 = attacks::Effect::None,
                attacks::Effect::StatusChange1(_, val, _) => if val < 0 {self.e3 = attacks::Effect::None;}
                _ => {},
            };
            match self.e4 {
                attacks::Effect::Flinch10 | attacks::Effect::Flinch33 => self.e4 = attacks::Effect::None,
                attacks::Effect::StatusChange1(_, val, _) => if val < 0 {self.e4 = attacks::Effect::None;}
                _ => {},
            }
        }

        else {
            match self.e1 {
                attacks::Effect::Flinch10 | attacks::Effect::Flinch33 => self.e1 = attacks::Effect::None,
                attacks::Effect::StatusChange1(_, val, _) => if val < 0 {self.e1 = attacks::Effect::None;}
                _ => {},
            };
            match self.e2 {
                attacks::Effect::Flinch10 | attacks::Effect::Flinch33 => self.e2 = attacks::Effect::None,
                attacks::Effect::StatusChange1(_, val, _) => if val < 0 {self.e2 = attacks::Effect::None;}
                _ => {},
            };
        }
    }

    // basic swap
    pub fn swap(&mut self, slot: usize, which: bool) {
        if which {
            self.own_team[self.p1].offset = 0;
            self.own_team[self.p1].stat_reset();
            self.p1 = slot;
        }
        else {
            self.enemy_team[self.p2].offset = 0;
            self.enemy_team[self.p2].stat_reset();
            self.p2 = slot;
        }
        self.send();
        self.timer = 120;
        if which {
            self.own_sent = true;
            self.text = format!("You sent out {}!", self.own_team[slot].name());
            self.textcount = 0;
            let _ = self.own_sounds[self.p1].play();
        }
        else {
            self.enemy_sent = true;
            self.text = format!("Opponent sent out {}!", self.enemy_team[slot].name());
            self.textcount = 0;
            let _ = self.enemy_sounds[self.p2].play();
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
        self.enemy_sent = false;
        self.swap(slot, false);
    }

    pub fn enemy_action(&mut self) {
        let random = rand::thread_rng().gen_range(0,4);
        self.a2 = Action::Attack(self.enemy_team[self.p2].pokemon.moves[random]);
        // gotta make that more elaborate probably
    }

    pub fn stat_eff(&mut self, who: bool) {
        if who {
            match self.own_team[self.p1].status {
                attacks::Status::Burn | attacks::Status::Poison => {
                    self.user = true;
                    self.dmg = (self.own_team[self.p1].pokemon.health/16) as i32;    // maybe change poison to 1/8 like in gen2+
                    self.text = format!("{} took {} damage!", self.own_team[self.p1].name(), self.own_team[self.p1].status.name());
                    self.textcount = 0;
                    self.timer = self.dmg as u32 + 90;
                },
                attacks::Status::Sleep(val) => {
                    if val == 1 {
                        self.own_team[self.p1].status = attacks::Status::None;
                        self.text = format!("{} woke up!", self.own_team[self.p1].name());
                        self.textcount = 0;
                    }
                    else {
                        self.own_team[self.p1].status = attacks::Status::Sleep(val-1);
                        self.text = format!("{} is still asleep!", self.own_team[self.p1].name());
                        self.textcount = 0;
                    }
                    self.timer = 60;
                }
                attacks::Status::Freeze(val) => {
                    if val == 1 {
                        self.own_team[self.p1].status = attacks::Status::None; 
                        self.text = format!("{} unfroze!", self.own_team[self.p1].name());
                        self.textcount = 0;
                    }
                    else {
                        self.own_team[self.p1].status = attacks::Status::Freeze(val-1);
                        self.text = format!("{} is still frozen!", self.own_team[self.p1].name());
                        self.textcount = 0;
                    }
                    self.timer = 60;
                },
                _ => {},
            };
        }
        else {
            match self.enemy_team[self.p2].status {
                attacks::Status::Burn | attacks::Status::Poison => {
                    self.user = false;
                    self.dmg = (self.enemy_team[self.p2].pokemon.health/16) as i32;  // maybe change poison to 1/8 like in gen2+
                    self.text = format!("Enemy {} took {} damage!", self.enemy_team[self.p2].name(), self.enemy_team[self.p2].status.name());
                    self.textcount = 0;
                    self.timer = self.dmg as u32 + 90;
                },
                attacks::Status::Sleep(val) => {
                    if val == 1 {
                        self.enemy_team[self.p2].status = attacks::Status::None; 
                        self.text = format!("Enemy {} woke up!", self.enemy_team[self.p2].name());
                        self.textcount = 0;
                    }
                    else {
                        self.enemy_team[self.p2].status = attacks::Status::Sleep(val-1);
                        self.text = format!("Enemy {} is still asleep!", self.enemy_team[self.p2].name());
                        self.textcount = 0;
                    }
                    self.timer = 60;
                }
                attacks::Status::Freeze(val) => {
                    if val == 1 {
                        self.enemy_team[self.p2].status = attacks::Status::None; 
                        self.text = format!("Enemy {} unfroze!", self.enemy_team[self.p2].name());
                        self.textcount = 0;
                    }
                    else {
                        self.enemy_team[self.p2].status = attacks::Status::Freeze(val-1);
                        self.text = format!("Enemy {} is still frozen!", self.enemy_team[self.p2].name());
                        self.textcount = 0;
                    }
                    self.timer = 60;
                },
                _ => {},
            };
        }
    }
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