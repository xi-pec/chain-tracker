pub mod list;
pub mod hook;

use list::*;
use hook::*;

use crate::console::log;

pub unsafe fn run(cmd: &str) {
    let mut tokens = cmd.split(" ");

    let Some(main) = tokens.next()
    else { return log("No command to execute.") };

    match main {
        "list" => list(),
        "hook" => hook(&mut tokens),
        _ => log(format!("Could not find command \"{}\".", main))
    }
}