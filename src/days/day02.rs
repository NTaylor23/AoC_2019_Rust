use crate::utils;

pub fn day02a(input: &Vec<i32>) -> i32 {
    return utils::intcode_compiler(input, 12, 2);
}

pub fn day02b(input: &Vec<i32>) -> i32 {
    let target: i32 = 19690720;
    let mut ans: i32 = 0;
    for noun in 0..=99 {
        for verb in 0..=99 {
            // Can be optimized, use binary search?
            let res = utils::intcode_compiler(input, noun, verb);
            if res == target {
                ans = 100 * noun + verb;
            }
        }
    }
    ans
}