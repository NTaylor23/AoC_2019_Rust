use ndarray::{s, Array3, ArrayView3};
use std::cmp::min;

static WIDTH: usize = 25;
static HEIGHT: usize = 6;

fn count_instances(layer: ArrayView3<u32>, target: u32) -> u32 {
    layer.iter().filter(|&&n| n == target).count() as u32
}

pub fn day08a(nums: &Vec<u32>) -> i32 {
    let num_layers = nums.len() / (WIDTH * HEIGHT);

    let mut target_idx = 0;
    let mut min_zero_count = u32::MAX;

    let image: Array3<u32> =
        Array3::from_shape_vec((num_layers, HEIGHT, WIDTH), nums.to_vec()).unwrap();

    for i in (0..num_layers) {
        let zeros = count_instances(
            image
                .slice(s![i..i + 1, .., ..])
                .view(), 0
        );

        if zeros < min_zero_count {
            target_idx = i;
            min_zero_count = zeros;
        }
    }

    let dim = image.slice(s![target_idx..target_idx + 1, .., ..]);
    (count_instances(dim.view(), 1) * count_instances(dim.view(), 2)) as i32
}

fn render(layers: ArrayView3<u32>) -> String {
    let mut image = String::with_capacity(WIDTH * HEIGHT + HEIGHT);

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let z_axis = layers.slice(s![.., i, j]);
            let pixel = z_axis.iter().find(|&&n| n < 2);

            match pixel {
                Some(0) => image.push_str("⬛"),
                Some(1) => image.push_str("⬜"),
                _ => println!("Error"),
            }
        }
        image.push_str("\n");
    }

    image
}

pub fn day08b(nums: &Vec<u32>) -> String {
    let num_layers = nums.len() / (WIDTH * HEIGHT);

    let image: Array3<u32> =
        Array3::from_shape_vec((num_layers, HEIGHT, WIDTH), nums.to_vec()).unwrap();

    render(image.view())
}
