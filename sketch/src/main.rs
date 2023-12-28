pub mod examples;
pub mod intro;
pub mod mutability;
pub mod structs;

fn main() {
    intro::test_intro();
    println!("\n--- # Struct Tests");
    structs::test_struct();
    println!("\n--- # Tuple Tests");
    structs::test_tuple();
    println!("\n--- # Enum Tests");
    structs::test_enums();
    println!("\n--- # Mutability Tests");
    mutability::test_mutability();
    println!("\n--- # Examples Tests");
    examples::test_examples();
}
