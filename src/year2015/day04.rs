use md5::{
    Digest,
    Md5,
};

type Input = String;

pub fn parse(input: &str) -> Input {
    input.into()
}

fn find_leading_zeros(input: &str, n: usize) -> usize {
    let bits = 4 * n;
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

pub fn part1(input: &Input) -> usize {
    find_leading_zeros(input, 5)
}

pub fn part2(input: &Input) -> usize {
    find_leading_zeros(input, 6)
}
