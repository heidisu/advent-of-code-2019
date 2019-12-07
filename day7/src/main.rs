mod permutations;
use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::fs;
use std::collections::HashMap;
use permutations::*;

#[derive(Debug)]
struct Instruction {
    code: i64,
    modes: HashMap<i64, bool>
}

fn parse_instruction(instruction: i64) -> Instruction {
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


fn file_to_vec(filename: String) -> io::Result<Vec<i64>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    let rows : Vec<String> = file_reader.lines().filter_map(io::Result::ok).collect();
    let v: Vec<i64> = rows[0].split(",").map(|x| x.parse::<i64>().expect("parse error")).collect();

    return Ok(v);
}

fn get_value(idx: usize, mode: bool, program: &Vec<i64>) -> i64{
    let pnt = program[idx];
    if mode {
        return pnt;
    }
    else {
        return program[pnt as usize];
    }
}

fn get_index (idx: usize, mode: bool, program: &Vec<i64>) -> usize {
    let pnt = program[idx];
    if mode {
        return idx;
    }
    else{
        return pnt as usize;
    }
}


fn run_program(program: &mut Vec<i64>, inputs: Vec<i64>) -> i64{
    let mut index = 0;
    let mut input_index = 0;
    loop {
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
                program[idx as usize] = inputs[input_index];
                if input_index < inputs.len() - 1 {
                    input_index +=1;
                }
                //println!("input: {:?}", input_index);
                index += 2;
            },
            4 => {
                let idx = program[index + 1];
                return program[idx as usize];
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
            99 => {
                panic!("Halt");
            },
            _ => {
                panic!("Wrong opcode: {:?}", instruction.code);
            }
        }
    }
} 

fn start_program(program: &Vec<i64>, input: Vec<i64>)-> i64 {
    let mut program = program.clone();
    return run_program(&mut program, input);
}

fn run_amplifier(program: &Vec<i64>, phase: i64, input: i64) -> i64{
    let inputs = vec![phase, input];
    return start_program(program, inputs);
}

fn run_amplifiers(program: &Vec<i64>, phases: &Vec<i64>) -> i64{
    return phases.into_iter().fold(0, |output, phase| run_amplifier(program, *phase, output));
}

fn get_max_thruster(program: &Vec<i64>)->i64{
    return permutations(0, 4).map(|v| run_amplifiers(program, &v.iter().map(|x| *x as i64).collect::<Vec<i64>>())).max().unwrap();
}

// does not compute.... :-(
fn run_amplifiers_loop(program: &Vec<i64>, phases: &Vec<i64>)-> i64{
    let program1 = program.clone();
    let program2 = program.clone();
    let program3 = program.clone();
    let program4 = program.clone();
    let program5 = program.clone();
    let mut programs = vec![program1, program2, program3, program4, program5]; 

    let mut input = 0;

    for i in 0 .. 5 {
        input = run_program(&mut programs[i], vec![phases[i], input]);
        println!("input: {:?}", input);
    }

    let mut index = 0;
    let mut counter = 0;
    while counter < 10000 {
        if index == 0 {
            println!("program1: {:?}", programs[0]);
        }
        input = run_program(&mut programs[index], vec![input]);
        println!("input: {:?}", input);
        if index == programs.len() - 1{
            index = 0;
        } else {
            index += 1;
        }
        counter += 1;
        if counter % 1000 == 0 {
            println!("counter: {:?}", counter)
        }
    }
    return input;
}

fn get_max_thruster_loop(program: &Vec<i64>)->i64{
    return permutations(5, 9).map(|v| run_amplifiers(program, &v.iter().map(|x| *x as i64).collect::<Vec<i64>>())).max().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jump_position_mode_zero_input(){
        let program = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
        let input = vec![0];
        let result = start_program(&program, input);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_run_amplifiers(){
        let program = vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];
        let phases = vec![4,3,2,1,0];
        let result = run_amplifiers(&program, &phases);
        assert_eq!(result, 43210)
    }

    #[test]
    fn test_run_amplifiers_2(){
        let program = vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0];
        let phases = vec![0,1,2,3,4];
        let result = run_amplifiers(&program, &phases);
        assert_eq!(result, 54321)
    }

    #[test]
    fn test_run_amplifiers_3(){
        let program = vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0];
        let phases = vec![1,0,4,3,2];
        let result = run_amplifiers(&program, &phases);
        assert_eq!(result, 65210)
    }

    #[test]
    fn get_max_thrusters(){
        let program = vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];
        let result = get_max_thruster(&program);
        assert_eq!(result, 43210)
    }
    

    #[test]
    fn test_get_max_thrusters_2(){
        let program = vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0];
        let result = get_max_thruster(&program);
        assert_eq!(result, 54321)
    }

    #[test]
    fn test_get_max_thrusters_3(){
        let program = vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0];
        let result = get_max_thruster(&program);
        assert_eq!(result, 65210)
    }

    //#[test]
    fn test_max_thrust_loop(){
        let program = vec![3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5];
        let phases = vec![9,8,7,6,5];
        let result = run_amplifiers_loop(&program, &phases);
        assert_eq!(result, 139629729)
    }  

    //#[test]
    fn test_max_thrust_loop_2(){
        let program = vec![3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5];
        let phases = vec![9,8,7,6,5];
        let result = run_amplifiers_loop(&program, &phases);
        assert_eq!(result, 139629729)
    }    

    //#[test]
    fn test_max_thrust_loop_3(){
        let program = vec![3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10];
        let phases = vec![9,7,8,5,6];
        let result = run_amplifiers_loop(&program, &phases);
        assert_eq!(result, 18216)
    }  
}

// Task 1: 273814
fn main() {
    let row = file_to_vec("data.txt".to_string());
    match row {
    Ok(numbers) => {
        let answer1 = get_max_thruster(&numbers);
        println!("Task 1: {:?}", answer1);
        //let answer2 = get_max_thruster_loop(&numbers);
        //sprintln!("Task 2: {:?}", answer2);
    },
    Err(e) => println!("Error reading file: {:?}", e)
    }   
}