use std::fmt::Display;
use std::usize;

use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::polynome::Polynome;
use crate::polynomes;
use crate::Polynomes;
use crate::N;

#[derive(Debug, Clone)]
pub struct Langford {
    polynomess: Vec<Polynomes>,
}
impl Langford {
    pub fn new(polynomess: Vec<Polynomes>) -> Self {
        Self { polynomess }
    }
    pub fn get_diviseur_index(&self, divisor: usize) -> Vec<usize> {
        let mut res = vec![];
        let mut cp = divisor;
        for i in (2..=(self.polynomess.len())).rev() {
            if cp % i == 0 {
                cp /= i;
                res.push(i - 1);
            }
        }
        if cp == 1 {
            println!("Final divisor is {}", divisor);
            return res;
        }
        self.get_diviseur_index(divisor + 1)
    }
    pub fn len(&self) -> usize {
        return self.polynomess.len();
    }
    pub fn filter(&mut self, p: &Polynome) {
        let repr = p.get_repr();
        self.polynomess.iter_mut().for_each(|ps| {
            ps.inner.retain(|p1| p1.get_repr() & repr == 0);
        });
    }
    pub fn fold(mut self) -> Polynomes {
        let init = self.polynomess.remove(0);
        self.polynomess.into_iter().fold(init, |f, p1s| {
            //println!("fold!");
            &f * &p1s
        })
    }
    pub fn solve(mut self, depth: i32) -> i32 {
        let mut ps = self.polynomess.pop().unwrap();
        for _ in 0..depth {
            ps = &ps * &self.polynomess.pop().unwrap();
        }

        println!("Need {} iters", ps.len());
        ps.into_iter()
            .map(|p| {
                let mut cpy = self.clone();
                cpy.filter(&p);
                println!("iter");
                cpy.fold().get_first_count()
            })
            .sum()
    }
}
impl Display for Langford {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::from("[\n");
        for ps in &self.polynomess {
            buffer.push_str(&format!("    {}\n", ps));
        }
        buffer.push(']');
        write!(f, "{}", buffer)
    }
}
