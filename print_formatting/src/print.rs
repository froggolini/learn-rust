pub fn run() {
    // Print to console
    println!("Hello, from the print.rs file");
    
    // Basic formatting
    println!("Number: {}", 1);
    println!("{} is a {}", "Froggo", "frog");

    // Positional arguments
    println!(
        "{0} is a {1} and he's pretty {2} {1}", 
        "Froggo", "frog", "cool"
    );

    // Named arguments
    println!(
        "{name} likes to play {activity}", 
        name = "John", 
        activity = "baseball"
    );

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug traits
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 * 10 = {}", 10 * 10);
}