use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::fs;
use std::collections::HashSet;
use std::iter::FromIterator;

fn file_to_vec(filename: String) -> io::Result<(Vec<String>, Vec<String>)> {
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    let rows : Vec<String> = file_reader.lines().filter_map(io::Result::ok).collect();
    let v1: Vec<String> = rows[0].split(",").map(|x| x.parse::<String>().expect("parse error")).collect();
    let v2: Vec<String> = rows[1].split(",").map(|x| x.parse::<String>().expect("parse error")).collect();


    return Ok((v1, v2));
}

#[derive(Debug)]
struct Move {
    direction: (i32, i32),
    step: i32
}

fn to_move(code: String) -> Move {
    let letter = &code[0..1];
    let step: i32 = (&code[1..]).parse().unwrap();
    let direction = match letter {
        "R" => Ok((1, 0)),
        "L" => Ok((-1, 0)),
        "U" => Ok((0, 1)),
        "D" => Ok((0, -1)),
        _ => Err("Illegal direction")
    };
    return Move { direction: direction.unwrap(), step: step };
}

fn expand_move(mv: &Move, start: (i32, i32)) -> Vec<(i32, i32)> {
    let mut expanded = Vec::new();
    for i in 1 .. mv.step + 1 {
        let (x, y) = start;
        let (d1, d2) = mv.direction;
        expanded.push((x + i * d1, y + i * d2));
    }
    return expanded;
}

fn expand_vector(moves: Vec<Move>) -> Vec<(i32, i32)> {
    let mut start = (0, 0);
    let mut expanded = Vec::new();
    for mv in moves.iter() {
        let mv_expanded = expand_move(&mv, start);
        start = mv_expanded.last().unwrap().clone();
        expanded.extend(&mv_expanded);
    }
    return expanded;
}

fn count_moves(moves: &Vec<(i32, i32)>, inters: &(i32, i32)) -> Result<i32, String> {
    let mut count = 0;
    let (x, y) = *inters;
    for (p1, p2) in moves.iter(){
        count += 1;
        if x == *p1 && y == *p2{
            return Ok(count);
        }
    }
    return Err("wrong use of function".to_string());
} 

fn manhatten_distance((x, y): &(i32, i32)) -> i32{
    return x.abs() + y.abs();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_move() {
        let result = to_move("R75".to_string());
        assert_eq!(result.direction, (1, 0));
        assert_eq!(result.step, 75)
    }
    
    #[test]
    fn test_expands_move() {
        let result = expand_move(&Move {direction: (0, -1), step: 4}, (2, 1));
        let expected = vec![(2, 0), (2, -1), (2, -2), (2, -3)];
        assert_eq!(result, expected);
    }
}

fn main() {
    let row = file_to_vec("data.txt".to_string());
    match row {
    Ok((v1, v2)) => {
        let moves1: Vec<Move> = v1.iter().map(|x| to_move(x.to_string())).collect();
        let moves2: Vec<Move> = v2.iter().map(|x| to_move(x.to_string())).collect();
        let expanded1 = expand_vector(moves1);
        let expanded2 = expand_vector(moves2);
        let exp_set1: HashSet<(i32, i32)> = HashSet::from_iter(expanded1.clone());
        let exp_set2: HashSet<(i32, i32)> = HashSet::from_iter(expanded2.clone());
        let intersection: HashSet<_> = exp_set1.intersection(&exp_set2).collect();
      
        let task1 = intersection.iter().map(|x| manhatten_distance(x)).min().unwrap();
        println!("task 1: {:?}", task1);
        let task2 = intersection.iter().map(|x| count_moves(&expanded1, x).unwrap() + count_moves(&expanded2, x).unwrap()).min().unwrap();
        println!("task 2: {:?}", task2);
    },
    Err(e) => println!("Error reading file: {:?}", e)
    }  
}
