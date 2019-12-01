use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::fs;

fn file_to_vec(filename: String) -> io::Result<Vec<i32>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).map(|x| x.parse::<i32>().expect("parse error")).collect())
}

fn total_fuel(num: i32, res: i32) -> i32 {
    let next: i32 = num / 3 - 2;
    if next <= 0 {
        return res;
    }
    return total_fuel(next, res + next);
}


#[cfg(test)]
mod tests {
    use crate::total_fuel;

    #[test]
    fn test_total_fuel() {
        assert_eq!(total_fuel(1969, 0), 966);
    }
}

fn main() {
    let lines = file_to_vec("data.txt".to_string());
    match lines {
    Ok(numbers) => {
        let res1 : i32 = numbers.iter().map(|x| x / 3 - 2).sum::<i32>();
        println!("Task 1 : {:?}", res1);
        let res2 : i32 = numbers.iter().map(|x| total_fuel(*x, 0)).sum::<i32>();
        println!("Task 2 : {:?}", res2);
    },
    Err(e) => println!("Error reading file: {:?}", e)
    }   
}