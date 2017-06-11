
fn main() {

    // ---- Tuple Dest...
    let tp01 = (0, -2);

    println!("> Tuple {:?}", tp01);

    match tp01 {
        (0, y) => println!("> (ZERO, {})", y),
        (x, 0) => println!("> ({}, ZERO)", x),
        _ => println!("> ...??? "),
    }

    // ---- Enum Destructuring
    #[allow(dead_code)]
    enum Color {
        RGB(u32, u32, u32),
        CMYK(u32, u32, u32, u32),
    }

    let color = Color::RGB(11, 22, 33);

    match color {
        Color::RGB(r, g, b) =>
            println!("RGB: {}, {}, {}", r, g, b),
        _ => println!("> No Match"),
    }


    // ---- Ref Destructuring
    #[allow(unused_variables)]
    let nref = &10;

    let ref nref = 20;

    match nref {
        &nval => println!("> Value by Destructuring: {}", nval),
    }

    match *nref {
        nval => println!("> Value by deferenced point match: {}", nval),
    }


    let value = 30;
    let mut mut_value = 40;

    match value {
        ref nref => println!("> Ref to a value = {:?}", nref),
    }

    match mut_value {
        ref mut mut_nref => {
            *mut_nref += 10;
            println!("> Mutable NRef value = {:?}", mut_nref);
        },
    }

}
