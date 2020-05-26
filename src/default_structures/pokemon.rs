
use crate::default_structures::Type;
use crate::default_structures::attacks;

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
