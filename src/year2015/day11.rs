pub fn parse(input: &str) -> [u8; 8] {
    input.as_bytes().try_into().unwrap()
}

pub fn part1(password: &[u8; 8]) -> String {
    let mut password = password.to_vec();
    loop {
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

        if !password
            .windows(3)
            .any(|w| w[0] + 1 == w[1] && w[1] + 1 == w[2])
        {
            continue;
        }

        if password
            .iter()
            .any(|&c| c == b'i' || c == b'o' || c == b'l')
        {
            continue;
        }

        if (b'a'..=b'z')
            .filter(|&c| password.windows(2).any(|w| w[0] == c && w[1] == c))
            .count()
            < 2
        {
            continue;
        }

        break;
    }

    String::from_utf8(password).unwrap()
}

pub fn part2(input: &[u8; 8]) -> usize {
    input.len()
}
