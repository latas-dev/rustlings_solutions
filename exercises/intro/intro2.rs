// intro2.rs
// Make the code print a greeting to the world.
// Execute `rustlings hint intro2` or use the `hint` watch subcommand for a hint.


fn main() {
    let my_var = "World";
    let my_string: String = String::from("World");

    println!("Hello {}!", my_var);
    println!("Hello {}!", my_string);
    println!("Hello {}!", &my_string[2..4]);
}
