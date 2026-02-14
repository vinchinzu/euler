fn count_single(d: i64, mut v: i64) -> i32 {
    if v == 0 && d == 0 { return 1; }
    let mut res = 0;
    while v > 0 {
        if v % 10 == d { res += 1; }
        v /= 10;
    }
    res
}

fn count_digit(d: i64, v: i64) -> i64 {
    if v < 0 { return 0; }
    if v < 10 { return if v >= d { 1 } else { 0 }; }

    let base: i64 = 10;
    let mut shift: i64 = 1;
    let mut multi: i64 = 0;
    while shift * base <= v {
        shift *= base;
        multi += 1;
    }
    multi *= shift / base;

    let first = v / shift;
    let rem = v % shift;
    let mut res = first * multi + count_digit(d, rem);
    if d == first { res += rem + 1; }
    if d < first && d > 0 { res += shift; }
    res
}

fn find_all(d: i64, fr: i64, to_n: i64) -> i64 {
    let center = (fr + to_n) / 2;
    if fr == center {
        return if count_digit(d, fr) == fr { fr } else { 0 };
    }

    let mut result: i64 = 0;
    let mut cur_fr = fr;
    let mut cur_count = count_digit(d, fr);

    while cur_count == cur_fr && cur_fr < to_n {
        result += cur_fr;
        cur_fr += 1;
        cur_count += count_single(d, cur_fr) as i64;
    }
    if cur_fr >= to_n + 1 { return result; }

    let fr = cur_fr;
    let count_fr_val = cur_count;
    let center = (fr + to_n) / 2;
    let count_center = count_digit(d, center);
    let count_to = count_digit(d, to_n);

    if count_center >= fr && center >= count_fr_val && center > fr {
        result += find_all(d, fr, center);
    }
    if count_to >= center && to_n >= count_center && center < to_n {
        result += find_all(d, center, to_n);
    }
    result
}

fn main() {
    let max_n: i64 = 1_000_000_000_000;
    let mut total: i64 = 0;
    for d in 1..=9 {
        total += find_all(d, 0, max_n);
    }
    println!("{}", total);
}
