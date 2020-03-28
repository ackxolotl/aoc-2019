use std::cmp::Ordering;

fn main() {
    let min = 165_432;
    let max = 707_912;

    let passwords = (min..max).filter(|x| is_password_one(*x)).count();

    println!("Passwords part 1: {}", passwords);

    let passwords = (min..max).filter(|x| is_password_two(*x)).count();

    println!("Passwords part 2: {}", passwords);
}

fn is_password_one(num: usize) -> bool {
    let mut duplicate = false;

    for (first, second) in num.to_string().chars().zip(num.to_string().chars().skip(1)) {
        if first > second {
            return false;
        }

        if first == second {
            duplicate = true;
        }
    }

    duplicate
}

fn is_password_two(num: usize) -> bool {
    let mut duplicate = false;
    let mut repetitive = 0;

    for (first, second) in num.to_string().chars().zip(num.to_string().chars().skip(1)) {
        match first.cmp(&second) {
            Ordering::Greater => {
                return false;
            }
            Ordering::Equal => {
                repetitive += 1;
            }
            Ordering::Less => {
                if repetitive == 1 {
                    duplicate = true;
                }
                repetitive = 0;
            }
        }
    }

    duplicate || (repetitive == 1)
}

#[test]
fn is_password_one_test() {
    assert!(is_password_one(111));
    assert!(is_password_one(1223));

    assert!(!is_password_one(1221));
    assert!(!is_password_one(123));
}

#[test]
fn is_password_two_test() {
    assert!(is_password_two(112233));
    assert!(is_password_two(111122));
    assert!(is_password_two(112222));

    assert!(!is_password_two(123444));
    assert!(!is_password_two(123456));
}
