use rand::distributions::{Alphanumeric, DistString};
use rand::prelude::SliceRandom;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Deserialize)]
pub enum CheatCodeKind {
    // Mandatory
    Jump,

    // Common
    Crouch,
    Attack,
    AttackDmgBoost,
    AttackFireRateBoost,
    MoveLeft,
    SpeedBoost1,
    SpeedBoost2,
    SpeedBoost3,
    Armor,
    Dash,

    // Rare
    DoubleJump,
    SpeedBoost4,
    SpeedBoost5,
    Shield,

    // Legendary
    ExtraLife,
    TempInvicibility,
    Fly,
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
    pub fn repr(&self) -> String {
        match self {
            CheatCodeActivationResult::Activated(kind) => {
                return format!("[{:?}] cheat code successfully activated", kind)
            }
            CheatCodeActivationResult::AlreadyActivated(kind) => {
                return format!("[{:?}] already activated", kind)
            }
            CheatCodeActivationResult::NotFound => {
                "cheat code not recognized by the system".to_string()
            }
        }
    }
}

#[derive(Clone)]
pub struct CheatCode {
    pub kind: CheatCodeKind,
    pub rarity: CheatCodeRarity,
    pub text: String,
    pub dependencies: Vec<CheatCodeKind>,
    pub image: String,
    pub help_text: String,
}

impl CheatCode {
    pub fn new(
        kind: CheatCodeKind,
        rarity: CheatCodeRarity,
        text: &str,
        dependencies: Vec<CheatCodeKind>,
        image: String,
        help_text: String,
    ) -> Self {
        Self {
            kind,
            rarity,
            text: text.to_string(),
            dependencies,
            image,
            help_text,
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
                code.rarity == CheatCodeRarity::Mandatory
                    && !self.is_code_activated(&(*kind).clone())
            })
            .map(|(kind, _)| *kind)
            .collect::<Vec<CheatCodeKind>>();
        // if there is a mandatory code to be chosen, then return it
        if !mandatories.is_empty() {
            return *mandatories.choose(&mut rand::thread_rng()).unwrap();
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
                    .count();

                // if the code is not activated and has no missing deps
                // then it's a potential candidate
                missing_deps == 0 && !self.is_code_activated(kind)
            })
            .map(|(_, code)| code)
            .collect::<Vec<&CheatCode>>();

        // then return a random code based on their rarity (rarity is the weight)

        available_codes
            .choose_weighted(&mut rand::thread_rng(), |code| code.rarity as u8)
            .unwrap()
            .kind
    }

    pub fn activate_code(&mut self, text: &str) -> CheatCodeActivationResult {
        // iteration over all the existing codes
        for (_, code) in self.codes.iter() {
            // it we found a code
            if code.text.eq(&text.to_lowercase()) {
                if self.is_code_activated(&code.kind) {
                    return CheatCodeActivationResult::AlreadyActivated(code.kind);
                }

                // if the code hasn't been activated do it
                self.activated.push(code.kind);
                return CheatCodeActivationResult::Activated(code.kind);
            }
        }
        CheatCodeActivationResult::NotFound
    }

    pub fn deactivate_code(&mut self, kind: &CheatCodeKind) {
        if self.is_code_activated(kind) {
            let index = self.activated.iter().position(|r| r == kind).unwrap();
            self.activated.remove(index);
        }
    }

    pub fn is_code_activated(&self, kind: &CheatCodeKind) -> bool {
        self.activated.contains(kind)
    }

    pub fn new() -> Self {
        let mut codes: HashMap<CheatCodeKind, CheatCode> = HashMap::new();

        // Mandatory
        insert_cheat(
            &mut codes,
            CheatCodeKind::Jump,
            CheatCodeRarity::Mandatory,
            vec![],
            "jump.png",
            "Press space to jump",
        );

        // Common
        insert_cheat(
            &mut codes,
            CheatCodeKind::Crouch,
            CheatCodeRarity::Common,
            vec![],
            "crouch.png",
            "Press Ctrl to crouch",
        );
        insert_cheat(
            &mut codes,
            CheatCodeKind::Attack,
            CheatCodeRarity::Common,
            vec![],
            "attack.png",
            "Press Enter to Attack",
        );
        /*insert_cheat(
            &mut codes,
            CheatCodeKind::AttackDmgBoost,
            CheatCodeRarity::Common,
            vec![CheatCodeKind::Attack],
            "attack_dmg_boost.png",
            "Damage boost applied",
        );
        insert_cheat(
            &mut codes,
            CheatCodeKind::AttackFireRateBoost,
            CheatCodeRarity::Common,
            vec![CheatCodeKind::Attack],
            "attack_fr_boost.png",
            "Better fire rate",
        );*/
        insert_cheat(
            &mut codes,
            CheatCodeKind::MoveLeft,
            CheatCodeRarity::Common,
            vec![],
            "move_left.png",
            "Press 'A' to move left",
        );
        insert_cheat(
            &mut codes,
            CheatCodeKind::SpeedBoost1,
            CheatCodeRarity::Common,
            vec![],
            "speed.png",
            "Speed increase lvl. 1",
        );
        insert_cheat(
            &mut codes,
            CheatCodeKind::SpeedBoost2,
            CheatCodeRarity::Common,
            vec![CheatCodeKind::SpeedBoost1],
            "speed.png",
            "Speed increase lvl. 2",
        );
        insert_cheat(
            &mut codes,
            CheatCodeKind::SpeedBoost3,
            CheatCodeRarity::Common,
            vec![CheatCodeKind::SpeedBoost1, CheatCodeKind::SpeedBoost2],
            "speed.png",
            "Speed increase lvl. 3",
        );
        /*insert_cheat(
            &mut codes,
            CheatCodeKind::Armor,
            CheatCodeRarity::Common,
            vec![],
            "armor.png",
            "Better armor",
        );
        insert_cheat(
            &mut codes,
            CheatCodeKind::Dash,
            CheatCodeRarity::Common,
            vec![],
            "dash.png",
            "Double tap 'D' to dash",
        );*/

        // Rare
        insert_cheat(
            &mut codes,
            CheatCodeKind::DoubleJump,
            CheatCodeRarity::Rare,
            vec![CheatCodeKind::Jump],
            "double_jump.png",
            "Press space in air to jump",
        );
        /*insert_cheat(
            &mut codes,
            CheatCodeKind::SpeedBoost4,
            CheatCodeRarity::Rare,
            vec![
                CheatCodeKind::SpeedBoost1,
                CheatCodeKind::SpeedBoost2,
                CheatCodeKind::SpeedBoost3,
            ],
            "speed.png",
            "Speed increase lvl. 4",
        );
        insert_cheat(
            &mut codes,
            CheatCodeKind::SpeedBoost5,
            CheatCodeRarity::Rare,
            vec![
                CheatCodeKind::SpeedBoost1,
                CheatCodeKind::SpeedBoost2,
                CheatCodeKind::SpeedBoost3,
                CheatCodeKind::SpeedBoost4,
            ],
            "speed.png",
            "Speed increase lvl. 5",
        );
        insert_cheat(
            &mut codes,
            CheatCodeKind::Shield,
            CheatCodeRarity::Rare,
            vec![CheatCodeKind::Jump],
            "shield.png",
            "Shield activated",
        );*/

        // Legendary
        insert_cheat(
            &mut codes,
            CheatCodeKind::ExtraLife,
            CheatCodeRarity::Legendary,
            vec![],
            "extra_life.png",
            "Got one extra life",
        );
        /*insert_cheat(
            &mut codes,
            CheatCodeKind::TempInvicibility,
            CheatCodeRarity::Legendary,
            vec![CheatCodeKind::Armor, CheatCodeKind::Shield],
            "temp_invincibility.png",
            "Temporary invincible",
        );
        insert_cheat(
            &mut codes,
            CheatCodeKind::Fly,
            CheatCodeRarity::Legendary,
            vec![CheatCodeKind::Jump, CheatCodeKind::DoubleJump],
            "fly.png",
            "Hold space to fly",
        );*/

        Self {
            codes,
            activated: Vec::new(),
        }
    }
}

fn insert_cheat(
    codes: &mut HashMap<CheatCodeKind, CheatCode>,
    kind: CheatCodeKind,
    rarity: CheatCodeRarity,
    dependencies: Vec<CheatCodeKind>,
    image_path: &str,
    help_text: &str,
) {
    codes.insert(
        kind,
        CheatCode::new(
            kind,
            rarity,
            &generate_random_code(rarity),
            dependencies,
            image_path.to_string(),
            help_text.to_string(),
        ),
    );
}

pub fn generate_random_code(rarity: CheatCodeRarity) -> String {
    // length is based on the rarity
    let length = match rarity {
        CheatCodeRarity::Mandatory => 4,
        CheatCodeRarity::Common => 4,
        CheatCodeRarity::Rare => 6,
        CheatCodeRarity::Legendary => 8,
    };

    Alphanumeric
        .sample_string(&mut rand::thread_rng(), length)
        .to_lowercase()
}

pub fn randomize_text(s: &String, indices: Vec<u8>, is_random_string: bool) -> String {
    if !is_random_string {
        let mut result = vec![' '; s.len()];
        for (i, c) in indices.into_iter().zip(s.chars()) {
            result[i as usize] = c;
        }
        return result.into_iter().collect();
    } else {
        return Alphanumeric
            .sample_string(&mut rand::thread_rng(), s.len())
            .to_lowercase();
    }
}
