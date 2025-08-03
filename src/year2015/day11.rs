pub fn parse(input: &str) -> [u8; 8] {
    input.as_bytes().try_into().unwrap()
}

fn consecutive(password: &[u8]) -> bool {
    password
        .windows(3)
        .any(|w| w[0] + 1 == w[1] && w[1] + 1 == w[2])
}

fn forbidden(password: &[u8]) -> bool {
    password
        .iter()
        .any(|&c| c == b'i' || c == b'o' || c == b'l')
}

fn two_pairs(password: &[u8]) -> bool {
    password.windows(2).filter(|w| w[0] == w[1]).count() >= 2
}

pub fn part1(password: &[u8; 8]) -> String {
    let mut password = password.to_vec();
    while !(consecutive(&password) && !forbidden(&password) && two_pairs(&password)) {
        // increment
        let mut i = 7;
        loop {
            password[i] += 1;
            if password[i] > b'z' {
                password[i] = b'a';
                i -= 1;
            } else {
                break;
            }
        }
    }

    String::from_utf8(password).unwrap()
}

pub fn part2(input: &[u8; 8]) -> usize {
    input.len()
}
