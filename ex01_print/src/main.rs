use std::fmt::{self, Formatter, Display};

struct Point2D {
    x: f32,
    y: f32
}

struct Point2D_noname (f32, f32);

impl Display for Point2D {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        let ew = if self.x >= 0.0 {'E'} else {'W'};
        let ns = if self.y >= 0.0 {'N'} else {'S'};
        write!(fmt, "({:.3}, {:.3}) at {}{}", self.x, self.y, ns, ew)
    }
}

fn print_slice(slice: &[i32]) {
    for num in slice.iter() {
        println!("- {}", num);
    }
}

fn main() {
    // ---- Test Basic Formatting
    println!("Version: {}", 1.7);

    // With Indexed Arguments
    println!("{2} of {1} : {1} {0}", "red", "rose", "color");

    // With Named Arguments
    println!("{entity} of {element} : {element} {value}", value="red", element="rose", entity="color");

    //
    let nval:i32 = 255;
    println!("Binary Value of {0} = {0:b}", nval);
    println!("Hex Value of {0} = {0:X}", nval);
    println!("Octet Value of {0} = {0:o}", nval);

    println!("Right-Aligned Text |{0:>08}|", nval);
    println!("Left-Aligned Text |{0:<8}|", nval);

    // ---- Test Display for Point2D
    for pt in [
        Point2D { x: 3.0, y: -10.3 },
        Point2D { x: -5.0, y: 8.1 }
    ].iter() {
        println!("> {}", *pt);
    }

    // ---- Print Array(Slice)
    let a_n: [i32;5] = [10, 20, 30, 40, 50];
    print_slice(&a_n[2..5]);
}
