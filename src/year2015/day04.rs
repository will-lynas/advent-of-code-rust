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

    // Pre-create the hasher so we don't have to create it for each iteration
    let mut hasher = Md5::new();
    // Pre-create the buffer to avoid allocations in each iteration
    let mut buf = itoa::Buffer::new();

    for i in 0.. {
        hasher.update(input);
        hasher.update(buf.format(i));
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
