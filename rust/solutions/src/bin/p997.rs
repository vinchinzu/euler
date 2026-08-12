// Problem 997: Dice Box
// f(x,y,z) = 3 * 2^(x+y+z-1) * (2^x + 2^y + 2^z - 4)

fn f(x: u32, y: u32, z: u32) -> u64 {
    3 * (1u64 << (x + y + z - 1)) * ((1u64 << x) + (1u64 << y) + (1u64 << z) - 4)
}

fn main() {
    debug_assert_eq!(f(1, 1, 1), 24);
    debug_assert_eq!(f(2, 3, 4), 18432);
    println!("{}", f(9, 10, 11));
}
