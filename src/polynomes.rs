use std::{collections::HashMap, ops::Mul};

use crate::polynome::Polynome;

///Une structure pour représenter un essemble de Polynome qui vont ensuite
/// être mutliplier
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
}

///Dans cette première version j'utlise une hashmap pour pouvoir réduire mon polynome a chaque étape
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
