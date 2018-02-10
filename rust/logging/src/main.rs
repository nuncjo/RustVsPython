#[macro_use]
extern crate log;
extern crate env_logger;

fn logging_flow() {
    let modules = vec!["init", "analytics"];
    trace!("Starting Flow");
    if modules.contains(&"init") {
        info!("Loading init module..");
    } else {
        debug!("Module init not found");
        panic!("Required module not found!");
    }

    if modules.contains(&"security") == false {
        warn!("Module security not found!")
    }

}

fn main() {
    // run by typing in console: RUST_LOG="info,warn,debug,trace" cargo run
    env_logger::init();
    logging_flow();
    /* output:
    TRACE 2018-02-10T17:46:44Z: logging: Starting Flow
    INFO 2018-02-10T17:46:44Z: logging: Loading init module..
    WARN 2018-02-10T17:46:44Z: logging: Module security not found!
    */
}
