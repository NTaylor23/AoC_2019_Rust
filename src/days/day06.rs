use std::collections::{hash_map::DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};

fn str_to_u32(s: &str) -> u32 {
    let mut hs = DefaultHasher::new();
    s.hash(&mut hs);
    hs.finish() as u32
}

pub fn day06a(lines: &Vec<String>) -> i32 {
    let mut pairs: HashMap<u32, u32> = HashMap::new();
    let mut hosts: Vec<u32> = Vec::new();

    for line in lines {
        let mut parts = line.split(")");
        let orbiter: u32 = str_to_u32(parts.next().unwrap());
        let host: u32 = str_to_u32(parts.next().unwrap());
        hosts.push(host);
        pairs.insert(host, orbiter);
    }

    let mut counts: HashMap<u32, u32> = HashMap::new();

    for host in hosts {
        let mut tmp = host;
        counts.insert(host, 0);
        while pairs.contains_key(&tmp) {
            counts.insert(host, counts[&host] + 1);
            tmp = pairs[&tmp];
        }
    }

    let res: u32 = Iterator::sum(counts.values());
    res as i32
}

pub fn day06b(lines: &Vec<String>) -> i32 {
    let start: u32 = str_to_u32("YOU");
    let dest: u32 = str_to_u32("SAN");

    let mut pairs: HashMap<u32, u32> = HashMap::new();

    for line in lines {
        let mut parts = line.split(")");
        let orbiter: u32 = str_to_u32(parts.next().unwrap());
        let host: u32 = str_to_u32(parts.next().unwrap());
        pairs.insert(host, orbiter);
    }

    let mut tmp = start;
    let mut steps = 0;
    let exit = str_to_u32("COM");

    let mut path: HashMap<u32, u32> = HashMap::new();

    while tmp != exit {
        path.insert(tmp, steps);
        tmp = pairs[&tmp];
        steps += 1;
    }

    tmp = dest;
    steps = 0;

    while !path.contains_key(&tmp) {
        tmp = pairs[&tmp];
        steps += 1;
    }

    let ans: u32 = (steps - 1) + (path[&tmp] - 1);
    ans as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        assert_eq!(str_to_u32("K"), 4028726762);
        assert_eq!(str_to_u32("I"), 294636003);
    }
}
