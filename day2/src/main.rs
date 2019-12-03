use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::fs;

fn file_to_vec(filename: String) -> io::Result<Vec<i32>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    let rows : Vec<String> = file_reader.lines().filter_map(io::Result::ok).collect();
    let v: Vec<i32> = rows[0].split(",").map(|x| x.parse::<i32>().expect("parse error")).collect();

    return Ok(v);
}

fn run_program(program: &mut Vec<i32>) {
    let mut index = 0;
    let mut done = false;
    
    while !done {
        let opcode = program[index];
        if opcode == 1 || opcode == 2 {
            let idx1 = program[index + 1];
            let idx2 = program[index + 2];
            let idx3 = program[index + 3];
            program[idx3 as usize] = 
                if opcode == 1 {
                    program[idx1 as usize] + program[idx2 as usize]
                }
                else {
                    program[idx1 as usize] * program[idx2 as usize]
                };
            index += 4;
        } 
        else {
            done = true;
        }
    }
} 

fn start_program(program: &Vec<i32>, noun: i32, verb: i32)-> i32 {
    let mut result = program.clone();
    result[1] = noun;
    result[2] = verb;
    run_program(&mut result);
    return result[0];
}


fn task2(program: &Vec<i32>) -> i32 {
    for noun in 0..99{
        for verb in 0..99{
            if start_program(program, noun, verb) == 19690720 {
                return noun * 100 + verb;
            }
        }
    }

    return -1;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_program() {
        let mut program = vec![2,4,4,5,99,0];
        let expected = vec![2,4,4,5,99,9801];
        run_program(&mut program);
        assert_eq!(program, expected);
    }

    #[test]
    fn test_start_program(){
        let program = vec![1,0,0,4,99,5,6,0,99];
        assert_eq!(start_program(&program, 1, 1), 30);
    }
}

fn main() {
    let row = file_to_vec("data.txt".to_string());
    match row {
    Ok(numbers) => {
        let answer1 = start_program(&numbers,12, 2);
        println!("Task 1: {:?}", answer1);
        let answer2 = task2(&numbers);
        println!("Task 2: {:?}", answer2);
    },
    Err(e) => println!("Error reading file: {:?}", e)
    }   
}
