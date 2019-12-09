use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
enum Mode {
    Position,
    Intermediate,
    Relative
}

fn int_to_mode(mode: &i64)-> Mode{
    return match mode {
        0 => Mode::Position,
        1 => Mode::Intermediate,
        2 => Mode::Relative,
        _ => panic!("Unknown mode")
    };
}

#[derive(Debug)]
struct Instruction {
    code: i64,
    modes: HashMap<i64, Mode>
}

fn parse_instruction(instruction: i64) -> Instruction {
    let opcode = instruction % 100;
    let mode1 = instruction % 1000 / 100;
    let mode2 = instruction % 10000 / 1000;
    let mode3 = instruction / 10000;
    let mut modes = HashMap::new();
    modes.insert(1, int_to_mode(&mode1));
    modes.insert(2, int_to_mode(&mode2));
    modes.insert(3, int_to_mode(&mode3));
    return Instruction { code: opcode, modes: modes };
}


fn file_to_vec(filename: String) -> io::Result<Vec<i64>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    let rows : Vec<String> = file_reader.lines().filter_map(io::Result::ok).collect();
    let v: Vec<i64> = rows[0].split(",").map(|x| x.parse::<i64>().expect("parse error")).collect();

    return Ok(v);
}

fn get_value(idx: i64, mode: &Mode, relative_base: &i64, program: &HashMap<i64, i64>) -> i64{
    let pnt = program.get(&idx).unwrap_or(&0);
    return match mode {
        Mode::Position => *program.get(&pnt).unwrap_or(&0),
        Mode::Intermediate => *pnt,
        Mode::Relative => *program.get(&(pnt + relative_base)).unwrap_or(&0),
    }
}

fn get_index (idx: i64, mode: &Mode, relative_base: &i64, program: &HashMap<i64, i64>) -> i64 {
    let pnt = program.get(&idx).unwrap_or(&0);
    return match mode {
        Mode::Position => *pnt,
        Mode::Intermediate => idx,
        Mode::Relative => *pnt + *relative_base,
    };
}

fn run_program(program: &mut HashMap<i64, i64>, input: i64) -> Vec<i64>{
    let mut index = 0;
    let mut done = false;
    let mut output = Vec::new();
    let mut relative_base = 0;
    
    while !done {
        let instruction = parse_instruction(program[&index]);
        let mode1 = &instruction.modes[&1];
        let mode2 = &instruction.modes[&2];
        let mode3 = &instruction.modes[&3];
        match instruction.code {
            1 | 2 => {
                let val1 = get_value(index + 1, mode1, &relative_base, program);
                let val2 = get_value(index + 2, mode2, &relative_base, program);
                let insert_index = get_index(index + 3, mode3, &relative_base, program);
        
                let new_val = 
                    if instruction.code == 1 {
                        val1 + val2
                    }
                    else {
                        val1 * val2
                    };
                program.insert(insert_index, new_val);
                index += 4;
            },
            3 => {
                let idx = get_index(index + 1, mode1, &relative_base, program);
                program.insert(idx, input);
                index += 2;
            },
            4 => {
                let mode1 = &instruction.modes[&1];
                let val = get_value(index +1, mode1, &relative_base, program);
                output.push(get_value(index +1, mode1, &relative_base, program));
                index += 2;
            },
            5 => {
                let val1 = get_value(index + 1, mode1, &relative_base, program);
                if val1 != 0 {
                    index = get_value(index + 2, mode2, &relative_base, program);
                }
                else {
                    index += 3;
                }
            },
            6 => {
                let val1 = get_value(index + 1, mode1, &relative_base, program);
                if val1 == 0 {
                    index = get_value(index + 2, mode2, &relative_base, program);
                }
                else {
                    index += 3;
                }
            },
            7 => {
                let val1 = get_value(index + 1, mode1, &relative_base, program);
                let val2 = get_value(index + 2, mode2, &relative_base, program);
                let val3 = get_index(index + 3, mode3, &relative_base, program);
                if val1 < val2 {
                    program.insert(val3, 1);
                }
                else {
                    program.insert(val3, 0);
                }
                index += 4;
            },
            8 => {
                let val1 = get_value(index + 1, mode1, &relative_base, program);
                let val2 = get_value(index + 2, mode2, &relative_base, program);
                let val3 = get_index(index + 3, mode3, &relative_base, program);
                if val1 == val2 {
                    program.insert(val3, 1);
                }
                else {
                    program.insert(val3, 0);
                }
                index += 4;
            },
            9 => {
                relative_base += get_value(index + 1, mode1, &relative_base, program);
                index += 2;
            }
            99 => done = true,
            _ => {
                println!("Wrong opcode!!! {:?}", instruction.code);
                break;
            }
        }
    }

    return output;
} 

fn start_program(program: &Vec<i64>, input: i64)-> Vec<i64> {
    let mut map = HashMap::new();
    for (i, elem) in program.iter().enumerate(){
        map.insert(i as i64, *elem);
    }
    return run_program(&mut map, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jump_position_mode_zero_input(){
        let program = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
        let result = start_program(&program, 0);
        assert_eq!(result, vec![0]);
    }

    #[test]
    fn test_jump_position_mode_non_zero_input(){
        let program = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
        let result = start_program(&program, 5);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test_opcode_9(){
        let program = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
        let result = start_program(&program, 1);
        assert_eq!(result, program);
    }

    #[test]
    fn test_opcode_9_2(){
        let program = vec![104,1125899906842624,99];
        let result = start_program(&program, 1);
        assert_eq!(result, vec![1125899906842624]);
    }

    #[test]
    fn test_opcode_9_3(){
        let program = vec![1102,34915192,34915192,7,4,7,99,0];
        let result = start_program(&program , 1);
        assert_eq!(result, vec![1219070632396864]);
    }

    #[test]
    fn example_program_input_less_than_8(){
    let program = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
            1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
            999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
        let result = start_program(&program, 0);
        assert_eq!(result, vec![999]);
    }

    #[test]
    fn example_program_input_equal_8(){
        let program = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
            1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
            999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
        let result = start_program(&program, 8);
        assert_eq!(result, vec![1000]);
    }

    #[test]
    fn example_program_input_greater_than_8(){
        let program = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
            1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
            999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
        let result = start_program(&program, 13223);
        assert_eq!(result, vec![1001]);
    }
}


// Task 1: 2171728567
// Task 2: 49815
fn main() {
    let row = file_to_vec("data.txt".to_string());
    match row {
    Ok(numbers) => {
       let answer1 = start_program(&numbers, 1);
        println!("Task 1: {:?}", answer1);
       let answer2 = start_program(&numbers, 2);
        println!("Task 2: {:?}", answer2);
    },
    Err(e) => println!("Error reading file: {:?}", e)
    }   
}