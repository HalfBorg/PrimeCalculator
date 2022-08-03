use log::{error, info, warn, debug, trace};
use log4rs;
use std::{thread, time};

fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    #[cfg(debug_assertions)]
    {
        info!("Program started, this is a DEBUG");
    }
    #[cfg(not(debug_assertions))]
    {
        info!("Program started, this is RELEASE!");
    }

        trace!("My trace");
        debug!("My Debug");
        info!("My info");
        warn!("My warning");
        error!("My error");

    thread::sleep(time::Duration::from_millis(100));
}