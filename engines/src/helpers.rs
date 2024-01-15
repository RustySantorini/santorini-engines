pub(crate) mod turn {
    pub const W: u8 = 0;
    pub const U: u8 = 1;
}

pub(crate) mod squares {
    pub const A1: usize = 0;
    pub const A2: usize = 1;
    pub const A3: usize = 2;
    pub const A4: usize = 3;
    pub const A5: usize = 4;

    pub const B1: usize = 5;
    pub const B2: usize = 6;
    pub const B3: usize = 7;
    pub const B4: usize = 8;
    pub const B5: usize = 9;

    pub const C1: usize = 10;
    pub const C2: usize = 11;
    pub const C3: usize = 12;
    pub const C4: usize = 13;
    pub const C5: usize = 14;

    pub const D1: usize = 15;
    pub const D2: usize = 16;
    pub const D3: usize = 17;
    pub const D4: usize = 18;
    pub const D5: usize = 19;

    pub const E1: usize = 20;
    pub const E2: usize = 21;
    pub const E3: usize = 22;
    pub const E4: usize = 23;
    pub const E5: usize = 24;

}

pub(crate) mod workers {
    pub const W1: usize = 0;
    pub const W2: usize = 1;
    pub const U1: usize = 2;
    pub const U2: usize = 3;
}

pub fn hash_workers(workers:[usize;4]) -> usize{
    let mut sum:usize = 0;
    let base:usize = 25;
    for i in 0..4{
        sum += base.pow(i) * workers[i as usize];
    }
    sum
}

pub fn unhash_workers(mut hash: usize) -> [usize; 4] {
    let mut w: [usize; 4] = [0; 4]; // Initialize the array with zeros
    let base: usize = 25;
    for i in 0..4 {
        w[i] = hash % base;
        hash /= base;
    }
    w
}


#[cfg(test)]
mod tests {
    use crate::helpers::squares::*;

    use super::*;
    
    #[test]
    fn hashing(){
        let w1 = [A1, A2, A3, A4];
        let w2 = [C3, C4, B3, D3];
        let w3 = [A1, A5, E1, E5];
        for i in [w1, w2, w3]{
            assert_eq!(i, unhash_workers(hash_workers(i)));
        }
    }

}