use itertools::Itertools;

fn is_valid_password(number: &u32) -> bool{
    let digits: Vec<_> = number.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
    let mut found_pair = false;
    for (i, item) in digits.iter().enumerate(){
        if i > 0 && item < &digits[i-1]{
            return false;
        }
        if i > 0 && item == &digits[i-1]{
            found_pair = true;
        }
    }
    return found_pair;
}

fn is_valid_password_strict(number: &u32) -> bool{
    let digits: Vec<_> = number.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
    let mut found_pair = false;
    for (i, item) in digits.iter().enumerate(){
        if i > 0 && item < &digits[i-1]{
            return false;
        }
    }
    return  digits.iter().group_by(|x| *x).into_iter().map(|(grp, vals)| vals.count() as u32).any(|x| x == 2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(is_valid_password(&111111), true);
    }
    
    #[test]
    fn test_2() {
        assert_eq!(is_valid_password(&223450), false);
    }

    #[test]
    fn test_3() {
        assert_eq!(is_valid_password(&123789), false);
    }
    
    #[test]
    fn test_4() {
        assert_eq!(is_valid_password_strict(&112233), true);
    }
    
    #[test]
    fn test_5() {
        assert_eq!(is_valid_password_strict(&123444), false);
    }

    #[test]
    fn test_6() {
        assert_eq!(is_valid_password_strict(&111122), true);
    }

    #[test]
    fn test_7() {
        assert_eq!(is_valid_password_strict(&111234), false);
    }

    #[test]
    fn test_8() {
        assert_eq!(is_valid_password_strict(&1233345), false);
    }
}

fn main() {
    let task1 = (357253..892942).filter(|x| is_valid_password(&x)).count();
    println!("Task 1: {:?}", task1);
    let task2 = (357253..892942).filter(|x| is_valid_password_strict(&x)).count();
    println!("Task 2: {:?}", task2);
}
