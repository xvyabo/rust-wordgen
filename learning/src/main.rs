fn main() {
    let x = "hello world!";
    println!("{}", x.chars().nth(0).unwrap().to_string().as_str());
}
