use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines(day_num: u8) -> io::Result<io::Lines<io::BufReader<File>>> {
    let path_name = format!("./inputs/day{}.txt", day_num);
    let path = Path::new(&path_name);

    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}
