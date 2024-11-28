use std::ops::Mul;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Polynome {
    repr: u64,
    count: i32,
}
impl Polynome {
    pub fn new(pos1: usize, pos2: usize) -> Self {
        Self {
            repr: (1u64 << (pos1)) | (1u64 << (pos2)),
            count: 1,
        }
    }
    pub fn add_count(&mut self, other: &Self) {
        self.count += other.count;
    }
    pub fn get_repr(&self) -> u64 {
        self.repr
    }
}
impl Mul<&Polynome> for &Polynome {
    type Output = Option<Polynome>;
    fn mul(self, other: &Polynome) -> Self::Output {
        if (self.repr & other.repr) != 0 {
            return None;
        }
        Some(Polynome {
            repr: self.repr | other.repr,
            count: self.count * other.count,
        })
    }
}
