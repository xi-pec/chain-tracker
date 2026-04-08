pub mod utils;
pub mod mdb;
pub mod hooks;

use crate::il2cpp::api::IAPI;

use hooks::Hooks;
use mdb::MDB;
use utils::Utils;

pub struct Core {
    pub hooks: Hooks,
    pub mdb: MDB,
    pub utils: Utils
}

impl Core {
    pub unsafe fn init(il2cpp: IAPI) -> Self {
        let hooks = Hooks::init(il2cpp);
        let mdb = MDB::init();
        let utils = Utils::init(il2cpp);

        Self { hooks, mdb, utils }
    }

    pub unsafe fn setup(&self) {
        self.hooks.setup();        
        self.mdb.load();
    }
}