pub fn run() {
    // Print to console
    println!("Hello from print file");
    // Basic formating
    println!("Number: {}",1);
    println!("{} is from {}","Sarmad", "Pakistan");
    
    // positional arguments
    println!("{0} is from {1} and {0} likes {2}","Sarmad","Pakistan","Coding");

    // named arguments
    println!("{name} is from {lives} and {name} likes {like}", name = "Sarmad",lives="Pakistan", like="Coding");

    // Placesholder traits
    println!("Binary : {:b} Hex {:x} Octal {:o} ", 10,10,10);

    // Placesholder for debug traits
    println!("{:?}", (true,10,"Hello"));

    // Basic Math
    println!("10 + 10 = {}", 10+10);


}