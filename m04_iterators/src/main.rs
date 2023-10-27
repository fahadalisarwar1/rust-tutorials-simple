fn main() {
    let mut iter = 0..=3;

    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    let s: i32 = iter.sum();
    println!("{:?}", s);


    let arr = [10, 20, 40, 50, 60, 70, 80];
    let slice = &arr;

    for s in slice.windows(2){
        println!("{:?}", s);
    }
    /*
    [10, 20]
[20, 40]
[40, 50]
[50, 60]
[60, 70]
[70, 80]
     */

    // lets see how we can extend vectors
    let mut v= vec![10, 20, 40, 50, 60, 70];
    v.extend(0..5);

    
    println!("{:?}", v);
    // [10, 20, 40, 50, 60, 70, 0, 1, 2, 3, 4]

    
    }
