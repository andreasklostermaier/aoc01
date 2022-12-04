use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {

    solution_a();
 
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn solution_a() {

    let mut current_elf        = 1usize;
    let mut current_cal        = 0u32;
    let mut max_cal_elf        = 1usize;
    let mut max_cal            = 0u32;
    let mut cal_list: Vec<u32> = Vec::new();

    if let Ok(lines) = read_lines("./data/input.txt") {

        for calories in lines.flatten() {
            let parsed_cal = calories.parse::<u32>();
            match parsed_cal {
                Ok(cal) => {
                    current_cal += cal;
                },
                Err(_) => {
                    if current_cal > max_cal {
                        max_cal_elf = current_elf;
                        max_cal     = current_cal;
                    }
                    cal_list.push(current_cal);
                    current_elf += 1;
                    current_cal = 0;
                },
            }
        }
    }

    // Part 1
    println!("Solution A");
    println!("Part 1: Elf {} is carrying the most calories: {}", max_cal_elf, max_cal);

    // Part 2
    cal_list.sort();
    cal_list.reverse();
    let three_most_cals: u32 = cal_list[0..3].iter().sum();
    println!("Part 2: Accumulated calories of the three elves with the largest rations: {:?}\n", three_most_cals);

}


