pub mod install;
pub mod uninstall;

use install::*;
use uninstall::*;

use std::str::Split;

use crate::console::log;

pub unsafe fn hook(args: &mut Split<'_, &str>) {
    let Some(sub) = args.next()
    else { return log(format!("No subcommand provided for command \"hook\".")) };

    match sub {
        "install" => hook_install(args),
        "uninstall" => hook_uninstall(args),
        _ => log(format!("Could not find subcommand \"{}\"", sub))
    }
}