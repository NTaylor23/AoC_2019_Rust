pub fn day01a(input: &Vec<i32>) -> i32 {
    let fuel_required: i32 = input
        .iter()
        .map(|n: &i32| (n / 3) - 2)
        .sum();

    fuel_required
}

fn reduce_to_zero(n: i32, acc: i32) -> i32 {
    if n <= 0 {
        return acc;
    }
    let reduced: i32 = std::cmp::max(n / 3 - 2, 0);
    reduce_to_zero(reduced, acc + reduced)
}

pub fn day01b(input: &Vec<i32>) -> i32 {
    let fuel_required: i32 = input
        .iter()
        .map(|n: &i32| reduce_to_zero(*n, 0))
        .sum();

    fuel_required
}