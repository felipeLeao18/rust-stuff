use std::collections::BTreeMap;
use std::{env, path::PathBuf};

fn main() {
    let path = PathBuf::from(env::current_dir().unwrap().join("src/input.txt"));
    let buffer: String = std::fs::read_to_string(path).unwrap();

    let mut calories_by_elf: BTreeMap<i32, usize> = BTreeMap::new();

    for (i, calories_group) in buffer.split_terminator("\n\n").enumerate() {
        let mut total_calories_by_elf: i32 = 0;
        for calory in calories_group.split_terminator("\n") {
            total_calories_by_elf += calory.parse::<i32>().unwrap();
        }

        calories_by_elf.insert(total_calories_by_elf, i);
    }

    let result: Option<&i32> = calories_by_elf.keys().last();
    println!("{:?}", result);

    let top_three_sum: i32 = calories_by_elf
        .keys()
        .rev()
        .take(3)
        .fold(0, |acc, x| acc + x);

    println!("{}", top_three_sum);
}
