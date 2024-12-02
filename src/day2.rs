fn is_safe(i: impl DoubleEndedIterator<Item = u32> + Clone) -> bool {
    fn is_safe_increasing(mut i: impl Iterator<Item = u32>) -> bool {
        let Some(mut current) = i.next() else {
            return false;
        };

        for next in i {
            if next
                .checked_sub(current)
                .is_some_and(|diff| diff != 0 && diff <= 3)
            {
                current = next;
            } else {
                return false;
            }
        }

        true
    }

    is_safe_increasing(i.clone()) || is_safe_increasing(i.rev())
}

#[aoc_runner_derive::aoc(day2, part1)]
fn part1(input: &str) -> usize {
    let mut line_values = Vec::new();

    input
        .lines()
        .filter(|line| {
            if line.is_empty() {
                return false;
            }

            line_values.clear();

            line_values.extend(
                line.split_ascii_whitespace()
                    .map(|value| value.parse::<u32>().unwrap()),
            );

            is_safe(line_values.iter().copied())
        })
        .count()
}

#[aoc_runner_derive::aoc(day2, part2)]
fn part2(input: &str) -> usize {
    let mut line_values = Vec::new();

    input
        .lines()
        .filter(|line| {
            if line.is_empty() {
                return false;
            }

            line_values.clear();

            line_values.extend(
                line.split_ascii_whitespace()
                    .map(|value| value.parse::<u32>().unwrap()),
            );

            fn is_safe_skipping(
                i: impl DoubleEndedIterator<Item = u32> + ExactSizeIterator + Clone,
                n: usize,
            ) -> bool {
                is_safe(i.clone().take(n).chain(i.skip(n + 1)))
            }

            (0..line_values.len()).any(|n| is_safe_skipping(line_values.iter().copied(), n))
        })
        .count()
}
