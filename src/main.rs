use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        
        let nums: Vec<i32> = lines.map(|v| v.unwrap().parse().unwrap()).collect();

        for (i, num1) in nums.iter().enumerate() {
            for (j, num2) in nums[i..].iter().enumerate() {
                if num1 + num2 == 2020 {
                    println!("2-sum {}", num1 * num2)
                }

                if let Some(val) = &nums[j..].iter().find(|&&v| v == 2020 - (num1 + num2)) {
                    println!("3-sum {}", *val * num1 * num2)
                }
            }
        }
    }
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
  