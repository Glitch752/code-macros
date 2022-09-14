use super::super::get_macros;
// use super::super::execute::run_macro_initiator;
use super::Initiator;
use super::super::Macro;

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

static mut RUNNER: Option<Runner> = None;

pub fn listen_initiator_cron() {
    println!("updating");
    unsafe {
        match &RUNNER {
            Some(runner) => {
                RUNNER.take().unwrap().stop();
            }
            None => {}
        }
    };

    let macros: Vec<Macro> = get_macros();

    let mut cron_initiators: Vec<Initiator> = vec![];
    for macro_ in macros {
        let initiators: Vec<Initiator> =  macro_.macro_.initiators.unwrap_or(vec![]);
        for initiator in initiators {
            if initiator.type_ == "time" {
                cron_initiators.push(initiator);
            }
        }
    }

    let mut runner: Runner = Runner::new();

    for cron_initiator in cron_initiators {
        runner = runner.add(Box::new(ExampleJob::new(
            cron_initiator.data.cron.unwrap().to_string()
        )));
    }

    runner = runner.run();

    unsafe {
        RUNNER = Some(runner);
    }
}