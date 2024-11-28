use iter::ParallelIterator;
use rayon::*;
use slice::ParallelSlice;
use std::time::Instant;

use polynome::Polynome;
use polynomes::Polynomes;

mod polynome;
mod polynomes;

const N: usize = 16;
const SIZE: usize = 2 * N;

fn main() {
    let now = Instant::now();
    let mut start = get_polynomess();
    for i in (1..(N)).rev() {
        start[i - 1] = &start[i - 1] * &start[i];
    }
    //une tentative de paraleliser mais imcompréhension ça rend le calcule super lent peut-être que l'ordre a y avoir
    /*while start.len() != 1 {
        println!("len: {}", start.len());
        start = start
            .par_chunks(2)
            .map(|c| {
                if c.len() == 2 {
                    return &c[0] * &c[1];
                }
                return c[0].clone();
            })
            .collect();
    }*/

    println!("{:?} compute in {}ms", start[0], now.elapsed().as_millis())
}
//(x0x2 x1x3 x2x4 x3x5) * (x0x3  x1x4 x2x5) * (x0x4 x1x5)
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
