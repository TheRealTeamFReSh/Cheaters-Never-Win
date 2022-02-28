use rand::distributions::{Alphanumeric, DistString};
use rand::prelude::SliceRandom;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum CheatCodeKind {
    Jump,
    DoubleJump,
    Attack,
    MoveLeft,
}

// here the value is the weight for the weighted distribution
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum CheatCodeRarity {
    Mandatory = 0, // weight of zero because it is not present in the distribution
    Common = 10,
    Rare = 5,
    Legendary = 2,
}

#[derive(Debug)]
pub enum CheatCodeActivationResult {
    NotFound,
    Activated(CheatCodeKind),
    AlreadyActivated(CheatCodeKind),
}
impl CheatCodeActivationResult {
    pub fn to_string(&self) -> String {
        match self {
            CheatCodeActivationResult::Activated(kind) => {
                return format!("[{:?}] cheat code successfully activated", kind)
            }
            CheatCodeActivationResult::AlreadyActivated(kind) => {
                return format!("[{:?}] already activated", kind)
            }
            CheatCodeActivationResult::NotFound => {
                return "cheat code not recognized by the system".to_string()
            }
        }
    }
}

#[derive(Debug)]
pub struct CheatCode {
    pub kind: CheatCodeKind,
    pub rarity: CheatCodeRarity,
    pub text: String,
    pub dependencies: Vec<CheatCodeKind>,
}

impl CheatCode {
    pub fn new(
        kind: CheatCodeKind,
        rarity: CheatCodeRarity,
        text: &str,
        dependencies: Vec<CheatCodeKind>,
    ) -> Self {
        Self {
            kind,
            rarity,
            text: text.to_string(),
            dependencies,
        }
    }
}

pub struct CheatCodeResource {
    pub codes: HashMap<CheatCodeKind, CheatCode>,
    activated: Vec<CheatCodeKind>,
}

impl CheatCodeResource {
    pub fn get_next_code(&self) -> CheatCodeKind {
        // first get a list of mandatory cheat codes (JUMP)
        let mandatories = self
            .codes
            .iter()
            .filter(|(kind, code)| {
                code.rarity == CheatCodeRarity::Mandatory && !self.is_code_activated(kind.clone())
            })
            .map(|(kind, _)| kind.clone())
            .collect::<Vec<CheatCodeKind>>();
        // if there is a mandatory code to be chosen, then return it
        if mandatories.len() > 0 {
            return mandatories.choose(&mut rand::thread_rng()).unwrap().clone();
        }

        // then we grab all the codes that haven't been activated yet
        // don't forget to check for dependencies!
        let available_codes = self
            .codes
            .iter()
            .filter(|(kind, code)| {
                let missing_deps = code
                    .dependencies
                    .iter()
                    .filter(|kind| !self.is_code_activated(kind))
                    .collect::<Vec<&CheatCodeKind>>();

                // if the code is not activated and has no missing deps
                // then it's a potential candidate
                missing_deps.len() == 0 && !self.is_code_activated(kind)
            })
            .map(|(_, code)| code)
            .collect::<Vec<&CheatCode>>();

        // then return a random code based on their rarity (rarity is the weight)

        available_codes
            .choose_weighted(&mut rand::thread_rng(), |code| code.rarity.clone() as u8)
            .unwrap()
            .kind
    }

    pub fn activate_code(&mut self, text: &str) -> CheatCodeActivationResult {
        // iteration over all the existing codes
        for (_, code) in self.codes.iter() {
            // it we found a code
            if code.text.eq(text) {
                if self.is_code_activated(&code.kind) {
                    return CheatCodeActivationResult::AlreadyActivated(code.kind.clone());
                }

                // if the code hasn't been activated do it
                self.activated.push(code.kind.clone());
                return CheatCodeActivationResult::Activated(code.kind.clone());
            }
        }
        return CheatCodeActivationResult::NotFound;
    }

    pub fn is_code_activated(&self, kind: &CheatCodeKind) -> bool {
        self.activated.contains(kind)
    }

    pub fn new() -> Self {
        let mut codes: HashMap<CheatCodeKind, CheatCode> = HashMap::new();

        codes.insert(
            CheatCodeKind::Jump,
            CheatCode::new(
                CheatCodeKind::Jump,
                CheatCodeRarity::Mandatory,
                &generate_random_code(CheatCodeRarity::Mandatory),
                vec![],
            ),
        );
        codes.insert(
            CheatCodeKind::DoubleJump,
            CheatCode::new(
                CheatCodeKind::DoubleJump,
                CheatCodeRarity::Common,
                &generate_random_code(CheatCodeRarity::Common),
                vec![CheatCodeKind::Jump],
            ),
        );
        codes.insert(
            CheatCodeKind::Attack,
            CheatCode::new(
                CheatCodeKind::Attack,
                CheatCodeRarity::Rare,
                &generate_random_code(CheatCodeRarity::Rare),
                vec![],
            ),
        );
        codes.insert(
            CheatCodeKind::MoveLeft,
            CheatCode::new(
                CheatCodeKind::MoveLeft,
                CheatCodeRarity::Legendary,
                &generate_random_code(CheatCodeRarity::Legendary),
                vec![],
            ),
        );

        Self {
            codes,
            activated: Vec::new(),
        }
    }
}

pub fn generate_random_code(rarity: CheatCodeRarity) -> String {
    // length is based on the rarity
    let length = match rarity {
        CheatCodeRarity::Mandatory => 4,
        CheatCodeRarity::Common => 4,
        CheatCodeRarity::Rare => 6,
        CheatCodeRarity::Legendary => 8,
    };

    Alphanumeric.sample_string(&mut rand::thread_rng(), length)
}