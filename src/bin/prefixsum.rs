
/// Parallel Prefix Sum
/// @version 1.0
/// @author Atiwat Rachatawarn

use rayon::prelude::*;

fn parallel_prefix_sum(xs: &Vec<u64>) -> Vec<u64> {
    let count_vec: Vec<u64> = (0..(xs.len() as u64) + 1).collect();
    count_vec.par_iter()
        .map(|x| xs[0..*x as usize]
            .to_vec()
            .par_iter()
            .cloned()
            .reduce(||0 , |a,b| a+b))
        .collect()
}

fn main() {
    println!("{:?}", parallel_prefix_sum(&vec![]));
    println!("{:?}", parallel_prefix_sum(&vec![1, 2, 3, 4]));
    println!("{:?}", parallel_prefix_sum(&vec![8, 9]));

    println!("{}", parallel_prefix_sum(&vec![]) == vec![0]);
    println!("{}", parallel_prefix_sum(&vec![1, 2, 3, 4]) == vec![0, 1, 3, 6, 10]);
    println!("{}", parallel_prefix_sum(&vec![8, 9]) == vec![0, 8, 17]);
}
