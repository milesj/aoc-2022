fn is_all_diff_chars_u8(marker: &[u8]) -> bool {
    let mut o = 0;
    let mut i;

    while o < marker.len() {
        i = 0;

        while i < marker.len() {
            if o != i && &marker[o..o + 1] == &marker[i..i + 1] {
                return false;
            }

            i += 1;
        }

        o += 1;
    }

    true
}

fn part_1() {
    let data = include_str!("input.txt");

    let index = data
        .as_bytes()
        .windows(4)
        .position(|set| is_all_diff_chars_u8(&set))
        .unwrap()
        + 4;

    println!("marker index = {}", index);
}

fn part_2() {
    let data = include_str!("input.txt");

    let index = data
        .as_bytes()
        .windows(14)
        .position(|set| is_all_diff_chars_u8(&set))
        .unwrap()
        + 14;

    println!("marker index = {}", index);
}

fn main() {
    part_1();
    println!("-----");
    part_2();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff() {
        assert!(is_all_diff_chars("abcd"));
        assert!(is_all_diff_chars("wxyz"));
        assert!(is_all_diff_chars("fourte3ncha4s!"));

        assert!(!is_all_diff_chars("abcc"));
        assert!(!is_all_diff_chars("abbc"));
        assert!(!is_all_diff_chars("aaaa"));
    }
}
