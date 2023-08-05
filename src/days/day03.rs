use std::collections::HashSet;

fn traverse(
    movements: &Vec<String>, 
    pt: &mut (i32, i32), 
    visited: &mut HashSet<(i32, i32)>,
    f: fn(&mut HashSet<(i32, i32)>, (i32, i32), &mut i32),
    intersect: &mut i32
) {
    for movement in movements {
        let direction = movement.chars().nth(0).unwrap();
        let value: i32 = movement[1..].parse::<i32>().unwrap();
        let mut _pt: (i32, i32) = (0, 0);

        match direction {
            'L' => _pt.0 = -1, 'R' => _pt.0 = 1, 
            'U' => _pt.1 = -1, _ => _pt.1 = 1
        }

        for _ in 0..value {
            let new = (pt.0 + _pt.0, pt.1 + _pt.1);
            f(visited, new, intersect);
            *pt = new;
        }
    }
}

fn add_point_to_hash_set(visited: &mut HashSet<(i32, i32)>, new: (i32, i32), intersect: &mut i32) {
    visited.insert(new);
}

fn compare_points_in_hashset(visited: &mut HashSet<(i32, i32)>, new: (i32, i32), intersect: &mut i32) {
    if visited.contains(&new) {
        *intersect = std::cmp::min(*intersect, 
            i32::abs(new.0) + i32::abs(new.1));
    }
}

pub fn day03a(input: &Vec<Vec<String>>) -> i32 {
    let lines = input.clone();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut pt: (i32, i32) = (0, 0);
    let mut intersect: i32 = i32::MAX;

    traverse(&lines[0], &mut pt, &mut visited, add_point_to_hash_set, &mut intersect);
    pt = (0, 0);
    traverse(&lines[1], &mut pt, &mut visited, compare_points_in_hashset, &mut intersect);

    println!("{}", intersect);

    0
}