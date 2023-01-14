use super::super::get_macros;
use super::super::execute::run_macro_initiator;
use super::Initiator;
use super::super::Macro;

use crony::{Job, Runner, Schedule};
// use std::thread;
use derive_new::new;

#[derive(new)]
struct ExampleJob {
    initiator: Initiator,
    macro_: Macro
}

impl Job for ExampleJob {
    fn schedule(&self) -> Schedule {
        self.initiator.data.cron.as_ref().unwrap().parse().unwrap()
    }
    fn handle(&self) {
        run_macro_initiator(self.initiator.clone(), self.macro_.clone())
    }
}

static mut RUNNER: Option<Runner> = None;

pub fn listen_initiator_cron() {
    unsafe {
        match &RUNNER {
            Some(_) => {
                RUNNER.take().unwrap().stop();
            }
            None => {}
        }
    };

    let macros: Vec<Macro> = get_macros();
    
    let mut runner: Runner = Runner::new();

    for macro_ in macros {
        let initiators: Vec<Initiator> =  macro_.clone().macro_.initiators.unwrap_or_else(|| vec![]);
        for initiator in initiators {
            if initiator.type_ == "time" {
                runner = runner.add(Box::new(ExampleJob::new(
                    initiator.clone(),
                    macro_.clone()
                )));
            }
        }
    }

    runner = runner.run();

    unsafe {
        RUNNER = Some(runner);
    }
}