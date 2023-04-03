use std::{collections::HashMap, env, path::PathBuf};

fn main() {
    let path = PathBuf::from(env::current_dir().unwrap().join("src/input.txt"));
    let buffer: String = std::fs::read_to_string(path).unwrap();

    let mut calories_by_elf: HashMap<i32, usize> = HashMap::new();

    for (i, calories_group) in buffer.split_terminator("\n\n").enumerate() {
        let mut total_calories_by_elf: i32 = 0;
        for calory in calories_group.split_terminator("\n") {
            total_calories_by_elf += calory.parse::<i32>().unwrap();
        }

        calories_by_elf.insert(total_calories_by_elf, i);
    }

    let result: Option<&i32> = calories_by_elf.keys().max();
    println!("{:?}", result);
}
