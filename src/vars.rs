// Variables hild primitive data or refrences to data
// Variables are imutable by default
// Rust is block-scoped language
pub fn run () {
    let name = "Sarmad";
    let mut age = 26;
    println!("M name is {} and i am {} years old",name, age);
    age = 27;
    println!("M name is {} and i am {} years old",name, age);
// Define Constant
const ID: i32 = 001;
println!("ID: {}",ID);
// Assign mutiple vars
let ( _name , _age ) = ("Sarmad",26);
println!("M name is {} and i am {} years old",_name, _age);

}