use std::{collections::HashMap, ops::Mul};

use crate::polynome::Polynome;

#[derive(Debug, Clone)]
pub struct Polynomes {
    inner: Vec<Polynome>,
}
impl Polynomes {
    pub fn new(inner: Vec<Polynome>) -> Self {
        Self { inner }
    }
}

impl Mul<&Polynomes> for &Polynomes {
    type Output = Polynomes;
    fn mul(self, rhs: &Polynomes) -> Self::Output {
        let mut map: HashMap<u64, Polynome> = HashMap::new();
        for p1 in self.inner.iter() {
            for p2 in rhs.inner.iter() {
                if let Some(temp) = p1 * p2 {
                    if let Some(t) = map.get_mut(&temp.get_repr()) {
                        t.add_count(&temp);
                    } else {
                        map.insert(temp.get_repr(), temp);
                    }
                }
            }
        }

        return Polynomes {
            inner: map.into_values().collect(),
        };
    }
}
