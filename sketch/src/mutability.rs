// mutability => ability to change
// mutable => can be changed
// immutable => can not be changed
pub fn test_mutability() {
    test_mut();
    test_shadowing();
}

fn test_mut() {
    // This block can not compile. Because by default all variables in rust is IMMUTABLE (can not be changed)
    // To by pass this issue you should declare variable with `mut` keyword as shown below
    /*
       let my_number = 3;
       println!("My Number is: {}", my_number);
       my_number = 5;
       println!("My Number is: {}", my_number);
    */
    let mut my_number = 3;
    println!("My Number is: {}", my_number);
    my_number = 5;
    println!("My Number is: {}", my_number);
}

fn test_shadowing() {
    // Shadowing works like creating new variable with prev name
    let my_number = 3;
    println!("My Number is: {}", my_number);
    let my_number = 5;
    println!("My Number is (shadowed): {}", my_number);
}
