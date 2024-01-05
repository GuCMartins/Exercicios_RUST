// primitive_types5.rs
//
// Destructure the `cat` tuple so that the println will work.
//
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let cat = ("Furry McFurson", 3.5,"Black");
    let (name, age, fur) = cat;//recebe os valores da tupla em ordem

    println!("{} is {} years old, with {} fur", name, age,fur);
}
