fn is_valid(n: i32, strict_pair: bool) -> bool {
    let mut counts: Vec<i32> = vec![0; 10];
    let mut tmp = n;
    let mut repeat_found = false;
    let mut prev = i32::MAX;

    while tmp > 0 {
        let digit = tmp % 10;
        counts[digit as usize] += 1;
        
        if digit == prev {
            repeat_found = true;
        } else if digit > prev {
            return false;
        }

        prev = digit;
        tmp /= 10;
    }

    if strict_pair {
        return counts.contains(&2);
    }
    repeat_found
}

fn get_range(input: &str) -> Vec<i32> {
    input
        .split("-")
        .map(|n| n.parse::<i32>().unwrap())
        .collect()
}

pub fn day04a(input: &str) -> i32 {
    let range = get_range(input);
    let mut res = 0;
    for num in range[0]..=range[1] {
        if is_valid(num, false) {
            res += 1;
        }
    }

    res
}

pub fn day04b(input: &str) -> i32 {
    let range = get_range(input);
    let mut res = 0;
    for num in range[0]..=range[1] {
        if is_valid(num, true) {
            res += 1;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        assert_eq!(is_valid(111111, false), true);
        assert_eq!(is_valid(223450, false), false);
        assert_eq!(is_valid(123789, false), false);
        
        assert_eq!(is_valid(112233, true), true);
        assert_eq!(is_valid(123444, true), false);
        assert_eq!(is_valid(111122, true), true);
    }
}