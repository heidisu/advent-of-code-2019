use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::fs;
use petgraph::Graph;
use petgraph::prelude::*;
use petgraph::algo::astar;
use std::collections::HashMap;


fn to_tuples<'a>(lines: &'a Vec<String>) -> Vec<(&'a str, &'a str)> {
    let mut tuples: Vec<(&str, &str)> = Vec::new();
    for line in lines.into_iter(){
        let splits: Vec<&str> = line.split(")").collect();
        tuples.push((splits[1], splits[0]));
    }
    return tuples;
}

fn file_to_lines(filename: &str) -> io::Result<Vec<String>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    let lines : Vec<String> = file_reader.lines().filter_map(io::Result::ok).collect();
    return Ok(lines);
}

fn get_path_count(tuples: &Vec<(&str, &str)>) -> i32{
    let mut paths = Graph::<&str, &i32>::new();
    let mut nodes = HashMap::new();
    let mut counter = 0;
    for (x, y) in tuples.iter(){
        let x_node_ref = nodes.entry(x).or_insert_with(|| paths.add_node(x));
        let x_node = *x_node_ref;
        let y_node = nodes.entry(y).or_insert_with(|| paths.add_node(y));
        paths.add_edge(x_node, *y_node, &1);
    }
    
    let com = nodes[&"COM"];
    for node in nodes.values(){
        match  astar(&paths, *node, |finish| finish == com, |_| 1, |_| 1){
            Some((ct, _)) => {
                counter += ct;
            },
            None => (),
        }
    }

    return counter;
}

fn you_to_santa(tuples: Vec<(&str, &str)>) -> i32{
    let mut paths = Graph::<&str, &i32, Undirected>::new_undirected();
    let mut nodes = HashMap::new();
    let mut you_to_santa = 0;

    for (x, y) in tuples.iter(){
        let x_node_ref = nodes.entry(x).or_insert_with(|| paths.add_node(x));
        let x_node = *x_node_ref;
        let y_node = nodes.entry(y).or_insert_with(|| paths.add_node(y));
        paths.add_edge(x_node, *y_node, &1);
    }
    
    let you = nodes[&"YOU"];
    let san = nodes[&"SAN"];
    match  astar(&paths, you, |finish| finish == san, |_| 1, |_| 1){
            Some((ct, _)) => {
                you_to_santa += ct;
            },
            None => (),
        }

    return you_to_santa - 2;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_path_count() {
        let tuples = vec![("C", "B"), ("G", "B"), ("B", "COM")];
        let count = get_path_count(&tuples);
        assert_eq!(count, 5);
    }

     #[test]
    fn test_get_path_count_2() {
        let tuples = 
            vec![("B", "COM"), ("C", "B"), ("D", "C"), 
            ("D", "C"), ("E", "D"), ("F", "E"), ("G", "B")
            ,("H", "G"), ("I", "D"), ("J", "E"), ("K", "J"), ("L", "K")];
        let count = get_path_count(&tuples);
        assert_eq!(count, 42);
    }

    #[test]
    fn test_you_to_santa() {
        let tuples = 
            vec![("B", "COM"), ("C", "B"), ("D", "C"), 
            ("D", "C"), ("E", "D"), ("F", "E"), ("G", "B")
            ,("H", "G"), ("I", "D"), ("J", "E"), ("K", "J"), ("L", "K"), ("YOU", "K"), ("SAN", "I")];
        let count = you_to_santa(tuples);
        assert_eq!(count, 4);
    }
}


// Task 1: 144909
// Task 2: 259
fn main() {
    match file_to_lines("data.txt") {
        Ok(lines) => {
            let tuples = to_tuples(&lines);
            let task1 = get_path_count(&tuples);
            println!("Task 1: {:?}", task1);
            let task2 = you_to_santa(tuples);
            println!("Task 2: {:?}", task2);
        },
        Err(_) => println!("Error reading file")
    }
}
