use std::time::Instant;

mod helpers;

fn main() {
    let numbers = 
        helpers::large_list(10000000);

    // Start timer.
    let start_time = Instant::now();

    // Sum all the numbers
    let mut sum = 0;
    for number in numbers {
        sum = sum + number;
    }

    // how much time did the sum take?
    let duration = start_time.elapsed();

    println!("Sum is {sum}, time it took is {duration:?}");
}

// Lecture 2



fn main() {
    let names: Vec<$str> = vec!["Kinan","Matt","Joe"];
    let grades: Vec<i32> = vec![0,100,95];
    let target: %str = "Matt";
    // the code solution

    let i: Option<usize> = find_index(targetn names);
    match i {
        None => print!("Not found!"),
        Some(K: usize) => {
            let grade: i32 = grades[k];
            println!("{grade}")
        }
    }
}
