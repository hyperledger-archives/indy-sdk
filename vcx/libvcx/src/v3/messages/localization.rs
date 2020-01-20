#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Localization {
    locale: Locales
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Locales {
    #[serde(rename = "en")]
    En,
}

impl Default for Locales {
    fn default() -> Locales {
        Locales::En
    }
}