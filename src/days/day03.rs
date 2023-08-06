use std::collections::HashMap;

#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn traverse_line(
    movements: &[String],
    pt: &mut Point,
    visited: &mut HashMap<Point, i32>,
    f: fn(&mut HashMap<Point, i32>, Point, &mut i32),
    context_val: &mut i32,
) {
    for movement in movements {
        let direction: char = movement.chars().nth(0).unwrap();
        let value: i32 = movement[1..].parse::<i32>().unwrap();
        let mut _pt: Point = Point { x: 0, y: 0 };

        match direction {
            'L' => _pt.x = -1,
            'R' => _pt.x = 1,
            'U' => _pt.y = -1,
            _ => _pt.y = 1,
        }

        for _ in 0..value {
            let new = Point {
                x: pt.x + _pt.x,
                y: pt.y + _pt.y,
            };
            f(visited, new, context_val);
            *pt = new;
        }
    }
}

fn add_point_to_hash_map(
    visited: &mut HashMap<Point, i32>, 
    new: Point, 
    steps: &mut i32
) {
    *steps += 1;
    visited.insert(new, *steps * -1);
}

fn compare_points_in_hash_map(
    visited: &mut HashMap<Point, i32>, 
    new: Point, 
    intersect: &mut i32
) {
    if visited.contains_key(&new) {
        *intersect = std::cmp::min(
            *intersect, 
            i32::abs(new.x) + i32::abs(new.y)
        );
    }
}

fn find_intersects(
    visited: &mut HashMap<Point, i32>, 
    new: Point, 
    steps: &mut i32
) {
    *steps += 1;
    if visited.contains_key(&new) {
        match visited.get(&new) {
            Some(_steps) => {
                visited.insert(new, *steps + (_steps * -1));
            }
            _ => panic!("Error"),
        }
    }
    
}

pub fn day03a(input: &[Vec<String>]) -> i32 {
    let mut visited: HashMap<Point, i32> = HashMap::new();
    let mut pt: Point = Point { x: 0, y: 0 };
    let mut intersect: i32 = i32::MAX;
    let mut steps: i32 = 0;

    // add all points from first set of lines
    traverse_line(
        &input[0],
        &mut pt,
        &mut visited,
        add_point_to_hash_map,
        &mut steps,
    );

    // reset origin point
    pt = Point { x: 0, y: 0 };

    // find closest intersection
    traverse_line(
        &input[1],
        &mut pt,
        &mut visited,
        compare_points_in_hash_map,
        &mut intersect,
    );
    intersect
}

pub fn day03b(input: &[Vec<String>]) -> i32 {
    let mut visited: HashMap<Point, i32> = HashMap::new();
    let mut pt: Point = Point { x: 0, y: 0 };
    let mut steps: i32 = 0;

    // add all points from first set of lines and set the steps to negative
    traverse_line(
        &input[0],
        &mut pt,
        &mut visited,
        add_point_to_hash_map,
        &mut steps,
    );

    // add the points from the second set of lines - if they intersect,
    // calculate the distance and flip value to positive
    steps = 0;
    pt = Point { x: 0, y: 0 };
    traverse_line(
        &input[1],
        &mut pt,
        &mut visited,
        find_intersects,
        &mut steps,
    );

    return *visited.values().filter(|&&n| n > 0).min().unwrap_or(&-1);
}
