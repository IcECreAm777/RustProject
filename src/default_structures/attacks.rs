use crate::default_structures::Type;

pub struct Attack {
    pub name: &'static str,
    pub etype: Type,
    pub atype: AttackType,
    pub strgth: u32,
    pub acc: u32,
    //ap: u8,
    //effect: //TODO data type
}

pub enum AttackType {
    Physicial,
    Special
}

pub fn dummy() -> Attack {
    return  Attack {
        name: "None",
        etype: Type::None,
        atype: AttackType::Physicial,
        strgth: 0,
        acc: 0
    };
}

pub fn tackle() -> Attack {
    return Attack {
        name: "Tackle",
        etype: Type::Normal,
        atype: AttackType::Physicial,
        strgth: 35,
        acc: 95
    }
}
