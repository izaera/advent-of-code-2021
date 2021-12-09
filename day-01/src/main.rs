use std::fs;

fn main() {
    let values = fs::read_to_string("/home/ivan/Liferay/Rust/advent-of-code-2021/day-01/input").unwrap();
    let values : Vec<&str> = values.trim().split("\n").collect();
    let values : Vec<u32> = values.iter().map(|val| -> u32 { val.parse().unwrap() }).collect();

    let mut incs = 0;
    let mut prev = values[0];

    for i in 1..values.len() {
        if values[i] > prev {
            incs = incs + 1;
        }

        prev = values[i];
    }

    dbg!(incs);
}
