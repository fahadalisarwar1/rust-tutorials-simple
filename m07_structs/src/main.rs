#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    fn new(first_name: &str, last_name: &str) -> Person {
        Person {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
        }
    }

    fn full_name(&self) -> String{
        format!("{} {}", self.first_name, self.last_name)
    }
}

fn main() {
    let p = Person::new("John", "ali");

    println!("{:?}", p);
    println!("{}", p.full_name());
}
