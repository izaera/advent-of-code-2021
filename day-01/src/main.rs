use std::fs;

fn main() {
    let values = read_values();

    dbg!(find_increments(&values, 1));
    dbg!(find_increments(&values, 3));
}

fn find_increments(values: &Vec<u32>, window: usize) -> u32 {
    let mut incs = 0;
    let mut prev = values[0];

    for i in window..values.len() {
        if values[i] > prev {
            incs = incs + 1;
        }

        prev = values[i-(window - 1)];
    }

    incs
}

fn read_values() -> Vec<u32> {
    let values = fs::read_to_string("/home/ivan/Liferay/Rust/advent-of-code-2021/day-01/input").unwrap();
    let values : Vec<&str> = values.trim().split("\n").collect();

    values.iter().map(|val| -> u32 { val.parse().unwrap() }).collect()
}
