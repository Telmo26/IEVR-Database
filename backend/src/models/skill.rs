use serde::{Deserialize, Serialize};
use sqlx::{Row, sqlite::SqliteRow};

use super::common::Element;

#[derive(Debug, Serialize, Deserialize)]
pub struct Hissatsu {
    id: i32,
    name: String,
    power: u8,
    element: Element,
    category: Category,
    growth_rate: u8,
    is_block: bool,
    is_longshot: bool,
    tp_consumption: u8,
    cooldown: u8
}

impl Hissatsu {
    pub fn parse(row: SqliteRow) -> Option<Hissatsu> {
        Some(Hissatsu {
            id: row.try_get("skill_id").ok()?,
            name: row.try_get("name").ok()?,
            power: row.try_get("power").ok()?,
            element: Element::from(row.get::<i32, &str>("element")),
            category: Category::from(row.get::<u8, &str>("category")),
            growth_rate: row.try_get("growth_rate").ok()?,
            is_block: row.try_get("is_block").ok()?,
            is_longshot: row.try_get("is_longshot").ok()?,
            tp_consumption: row.try_get("tp_consumption").ok()?,
            cooldown: row.try_get("cooldown").ok()?
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Aura {
    id: i32,
    name: String,
    aura_type: AuraType,
    element: Element,
    hissatsu: Option<Hissatsu>,
}

impl Aura {
    pub fn parse(row: SqliteRow) -> Option<Aura> {
        Some(Aura {
            id: row.try_get("aura_id").ok()?,
            name: row.try_get("aura_name").ok()?,
            aura_type: AuraType::from(row.get::<u8, &str>("aura_type")),
            element: Element::from(row.get::<i32, &str>("aura_element")),
            hissatsu: {
                let value = row.try_get::<i32, &str>("skill_id").ok()?;
                if value != 0 {
                    Hissatsu::parse(row)
                } else { None }
            },
        })
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    SHOOT = 1,
    DRIBBLE = 2,
    DEFENSE = 3,
    GOALKEEPER = 4,
}

impl From<u8> for Category {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::SHOOT,
            2 => Self::DRIBBLE,
            3 => Self::DEFENSE,
            4 => Self::GOALKEEPER,
            _ => unreachable!()
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash)]
#[serde(rename_all = "lowercase")]
pub enum AuraType {
    Keshin = 0,
    Armed = 1,
    MixiMax = 2,
    Totem = 3,
    BondTransform = 4,
    Awakening = 5,
    ModeChange = 6,
    AwakeningChange = 7
}

impl From<u8> for AuraType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Keshin,
            1 => Self::Armed,
            2 => Self::MixiMax,
            3 => Self::Totem,
            4 => Self::BondTransform,
            5 => Self::Awakening,
            6 => Self::ModeChange,
            7 => Self::AwakeningChange,
            _ => unreachable!()
        }
    }
}