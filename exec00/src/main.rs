extern crate rand;

use std::io;
use std::io::Write;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    //---- io write excample
    {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        handle.write(b"Hello, Rust\n").expect("Error");
    }

    //---- Using println! macro
    println!("> Start");

    //---- Generate random number
    let sec_no = rand::thread_rng().gen_range(0,100);
    println!("> Sec No = {}", sec_no);

    //---- Main Loop
    loop {
        println!("> Input No ");
        let mut user_in = String::new();
        io::stdin().read_line(&mut user_in).expect("@@Error: read_line");

        println!("> Your Input: {}", user_in.trim());
        let guessed_no: u32 = match user_in.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println! ("> Wrong Format, Ignored");
                continue;
            }
        };

        //---- u32 compare method #1
        {
            if guessed_no < sec_no {
                println!("> Smaller");
            } else if guessed_no.eq(&sec_no) {
            //} else if guessed_no == sec_no {
                println!("> Match!");
            } else if guessed_no > sec_no {
                println!("> Bigger");
            }
        }

        //---- u32 compare method #2
        match guessed_no.cmp(&sec_no) {
            Ordering::Less => println!("> Smaller"),
            Ordering::Greater => println!("> Bigger"),
            Ordering::Equal => {
                println!("> Match!");
                break;
            }
        }
    }
}
