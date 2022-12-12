use clokwerk::{Scheduler, TimeUnits};

pub fn setup_analyzer() -> Scheduler {
    let mut scheduler = Scheduler::new();

    scheduler.every(10.seconds()).run(|| {
        println!("Hello, world!");
    });

    scheduler
}
