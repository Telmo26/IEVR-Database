use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Hash)]
#[serde(rename_all = "lowercase")]
#[allow(non_camel_case_types)]
pub enum Language {
    DE,
    EN,
    ES,
    FR,
    IT,
    JA,
    PT,
    ZH_HANS,
    ZH_HANT
}

impl Language {
    pub fn default() -> Language { Language::EN }

    pub fn to_sql(&self) -> &'static str {
        match self {
            Self::DE => "de",
            Self::EN => "en",
            Self::ES => "es",
            Self::FR => "fr",
            Self::IT => "it",
            Self::JA => "ja",
            Self::PT => "pt",
            Self::ZH_HANS => "zh_hans",
            Self::ZH_HANT => "zh_hant"
        }
    }
}