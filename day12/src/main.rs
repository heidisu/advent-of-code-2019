use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
struct Point {
    x: i32,
    y: i32,
    z: i32
}

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
struct Moon {
    position: Point,
    velocity: Point
}

fn get_velocity_change(point: &Point, other: &Point)-> Point{
    let gravity = |x, y| if x > y { return -1 } else if x < y { return 1} else {return 0}; 
    return Point {
        x: gravity(point.x, other.x),
        y: gravity(point.y, other.y),
        z: gravity(point.z, other.z)
    }
}

fn step(moon: &Moon, others: &Vec<Moon>)->Moon{
    let gravity = 
        others
            .iter()
            .map(|m| get_velocity_change(&moon.position, &m.position))
            .fold(Point{ x: 0, y:0, z: 0}, |acc, m| Point { x: acc.x + m.x, y:acc.y + m.y, z: acc.z + m.z });
    let new_velocity = Point {x: moon.velocity.x + gravity.x, y: moon.velocity.y + gravity.y, z: moon.velocity.z + gravity.z};
    return Moon {
        position: Point {x: moon.position.x + new_velocity.x, y: moon.position.y + new_velocity.y, z: moon.position.z + new_velocity.z},
        velocity: new_velocity
    };
}

fn step_all(moons: &Vec<Moon>, steps: i32)->Vec<Moon>{
    if steps == 0 {
        return moons.to_vec();
    }
    else {
        let new_moons: Vec<Moon> = moons.iter().map(|m| step(m, moons)).collect();
        return step_all(&new_moons, steps - 1);
    }
}

fn get_energy(moon: &Moon)->i32{
    let potential = moon.position.x.abs() + moon.position.y.abs() + moon.position.z.abs();
    let kinetic = moon.velocity.x.abs() + moon.velocity.y.abs() + moon.velocity.z.abs();
    return potential * kinetic;
}

fn get_total_energy(moons: &Vec<Moon>)->i32{
    return moons.iter().map(|m| get_energy(m)).sum();
}

// way too slow
fn get_history_repeats_itself(moons: &Vec<Moon>)->i32{
    let mut counter = 0;
    let mut states: HashSet<_> = HashSet::new();
    let mut moons = moons.to_vec();
    loop {
        let new_moons = step_all(&moons, 1);
        let tuple = (new_moons[0], new_moons[1], new_moons[2], new_moons[3]);
        if !states.insert(tuple){
            return counter;
        }
        moons = new_moons;
        counter += 1;
        if counter % 100000 == 0 {
            println!("counter: {:?}", counter);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static MOON_1: Moon = Moon { position: Point{ x: -1, y: 0, z: 2 }, velocity: Point{ x: 0, y: 0, z: 0}};
    static MOON_2: Moon = Moon { position: Point{ x: 2, y: -10, z: -7 }, velocity: Point{ x: 0, y: 0, z: 0}};
    static MOON_3: Moon = Moon { position: Point{ x: 4, y: -8, z: 8 }, velocity: Point{ x: 0, y: 0, z: 0}};
    static MOON_4: Moon = Moon { position: Point{ x: 3, y: 5, z: -1 }, velocity: Point{ x: 0, y: 0, z: 0}};

    #[test]
    fn test_get_velocity_change(){
        let point = Point { x: 3, y: 6, z: 2 };
        let other = Point { x: 5, y: 3, z: 2 };
        let point = get_velocity_change(&point, &other);
        assert_eq!(point, Point { x: 1, y: -1, z: 0 });
    }

    #[test]
    fn test_step(){
        let moons = vec![MOON_1, MOON_2,MOON_3, MOON_4];
        let moon3step1 = step(&MOON_3, &moons);

        assert_eq!(moon3step1.position, Point{ x: 1, y: -7, z: 5 });
        assert_eq!(moon3step1.velocity, Point{ x: -3, y: 1, z: -3 });
    }


    #[test]
    fn test_step_all(){
        let moons = vec![MOON_1, MOON_2,MOON_3, MOON_4];
        let new_moons = step_all(&moons, 7);
        assert_eq!(new_moons[1].position, Point{ x: 1, y: -4, z: -4 });
        assert_eq!(new_moons[1].velocity, Point{ x: -2, y: -4, z: -4 });
    }

    #[test]
    fn test_get_total_energy(){
        let moons = vec![MOON_1, MOON_2,MOON_3, MOON_4];
        let new_moons = step_all(&moons, 10);
        let total_energy = get_total_energy(&new_moons);
        assert_eq!(total_energy, 179);
    }
    #[test]
    fn test_history_repeats_itself(){
        let moons = vec![MOON_1, MOON_2,MOON_3, MOON_4];
        let repeats = get_history_repeats_itself(&moons);
        assert_eq!(repeats, 2772);
    }
}

// Task 1: 6490
fn main() {
    let io = Moon { position: Point{ x: 13, y: 9, z: 5 }, velocity: Point{ x: 0, y: 0, z: 0}};
    let europa = Moon { position: Point{ x: 8, y: 14, z: -2 }, velocity: Point{ x: 0, y: 0, z: 0}};
    let ganymede = Moon { position: Point{ x: -5, y: 4, z: 11 }, velocity: Point{ x: 0, y: 0, z: 0}};
    let callisto = Moon { position: Point{ x: 2, y: -6, z: 1 }, velocity: Point{ x: 0, y: 0, z: 0}};
    let moons = vec![io, europa, ganymede, callisto];
    
    let new_moons = step_all(&moons, 1000);
    let total_energy = get_total_energy(&new_moons);
    println!("Task 1: {:?}", total_energy);
    
    let repeats = get_history_repeats_itself(&moons);
    println!("Task 2: {:?}", repeats);
}
