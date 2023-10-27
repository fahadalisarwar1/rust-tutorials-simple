fn sum(values: &[i32])->i32{
    let mut summation = 0;
    for i in 0..values.len()    {
        summation += values[i]
    }
    summation
}

fn main() {
    let arr: [i32; 5] = [10, 244, 40, 77,88];
    let first = arr[0];
    println!("{}", first);
    for i in 0..=2{
        println!("{}", arr[i]);
    }
    println!("{}", arr.len());
    // println!("{}", arr[5]);

    // slices
    let answer = sum(&arr);
    println!("{}", answer);

    println!("{:?}", arr);
    let arr_slice = &arr[0..2];
    println!("{:?}", arr_slice);
}
