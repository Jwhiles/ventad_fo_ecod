use std::fs;

fn main() {
    let input = fs::read_to_string("./input").expect("failed to read");
    let mut res = vec![0];

    let mut i = 0;
    for line in input.lines() {
        if line == "" {
            i = i + 1;
            res.push(0);
        } else {
            res[i] = res[i] + line.parse::<i32>().unwrap();
        }
    }
    let max = res.iter().max();
    match max {
        Some(m) => println!( "most calories value: {}", m ),
        None      => println!( "Vector is empty" ),
    };

    res.sort();
    res.reverse();
    println!("{}", res[0] + res[1] + res[2]);
}
