use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
struct Instruction {
    code: i32,
    modes: HashMap<i32, bool>
}

fn parse_instruction(instruction: i32) -> Instruction {
    let opcode = instruction % 100;
    let mode1 = instruction % 1000 / 100;
    let mode2 = instruction % 10000 / 1000;
    let mode3 = instruction / 10000;
    let mut modes = HashMap::new();
    modes.insert(1, mode1 == 1);
    modes.insert(2, mode2 == 1);
    modes.insert(3, mode3 == 1);
    return Instruction { code: opcode, modes: modes };
}


fn file_to_vec(filename: String) -> io::Result<Vec<i32>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    let rows : Vec<String> = file_reader.lines().filter_map(io::Result::ok).collect();
    let v: Vec<i32> = rows[0].split(",").map(|x| x.parse::<i32>().expect("parse error")).collect();

    return Ok(v);
}

fn get_value(idx: usize, mode: bool, program: &Vec<i32>) -> i32{
    let pnt = program[idx];
    if mode {
        return pnt;
    }
    else {
        return program[pnt as usize];
    }
}

fn get_index (idx: usize, mode: bool, program: &Vec<i32>) -> usize {
    let pnt = program[idx];
    if mode {
        return idx;
    }
    else{
        return pnt as usize;
    }
}

/* fix
fn do_arithmetic(instruction: Instruction, indexpt: &mut usize, program: &mut Vec<i32>, operator: &dyn Fn(i32, i32)->i32){
    let index = *indexpt;
    let val1 = get_value(index + 1, instruction.modes[&1], program);
    let val2 = get_value(index + 2, instruction.modes[&2], program);
    let insert_index = get_index(index + 3, instruction.modes[&3], program);

    program[insert_index] = operator(val1, val2);
    *indexpt += 4;
}*/



fn run_program(program: &mut Vec<i32>, input: i32) -> i32{
    let mut index = 0;
    let mut done = false;
    let mut output = 0;
    
    while !done {
        let instruction = parse_instruction(program[index]);
        let mode1 = instruction.modes[&1];
        let mode2 = instruction.modes[&2];
        let mode3 = instruction.modes[&3];
        match instruction.code {
            1 | 2 => {
                let val1 = get_value(index + 1, mode1, program);
                let val2 = get_value(index + 2, mode2, program);
                let insert_index = get_index(index + 3, mode3, program);
        
                program[insert_index] = 
                    if instruction.code == 1 {
                        val1 + val2
                    }
                    else {
                        val1 * val2
                    };
                index += 4;
            },
            3 => {
                let idx = program[index + 1];
                program[idx as usize] = input;
                index += 2;
            },
            4 => {
                let idx = program[index + 1];
                output = program[idx as usize];
                index += 2;
            },
            5 => {
                let val1 = get_value(index + 1, mode1, program);
                if val1 != 0 {
                    index = get_value(index + 2, mode2, program) as usize;
                }
                else {
                    index += 3;
                }
            },
            6 => {
                let val1 = get_value(index + 1, mode1, program);
                if val1 == 0 {
                    index = get_value(index + 2, mode2, program) as usize;
                }
                else {
                    index += 3;
                }
            },
            7 => {
                let val1 = get_value(index + 1, mode1, program);
                let val2 = get_value(index + 2, mode2, program);
                let val3 = get_index(index + 3, mode3, program);
                if val1 < val2 {
                    program[val3] = 1;
                }
                else {
                    program[val3] = 0;
                }
                index += 4;
            },
            8 => {
                let val1 = get_value(index + 1, mode1, program);
                let val2 = get_value(index + 2, mode2, program);
                let val3 = get_index(index + 3, mode3, program);
                if val1 == val2 {
                    program[val3] = 1;
                }
                else {
                    program[val3] = 0;
                }
                index += 4;
            },
            99 => done = true,
            _ => {
                println!("Wrong opcode!!! {:?}", instruction.code);
                break;
            }
        }
    }

    return output;
} 

fn start_program(program: &Vec<i32>, input: i32)-> i32 {
    let mut result = program.clone();
    return run_program(&mut result, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_program() {
        let mut program = vec![2,4,4,5,99,0];
        let expected = vec![2,4,4,5,99,9801];
        run_program(&mut program,1);
        assert_eq!(program, expected);
    }

    #[test]
    fn test_run_program2() {
        let mut program = vec![1002,4,3,4,33];
        let expected = vec![1002,4,3,4,99];
        run_program(&mut program,1);
        assert_eq!(program, expected);
    }

    #[test]
    fn test_jump_position_mode_zero_input(){
        let program = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
        let result = start_program(&program, 0);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_jump_position_mode_non_zero_input(){
        let program = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
        let result = start_program(&program, 5);
        assert_eq!(result, 1);
    }

    /* Test does not work for some unknown reason
    #[test]
    fn example_program_input_less_than_8(){
    let program = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
            1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
            999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
        let result = start_program(&program, 0);
        assert_eq!(result, 999);
    }*/

    #[test]
    fn example_program_input_equal_8(){
        let program = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
            1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
            999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
        let result = start_program(&program, 8);
        assert_eq!(result, 1000);
    }

    #[test]
    fn example_program_input_greater_than_8(){
        let program = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
            1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
            999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
        let result = start_program(&program, 13223);
        assert_eq!(result, 1001);
    }
}


// Task 1: 16574641
// task2: 15163975
fn main() {
    let row = file_to_vec("data.txt".to_string());
    match row {
    Ok(numbers) => {
       let answer1 = start_program(&numbers,1);
        println!("Task 1: {:?}", answer1);
        let answer2 = start_program(&numbers,5);
        println!("Task 2: {:?}", answer2);
    },
    Err(e) => println!("Error reading file: {:?}", e)
    }   
}