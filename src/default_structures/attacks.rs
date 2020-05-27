use crate::default_structures::Type;

pub struct Attack {
    pub name: &'static str,
    pub etype: Type,
    pub atype: AttackType,
    pub strength: u32,
    pub acc: u32,
    //ap: u8,
    //effect: //TODO data type
    //mirror move: Bool,
}

pub enum AttackType {
    Physicial,
    Special,
    Status,
}

pub fn dummy() -> Attack {
    return  Attack {
        name: "None",
        etype: Type::None,
        atype: AttackType::Physicial,
        strength: 0,
        acc: 0
        //ap: 0,
        //effect: None,
        //mirror move: false
    };
}

pub fn absorb() -> Attack {
    return Attack {
        name: "Absorb",
        etype: Type::Grass,
        atype: AttackType::Special,
        strength: 20,
        acc: 100,
        //ap: 25,
        //effect: Heal dmg/2, außer substitute
        //mirror move: True
    };
} 

pub fn acid() -> Attack {
    return Attack {
        name: "Acid",
        etype: Type::Poison,
        atype: AttackType::Special,
        strength: 40,
        acc: 100,
        //ap: 30,
        //effect: 33.2% chance Def um "1 stage" runter
        //mirror move: True
    };
}

pub fn acid_armor() -> Attack {
    return Attack {
        name: "Acid Armor",
        etype: Type::Poison,
        atype: AttackType::Status,
        strength: 0,
        acc: 0,
        //ap: 20,
        //effect: Defense up 2 stages,
        //mirror move: False,
    };
}

pub fn agility() -> Attack {
    return Attack {
        name: "Agility",
        etype: Type::Psychic,
        atype: AttackType::Status,
        strength: 0,
        acc: 0,
        //ap: 30,
        //effect: Speed up 2 stages,
        //mirror move: False,
    };
}

pub fn amnesia() -> Attack {
    return Attack {
        name: "Amnesia",
        etype: Type::Psychic,
        atype: AttackType::Status,
        strength: 0,
        acc: 0,
        //ap: 20,
        //effect: Special Def up 2 stages,
        //mirror move: False,
    };
}

pub fn aurora_beam() -> Attack {
    return Attack {
        name: "Aurora Beam",
        etype: Type::Ice,
        atype: AttackType::Special,
        strength: 65,
        acc: 100,
        //ap: 20,
        //effect: 33,2% chance opp. attack down 1 stage,
        //mirror move: True,
    };
}

pub fn barrage() -> Attack {
    return Attack {
        name: "Barrage",
        etype: Type::Normal,
        atype: AttackType::Physicial,
        strength: 15,
        acc: 85,
        //ap: 20,
        //effect: Chance 2, 3 Hits: 37,5%, 4,5: 12,5%, alle konsekutiven Hits schaden so viel wie der erste,
        //mirror move: True,
    };
}

pub fn barrier() -> Attack {
    return Attack {
        name: "Barrier",
        etype: Type::Psychic,
        atype: AttackType::Special,
        strength: 0,
        acc: 0,
        //ap: 20,
        //effect: Defense up 2 stages,
        //mirror move: False,
    };
}

pub fn bide() -> Attack {
    return Attack {
        name: "Bide",
        etype: Type::Normal,
        atype: AttackType::Physicial,
        strength: 0,
        acc: 101,
        //ap: ,
        //effect: 2 oder 3 Runden (random) nichts tun (kann geswichted werden) / returned Schaden_received*2, 
        //Typ hat keinen Effekt!! trifft auch Ghost trifft IMMER auch während Dig oder Fly
        //https://bulbapedia.bulbagarden.net/wiki/Bide_(move) ,
        //mirror move: False,
    };
}

pub fn bind() -> Attack {
    return Attack {
        name: "Bind",
        etype: Type::Normal,
        atype: AttackType::Physicial,
        strength: 15,
        acc: 85,
        //ap: 20,
        //effect: Damage für 2-5 turns, in Gen1: kann für Duration nicht angreifen, chances wie bei Barrage
        //https://bulbapedia.bulbagarden.net/wiki/Bind_(move),
        //mirror move: True,
    };
}

pub fn bite() -> Attack {
    return Attack {
        name: "Bite",
        etype: Type::Normal,
        atype: AttackType::Physicial,
        strength: 60,
        acc: 100,
        //ap: 25,
        //effect: 10% flinch chance,
        //mirror move: True,
    };
}

pub fn blizzard() -> Attack {
    return Attack {
        name: "Blizzard",
        etype: Type::Ice,
        atype: AttackType::Special,
        strength: 110,
        acc: 90,
        //ap: 5,
        //effect: 10% freeze chance,
        //mirror move: True,
    };
}

pub fn body_slam() -> Attack {
    return Attack {
        name: "Body Slam",
        etype: Type::Normal,
        atype: AttackType::Physicial,
        strength: 85,
        acc: 100,
        //ap: 15,
        //effect: 30% paral, kann keine normal paralysieren(??),
        //mirror move: True,
    };
}

pub fn bone_club() -> Attack {
    return Attack {
        name: "Bone Club",
        etype: Type::Ground,
        atype: AttackType::Physicial,
        strength: 65,
        acc: 85,
        //ap: 20,
        //effect: 10% flinch,
        //mirror move: True,
    };
}

pub fn bonemerang() -> Attack {
    return Attack {
        name: "Bonemerang",
        etype: Type::Ground,
        atype: AttackType::Physicial,
        strength: 50,
        acc: 90,
        //ap: 10,
        //effect: two hits, 2. Hit dealt genauso viel wie erster,
        //mirror move: True,
    };
}

pub fn bubble() -> Attack {
    return Attack {
        name: "Bubble",
        etype: Type::Water,
        atype: AttackType::Special,
        strength: 20,
        acc: 100,
        //ap: 30,
        //effect: 33,2% chance speed down one stage,
        //mirror move: True,
    };
}

pub fn bubble_beam() -> Attack {
    return Attack {
        name: "Bubble Beam",
        etype: Type::Water,
        atype: AttackType::Special,
        strength: 65,
        acc: 100,
        //ap: 20,
        //effect: 33,2% chance speed down one stage,
        //mirror move: True,
    };
}

pub fn clamp() -> Attack {
    return Attack {
        name: "Clamp",
        etype: Type::Water,
        atype: AttackType::Physicial,
        strength: 35,
        acc: 85,
        //ap: 15,
        //effect: wie Bind eventuell changen?,
        //mirror move: True,
    };
}

pub fn comet_punch() -> Attack {
    return Attack {
        name: "Comet Punch",
        etype: Type::Normal,
        atype: AttackType::Physicial,
        strength: 18,
        acc: 85,
        //ap: 15,
        //effect: Siehe Barrage,
        //mirror move: True,
    };
}

pub fn confuse_ray() -> Attack {
    return Attack {
        name: "Confuse Ray",
        etype: Type::Ghost,
        atype: AttackType::Status,
        strength: 0,
        acc: 100,
        //ap: 10,
        //effect: causes confusion,
        //mirror move: True,
    };
}

pub fn confusion() -> Attack {
    return Attack {
        name: "Confusion",
        etype: Type::Psychic,
        atype: AttackType::Special,
        strength: 50,
        acc: 100,
        //ap: 25,
        //effect: 10% confuse chance,
        //mirror move: True,
    };
}

pub fn constrict() -> Attack {
    return Attack {
        name: "Constrict",
        etype: Type::Normal,
        atype: AttackType::Physicial,
        strength: 10,
        acc: 100,
        //ap: 35,
        //effect: 33,2% chance speed down one stage,
        //mirror move: True,
    };
}

pub fn conversion() -> Attack {
    return Attack {
        name: "Conversion",
        etype: Type::Normal,
        atype: AttackType::Status,
        strength: 0,
        acc: 0,
        //ap: 30,
        //effect: ändert Typ of self zu Typ des Targets,
        //mirror move: False,
    };
}

pub fn counter() -> Attack {
    return Attack {
        name: "Counter",
        etype: Type::Fighting,
        atype: AttackType::Physicial,
        strength: 0,
        acc: 100,
        //ap: 20,
        //effect: last dmg done for counter > 0 und von Normal/Fighting attack -> double dmg sonst miss, keine Typ-Effectiveness
        //decreased priority, konter auch nur letzten Hit von Moves mit mehreren Hits,
        //mirror move: False,
    };
}

pub fn crabhammer() -> Attack {
    return Attack {
        name: "Crabhammer",
        etype: Type::Water,
        atype: AttackType::Physicial,
        strength: 90,
        acc: 85,
        //ap: 10,
        //effect: increased crit rate,
        //mirror move: True,
    };
}

pub fn cut() -> Attack {
    return Attack {
        name: "Cut",
        etype: Type::Normal,
        atype: AttackType::Physicial,
        strength: 50,
        acc: 95,
        //ap: 30,
        //effect: none,
        //mirror move: 30,
    };
}

pub fn defense_curl() -> Attack {
    return Attack {
        name: "Defense Curl",
        etype: Type::Normal,
        atype: AttackType::Status,
        strength: 0,
        acc: 0,
        //ap: 40,
        //effect: Def up 1 stage,
        //mirror move: False,
    };
}

pub fn dig() -> Attack {
    return Attack {
        name: "Dig",
        etype: Type::Ground,
        atype: AttackType::Physicial,
        strength: 80, //100 in gen1
        acc: 100,
        //ap: 10,
        //effect: 1. Turn semi invulnerable (bis auf Bide, Swift, Transform),
        //mirror move: True,
    };
}

pub fn disable() -> Attack {
    return Attack {
        name: "Disable",
        etype: Type::Normal,
        atype: AttackType::Status,
        strength: 0,
        acc: 20,
        //ap: 100,
        //effect: disables randomly 1 attack für 0-7 turns, -1 jede Runde,
        //mirror move: True,
    };
}

pub fn dizzy_punch() -> Attack {
    return Attack {
        name: "Dizzy Punch",
        etype: Type::Normal,
        atype: AttackType::Physicial,
        strength: 70,
        acc: 100,
        //ap: 10,
        //effect: none,
        //mirror move: True,
    };
}

pub fn double_kick() -> Attack {
    return Attack {
        name: "Double Kick",
        etype: Type::Fighting,
        atype: AttackType::Physicial,
        strength: 30,
        acc: 100,
        //ap: 30,
        //effect: hits twice, 2. as much as 1st nur 1. kann critten,
        //mirror move: True,
    };
}

pub fn double_slap() -> Attack {
    return Attack {
        name: "Double Slap",
        etype: Type::Normal,
        atype: AttackType::Physicial,
        strength: 15,
        acc: 85,
        //ap: 10,
        //effect: hits 2-5 times siehe Barrage,
        //mirror move: True,
    };
}

pub fn double_edge() -> Attack {
    return Attack {
        name: "Double Edge",
        etype: Type::Normal,
        atype: AttackType::Physicial,
        strength: 120, //100 in gen1
        acc: 100,
        //ap: 15,
        //effect: recoil: 1/4 of dmg done,
        //mirror move: True,
    };
}

pub fn dragon_rage() -> Attack {
    return Attack {
        name: "Dragon Rage",
        etype: Type::Dragon,
        atype: AttackType::Special,
        strength: 0,
        acc: 100,
        //ap: 10,
        //effect: genau 40HP dmg keine Weaknesses/Resistances,
        //mirror move: True,
    };
}

pub fn dream_eater() -> Attack {
    return Attack {
        name: "Dream Eater",
        etype: Type::Psychic,
        atype: AttackType::Special,
        strength: 100,
        acc: 100,
        //ap: 15,
        //effect: nur if target asleep, heals 50% of dmg dealt, nothing if target not asleep,
        //mirror move: True,
    };
}

pub fn drill_peck() -> Attack {
    return Attack {
        name: "Drill Peck",
        etype: Type::Flying,
        atype: AttackType::Physicial,
        strength: 80,
        acc: 100,
        //ap: 2ß,
        //effect: none,
        //mirror move: True,
    };
}

pub fn earthquake() -> Attack {
    return Attack {
        name: "Earthquake",
        etype: Type::Ground,
        atype: AttackType::Physicial,
        strength: 100,
        acc: 100,
        //ap: 10,
        //effect: none, maybe include hitting dig? if so power *2
        //mirror move: True,
    };
}

pub fn egg_bomb() -> Attack {
    return Attack {
        name: "Egg Bomb",
        etype: Type::Normal,
        atype: AttackType::Physicial,
        strength: 100,
        acc: 75,
        //ap: 10,
        //effect: none,
        //mirror move: True,
    };
}

pub fn ember() -> Attack {
    return Attack {
        name: "Ember",
        etype: Type::Fire,
        atype: AttackType::Special,
        strength: 40,
        acc: 100,
        //ap: 25,
        //effect: 10% burn chance,
        //mirror move: True,
    };
}

pub fn explosion() -> Attack {
    return Attack {
        name: "Explosion",
        etype: Type::Normal,
        atype: AttackType::Physicial,
        strength: 250,  // gen 1 170 (theoretisch mal 2)
        acc: 100,
        //ap: 5,
        //effect: rip user (gen 1 halves target defense during calc),
        //mirror move: True,
    };
}

pub fn fire_blast() -> Attack {
    return Attack {
        name: "Fire Blast",
        etype: Type::Fire,
        atype: AttackType::Special,
        strength: 110,
        acc: 85,
        //ap: 5,
        //effect: 30% burn chance,
        //mirror move: True,
    };
}

pub fn fire_punch() -> Attack {
    return Attack {
        name: "Fire Punch",
        etype: Type::Fire,
        atype: AttackType::Physicial,
        strength: 75,
        acc: 100,
        //ap: 15,
        //effect: 10% burn chance,
        //mirror move: True,
    };
}

pub fn fire_spin() -> Attack {
    return Attack {
        name: "Fire Spin",
        etype: Type::Fire,
        atype: AttackType::Special,
        strength: 35, // gen 1 15
        acc: 85, // gen 1 75
        //ap: 15,
        //effect: Wie Bind etc,
        //mirror move: True,
    };
}

pub fn fissure() -> Attack {
    return Attack {
        name: "Fissure",
        etype: Type::Ground,
        atype: AttackType::Physicial,
        strength: 0,
        acc: 30,
        //ap: 5,
        //effect: INSTAKILL POGGERS (gen 1: wont hit target with higher speed),
        //mirror move: False,
    };
}

pub fn flamethrower() -> Attack {
    return Attack {
        name: "Flamethrower",
        etype: Type::Fire,
        atype: AttackType::Special,
        strength: 90,
        acc: 100,
        //ap: 15,
        //effect: 10% burn chance,
        //mirror move: True,
    };
}

pub fn flash() -> Attack {
    return Attack {
        name: "Flash",
        etype: Type::Normal,
        atype: AttackType::Status,
        strength: 0,
        acc: 100, // gen 1 70
        //ap: 20,
        //effect: target acc one stage down,
        //mirror move: False,
    };
}

pub fn fly() -> Attack {
    return Attack {
        name: "Fly",
        etype: Type::Flying,
        atype: AttackType::Physicial,
        strength: 90,
        acc: 95,
        //ap: 15,
        //effect: wie Dig kann auch von paraly./confusion disrupted werden etc (maybe also include hit by thunder?),
        //mirror move: True,
    };
}

pub fn focus_energy() -> Attack {
    return Attack {
        name: "Focus Energy",
        etype: Type::Normal,
        atype: AttackType::Status,
        strength: 0,
        acc: 0,
        //ap: 30,
        //effect: Gen 1 bug: statt *4 /4 crit rate ((andere Formel in Stadium)),
        //mirror move: False,
    };
}

pub fn fury_attack() -> Attack {
    return Attack {
        name: "Fury Attack",
        etype: Type::Normal,
        atype: AttackType::Physicial,
        strength: 15,
        acc: 85,
        //ap: 20,
        //effect: wie barrage,
        //mirror move: True,
    };
}

pub fn fury_swipes() -> Attack {
    return Attack {
        name: "Fury Swipes",
        etype: Type::Normal,
        atype: AttackType::Physicial,
        strength: 18,
        acc: 80,
        //ap: 15,
        //effect: wie barrage,
        //mirror move: True,
    };
}

pub fn glare() -> Attack {
    return Attack {
        name: "Glare",
        etype: Type::Normal,
        atype: AttackType::Status,
        strength: 0,
        acc: 75, // gen 1 75, gen 5 90, gen 6+ 100
        //ap: 30,
        //effect: paralysis (gen 1 can hit ghost),
        //mirror move: True,
    };
}

pub fn grwol() -> Attack {
    return Attack {
        name: "Growl",
        etype: Type::Normal,
        atype: AttackType::Status,
        strength: 0,
        acc: 100,
        //ap: 40,
        //effect: attack down 1 stage,
        //mirror move: False,
    };
}

pub fn growth() -> Attack {
    return Attack {
        name: "Growth",
        etype: Type::Normal,
        atype: AttackType::Status,
        strength: 0,
        acc: 0,
        //ap: 20,
        //effect: special atk up 1 stage,
        //mirror move: False,
    };
}

pub fn guillotine() -> Attack {
    return Attack {
        name: "Guillotine",
        etype: Type::Normal,
        atype: AttackType::Physicial,
        strength: 0,
        acc: 30,
        //ap: 5,
        //effect: OHK (again no hit if target init > in gen 1),
        //mirror move: False,
    };
}

pub fn gust() -> Attack {
    return Attack {
        name: "Gust",
        etype: Type::Flying, // normal in gen 1
        atype: AttackType::Special,
        strength: 40,
        acc: 100,
        //ap: 35,
        //effect: none (gen 2: can hit fly),
        //mirror move: True,
    };
}

pub fn hardne() -> Attack {
    return Attack {
        name: "Harden",
        etype: Type::Normal,
        atype: AttackType::Status,
        strength: 0,
        acc: 0,
        //ap: 30,
        //effect: def up 1 stage,
        //mirror move: False,
    };
}

pub fn haze() -> Attack {
    return Attack {
        name: "Haze",
        etype: Type::Ice,
        atype: AttackType::Status,
        strength: 0,
        acc: 0,
        //ap: 30,
        //effect: reset stat stages to 0 remove stat reductions from burn and paral, - focus energy
        //cures confusion, bad poison -> regular poison, -non-volatile status -> burn, paral,...,
        //mirror move: False,
    };
}

pub fn headbutt() -> Attack {
    return Attack {
        name: "Headbutt",
        etype: Type::Normal,
        atype: AttackType::Physicial,
        strength: 70,
        acc: 100,
        //ap: 15,
        //effect: 30% flinch,
        //mirror move: True,
    };
}

pub fn high_jump() -> Attack {
    return Attack {
        name: "High Jump",
        etype: Type::Fighting,
        atype: AttackType::Physicial,
        strength: 130, //gen 1 85
        acc: 90,
        //ap: 10,
        //effect: if miss: 1hp dmg,
        //mirror move: True,
    };
}

pub fn horn() -> Attack {
    return Attack {
        name: "Horn",
        etype: Type::Normal,
        atype: AttackType::Physicial,
        strength: 65,
        acc: 100,
        //ap: 25,
        //effect: none,
        //mirror move: True,
    };
}

pub fn horn_drill() -> Attack {
    return Attack {
        name: "Horn Drill",
        etype: Type::Normal,
        atype: AttackType::Physicial,
        strength: 0,
        acc: 30,
        //ap: 5,
        //effect: OHK again if init > oof,
        //mirror move: False,
    };
}

pub fn hydro_pump() -> Attack {
    return Attack {
        name: "Hydro Pump",
        etype: Type::Water,
        atype: AttackType::Special,
        strength: 110,
        acc: 80,
        //ap: 5,
        //effect: none,
        //mirror move: True,
    };
}

pub fn hyper_beam() -> Attack {
    return Attack {
        name: "Hyper Beam",
        etype: Type::Normal,
        atype: AttackType::Special,
        strength: 150,
        acc: 90,
        //ap: 5,
        //effect: recharge turn (not if miss in gen 1) ist quasi aktiv,
        //mirror move: True,
    };
}

pub fn hyper_fang() -> Attack {
    return Attack {
        name: "Hyper Fang",
        etype: Type::Normal,
        atype: AttackType::Physicial,
        strength: 80,
        acc: 90,
        //ap: 15,
        //effect: 10% flinch,
        //mirror move: True,
    };
}

pub fn hypnosis() -> Attack {
    return Attack {
        name: "Hypnosis",
        etype: Type::Psychic,
        atype: AttackType::Status,
        strength: 0,
        acc: 60,
        //ap: 20,
        //effect: schleep,
        //mirror move: True,
    };
}

pub fn ice_beam() -> Attack {
    return Attack {
        name: "Ice Beam",
        etype: Type::Ice,
        atype: AttackType::Special,
        strength: 90,
        acc: 100,
        //ap: 10,
        //effect: 10% freeze chance,
        //mirror move: True,
    };
}

pub fn ice_punch() -> Attack {
    return Attack {
        name: "Ice Punch",
        etype: Type::Ice,
        atype: AttackType::Physicial,
        strength: 75,
        acc: 100,
        //ap: 15,
        //effect: 10% freeze chance,
        //mirror move: True,
    };
}

pub fn jump_kick() -> Attack {
    return Attack {
        name: "Jump Kick",
        etype: Type::Fighting,
        atype: AttackType::Physicial,
        strength: 100, //gen 1 70
        acc: 95,
        //ap: 10,
        //effect: ,
        //mirror move: True,
    };
}

pub fn karate_chop() -> Attack {
    return Attack {
        name: "Karate Chop",
        etype: Type::Fighting,
        atype: AttackType::Physicial,
        strength: 50,
        acc: 100,
        //ap: 25,
        //effect: crit hit rate up,
        //mirror move: True,
    };
}

pub fn kinesis() -> Attack {
    return Attack {
        name: "Kinesis",
        etype: Type::Psychic,
        atype: AttackType::Status,
        strength: 0,
        acc: 80,
        //ap: 15,
        //effect: target acc down 1 stage,
        //mirror move: True,
    };
}

pub fn leech_life() -> Attack {
    return Attack {
        name: "Leech Life",
        etype: Type::Bug,
        atype: AttackType::Physicial,
        strength: 80,
        acc: 100,
        //ap: 10,
        //effect: heal up to 50% of dealt,
        //mirror move: True,
    };
}

pub fn leech_seed() -> Attack {
    return Attack {
        name: "Leech Seed",
        etype: Type::Grass,
        atype: AttackType::Status,
        strength: 0,
        acc: 90,
        //ap: 10,
        //effect: plant seed gachiBass after turn 1/16 of target hp drained (round down, not 0) + added to other pok
        //doesnt work against plant pok also if toxic durch N parameter affected -> ^ each turn,
        //mirror move: True,
    };
}

pub fn leer() -> Attack {
    return Attack {
        name: "Leer",
        etype: Type::Normal,
        atype: AttackType::Status,
        strength: 0,
        acc: 100,
        //ap: 30,
        //effect: defense down 1 stage,
        //mirror move: True,
    };
}

pub fn lick() -> Attack {
    return Attack {
        name: "Lick",
        etype: Type::Ghost,
        atype: AttackType::Physicial,
        strength: 30,
        acc: 100,
        //ap: 30,
        //effect: 30% paral chance, cannot paral ghost in gen1,
        //mirror move: True,
    };
}

pub fn light_screen() -> Attack {
    return Attack {
        name: "Light Screen",
        etype: Type::Psychic,
        atype: AttackType::Status,
        strength: 0,
        acc: 0,
        //ap: 30,
        //effect: user special def +2 stages when opponent dmges user with special move,
        //mirror move: False,
    };
}

pub fn lovely_kiss() -> Attack {
    return Attack {
        name: "Lovely Kiss",
        etype: Type::Normal,
        atype: AttackType::Status,
        strength: 0,
        acc: 75,
        //ap: 10,
        //effect: schleep,
        //mirror move: True,
    };
}

pub fn low_kick() -> Attack { // wenn nicht Gen 1 def: weight etc.
    return Attack {
        name: "Low Kick",
        etype: Type::Fighting,
        atype: AttackType::Physicial,
        strength: 50,
        acc: 90,
        //ap: 20,
        //effect: 30% flinch,
        //mirror move: True,
    };
}

pub fn meditate() -> Attack {
    return Attack {
        name: "Meditate",
        etype: Type::Psychic,
        atype: AttackType::Status,
        strength: 0,
        acc: 0,
        //ap: 40,
        //effect: attack up 1 stage,
        //mirror move: False,
    };
}

pub fn mega_drain() -> Attack {
    return Attack {
        name: "Mega Drain",
        etype: Type::Grass,
        atype: AttackType::Special,
        strength: 40,
        acc: 100,
        //ap: 15,
        //effect: heal up to 50% of dealt,
        //mirror move: True,
    };
}

pub fn mega_kick() -> Attack {
    return Attack {
        name: "Mega Kick",
        etype: Type::Normal,
        atype: AttackType::Physicial,
        strength: 120,
        acc: 75,
        //ap: 5,
        //effect: none,
        //mirror move: True,
    };
}

pub fn mega_punch() -> Attack {
    return Attack {
        name: "Mega Punch",
        etype: Type::Normal,
        atype: AttackType::Physicial,
        strength: 80,
        acc: 85,
        //ap: 20,
        //effect: none,
        //mirror move: True,
    };
}

pub fn metronome() -> Attack {
    return Attack {
        name: "Metronome",
        etype: Type::Normal,
        atype: AttackType::Status,
        strength: 0,
        acc: 0,
        //ap: 10,
        //effect: select random move and execute,
        //mirror move: False,
    };
}

pub fn mimic() -> Attack {
    return Attack {
        name: "Mimic",
        etype: Type::Normal,
        atype: AttackType::Status,
        strength: 0,
        acc: 0,
        //ap: 10,
        //effect: Copy target move, until battle end or switch out,
        //mirror move: False,
    };
}

pub fn minimize() -> Attack {
    return Attack {
        name: "Minimize",
        etype: Type::Normal,
        atype: AttackType::Status,
        strength: 0,
        acc: 0,
        //ap: 10, 20 till gen 5
        //effect: evasion up 1 stage + tiny image till faint/switch/end in gen1,
        //mirror move: False,
    };
}

pub fn mirror_move() -> Attack {
    return Attack {
        name: "Mirror Move",
        etype: Type::Flying,
        atype: AttackType::Status,
        strength: 0,
        acc: 0,
        //ap: 20,
        //effect: use last move targetted at user by Pokemon on field
        //fail if no move selection, switch out in same round oder last use war mirror move,
        //mirror move: False,
    };
}

pub fn mist() -> Attack {
    return Attack {
        name: "Mist",
        etype: Type::Ice,
        atype: AttackType::Status,
        strength: 0,
        acc: 0,
        //ap: 30,
        //effect: protection from stat changes till switched out Moves die Stat change als Side effect haben können weiterhin (z.B. Acid) in gen1,
        //mirror move: False,
    };
}

pub fn night_shade() -> Attack {
    return Attack {
        name: "Night Shade",
        etype: Type::Ghost,
        atype: AttackType::Special,
        strength: 0,
        acc: 100,
        //ap: 15,
        //effect: so viel wie user level,
        //mirror move: True,
    };
}

pub fn pay_day() -> Attack {
    return Attack {
        name: "Pay Day",
        etype: Type::Normal,
        atype: AttackType::Physicial,
        strength: 40,
        acc: 100,
        //ap: 20,
        //effect: none for battle,
        //mirror move: True,
    };
}

pub fn peck() -> Attack {
    return Attack {
        name: "Pekc",
        etype: Type::Flying,
        atype: AttackType::Physicial,
        strength: 35,
        acc: 100,
        //ap: 35,
        //effect: none,
        //mirror move: True,
    };
}

pub fn petal_dance() -> Attack {
    return Attack {
        name: "Petal Dance",
        etype: Type::Grass,
        atype: AttackType::Special,
        strength: 120, //gen 1 70
        acc: 100,
        //ap: 10, //gen 1 20
        //effect: 3-4 moves in a row, cannot switch out can only use petal, if fully exec. confusion,
        //mirror move: True,
    };
}

pub fn pin_missile() -> Attack {
    return Attack {
        name: "Pin Missile",
        etype: Type::Bug,
        atype: AttackType::Physicial,
        strength: 25, //gen 1 14
        acc: 95, //gen 1 85
        //ap: 20,
        //effect: wie barrage,
        //mirror move: True,
    };
}

pub fn poison_gas() -> Attack {
    return Attack {
        name: "Poison Gas",
        etype: Type::Poison,
        atype: AttackType::Status,
        strength: 0,
        acc: 90, //gen 1 55
        //ap: 40,
        //effect: poison,
        //mirror move: True,
    };
}

pub fn poison_powder() -> Attack {
    return Attack {
        name: "Poison Powder",
        etype: Type::Poison,
        atype: AttackType::Status,
        strength: 0,
        acc: 75,
        //ap: 35,
        //effect: poisons (not steel/poison),
        //mirror move: True,
    };
}

pub fn poison_sting() -> Attack {
    return Attack {
        name: "Poison Sting",
        etype: Type::Poison,
        atype: AttackType::Physicial,
        strength: 15,
        acc: 100,
        //ap: 35,
        //effect: 20% poison,
        //mirror move: True,
    };
}

pub fn pound() -> Attack {
    return Attack {
        name: "Pound",
        etype: Type::Normal,
        atype: AttackType::Physicial,
        strength: 40,
        acc: 100,
        //ap: 35,
        //effect: none,
        //mirror move: True,
    };
}

pub fn psybeam() -> Attack {
    return Attack {
        name: "Psybeam",
        etype: Type::Psychic,
        atype: AttackType::Special,
        strength: 65,
        acc: 100,
        //ap: 20,
        //effect: 10% confusion,
        //mirror move: True,
    };
}

pub fn psychic() -> Attack {
    return Attack {
        name: "Psychic",
        etype: Type::Psychic,
        atype: AttackType::Special,
        strength: 90,
        acc: 100,
        //ap: 10,
        //effect: 33,2% sp def down 1 stage,
        //mirror move: True,
    };
}

pub fn psywave() -> Attack {
    return Attack {
        name: "Psywave",
        etype: Type::Psychic,
        atype: AttackType::Special,
        strength: 0,
        acc: 100,
        //ap: 15,
        //effect: dmg random von 1 bis 1.5x user level,
        //mirror move: True,
    };
}

pub fn quick_attack() -> Attack {
    return Attack {
        name: "Quick Attack",
        etype: Type::Normal,
        atype: AttackType::Physicial,
        strength: 40,
        acc: 100,
        //ap: 30,
        //effect: prio 1,
        //mirror move: True,
    };
}

/*pub fn name() -> Attack {
    return Attack {
        name: name,
        etype: Type::,
        atype: AttackType::,
        strength: ,
        acc: ,
        //ap: ,
        //effect: ,
        //mirror move: ,
    };
}*/

pub fn tackle() -> Attack {
    return Attack {
        name: "Tackle",
        etype: Type::Normal,
        atype: AttackType::Physicial,
        strength: 35,
        acc: 95
        //ap: 35,
        //effect none
        //mirror move: True
    }
}
