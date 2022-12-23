pub(crate) fn part1(input: &str) -> usize {
    fn inc(counters: u128, c: char) -> u128 {
        counters + (1 << 3 * (c as u8 - b'a'))
    }

    fn dec(counters: u128, c: char) -> u128 {
        counters - (1 << 3 * (c as u8 - b'a'))
    }

    fn detected(counters: u128) -> bool {
        0 == counters & 0xb6db6db6_db6db6db_6db6db6d_b6db6db6
    }

    // This assumes the detection is at n if the there's none found for the
    // first n-1 elements.
    input
        .chars()
        .zip(input.chars().skip(4))
        .scan(
            // Prime the counts
            input.chars().take(4).fold(0, |counts, c| inc(counts, c)),
            |counts, (old, cur)| {
                if detected(*counts) {
                    None
                } else {
                    // abcdef -> (a)bcd(e) -> add e, sub a
                    *counts = dec(inc(*counts, cur), old);
                    Some(true)
                }
            },
        )
        .count()
        + 4
}

pub(crate) fn part2(input: &str) -> usize {
    fn inc(mut counters: [u8; 26], c: char) -> [u8; 26] {
        counters[(c as u8 - b'a') as usize] += 1;
        counters
    }

    fn dec(mut counters: [u8; 26], c: char) -> [u8; 26] {
        counters[(c as u8 - b'a') as usize] -= 1;
        counters
    }

    fn detected(counters: [u8; 26]) -> bool {
        counters.iter().all(|&c| c <= 1)
    }

    // This assumes the detection is at n if the there's none found for the
    // first n-1 elements.
    input
        .chars()
        .zip(input.chars().skip(14))
        .scan(
            // Prime the counts
            input
                .chars()
                .take(14)
                .fold([0; 26], |counts, c| inc(counts, c)),
            |counts, (old, cur)| {
                if detected(*counts) {
                    None
                } else {
                    // abcdef -> (a)bcd(e) -> add e, sub a
                    *counts = dec(inc(*counts, cur), old);
                    Some(true)
                }
            },
        )
        .count()
        + 14
}

#[test]
fn day6() {
    assert_eq!(7, part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
    assert_eq!(5, part1("bvwbjplbgvbhsrlpgdmjqwftvncz"));
    assert_eq!(6, part1("nppdvjthqldpwncqszvftbrmjlhg"));
    assert_eq!(10, part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
    assert_eq!(11, part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));

    assert_eq!(19, part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
    assert_eq!(23, part2("bvwbjplbgvbhsrlpgdmjqwftvncz"));
    assert_eq!(23, part2("nppdvjthqldpwncqszvftbrmjlhg"));
    assert_eq!(29, part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
    assert_eq!(26, part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
}
