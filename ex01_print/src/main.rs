use std::fmt::{self, Formatter, Display};

struct Point2D {
    x: f32,
    y: f32
}

impl Display for Point2D {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        let ew = if self.x >= 0.0 {'E'} else {'W'};
        let ns = if self.y >= 0.0 {'N'} else {'S'};
        write!(fmt, "({:.3}, {:.3}) at {}{}", self.x, self.y, ns, ew)
    }
}

fn main() {
    // ---- Test Basic Formatting
    println!("Version: {}", 1.7);

    // With Indexed Arguments
    println!("{2} of {1} : {1} {0}", "red", "rose", "color");

    // With Named Arguments
    println!("{entity} of {element} : {element} {value}", value="red", element="rose", entity="color");

    println!("Binary Value of {0} = {0:b}", 255 );
    println!("Hex Value of {0} = {0:X}", 255 );
    println!("Octet Value of {0} = {0:o}", 255 );

    println!("Right-Aligned Text |{0:>8}|", 255 );
    println!("Right-Aligned Text |{0:>08}|", 255 );


    // ---- Test Display for Point2D
    for pt in [
        Point2D { x: 3.0, y: -10.3 },
        Point2D { x: -5.0, y: 8.1 }
    ].iter() {
        println!("> {}", *pt);
    }
}
