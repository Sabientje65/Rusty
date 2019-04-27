use std::process::{ExitCode, Termination};
use std::process::exit as doExit;
//use rand::Rng;

fn main() -> IntegerTermination {
//    doExit(1)
    IntegerTermination::new(1)
}

struct IntegerTermination{ code: i32 }

impl IntegerTermination{
    fn new(code: i32) -> IntegerTermination{
        IntegerTermination { code }
    }
}

impl std::process::Termination for IntegerTermination {
    fn report(self) -> i32 {
        self.code
    }
}