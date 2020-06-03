
use crate::default_structures::{Type, attacks};
use std::fmt::{Display, Result, Formatter};

#[derive(Copy, Clone, PartialEq, Eq, std::hash::Hash)]
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
    pub moves: [attacks::Attack; 4]
}

impl Display for Pokemon {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}\n{} | {}\n{:<10}{}\n{:<10}{}\n{:<10}{}\n{:<10}{}\n{:<10}{}\n{:<10}{}\n", 
            self.name, self.ftype, self.stype, "hp", self.health, "atk", self.atk, 
            "def", self.def, "sp atk", self.sp_atk, "sp def", self.sp_def, "init", self.init)
    }
}

impl Pokemon {
    //TODO add functions
}

pub fn dummy_pokemon() -> Pokemon {
    Pokemon {
        name: "Dummy",
        ftype: Type::None,
        stype: Type::None,
        health: 0,
        atk: 0,
        def: 0,
        sp_atk: 0,
        sp_def: 0,
        init: 0,
        moves: [attacks::dummy(); 4]
    }
}

pub fn abra() -> Pokemon {
    Pokemon {
        name: "Abra",
        ftype: Type::Psychic,
        stype: Type::None,
        health: 160,
        atk: 40,
        def: 31,
        sp_atk: 193,
        sp_def: 103,
        init: 166,
        moves: [attacks::dummy(); 4]  
    }
}

pub fn aerodactyl() -> Pokemon {
    Pokemon {
        name: "Aerodactyl",
        ftype: Type::Rock,
        stype: Type::Flying,
        health: 270,
        atk: 193,
        def: 121,
        sp_atk: 112,
        sp_def: 139,
        init: 238,
        moves: [attacks::dummy(); 4]
    }
}

pub fn alakazam() -> Pokemon {
    Pokemon {
        name: "Alakazam",
        ftype: Type::Psychic,
        stype: Type::None,
        health: 220,
        atk: 94,
        def: 85,
        sp_atk: 247,
        sp_def: 157,
        init: 220,
        moves: [attacks::dummy(); 4] 
    }
}

pub fn arbok() -> Pokemon {
    Pokemon {
        name: "Arbok",
        ftype: Type::Poison,
        stype: Type::None,
        health: 230,
        atk: 157,
        def: 128,
        sp_atk: 121,
        sp_def: 146,
        init: 148,
        moves: [attacks::dummy(); 4]  
    }
}

pub fn arcanine() -> Pokemon {
    Pokemon {
        name: "Arcanine",
        ftype: Type::Fire,
        stype: Type::None,
        health: 290,
        atk: 202,
        def: 148,
        sp_atk: 184,
        sp_def: 148,
        init: 175,
        moves: [attacks::dummy(); 4]  
    }
}

pub fn articuno() -> Pokemon {
    Pokemon {
        name: "Articuno",
        ftype: Type::Ice,
        stype: Type::Flying,
        health: 290,
        atk: 157,
        def: 184,
        sp_atk: 175,
        sp_def: 229,
        init: 157,
        moves: [attacks::dummy(); 4]   
    }
}

pub fn beedrill() -> Pokemon {
    Pokemon {
        name: "Beedrill",
        ftype: Type::Bug,
        stype: Type::Poison,
        health: 240,
        atk: 148,
        def: 76,
        sp_atk: 85,
        sp_def: 148,
        init: 139,
        moves: [attacks::dummy(); 4] 
    }
}

pub fn bellsprout() -> Pokemon {
    Pokemon {
        name: "Bellsprout",
        ftype: Type::Grass,
        stype: Type::Poison,
        health: 210,
        atk: 139,
        def: 67,
        sp_atk: 130,
        sp_def: 58,
        init: 76,
        moves: [attacks::dummy(); 4] 
    }
}

pub fn blastoise() -> Pokemon {
    Pokemon {
        name: "Blastoise",
        ftype: Type::Water,
        stype: Type::None,
        health: 268,
        atk: 153,
        def: 184,
        sp_atk: 157,
        sp_def: 193,
        init: 144,
        moves: [attacks::dummy(); 4]
    }
}

pub fn bulbasur() -> Pokemon {
    Pokemon {
        name: "Bulbasur",
        ftype: Type::Grass,
        stype: Type::Poison,
        health: 200,
        atk: 92,
        def: 90,
        sp_atk: 121,
        sp_def: 121,
        init: 85,
        moves: [attacks::dummy(); 4] 
    }
}

pub fn butterfree() -> Pokemon {
    Pokemon {
        name: "Butterfree",
        ftype: Type::Bug,
        stype: Type::Flying,
        health: 230,
        atk: 85,
        def: 94,
        sp_atk: 148,
        sp_def: 148,
        init: 130,
        moves: [attacks::dummy(); 4]
    }
}

pub fn caterpie() -> Pokemon {
    Pokemon {
        name: "Caterpie",
        ftype: Type::Bug,
        stype: Type::None,
        health: 200,
        atk: 58,
        def: 67,
        sp_atk: 40,
        sp_def: 40,
        init: 85,
        moves: [attacks::dummy(); 4]
    }
}

pub fn chansey() -> Pokemon {
    Pokemon {
        name: "Chansey",
        ftype: Type::Normal,
        stype: Type::None,
        health: 610,
        atk: 13,
        def: 13,
        sp_atk: 67,
        sp_def: 193,
        init: 94,
        moves: [attacks::dummy(); 4] 
    }
}

pub fn charizard() -> Pokemon {
    Pokemon {
        name: "Charizard",
        ftype: Type::Fire,
        stype: Type::Flying,
        health: 266,
        atk: 155,
        def: 144,
        sp_atk: 200,
        sp_def: 157,
        init: 184,
        moves: [attacks::dummy(); 4]
    }
}

pub fn charmander() -> Pokemon {
    Pokemon {
        name: "Charmander",
        ftype: Type::Fire,
        stype: Type::None,
        health: 188,
        atk: 98,
        def: 81,
        sp_atk: 112,
        sp_def: 94,
        init: 121,
        moves: [attacks::dummy(); 4]
    }
}

pub fn charmaleon() -> Pokemon {
    Pokemon {
        name: "Charmaleon",
        ftype: Type::Fire,
        stype: Type::None,
        health: 226,
        atk: 119,
        def: 108,
        sp_atk: 148,
        sp_def: 121,
        init: 148,
        moves: [attacks::dummy(); 4]
    }
}

pub fn clefable() -> Pokemon {
    Pokemon {
        name: "Clefable",
        ftype: Type::Normal,
        stype: Type::None,
        health: 300,
        atk: 130,
        def: 135,
        sp_atk: 157,
        sp_def: 166,
        init: 112,
        moves: [attacks::dummy(); 4]
    }
}

pub fn clefairy() -> Pokemon {
    Pokemon {
        name: "Clefairy",
        ftype: Type::Normal,
        stype: Type::None,
        health: 250,
        atk: 85,
        def: 90,
        sp_atk: 112,
        sp_def: 121,
        init: 67,
        moves: [attacks::dummy(); 4]
    }
}

pub fn cloyster() -> Pokemon {
    Pokemon {
        name: "Cloyster",
        ftype: Type::Water,
        stype: Type::Ice,
        health: 210,
        atk: 175,
        def: 328,
        sp_atk: 157,
        sp_def: 85,
        init: 130,
        moves: [attacks::dummy(); 4] 
    }
}

pub fn cubone() -> Pokemon {
    Pokemon {
        name: "Cubone",
        ftype: Type::Ground,
        stype: Type::None,
        health: 210,
        atk: 94,
        def: 175,
        sp_atk: 76,
        sp_def: 94,
        init: 67,
        moves: [attacks::dummy(); 4]
    }
}

pub fn dewgong() -> Pokemon {
    Pokemon {
        name: "Dewgong",
        ftype: Type::Water,
        stype: Type::Ice,
        health: 290,
        atk: 130,
        def: 148,
        sp_atk: 130,
        sp_def: 175,
        init: 130,
        moves: [attacks::dummy(); 4]
    }
}
pub fn diglett() -> Pokemon {
    Pokemon {
        name: "Diglett",
        ftype: Type::Ground,
        stype: Type::None,
        health: 130,
        atk: 103,
        def: 49,
        sp_atk: 67,
        sp_def: 85,
        init: 175,
        moves: [attacks::dummy(); 4]
    }
}

pub fn ditto() -> Pokemon {
    Pokemon {
        name: "Ditto",
        ftype: Type::Normal,
        stype: Type::None,
        health: 206,
        atk: 90,
        def: 90,
        sp_atk: 90,
        sp_def: 90,
        init: 90,
        moves: [attacks::dummy(); 4]
    }
}

pub fn dodrio() -> Pokemon {
    Pokemon {
        name: "Dodrio",
        ftype: Type::Normal,
        stype: Type::Flying,
        health: 230,
        atk: 202,
        def: 130,
        sp_atk: 112,
        sp_def: 112,
        init: 184,
        moves: [attacks::dummy(); 4]
    }
}

pub fn doduo() -> Pokemon {
    Pokemon {
        name: "Doduo",
        ftype: Type::Normal,
        stype: Type::Flying,
        health: 180,
        atk: 157,
        def: 85,
        sp_atk: 67,
        sp_def: 67,
        init: 139,
        moves: [attacks::dummy(); 4]
    }
}

pub fn dragonair() -> Pokemon {
    Pokemon {
        name: "Dragonair",
        ftype: Type::Dragon,
        stype: Type::None,
        health: 232,
        atk: 155,
        def: 121,
        sp_atk: 130,
        sp_def: 130,
        init: 130,
        moves: [attacks::dummy(); 4]
    }
}

pub fn dragonite() -> Pokemon {
    Pokemon {
        name: "Dragonite",
        ftype: Type::Dragon,
        stype: Type::Flying,
        health: 292,
        atk: 245,
        def: 175,
        sp_atk: 184,
        sp_def: 184,
        init: 148,
        moves: [attacks::dummy(); 4]
    }
}

pub fn dratini() -> Pokemon {
    Pokemon {
        name: "Dratini",
        ftype: Type::Dragon,
        stype: Type::None,
        health: 192,
        atk: 119,
        def: 85,
        sp_atk: 94,
        sp_def: 94,
        init: 94,
        moves: [attacks::dummy(); 4]
    }
}

pub fn drowzee() -> Pokemon {
    Pokemon {
        name: "Drowzee",
        ftype: Type::Psychic,
        stype: Type::None,
        health: 230,
        atk: 90,
        def: 85,
        sp_atk: 81,
        sp_def: 166,
        init: 80,
        moves: [attacks::dummy(); 4]
    }
}

pub fn dugtrio() -> Pokemon {
    Pokemon {
        name: "Dugtrio",
        ftype: Type::Ground,
        stype: Type::None,
        health: 180,
        atk: 148,
        def: 94,
        sp_atk: 94,
        sp_def: 130,
        init: 220,
        moves: [attacks::dummy(); 4]
    }
}

pub fn eevee() -> Pokemon {
    Pokemon {
        name: "Eevee",
        ftype: Type::Normal,
        stype: Type::None,
        health: 220,
        atk: 103,
        def: 94,
        sp_atk: 85,
        sp_def: 121,
        init: 103,
        moves: [attacks::dummy(); 4]
    }
}

pub fn ekans() -> Pokemon {
    Pokemon {
        name: "Ekans",
        ftype: Type::Poison,
        stype: Type::None,
        health: 180,
        atk: 112,
        def: 83,
        sp_atk: 76,
        sp_def: 101,
        init: 103,
        moves: [attacks::dummy(); 4]
    }
}

pub fn electabuzz() -> Pokemon {
    Pokemon {
        name: "Electabuzz",
        ftype: Type::Electric,
        stype: Type::None,
        health: 240,
        atk: 153,
        def: 107,
        sp_atk: 175,
        sp_def: 157,
        init: 193,
        moves: [attacks::dummy(); 4]
    }
}

pub fn electrode() -> Pokemon {
    Pokemon {
        name: "Electrode",
        ftype: Type::Electric,
        stype: Type::None,
        health: 230,
        atk: 94,
        def: 130,
        sp_atk: 148,
        sp_def: 148,
        init:256 ,
        moves: [attacks::dummy(); 4]
    }
}

pub fn exeggcute() -> Pokemon {
    Pokemon {
        name: "Exeggcute",
        ftype: Type::Grass,
        stype: Type::Psychic,
        health: 230,
        atk: 76,
        def: 148,
        sp_atk: 112,
        sp_def: 85,
        init: 76,
        moves: [attacks::dummy(); 4] 
    }
}

pub fn exeggcutor() -> Pokemon {
    Pokemon {
        name: "Exeggcutor",
        ftype: Type::Grass,
        stype: Type::Psychic,
        health: 300,
        atk: 175,
        def: 157,
        sp_atk: 229,
        sp_def: 121,
        init: 103,
        moves: [attacks::dummy(); 4]
    }
}

pub fn farfetch() -> Pokemon {
    Pokemon {
        name: "Farfetch",
        ftype: Type::Normal,
        stype: Type::Flying,
        health: 214,
        atk: 121,
        def: 103,
        sp_atk: 108,
        sp_def: 116,
        init: 112,
        moves: [attacks::dummy(); 4]
    }
}

pub fn fearow() -> Pokemon {
    Pokemon {
        name: "Fearow",
        ftype: Type::Normal,
        stype: Type::Flying,
        health: 240,
        atk: 166,
        def: 121,
        sp_atk: 114,
        sp_def: 114,
        init: 184,
        moves: [attacks::dummy(); 4] 
    }
}

pub fn flareon() -> Pokemon {
    Pokemon {
        name: "Flareon",
        ftype: Type::Fire,
        stype: Type::None,
        health: 240,
        atk: 238,
        def: 112,
        sp_atk: 175,
        sp_def: 202,
        init: 121,
        moves: [attacks::dummy(); 4]
    }
}

pub fn gastly() -> Pokemon {
    Pokemon {
        name: "Gastly",
        ftype: Type::Ghost,
        stype: Type::Poison,
        health: 170,
        atk: 67,
        def: 58,
        sp_atk: 184,
        sp_def: 67,
        init: 148,
        moves: [attacks::dummy(); 4]
    }
}

pub fn gengar() -> Pokemon {
    Pokemon {
        name: "Gengar",
        ftype: Type::Ghost,
        stype: Type::Poison,
        health: 230,
        atk: 121,
        def: 112,
        sp_atk: 238,
        sp_def: 139,
        init: 202,
        moves: [attacks::dummy(); 4]
    }
}

pub fn geodude() -> Pokemon {
    Pokemon {
        name: "Geodude",
        ftype: Type::Rock,
        stype: Type::Ground,
        health: 190,
        atk: 148,
        def: 184,
        sp_atk: 58,
        sp_def: 58,
        init: 40,
        moves: [attacks::dummy(); 4]
    }
}

pub fn gloom() -> Pokemon {
    Pokemon {
        name: "Gloom",
        ftype: Type::Grass,
        stype: Type::Poison,
        health: 230,
        atk: 121,
        def: 130,
        sp_atk: 157,
        sp_def: 139,
        init: 76,
        moves: [attacks::dummy(); 4]
    }
}

pub fn golbat() -> Pokemon {
    Pokemon {
        name: "Golbat",
        ftype: Type::Poison,
        stype: Type::Flying,
        health: 260,
        atk: 148,
        def: 130,
        sp_atk: 121,
        sp_def: 139,
        init: 166,
        moves: [attacks::dummy(); 4]
    }
}

pub fn goldeen() -> Pokemon {
    Pokemon {
        name: "Goldeen",
        ftype: Type::Water,
        stype: Type::None,
        health: 200,
        atk: 125,
        def: 112,
        sp_atk: 67,
        sp_def: 94,
        init: 117,
        moves: [attacks::dummy(); 4]
    }
}

pub fn golduck() -> Pokemon {
    Pokemon {
        name: "Golduck",
        ftype: Type::Water,
        stype: Type::None,
        health: 270,
        atk: 152,
        def: 144,
        sp_atk: 175,
        sp_def: 148,
        init: 157,
        moves: [attacks::dummy(); 4]
    }
}

pub fn golem() -> Pokemon {
    Pokemon {
        name: "Golem",
        ftype: Type::Rock,
        stype: Type::Ground,
        health: 270,
        atk: 202,
        def: 238,
        sp_atk: 103,
        sp_def: 121,
        init: 85,
        moves: [attacks::dummy(); 4]
    }
}

pub fn graveler() -> Pokemon {
    Pokemon {
        name: "Graveler",
        ftype: Type::Rock,
        stype: Type::Ground,
        health: 220,
        atk: 175,
        def: 211,
        sp_atk: 85,
        sp_def: 85,
        init: 67,
        moves: [attacks::dummy(); 4]
    }
}

pub fn grimer() -> Pokemon {
    Pokemon {
        name: "Grimer",
        ftype: Type::Poison,
        stype: Type::None,
        health: 270,
        atk: 148,
        def: 94,
        sp_atk: 76,
        sp_def: 94,
        init: 49,
        moves: [attacks::dummy(); 4]
    }
}

pub fn growlithe() -> Pokemon {
    Pokemon {
        name: "Growlithe",
        ftype: Type::Fire,
        stype: Type::None,
        health: 220,
        atk: 130,
        def: 85,
        sp_atk: 130,
        sp_def: 94,
        init: 112,
        moves: [attacks::dummy(); 4]
    }
}

pub fn gyarados() -> Pokemon {
    Pokemon {
        name: "Gyarados",
        ftype: Type::Water,
        stype: Type::Flying,
        health: 300,
        atk: 229,
        def: 146,
        sp_atk: 112,
        sp_def: 184,
        init: 150,
        moves: [attacks::dummy(); 4]
    }
}

pub fn hauter() -> Pokemon {
    Pokemon {
        name: "Hauter",
        ftype: Type::Ghost,
        stype: Type::Poison,
        health: 200,
        atk: 94,
        def: 85,
        sp_atk: 211,
        sp_def: 103,
        init: 175,
        moves: [attacks::dummy(); 4]
    }
}

pub fn hitmonchan() -> Pokemon {
    Pokemon {
        name: "Hitmonchan",
        ftype: Type::Fighting,
        stype: Type::None,
        health: 210,
        atk: 193,
        def: 146,
        sp_atk: 67,
        sp_def: 202,
        init: 141,
        moves: [attacks::dummy(); 4]
    }
}

pub fn hitmonlee() -> Pokemon {
    Pokemon {
        name: "Hitmonlee",
        ftype: Type::Fighting,
        stype: Type::None,
        health: 210,
        atk: 220,
        def: 99,
        sp_atk: 67,
        sp_def: 202,
        init: 161,
        moves: [attacks::dummy(); 4]
    }
}

pub fn horsea() -> Pokemon {
    Pokemon {
        name: "Horsea",
        ftype: Type::Water,
        stype: Type::None,
        health: 170,
        atk: 76,
        def: 130,
        sp_atk: 130,
        sp_def: 49,
        init: 112,
        moves: [attacks::dummy(); 4]
    }
}

pub fn hypno() -> Pokemon {
    Pokemon {
        name: "Hypno",
        ftype: Type::Psychic,
        stype: Type::None,
        health: 280,
        atk: 135,
        def: 130,
        sp_atk: 135,
        sp_def: 211,
        init: 125,
        moves: [attacks::dummy(); 4]
    }
}

pub fn ivysaur() -> Pokemon {
    Pokemon {
        name: "Ivysaur",
        ftype: Type::Grass,
        stype: Type::Poison,
        health: 230,
        atk: 116,
        def: 117,
        sp_atk: 148,
        sp_def: 148,
        init: 112,
        moves: [attacks::dummy(); 4]
    }
}

pub fn jigglypuff() -> Pokemon {
    Pokemon {
        name: "jigglypuff",
        ftype: Type::Normal,
        stype: Type::None,
        health: 340,
        atk: 85,
        def: 40,
        sp_atk: 85,
        sp_def: 49,
        init: 40,
        moves: [attacks::dummy(); 4]
    }
}

pub fn jolteon() -> Pokemon {
    Pokemon {
        name: "Jolteon",
        ftype: Type::Electric,
        stype: Type::None,
        health: 240,
        atk: 121,
        def: 112,
        sp_atk: 202,
        sp_def: 175,
        init: 238,
        moves: [attacks::dummy(); 4]
    }
}

pub fn jynx() -> Pokemon {
    Pokemon {
        name: "Jynx",
        ftype: Type::Ice,
        stype: Type::Psychic,
        health: 240,
        atk: 94,
        def: 67,
        sp_atk: 211,
        sp_def: 175,
        init: 175,
        moves: [attacks::dummy(); 4]
    }
}

pub fn kabuto() -> Pokemon {
    Pokemon {
        name: "Kabuto",
        ftype: Type::Rock,
        stype: Type::Water,
        health: 170,
        atk: 148,
        def: 166,
        sp_atk: 103,
        sp_def: 85,
        init: 103,
        moves: [attacks::dummy(); 4]
    }
}

pub fn kabutops() -> Pokemon {
    Pokemon {
        name: "Kabutops",
        ftype: Type::Rock,
        stype: Type::Water,
        health: 230,
        atk: 211,
        def: 193,
        sp_atk: 121,
        sp_def: 130,
        init: 148,
        moves: [attacks::dummy(); 4]
    }
}

pub fn kadabra() -> Pokemon {
    Pokemon {
        name: "Bulbasur",
        ftype: Type::Psychic,
        stype: Type::None,
        health: 190,
        atk: 67,
        def: 58,
        sp_atk: 220,
        sp_def: 130,
        init: 193,
        moves: [attacks::dummy(); 4]
    }
}

pub fn kakuna() -> Pokemon {
    Pokemon {
        name: "Kakuna",
        ftype: Type::Bug,
        stype: Type::Poison,
        health: 200,
        atk: 49,
        def: 94,
        sp_atk: 49,
        sp_def: 49,
        init: 67,
        moves: [attacks::dummy(); 4]
    }
}

pub fn kangaskhan() -> Pokemon {
    Pokemon {
        name: "Kangaskhan",
        ftype: Type::Normal,
        stype: Type::None,
        health: 320,
        atk: 175,
        def: 148,
        sp_atk: 76,
        sp_def: 148,
        init: 166,
        moves: [attacks::dummy(); 4]
    }
}

pub fn kingler() -> Pokemon {
    Pokemon {
        name: "Kingler",
        ftype: Type::Water,
        stype: Type::None,
        health: 220,
        atk: 238,
        def: 211,
        sp_atk: 94,
        sp_def: 94,
        init: 139,
        moves: [attacks::dummy(); 4]
    }
}

pub fn koffing() -> Pokemon {
    Pokemon {
        name: "Koffing",
        ftype: Type::Poison,
        stype: Type::None,
        health: 190,
        atk: 121,
        def: 175,
        sp_atk: 112,
        sp_def: 85,
        init: 67,
        moves: [attacks::dummy(); 4]
    }
}

pub fn krabby() -> Pokemon {
    Pokemon {
        name: "Krabby",
        ftype: Type::Water,
        stype: Type::None,
        health: 170,
        atk: 193,
        def: 166,
        sp_atk: 49,
        sp_def: 49,
        init: 94,
        moves: [attacks::dummy(); 4]
    }
}

pub fn lapras() -> Pokemon {
    Pokemon {
        name: "Lapras",
        ftype: Type::Water,
        stype: Type::Ice,
        health: 370,
        atk: 157,
        def: 148,
        sp_atk: 157,
        sp_def: 175,
        init: 112,
        moves: [attacks::dummy(); 4]
    }
}

pub fn lickitung() -> Pokemon {
    Pokemon {
        name: "Lickitung",
        ftype: Type::Normal,
        stype: Type::None,
        health: 290,
        atk: 103,
        def: 139,
        sp_atk: 112,
        sp_def: 139,
        init: 58,
        moves: [attacks::dummy(); 4]
    }
}

pub fn machamp() -> Pokemon {
    Pokemon {
        name: "Machamp",
        ftype: Type::Fighting,
        stype: Type::None,
        health: 290,
        atk: 238,
        def: 148,
        sp_atk: 121,
        sp_def: 157,
        init: 103,
        moves: [attacks::dummy(); 4]
    }
}

pub fn machoke() -> Pokemon {
    Pokemon {
        name: "Machoke",
        ftype: Type::Fighting,
        stype: Type::None,
        health: 270,
        atk: 184,
        def: 130,
        sp_atk: 94,
        sp_def: 112,
        init: 85,
        moves: [attacks::dummy(); 4]
    }
}

pub fn machop() -> Pokemon {
    Pokemon {
        name: "Machop",
        ftype: Type::Fighting,
        stype: Type::None,
        health: 250,
        atk: 148,
        def: 94,
        sp_atk: 67,
        sp_def: 67,
        init: 67,
        moves: [attacks::dummy(); 4]
        
        
        
    }
}

pub fn magikarp() -> Pokemon {
    Pokemon {
        name: "Magikarp",
        ftype: Type::Water,
        stype: Type::None,
        health: 150,
        atk: 22,
        def: 103,
        sp_atk: 31,
        sp_def: 40,
        init: 148,
        moves: [attacks::dummy(); 4]
    }
}

pub fn magmar() -> Pokemon {
    Pokemon {
        name: "Magmar",
        ftype: Type::Fire,
        stype: Type::None,
        health: 240,
        atk: 175,
        def: 107,
        sp_atk: 184,
        sp_def: 157,
        init: 171,
        moves: [attacks::dummy(); 4]
    }
}

pub fn magnemite() -> Pokemon {
    Pokemon {
        name: "Magnemite",
        ftype: Type::Electric,
        stype: Type::None,
        health: 160,
        atk: 67,
        def: 130,
        sp_atk: 175,
        sp_def: 103,
        init: 85,
        moves: [attacks::dummy(); 4]
    }
}

pub fn magneton() -> Pokemon {
    Pokemon {
        name: "Magneton",
        ftype: Type::Electric,
        stype: Type::None,
        health: 210,
        atk: 112,
        def: 175,
        sp_atk: 220,
        sp_def: 130,
        init: 130,
        moves: [attacks::dummy(); 4]
    }
}

pub fn mankey() -> Pokemon {
    Pokemon {
        name: "Mankey",
        ftype: Type::Fighting,
        stype: Type::None,
        health: 190,
        atk: 148,
        def: 67,
        sp_atk: 67,
        sp_def: 85,
        init: 130,
        moves: [attacks::dummy(); 4]
    }
}

pub fn marowak() -> Pokemon {
    Pokemon {
        name: "Marowak",
        ftype: Type::Ground,
        stype: Type::None,
        health: 230,
        atk: 148,
        def: 202,
        sp_atk: 94,
        sp_def: 148,
        init: 85,
        moves: [attacks::dummy(); 4]
    }
}

pub fn meowth() -> Pokemon {
    Pokemon {
        name: "Meowth",
        ftype: Type::Normal,
        stype: Type::None,
        health: 190,
        atk: 85,
        def: 67,
        sp_atk: 76,
        sp_def: 76,
        init: 166,
        moves: [attacks::dummy(); 4]
    }
}

pub fn metapod() -> Pokemon {
    Pokemon {
        name: "Metapod",
        ftype: Type::Bug,
        stype: Type::None,
        health: 210,
        atk: 40,
        def: 103,
        sp_atk: 49,
        sp_def: 49,
        init: 58,
        moves: [attacks::dummy(); 4]
    }
}

pub fn mew() -> Pokemon {
    Pokemon {
        name: "Mew",
        ftype: Type::Psychic,
        stype: Type::None,
        health: 310,
        atk: 184,
        def: 184,
        sp_atk: 184,
        sp_def: 184,
        init: 184,
        moves: [attacks::dummy(); 4] 
    }
}

pub fn mewtwo() -> Pokemon {
    Pokemon {
        name: "Mewtwo",
        ftype: Type::Psychic,
        stype: Type::None,
        health: 322,
        atk: 202,
        def: 166,
        sp_atk: 281,
        sp_def: 166,
        init: 238,
        moves: [attacks::dummy(); 4]
    }
}

pub fn moltres() -> Pokemon {
    Pokemon {
        name: "Moltres",
        ftype: Type::Fire,
        stype: Type::Flying,
        health: 290,
        atk: 184,
        def: 166,
        sp_atk: 229,
        sp_def: 157,
        init: 166,
        moves: [attacks::dummy(); 4]
    }
}

pub fn mrmime() -> Pokemon {
    Pokemon {
        name: "Mr.Mime",
        ftype: Type::Psychic,
        stype: Type::None,
        health: 190,
        atk: 85,
        def: 121,
        sp_atk: 184,
        sp_def: 220,
        init: 166,
        moves: [attacks::dummy(); 4]
    }
}

pub fn muk() -> Pokemon {
    Pokemon {
        name: "Muk",
        ftype: Type::Poison,
        stype: Type::None,
        health: 320,
        atk: 193,
        def: 139,
        sp_atk: 121,
        sp_def: 184,
        init: 94,
        moves: [attacks::dummy(); 4] 
    }
}

pub fn nidoking() -> Pokemon {
    Pokemon {
        name: "Nidoking",
        ftype: Type::Poison,
        stype: Type::Ground,
        health: 272,
        atk: 170,
        def: 143,
        sp_atk: 157,
        sp_def: 139,
        init: 157,
        moves: [attacks::dummy(); 4]  
    }
}

pub fn nidoqueen() -> Pokemon {
    Pokemon {
        name: "Nidoqueen",
        ftype: Type::Poison,
        stype: Type::Ground,
        health: 290,
        atk: 152,
        def: 161,
        sp_atk: 139,
        sp_def: 157,
        init: 141,
        moves: [attacks::dummy(); 4]
    }
}

pub fn nidoranf() -> Pokemon {
    Pokemon {
        name: "Nidoran",
        ftype: Type::Poison,
        stype: Type::None,
        health: 220,
        atk: 89,
        def: 98,
        sp_atk: 76,
        sp_def: 76,
        init: 78,
        moves: [attacks::dummy(); 4]
    }
}

pub fn nidoranm() -> Pokemon {
    Pokemon {
        name: "Nidoran",
        ftype: Type::Poison,
        stype: Type::None,
        health: 202,
        atk: 107,
        def: 76,
        sp_atk: 76,
        sp_def: 76,
        init: 94,
        moves: [attacks::dummy(); 4]
    }
}

pub fn nidorina() -> Pokemon {
    Pokemon {
        name: "Nidorina",
        ftype: Type::Poison,
        stype: Type::None,
        health: 250,
        atk: 116,
        def: 125,
        sp_atk: 103,
        sp_def: 103,
        init: 105,
        moves: [attacks::dummy(); 4]
    }
}

pub fn nidorino() -> Pokemon {
    Pokemon {
        name: "Nidorino",
        ftype: Type::Poison,
        stype: Type::None,
        health: 232,
        atk: 134,
        def: 107,
        sp_atk: 103,
        sp_def: 103,
        init: 121,
        moves: [attacks::dummy(); 4]
    }
}

pub fn ninetales() -> Pokemon {
    Pokemon {
        name: "Ninetales",
        ftype: Type::Fire,
        stype: Type::None,
        health: 256,
        atk: 141,
        def: 139,
        sp_atk: 150,
        sp_def: 184,
        init: 184,
        moves: [attacks::dummy(); 4]
    }
}

pub fn oddish() -> Pokemon {
    Pokemon {
        name: "Oddish",
        ftype: Type::Grass,
        stype: Type::Poison,
        health: 200,
        atk: 94,
        def: 103,
        sp_atk: 139,
        sp_def: 121,
        init: 58,
        moves: [attacks::dummy(); 4]
    }
}

pub fn omanyte() -> Pokemon {
    Pokemon {
        name: "Omanyte",
        ftype: Type::Rock,
        stype: Type::Water,
        health: 180,
        atk: 76,
        def: 184,
        sp_atk: 166,
        sp_def: 103,
        init: 67,
        moves: [attacks::dummy(); 4]
    }
}

pub fn omastar() -> Pokemon {
    Pokemon {
        name: "Omastar",
        ftype: Type::Rock,
        stype: Type::Water,
        health: 250,
        atk: 112,
        def: 229,
        sp_atk: 211,
        sp_def: 130,
        init: 103,
        moves: [attacks::dummy(); 4]
    }
}

pub fn onix() -> Pokemon {
    Pokemon {
        name: "Onix",
        ftype: Type::Rock,
        stype: Type::Ground,
        health: 180,
        atk: 85,
        def: 292,
        sp_atk: 58,
        sp_def: 85,
        init: 130,
        moves: [attacks::dummy(); 4]
    }
}

pub fn paras() -> Pokemon {
    Pokemon {
        name: "Paras",
        ftype: Type::Bug,
        stype: Type::Grass,
        health: 180,
        atk: 130,
        def: 103,
        sp_atk: 85,
        sp_def: 103,
        init: 49,
        moves: [attacks::dummy(); 4]
    }
}

pub fn parasect() -> Pokemon {
    Pokemon {
        name: "Parasect",
        ftype: Type::Bug,
        stype: Type::Grass,
        health: 230,
        atk: 175,
        def: 148,
        sp_atk: 112,
        sp_def: 148,
        init: 58,
        moves: [attacks::dummy(); 4]
    }
}

pub fn persian() -> Pokemon {
    Pokemon {
        name: "Persian",
        ftype: Type::Normal,
        stype: Type::None,
        health: 240,
        atk: 130,
        def: 112,
        sp_atk: 121,
        sp_def: 121,
        init: 211,
        moves: [attacks::dummy(); 4]
    }
}

pub fn pidgeot() -> Pokemon {
    Pokemon {
        name: "Pidgeot",
        ftype: Type::Normal,
        stype: Type::Flying,
        health: 276,
        atk: 148,
        def: 139,
        sp_atk: 130,
        sp_def: 130,
        init: 168,
        moves: [attacks::dummy(); 4]
    }
}

pub fn pidgeotto() -> Pokemon {
    Pokemon {
        name: "Pidgeotto",
        ftype: Type::Normal,
        stype: Type::Flying,
        health: 236,
        atk: 112,
        def: 103,
        sp_atk: 94,
        sp_def: 94,
        init: 132,
        moves: [attacks::dummy(); 4]
    }
}

pub fn pidgey() -> Pokemon {
    Pokemon {
        name: "Pidgey",
        ftype: Type::Normal,
        stype: Type::Flying,
        health: 190,
        atk: 85,
        def: 76,
        sp_atk: 67,
        sp_def: 67,
        init: 105,
        moves: [attacks::dummy(); 4]
    }
}

pub fn pikachu() -> Pokemon {
    Pokemon {
        name: "Pikachu",
        ftype: Type::Electric,
        stype: Type::None,
        health: 180,
        atk: 103,
        def: 58,
        sp_atk: 94,
        sp_def: 76,
        init: 166,
        moves: [attacks::dummy(); 4]
    }
}

pub fn pinsir() -> Pokemon {
    Pokemon {
        name: "Pinsir",
        ftype: Type::Bug,
        stype: Type::None,
        health: 240,
        atk: 229,
        def: 184,
        sp_atk: 103,
        sp_def: 130,
        init: 157,
        moves: [attacks::dummy(); 4] 
    }
}

pub fn poliwag() -> Pokemon {
    Pokemon {
        name: "Poliwag",
        ftype: Type::Water,
        stype: Type::None,
        health: 190,
        atk: 94,
        def: 76,
        sp_atk: 76,
        sp_def: 76,
        init: 166,
        moves: [attacks::dummy(); 4]
    }
}

pub fn poliwhirl() -> Pokemon {
    Pokemon {
        name: "Poliwhirl",
        ftype: Type::Water,
        stype: Type::None,
        health: 240,
        atk: 121,
        def: 121,
        sp_atk: 94,
        sp_def: 94,
        init: 166,
        moves: [attacks::dummy(); 4]
    }
}

pub fn poliwrath() -> Pokemon {
    Pokemon {
        name: "Poliwrath",
        ftype: Type::Water,
        stype: Type::Fighting,
        health: 290,
        atk: 157,
        def: 175,
        sp_atk: 130,
        sp_def: 166,
        init: 130,
        moves: [attacks::dummy(); 4]
    }
}

pub fn ponyta() -> Pokemon {
    Pokemon {
        name: "Ponyta",
        ftype: Type::Fire,
        stype: Type::None,
        health: 210,
        atk: 157,
        def: 103,
        sp_atk: 121,
        sp_def: 121,
        init: 166,
        moves: [attacks::dummy(); 4]
    }
}

pub fn porygon() -> Pokemon {
    Pokemon {
        name: "Porygon",
        ftype: Type::Normal,
        stype: Type::None,
        health: 240,
        atk: 112,
        def: 130,
        sp_atk: 157,
        sp_def: 139,
        init: 76,
        moves: [attacks::dummy(); 4]
    }
}

pub fn primeape() -> Pokemon {
    Pokemon {
        name: "Primeape",
        ftype: Type::Fighting,
        stype: Type::None,
        health: 240,
        atk: 193,
        def: 112,
        sp_atk: 112,
        sp_def: 130,
        init: 175,
        moves: [attacks::dummy(); 4]
    }
}

pub fn psyduck() -> Pokemon {
    Pokemon {
        name: "Parasect",
        ftype: Type::Water,
        stype: Type::None,
        health: 210,
        atk: 98,
        def: 90,
        sp_atk: 121,
        sp_def: 94,
        init: 103,
        moves: [attacks::dummy(); 4]
    }
}

pub fn raichu() -> Pokemon {
    Pokemon {
        name: "Raichu",
        ftype: Type::Electric,
        stype: Type::None,
        health: 230,
        atk: 166,
        def: 103,
        sp_atk: 166,
        sp_def: 148,
        init: 184,
        moves: [attacks::dummy(); 4]
    }
}

pub fn rapidash() -> Pokemon {
    Pokemon {
        name: "Rapidash",
        ftype: Type::Fire,
        stype: Type::None,
        health: 240,
        atk: 184,
        def: 130,
        sp_atk: 148,
        sp_def: 148,
        init: 193,
        moves: [attacks::dummy(); 4]
    }
}

pub fn raticate() -> Pokemon {
    Pokemon {
        name: "Raticate",
        ftype: Type::Normal,
        stype: Type::None,
        health: 220,
        atk: 150,
        def: 112,
        sp_atk: 94,
        sp_def: 130,
        init: 179,
        moves: [attacks::dummy(); 4]
    }
}

pub fn rattata() -> Pokemon {
    Pokemon {
        name: "Rattata",
        ftype: Type::Normal,
        stype: Type::None,
        health: 170,
        atk: 105,
        def: 67,
        sp_atk: 49,
        sp_def: 67,
        init: 134,
        moves: [attacks::dummy(); 4]
    }
}

pub fn rhydon() -> Pokemon {
    Pokemon {
        name: "Rhydon",
        ftype: Type::Ground,
        stype: Type::Rock,
        health: 320,
        atk: 238,
        def: 220,
        sp_atk: 85,
        sp_def: 85,
        init: 76,
        moves: [attacks::dummy(); 4]
    }
}

pub fn rhyhorn() -> Pokemon {
    Pokemon {
        name: "Rhyhorn",
        ftype: Type::Ground,
        stype: Type::Rock,
        health: 270,
        atk: 157,
        def: 175,
        sp_atk: 58,
        sp_def: 58,
        init: 49,
        moves: [attacks::dummy(); 4]
    }
}

pub fn sandshrew() -> Pokemon {
    Pokemon {
        name: "Sandshrew",
        ftype: Type::Ground,
        stype: Type::None,
        health: 210,
        atk: 139,
        def: 157,
        sp_atk: 40,
        sp_def: 58,
        init: 76,
        moves: [attacks::dummy(); 4]
    }
}

pub fn sandslash() -> Pokemon {
    Pokemon {
        name: "Sandslash",
        ftype: Type::Ground,
        stype: Type::None,
        health: 260,
        atk: 184,
        def: 202,
        sp_atk: 85,
        sp_def: 103,
        init: 121,
        moves: [attacks::dummy(); 4]
    }
}

pub fn scyther() -> Pokemon {
    Pokemon {
        name: "Scyther",
        ftype: Type::Bug,
        stype: Type::Flying,
        health: 250,
        atk: 202,
        def: 148,
        sp_atk: 103,
        sp_def: 148,
        init: 193,
        moves: [attacks::dummy(); 4]
    }
}

pub fn seadra() -> Pokemon {
    Pokemon {
        name: "Seadra",
        ftype: Type::Water,
        stype: Type::None,
        health: 220,
        atk: 121,
        def: 175,
        sp_atk: 175,
        sp_def: 85,
        init: 157,
        moves: [attacks::dummy(); 4]
    }
}

pub fn seaking() -> Pokemon {
    Pokemon {
        name: "Seaking",
        ftype: Type::Water,
        stype: Type::None,
        health: 270,
        atk: 170,
        def: 121,
        sp_atk: 121,
        sp_def: 148,
        init: 126,
        moves: [attacks::dummy(); 4]
    }
}

pub fn seel() -> Pokemon {
    Pokemon {
        name: "Seel",
        ftype: Type::Water,
        stype: Type::None,
        health: 240,
        atk: 85,
        def: 103,
        sp_atk: 85,
        sp_def: 130,
        init: 85,
        moves: [attacks::dummy(); 4]
    }
}

pub fn shellder() -> Pokemon {
    Pokemon {
        name: "Shellder",
        ftype: Type::Water,
        stype: Type::None,
        health: 170,
        atk: 121,
        def: 184,
        sp_atk: 85,
        sp_def: 49,
        init: 76,
        moves: [attacks::dummy(); 4]
    }
}

pub fn slowbro() -> Pokemon {
    Pokemon {
        name: "Slowbro",
        ftype: Type::Water,
        stype: Type::Psychic,
        health: 300,
        atk: 139,
        def: 202,
        sp_atk: 184,
        sp_def: 148,
        init: 58,
        moves: [attacks::dummy(); 4]
    }
}

pub fn slowpoke() -> Pokemon {
    Pokemon {
        name: "Slowpoke",
        ftype: Type::Water,
        stype: Type::Psychic,
        health: 290,
        atk: 121,
        def: 121,
        sp_atk: 76,
        sp_def: 76,
        init: 31,
        moves: [attacks::dummy(); 4]
    }
}

pub fn snorlax() -> Pokemon {
    Pokemon {
        name: "Snorlax",
        ftype: Type::Normal,
        stype: Type::None,
        health: 430,
        atk: 202,
        def: 121,
        sp_atk: 121,
        sp_def: 202,
        init: 58,
        moves: [attacks::dummy(); 4]
    }
}

pub fn spearow() -> Pokemon {
    Pokemon {
        name: "Spearow",
        ftype: Type::Normal,
        stype: Type::Flying,
        health: 190,
        atk: 112,
        def: 58,
        sp_atk: 60,
        sp_def: 60,
        init: 130,
        moves: [attacks::dummy(); 4]
    }
}

pub fn squirtle() -> Pokemon {
    Pokemon {
        name: "Squirtle",
        ftype: Type::Water,
        stype: Type::None,
        health: 198,
        atk: 90,
        def: 121,
        sp_atk: 94,
        sp_def: 119,
        init: 81,
        moves: [attacks::dummy(); 4]
    }
}

pub fn starmie() -> Pokemon {
    Pokemon {
        name: "Starmie",
        ftype: Type::Water,
        stype: Type::Psychic,
        health: 230,
        atk: 139,
        def: 157,
        sp_atk: 184,
        sp_def: 157,
        init: 211,
        moves: [attacks::dummy(); 4]
    }
}

pub fn staryu() -> Pokemon {
    Pokemon {
        name: "Staryu",
        ftype: Type::Water,
        stype: Type::None,
        health: 170,
        atk: 85,
        def: 103,
        sp_atk: 130,
        sp_def: 103,
        init: 157,
        moves: [attacks::dummy(); 4]
    }
}

pub fn tangela() -> Pokemon {
    Pokemon {
        name: "Tangela",
        ftype: Type::Grass,
        stype: Type::None,
        health: 240,
        atk: 103,
        def: 211,
        sp_atk: 184,
        sp_def: 76,
        init: 112,
        moves: [attacks::dummy(); 4]
    }
}

pub fn tauros() -> Pokemon {
    Pokemon {
        name: "Tauros",
        ftype: Type::Normal,
        stype: Type::None,
        health: 260,
        atk: 184,
        def: 175,
        sp_atk: 76,
        sp_def: 130,
        init: 202,
        moves: [attacks::dummy(); 4]
    }
}

pub fn tentacool() -> Pokemon {
    Pokemon {
        name: "Tentacool",
        ftype: Type::Water,
        stype: Type::Poison,
        health: 190,
        atk: 76,
        def: 67,
        sp_atk: 94,
        sp_def: 184,
        init: 130,
        moves: [attacks::dummy(); 4]
    }
}

pub fn tentacruel() -> Pokemon {
    Pokemon {
        name: "Tentacruel",
        ftype: Type::Water,
        stype: Type::Poison,
        health: 270,
        atk: 130,
        def: 121,
        sp_atk: 148,
        sp_def: 220,
        init: 184,
        moves: [attacks::dummy(); 4]
    }
}

pub fn vaporeon() -> Pokemon {
    Pokemon {
        name: "Vaporeon",
        ftype: Type::Water,
        stype: Type::None,
        health: 370,
        atk: 121,
        def: 112,
        sp_atk: 202,
        sp_def: 175,
        init: 121,
        moves: [attacks::dummy(); 4]
    }
}

pub fn venomoth() -> Pokemon {
    Pokemon {
        name: "Venomoth",
        ftype: Type::Bug,
        stype: Type::Poison,
        health: 250,
        atk: 121,
        def: 112,
        sp_atk: 166,
        sp_def: 139,
        init: 166,
        moves: [attacks::dummy(); 4]
    }
}

pub fn venonat() -> Pokemon {
    Pokemon {
        name: "Venonat",
        ftype: Type::Bug,
        stype: Type::Poison,
        health: 230,
        atk: 103,
        def: 94,
        sp_atk: 76,
        sp_def: 103,
        init: 85,
        moves: [attacks::dummy(); 4]
    }
}

pub fn venusaur() -> Pokemon {
    Pokemon {
        name: "Venusaur",
        ftype: Type::Grass,
        stype: Type::Poison,
        health: 270,
        atk: 152,
        def: 153,
        sp_atk: 184,
        sp_def: 184,
        init: 148,
        moves: [attacks::dummy(); 4]
    }
}

pub fn victreebel() -> Pokemon {
    Pokemon {
        name: "Victreble",
        ftype: Type::Grass,
        stype: Type::Poison,
        health: 270,
        atk: 193,
        def: 121,
        sp_atk: 184,
        sp_def: 112,
        init: 130,
        moves: [attacks::dummy(); 4]
    }
}

pub fn vileplume() -> Pokemon {
    Pokemon {
        name: "Vileplume",
        ftype: Type::Grass,
        stype: Type::Poison,
        health: 260,
        atk: 148,
        def: 157,
        sp_atk: 184,
        sp_def: 166,
        init: 94,
        moves: [attacks::dummy(); 4]
    }
}

pub fn voltorb() -> Pokemon {
    Pokemon {
        name: "Voltorb",
        ftype: Type::Electric,
        stype: Type::None,
        health: 190,
        atk: 58,
        def: 94,
        sp_atk: 103,
        sp_def: 103,
        init: 184,
        moves: [attacks::dummy(); 4]
    }
}

pub fn vulpix() -> Pokemon {
    Pokemon {
        name: "Vulpix",
        ftype: Type::Fire,
        stype: Type::None,
        health: 186,
        atk: 78,
        def: 76,
        sp_atk: 94,
        sp_def: 121,
        init: 121,
        moves: [attacks::dummy(); 4]
    }
}

pub fn wartortle() -> Pokemon {
    Pokemon {
        name: "Wartortle",
        ftype: Type::Water,
        stype: Type::None,
        health: 228,
        atk: 117,
        def: 148,
        sp_atk: 121,
        sp_def: 148,
        init: 108,
        moves: [attacks::dummy(); 4]
    }
}

pub fn weedle() -> Pokemon {
    Pokemon {
        name: "Weedle",
        ftype: Type::Bug,
        stype: Type::Poison,
        health: 190,
        atk: 67,
        def: 58,
        sp_atk: 40,
        sp_def: 40,
        init: 94,
        moves: [attacks::dummy(); 4]
    }
}

pub fn weepinbell() -> Pokemon {
    Pokemon {
        name: "Weepinbell",
        ftype: Type::Grass,
        stype: Type::Poison,
        health: 240,
        atk: 166,
        def: 94,
        sp_atk: 157,
        sp_def: 85,
        init: 103,
        moves: [attacks::dummy(); 4]
    }
}

pub fn weezing() -> Pokemon {
    Pokemon {
        name: "Weezing",
        ftype: Type::Poison,
        stype: Type::None,
        health: 240,
        atk: 166,
        def: 220,
        sp_atk: 157,
        sp_def: 130,
        init: 112,
        moves: [attacks::dummy(); 4]
    }
}

pub fn wigglystuff() -> Pokemon {
    Pokemon {
        name: "Wigglystuff",
        ftype: Type::Normal,
        stype: Type::None,
        health: 390,
        atk: 130,
        def: 85,
        sp_atk: 139,
        sp_def: 94,
        init: 85,
        moves: [attacks::dummy(); 4]
    }
}

pub fn zaptos() -> Pokemon {
    Pokemon {
        name: "Zaptos",
        ftype: Type::Electric,
        stype: Type::Flying,
        health: 290,
        atk: 166,
        def: 157,
        sp_atk: 229,
        sp_def: 166,
        init: 184,
        moves: [attacks::dummy(); 4]
    }
}

pub fn zubat() -> Pokemon {
    Pokemon {
        name: "Zubat",
        ftype: Type::Poison,
        stype: Type::Flying,
        health: 190,
        atk: 85,
        def: 67,
        sp_atk: 58,
        sp_def: 76,
        init: 103,
        moves: [attacks::dummy(); 4]
    }
}
