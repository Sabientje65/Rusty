use core::borrow::Borrow;
use rand::Rng;
use std::cmp::Ordering;
use std::str::FromStr;
use std::num::Wrapping;

const MIN: u32 = 0;
const MAX: u32 = 5;

fn main(){
    
    let input = std::io::stdin();
    let value = generate_number();
    
    loop {
        let guess: u32 = read_valid_guess(&input);
        let guess2: &str = "String pöinter";
        let mut guess2: u64 = 23;
        
//        let wrapped = Wrapping(23);
//        wrapped + 3;
        
        let compound: [i32; 3] = [3,4,5];
        let tupper = (500, 42, 23);
        let p = tupper.0;
        
//        let len: Fn<u32> = compound.len;
//        let len = len();
        
        match guess.cmp(&value){
            Ordering::Less => println!("Smöl"),
            Ordering::Equal => {
                println!("Ok");
                break;
            },
            Ordering::Greater => println!("Too big m8")
        }   
    }
    
    println!("Sup bro");
}
/*
Benor
*/
fn read_valid_guess<T: FromStr>(input: &std::io::Stdin) -> T{
    let mut guess = String::new();
    input.read_line(&mut guess);

    match guess
        .trim()
        .parse() {
            Ok(v) => v,
            Err(_) => {
                println!("Enter a value, you fuckwad");
                read_valid_guess(&input)
            }
    }
}

fn generate_number() -> u32{
    rand::thread_rng().gen_range(MIN, MAX)
}