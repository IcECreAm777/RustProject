
use crate::default_structures::Type;
use crate::default_structures::attacks;

#[derive(Copy, Clone)]
pub struct Pokemon {
    //id: u32, //TODO maybe later
    pub name: &'static str,
    pub ftype: Type,
    pub stype: Type,
    pub health: u32,
    pub atk: u32,
    pub def: u32,
    pub sp_atk: u32,
    pub sp_def: u32,
    pub init: u32,
    pub m1: attacks::Attack, //TODO mutable for attack choosing
    pub m2: attacks::Attack,
    pub m3: attacks::Attack,
    pub m4: attacks::Attack
}

impl Pokemon {
    //TODO add functions
}

pub fn abra() -> Pokemon {
    return Pokemon {
        name: "Abra",
        ftype: Type::Psychic,
        stype: Type::None,
        health: 160,
        atk: 40,
        def: 31,
        sp_atk: 193,
        sp_def: 103,
        init: 166,
        m1: attacks::dummy(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn aerodactyl() -> Pokemon {
    return Pokemon {
        name: "Aerodactyl",
        ftype: Type::Rock,
        stype: Type::Flying,
        health: 270,
        atk: 193,
        def: 121,
        sp_atk: 112,
        sp_def: 139,
        init: 238,
        m1: attacks::dummy(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn alakazam() -> Pokemon {
    return Pokemon {
        name: "Alakazam",
        ftype: Type::Psychic,
        stype: Type::None,
        health: 220,
        atk: 94,
        def: 85,
        sp_atk: 247,
        sp_def: 157,
        init: 220,
        m1: attacks::dummy(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn arbok() -> Pokemon {
    return Pokemon {
        name: "Arbok",
        ftype: Type::Poison,
        stype: Type::None,
        health: 230,
        atk: 157,
        def: 128,
        sp_atk: 121,
        sp_def: 146,
        init: 148,
        m1: attacks::dummy(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn arcanine() -> Pokemon {
    return Pokemon {
        name: "Arcanine",
        ftype: Type::Fire,
        stype: Type::None,
        health: 290,
        atk: 202,
        def: 148,
        sp_atk: 184,
        sp_def: 148,
        init: 175,
        m1: attacks::dummy(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn articuno() -> Pokemon {
    return Pokemon {
        name: "Articuno",
        ftype: Type::Ice,
        stype: Type::Flying,
        health: 290,
        atk: 157,
        def: 184,
        sp_atk: 175,
        sp_def: 229,
        init: 157,
        m1: attacks::dummy(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn beedrill() -> Pokemon {
    return Pokemon {
        name: "Beedrill",
        ftype: Type::Bug,
        stype: Type::Poison,
        health: 240,
        atk: 148,
        def: 76,
        sp_atk: 85,
        sp_def: 148,
        init: 139,
        m1: attacks::dummy(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn bellsprout() -> Pokemon {
    return Pokemon {
        name: "Bellsprout",
        ftype: Type::Grass,
        stype: Type::Poison,
        health: 210,
        atk: 139,
        def: 67,
        sp_atk: 130,
        sp_def: 58,
        init: 76,
        m1: attacks::dummy(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn blastoise() -> Pokemon {
    return Pokemon {
        name: "Blastoise",
        ftype: Type::Water,
        stype: Type::None,
        health: 268,
        atk: 153,
        def: 184,
        sp_atk: 157,
        sp_def: 193,
        init: 144,
        m1: attacks::dummy(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn bulbasur() -> Pokemon {
    return Pokemon {
        name: "Bulbasur",
        ftype: Type::Grass,
        stype: Type::Poison,
        health: 200,
        atk: 92,
        def: 90,
        sp_atk: 121,
        sp_def: 121,
        init: 85,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn butterfree() -> Pokemon {
    return Pokemon {
        name: "Butterfree",
        ftype: Type::Bug,
        stype: Type::Flying,
        health: 230,
        atk: 85,
        def: 94,
        sp_atk: 148,
        sp_def: 148,
        init: 130,
        m1: attacks::dummy(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn caterpie() -> Pokemon {
    return Pokemon {
        name: "Caterpie",
        ftype: Type::Bug,
        stype: Type::None,
        health: 200,
        atk: 58,
        def: 67,
        sp_atk: 40,
        sp_def: 40,
        init: 85,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn chansey() -> Pokemon {
    return Pokemon {
        name: "Chansey",
        ftype: Type::Normal,
        stype: Type::None,
        health: 610,
        atk: 13,
        def: 13,
        sp_atk: 67,
        sp_def: 193,
        init: 94,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn charizard() -> Pokemon {
    return Pokemon {
        name: "Charizard",
        ftype: Type::Fire,
        stype: Type::Flying,
        health: 266,
        atk: 155,
        def: 144,
        sp_atk: 200,
        sp_def: 157,
        init: 184,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn charmander() -> Pokemon {
    return Pokemon {
        name: "Charmander",
        ftype: Type::Fire,
        stype: Type::None,
        health: 188,
        atk: 98,
        def: 81,
        sp_atk: 112,
        sp_def: 94,
        init: 121,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn charmaleon() -> Pokemon {
    return Pokemon {
        name: "Charmaleon",
        ftype: Type::Fire,
        stype: Type::None,
        health: 226,
        atk: 119,
        def: 108,
        sp_atk: 148,
        sp_def: 121,
        init: 148,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn clefable() -> Pokemon {
    return Pokemon {
        name: "Clefable",
        ftype: Type::Normal,
        stype: Type::None,
        health: 300,
        atk: 130,
        def: 135,
        sp_atk: 157,
        sp_def: 166,
        init: 112,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn clefairy() -> Pokemon {
    return Pokemon {
        name: "Clefairy",
        ftype: Type::Normal,
        stype: Type::None,
        health: 250,
        atk: 85,
        def: 90,
        sp_atk: 112,
        sp_def: 121,
        init: 67,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn cloyster() -> Pokemon {
    return Pokemon {
        name: "Cloyster",
        ftype: Type::Water,
        stype: Type::Ice,
        health: 210,
        atk: 175,
        def: 328,
        sp_atk: 157,
        sp_def: 85,
        init: 130,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn cubone() -> Pokemon {
    return Pokemon {
        name: "Cubone",
        ftype: Type::Ground,
        stype: Type::None,
        health: 210,
        atk: 94,
        def: 175,
        sp_atk: 76,
        sp_def: 94,
        init: 67,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn Dewgong() -> Pokemon {
    return Pokemon {
        name: "Dewgong",
        ftype: Type::Water,
        stype: Type::Ice,
        health: 290,
        atk: 130,
        def: 148,
        sp_atk: 130,
        sp_def: 175,
        init: 130,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}
pub fn diglett() -> Pokemon {
    return Pokemon {
        name: "Diglett",
        ftype: Type::Ground,
        stype: Type::None,
        health: 130,
        atk: 103,
        def: 49,
        sp_atk: 67,
        sp_def: 85,
        init: 175,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn ditto() -> Pokemon {
    return Pokemon {
        name: "Ditto",
        ftype: Type::Normal,
        stype: Type::None,
        health: 206,
        atk: 90,
        def: 90,
        sp_atk: 90,
        sp_def: 90,
        init: 90,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn dodrio() -> Pokemon {
    return Pokemon {
        name: "Dodrio",
        ftype: Type::Normal,
        stype: Type::Flying,
        health: 230,
        atk: 202,
        def: 130,
        sp_atk: 112,
        sp_def: 112,
        init: 184,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn doduo() -> Pokemon {
    return Pokemon {
        name: "Doduo",
        ftype: Type::Normal,
        stype: Type::Flying,
        health: 180,
        atk: 157,
        def: 85,
        sp_atk: 67,
        sp_def: 67,
        init: 139,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn dragonair() -> Pokemon {
    return Pokemon {
        name: "Dragonair",
        ftype: Type::Dragon,
        stype: Type::None,
        health: 232,
        atk: 155,
        def: 121,
        sp_atk: 130,
        sp_def: 130,
        init: 130,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn dragonite() -> Pokemon {
    return Pokemon {
        name: "Dragonite",
        ftype: Type::Dragon,
        stype: Type::Flying,
        health: 292,
        atk: 245,
        def: 175,
        sp_atk: 184,
        sp_def: 184,
        init: 148,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn dratini() -> Pokemon {
    return Pokemon {
        name: "Dratini",
        ftype: Type::Dragon,
        stype: Type::None,
        health: 192,
        atk: 119,
        def: 85,
        sp_atk: 94,
        sp_def: 94,
        init: 94,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn drowzee() -> Pokemon {
    return Pokemon {
        name: "Drowzee",
        ftype: Type::Psychic,
        stype: Type::None,
        health: 230,
        atk: 90,
        def: 85,
        sp_atk: 81,
        sp_def: 166,
        init: 80,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn dugtrio() -> Pokemon {
    return Pokemon {
        name: "Dugtrio",
        ftype: Type::Ground,
        stype: Type::None,
        health: 180,
        atk: 148,
        def: 94,
        sp_atk: 94,
        sp_def: 130,
        init: 220,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn eevee() -> Pokemon {
    return Pokemon {
        name: "Eevee",
        ftype: Type::Normal,
        stype: Type::None,
        health: 220,
        atk: 103,
        def: 94,
        sp_atk: 85,
        sp_def: 121,
        init: 103,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn ekans() -> Pokemon {
    return Pokemon {
        name: "Ekans",
        ftype: Type::Poison,
        stype: Type::None,
        health: 180,
        atk: 112,
        def: 83,
        sp_atk: 76,
        sp_def: 101,
        init: 103,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn electabuzz() -> Pokemon {
    return Pokemon {
        name: "Electabuzz",
        ftype: Type::Electric,
        stype: Type::None,
        health: 240,
        atk: 153,
        def: 107,
        sp_atk: 175,
        sp_def: 157,
        init: 193,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn electrode() -> Pokemon {
    return Pokemon {
        name: "Electrode",
        ftype: Type::Electric,
        stype: Type::None,
        health: 230,
        atk: 94,
        def: 130,
        sp_atk: 148,
        sp_def: 148,
        init:256 ,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn exeggcute() -> Pokemon {
    return Pokemon {
        name: "Exeggcute",
        ftype: Type::Grass,
        stype: Type::Psychic,
        health: 230,
        atk: 76,
        def: 148,
        sp_atk: 112,
        sp_def: 85,
        init: 76,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn exeggcutor() -> Pokemon {
    return Pokemon {
        name: "Exeggcutor",
        ftype: Type::Grass,
        stype: Type::Psychic,
        health: 300,
        atk: 175,
        def: 157,
        sp_atk: 229,
        sp_def: 121,
        init: 103,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn farfetch() -> Pokemon {
    return Pokemon {
        name: "Farfetch",
        ftype: Type::Normal,
        stype: Type::Flying,
        health: 214,
        atk: 121,
        def: 103,
        sp_atk: 108,
        sp_def: 116,
        init: 112,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn fearow() -> Pokemon {
    return Pokemon {
        name: "Fearow",
        ftype: Type::Normal,
        stype: Type::Flying,
        health: 240,
        atk: 166,
        def: 121,
        sp_atk: 114,
        sp_def: 114,
        init: 184,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn Flareon() -> Pokemon {
    return Pokemon {
        name: "Flareon",
        ftype: Type::Fire,
        stype: Type::None,
        health: 240,
        atk: 238,
        def: 112,
        sp_atk: 175,
        sp_def: 202,
        init: 121,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn gastly() -> Pokemon {
    return Pokemon {
        name: "Gastly",
        ftype: Type::Ghost,
        stype: Type::Poison,
        health: 170,
        atk: 67,
        def: 58,
        sp_atk: 184,
        sp_def: 67,
        init: 148,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn gengar() -> Pokemon {
    return Pokemon {
        name: "Gengar",
        ftype: Type::Ghost,
        stype: Type::Poison,
        health: 230,
        atk: 121,
        def: 112,
        sp_atk: 238,
        sp_def: 139,
        init: 202,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn geodude() -> Pokemon {
    return Pokemon {
        name: "Geodude",
        ftype: Type::Rock,
        stype: Type::Ground,
        health: 190,
        atk: 148,
        def: 184,
        sp_atk: 58,
        sp_def: 58,
        init: 40,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn gloom() -> Pokemon {
    return Pokemon {
        name: "Gloom",
        ftype: Type::Grass,
        stype: Type::Poison,
        health: 230,
        atk: 121,
        def: 130,
        sp_atk: 157,
        sp_def: 139,
        init: 76,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn golbat() -> Pokemon {
    return Pokemon {
        name: "Golbat",
        ftype: Type::Poison,
        stype: Type::Flying,
        health: 260,
        atk: 148,
        def: 130,
        sp_atk: 121,
        sp_def: 139,
        init: 166,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn goldeen() -> Pokemon {
    return Pokemon {
        name: "Goldeen",
        ftype: Type::Water,
        stype: Type::None,
        health: 200,
        atk: 125,
        def: 112,
        sp_atk: 67,
        sp_def: 94,
        init: 117,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn golduck() -> Pokemon {
    return Pokemon {
        name: "Golduck",
        ftype: Type::Water,
        stype: Type::None,
        health: 270,
        atk: 152,
        def: 144,
        sp_atk: 175,
        sp_def: 148,
        init: 157,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn golem() -> Pokemon {
    return Pokemon {
        name: "Golem",
        ftype: Type::Rock,
        stype: Type::Ground,
        health: 270,
        atk: 202,
        def: 238,
        sp_atk: 103,
        sp_def: 121,
        init: 85,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn graveler() -> Pokemon {
    return Pokemon {
        name: "Graveler",
        ftype: Type::Rock,
        stype: Type::Ground,
        health: 220,
        atk: 175,
        def: 211,
        sp_atk: 85,
        sp_def: 85,
        init: 67,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn grimer() -> Pokemon {
    return Pokemon {
        name: "Grimer",
        ftype: Type::Poison,
        stype: Type::None,
        health: 270,
        atk: 148,
        def: 94,
        sp_atk: 76,
        sp_def: 94,
        init: 49,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn growlithe() -> Pokemon {
    return Pokemon {
        name: "Growlithe",
        ftype: Type::Fire,
        stype: Type::None,
        health: 220,
        atk: 130,
        def: 85,
        sp_atk: 130,
        sp_def: 94,
        init: 112,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn gyarados() -> Pokemon {
    return Pokemon {
        name: "Gyarados",
        ftype: Type::Water,
        stype: Type::Flying,
        health: 300,
        atk: 229,
        def: 146,
        sp_atk: 112,
        sp_def: 184,
        init: 150,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn hauter() -> Pokemon {
    return Pokemon {
        name: "Hauter",
        ftype: Type::Ghost,
        stype: Type::Poison,
        health: 200,
        atk: 94,
        def: 85,
        sp_atk: 211,
        sp_def: 103,
        init: 175,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn hitmonchan() -> Pokemon {
    return Pokemon {
        name: "Hitmonchan",
        ftype: Type::Fighting,
        stype: Type::None,
        health: 210,
        atk: 193,
        def: 146,
        sp_atk: 67,
        sp_def: 202,
        init: 141,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn hitmonlee() -> Pokemon {
    return Pokemon {
        name: "Hitmonlee",
        ftype: Type::Fighting,
        stype: Type::None,
        health: 210,
        atk: 220,
        def: 99,
        sp_atk: 67,
        sp_def: 202,
        init: 161,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn horsea() -> Pokemon {
    return Pokemon {
        name: "Horsea",
        ftype: Type::Water,
        stype: Type::None,
        health: 170,
        atk: 76,
        def: 130,
        sp_atk: 130,
        sp_def: 49,
        init: 112,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn hypno() -> Pokemon {
    return Pokemon {
        name: "Hypno",
        ftype: Type::Psychic,
        stype: Type::None,
        health: 280,
        atk: 135,
        def: 130,
        sp_atk: 135,
        sp_def: 211,
        init: 125,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn ivysaur() -> Pokemon {
    return Pokemon {
        name: "Ivysaur",
        ftype: Type::Grass,
        stype: Type::Poison,
        health: 230,
        atk: 116,
        def: 117,
        sp_atk: 148,
        sp_def: 148,
        init: 112,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn jigglypuff() -> Pokemon {
    return Pokemon {
        name: "jigglypuff",
        ftype: Type::Normal,
        stype: Type::None,
        health: 340,
        atk: 85,
        def: 40,
        sp_atk: 85,
        sp_def: 49,
        init: 40,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn jolteon() -> Pokemon {
    return Pokemon {
        name: "Jolteon",
        ftype: Type::Electric,
        stype: Type::None,
        health: 240,
        atk: 121,
        def: 112,
        sp_atk: 202,
        sp_def: 175,
        init: 238,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn jynx() -> Pokemon {
    return Pokemon {
        name: "Jynx",
        ftype: Type::Ice,
        stype: Type::Psychic,
        health: 240,
        atk: 94,
        def: 67,
        sp_atk: 211,
        sp_def: 175,
        init: 175,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn kabuto() -> Pokemon {
    return Pokemon {
        name: "Kabuto",
        ftype: Type::Rock,
        stype: Type::Water,
        health: 170,
        atk: 148,
        def: 166,
        sp_atk: 103,
        sp_def: 85,
        init: 103,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn kabutops() -> Pokemon {
    return Pokemon {
        name: "Kabutops",
        ftype: Type::Rock,
        stype: Type::Water,
        health: 230,
        atk: 211,
        def: 193,
        sp_atk: 121,
        sp_def: 130,
        init: 148,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn kadabra() -> Pokemon {
    return Pokemon {
        name: "Bulbasur",
        ftype: Type::Psychic,
        stype: Type::None,
        health: 190,
        atk: 67,
        def: 58,
        sp_atk: 220,
        sp_def: 130,
        init: 193,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn kakuna() -> Pokemon {
    return Pokemon {
        name: "Kakuna",
        ftype: Type::Bug,
        stype: Type::Poison,
        health: 200,
        atk: 49,
        def: 94,
        sp_atk: 49,
        sp_def: 49,
        init: 67,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn kangaskhan() -> Pokemon {
    return Pokemon {
        name: "Kangaskhan",
        ftype: Type::Normal,
        stype: Type::None,
        health: 320,
        atk: 175,
        def: 148,
        sp_atk: 76,
        sp_def: 148,
        init: 166,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn kingler() -> Pokemon {
    return Pokemon {
        name: "Kingler",
        ftype: Type::Water,
        stype: Type::None,
        health: 220,
        atk: 238,
        def: 211,
        sp_atk: 94,
        sp_def: 94,
        init: 139,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn koffing() -> Pokemon {
    return Pokemon {
        name: "Koffing",
        ftype: Type::Poison,
        stype: Type::None,
        health: 190,
        atk: 121,
        def: 175,
        sp_atk: 112,
        sp_def: 85,
        init: 67,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn krabby() -> Pokemon {
    return Pokemon {
        name: "Krabby",
        ftype: Type::Water,
        stype: Type::None,
        health: 170,
        atk: 193,
        def: 166,
        sp_atk: 49,
        sp_def: 49,
        init: 94,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn lapras() -> Pokemon {
    return Pokemon {
        name: "Lapras",
        ftype: Type::Water,
        stype: Type::Ice,
        health: 370,
        atk: 157,
        def: 148,
        sp_atk: 157,
        sp_def: 175,
        init: 112,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn lickitung() -> Pokemon {
    return Pokemon {
        name: "Lickitung",
        ftype: Type::Normal,
        stype: Type::None,
        health: 290,
        atk: 103,
        def: 139,
        sp_atk: 112,
        sp_def: 139,
        init: 58,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn machamp() -> Pokemon {
    return Pokemon {
        name: "Machamp",
        ftype: Type::Fighting,
        stype: Type::None,
        health: 290,
        atk: 238,
        def: 148,
        sp_atk: 121,
        sp_def: 157,
        init: 103,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn machoke() -> Pokemon {
    return Pokemon {
        name: "Machoke",
        ftype: Type::Fighting,
        stype: Type::None,
        health: 270,
        atk: 184,
        def: 130,
        sp_atk: 94,
        sp_def: 112,
        init: 85,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn machop() -> Pokemon {
    return Pokemon {
        name: "Machop",
        ftype: Type::Fighting,
        stype: Type::None,
        health: 250,
        atk: 148,
        def: 94,
        sp_atk: 67,
        sp_def: 67,
        init: 67,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn magikarp() -> Pokemon {
    return Pokemon {
        name: "Magikarp",
        ftype: Type::Water,
        stype: Type::None,
        health: 150,
        atk: 22,
        def: 103,
        sp_atk: 31,
        sp_def: 40,
        init: 148,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn magmar() -> Pokemon {
    return Pokemon {
        name: "Magmar",
        ftype: Type::Fire,
        stype: Type::None,
        health: 240,
        atk: 175,
        def: 107,
        sp_atk: 184,
        sp_def: 157,
        init: 171,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn magnemite() -> Pokemon {
    return Pokemon {
        name: "Magnemite",
        ftype: Type::Electric,
        stype: Type::None,
        health: 160,
        atk: 67,
        def: 130,
        sp_atk: 175,
        sp_def: 103,
        init: 85,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn magneton() -> Pokemon {
    return Pokemon {
        name: "Magneton",
        ftype: Type::Electric,
        stype: Type::None,
        health: 210,
        atk: 112,
        def: 175,
        sp_atk: 220,
        sp_def: 130,
        init: 130,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn mankey() -> Pokemon {
    return Pokemon {
        name: "Mankey",
        ftype: Type::Fighting,
        stype: Type::None,
        health: 190,
        atk: 148,
        def: 67,
        sp_atk: 67,
        sp_def: 85,
        init: 130,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn marowak() -> Pokemon {
    return Pokemon {
        name: "Marowak",
        ftype: Type::Ground,
        stype: Type::None,
        health: 230,
        atk: 148,
        def: 202,
        sp_atk: 94,
        sp_def: 148,
        init: 85,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn meowth() -> Pokemon {
    return Pokemon {
        name: "Meowth",
        ftype: Type::Normal,
        stype: Type::None,
        health: 190,
        atk: 85,
        def: 67,
        sp_atk: 76,
        sp_def: 76,
        init: 166,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn metapod() -> Pokemon {
    return Pokemon {
        name: "Metapod",
        ftype: Type::Bug,
        stype: Type::None,
        health: 210,
        atk: 40,
        def: 103,
        sp_atk: 49,
        sp_def: 49,
        init: 58,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn mew() -> Pokemon {
    return Pokemon {
        name: "Mew",
        ftype: Type::Psychic,
        stype: Type::None,
        health: 310,
        atk: 184,
        def: 184,
        sp_atk: 184,
        sp_def: 184,
        init: 184,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn mewtwo() -> Pokemon {
    return Pokemon {
        name: "Mewtwo",
        ftype: Type::Psychic,
        stype: Type::None,
        health: 322,
        atk: 202,
        def: 166,
        sp_atk: 281,
        sp_def: 166,
        init: 238,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn moltres() -> Pokemon {
    return Pokemon {
        name: "Moltres",
        ftype: Type::Fire,
        stype: Type::Flying,
        health: 290,
        atk: 184,
        def: 166,
        sp_atk: 229,
        sp_def: 157,
        init: 166,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn mrmime() -> Pokemon {
    return Pokemon {
        name: "Mr.Mime",
        ftype: Type::Psychic,
        stype: Type::None,
        health: 190,
        atk: 85,
        def: 121,
        sp_atk: 184,
        sp_def: 220,
        init: 166,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn muk() -> Pokemon {
    return Pokemon {
        name: "Muk",
        ftype: Type::Poison,
        stype: Type::None,
        health: 320,
        atk: 193,
        def: 139,
        sp_atk: 121,
        sp_def: 184,
        init: 94,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn nidoking() -> Pokemon {
    return Pokemon {
        name: "Nidoking",
        ftype: Type::Poison,
        stype: Type::Ground,
        health: 272,
        atk: 170,
        def: 143,
        sp_atk: 157,
        sp_def: 139,
        init: 157,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn nidoqueen() -> Pokemon {
    return Pokemon {
        name: "Nidoqueen",
        ftype: Type::Poison,
        stype: Type::Ground,
        health: 290,
        atk: 152,
        def: 161,
        sp_atk: 139,
        sp_def: 157,
        init: 141,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn nidoranf() -> Pokemon {
    return Pokemon {
        name: "Nidoran",
        ftype: Type::Poison,
        stype: Type::None,
        health: 220,
        atk: 89,
        def: 98,
        sp_atk: 76,
        sp_def: 76,
        init: 78,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn nidoranm() -> Pokemon {
    return Pokemon {
        name: "Nidoran",
        ftype: Type::Poison,
        stype: Type::None,
        health: 202,
        atk: 107,
        def: 76,
        sp_atk: 76,
        sp_def: 76,
        init: 94,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn nidorina() -> Pokemon {
    return Pokemon {
        name: "Nidorina",
        ftype: Type::Poison,
        stype: Type::None,
        health: 250,
        atk: 116,
        def: 125,
        sp_atk: 103,
        sp_def: 103,
        init: 105,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn nidorino() -> Pokemon {
    return Pokemon {
        name: "Nidorino",
        ftype: Type::Poison,
        stype: Type::None,
        health: 232,
        atk: 134,
        def: 107,
        sp_atk: 103,
        sp_def: 103,
        init: 121,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn ninetales() -> Pokemon {
    return Pokemon {
        name: "Ninetales",
        ftype: Type::Fire,
        stype: Type::None,
        health: 256,
        atk: 141,
        def: 139,
        sp_atk: 150,
        sp_def: 184,
        init: 184,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn oddish() -> Pokemon {
    return Pokemon {
        name: "Oddish",
        ftype: Type::Grass,
        stype: Type::Poison,
        health: 200,
        atk: 94,
        def: 103,
        sp_atk: 139,
        sp_def: 121,
        init: 58,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn omanyte() -> Pokemon {
    return Pokemon {
        name: "Omanyte",
        ftype: Type::Rock,
        stype: Type::Water,
        health: 180,
        atk: 76,
        def: 184,
        sp_atk: 166,
        sp_def: 103,
        init: 67,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn omastar() -> Pokemon {
    return Pokemon {
        name: "Omastar",
        ftype: Type::Rock,
        stype: Type::Water,
        health: 250,
        atk: 112,
        def: 229,
        sp_atk: 211,
        sp_def: 130,
        init: 103,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn onix() -> Pokemon {
    return Pokemon {
        name: "Onix",
        ftype: Type::Rock,
        stype: Type::Ground,
        health: 180,
        atk: 85,
        def: 292,
        sp_atk: 58,
        sp_def: 85,
        init: 130,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn paras() -> Pokemon {
    return Pokemon {
        name: "Paras",
        ftype: Type::Bug,
        stype: Type::Grass,
        health: 180,
        atk: 130,
        def: 103,
        sp_atk: 85,
        sp_def: 103,
        init: 49,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn parasect() -> Pokemon {
    return Pokemon {
        name: "Parasect",
        ftype: Type::Bug,
        stype: Type::Grass,
        health: 230,
        atk: 175,
        def: 148,
        sp_atk: 112,
        sp_def: 148,
        init: 58,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn persian() -> Pokemon {
    return Pokemon {
        name: "Persian",
        ftype: Type::Normal,
        stype: Type::None,
        health: 240,
        atk: 130,
        def: 112,
        sp_atk: 121,
        sp_def: 121,
        init: 211,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn pidgeot() -> Pokemon {
    return Pokemon {
        name: "Pidgeot",
        ftype: Type::Normal,
        stype: Type::Flying,
        health: 276,
        atk: 148,
        def: 139,
        sp_atk: 130,
        sp_def: 130,
        init: 168,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn pidgeotto() -> Pokemon {
    return Pokemon {
        name: "Pidgeotto",
        ftype: Type::Normal,
        stype: Type::Flying,
        health: 236,
        atk: 112,
        def: 103,
        sp_atk: 94,
        sp_def: 94,
        init: 132,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn pidgey() -> Pokemon {
    return Pokemon {
        name: "Pidgey",
        ftype: Type::Normal,
        stype: Type::Flying,
        health: 190,
        atk: 85,
        def: 76,
        sp_atk: 67,
        sp_def: 67,
        init: 105,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn pikachu() -> Pokemon {
    return Pokemon {
        name: "Pikachu",
        ftype: Type::Electric,
        stype: Type::None,
        health: 180,
        atk: 103,
        def: 58,
        sp_atk: 94,
        sp_def: 76,
        init: 166,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn pinsir() -> Pokemon {
    return Pokemon {
        name: "Pinsir",
        ftype: Type::Bug,
        stype: Type::None,
        health: 240,
        atk: 229,
        def: 184,
        sp_atk: 103,
        sp_def: 130,
        init: 157,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn poliwag() -> Pokemon {
    return Pokemon {
        name: "Poliwag",
        ftype: Type::Water,
        stype: Type::None,
        health: 190,
        atk: 94,
        def: 76,
        sp_atk: 76,
        sp_def: 76,
        init: 166,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn poliwhirl() -> Pokemon {
    return Pokemon {
        name: "Poliwhirl",
        ftype: Type::Water,
        stype: Type::None,
        health: 240,
        atk: 121,
        def: 121,
        sp_atk: 94,
        sp_def: 94,
        init: 166,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn poliwrath() -> Pokemon {
    return Pokemon {
        name: "Poliwrath",
        ftype: Type::Water,
        stype: Type::Fighting,
        health: 290,
        atk: 157,
        def: 175,
        sp_atk: 130,
        sp_def: 166,
        init: 130,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn ponyta() -> Pokemon {
    return Pokemon {
        name: "Ponyta",
        ftype: Type::Fire,
        stype: Type::None,
        health: 210,
        atk: 157,
        def: 103,
        sp_atk: 121,
        sp_def: 121,
        init: 166,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn porygon() -> Pokemon {
    return Pokemon {
        name: "Porygon",
        ftype: Type::Normal,
        stype: Type::None,
        health: 240,
        atk: 112,
        def: 130,
        sp_atk: 157,
        sp_def: 139,
        init: 76,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn primeape() -> Pokemon {
    return Pokemon {
        name: "Primeape",
        ftype: Type::Fighting,
        stype: Type::None,
        health: 240,
        atk: 193,
        def: 112,
        sp_atk: 112,
        sp_def: 130,
        init: 175,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn psyduck() -> Pokemon {
    return Pokemon {
        name: "Parasect",
        ftype: Type::Water,
        stype: Type::None,
        health: 210,
        atk: 98,
        def: 90,
        sp_atk: 121,
        sp_def: 94,
        init: 103,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn raichu() -> Pokemon {
    return Pokemon {
        name: "Raichu",
        ftype: Type::Electric,
        stype: Type::None,
        health: 230,
        atk: 166,
        def: 103,
        sp_atk: 166,
        sp_def: 148,
        init: 184,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn rapidash() -> Pokemon {
    return Pokemon {
        name: "Rapidash",
        ftype: Type::Fire,
        stype: Type::None,
        health: 240,
        atk: 184,
        def: 130,
        sp_atk: 148,
        sp_def: 148,
        init: 193,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn raticate() -> Pokemon {
    return Pokemon {
        name: "Raticate",
        ftype: Type::Normal,
        stype: Type::None,
        health: 220,
        atk: 150,
        def: 112,
        sp_atk: 94,
        sp_def: 130,
        init: 179,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn rattata() -> Pokemon {
    return Pokemon {
        name: "Rattata",
        ftype: Type::Normal,
        stype: Type::None,
        health: 170,
        atk: 105,
        def: 67,
        sp_atk: 49,
        sp_def: 67,
        init: 134,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn rhydon() -> Pokemon {
    return Pokemon {
        name: "Rhydon",
        ftype: Type::Ground,
        stype: Type::Rock,
        health: 320,
        atk: 238,
        def: 220,
        sp_atk: 85,
        sp_def: 85,
        init: 76,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn rhyhorn() -> Pokemon {
    return Pokemon {
        name: "Rhyhorn",
        ftype: Type::Ground,
        stype: Type::Rock,
        health: 270,
        atk: 157,
        def: 175,
        sp_atk: 58,
        sp_def: 58,
        init: 49,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn sandshrew() -> Pokemon {
    return Pokemon {
        name: "Sandshrew",
        ftype: Type::Ground,
        stype: Type::None,
        health: 210,
        atk: 139,
        def: 157,
        sp_atk: 40,
        sp_def: 58,
        init: 76,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn sandslash() -> Pokemon {
    return Pokemon {
        name: "Sandslash",
        ftype: Type::Ground,
        stype: Type::None,
        health: 260,
        atk: 184,
        def: 202,
        sp_atk: 85,
        sp_def: 103,
        init: 121,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn scyther() -> Pokemon {
    return Pokemon {
        name: "Scyther",
        ftype: Type::Bug,
        stype: Type::Flying,
        health: 250,
        atk: 202,
        def: 148,
        sp_atk: 103,
        sp_def: 148,
        init: 193,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn seadra() -> Pokemon {
    return Pokemon {
        name: "Seadra",
        ftype: Type::Water,
        stype: Type::None,
        health: 220,
        atk: 121,
        def: 175,
        sp_atk: 175,
        sp_def: 85,
        init: 157,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn seaking() -> Pokemon {
    return Pokemon {
        name: "Seaking",
        ftype: Type::Water,
        stype: Type::None,
        health: 270,
        atk: 170,
        def: 121,
        sp_atk: 121,
        sp_def: 148,
        init: 126,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn seel() -> Pokemon {
    return Pokemon {
        name: "Seel",
        ftype: Type::Water,
        stype: Type::None,
        health: 240,
        atk: 85,
        def: 103,
        sp_atk: 85,
        sp_def: 130,
        init: 85,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn shellder() -> Pokemon {
    return Pokemon {
        name: "Shellder",
        ftype: Type::Water,
        stype: Type::None,
        health: 170,
        atk: 121,
        def: 184,
        sp_atk: 85,
        sp_def: 49,
        init: 76,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn slowbro() -> Pokemon {
    return Pokemon {
        name: "Slowbro",
        ftype: Type::Water,
        stype: Type::Psychic,
        health: 300,
        atk: 139,
        def: 202,
        sp_atk: 184,
        sp_def: 148,
        init: 58,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn slowpoke() -> Pokemon {
    return Pokemon {
        name: "Slowpoke",
        ftype: Type::Water,
        stype: Type::Psychic,
        health: 290,
        atk: 121,
        def: 121,
        sp_atk: 76,
        sp_def: 76,
        init: 31,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn snorlax() -> Pokemon {
    return Pokemon {
        name: "Snorlax",
        ftype: Type::Normal,
        stype: Type::None,
        health: 430,
        atk: 202,
        def: 121,
        sp_atk: 121,
        sp_def: 202,
        init: 58,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn spearow() -> Pokemon {
    return Pokemon {
        name: "Spearow",
        ftype: Type::Normal,
        stype: Type::Flying,
        health: 190,
        atk: 112,
        def: 58,
        sp_atk: 60,
        sp_def: 60,
        init: 130,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn squirtle() -> Pokemon {
    return Pokemon {
        name: "Squirtle",
        ftype: Type::Water,
        stype: Type::None,
        health: 198,
        atk: 90,
        def: 121,
        sp_atk: 94,
        sp_def: 119,
        init: 81,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn starmie() -> Pokemon {
    return Pokemon {
        name: "Starmie",
        ftype: Type::Water,
        stype: Type::Psychic,
        health: 230,
        atk: 139,
        def: 157,
        sp_atk: 184,
        sp_def: 157,
        init: 211,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn staryu() -> Pokemon {
    return Pokemon {
        name: "Staryu",
        ftype: Type::Water,
        stype: Type::None,
        health: 170,
        atk: 85,
        def: 103,
        sp_atk: 130,
        sp_def: 103,
        init: 157,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn tangela() -> Pokemon {
    return Pokemon {
        name: "Tangela",
        ftype: Type::Grass,
        stype: Type::None,
        health: 240,
        atk: 103,
        def: 211,
        sp_atk: 184,
        sp_def: 76,
        init: 112,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn tauros() -> Pokemon {
    return Pokemon {
        name: "Tauros",
        ftype: Type::Normal,
        stype: Type::None,
        health: 260,
        atk: 184,
        def: 175,
        sp_atk: 76,
        sp_def: 130,
        init: 202,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn tentacool() -> Pokemon {
    return Pokemon {
        name: "Tentacool",
        ftype: Type::Water,
        stype: Type::Poison,
        health: 190,
        atk: 76,
        def: 67,
        sp_atk: 94,
        sp_def: 184,
        init: 130,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn tentacruel() -> Pokemon {
    return Pokemon {
        name: "Tentacruel",
        ftype: Type::Water,
        stype: Type::Poison,
        health: 270,
        atk: 130,
        def: 121,
        sp_atk: 148,
        sp_def: 220,
        init: 184,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn vaporeon() -> Pokemon {
    return Pokemon {
        name: "Vaporeon",
        ftype: Type::Water,
        stype: Type::None,
        health: 370,
        atk: 121,
        def: 112,
        sp_atk: 202,
        sp_def: 175,
        init: 121,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn venomoth() -> Pokemon {
    return Pokemon {
        name: "Venomoth",
        ftype: Type::Bug,
        stype: Type::Poison,
        health: 250,
        atk: 121,
        def: 112,
        sp_atk: 166,
        sp_def: 139,
        init: 166,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn venonat() -> Pokemon {
    return Pokemon {
        name: "Venonat",
        ftype: Type::Bug,
        stype: Type::Poison,
        health: 230,
        atk: 103,
        def: 94,
        sp_atk: 76,
        sp_def: 103,
        init: 85,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn venusaur() -> Pokemon {
    return Pokemon {
        name: "Venusaur",
        ftype: Type::Grass,
        stype: Type::Poison,
        health: 270,
        atk: 152,
        def: 153,
        sp_atk: 184,
        sp_def: 184,
        init: 148,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn victreebel() -> Pokemon {
    return Pokemon {
        name: "Victreble",
        ftype: Type::Grass,
        stype: Type::Poison,
        health: 270,
        atk: 193,
        def: 121,
        sp_atk: 184,
        sp_def: 112,
        init: 130,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn vileplume() -> Pokemon {
    return Pokemon {
        name: "Vileplume",
        ftype: Type::Grass,
        stype: Type::Poison,
        health: 260,
        atk: 148,
        def: 157,
        sp_atk: 184,
        sp_def: 166,
        init: 94,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn voltorb() -> Pokemon {
    return Pokemon {
        name: "Voltorb",
        ftype: Type::Electric,
        stype: Type::None,
        health: 190,
        atk: 58,
        def: 94,
        sp_atk: 103,
        sp_def: 103,
        init: 184,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn vulpix() -> Pokemon {
    return Pokemon {
        name: "Vulpix",
        ftype: Type::Fire,
        stype: Type::None,
        health: 186,
        atk: 78,
        def: 76,
        sp_atk: 94,
        sp_def: 121,
        init: 121,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn wartortle() -> Pokemon {
    return Pokemon {
        name: "Wartortle",
        ftype: Type::Water,
        stype: Type::None,
        health: 228,
        atk: 117,
        def: 148,
        sp_atk: 121,
        sp_def: 148,
        init: 108,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn weedle() -> Pokemon {
    return Pokemon {
        name: "Weedle",
        ftype: Type::Bug,
        stype: Type::Poison,
        health: 190,
        atk: 67,
        def: 58,
        sp_atk: 40,
        sp_def: 40,
        init: 94,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn weepinbell() -> Pokemon {
    return Pokemon {
        name: "Weepinbell",
        ftype: Type::Grass,
        stype: Type::Poison,
        health: 240,
        atk: 166,
        def: 94,
        sp_atk: 157,
        sp_def: 85,
        init: 103,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn weezing() -> Pokemon {
    return Pokemon {
        name: "Weezing",
        ftype: Type::Poison,
        stype: Type::None,
        health: 240,
        atk: 166,
        def: 220,
        sp_atk: 157,
        sp_def: 130,
        init: 112,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn wigglystuff() -> Pokemon {
    return Pokemon {
        name: "Wigglystuff",
        ftype: Type::Normal,
        stype: Type::None,
        health: 390,
        atk: 130,
        def: 85,
        sp_atk: 139,
        sp_def: 94,
        init: 85,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn zaptos() -> Pokemon {
    return Pokemon {
        name: "Zaptos",
        ftype: Type::Electric,
        stype: Type::Flying,
        health: 290,
        atk: 166,
        def: 157,
        sp_atk: 229,
        sp_def: 166,
        init: 184,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}

pub fn zubat() -> Pokemon {
    return Pokemon {
        name: "Zubat",
        ftype: Type::Poison,
        stype: Type::Flying,
        health: 190,
        atk: 85,
        def: 67,
        sp_atk: 58,
        sp_def: 76,
        init: 103,
        m1: attacks::tackle(),
        m2: attacks::dummy(),
        m3: attacks::dummy(),
        m4: attacks::dummy(),
    }
}
