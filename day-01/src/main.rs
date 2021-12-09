use std::fs;

fn main() {
    let values = read_values();
    let incs = find_increments_3(values);

    dbg!(incs);
}

fn find_increments_1(values: Vec<u32>) -> u32 {
    let mut incs = 0;
    let mut prev = values[0];

    for i in 1..values.len() {
        if values[i] > prev {
            incs = incs + 1;
        }

        prev = values[i];
    }

    incs
}

fn find_increments_3(values: Vec<u32>) -> u32 {
    let mut incs = 0;
    let mut prev = values[0];

    for i in 3..values.len() {
        if values[i] > prev {
            incs = incs + 1;
        }

        prev = values[i-2];
    }

    incs
}

fn read_values() -> Vec<u32> {
    let values = fs::read_to_string("/home/ivan/Liferay/Rust/advent-of-code-2021/day-01/input").unwrap();
    let values : Vec<&str> = values.trim().split("\n").collect();

    values.iter().map(|val| -> u32 { val.parse().unwrap() }).collect()
}
