// ---- main
fn main() {
    let nval: u32 = 0u32;
    let bool_val: bool = true;
    let unit: () = (); // unit type

    println!("{}, {}, {:?}", nval, bool_val, unit);

    // unused variable without warning
    let _unused_var: f64 = 0.0;

    // unused variable that emits warning
    let val: f64 = 3.14;
    {
        let _block_var = 30;

        // variable shadowing
        let val: i32 = -100;
        println!("interval Var: {:?}", val);
    }

    // variable shadowing
    let val: bool = true;
    println!("interval Var: {:?}", val);

    // ------------------------ Casting
    println!("bool {} = int {}", true, true as i32);
    println!("bool {} = int {}", false, false as i32);

    // ------------------------ size_of_val
    println!("size of int : {}", std::mem::size_of_val(&10i32));
    println!("size of f64 : {}", std::mem::size_of_val(&10.0f64));

    // ------------------------ Type Alias
    // Type name must have Camel-Case names
    // ex) LongInt, ShotInt, etc
    type Int = i32;
    type Double = f64;

    let _val: Int = 63;
    let _val: Double = 63.00;
}
