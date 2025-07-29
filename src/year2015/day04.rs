use md5::{
    Digest,
    Md5,
};

type Input = String;

pub fn parse(input: &str) -> Input {
    input.into()
}

pub fn part1(input: &Input) -> usize {
    let bits = 4 * 5;
    let mask = ((1u128 << bits) - 1) << (128 - bits);
    let mut hasher = Md5::new();
    for i in 0.. {
        hasher.update(format!("{input}{i}"));
        let hash: [u8; 16] = hasher.finalize_reset().into();
        let value = u128::from_be_bytes(hash);
        if value & mask == 0 {
            return i;
        }
    }
    unreachable!()
}

pub fn part2(input: &Input) -> usize {
    input.len()
}
