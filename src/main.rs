use iter::{ParallelBridge, ParallelIterator};
use langford::Langford;
use rayon::*;

use std::time::Instant;

use polynome::Polynome;
use polynomes::Polynomes;

mod langford;
mod polynome;
mod polynomes;

const N: usize = 12;
const SIZE: usize = 2 * N;
//[0, 1, 5, 4, 2, 3, 11, 8, 12, 10, 6, 7, 9, 14, 13]

fn main() {
    sequential_compute();

    //iter chacun des polynome prend le dernier et le multiplie avec l'avant dernier puis l'ajoute a la fin
    //le processus est répeter jusqu'à temps qu'il y est plus qu'un polynome le final
}

fn sequential_compute() {
    let start: Vec<_> = get_polynomess();

    let langfords = Langford::new(start);
    println!("{}", &langfords);
    let res = langfords.solve(7) / 2;
    println!("res: {res}")
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
