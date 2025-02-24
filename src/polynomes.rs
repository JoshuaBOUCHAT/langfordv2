use std::{
    collections::HashMap,
    fmt::{Display, Write},
    ops::Mul,
};

use crate::polynome::Polynome;

///Une structure pour représenter un essemble de Polynome qui vont ensuite
/// être mutliplier
///
#[derive(Debug, Clone)]
pub struct Polynomes {
    pub inner: Vec<Polynome>,
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
}
impl Mul<&Polynomes> for &Polynomes {
    type Output = Polynomes;
    fn mul(self, rhs: &Polynomes) -> Self::Output {
        let mut map: HashMap<u64, Polynome> = HashMap::with_capacity(1000);

        for p1 in self.inner.iter() {
            for p2 in rhs.inner.iter() {
                if let Some(temp) = p1 * p2 {
                    if let Some(p) = map.get_mut(&temp.get_repr()) {
                        p.add_count(temp.get_count());
                    } else {
                        map.insert(temp.get_repr(), temp);
                    }
                }
            }
        }
        Polynomes::new(map.into_values().collect())
    }
}
impl IntoIterator for Polynomes {
    type Item = Polynome;
    type IntoIter = std::vec::IntoIter<Polynome>;
    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}
impl Display for Polynomes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::from("[");
        for p in self.inner.iter() {
            buffer.push_str(&format!("{}, ", p));
        }
        if self.inner.len() != 0 {
            let _ = buffer.pop();
            let _ = buffer.pop();
        }
        buffer.push(']');

        write!(f, "{}", &buffer)
    }
}
