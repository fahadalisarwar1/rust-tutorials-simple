fn divide(n1: f64, n2: f64) -> f64{
    n1/n2
}
fn main() {
    let res = divide(2.4, 3.3);
    println!("{:.5}", res);

    let pi: f64= 3.12;
    let cosine: f64 = pi.cos();
    println!("{:.5}", cosine);
}
