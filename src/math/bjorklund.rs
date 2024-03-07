use once_cell::sync::Lazy;

// 1-to-1 translated from https://github.com/brianhouse/bjorklund/blob/master/__init__.py
fn bjorklund_bool(steps: usize, pulses: usize) -> Vec<bool> {
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

    fn build(
        pattern: &mut Vec<usize>,
        counts: &mut Vec<usize>,
        remainders: &mut Vec<usize>,
        level: isize,
    ) {
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

    let level = level as isize;
    build(&mut pattern, &mut counts, &mut remainders, level);
    let i = pattern.iter().position(|x| *x == 1).unwrap();
    let result = pattern[i as usize..]
        .iter()
        .chain(pattern[0..i as usize].iter())
        .map(|x| *x == 1)
        .collect::<Vec<_>>();

    result
}

const BJORKLUND_CACHE_MAX: usize = 64;

static CACHED_BJORKLUND: Lazy<Vec<bool>> = Lazy::new(|| {
    let mut vec = vec![false; BJORKLUND_CACHE_MAX * BJORKLUND_CACHE_MAX * BJORKLUND_CACHE_MAX];
    for steps in 1..=BJORKLUND_CACHE_MAX {
        for pulses in 1..=steps {
            let pattern = bjorklund_bool(steps, pulses);
            for (index, val) in pattern.into_iter().enumerate() {
                let combined_index = cached_bjorklund_index(steps, pulses, index);
                vec[combined_index] = val;
            }
        }
    }

    vec
});

fn cached_bjorklund_index(steps: usize, pulses: usize, index: usize) -> usize {
    let steps_offset = (steps - 1) as usize * BJORKLUND_CACHE_MAX * BJORKLUND_CACHE_MAX;
    let pulses_offset = (pulses - 1) as usize * BJORKLUND_CACHE_MAX;
    steps_offset + pulses_offset + index as usize
}

pub fn cached_bjorklund(steps: usize, pulses: usize, index: usize) -> bool {
    CACHED_BJORKLUND[cached_bjorklund_index(steps, pulses, index)]
}

#[macro_export]
macro_rules! euc {
    ($steps: expr, $pulses: expr) => {
        |t: Float| {
            use $crate::math::bjorklund::cached_bjorklund;
            let steps: usize = $steps;
            let pulses: usize = $pulses;
            let index = (steps as Float * (t % 1.0)) as usize;
            if cached_bjorklund(steps, pulses, index) {
                (t * steps as Float) % 1.0
            } else {
                0.0
            }
        }
    };
}

#[cfg(test)]
mod tests {

    use crate::math::bjorklund::cached_bjorklund;

    use super::bjorklund_bool;

    #[test]
    fn test_bjorklund() {
        assert_eq!(bjorklund_bool(2, 1), vec![true, false]);
        assert_eq!(
            bjorklund_bool(7, 3),
            vec![true, false, true, false, true, false, false]
        );
        assert_eq!(
            bjorklund_bool(8, 4),
            vec![true, false, true, false, true, false, true, false]
        );
        assert_eq!(
            bjorklund_bool(9, 1),
            vec![true, false, false, false, false, false, false, false, false]
        );
        assert_eq!(
            bjorklund_bool(10, 10),
            vec![true, true, true, true, true, true, true, true, true, true]
        );
        assert_eq!(
            bjorklund_bool(16, 3),
            vec![
                true, false, false, false, false, true, false, false, false, false, true, false,
                false, false, false, false
            ]
        );

        assert_eq!(
            bjorklund_bool(145, 92),
            vec![
                true, false, true, true, false, true, true, false, true, false, true, true, false,
                true, true, false, true, true, false, true, false, true, true, false, true, true,
                false, true, true, false, true, false, true, true, false, true, true, false, true,
                true, false, true, false, true, true, false, true, true, false, true, false, true,
                true, false, true, true, false, true, true, false, true, false, true, true, false,
                true, true, false, true, true, false, true, false, true, true, false, true, true,
                false, true, true, false, true, false, true, true, false, true, true, false, true,
                true, false, true, false, true, true, false, true, true, false, true, false, true,
                true, false, true, true, false, true, true, false, true, false, true, true, false,
                true, true, false, true, true, false, true, false, true, true, false, true, true,
                false, true, true, false, true, false, true, true, false, true, true, false, true,
                true, false
            ]
        );
    }

    #[test]
    fn test_cached_bjorklund() {
        assert_eq!(bjorklund_bool(16, 3)[3], cached_bjorklund(16, 3, 3));
        assert_eq!(bjorklund_bool(19, 12)[4], cached_bjorklund(19, 12, 4));
        assert_eq!(bjorklund_bool(8, 7)[1], cached_bjorklund(8, 7, 1));
        assert_eq!(bjorklund_bool(8, 7)[2], cached_bjorklund(8, 7, 2));
        assert_eq!(bjorklund_bool(23, 4)[3], cached_bjorklund(24, 4, 3));
        for i in 0..32 {
            assert_eq!(bjorklund_bool(32, 7)[i], cached_bjorklund(32, 7, i));
        }
    }
}
