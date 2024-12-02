use std::ops::Mul;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]

///Représente un polynome si le bit 0 est occupé dans repr alors on a x1
/// donc 1 * x1x3 donne  count:1 101
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
    pub fn add_poly(&mut self, other: &Self) {
        self.count += other.count;
    }
    pub fn get_repr(&self) -> u64 {
        self.repr
    }
    pub fn get_count(&self) -> i32 {
        return self.count;
    }
    pub fn add_count(&mut self, count: i32) {
        self.count += count;
    }
    pub fn get_parts(&self) -> (u64, i32) {
        (self.repr, self.count)
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
