//use std::process::{ExitCode, Termination};
//use std::process::exit as doExit;
//use rand::Rng;

use std::thread::sleep;
use std::time::Duration;
use std::ops::Range;

fn main(){
//    println!("{}", exercise(9).to_string());
    
    let iterable = [5,6,7];

    let range: Range<i32> = (1..9);

    println!("Fibonacci: {}", fibonacci(25));
    
    for value in range.rev() {
        println!("{}", value)
    }
    
    looper(&|| {&1})
    
//    let mut cur: i32 = 0;
//    looper({
//        cur = cur + 1;
//        cur
//    });
    
//    doExit(1)
//    IntegerTermination::new(1)
}

fn fibonacci(nth: i32) -> i32{
    let mut current: i32 = 1;
    let mut previous: i32 = 0;
    let mut count: i32 = 0;
    
    loop{
        current = current + previous;
        previous = current;
        count = count + 1;
        
        if count == nth {
            break
        }
    }

    current
}

fn looper<T: ToString>(get_value: &Fn() -> T){
    loop{
        sleep(Duration::from_millis(500));
        
        println!("value: {}", get_value().to_string());
    }
}

fn exercise(num: i32) -> i32{
    let output = if num > 5 {
        9
    } else{
        3
    };
    
    output
}

//struct IntegerTermination{ code: i32 }
//
//impl IntegerTermination{
//    fn new(code: i32) -> IntegerTermination{
//        IntegerTermination { code }
//    }
//}
//
//impl std::process::Termination for IntegerTermination {
//    fn report(self) -> i32 {
//        self.code
//    }
//}