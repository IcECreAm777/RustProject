
use crate::default_structures::{Type, attacks};
use crate::game_assets::PokemonAssets;
use std::fmt::{Display, Result, Formatter};
use std::hash::{Hash, Hasher};
use ggez::{Context};

#[derive(Clone)]
pub struct Pokemon {
    pub name: &'static str,
    pub ftype: Type,
    pub stype: Type,
    pub health: u32,
    pub atk: i32,
    pub def: i32,
    pub sp_atk: i32,
    pub sp_def: i32,
    pub init: i32,
    pub moves: [attacks::Attack; 4], 
    pub assets: PokemonAssets
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

pub fn dummy_pokemon(ctx: &mut Context) -> Pokemon {
    Pokemon {
        name: "Dummy",
        ftype: Type::Normal,
        stype: Type::None,
        health: 200,
        atk: 50,
        def: 50,
        sp_atk: 10,
        sp_def: 10,
        init: 10,
        moves: [attacks::dummy(); 4],
        assets: PokemonAssets::new(ctx, "/battle_cries/000 - Dummy.wav", "/sprites/dummy.png").unwrap()
    }
}

pub fn abra(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/063 - Abra.wav", "/sprites/63.png").unwrap()
    }
}

pub fn aerodactyl(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/142 - Aerodactyl.wav", "/sprites/142.png").unwrap() 
    }
}

pub fn alakazam(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/065 - Alakazam.wav", "/sprites/65.png").unwrap()
    }
}

pub fn arbok(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/024 - Arbok.wav", "/sprites/24.png").unwrap()
    }
}

pub fn arcanine(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/059 - Arcanine.wav", "/sprites/59.png").unwrap() 
    }
}

pub fn articuno(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/144 - Articuno.wav", "/sprites/144.png").unwrap()
    }
}

pub fn beedrill(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/015 - Beedrill.wav", "/sprites/15.png").unwrap()
    }
}

pub fn bellsprout(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/069 - Bellsprout.wav", "/sprites/69.png").unwrap()
    }
}

pub fn blastoise(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/009 - Blastoise.wav", "/sprites/9.png").unwrap()
    }
}

pub fn bulbasur(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/001 - bulbasaur.wav", "/sprites/1.png").unwrap()
    }
}

pub fn butterfree(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/012 - Butterfree.wav", "/sprites/12.png").unwrap()
    }
}

pub fn caterpie(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/010 - Caterpie.wav", "/sprites/10.png").unwrap()
    }
}

pub fn chansey(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/113 - Chansey.wav", "/sprites/113.png").unwrap()
    }
}

pub fn charizard(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/006 - Charizard.wav", "/sprites/6.png").unwrap() 
    }
}

pub fn charmander(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/004 - Charmander.wav", "/sprites/4.png").unwrap()
    }
}

pub fn charmaleon(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/005 - Charmeleon.wav", "/sprites/5.png").unwrap()
    }
}

pub fn clefable(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/036 - Clefable.wav", "/sprites/36.png").unwrap()
    }
}

pub fn clefairy(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/035 - Clefairy.wav", "/sprites/35.png").unwrap()
    }
}

pub fn cloyster(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/091 - Cloyster.wav", "/sprites/91.png").unwrap()
    }
}

pub fn cubone(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/104 - Cubone.wav", "/sprites/104.png").unwrap()
    }
}

pub fn dewgong(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/087 - Dewgong.wav", "/sprites/87.png").unwrap()
    }
}
pub fn diglett(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/050 - Diglett.wav", "/sprites/50.png").unwrap()
    }
}

pub fn ditto(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/132 - Ditto.wav", "/sprites/132.png").unwrap() 
    }
}

pub fn dodrio(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/085 - Dodrio.wav", "/sprites/85.png").unwrap()
    }
}

pub fn doduo(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/084 - Doduo.wav", "/sprites/84.png").unwrap()
    }
}

pub fn dragonair(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/148 - Dragonair.wav", "/sprites/148.png").unwrap()
    }
}

pub fn dragonite(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/149 - Dragonite.wav", "/sprites/149.png").unwrap()
    }
}

pub fn dratini(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/147 - Dratini.wav", "/sprites/147.png").unwrap()
    }
}

pub fn drowzee(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/096 - Drowzee.wav", "/sprites/96.png").unwrap()
    }
}

pub fn dugtrio(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/051 - Dugtrio.wav", "/sprites/51.png").unwrap()
    }
}

pub fn eevee(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/133 - Eevee.wav", "/sprites/133.png").unwrap()
    }
}

pub fn ekans(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/023 - Ekans.wav", "/sprites/23.png").unwrap()
    }
}

pub fn electabuzz(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/125 - Electabuzz.wav", "/sprites/125.png").unwrap()
    }
}

pub fn electrode(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/101 - Electrode.wav", "/sprites/101.png").unwrap()
    }
}

pub fn exeggcute(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/102 - Exeggcute.wav", "/sprites/102.png").unwrap()
    }
}

pub fn exeggcutor(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/103 - Exeggutor.wav", "/sprites/103.png").unwrap()
    }
}

pub fn farfetch(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/083 - Farfetch'd.wav", "/sprites/83.png").unwrap()
    }
}

pub fn fearow(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/022 - Fearow.wav", "/sprites/22.png").unwrap()
    }
}

pub fn flareon(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/136 - Flareon.wav", "/sprites/136.png").unwrap()
    }
}

pub fn gastly(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/092 - Gastly.wav", "/sprites/92.png").unwrap()
    }
}

pub fn gengar(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/094 - Gengar.wav", "/sprites/94.png").unwrap()
    }
}

pub fn geodude(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/074 - Geodude.wav", "/sprites/74.png").unwrap()
    }
}

pub fn gloom(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/044 - Gloom.wav", "/sprites/44.png").unwrap()
    }
}

pub fn golbat(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/042 - Golbat.wav", "/sprites/42.png").unwrap()
    }
}

pub fn goldeen(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/118 - Goldeen.wav", "/sprites/118.png").unwrap()
    }
}

pub fn golduck(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/055 - Golduck.wav", "/sprites/55.png").unwrap()
    }
}

pub fn golem(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/076 - Golem.wav", "/sprites/76.png").unwrap()
    }
}

pub fn graveler(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/075 - Graveler.wav", "/sprites/75.png").unwrap()
    }
}

pub fn grimer(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/088 - Grimer.wav", "/sprites/88.png").unwrap()
    }
}

pub fn growlithe(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/058 - Growlithe.wav", "/sprites/58.png").unwrap()
    }
}

pub fn gyarados(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/130 - Gyarados.wav", "/sprites/130.png").unwrap()
    }
}

pub fn haunter(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/093 - Haunter.wav", "/sprites/93.png").unwrap()
    }
}

pub fn hitmonchan(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/107 - Hitmonchan.wav", "/sprites/107.png").unwrap()
    }
}

pub fn hitmonlee(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/106 - Hitmonlee.wav", "/sprites/106.png").unwrap()
    }
}

pub fn horsea(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/116 - Horsea.wav", "/sprites/116.png").unwrap()
    }
}

pub fn hypno(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/097 - Hypno.wav", "/sprites/97.png").unwrap()
    }
}

pub fn ivysaur(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/002 - Ivysaur.wav", "/sprites/2.png").unwrap()
    }
}

pub fn jigglypuff(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/039 - Jigglypuff.wav", "/sprites/39.png").unwrap()
    }
}

pub fn jolteon(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/135 - Jolteon.wav", "/sprites/135.png").unwrap()
    }
}

pub fn jynx(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/124 - Jynx.wav", "/sprites/124.png").unwrap()
    }
}

pub fn kabuto(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/140 - Kabuto.wav", "/sprites/140.png").unwrap()
    }
}

pub fn kabutops(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/141 - Kabutops.wav", "/sprites/141.png").unwrap()
    }
}

pub fn kadabra(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/064 - Kadabra.wav", "/sprites/64.png").unwrap()
    }
}

pub fn kakuna(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/014 - Kakuna.wav", "/sprites/14.png").unwrap()
        
    }
}

pub fn kangaskhan(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/115 - Kangaskhan.wav", "/sprites/115.png").unwrap()
    }
}

pub fn kingler(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/099 - Kingler.wav", "/sprites/99.png").unwrap()
    }
}

pub fn koffing(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/109 - Koffing.wav", "/sprites/109.png").unwrap()
    }
}

pub fn krabby(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/098 - Krabby.wav", "/sprites/98.png").unwrap()
    }
}

pub fn lapras(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/131 - Lapras.wav", "/sprites/131.png").unwrap()
    }
}

pub fn lickitung(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/108 - Lickitung.wav", "/sprites/108.png").unwrap()
    }
}

pub fn machamp(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/068 - Machamp.wav", "/sprites/68.png").unwrap()
    }
}

pub fn machoke(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/067 - Machoke.wav", "/sprites/67.png").unwrap()
    }
}

pub fn machop(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/066 - Machop.wav", "/sprites/66.png").unwrap()
    }
}

pub fn magikarp(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/129 - Magikarp.wav", "/sprites/129.png").unwrap()
    }
}

pub fn magmar(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/126 - Magmar.wav", "/sprites/126.png").unwrap()
    }
}

pub fn magnemite(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/081 - Magnemite.wav", "/sprites/81.png").unwrap()
    }
}

pub fn magneton(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/082 - Magneton.wav", "/sprites/82.png").unwrap()
    }
}

pub fn mankey(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/056 - Mankey.wav", "/sprites/56.png").unwrap()
    }
}

pub fn marowak(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/105 - Marowak.wav", "/sprites/105.png").unwrap()
    }
}

pub fn meowth(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/052 - Meowth.wav", "/sprites/52.png").unwrap()
    }
}

pub fn metapod(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/011 - Metapod.wav", "/sprites/11.png").unwrap()
    }
}

pub fn mew(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/151 - Mew.wav", "/sprites/151.png").unwrap()
    }
}

pub fn mewtwo(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/150 - Mewtwo.wav", "/sprites/150.png").unwrap()
    }
}

pub fn moltres(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/146 - Moltres.wav", "/sprites/146.png").unwrap()
    }
}

pub fn mrmime(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/122 - Mr. Mime.wav", "/sprites/122.png").unwrap()
    }
}

pub fn muk(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/089 - Muk.wav", "/sprites/89.png").unwrap()
    }
}

pub fn nidoking(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/034 - Nidoking.wav", "/sprites/34.png").unwrap()
    }
}

pub fn nidoqueen(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/031 - Nidoqueen.wav", "/sprites/31.png").unwrap()
    }
}

pub fn nidoranf(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/029 - Nidoran (Female).wav", "/sprites/29.png").unwrap()
    }
}

pub fn nidoranm(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/032 - Nidoran (Male).wav", "/sprites/32.png").unwrap()
    }
}

pub fn nidorina(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/030 - Nidorina.wav", "/sprites/30.png").unwrap()
    }
}

pub fn nidorino(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/033 - Nidorino.wav", "/sprites/33.png").unwrap()
    }
}

pub fn ninetales(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/038 - Ninetales.wav", "/sprites/38.png").unwrap()
    }
}

pub fn oddish(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/043 - Oddish.wav", "/sprites/43.png").unwrap()
    }
}

pub fn omanyte(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/138 - Omanyte.wav", "/sprites/138.png").unwrap()
    }
}

pub fn omastar(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/139 - Omastar.wav", "/sprites/139.png").unwrap()
    }
}

pub fn onix(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/095 - Onix.wav", "/sprites/95.png").unwrap()

    }
}

pub fn paras(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/046 - Paras.wav", "/sprites/46.png").unwrap()
    }
}

pub fn parasect(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/047 - Parasect.wav", "/sprites/47.png").unwrap()
    }
}

pub fn persian(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/053 - Persian.wav", "/sprites/53.png").unwrap()
    }
}

pub fn pidgeot(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/018 - Pidgeot.wav", "/sprites/18.png").unwrap()
    }
}

pub fn pidgeotto(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/017 - Pidgeotto.wav", "/sprites/17.png").unwrap()
    }
}

pub fn pidgey(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/016 - Pidgey.wav", "/sprites/16.png").unwrap()
    }
}

pub fn pikachu(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/025 - Pikachu.wav", "/sprites/25.png").unwrap()
    }
}

pub fn pinsir(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/127 - Pinsir.wav", "/sprites/127.png").unwrap()
    }
}

pub fn poliwag(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/060 - Poliwag.wav", "/sprites/60.png").unwrap()
    }
}

pub fn poliwhirl(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/061 - Poliwhirl.wav", "/sprites/61.png").unwrap()
    }
}

pub fn poliwrath(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/062 - Poliwrath.wav", "/sprites/62.png").unwrap()
    }
}

pub fn ponyta(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/077 - Ponyta.wav", "/sprites/77.png").unwrap()
    }
}

pub fn porygon(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/137 - Porygon.wav", "/sprites/137.png").unwrap()

    }
}

pub fn primeape(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/057 - Primeape.wav", "/sprites/57.png").unwrap()
    }
}

pub fn psyduck(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/054 - Psyduck.wav", "/sprites/54.png").unwrap()
    }
}

pub fn raichu(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/026 - Raichu.wav", "/sprites/26.png").unwrap()
    }
}

pub fn rapidash(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/078 - Rapidash.wav", "/sprites/78.png").unwrap()
    }
}

pub fn raticate(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/020 - raticate.wav", "/sprites/20.png").unwrap()
    }
}

pub fn rattata(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/019 - Rattata.wav", "/sprites/19.png").unwrap()
    }
}

pub fn rhydon(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/112 - Rhydon.wav", "/sprites/112.png").unwrap()
    }
}

pub fn rhyhorn(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/111 - Rhyhorn.wav", "/sprites/111.png").unwrap()
    }
}

pub fn sandshrew(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/027 - Sandshrew.wav", "/sprites/27.png").unwrap()
    }
}

pub fn sandslash(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/028 - Sandslash.wav", "/sprites/28.png").unwrap()
    }
}

pub fn scyther(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/123 - Scyther.wav", "/sprites/123.png").unwrap()
    }
}

pub fn seadra(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/117 - Seadra.wav", "/sprites/117.png").unwrap()
    }
}

pub fn seaking(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/119 - Seaking.wav", "/sprites/119.png").unwrap()
    }
}

pub fn seel(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/086 - Seel.wav", "/sprites/86.png").unwrap()
    }
}

pub fn shellder(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/090 - Shellder.wav", "/sprites/90.png").unwrap()
    }
}

pub fn slowbro(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/080 - Slowbro.wav", "/sprites/80.png").unwrap()
    }
}

pub fn slowpoke(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/079 - Slowpoke.wav", "/sprites/79.png").unwrap()
    }
}

pub fn snorlax(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/143 - Snorlax.wav", "/sprites/143.png").unwrap()
    }
}

pub fn spearow(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/021 - Spearow.wav", "/sprites/21.png").unwrap()
    }
}

pub fn squirtle(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/007 - Squirtle.wav", "/sprites/7.png").unwrap()
    }
}

pub fn starmie(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/121 - Starmie.wav", "/sprites/121.png").unwrap()
    }
}

pub fn staryu(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/120 - Staryu.wav", "/sprites/120.png").unwrap()
    }
}

pub fn tangela(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/114 - Tangela.wav", "/sprites/114.png").unwrap()
    }
}

pub fn tauros(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/128 - Tauros.wav", "/sprites/128.png").unwrap()
    }
}

pub fn tentacool(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/072 - Tentacool.wav", "/sprites/72.png").unwrap()
    }
}

pub fn tentacruel(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/073 - Tentacruel.wav", "/sprites/73.png").unwrap()
    }
}

pub fn vaporeon(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/134 - Vaporeon.wav", "/sprites/134.png").unwrap()
    }
}

pub fn venomoth(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/049 - Venomoth.wav", "/sprites/49.png").unwrap()
    }
}

pub fn venonat(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/048 - Venonat.wav", "/sprites/48.png").unwrap()
    }
}

pub fn venusaur(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/003 - Venusaur.wav", "/sprites/3.png").unwrap()
    }
}

pub fn victreebel(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/071 - Victreebel.wav", "/sprites/71.png").unwrap()
    }
}

pub fn vileplume(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/045 - Vileplume.wav", "/sprites/45.png").unwrap()
    }
}

pub fn voltorb(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/100 - Voltorb.wav", "/sprites/100.png").unwrap()
    }
}

pub fn vulpix(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/037 - Vulpix.wav", "/sprites/37.png").unwrap()
    }
}

pub fn wartortle(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/008 - Wartortle.wav", "/sprites/8.png").unwrap()
    }
}

pub fn weedle(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/013 - Weedle.wav", "/sprites/13.png").unwrap()
    }
}

pub fn weepinbell(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/070 - Weepinbell.wav", "/sprites/70.png").unwrap()
    }
}

pub fn weezing(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/110 - Weezing.wav", "/sprites/110.png").unwrap()
    }
}

pub fn wigglytuff(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/040 - Wigglytuff.wav", "/sprites/40.png").unwrap()
    }
}

pub fn zapdos(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/145 - Zapdos.wav", "/sprites/145.png").unwrap()
    }
}

pub fn zubat(ctx: &mut Context) -> Pokemon {
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
        assets: PokemonAssets::new(ctx, "/battle_cries/041 - Zubat.wav", "/sprites/41.png").unwrap()
    }
}
