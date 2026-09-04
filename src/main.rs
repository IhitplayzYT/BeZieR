use crate::helper::Helper::CLI;
use crate::ui::run_app;

mod helper;
mod model;
mod ui;

fn main() {
    let mut clargs = CLI::new();
    clargs.Parse_Args();

    if clargs.dbg{
        println!("{clargs:?}");
    }

    if let Err(e) = run_app() {
        eprintln!("Error running application: {}", e);
    }
}
