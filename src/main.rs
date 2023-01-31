fn main() {
    /*    let map = &mut [0; 29];
        let arr = &mut [5, 3, 2, 2, 1, 1];

        reachable(arr, map);

        for i in 100..999 {
            if get(map, i) {
                println!("{}", i);
            }
        }
    */
    println!("{}", percentage([11, 37, 61, 89]));
}

fn percentage(nums: [u64; 4]) -> f64 {
    let mut fract = 0;
    let mut count = 0_u64;
    let mut idx;

    for i in (1..=4).rev() {
        idx = [0, 1, 2, 3];

        loop {
            let nums = idx.map(|i| nums[i]);
            let (pp, c) = p_of_comb(&nums[..i]);
            fract += pp;
            count += c;

            if !next_c(4, &mut idx[..i]) {
                break;
            }
        }
    }

    fract as f64 / count as f64
}

fn p_of_comb(comb: &[u64]) -> (u64, u64) {
    let rem = 6 - comb.len();
    let mut arr = [0; 6];
    let mut pp = 0_u64;
    let mut count = 0;

    arr[rem..].copy_from_slice(comb);
    arr[..rem].copy_from_slice(&[1, 2, 3, 4, 5, 6][..rem]);

    loop {
        pp += num_reachable(decode(rem, arr));
        count += 9;

        if !next_game(&mut arr[..rem]) {
            break (pp, count);
        }
    }
}

fn decode(len: usize, mut arr: [u64; 6]) -> [u64; 6] {
    for i in 0..len {
        let c = arr[i] as usize;
        if c > 10 {
            arr[i] = arr[c - 11];
        }
    }

    arr
}

fn next_game(curr: &mut [u64]) -> bool {
    let len_uniq = curr.iter().position(|&i| i > 10).unwrap_or(curr.len());

    next_g(10 + len_uniq as u64, curr)
}

fn next_g(mut len: u64, curr: &mut [u64]) -> bool {
    for i in (0..curr.len()).rev() {
        if curr[i] < len {
            if curr[i] == 10 && curr.len() - i > i {
                if !next_g(10, &mut curr[..=i]) {
                    return false;
                }
            } else {
                curr[i] += 1;
            }
            let mut prev = curr[i];

            for i in i + 1..curr.len() {
                curr[i] = prev + 1;
                prev += 1;
            }

            return true;
        }

        len -= 1;
    }

    false
}

fn next_c(mut len: usize, curr: &mut [usize]) -> bool {
    for i in (0..curr.len()).rev() {
        len -= 1;

        if curr[i] < len {
            curr[i] += 1;
            let mut prev = curr[i];

            for i in i + 1..curr.len() {
                curr[i] = prev + 1;
                prev += 1;
            }

            return true;
        }
    }

    false
}

fn get(arr: &[bool; 900], idx: u64) -> bool {
    let idx = idx - 100;
    arr[idx as usize]
}

fn insert(arr: &mut [bool; 900], idx: u64) {
    if (100..=999).contains(&idx) {
        let idx = idx - 100;
        arr[idx as usize] = true;
    }
}

fn num_reachable(mut nums: [u64; 6]) -> u64 {
    let mut map = [false; 900];

    nums.iter().for_each(|&i| insert(&mut map, i));

    reachable(&mut nums, &mut map);

    map.iter().copied().map(|i| i as u64).sum::<u64>()
}

fn dummy(_: &mut [u64], _: &mut [bool; 900]) {}

fn reachable(nums: &mut [u64], map: &mut [bool; 900]) {
    let reachable = if nums.len() < 3 { dummy } else { reachable };

    for fst in 0..nums.len() {
        let first = nums[fst];

        nums.swap(0, fst);

        let nums_cpy = &mut nums[1..];

        for sec in fst..nums_cpy.len() {
            let el = nums_cpy[sec];
            let mut f = |calc| {
                insert(map, calc);
                nums_cpy[sec] = calc;

                reachable(nums_cpy, map);
            };

            let (gt, lt) = (first.max(el), first.min(el));

            if lt > 0 {
                f(gt + lt);
                if gt != 2 * lt {
                    f(gt - lt);
                }
            }

            if lt > 1 {
                f(gt * lt);

                let calc = gt / lt;
                if lt != calc && calc * lt == gt {
                    f(calc);
                }
            }

            nums_cpy[sec] = el;
        }
        nums.swap(0, fst);
    }
}
