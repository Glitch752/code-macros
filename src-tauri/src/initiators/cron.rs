// use super::super::get_macros;
// use super::super::execute::run_macro_initiator;
// use super::Initiator;
// use super::super::Macro;

use crony::{Job, Runner, Schedule};
// use std::thread;
use derive_new::new;

#[derive(new)]
struct ExampleJob {
    cron: String
}

impl Job for ExampleJob {
    fn schedule(&self) -> Schedule {
        self.cron.parse().unwrap()
    }
    fn handle(&self) {
        println!("Hello, I am a cron job running at: {}", self.now());
    }
}

pub fn listen_initiator_cron() {
    // let macros: Vec<Macro> = get_macros();
    let mut runner: Runner = Runner::new();

    runner = runner.add(Box::new(ExampleJob::new("* * * * * *".to_string())));

    runner.run();
}