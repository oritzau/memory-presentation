fn main() { //main scope created
    if 2 == 2 { //subscope created
        let ex_str = String::from("hello"); //ex_str brought into subscope
    } //subscope ends
    println!("{}", ex_str);
} //main scope ends
