
use crate::default_structures::{Type, attacks};
use crate::game_assets::PokemonAssets;
use std::fmt::{Display, Result, Formatter};
use std::hash::{Hash, Hasher};
use ggez::{Context, GameResult};

#[derive(Clone)]
pub struct Pokemon {
    pub name: &'static str,
    pub ftype: Type,
    pub stype: Type,
    pub health: u32,
    pub atk: u32,
    pub def: u32,
    pub sp_atk: u32,
    pub sp_def: u32,
    pub init: u32,
    pub moves: [attacks::Attack; 4], 
    pub sprite_path: &'static str,
    pub battle_cry_path: &'static str
}

impl Display for Pokemon {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}\n{} | {}\n{:<10}{}\n{:<10}{}\n{:<10}{}\n{:<10}{}\n{:<10}{}\n{:<10}{}\n", 
            self.name, self.ftype, self.stype, "hp", self.health, "atk", self.atk, 
            "def", self.def, "sp atk", self.sp_atk, "sp def", self.sp_def, "init", self.init)
    }
}

impl PartialEq for Pokemon {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Pokemon {}

impl Hash for Pokemon {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Pokemon {
    pub fn load_assets(&self, ctx: &mut Context) -> GameResult<PokemonAssets> {
        PokemonAssets::new(ctx, self.battle_cry_path, self.sprite_path)
    }
}

pub fn dummy_pokemon() -> Pokemon {
    Pokemon {
        name: "Dummy",
        ftype: Type::None,
        stype: Type::None,
        health: 100,
        atk: 10,
        def: 10,
        sp_atk: 10,
        sp_def: 10,
        init: 10,
        moves: [attacks::dummy(); 4],
        battle_cry_path: "",
        sprite_path: ""
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/063 - Abra.wav",
        sprite_path: "/sprites/63.png"  
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/142 - Aerodactyl.wav",
        sprite_path: "/sprites/142.png" 
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/065 - Alakazam.wav",
        sprite_path: "/sprites/65.png" 
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/024 - Arbok.wav",
        sprite_path: "/sprites/24.png" 
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/059 - Arcanine.wav",
        sprite_path: "/sprites/59.png" 
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/144 - Articuno.wav",
        sprite_path: "/sprites/144.png" 
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/015 - Beedrill.wav",
        sprite_path: "/sprites/15.png" 
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/069 - Bellsprout.wav",
        sprite_path: "/sprites/69.png" 
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/009 - Blastoise.wav",
        sprite_path: "/sprites/9.png" 
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/001 - bulbasaur.wav",
        sprite_path: "/sprites/1.png" 
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/012 - Butterfree.wav",
        sprite_path: "/sprites/12.png" 
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/010 - Caterpie.wav",
        sprite_path: "/sprites/10.png" 
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/113 - Chansey.wav",
        sprite_path: "/sprites/113.png" 
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/006 - Charizard.wav",
        sprite_path: "/sprites/6.png" 
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/004 - Charmander.wav",
        sprite_path: "/sprites/4.png" 
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/005 - Charmaleon.wav",
        sprite_path: "/sprites/5.png" 
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/036 - Clefable.wav",
        sprite_path: "/sprites/36.png" 
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/035 - Clefairy.wav",
        sprite_path: "/sprites/35.png" 
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/091 - Cloyster.wav",
        sprite_path: "/sprites/91.png" 
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/104 - Cubone.wav",
        sprite_path: "/sprites/104.png" 
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/087 - Dewgong.wav",
        sprite_path: "/sprites/87.png" 
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/050 - Diglett.wav",
        sprite_path: "/sprites/50.png" 
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/132 - Ditto.wav",
        sprite_path: "/sprites/132.png" 
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/085 - Dodrio.wav",
        sprite_path: "/sprites/85.png" 
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/084 - Doduo.wav",
        sprite_path: "/sprites/84.png" 
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/148 - Dragonair.wav",
        sprite_path: "/sprites/148.png" 
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/149 - Dragonite.wav",
        sprite_path: "/sprites/149.png" 
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/147 - Dratini.wav",
        sprite_path: "/sprites/147.png" 
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/096 - Drowzee.wav",
        sprite_path: "/sprites/96.png" 
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/051 - Dugtrio.wav",
        sprite_path: "/sprites/51.png" 
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/133 - Eevee.wav",
        sprite_path: "/sprites/133.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/023 - Ekans.wav",
        sprite_path: "/sprites/23.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/125 - Electabuzz.wav",
        sprite_path: "/sprites/125.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/101 - Electrode.wav",
        sprite_path: "/sprites/101.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/102 - Exeggcute.wav",
        sprite_path: "/sprites/102.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/103 - Exeggcutor.wav",
        sprite_path: "/sprites/103.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/083 - Farfetch'd.wav",
        sprite_path: "/sprites/83.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/022 - Fearow.wav",
        sprite_path: "/sprites/22.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/136 - Flareon.wav",
        sprite_path: "/sprites/136.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/092 - Gastly.wav",
        sprite_path: "/sprites/92.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/094 - Gengar.wav",
        sprite_path: "/sprites/94.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/074 - Geodude.wav",
        sprite_path: "/sprites/74.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/044 - Gloom.wav",
        sprite_path: "/sprites/44.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/042 - Golbat.wav",
        sprite_path: "/sprites/42.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/118 - Goldeen.wav",
        sprite_path: "/sprites/118.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/055 - Golduck.wav",
        sprite_path: "/sprites/55.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/076 - Golem.wav",
        sprite_path: "/sprites/76.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/075 - Graveler.wav",
        sprite_path: "/sprites/75.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/088 - Grimer.wav",
        sprite_path: "/sprites/88.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/058 - Growlithe.wav",
        sprite_path: "/sprites/58.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/130 - Gyarados.wav",
        sprite_path: "/sprites/130.png"
    }
}

pub fn haunter() -> Pokemon {
    Pokemon {
        name: "Haunter",
        ftype: Type::Ghost,
        stype: Type::Poison,
        health: 200,
        atk: 94,
        def: 85,
        sp_atk: 211,
        sp_def: 103,
        init: 175,
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/093 - Haunter.wav",
        sprite_path: "/sprites/93.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/107 - Hitmonchan.wav",
        sprite_path: "/sprites/107.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/106 - Hitmonlee.wav",
        sprite_path: "/sprites/106.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/116 - Horsea.wav",
        sprite_path: "/sprites/116.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/097 - Hypno.wav",
        sprite_path: "/sprites/97.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/002 - Ivysaur.wav",
        sprite_path: "/sprites/2.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/039 - Jigglypuff.wav",
        sprite_path: "/sprites/39.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/135 - Jolteon.wav",
        sprite_path: "/sprites/135.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/124 - Jynx.wav",
        sprite_path: "/sprites/124.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/140 - Kabuto.wav",
        sprite_path: "/sprites/140.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/141 - Kabutops.wav",
        sprite_path: "/sprites/141.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/064 - Kadabra.wav",
        sprite_path: "/sprites/64.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/014 - Kakuna.wav",
        sprite_path: "/sprites/14.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/115 - Kangaskhan.wav",
        sprite_path: "/sprites/115.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/099 - Kingler.wav",
        sprite_path: "/sprites/99.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/109 - Koffing.wav",
        sprite_path: "/sprites/109.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/098 - Krabby.wav",
        sprite_path: "/sprites/98.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/131 - Lapras.wav",
        sprite_path: "/sprites/131.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/108 - Lickitung.wav",
        sprite_path: "/sprites/108.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/068 - Machamp.wav",
        sprite_path: "/sprites/68.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/067 - Machoke.wav",
        sprite_path: "/sprites/67.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/066 - Machop.wav",
        sprite_path: "/sprites/66.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/129 - Magikarp.wav",
        sprite_path: "/sprites/129.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/126 - Magmar.wav",
        sprite_path: "/sprites/126.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/081 - Magnemite.wav",
        sprite_path: "/sprites/81.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/082 - Magneton.wav",
        sprite_path: "/sprites/82.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/056 - Mankey.wav",
        sprite_path: "/sprites/56.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/105 - Marowak.wav",
        sprite_path: "/sprites/105.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/052 - Meowth.wav",
        sprite_path: "/sprites/52.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/011 - Metapod.wav",
        sprite_path: "/sprites/11.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/151 - Mew.wav",
        sprite_path: "/sprites/151.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/150 - Mewtwo.wav",
        sprite_path: "/sprites/150.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/146 - Moltres.wav",
        sprite_path: "/sprites/146.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/122 - Mr. Mime.wav",
        sprite_path: "/sprites/122.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/089 - Muk.wav",
        sprite_path: "/sprites/89.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/034 - Nidoking.wav",
        sprite_path: "/sprites/34.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/031 - Nidoqueen.wav",
        sprite_path: "/sprites/31.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/029 - Nidoran (Female).wav",
        sprite_path: "/sprites/29.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/032 - Nidoran (Male).wav",
        sprite_path: "/sprites/32.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/030 - Nidorina.wav",
        sprite_path: "/sprites/30.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/033 - Nidorino.wav",
        sprite_path: "/sprites/33.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/038 - Ninetales.wav",
        sprite_path: "/sprites/38.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/043 - Oddish.wav",
        sprite_path: "/sprites/43.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/138 - Omanyte.wav",
        sprite_path: "/sprites/138.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/139 - Amoroso.wav",
        sprite_path: "/sprites/139.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/095 - Onix.wav",
        sprite_path: "/sprites/95.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/046 - Paras.wav",
        sprite_path: "/sprites/46.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/047 - Parasect.wav",
        sprite_path: "/sprites/47.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/053 - Persian.wav",
        sprite_path: "/sprites/53.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/018 - Pidgeot.wav",
        sprite_path: "/sprites/18.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/017 - Pidgeotto.wav",
        sprite_path: "/sprites/17.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/016 - Pidgey.wav",
        sprite_path: "/sprites/16.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/025 - Pikachu.wav",
        sprite_path: "/sprites/25.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/127 - Pinsir.wav",
        sprite_path: "/sprites/127.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/060 - Poliwag.wav",
        sprite_path: "/sprites/60.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/061 - Poliwhirl.wav",
        sprite_path: "/sprites/61.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/062 - Poliwrath.wav",
        sprite_path: "/sprites/62.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/077 - Ponita.wav",
        sprite_path: "/sprites/77.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/137 - Porygon.wav",
        sprite_path: "/sprites/137.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/057 - Rasaff.wav",
        sprite_path: "/sprites/57.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/054 - Psyduck.wav",
        sprite_path: "/sprites/54.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/026 - Raichu.wav",
        sprite_path: "/sprites/26.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/078 - Rapidash.wav",
        sprite_path: "/sprites/78.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/020 - raticate.wav",
        sprite_path: "/sprites/20.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/019 - Rattata.wav",
        sprite_path: "/sprites/19.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/112 - Rhydon.wav",
        sprite_path: "/sprites/112.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/111 - Rhyhorn.wav",
        sprite_path: "/sprites/111.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/027 - Sandshrew.wav",
        sprite_path: "/sprites/27.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/028 - Sandslash.wav",
        sprite_path: "/sprites/28.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/122 - Scyther.wav",
        sprite_path: "/sprites/122.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/117 - Seadra.wav",
        sprite_path: "/sprites/117.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/119 - Golking.wav",
        sprite_path: "/sprites/119.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/086 - Seel.wav",
        sprite_path: "/sprites/86.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/090 - Shellder.wav",
        sprite_path: "/sprites/90.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/080 - Slowbro.wav",
        sprite_path: "/sprites/80.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/079 - Slowpoke.wav",
        sprite_path: "/sprites/79.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/143 - Snorlax.wav",
        sprite_path: "/sprites/143.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/021 - Spearow.wav",
        sprite_path: "/sprites/21.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/007 - Squirtle.wav",
        sprite_path: "/sprites/7.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/121 - Starmie.wav",
        sprite_path: "/sprites/121.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/120 - Staryu.wav",
        sprite_path: "/sprites/120.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/114 - Tangela.wav",
        sprite_path: "/sprites/114.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/128 - Tauros.wav",
        sprite_path: "/sprites/128.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/072 - Tentacool.wav",
        sprite_path: "/sprites/72.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/073 - Tentacruel.wav",
        sprite_path: "/sprites/73.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/134 - Vaporeon.wav",
        sprite_path: "/sprites/134.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/049 - Venomoth.wav",
        sprite_path: "/sprites/49.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/048 - Venonat.wav",
        sprite_path: "/sprites/48.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/003 - Venusaur.wav",
        sprite_path: "/sprites/3.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/071 - Victreebel.wav",
        sprite_path: "/sprites/71.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/045 - Vileplume.wav",
        sprite_path: "/sprites/45.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/100 - Voltorb.wav",
        sprite_path: "/sprites/100.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/037 - Vilpix.wav",
        sprite_path: "/sprites/37.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/008 - Wartorle.wav",
        sprite_path: "/sprites/8.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/013 - Weedle.wav",
        sprite_path: "/sprites/13.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/070 - Weepinbell.wav",
        sprite_path: "/sprites/70.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/110 - Weezing.wav",
        sprite_path: "/sprites/110.png"
    }
}

pub fn wigglytuff() -> Pokemon {
    Pokemon {
        name: "Wigglytuff",
        ftype: Type::Normal,
        stype: Type::None,
        health: 390,
        atk: 130,
        def: 85,
        sp_atk: 139,
        sp_def: 94,
        init: 85,
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/040 - Wigglytuff.wav",
        sprite_path: "/sprites/40.png"
    }
}

pub fn zapdos() -> Pokemon {
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/145 - Zapdos.wav",
        sprite_path: "/sprites/145.png"
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
        moves: [attacks::dummy(); 4],
        battle_cry_path: "/battle_cries/041 - Zubat.wav",
        sprite_path: "/sprites/41.png"
    }
}
