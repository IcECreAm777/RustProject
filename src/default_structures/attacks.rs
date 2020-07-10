use crate::default_structures::Type;
use std::fmt::{Display, Formatter, Result};

#[derive(Copy, Clone, PartialEq, Eq, std::hash::Hash)]
pub struct Attack {
    pub name: &'static str,
    pub etype: Type,
    pub atype: AttackType,
    pub strength: u32,
    pub acc: u32,
    //ap: u8,
    pub effect_1: Effect,
    pub effect_2: Effect,
    //mirror move: Bool,
}

impl Display for Attack {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}\n{} | {}\n{:<10}{}\n{:<10}{}", self.name, self.etype, self.atype, "strength", self.strength, "acc", self.acc)
    }
}

impl Attack {
    pub fn name(&self) -> &'static str {
        self.name
    }
}

#[derive(Copy, Clone, PartialEq, Eq, std::hash::Hash)]
pub enum AttackType {
    Physical,
    Special,
    Status,
}

impl Display for AttackType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match *self {
            AttackType::Physical => write!(f, "Physical"),
            AttackType::Special => write!(f, "Special"),
            AttackType::Status => write!(f, "Status"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, std::hash::Hash)]
pub enum Status {
    None,
    Paralysis,
    Burn,
    Freeze (u8),
    Poison,
    //Bad_Poison,
    Sleep (u8),
}

impl Status {
    pub fn name(&self) -> &'static str {
        match self {
            Status::Burn => "Burn",
            Status::Poison => "Poison",
            Status::Paralysis => "Paralysis",
            Status::None => "None",
            Status::Freeze(_u8) => "Freeze",
            Status::Sleep(_u8) => "Sleep",
        }

    }
}

// if let Foo::Bar(ref mut wrapped_value) = foo {
//     *wrapped_value = 15;
// }
// foo = Foo::Bar(15)

#[derive(Copy, Clone, PartialEq, Eq, std::hash::Hash)]
pub enum StatChange1 {       // Werte mit i8/u8 sind mit einer Chance z.B. 33,2% auf Def down 1 stage ist DC(-1,33)
    Attack (i8),
    Defense (i8),
    SAttack (i8),
    SDefense (i8),
    Init (i8),
    AC (i8,u8),
    DC (i8,u8),
    SAC (i8,u8),
    SDC (i8,u8),
    IC (i8,u8),
}

#[derive(Copy, Clone, PartialEq, Eq, std::hash::Hash)]
pub enum StatChange2 {       // Werte mit i8/u8 sind mit einer Chance z.B. 33,2% auf Def down 1 stage ist DC(-1,33)
    Attack (i8),
    Defense (i8),
    SAttack (i8),
    SDefense (i8),
    Init (i8),
    AC (i8,u8),
    DC (i8,u8),
    SAC (i8,u8),
    SDC (i8,u8),
    IC (i8,u8),
}

//TODO impl stat change function in battle

//pub enum Dot {
    //TODO leech seed, etc.
//}
#[derive(Copy, Clone, PartialEq, Eq, std::hash::Hash)]
pub enum Effect {
    None,
    Prio,
    Status (Status, u8),
    StatusChange1 (StatChange1),
    StatusChange2 (StatChange2),
    Flinch10,
    Flinch33,
    Absorb(i32),
    Recoil(i32),
    //Dot (Dot)
}

pub fn dummy() -> Attack {
    Attack {
        name: "None",
        etype: Type::Normal,
        atype: AttackType::Physical,
        strength: 25,
        acc: 50,
        //ap: 0,
        effect_1: Effect::Absorb(0),
        effect_2: Effect::Recoil(0),
        //mirror move: false
    }
}

pub fn absorb() -> Attack {
    Attack {
        name: "Absorb",
        etype: Type::Grass,
        atype: AttackType::Special,
        strength: 20,
        acc: 100,
        //ap: 25,
        effect_1: Effect::Absorb(0),
        effect_2: Effect::None,
        //mirror move: True
    }
} 

pub fn acid() -> Attack {
    Attack {
        name: "Acid",
        etype: Type::Poison,
        atype: AttackType::Special,
        strength: 40,
        acc: 100,
        //ap: 30,
        effect_1: Effect::StatusChange2(StatChange2::DC(-1,33)),//effect: 33.2% chance Def um "1 stage" runter
        effect_2: Effect::None,
        //mirror move: True
    }
}

pub fn acid_armor() -> Attack {
    Attack {
        name: "Acid Armor",
        etype: Type::Poison,
        atype: AttackType::Status,
        strength: 0,
        acc: 0,
        //ap: 20,
        effect_1: Effect::StatusChange1(StatChange1::Defense(2)),//effect: Defense up 2 stages,
        effect_2: Effect::None,
        //mirror move: False,
    }
}

pub fn agility() -> Attack {
    Attack {
        name: "Agility",
        etype: Type::Psychic,
        atype: AttackType::Status,
        strength: 0,
        acc: 0,
        //ap: 30,
        effect_1: Effect::StatusChange1(StatChange1::Init(2)),//effect: Speed up 2 stages,
        effect_2: Effect::None,
        //mirror move: False,
    }
}

pub fn amnesia() -> Attack {
    Attack {
        name: "Amnesia",
        etype: Type::Psychic,
        atype: AttackType::Status,
        strength: 0,
        acc: 0,
        //ap: 20,
        effect_1: Effect::StatusChange1(StatChange1::SDefense(2)),//effect: Special Def up 2 stages,
        effect_2: Effect::None,
        //mirror move: False,
    }
}

pub fn aurora_beam() -> Attack {
    Attack {
        name: "Aurora Beam",
        etype: Type::Ice,
        atype: AttackType::Special,
        strength: 65,
        acc: 100,
        //ap: 20,
        effect_1: Effect::StatusChange2(StatChange2::AC(-1,33)),//effect: 33,2% chance opp. attack down 1 stage,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn barrage() -> Attack {
    Attack {
        name: "Barrage",
        etype: Type::Normal,
        atype: AttackType::Physical,
        strength: 15,
        acc: 85,
        //ap: 20,
        effect_1: Effect::None,//effect: Chance 2, 3 Hits: 37,5%, 4,5: 12,5%, alle konsekutiven Hits schaden so viel wie der erste,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn barrier() -> Attack {
    Attack {
        name: "Barrier",
        etype: Type::Psychic,
        atype: AttackType::Special,
        strength: 0,
        acc: 0,
        //ap: 20,
        effect_1: Effect::StatusChange1(StatChange1::Defense(2)),//effect: Defense up 2 stages,
        effect_2: Effect::None,
        //mirror move: False,
    }
}

pub fn bide() -> Attack {
    Attack {
        name: "Bide",
        etype: Type::Normal,
        atype: AttackType::Physical,
        strength: 0,
        acc: 101,
        //ap: ,
        effect_1: Effect::None,//effect: 2 oder 3 Runden (random) nichts tun (kann geswichted werden) / returned Schaden_received*2, 
        //Typ hat keinen Effekt!! trifft auch Ghost trifft IMMER auch während Dig oder Fly
        //https://bulbapedia.bulbagarden.net/wiki/Bide_(move) ,
        effect_2: Effect::None,
        //mirror move: False,
    }
}

pub fn bind() -> Attack {
    Attack {
        name: "Bind",
        etype: Type::Normal,
        atype: AttackType::Physical,
        strength: 15,
        acc: 85,
        //ap: 20,
        effect_1: Effect::None,//effect: Damage für 2-5 turns, in Gen1: kann für Duration nicht angreifen, chances wie bei Barrage
        //https://bulbapedia.bulbagarden.net/wiki/Bind_(move),
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn bite() -> Attack {
    Attack {
        name: "Bite",
        etype: Type::Normal,
        atype: AttackType::Physical,
        strength: 60,
        acc: 100,
        //ap: 25,
        effect_1: Effect::Flinch10,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn blizzard() -> Attack {
    Attack {
        name: "Blizzard",
        etype: Type::Ice,
        atype: AttackType::Special,
        strength: 110,
        acc: 90,
        //ap: 5,
        effect_1: Effect::Status(Status::Freeze(0), 10),//effect: 10% freeze chance,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn body_slam() -> Attack {
    Attack {
        name: "Body Slam",
        etype: Type::Normal,
        atype: AttackType::Physical,
        strength: 85,
        acc: 100,
        //ap: 15,
        effect_1: Effect::Status(Status::Paralysis, 30),//effect: 30% paral, kann keine normal paralysieren(??),
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn bone_club() -> Attack {
    Attack {
        name: "Bone Club",
        etype: Type::Ground,
        atype: AttackType::Physical,
        strength: 65,
        acc: 85,
        //ap: 20,
        effect_1: Effect::Flinch10,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn bonemerang() -> Attack {
    Attack {
        name: "Bonemerang",
        etype: Type::Ground,
        atype: AttackType::Physical,
        strength: 50,
        acc: 90,
        //ap: 10,
        effect_1: Effect::None,//effect: two hits, 2. Hit dealt genauso viel wie erster,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn bubble() -> Attack {
    Attack {
        name: "Bubble",
        etype: Type::Water,
        atype: AttackType::Special,
        strength: 20,
        acc: 100,
        //ap: 30,
        effect_1: Effect::StatusChange2(StatChange2::IC(-1,33)),//effect: 33,2% chance speed down one stage,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn bubble_beam() -> Attack {
    Attack {
        name: "Bubble Beam",
        etype: Type::Water,
        atype: AttackType::Special,
        strength: 65,
        acc: 100,
        //ap: 20,
        effect_1: Effect::StatusChange2(StatChange2::IC(-1,33)),//effect: 33,2% chance speed down one stage,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn clamp() -> Attack {
    Attack {
        name: "Clamp",
        etype: Type::Water,
        atype: AttackType::Physical,
        strength: 35,
        acc: 85,
        //ap: 15,
        effect_1: Effect::None,//effect: wie Bind eventuell changen?,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn comet_punch() -> Attack {
    Attack {
        name: "Comet Punch",
        etype: Type::Normal,
        atype: AttackType::Physical,
        strength: 18,
        acc: 85,
        //ap: 15,
        effect_1: Effect::None,//effect: Siehe Barrage,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn confuse_ray() -> Attack {
    Attack {
        name: "Confuse Ray",
        etype: Type::Ghost,
        atype: AttackType::Status,
        strength: 0,
        acc: 100,
        //ap: 10,
        effect_1: Effect::None,//effect: causes confusion,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn confusion() -> Attack {
    Attack {
        name: "Confusion",
        etype: Type::Psychic,
        atype: AttackType::Special,
        strength: 50,
        acc: 100,
        //ap: 25,
        effect_1: Effect::None,//effect: 10% confuse chance,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn constrict() -> Attack {
    Attack {
        name: "Constrict",
        etype: Type::Normal,
        atype: AttackType::Physical,
        strength: 10,
        acc: 100,
        //ap: 35,
        effect_1: Effect::StatusChange2(StatChange2::IC(-1,33)),//effect: 33,2% chance speed down one stage,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn conversion() -> Attack {
    Attack {
        name: "Conversion",
        etype: Type::Normal,
        atype: AttackType::Status,
        strength: 0,
        acc: 0,
        //ap: 30,
        effect_1: Effect::None,//effect: ändert Typ of self zu Typ des Targets,
        effect_2: Effect::None,
        //mirror move: False,
    }
}

pub fn counter() -> Attack {
    Attack {
        name: "Counter",
        etype: Type::Fighting,
        atype: AttackType::Physical,
        strength: 0,
        acc: 100,
        //ap: 20,
        effect_1: Effect::None,//effect: last dmg done for counter > 0 und von Normal/Fighting attack -> double dmg sonst miss, keine Typ-Effectiveness
        //decreased priority, konter auch nur letzten Hit von Moves mit mehreren Hits,
        effect_2: Effect::None,
        //mirror move: False,
    }
}

pub fn crabhammer() -> Attack {
    Attack {
        name: "Crabhammer",
        etype: Type::Water,
        atype: AttackType::Physical,
        strength: 90,
        acc: 85,
        //ap: 10,
        effect_1: Effect::None,//effect: increased crit rate,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn cut() -> Attack {
    Attack {
        name: "Cut",
        etype: Type::Normal,
        atype: AttackType::Physical,
        strength: 50,
        acc: 95,
        //ap: 30,
        effect_1: Effect::None,
        effect_2: Effect::None,
        //mirror move: 30,
    }
}

pub fn defense_curl() -> Attack {
    Attack {
        name: "Defense Curl",
        etype: Type::Normal,
        atype: AttackType::Status,
        strength: 0,
        acc: 0,
        //ap: 40,
        effect_1: Effect::StatusChange1(StatChange1::Defense(1)),//effect: Def up 1 stage,
        effect_2: Effect::None,
        //mirror move: False,
    }
}

pub fn dig() -> Attack {
    Attack {
        name: "Dig",
        etype: Type::Ground,
        atype: AttackType::Physical,
        strength: 80, //100 in gen1
        acc: 100,
        //ap: 10,
        effect_1: Effect::None,//effect: 1. Turn semi invulnerable (bis auf Bide, Swift, Transform),
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn disable() -> Attack {
    Attack {
        name: "Disable",
        etype: Type::Normal,
        atype: AttackType::Status,
        strength: 0,
        acc: 20,
        //ap: 100,
        effect_1: Effect::None,//effect: disables randomly 1 attack für 0-7 turns, -1 jede Runde,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn dizzy_punch() -> Attack {
    Attack {
        name: "Dizzy Punch",
        etype: Type::Normal,
        atype: AttackType::Physical,
        strength: 70,
        acc: 100,
        //ap: 10,
        effect_1: Effect::None,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn double_kick() -> Attack {
    Attack {
        name: "Double Kick",
        etype: Type::Fighting,
        atype: AttackType::Physical,
        strength: 30,
        acc: 100,
        //ap: 30,
        effect_1: Effect::None,//effect: hits twice, 2. as much as 1st nur 1. kann critten,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn double_slap() -> Attack {
    Attack {
        name: "Double Slap",
        etype: Type::Normal,
        atype: AttackType::Physical,
        strength: 15,
        acc: 85,
        //ap: 10,
        effect_1: Effect::None,//effect: hits 2-5 times siehe Barrage,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn double_team() -> Attack {
    Attack {
        name: "Double Team",
        etype: Type::Normal,
        atype: AttackType::Status,
        strength: 0,
        acc: 100,
        //ap: 15,
        effect_1: Effect::None,//effect: erhöht evasion (Muss nnoch geadded werden?)
        effect_2: Effect::None,
        //mirror move: False
    }
}

pub fn double_edge() -> Attack {
    Attack {
        name: "Double Edge",
        etype: Type::Normal,
        atype: AttackType::Physical,
        strength: 120, //100 in gen1
        acc: 100,
        //ap: 15,
        effect_1: Effect::Recoil(0),//1/4th
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn dragon_rage() -> Attack {
    Attack {
        name: "Dragon Rage",
        etype: Type::Dragon,
        atype: AttackType::Special,
        strength: 0,
        acc: 100,
        //ap: 10,
        effect_1: Effect::None,//effect: genau 40HP dmg keine Weaknesses/Resistances,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn dream_eater() -> Attack {
    Attack {
        name: "Dream Eater",
        etype: Type::Psychic,
        atype: AttackType::Special,
        strength: 100,
        acc: 100,
        //ap: 15,
        effect_1: Effect::None,//effect: nur if target asleep, heals 50% of dmg dealt, nothing if target not asleep,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn drill_peck() -> Attack {
    Attack {
        name: "Drill Peck",
        etype: Type::Flying,
        atype: AttackType::Physical,
        strength: 80,
        acc: 100,
        //ap: 2ß,
        effect_1: Effect::None,//effect: none,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn earthquake() -> Attack {
    Attack {
        name: "Earthquake",
        etype: Type::Ground,
        atype: AttackType::Physical,
        strength: 100,
        acc: 100,
        //ap: 10,
        effect_1: Effect::None,//effect: none, maybe include hitting dig? if so power *2
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn egg_bomb() -> Attack {
    Attack {
        name: "Egg Bomb",
        etype: Type::Normal,
        atype: AttackType::Physical,
        strength: 100,
        acc: 75,
        //ap: 10,
        effect_1: Effect::None,//effect: none,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn ember() -> Attack {
    Attack {
        name: "Ember",
        etype: Type::Fire,
        atype: AttackType::Special,
        strength: 40,
        acc: 100,
        //ap: 25,
        effect_1: Effect::Status(Status::Burn, 10),//effect: 10% burn chance,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn explosion() -> Attack {
    Attack {
        name: "Explosion",
        etype: Type::Normal,
        atype: AttackType::Physical,
        strength: 250,  // gen 1 170 (theoretisch mal 2)
        acc: 100,
        //ap: 5,
        effect_1: Effect::None,//effect: rip user (gen 1 halves target defense during calc),
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn fire_blast() -> Attack {
    Attack {
        name: "Fire Blast",
        etype: Type::Fire,
        atype: AttackType::Special,
        strength: 110,
        acc: 85,
        //ap: 5,
        effect_1: Effect::Status(Status::Burn, 30),//effect: 30% burn chance,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn fire_punch() -> Attack {
    Attack {
        name: "Fire Punch",
        etype: Type::Fire,
        atype: AttackType::Physical,
        strength: 75,
        acc: 100,
        //ap: 15,
        effect_1: Effect::Status(Status::Burn, 10),//effect: 10% burn chance,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn fire_spin() -> Attack {
    Attack {
        name: "Fire Spin",
        etype: Type::Fire,
        atype: AttackType::Special,
        strength: 35, // gen 1 15
        acc: 85, // gen 1 75
        //ap: 15,
        effect_1: Effect::None,//effect: Wie Bind etc,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn fissure() -> Attack {
    Attack {
        name: "Fissure",
        etype: Type::Ground,
        atype: AttackType::Physical,
        strength: 0,
        acc: 30,
        //ap: 5,
        effect_1: Effect::None,//effect: INSTAKILL POGGERS (gen 1: wont hit target with higher speed),
        effect_2: Effect::None,
        //mirror move: False,
    }
}

pub fn flamethrower() -> Attack {
    Attack {
        name: "Flamethrower",
        etype: Type::Fire,
        atype: AttackType::Special,
        strength: 90,
        acc: 100,
        //ap: 15,
        effect_1: Effect::Status(Status::Burn, 10),//effect: 10% burn chance,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn flash() -> Attack {
    Attack {
        name: "Flash",
        etype: Type::Normal,
        atype: AttackType::Status,
        strength: 0,
        acc: 100, // gen 1 70
        //ap: 20,
        effect_1: Effect::None, //effect: target acc one stage down,
        effect_2: Effect::None,
        //mirror move: False,
    }
}

pub fn fly() -> Attack {
    Attack {
        name: "Fly",
        etype: Type::Flying,
        atype: AttackType::Physical,
        strength: 90,
        acc: 95,
        //ap: 15,
        effect_1: Effect::None,//effect: wie Dig kann auch von paraly./confusion disrupted werden etc (maybe also include hit by thunder?),
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn focus_energy() -> Attack {
    Attack {
        name: "Focus Energy",
        etype: Type::Normal,
        atype: AttackType::Status,
        strength: 0,
        acc: 0,
        //ap: 30,
        effect_1: Effect::None,//effect: Gen 1 bug: statt *4 /4 crit rate ((andere Formel in Stadium)),
        effect_2: Effect::None,
        //mirror move: False,
    }
}

pub fn fury_attack() -> Attack {
    Attack {
        name: "Fury Attack",
        etype: Type::Normal,
        atype: AttackType::Physical,
        strength: 15,
        acc: 85,
        //ap: 20,
        effect_1: Effect::None,//effect: wie barrage,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn fury_swipes() -> Attack {
    Attack {
        name: "Fury Swipes",
        etype: Type::Normal,
        atype: AttackType::Physical,
        strength: 18,
        acc: 80,
        //ap: 15,
        effect_1: Effect::None,//effect: wie barrage,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn glare() -> Attack {
    Attack {
        name: "Glare",
        etype: Type::Normal,
        atype: AttackType::Status,
        strength: 0,
        acc: 75, // gen 1 75, gen 5 90, gen 6+ 100
        //ap: 30,
        effect_1: Effect::Status(Status::Paralysis, 100),//effect: paralysis (gen 1 can hit ghost),
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn growl() -> Attack {
    Attack {
        name: "Growl",
        etype: Type::Normal,
        atype: AttackType::Status,
        strength: 0,
        acc: 100,
        //ap: 40,
        effect_1: Effect::StatusChange2(StatChange2::Attack(-1)),//effect: attack down 1 stage,
        effect_2: Effect::None,
        //mirror move: False,
    }
}

pub fn growth() -> Attack {
    Attack {
        name: "Growth",
        etype: Type::Normal,
        atype: AttackType::Status,
        strength: 0,
        acc: 0,
        //ap: 20,
        effect_1: Effect::StatusChange1(StatChange1::SAttack(1)),//effect: special atk up 1 stage,
        effect_2: Effect::None,
        //mirror move: False,
    }
}

pub fn guillotine() -> Attack {
    Attack {
        name: "Guillotine",
        etype: Type::Normal,
        atype: AttackType::Physical,
        strength: 0,
        acc: 30,
        //ap: 5,
        effect_1: Effect::None,//effect: OHK (again no hit if target init > in gen 1),
        effect_2: Effect::None,
        //mirror move: False,
    }
}

pub fn gust() -> Attack {
    Attack {
        name: "Gust",
        etype: Type::Flying, // normal in gen 1
        atype: AttackType::Special,
        strength: 40,
        acc: 100,
        //ap: 35,
        effect_1: Effect::None,//effect: none (gen 2: can hit fly),
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn harden() -> Attack {
    Attack {
        name: "Harden",
        etype: Type::Normal,
        atype: AttackType::Status,
        strength: 0,
        acc: 0,
        //ap: 30,
        effect_1: Effect::StatusChange1(StatChange1::Defense(1)),//effect: def up 1 stage,
        effect_2: Effect::None,
        //mirror move: False,
    }
}

pub fn haze() -> Attack {
    Attack {
        name: "Haze",
        etype: Type::Ice,
        atype: AttackType::Status,
        strength: 0,
        acc: 0,
        //ap: 30,
        effect_1: Effect::None,//effect: reset stat stages to 0 remove stat reductions from burn and paral, - focus energy
        //cures confusion, bad poison -> regular poison, -non-volatile status -> burn, paral,...,
        effect_2: Effect::None,
        //mirror move: False,
    }
}

pub fn headbutt() -> Attack {
    Attack {
        name: "Headbutt",
        etype: Type::Normal,
        atype: AttackType::Physical,
        strength: 70,
        acc: 100,
        //ap: 15,
        effect_1: Effect::Flinch33,//effect: 30% flinch,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn high_jump() -> Attack {
    Attack {
        name: "High Jump",
        etype: Type::Fighting,
        atype: AttackType::Physical,
        strength: 130, //gen 1 85
        acc: 90,
        //ap: 10,
        effect_1: Effect::None,//effect: if miss: 1hp dmg,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn horn_attack() -> Attack {
    Attack {
        name: "Horn Attack",
        etype: Type::Normal,
        atype: AttackType::Physical,
        strength: 65,
        acc: 100,
        //ap: 25,
        effect_1: Effect::None,//effect: none,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn horn_drill() -> Attack {
    Attack {
        name: "Horn Drill",
        etype: Type::Normal,
        atype: AttackType::Physical,
        strength: 0,
        acc: 30,
        //ap: 5,
        effect_1: Effect::None,//effect: OHK again if init > oof,
        effect_2: Effect::None,
        //mirror move: False,
    }
}

pub fn hydro_pump() -> Attack {
    Attack {
        name: "Hydro Pump",
        etype: Type::Water,
        atype: AttackType::Special,
        strength: 110,
        acc: 80,
        //ap: 5,
        effect_1: Effect::None,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn hyper_beam() -> Attack {
    Attack {
        name: "Hyper Beam",
        etype: Type::Normal,
        atype: AttackType::Special,
        strength: 150,
        acc: 90,
        //ap: 5,
        effect_1: Effect::None,//effect: recharge turn (not if miss in gen 1) ist quasi aktiv,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn hyper_fang() -> Attack {
    Attack {
        name: "Hyper Fang",
        etype: Type::Normal,
        atype: AttackType::Physical,
        strength: 80,
        acc: 90,
        //ap: 15,
        effect_1: Effect::Flinch10,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn hypnosis() -> Attack {
    Attack {
        name: "Hypnosis",
        etype: Type::Psychic,
        atype: AttackType::Status,
        strength: 0,
        acc: 60,
        //ap: 20,
        effect_1: Effect::None,//effect: schleep,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn ice_beam() -> Attack {
    Attack {
        name: "Ice Beam",
        etype: Type::Ice,
        atype: AttackType::Special,
        strength: 90,
        acc: 100,
        //ap: 10,
        effect_1: Effect::Status(Status::Freeze(0), 10),//effect: 10% freeze chance,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn ice_punch() -> Attack {
    Attack {
        name: "Ice Punch",
        etype: Type::Ice,
        atype: AttackType::Physical,
        strength: 75,
        acc: 100,
        //ap: 15,
        effect_1: Effect::Status(Status::Freeze(0), 10),//effect: 10% freeze chance,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn jump_kick() -> Attack {
    Attack {
        name: "Jump Kick",
        etype: Type::Fighting,
        atype: AttackType::Physical,
        strength: 100, //gen 1 70
        acc: 95,
        //ap: 10,
        effect_1: Effect::None,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn karate_chop() -> Attack {
    Attack {
        name: "Karate Chop",
        etype: Type::Fighting,
        atype: AttackType::Physical,
        strength: 50,
        acc: 100,
        //ap: 25,
        effect_1: Effect::None,//effect: crit hit rate up,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn kinesis() -> Attack {
    Attack {
        name: "Kinesis",
        etype: Type::Psychic,
        atype: AttackType::Status,
        strength: 0,
        acc: 80,
        //ap: 15,
        effect_1: Effect::None,//effect: target acc down 1 stage,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn leech_life() -> Attack {
    Attack {
        name: "Leech Life",
        etype: Type::Bug,
        atype: AttackType::Physical,
        strength: 80,
        acc: 100,
        //ap: 10,
        effect_1: Effect::Absorb(0),
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn leech_seed() -> Attack {
    Attack {
        name: "Leech Seed",
        etype: Type::Grass,
        atype: AttackType::Status,
        strength: 0,
        acc: 90,
        //ap: 10,
        effect_1: Effect::None,//effect: plant seed gachiBass after turn 1/16 of target hp drained (round down, not 0) + added to other pok
        //doesnt work against plant pok also if toxic durch N parameter affected -> ^ each turn,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn leer() -> Attack {
    Attack {
        name: "Leer",
        etype: Type::Normal,
        atype: AttackType::Status,
        strength: 0,
        acc: 100,
        //ap: 30,
        effect_1: Effect::StatusChange2(StatChange2::Defense(-1)),//effect: defense down 1 stage,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn lick() -> Attack {
    Attack {
        name: "Lick",
        etype: Type::Ghost,
        atype: AttackType::Physical,
        strength: 30,
        acc: 100,
        //ap: 30,
        effect_1: Effect::Status(Status::Paralysis, 30),//effect: 30% paral chance, cannot paral ghost in gen1,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn light_screen() -> Attack {
    Attack {
        name: "Light Screen",
        etype: Type::Psychic,
        atype: AttackType::Status,
        strength: 0,
        acc: 0,
        //ap: 30,
        effect_1: Effect::None, //effect: user special def +2 stages when opponent dmges user with special move,
        effect_2: Effect::None,
        //mirror move: False,
    }
}

pub fn lovely_kiss() -> Attack {
    Attack {
        name: "Lovely Kiss",
        etype: Type::Normal,
        atype: AttackType::Status,
        strength: 0,
        acc: 75,
        //ap: 10,
        effect_1: Effect::Status(Status::Sleep(0), 100),//effect: schleep,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn low_kick() -> Attack { // wenn nicht Gen 1 def: weight etc.
    Attack {
        name: "Low Kick",
        etype: Type::Fighting,
        atype: AttackType::Physical,
        strength: 50,
        acc: 90,
        //ap: 20,
        effect_1: Effect::Flinch33,//effect: 30% flinch, do 33
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn meditate() -> Attack {
    Attack {
        name: "Meditate",
        etype: Type::Psychic,
        atype: AttackType::Status,
        strength: 0,
        acc: 0,
        //ap: 40,
        effect_1: Effect::StatusChange1(StatChange1::Attack(1)),//effect: attack up 1 stage,
        effect_2: Effect::None,
        //mirror move: False,
    }
}

pub fn mega_drain() -> Attack {
    Attack {
        name: "Mega Drain",
        etype: Type::Grass,
        atype: AttackType::Special,
        strength: 40,
        acc: 100,
        //ap: 15,
        effect_1: Effect::Absorb(0),
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn mega_kick() -> Attack {
    Attack {
        name: "Mega Kick",
        etype: Type::Normal,
        atype: AttackType::Physical,
        strength: 120,
        acc: 75,
        //ap: 5,
        effect_1: Effect::None,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn mega_punch() -> Attack {
    Attack {
        name: "Mega Punch",
        etype: Type::Normal,
        atype: AttackType::Physical,
        strength: 80,
        acc: 85,
        //ap: 20,
        effect_1: Effect::None,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn metronome() -> Attack {
    Attack {
        name: "Metronome",
        etype: Type::Normal,
        atype: AttackType::Status,
        strength: 0,
        acc: 0,
        //ap: 10,
        effect_1: Effect::None,//effect: select random move and execute,
        effect_2: Effect::None,
        //mirror move: False,
    }
}

pub fn mimic() -> Attack {
    Attack {
        name: "Mimic",
        etype: Type::Normal,
        atype: AttackType::Status,
        strength: 0,
        acc: 0,
        //ap: 10,
        effect_1: Effect::None,//effect: Copy target move, until battle end or switch out,
        effect_2: Effect::None,
        //mirror move: False,
    }
}

pub fn minimize() -> Attack {
    Attack {
        name: "Minimize",
        etype: Type::Normal,
        atype: AttackType::Status,
        strength: 0,
        acc: 0,
        //ap: 10, 20 till gen 5
        effect_1: Effect::None,//effect: evasion up 1 stage + tiny image till faint/switch/end in gen1,
        effect_2: Effect::None,
        //mirror move: False,
    }
}

pub fn mirror_move() -> Attack {
    Attack {
        name: "Mirror Move",
        etype: Type::Flying,
        atype: AttackType::Status,
        strength: 0,
        acc: 0,
        //ap: 20,
        effect_1: Effect::None,//effect: use last move targetted at user by Pokemon on field
        //fail if no move selection, switch out in same round oder last use war mirror move,
        effect_2: Effect::None,
        //mirror move: False,
    }
}

pub fn mist() -> Attack {
    Attack {
        name: "Mist",
        etype: Type::Ice,
        atype: AttackType::Status,
        strength: 0,
        acc: 0,
        //ap: 30,
        effect_1: Effect::None,//effect: protection from stat changes till switched out Moves die Stat change als Side effect haben können weiterhin (z.B. Acid) in gen1,
        effect_2: Effect::None,
        //mirror move: False,
    }
}

pub fn night_shade() -> Attack {
    Attack {
        name: "Night Shade",
        etype: Type::Ghost,
        atype: AttackType::Special,
        strength: 0,
        acc: 100,
        //ap: 15,
        effect_1: Effect::None,//effect: so viel wie user level,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn pay_day() -> Attack {
    Attack {
        name: "Pay Day",
        etype: Type::Normal,
        atype: AttackType::Physical,
        strength: 40,
        acc: 100,
        //ap: 20,
        effect_1: Effect::None,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn peck() -> Attack {
    Attack {
        name: "Pekc",
        etype: Type::Flying,
        atype: AttackType::Physical,
        strength: 35,
        acc: 100,
        //ap: 35,
        effect_1: Effect::None,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn petal_dance() -> Attack {
    Attack {
        name: "Petal Dance",
        etype: Type::Grass,
        atype: AttackType::Special,
        strength: 120, //gen 1 70
        acc: 100,
        //ap: 10, //gen 1 20
        effect_1: Effect::None,//effect: 3-4 moves in a row, cannot switch out can only use petal, if fully exec. confusion,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn pin_missile() -> Attack {
    Attack {
        name: "Pin Missile",
        etype: Type::Bug,
        atype: AttackType::Physical,
        strength: 25, //gen 1 14
        acc: 95, //gen 1 85
        //ap: 20,
        effect_1: Effect::None,//effect: wie barrage,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn poison_gas() -> Attack {
    Attack {
        name: "Poison Gas",
        etype: Type::Poison,
        atype: AttackType::Status,
        strength: 0,
        acc: 90, //gen 1 55
        //ap: 40,
        effect_1: Effect::Status(Status::Poison, 100),//effect: poison,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn poison_powder() -> Attack {
    Attack {
        name: "Poison Powder",
        etype: Type::Poison,
        atype: AttackType::Status,
        strength: 0,
        acc: 75,
        //ap: 35,
        effect_1: Effect::Status(Status::Poison, 100),//effect: poisons (not steel/poison),
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn poison_sting() -> Attack {
    Attack {
        name: "Poison Sting",
        etype: Type::Poison,
        atype: AttackType::Physical,
        strength: 15,
        acc: 100,
        //ap: 35,
        effect_1: Effect::Status(Status::Poison, 20),//effect: 20% poison,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn pound() -> Attack {
    Attack {
        name: "Pound",
        etype: Type::Normal,
        atype: AttackType::Physical,
        strength: 40,
        acc: 100,
        //ap: 35,
        effect_1: Effect::None,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn psybeam() -> Attack {
    Attack {
        name: "Psybeam",
        etype: Type::Psychic,
        atype: AttackType::Special,
        strength: 65,
        acc: 100,
        //ap: 20,
        effect_1: Effect::None,//effect: 10% confusion,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn psychic() -> Attack {
    Attack {
        name: "Psychic",
        etype: Type::Psychic,
        atype: AttackType::Special,
        strength: 90,
        acc: 100,
        //ap: 10,
        effect_1: Effect::StatusChange2(StatChange2::SDC(-1,33)),//effect: 33,2% sp def down 1 stage,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn psywave() -> Attack {
    Attack {
        name: "Psywave",
        etype: Type::Psychic,
        atype: AttackType::Special,
        strength: 0,
        acc: 100,
        //ap: 15,
        effect_1: Effect::None,//effect: dmg random von 1 bis 1.5x user level,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn quick_attack() -> Attack {
    Attack {
        name: "Quick Attack",
        etype: Type::Normal,
        atype: AttackType::Physical,
        strength: 40,
        acc: 100,
        //ap: 30,
        effect_1: Effect::Prio,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn rage() -> Attack {
    Attack {
        name: "Rage",
        etype: Type::Normal,
        atype: AttackType::Physical,
        strength: 20,
        acc: 100,
        //ap: 20,
        effect_1: Effect::None,//effect: endlos rage loop till fait oder end of battle wenn dmg genommen: atk 1 stage up; multi move: 1 for each
        //wenn es im 1. Turn missed keine side effects,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn razor_leaf() -> Attack {
    Attack {
        name: "Razor Leaf",
        etype: Type::Grass,
        atype: AttackType::Physical,
        strength: 55,
        acc: 95,
        //ap: 25,
        effect_1: Effect::None,//effect: increased crit ratio,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn razor_wind() -> Attack {
    Attack {
        name: "Razor Wind",
        etype: Type::Normal,
        atype: AttackType::Special,
        strength: 80,
        acc: 100, //gen 1 75
        //ap: 10,
        effect_1: Effect::None,//effect: 1. move nichts (<> made a whirlwind) turn danach dmg und -pp,
        effect_2: Effect::None,
        //mirror move: False,
    }
}

pub fn recover() -> Attack {
    Attack {
        name: "Recover",
        etype: Type::Normal,
        atype: AttackType::Status,
        strength: 0,
        acc: 0,
        //ap: 10, //20 in gen1
        effect_1: Effect::None,//effect: up to 50% of hp restore,
        effect_2: Effect::None,
        //mirror move: False,
    }
}

pub fn reflect() -> Attack {
    Attack {
        name: "Reflect",
        etype: Type::Psychic,
        atype: AttackType::Status,
        strength: 0,
        acc: 0,
        //ap: 20,
        effect_1: Effect::None,//effect: doubles defense when attacked with phys move bissle wie light screen,
        effect_2: Effect::None,
        //mirror move: False,
    }
}

pub fn rest() -> Attack {
    Attack {
        name: "Rest",
        etype: Type::Psychic,
        atype: AttackType::Status,
        strength: 0,
        acc: 0,
        //ap: 10,
        effect_1: Effect::None,//effect: user is put to sleep but HP are restored,
        effect_2: Effect::None,
        //mirror move: False,
    }
}

pub fn roar() -> Attack {
    Attack {
        name: "Roar",
        etype: Type::Normal,
        atype: AttackType::Status,
        strength: 0,
        acc: 0,
        //ap: 20,
        effect_1: Effect::None,//effect: in gen 1 none, in gen 2 force pokemon switch (random),
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn rock_slide() -> Attack {
    Attack {
        name: "Rock Slide",
        etype: Type::Rock,
        atype: AttackType::Physical,
        strength: 75,
        acc: 90,
        //ap: 10,
        effect_1: Effect::None,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn rock_throw() -> Attack {
    Attack {
        name: "Rock Throw",
        etype: Type::Rock,
        atype: AttackType::Physical,
        strength: 50,
        acc: 100, //gen 1 65
        //ap: 15,
        effect_1: Effect::None,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn rolling_kick() -> Attack {
    Attack {
        name: "Rolling Kick",
        etype: Type::Fighting,
        atype: AttackType::Physical,
        strength: 60,
        acc: 85,
        //ap: 15,
        effect_1: Effect::Flinch33,//effect: 30% flinch, do 33
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn sand_attack() -> Attack {
    Attack {
        name: "Sand Attack",
        etype: Type::Ground, //gen 1 normal
        atype: AttackType::Status,
        strength: 0,
        acc: 100,
        //ap: 15,
        effect_1: Effect::None,//effect: accuracy down 1 stage,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn scratch() -> Attack {
    Attack {
        name: "Scratch",
        etype: Type::Normal,
        atype: AttackType::Physical,
        strength: 40,
        acc: 100,
        //ap: 35,
        effect_1: Effect::None,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn screech() -> Attack {
    Attack {
        name: "Screech",
        etype: Type::Normal,
        atype: AttackType::Status,
        strength: 0,
        acc: 85,
        //ap: 40,
        effect_1: Effect::StatusChange2(StatChange2::Defense(-2)),//effect: defense down 2 stages,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn seismic_toss() -> Attack {
    Attack {
        name: "Seismic toss",
        etype: Type::Fighting,
        atype: AttackType::Physical,
        strength: 0,
        acc: 100,
        //ap: 20,
        effect_1: Effect::None,//effect: equal to user level in gen1, keinerlei Type-Inflictions -> kann auch ghost hitten,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn self_destruct() -> Attack {
    Attack {
        name: "Self-Destruct",
        etype: Type::Normal,
        atype: AttackType::Physical,
        strength: 200, //gen 1 130 + halves defense
        acc: 100,
        //ap: 5,
        effect_1: Effect::None,//effect: user faints,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn sharpen() -> Attack {
    Attack {
        name: "Sharpen",
        etype: Type::Normal,
        atype: AttackType::Status,
        strength: 0,
        acc: 0,
        //ap: 30,
        effect_1: Effect::StatusChange1(StatChange1::Attack(1)),//effect: attack up 1 stage,
        effect_2: Effect::None,
        //mirror move: False,
    }
}

pub fn sing() -> Attack {
    Attack {
        name: "Sing",
        etype: Type::Normal,
        atype: AttackType::Status,
        strength: 0,
        acc: 55,
        //ap: 15,
        effect_1: Effect::Status(Status::Sleep(0), 100),//effect: schleep,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn skull_bash() -> Attack {
    Attack {
        name: "Skull Bash",
        etype: Type::Normal,
        atype: AttackType::Physical,
        strength: 130,
        acc: 100,
        //ap: 10,
        effect_1: Effect::None,//effect: turn 1 nothing "lower head" turn 2 dmg -pp cant be switched out till completed,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn sky_attack() -> Attack {
    Attack {
        name: "Sky Attack",
        etype: Type::Flying,
        atype: AttackType::Physical,
        strength: 140,
        acc: 90,
        //ap: 5,
        effect_1: Effect::None,//effect: turn 1 nothing "glow" turn 2 dmg -pp,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn slam() -> Attack {
    Attack {
        name: "Slam",
        etype: Type::Normal,
        atype: AttackType::Physical,
        strength: 80,
        acc: 75,
        //ap: 20,
        effect_1: Effect::None,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn slash() -> Attack {
    Attack {
        name: "Slash",
        etype: Type::Normal,
        atype: AttackType::Physical,
        strength: 70,
        acc: 100,
        //ap: 20,
        effect_1: Effect::None,//effect: increased crit,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn sleep_powder() -> Attack {
    Attack {
        name: "Sleep Powder",
        etype: Type::Grass,
        atype: AttackType::Status,
        strength: 0,
        acc: 75,
        //ap: 15,
        effect_1: Effect::Status(Status::Sleep(0), 100),//effect: schleep,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn sludge() -> Attack {
    Attack {
        name: "Sludge",
        etype: Type::Poison,
        atype: AttackType::Special,
        strength: 65,
        acc: 100,
        //ap: 20,
        effect_1: Effect::Status(Status::Poison, 40),//effect: 40% poison,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn smog() -> Attack {
    Attack {
        name: "Smog",
        etype: Type::Poison,
        atype: AttackType::Special,
        strength: 30,
        acc: 70,
        //ap: 20,
        effect_1: Effect::Status(Status::Poison, 40),//effect: 40% poison,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn smokescreen() -> Attack {
    Attack {
        name: "Smokescreen",
        etype: Type::Normal,
        atype: AttackType::Status,
        strength: 0,
        acc: 100,
        //ap: 20,
        effect_1: Effect::None,//effect: acc down 1 stage,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn soft_boiled() -> Attack {
    Attack {
        name: "Soft-Boiled",
        etype: Type::Normal,
        atype: AttackType::Status,
        strength: 0,
        acc: 0,
        //ap: 10,
        effect_1: Effect::None,//effect: up to 50% regen,
        effect_2: Effect::None,
        //mirror move: False,
    }
}

pub fn solar_beam() -> Attack {
    Attack {
        name: "Solar Beam",
        etype: Type::Grass,
        atype: AttackType::Special,
        strength: 120,
        acc: 100,
        //ap: 10,
        effect_1: Effect::None,//effect: turn 1 "take in sunlight" turn 2 dmg etc.,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn sonic_boom() -> Attack {
    Attack {
        name: "Sonic Boom",
        etype: Type::Normal,
        atype: AttackType::Special,
        strength: 0,
        acc: 90,
        //ap: 20,
        effect_1: Effect::None,//effect: 20 dmg no weakness/resistance,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn spike_cannon() -> Attack {
    Attack {
        name: "Spike Cannon",
        etype: Type::Normal,
        atype: AttackType::Physical,
        strength: 20,
        acc: 100,
        //ap: 15,
        effect_1: Effect::None,//effect: wie Barrage,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn splash() -> Attack {
    Attack {
        name: "Splash",
        etype: Type::Normal,
        atype: AttackType::Status,
        strength: 0,
        acc: 0,
        //ap: 40,
        effect_1: Effect::None,//effect: none "No effect!",
        effect_2: Effect::None,
        //mirror move: False,
    }
}

pub fn spore() -> Attack {
    Attack {
        name: "Spore",
        etype: Type::Grass,
        atype: AttackType::Status,
        strength: 0,
        acc: 100,
        //ap: 15,
        effect_1: Effect::Status(Status::Sleep(0), 100),//effect: schleep,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn stomp() -> Attack {
    Attack {
        name: "Stomp",
        etype: Type::Normal,
        atype: AttackType::Physical,
        strength: 65,
        acc: 100,
        //ap: 20,
        effect_1: Effect::Flinch33,//effect: 30% flinch,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn strength() -> Attack {
    Attack {
        name: "Strength",
        etype: Type::Normal,
        atype: AttackType::Physical,
        strength: 80,
        acc: 100,
        //ap: 15,
        effect_1: Effect::None,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn string_shot() -> Attack {
    Attack {
        name: "String Shot",
        etype: Type::Bug,
        atype: AttackType::Status,
        strength: 0,
        acc: 95,
        //ap: 40,
        effect_1: Effect::StatusChange2(StatChange2::Init(-1)),//effect: init down 1 stage,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn struggle() -> Attack {
    Attack {
        name: "Struggle",
        etype: Type::Normal,
        atype: AttackType::Physical,
        strength: 50,
        acc: 100,
        //ap: 1,
        effect_1: Effect::None,//effect: 1/2 of dmg recoil, automatisch wenn keine ap mehr aber move, implement specially?
        effect_2: Effect::None,
        //mirror move: False,
    }
}

pub fn stun_spore() -> Attack {
    Attack {
        name: "Stun Spore",
        etype: Type::Grass,
        atype: AttackType::Status,
        strength: 0,
        acc: 75,
        //ap: 30,
        effect_1: Effect::Status(Status::Paralysis, 100),//effect: Paral,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn submission() -> Attack {
    Attack {
        name: "Submission",
        etype: Type::Fighting,
        atype: AttackType::Physical,
        strength: 80,
        acc: 80,
        //ap: 20,
        effect_1: Effect::Recoil(0),//effect: 25% recoil,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn substitue() -> Attack { //cancer???
    Attack {
        name: "Substitute",
        etype: Type::Normal,
        atype: AttackType::Status,
        strength: 0,
        acc: 0,
        //ap: 10,
        effect_1: Effect::None,//effect: https://bulbapedia.bulbagarden.net/wiki/Substitute_(move)#Generation_I,
        effect_2: Effect::None,
        //mirror move: False,
    }
}

pub fn super_fang() -> Attack {
    Attack {
        name: "Super Fang",
        etype: Type::Normal,
        atype: AttackType::Physical,
        strength: 0,
        acc: 90,
        //ap: 10,
        effect_1: Effect::None,//effect: 1/2 of enemy hp can hit ghost in gen1,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn supersonic() -> Attack {
    Attack {
        name: "Supersonic",
        etype: Type::Normal,
        atype: AttackType::Status,
        strength: 0,
        acc: 55,
        //ap: 20,
        effect_1: Effect::None,//effect: Konfuzius,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn surf() -> Attack {
    Attack {
        name: "Surf",
        etype: Type::Water,
        atype: AttackType::Special,
        strength: 90,
        acc: 100,
        //ap: 15,
        effect_1: Effect::None,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn swift() -> Attack {
    Attack {
        name: "Swift",
        etype: Type::Normal,
        atype: AttackType::Special,
        strength: 60,
        acc: 0,
        //ap: 20,
        effect_1: Effect::None,//effect: trifft immer auch dig oder dive(nur in gen 1),
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn sword_dance() -> Attack {
    Attack {
        name: "Sword Dance",
        etype: Type::Normal,
        atype: AttackType::Status,
        strength: 0,
        acc: 0,
        //ap: 20,
        effect_1: Effect::StatusChange1(StatChange1::Attack(2)),//effect: attack up 2 stages,
        effect_2: Effect::None,
        //mirror move: False,
    }
}

pub fn tackle() -> Attack {
    Attack {
        name: "Tackle",
        etype: Type::Normal,
        atype: AttackType::Physical,
        strength: 35,
        acc: 95,
        //ap: 35,
        effect_1: Effect::None,
        effect_2: Effect::None,
        //mirror move: True
    }
}

pub fn tail_whip() -> Attack {
    Attack {
        name: "Tail Whip",
        etype: Type::Normal,
        atype: AttackType::Status,
        strength: 0,
        acc: 100,
        //ap: 30,
        effect_1: Effect::StatusChange2(StatChange2::Defense(-1)),//effect: defense down 1 stage,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn take_down() -> Attack {
    Attack {
        name: "Take Down",
        etype: Type::Normal,
        atype: AttackType::Physical,
        strength: 90,
        acc: 85,
        //ap: 20,
        effect_1: Effect::Recoil(0),//effect: 1/4 recoil,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn teleport() -> Attack {
    Attack {
        name: "Teleport",
        etype: Type::Psychic,
        atype: AttackType::Status,
        strength: 0,
        acc: 0,
        //ap: 20,
        effect_1: Effect::None,//effect: kinda wie roar nur mit user,
        effect_2: Effect::None,
        //mirror move: False,
    }
}

pub fn thrash() -> Attack {
    Attack {
        name: "Thrash",
        etype: Type::Normal,
        atype: AttackType::Physical,
        strength: 120, // gen 1 90
        acc: 100,
        //ap: 10, //gen 1 20
        effect_1: Effect::None,//effect: dmg 3-4 turns kann nicht forcefully von selbst beendet werden auch, wenn richtig beendet confuse,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn thunder() -> Attack {
    Attack {
        name: "Thunder",
        etype: Type::Electric,
        atype: AttackType::Special,
        strength: 110,
        acc: 70,
        //ap: 10,
        effect_1: Effect::Status(Status::Paralysis, 10),//effect: 10% paral not elec, later during fly bounce sky drop,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn thunder_punch() -> Attack {
    Attack {
        name: "Thunder Punch",
        etype: Type::Electric,
        atype: AttackType::Physical,
        strength: 75,
        acc: 100,
        //ap: 15,
        effect_1: Effect::Status(Status::Paralysis, 10),//effect: 10% paral not elec,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn thunder_shock() -> Attack {
    Attack {
        name: "Thunder Shock",
        etype: Type::Electric,
        atype: AttackType::Special,
        strength: 40,
        acc: 100,
        //ap: 30,
        effect_1: Effect::Status(Status::Paralysis, 10),//effect: 10% paral not elec,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn thunder_wave() -> Attack {
    Attack {
        name: "Thunder Wave",
        etype: Type::Electric,
        atype: AttackType::Status,
        strength: 0,
        acc: 100,
        //ap: 20,
        effect_1: Effect::Status(Status::Paralysis, 100),//effect: paral cannot ground,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn thunderbolt() -> Attack {
    Attack {
        name: "Thunderbolt",
        etype: Type::Electric,
        atype: AttackType::Special,
        strength: 90,
        acc: 100,
        //ap: 15,
        effect_1: Effect::Status(Status::Paralysis, 10),//effect: 10% paral not elec auch later schon,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn toxic() -> Attack {
    Attack {
        name: "Toxic",
        etype: Type::Poison,
        atype: AttackType::Status,
        strength: 0,
        acc: 90, //gen 1 85
        //ap: 10,
        effect_1: Effect::Status(Status::Poison, 100),//effect: badly poisons dmg: N*x x is 1/16 of target max hp, while badly poisend n++ every dmg by poison or leech seed
        //haze/switch/end -> normal poison, rest: cured but N remains(maybe change),
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn transform() -> Attack { //fr??
    Attack {
        name: "Transform",
        etype: Type::Normal,
        atype: AttackType::Status,
        strength: 0,
        acc: 0,
        //ap: 10,
        effect_1: Effect::None,//effect: https://bulbapedia.bulbagarden.net/wiki/Transform_(move),
        effect_2: Effect::None,
        //mirror move: False,
    }
}

pub fn tri_attack() -> Attack {
    Attack {
        name: "Tri Attack",
        etype: Type::Normal,
        atype: AttackType::Special,
        strength: 80,
        acc: 100,
        //ap: 10,
        effect_1: Effect::None,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn twineedle() -> Attack {
    Attack {
        name: "Twineedle",
        etype: Type::Bug,
        atype: AttackType::Physical,
        strength: 25,
        acc: 100,
        //ap: 20,
        effect_1: Effect::None,//effect: double hit 2nd: 20% poison unless poison,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn vine_whip() -> Attack {
    Attack {
        name: "Vine Whip",
        etype: Type::Grass,
        atype: AttackType::Physical,
        strength: 45, //gen 1 35
        acc: 100,
        //ap: 25, //gen 1 10
        effect_1: Effect::None,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn vise_grip() -> Attack {
    Attack {
        name: "Vise Grip",
        etype: Type::Normal,
        atype: AttackType::Physical,
        strength: 55,
        acc: 100,
        //ap: 30,
        effect_1: Effect::None,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn water_gun() -> Attack {
    Attack {
        name: "Water Gun",
        etype: Type::Water,
        atype: AttackType::Special,
        strength: 40,
        acc: 100,
        //ap: 25,
        effect_1: Effect::None,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn waterfall() -> Attack {
    Attack {
        name: "Waterfall",
        etype: Type::Water,
        atype: AttackType::Physical,
        strength: 80,
        acc: 100,
        //ap: 15,
        effect_1: Effect::None,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn whirlwind() -> Attack {
    Attack {
        name: "Whirlwind",
        etype: Type::Normal,
        atype: AttackType::Status,
        strength: 0,
        acc: 0, //gen 1 85(?)
        //ap: 20,
        effect_1: Effect::None,//effect: switch out opponent pok,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn wing_attack() -> Attack {
    Attack {
        name: "Wing Attack",
        etype: Type::Flying,
        atype: AttackType::Physical,
        strength: 60, //gen 1 35
        acc: 100,
        //ap: 35,
        effect_1: Effect::None,
        effect_2: Effect::None,
        //mirror move: True,
    }
}

pub fn withdraw() -> Attack {
    Attack {
        name: "Withdraw",
        etype: Type::Water,
        atype: AttackType::Status,
        strength: 0,
        acc: 0,
        //ap: 40,
        effect_1: Effect::StatusChange1(StatChange1::Defense(1)),//effect: defense up 1 stage,
        effect_2: Effect::None,
        //mirror move: False,
    }
}

pub fn wrap() -> Attack {
    Attack {
        name: "Wrap",
        etype: Type::Normal,
        atype: AttackType::Physical,
        strength: 15,
        acc: 90,
        //ap: 20,
        effect_1: Effect::None,//effect: wie bind etc.,
        effect_2: Effect::None,
        //mirror move: True,
    }
}