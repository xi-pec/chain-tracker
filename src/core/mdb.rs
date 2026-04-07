pub mod chain_events;

use chain_events::{ChainEvent, ChainEventData};

use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

use sqlite;



pub struct MDB {
    connection: OnceLock<Mutex<sqlite::Connection>>,

    pub chain_events: OnceLock<HashMap<i32, ChainEventData>>
}

impl MDB {
    pub fn init() -> Self {
        let connection = OnceLock::new();
        let chain_events = OnceLock::new();

        let dir = env::current_dir();
        let dir_path = PathBuf::from(dir.unwrap());
        let mdb_path = dir_path.join( "UmamusumePrettyDerby_Jpn_Data\\Persistent\\master\\master.mdb");

        let db = sqlite::open(mdb_path).unwrap();
        let _ = connection.set(Mutex::new(db));

        Self { connection, chain_events }
    }

    pub fn load(&self) {
        self.load_chain_events();
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
            let mut chain_events: HashMap<i32, ChainEventData> = HashMap::new();

            for chain_event_data in chain_events_data {
                chain_events.insert(chain_event_data.story_id as i32, chain_event_data.data);
            }
            
            let _ = self.chain_events.set(chain_events);
        }
    }
}