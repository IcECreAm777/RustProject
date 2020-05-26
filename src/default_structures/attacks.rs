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
