use std::{fmt::Display, ops::Mul};

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
const ALPHA: [char; 62] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b',
    'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u',
    'v', 'w', 'x', 'y', 'z',
];
impl Display for Polynome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut repr = self.get_repr();
        let mut buffer = format!("{}_", self.get_count());
        let mut i = 0;
        while repr != 0 {
            if repr & 1 != 0 {
                buffer.push_str(&format!("{}x", ALPHA[i]));
            }
            repr >>= 1;
            i += 1;
        }
        write!(f, "{}", &buffer)
    }
}
