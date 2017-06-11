fn main() {
    let mut count = 0u32;

    // ---- loop
    loop {
        count += 1;

        if count == 3 {
            continue;
        }

        if count == 5 {
            break;
        }

        println!("> Count = {}", count);
    }


    // ---- while
    let mut n = 1;

    while n < 101 {
        n += 1;
    }
    println!("> n = {}", n);

    let mut is_bool: bool;
    for n in 1..31 {
        if n % 15 == 0 {
            println!("> {}", n);
        }

        is_bool =
            if n % 2 == 0 {
                true
            } else {
                false
            };

        println!("> Is {} is even? {} ", n, is_bool);
    }


    // ---- Match
    for n in 1..20 {
        match n {
            1 => println!("> One!"),
            2 | 3 | 5 | 7 | 11 => println! ("> Prime {}", n),
            _ => println!("> normal {}", n),
        }
    }

}