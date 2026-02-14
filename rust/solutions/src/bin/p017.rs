// Project Euler 17: Letter count for numbers 1-1000 written in words

fn word_length(n: u32) -> u32 {
    let ones_len = [0, 3, 3, 5, 4, 4, 3, 5, 5, 4];
    // one=3 two=3 three=5 four=4 five=4 six=3 seven=5 eight=5 nine=4
    let teens_len = [3, 6, 6, 8, 8, 7, 7, 9, 8, 8];
    // ten=3 eleven=6 twelve=6 thirteen=8 fourteen=8 fifteen=7
    // sixteen=7 seventeen=9 eighteen=8 nineteen=8
    let tens_len = [0, 0, 6, 6, 5, 5, 5, 7, 6, 6];
    // twenty=6 thirty=6 forty=5 fifty=5 sixty=5 seventy=7 eighty=6 ninety=6

    if n == 1000 {
        return 11; // "onethousand"
    }

    let mut len = 0u32;

    if n >= 100 {
        len += ones_len[(n / 100) as usize];
        len += 7; // "hundred"
        if n % 100 != 0 {
            len += 3; // "and"
        }
    }

    let rem = n % 100;
    if rem >= 20 {
        len += tens_len[(rem / 10) as usize];
        if rem % 10 != 0 {
            len += ones_len[(rem % 10) as usize];
        }
    } else if rem >= 10 {
        len += teens_len[(rem - 10) as usize];
    } else if rem > 0 {
        len += ones_len[rem as usize];
    }

    len
}

fn main() {
    let total: u32 = (1..=1000).map(word_length).sum();
    println!("{total}");
}
