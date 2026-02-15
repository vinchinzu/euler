// Project Euler 610 - Roman Numerals II
// Expected value of random Roman numeral string generation
//
// Symbols {I,V,X,L,C,D,M} each with probability q=0.14, # with p=0.02.
// Process: append letters to form valid minimal Roman numeral, stop at #.
// Expected value of the resulting number.
//
// Key insight: model states 0..999 (sub-thousand part), handle leading M's analytically.
// From empty state, the infinite M chain gives a geometric factor q/(1-q).

fn main() {
    let p: f64 = 0.02;
    let q: f64 = 0.14;

    // Build canonical Roman representations for 0..999
    let ones = ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];
    let tens = ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
    let huns = ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];

    let mut roman_str: Vec<String> = Vec::with_capacity(1000);
    for i in 0..1000 {
        let h = i / 100;
        let t = (i / 10) % 10;
        let o = i % 10;
        roman_str.push(format!("{}{}{}", huns[h], tens[t], ones[o]));
    }

    // Build reverse map: roman string -> number
    let mut r2n = std::collections::HashMap::new();
    for (i, s) in roman_str.iter().enumerate() {
        r2n.insert(s.as_str(), i);
    }

    // Build successor list for each state 0..999
    // Appending each of {I,V,X,L,C,D,M} to roman_str[n], check if result is valid
    let letters = ['I', 'V', 'X', 'L', 'C', 'D', 'M'];
    let mut succ: Vec<Vec<usize>> = Vec::with_capacity(1000);

    // We need owned strings for lookup since r2n borrows roman_str
    // Rebuild r2n with owned keys
    let mut r2n_owned: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for (i, s) in roman_str.iter().enumerate() {
        r2n_owned.insert(s.clone(), i);
    }

    for n in 0..1000usize {
        let mut succs = Vec::new();
        for &ch in &letters {
            let mut extended = roman_str[n].clone();
            extended.push(ch);
            if let Some(&target) = r2n_owned.get(&extended) {
                succs.push(target);
            }
        }
        succ.push(succs);
    }

    // Compute expected values E[n] for each state n = 0..999
    // Process from 999 down to 1 (state 0 = empty string handled separately)
    // E[n] = n if no successors
    // E[n] = (p*n + q*sum(E[m] for m in succ[n])) / (p + q*k) otherwise
    let mut e = vec![0.0_f64; 1000];
    for n in (1..1000).rev() {
        let k = succ[n].len();
        if k == 0 {
            e[n] = n as f64;
        } else {
            let sum_e: f64 = succ[n].iter().map(|&m| e[m]).sum();
            e[n] = (p * n as f64 + q * sum_e) / (p + q * k as f64);
        }
    }

    // From empty state: can draw any of {I,V,X,L,C,D,M} or #
    // M leads to another empty-like state but with +1000 contribution
    // The letters I,V,X,L,C,D lead to states with values 1,5,10,50,100,500
    // # leads to value 0
    //
    // The infinite M chain is handled analytically:
    // E0 = q/(1-q) * (1000 + sum(E[c] for c in initial non-M letters))
    let initial_letters = [
        ('I', 1), ('V', 5), ('X', 10), ('L', 50), ('C', 100), ('D', 500),
    ];
    let sum_initial: f64 = initial_letters.iter().map(|&(_, v)| e[v]).sum();

    let result = q / (1.0 - q) * (1000.0 + sum_initial);
    println!("{:.8}", result);
}
