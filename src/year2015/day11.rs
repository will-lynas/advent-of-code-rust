pub fn parse(input: &str) -> [u8; 8] {
    input.as_bytes().try_into().unwrap()
}

fn consecutive(password: &[u8]) -> bool {
    password
        .windows(3)
        .any(|w| w[0] + 1 == w[1] && w[1] + 1 == w[2])
}

fn two_pairs(password: &[u8]) -> bool {
    (b'a'..=b'z')
        .filter(|&c| password.windows(2).any(|w| w[0] == c && w[1] == c))
        .count()
        >= 2
}

fn increment(password: &mut [u8; 8]) {
    let mut i = 7;
    loop {
        match password[i] + 1 {
            // skip forbidden characters
            b'i' | b'o' | b'l' => {
                password[i] += 2;
            }
            _ => {
                password[i] += 1;
            }
        }
        if password[i] > b'z' {
            password[i] = b'a';
            i -= 1;
        } else {
            break;
        }
    }
}

fn make_valid(password: &mut [u8; 8]) {
    while !(consecutive(password) && two_pairs(password)) {
        increment(password);
    }
}

pub fn part1(password: &[u8; 8]) -> String {
    let mut password = password.to_owned();
    make_valid(&mut password);
    String::from_utf8(password.to_vec()).unwrap()
}

pub fn part2(password: &[u8; 8]) -> String {
    let mut password = password.to_owned();
    make_valid(&mut password);
    increment(&mut password);
    make_valid(&mut password);
    String::from_utf8(password.to_vec()).unwrap()
}
