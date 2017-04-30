
fn main() {
    let a = [1, 2, 3, 4, 5];

    // ---- Sum 01
    let sum01:u32 = a.iter().fold(0, |sum, &i| sum + i);
    println!("> Sum = {}", sum01);

    // ---- Sum 02
    // the sum of all of the elements of a
    let sum02:u32 = a.iter().sum();
    println!("> Sum = {}", sum02);
}