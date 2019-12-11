use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::fs;
use std::collections::HashMap;

fn file_to_points(filename: &str) -> io::Result<Vec<(i32, i32)>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    let rows: Vec<String> = file_reader.lines().filter_map(io::Result::ok).collect();
    let mut points: Vec<(i32, i32)> = Vec::new();
    for (i, row) in rows.iter().enumerate(){
        for (j, c) in row.chars().enumerate(){
            if c == '#'{
                points.push((i as i32, j as i32));
            }
        }
    }
    return Ok(points);
}

#[derive(Debug, Hash, Copy, Clone)]
struct AsteroideData {
    slope: (i32, i32),
    point: (i32, i32),
    center: (i32, i32),
    distance: u32,
}

impl PartialEq for AsteroideData {
    fn eq(&self, other: &AsteroideData) -> bool {
        return self.point == other.point && self.center == other.center;
    }
}

impl Eq for AsteroideData {}

fn get_asteroide_data(center: &(i32, i32), point: &(i32, i32))-> AsteroideData{
    let (x, y) = center;
    let (z, w) = point;
    let sl = (z- x, w - y);
    let dist = (z - x).abs() + (w - y).abs();
    return AsteroideData{ point: *point, center: *center, slope: sl, distance: dist as u32 };
}

fn same_slope_and_direction(a: &AsteroideData, b: &AsteroideData)->bool{
    let (x, y) = a.slope;
    let (z, w) = b.slope;
    let (p, q) = a.center;
    let (x1, y1) = a.point;
    let (x2, y2) = b.point;
    let center_between_points = (x1 < p && p < x2) || (y1 < q && q < y2);
    return x * w == y * z && !center_between_points;
}

fn group_asteroides(asteroides: &Vec<AsteroideData>)-> HashMap<AsteroideData, Vec<AsteroideData>>{
    let mut map: HashMap<AsteroideData, Vec<AsteroideData>> = HashMap::new();

    for asteroide in asteroides.iter(){
        let mut inserted = false;
        for (key, values) in map.iter_mut(){
            if same_slope_and_direction(key, asteroide){
                values.push(*asteroide);
                inserted = true;
            }
        }
        if !inserted {
            map.insert(*asteroide, vec![*asteroide]);
        }
    }
    return map;
}

fn find_closest(point: &(i32, i32), points: &Vec<(i32, i32)>)->i32{
    let data : Vec<_> = 
            points.iter().filter (|x| *x != point).map (|p| get_asteroide_data(point, p)).collect();   
    let groups = group_asteroides(&data);
    return groups.len() as i32;
}

fn max_asteroides(points: &Vec<(i32, i32)>)->((i32, i32), i32){
    return points.iter().map(|p| (*p, find_closest(p, points))).max_by(| (_, y), (_, w)| y.cmp(w)).unwrap();
}

fn atan2((x, y): (i32, i32))->f32{
    return (x as f32).atan2(y as f32);
}

// does not work
fn vaporize(max_point: &(i32, i32), points: &Vec<(i32, i32)>){
    let data : Vec<_> = 
            points.iter().filter (|x| *x != max_point).map (|p| get_asteroide_data(max_point, p)).collect();   
    let groups = group_asteroides(&data);
    let mut copy_groups = groups.clone();
    let mut sorted_keys = groups.keys().collect::<Vec<_>>();
    sorted_keys.sort_by(|x, y| atan2(x.slope).partial_cmp(&atan2(y.slope)).unwrap());
    let mut index = sorted_keys.iter().position(|x| { let (a, _) = x.slope; return a == 0}).unwrap();
    let mut counter = 1;
    println!("index {:?}", index);
    while counter <= 200 && index < sorted_keys.len() {
        let key = sorted_keys[index];
        let values = copy_groups.get_mut(&key).unwrap();
        values.sort_by(|x, y| x.distance.cmp(&y.distance));
        match values.pop(){
            Some(asteroide) => {
                println!("Counter: {:?}, Value: {:?}", counter, asteroide);
                counter += 1;
            },
            None => ()
        }
        if index == sorted_keys.len() - 1 {
            index = 0;
        }
        else {
            index += 1;
        }
    }
    println!("sorted: {:?}", index);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_closest(){
        let points = vec![(1, 0), (4, 0), (0, 2), (1, 2), (2,2), (3, 2), (4,2), (4, 3), (3, 4), (4, 4)];
        let closest = find_closest(&(3, 4), &points);
        assert_eq!(closest, 8);
    }

    #[test]
    fn test_find_closest_2(){
        let points = vec![(1, 0), (4, 0), (0, 2), (1, 2), (2,2), (3, 2), (4,2), (4, 3), (3, 4), (4, 4)];
        let closest = find_closest(&(1, 0), &points);
        assert_eq!(closest, 7);
    }

    #[test]
    fn test_find_closest_3(){
        let points = vec![(1, 0), (4, 0), (0, 2), (1, 2), (2,2), (3, 2), (4,2), (4, 3), (3, 4), (4, 4)];
        let closest = find_closest(&(4, 2), &points);
        assert_eq!(closest, 5);
    }

     #[test]
    fn test_example_1(){
        let points = file_to_points("test_1.txt").unwrap();
        let (_, max) = max_asteroides(&points);
        assert_eq!(max, 33);
    }

    #[test]
    fn test_example_2(){
        let points = file_to_points("test_2.txt").unwrap();
        let (_, max) = max_asteroides(&points);
        assert_eq!(max, 35);
    }

    #[test]
    fn test_example_3(){
        let points = file_to_points("test_3.txt").unwrap();
        let (_, max) = max_asteroides(&points);
        assert_eq!(max, 41);
    }


    #[test]
    fn test_example_4(){
        let points = file_to_points("test_4.txt").unwrap();
        let (_, max) = max_asteroides(&points);
        assert_eq!(max, 210);
    }

    //#[test]
    fn test_vaporize(){
        let points = file_to_points("test_4.txt").unwrap();
        let (pnt, _) = max_asteroides(&points);
        vaporize(&pnt, &points);
    }


    //#[test]
    fn test_vaporize_2(){
        let points = file_to_points("test_3.txt").unwrap();
        let (pnt, _) = max_asteroides(&points);
        vaporize(&pnt, &points);
    }
}


// Task 1: 282
// Task 2: ???
fn main() {
     match file_to_points("data.txt") {
        Ok(points) => {
            let (pnt, task1) = max_asteroides(&points);
            println!("Task 1: {:?}", task1);
          //  let image = decode_image(&layers);
            let task2 = vaporize(&pnt, &points);
           println!("Task 2: {:?}", task2);
           // print_image(&image, rows, cols)
        },
        Err(_) => println!("Error reading file")
    }
}
