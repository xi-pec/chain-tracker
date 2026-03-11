pub mod list;

use list::*;

use crate::console::log;

pub unsafe fn run(cmd: &str) {
    // let plugin = PLUGIN.get().unwrap();
    // let mutex = plugin.hooks.get().unwrap();
    // let hooks = mutex.lock().unwrap();

    let mut tokens = cmd.split(" ");
    
    let main = tokens.next().unwrap();

    match main {
        "list" => list(),
        _ => log(format!("Could not find command {}.", main))
    }
}