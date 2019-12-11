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

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up, 
    Down,
    Left, 
    Right
}

#[derive(Debug)]
struct Instruction {
    code: i64,
    modes: HashMap<i64, Mode>
}

#[derive(Debug)]
struct ProgramState {
    relative_base: i64,
    index: i64
}

fn int_to_mode(mode: &i64)-> Mode{
    return match mode {
        0 => Mode::Position,
        1 => Mode::Intermediate,
        2 => Mode::Relative,
        _ => panic!("Unknown mode {:?}")
    };
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


fn file_to_vec(filename: &str) -> io::Result<Vec<i64>> {
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

#[derive(Debug)]
struct Program<'a> {
    program: &'a mut HashMap<i64, i64>,
    index: i64
}

fn step_program(program: &mut HashMap<i64, i64>, program_state: &ProgramState, input: i64) -> (ProgramState, Vec<i64>){
    let mut index = program_state.index;
    let mut relative_base = program_state.relative_base;
    let mut done = false;
    let mut input_used = false;
    let mut output = Vec::new();
    
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
                if !input_used{
                let idx = get_index(index + 1, mode1, &relative_base, program);
                program.insert(idx, input);
                index += 2;
                input_used = true;
                } else {
                    return (ProgramState{index : index, relative_base: relative_base}, output);
                }
            },
            4 => {
                let mode1 = &instruction.modes[&1];
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

    return (ProgramState {index: index, relative_base: relative_base}, output);
} 

// 0 = left , 1 = right
fn new_direction(direction: &Direction, instruction: i64)->Direction{
    return match (*direction, instruction) {
        (Direction::Up, 0) => Direction::Left,
        (Direction::Up, 1) => Direction::Right,
        (Direction::Down, 0) => Direction::Right,
        (Direction::Down, 1) => Direction::Left,
        (Direction::Left, 0) => Direction::Down,
        (Direction::Left, 1) => Direction::Up,
        (Direction::Right, 0) => Direction::Up, 
        (Direction::Right, 1) => Direction::Down,
        _ => panic!("invalid direction values")
    };
}

fn new_point((x, y): (i64, i64), direction: &Direction)->(i64, i64){
    return match *direction {
        Direction::Left => (x - 1, y),
        Direction::Right => (x + 1, y),
        Direction::Up => (x, y + 1),
        Direction::Down => (x, y -1)
    }
}

fn start_painting_robot(program: &Vec<i64>, start_paint: i64)-> HashMap<(i64, i64), i64> {
    let mut map = HashMap::new();
    for (i, elem) in program.iter().enumerate(){
        map.insert(i as i64, *elem);
    }

    let mut point = (0, 0);
    let mut direction = Direction::Up;
    let mut program_state = ProgramState { index: 0, relative_base: 0 };
    let mut paint = start_paint;
    let mut painting: HashMap<(i64, i64), i64>= HashMap::new();

    loop {
        let (new_program_state, output) = step_program(&mut map, &program_state, paint);
        if output.len() == 0{
            break;
        }
        let new_paint = output[0];
        let new_dir = output[1];
        program_state = new_program_state;
        painting.insert(point, new_paint);
        direction = new_direction(&direction, new_dir);
        point = new_point(point, &direction);
        paint = *painting.get(&point).unwrap_or(&0);
    }
    return painting;
}

fn print_painting(painting: HashMap<(i64, i64), i64>){
    let x_min = painting.keys().map(|(x, _)| x).min().unwrap();
    let x_max = painting.keys().map(|(x, _)| x).max().unwrap();
    let y_min = painting.keys().map(|(_, y)| y).min().unwrap();
    let y_max = painting.keys().map(|(_, y)| y).max().unwrap();
    for j in *y_min .. (y_max + 1) {
        for i in *x_min .. (x_max + 1) {
            let pixel = painting.get(&(i, j)).unwrap_or(&0);
            if *pixel == 1 {
                print!("X");
            }
            else {
                print!(" ");
            }
        }
        println!("");
    }
}

// Task 1: 2418
// Task 2: GREJALPR
fn main() {
    let row = file_to_vec("data.txt");
    match row {
    Ok(numbers) => {
        let painting = start_painting_robot(&numbers, 0);
        println!("Task 1: {:?}", painting.len());
        let painting_2 = start_painting_robot(&numbers, 1);
        println!("Task 2:");
        print_painting(painting_2);
    },
    Err(e) => println!("Error reading file: {:?}", e)
    }   
}