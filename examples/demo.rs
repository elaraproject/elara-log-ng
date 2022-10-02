use elara_log::Logger;

fn main() {
    let mut log = Logger::new();
    
    log.info("Some info");
    log.debug("Debug message");
    log.warn("There's a problem!");
    
    if true {
        log.error("Panic!!!!")
    }
}
