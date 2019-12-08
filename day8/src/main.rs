use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::fs;

fn file_to_line(filename: &str) -> io::Result<String> {
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    let lines : Vec<String> = file_reader.lines().filter_map(io::Result::ok).collect();
    return Ok(lines.into_iter().nth(0).unwrap());
}

fn str_to_layers(line: &str, rows: i32, cols: i32) -> Vec<Vec<i32>>{
    let  mut layers = Vec::new();
    let size = (rows * cols) as usize;
    let num_layers = line.len() / size;
    for i in 0 .. num_layers {
        let part = &line[i *size .. (i + 1) * size];
        layers.push(part.chars().map(|x| x.to_digit(10).unwrap() as i32).collect());
    }
    return layers;
}

fn get_checksum(layers: &Vec<Vec<i32>>)->i32{
    let count_digits = |digit| move |x: &Vec<i32>| x.iter().filter(|y| *y == digit).count() as i32;
    return 
        layers
        .iter()
        .map(|x| (x, count_digits(&0)(x)))
        .min_by(| (_, x), (_, y)| x.cmp(y))
        .map (|(x, _)| count_digits(&1)(x)*count_digits(&2)(x)).unwrap();
}

fn decode_image(layers: &Vec<Vec<i32>>)->Vec<i32>{
    let size = layers[0].len();
    let extract_values = |index| move |layers: &Vec<Vec<i32>>| layers.iter().map(|x| *x.iter().nth(index).unwrap()).collect::<Vec<i32>>();
    let mut image = Vec::new();

    for i in 0 .. size {
        let values = extract_values(i)(&layers);
        let pixel = values.into_iter().skip_while(|x| *x == 2).nth(0).unwrap(); 
        image.push(pixel);
    }

    return image;
}

fn print_image(image: &Vec<i32>, rows: i32, columns: i32){
    for j in 0 .. columns {
        for i in 0 .. rows {
            let index = (i + (j * rows)) as usize;
            let mut pixel = " ";
            if image[index] == 1 {
                pixel = "X"
            } 
            print!("{}", pixel);
        }
        println!("");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_move() {
        let result = str_to_layers(&"123456789012", 3, 2);
        assert_eq!(result, vec![vec![1, 2, 3, 4, 5, 6], vec![7, 8, 9, 0, 1, 2]]);
    }

    #[test]
    fn test_get_checksum() {
        let layers = vec![vec![1, 2, 3, 4, 5, 6], vec![7, 8, 9, 0, 1, 2]];
        let result = get_checksum(&layers);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_decode_image() {
        let layers = vec![vec![0, 2, 2, 2], vec![1, 1, 2, 2], vec![2, 2, 1, 2], vec![0, 0, 0, 0]];
        let result = decode_image(&layers);
        assert_eq!(result, vec![0, 1, 1, 0]);
    }
}


// task 1: 2760
// Task 2: AGUEB
fn main() {
    match file_to_line("data.txt") {
        Ok(line) => {
            let rows = 25;
            let cols = 6;
            let layers = str_to_layers(&line, rows, cols);
            let task1 = get_checksum(&layers);
            println!("Task 1: {:?}", task1);
            let image = decode_image(&layers);
            println!("Task 2:");
            print_image(&image, rows, cols)
        },
        Err(_) => println!("Error reading file")
    }
}