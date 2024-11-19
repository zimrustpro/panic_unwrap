fn main() {
    let vector = vec![None, Some(1000)];
    try_two_unwraps(vector);
}

fn try_two_unwraps(input: Vec<Option<i32>>) {
    println!("Index 0 is: {}", input[0].unwrap());
    println!("Index 1 is: {}", input[1].unwrap());
}
