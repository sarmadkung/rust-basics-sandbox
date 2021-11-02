// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data
pub fn run() {
    // primitive
    let whr = "Where are you";

    // growable
    let mut name = String::from("What is your name ?");
   
    // length
    let len = name.len();

    name.push(' ');
    name.push_str("Hmm!");
    println!("{:?}",(whr,name,len));
}