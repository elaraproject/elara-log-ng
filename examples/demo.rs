use elara_log::prelude::*;

fn main() {
    Logger::new().init().unwrap();
    
    info!("Some {} info", "important");
    debug!("Debug message");
    warn!("There's a problem!");
    
    if true {
        error!("Panic!!!!")
    }
}
