/** Variables (declared with `let`) are immutable by default. Mutability can be introduced with the `mut` keyword. */
fn mutability() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

/**
Constants are variables that:
- Cannot be mutable
- Can pre-calculate their values
- Must declare their type
*/
fn constants() {
    const THREE_HOURS: u32 = 60 * 60 * 3;
    println!("Three Hours In Ms: {THREE_HOURS}");
}

/**
Shadowing is the practice of redeclaring an immutable variable's value (using the "let" and the same variable name). By
redeclaring the value, we change it for the remainder of the scope. This technique also means that the type of a
shadowed variable can change, as it's redeclaration is no different that a normal variable declaration.
*/
fn shadowing() {
    let x = 5; // Immutable Value

    let x = x + 1; // Another Immutable Value - Overriding the value of X.

    {
        let x = x * 2; // Shadowing only exists until the end of the scope.
        println!("The value of x is: {x}"); // x = 12 (6 * 2)
    }

    println!("The value of x is: {x}"); // x = 6 (5 + 1)

    let spaces = "    "; // Type: &'static str
    let spaces = spaces.len(); // Type: usize
    println!("Number of spaces: {spaces}");
}

fn main() {
    mutability();
    constants();
    shadowing();
}
