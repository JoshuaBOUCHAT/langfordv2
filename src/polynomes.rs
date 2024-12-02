use std::{collections::HashMap, ops::Mul};

use crate::{polynome::Polynome, FINAL};
use rayon::vec::IntoIter;

///Une structure pour représenter un essemble de Polynome qui vont ensuite
/// être mutliplier
///
#[derive(Debug, Clone)]
pub struct Polynomes {
    inner: Vec<Polynome>,
}
impl Polynomes {
    pub fn new(inner: Vec<Polynome>) -> Self {
        Self { inner }
    }
    pub fn get_first_count(&self) -> i32 {
        self.inner[0].get_count()
    }
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    pub fn add_polynome(&mut self, p: Polynome) {
        self.inner.push(p);
    }

    pub fn compact(&mut self) {
        let len = self.len();
        if len == 0 {
            return;
        }
        radsort::sort_by_key(&mut self.inner, |p| p.get_repr());

        let mut current_idx = 0;
        let mut current_repr = self.inner[0].get_repr();
        for i in 1..len {
            let (repr, count) = self.inner[i].get_parts();
            if repr == current_repr {
                self.inner[current_idx].add_count(count);
                continue;
            }
            current_idx += 1;
            current_repr = repr;
            self.inner[current_idx] = self.inner[i].clone();
        }
        self.inner.truncate(current_idx + 1);
        println!(
            "compacting ratio: {}",
            (len as f64 / self.len() as f64 - 1.0).ln_1p()
        )
    }
}
impl Mul<&Polynomes> for &Polynomes {
    type Output = Polynomes;
    fn mul(self, rhs: &Polynomes) -> Self::Output {
        let mut size_to_compact = 10000000;
        let mut vec = Polynomes::new(Vec::with_capacity(1000));

        for p1 in self.inner.iter() {
            for p2 in rhs.inner.iter() {
                if let Some(temp) = p1 * p2 {
                    vec.add_polynome(temp);
                }
            }
            if vec.len() > size_to_compact {
                vec.compact();
                size_to_compact = (vec.len() as f64 * 2.0) as usize;
            }
        }
        vec.compact();
        vec
    }
}
impl IntoIterator for Polynomes {
    type Item = Polynome;
    type IntoIter = std::vec::IntoIter<Polynome>;
    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}
