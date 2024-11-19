fn main() {
    let my_vec = vec![8, 9, 10, 10, 55, 99];
    print_all_three_things(my_vec);
}

fn print_all_three_things(vector: Vec<i32>) {
    if vector.len() != 3 {
        panic!("vector must always have three items");
    }
    println!("{}, {}, {}", vector[0], vector[1], vector[2]);
}
