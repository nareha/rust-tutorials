fn main() {
    // Mutability
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Constant
    const FOO_BAR: u8 = 5;
    println!("The value of FOO_BAR is: {FOO_BAR}");

    // Shadowing
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The number of spaces is: {spaces}");
}
