
use crate::default_structures::Type;

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
    pub init: u32
    //TODO attacks
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
        init: 85
    }
}
