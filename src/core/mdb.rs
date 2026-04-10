pub mod chain_events;
pub mod chara_names;
pub mod support_card;

use crate::core::localizer::LocalizedString;
use crate::plugin::PLUGIN;

use chain_events::{ChainEvent, ChainEventData};
use chara_names::CharaName;
use support_card::{SupportCard, SupportCardData};

use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

use sqlite;

pub struct MDB {
    connection: OnceLock<Mutex<sqlite::Connection>>,

    pub chain_events: OnceLock<HashMap<i64, ChainEventData>>,
    pub chara_names: OnceLock<HashMap<i64, LocalizedString>>,
    pub support_cards: OnceLock<HashMap<i64, SupportCardData>>
}

impl MDB {
    pub fn init() -> Self {
        let connection = OnceLock::new();

        let chain_events = OnceLock::new();
        let chara_names = OnceLock::new();
        let support_cards = OnceLock::new();

        let dir = env::current_dir();
        let path = PathBuf::from(dir.unwrap())
            .join( "UmamusumePrettyDerby_Jpn_Data")
            .join("Persistent")
            .join("master")
            .join("master.mdb");

        let db = sqlite::open(path).unwrap();
        let _ = connection.set(Mutex::new(db));

        Self { connection, chain_events, chara_names, support_cards }
    }

    pub fn load(&self) {
        self.load_chain_events();
        self.load_chara_names();
        self.load_support_cards();
    }

    pub fn query<F, T>(&self, sql: &str, mut callback: F) -> Option<Vec<T>>
    where F: FnMut(&mut sqlite::Statement) -> T,
    {
        let mutex = self.connection.get()?;
        let connection = mutex.lock().ok()?;
        let mut statement = connection.prepare(sql).ok()?;

        let mut results = Vec::new();
        while let Ok(sqlite::State::Row) = statement.next() {
            results.push(callback(&mut statement));
        }

        Some(results)
    }

    fn load_chain_events(&self) {
        let chain_events_data = self.query("
            SELECT story_id, show_progress_1, show_progress_2
            FROM 'single_mode_story_data'
            WHERE show_progress_2 != 0
        ", |data| {
            ChainEvent {
                story_id: data.read(0).unwrap(),
                data: ChainEventData {
                    current_progress: data.read(1).unwrap(),
                    max_progress: data.read(2).unwrap()
                }
            }
        });

        if let Some(chain_events_data) = chain_events_data {
            let mut chain_events: HashMap<i64, ChainEventData> = HashMap::new();

            for chain_event_data in chain_events_data {
                chain_events.insert(chain_event_data.story_id, chain_event_data.data);
            }

            let plugin = PLUGIN.get().unwrap();
            let console = &plugin.console;
            console.log(format!("Loaded {} chain event entries", chain_events.len()));
            
            let _ = self.chain_events.set(chain_events);
        }
    }

    fn load_chara_names(&self) {
        let chara_names_data = self.query("
            SELECT \"index\", text
            FROM 'text_data'
            WHERE id = 6 AND category = 6
        ", |data| {
            CharaName {
                chara_id: data.read(0).unwrap(),
                name: data.read(1).unwrap()
            }
        });

        if let Some(chara_names_data) = chara_names_data {
            let mut chara_names: HashMap<i64, LocalizedString> = HashMap::new();
            let plugin = PLUGIN.get().unwrap();

            for chara_name_data in chara_names_data {
                let localized = plugin.core.localizer.get_localization(
                    "6", 
                    &format!("{}",chara_name_data.chara_id),
                    &chara_name_data.name
                );

                chara_names.insert(chara_name_data.chara_id, localized);
            }

            let plugin = PLUGIN.get().unwrap();
            let console = &plugin.console;
            console.log(format!("Loaded {} character name entries", chara_names.len()));
            
            let _ = self.chara_names.set(chara_names);
        }
    }

    fn load_support_cards(&self) {
        let support_cards_data = self.query("
            SELECT id, chara_id, rarity, command_id, support_card_type
            FROM 'support_card_data'
        ", |data| {
            let command_id: i64 = data.read(3).unwrap();
            let card_type: i64 = data.read(4).unwrap();

            let r#type = match card_type {
                1 => match command_id {
                    101 => 1, // speed
                    105 => 2, // stamina
                    102 => 3, // power
                    103 => 4, // guts
                    106 => 5, // wit
                    _ => 0
                },

                2 => 6, // friend
                3 => 7, // group
                _ => 0
            };
            
            SupportCard {
                card_id: data.read(0).unwrap(),
                data: SupportCardData {
                    chara_id: data.read(1).unwrap(),
                    rarity: data.read(2).unwrap(),
                    r#type
                }
            }
        });

        if let Some(support_cards_data) = support_cards_data {
            let mut support_cards: HashMap<i64, SupportCardData> = HashMap::new();

            for support_card_data in support_cards_data {
                support_cards.insert(support_card_data.card_id, support_card_data.data);
            }

            let plugin = PLUGIN.get().unwrap();
            let console = &plugin.console;
            console.log(format!("Loaded {} support card entries", support_cards.len()));
            
            let _ = self.support_cards.set(support_cards);
        }
    }
}