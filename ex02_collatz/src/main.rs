use std::env;

fn main() {
    if env::args().len() < 2 {
        println!("> Usage: collatz [number]");
        return;
    }

    let i = env::args().nth(1).unwrap().parse::<i32>().unwrap();

    println!("> Collatz : {} -> 1 after {} steps", i, collatz(i));
}

fn collatz(_num: i32) -> i32 {
    println! ("> num : {}", _num);

    if _num == 1 || _num == 0 { return 0; }
    match _num % 2 {
        0 => { 1 + collatz(_num/2) }
        _ => { 1 + collatz(_num*3+1) }
    }
}
