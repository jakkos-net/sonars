// 1-to-1 translated from https://github.com/brianhouse/bjorklund/blob/master/__init__.py

fn bjorklund(steps: u8, pulses: u8) -> Vec<u8> {
    if pulses > steps {
        panic!("Steps: {steps} cannot bigger than pulses: {pulses} in bjorklund algorithm!");
    }

    let mut pattern = vec![];
    let mut counts = vec![];
    let mut remainders = vec![];
    let mut divisor = steps - pulses;
    remainders.push(pulses);
    let mut level = 0;
    loop {
        counts.push(divisor / remainders[level]);
        remainders.push(divisor % remainders[level]);
        divisor = remainders[level];
        level = level + 1;
        if remainders[level] <= 1 {
            break;
        }
    }
    counts.push(divisor);

    fn build(pattern: &mut Vec<u8>, counts: &mut Vec<u8>, remainders: &mut Vec<u8>, level: i16) {
        if level == -1 {
            pattern.push(0);
        } else if level == -2 {
            pattern.push(1);
        } else {
            for _ in 0..counts[level as usize] {
                build(pattern, counts, remainders, level - 1)
            }

            if remainders[level as usize] != 0 {
                build(pattern, counts, remainders, level - 2)
            }
        }
    }

    let level = level as i16;
    build(&mut pattern, &mut counts, &mut remainders, level);
    let i = pattern.iter().position(|x| *x == 1).unwrap();
    let result = pattern[i as usize..]
        .iter()
        .chain(pattern[0..i as usize].iter())
        .cloned()
        .collect::<Vec<_>>();

    result
}

#[cfg(test)]
mod tests {
    use super::bjorklund;

    #[test]
    fn test_bjorklund() {
        assert_eq!(bjorklund(2, 1), vec![1, 0]);
        assert_eq!(bjorklund(7, 3), vec![1, 0, 1, 0, 1, 0, 0]);
        assert_eq!(bjorklund(8, 4), vec![1, 0, 1, 0, 1, 0, 1, 0]);
        assert_eq!(bjorklund(9, 1), vec![1, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(bjorklund(10, 10), vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
        assert_eq!(
            bjorklund(16, 3),
            vec![1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0]
        );

        assert_eq!(
            bjorklund(145, 92),
            vec![
                1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1,
                1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1,
                0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0,
                1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0,
                1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1,
                1, 0, 1, 1, 0
            ]
        );
    }
}
