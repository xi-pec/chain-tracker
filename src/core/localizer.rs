use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::env;
use std::path::PathBuf;

type TextData = HashMap<String, HashMap<String, String>>;

pub struct LocalizedString {
    pub jp: String,
    pub en: Option<String>
}

impl LocalizedString {
    pub fn resolve(&self) -> String {
        match &self.en {
            Some(en) => en.to_string(),
            None => self.jp.to_string()
        }
    }
}

pub struct Localizer {
    entries: TextData
}

impl Localizer {
    pub fn init() -> Self {
        let entries = HashMap::new();

        let dir = env::current_dir();
        let path = PathBuf::from(dir.unwrap())
            .join("hachimi")
            .join("localized_data")
            .join("text_data_dict.json");
        if !path.exists() { return Self { entries } }

        let Ok(file) = File::open(path)
        else { return Self { entries } };

        let reader = BufReader::new(file);

        let Ok(json) =  serde_json::from_reader::<_, TextData>(reader)
        else { return Self { entries } };

        Self { entries: json }
    }

    pub fn get_localization<S: AsRef<str>>(&self, category: S, index: S, jp: S) -> LocalizedString {
        let jp = jp.as_ref().to_string();
        let mut localized = LocalizedString { jp, en: None };

        let Some(indexes) = self.entries.get(category.as_ref())
        else { return localized };

        let Some(en) = indexes.get(index.as_ref())
        else { return localized };

        localized.en = Some(en.to_string());

        localized
    }
}