fn main() {
    let ints = [1,2,3,4,5];
    let slice = &ints;

    let last = slice.get(5); // This returns an option variable 
    // This will return None since there is no element 5 in the arr. 

    println!("{:?}", last);
    let second_last = slice.get(4);
    println!("{:?}", second_last);
    // output = Some(4)
    // to get the actual number
    println!("{:?}", second_last.unwrap());
    // it will print 5

    // you can also check if something is none or not. 
    println!("{:?}", second_last.is_none()); // This is false because it is not none/ 
    
    println!("{:?}", last.is_none()); // This is true


    // Let's talk about vectors now

    let mut myvec: Vec<i32> = Vec::new();
    myvec.push(10);
    myvec.push(20);
    myvec.push(30); // returns option.


    let get_last = myvec.pop();
    println!("{:?}", get_last);

}
