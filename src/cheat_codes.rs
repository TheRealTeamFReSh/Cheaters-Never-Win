use std::collections::HashMap;

pub enum CheatCodeKind {
    Jump,
    DoubleJump,
    Attack,
    MoveLeft,
}

pub enum CheatCodeRarity {
    MANDATORY = 0,
    COMMON,
    RARE,
    LEGENDARY = 3,
}

pub struct CheatCode {
    kind: CheatCodeKind,
    rarity: CheatCodeRarity,
    text: String,
    dependencies: Vec<CheatCodeKind>,
}

pub struct CheatCodeResource {
    codes: HashMap<CheatCodeKind, CheatCode>,
    activated: Vec<CheatCodeKind>,
}

impl CheatCodeResource {
    pub fn new() -> Self {
        let codes: HashMap<CheatCodeKind, CheatCode> = HashMap::new();

        Self {
            codes,
            activated: Vec::new(),
        }
    }
}
