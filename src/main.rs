use iter::{ParallelBridge, ParallelIterator};
use rayon::*;

use std::{time::Instant, u128};

use polynome::Polynome;
use polynomes::Polynomes;

mod polynome;
mod polynomes;

const N: usize = 15;
const SIZE: usize = 2 * N;
const FINAL: u64 = (1u64 << (SIZE)) - 1;
//[0, 1, 5, 4, 2, 3, 11, 8, 12, 10, 6, 7, 9, 14, 13]

fn main() {
    sequential_compute();
    //iter chacun des polynome prend le dernier et le multiplie avec l'avant dernier puis l'ajoute a la fin
    //le processus est répeter jusqu'à temps qu'il y est plus qu'un polynome le final
}

fn sequential_compute() {
    let start: Vec<_> = get_polynomess();
    //let vec: Vec<_> = (0..N).rev().collect();
    let vec = vec![0, 1, 5, 4, 2, 3, 11, 8, 12, 10, 6, 7, 9, 14, 13];
    let now = Instant::now();
    let res = compute_polynomess(vec, start);
    println!(
        "compute in {} milis value: {}",
        now.elapsed().as_millis(),
        res / 2
    );
}
fn threaded_compute() {
    let mut start: Vec<_> = get_polynomess();
    let vec: Vec<_> = (0..N).rev().collect();
    //let vec = vec![0, 1, 5, 4, 2, 3, 11, 8, 12, 10, 6, 7, 9, 14, 13, 15];
    let now = Instant::now();

    let possiblilities = start.pop().unwrap();
    let res: i32 = possiblilities
        .into_iter()
        //.par_bridge()
        .map(|p| {
            let mut used = start.clone();
            used.push(Polynomes::new(vec![p]));
            compute_polynomess(vec.clone(), used)
        })
        .sum();
    println!(
        "compute in {} milis value: {}",
        now.elapsed().as_millis(),
        res / 2
    );
}
//Génere (x0x2 x1x3 x2x4 x3x5) * (x0x3  x1x4 x2x5) * (x0x4 x1x5) pour N =3
fn get_polynomess() -> Vec<Polynomes> {
    let mut res = vec![];
    for i in 1..=N {
        let mut vec = vec![];
        for j in 0..(SIZE - i - 1) {
            let temp = Polynome::new(j, j + i + 1);
            vec.push(temp);
        }
        res.push(Polynomes::new(vec))
    }
    res
}
fn compute_polynomess(mut order: Vec<usize>, polynomess: Vec<Polynomes>) -> i32 {
    let mut init = polynomess[order.pop().unwrap()].clone();
    for _ in 0..(N - 1) {
        let res = &init * &polynomess[order.pop().unwrap()];
        init = res;
    }
    init.get_first_count()
}
