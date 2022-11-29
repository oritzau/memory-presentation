fn main() {
    let ex_str = String::from("Hello World"); //ex_str is brought into scope with type String and value "Hello World"
    do_something(ex_str); //ownership of ex_str is given to function do_something()
    //ex_str is now owned by do_something() and does not exist in the scope of this main function
    
    //if we try to use ex_str now, the Rust compiler will throw an error, for example:
    println!("{ex_str}")
}

fn do_something(string:String) {
    println!("{}", string);
}