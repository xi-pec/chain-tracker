pub mod utils;
pub mod mdb;

use crate::il2cpp::api::IAPI;
use mdb::MDB;
use utils::Utils;

pub struct Core {
    pub il2cpp: IAPI,

    pub mdb: MDB,
    pub utils: Utils
}

impl Core {
    pub fn init(il2cpp: IAPI) -> Self {
        let mdb = MDB::init();
        let utils = Utils::init(il2cpp);

        mdb.load();

        Self { il2cpp, mdb, utils }
    }
}