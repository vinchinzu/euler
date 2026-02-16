// Problem 909: L-expressions I
//
// Evaluate S(S)(S(S))(S(S))(S(Z))(A)(0), report last 9 digits.
//
// Rules:
//   A(x) -> x + 1   (x is a natural number)
//   Z(u)(v) -> v     (for any L-expressions u, v)
//   S(u)(v)(w) -> v(u(v)(w))
//
// Analysis:
//   Z = church(0), S(Z) = church(1), S acts as successor.
//   SS = S(S): at the Church numeral function level, maps count k to k*(k+1).
//   SZ acts as identity.
//
//   Let E = S(SS)(SS). The expression is E(E(SZ))(A)(0).
//
//   E(SZ) = SS(SS(SS)(SZ)):
//     SS(SZ) -> count 2
//     S(SS)(SZ) = SZ is identity passes through -> count 2
//     SS(S(SS)(SZ)) = SS(count 2) -> count 6
//     SS(count 6) -> count 42
//   So E(SZ) has count 42.
//
//   E(V) where V has count 42:
//   E(V) = SS(SS(SS)(V)):
//     SS(V) -> count 42*43 = 1806
//     S(SS)(V)(f) = V(f^1806) = f^{42*1806} = f^75852 -> count 75852
//     SS(count 75852) -> count 75852*75853 = 5753601756
//     SS(count 5753601756) -> count 5753601756*5753601757
//
//   Answer = 5753601756 * 5753601757 mod 10^9

fn main() {
    const MOD: u64 = 1_000_000_000;

    // E(SZ) -> count 42
    // E(count 42):
    //   SS: 42 -> 42*43 = 1806
    //   S(SS): count -> 42 * 1806 = 75852
    //   SS: 75852 -> 75852 * 75853 = 5753601756
    //   SS: 5753601756 -> 5753601756 * 5753601757 mod 10^9

    let c: u64 = 75852 * 75853; // 5753601756
    let answer = ((c % MOD) as u128 * ((c + 1) % MOD) as u128 % MOD as u128) as u64;
    println!("{}", answer);
}
