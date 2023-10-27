fn dump(s: &str){
    println!("{}", s);
}
fn main() {
    let text = "hello"; // static string, cant be changed.
    println!("{}", text);

    let s = text.to_string(); // allocated string, can be modified.
    dump(text);
    dump(&s);  

    let mut test_str = String::new();

    test_str.push_str("fahad ali sarwar");
    println!("{}", test_str);

}
