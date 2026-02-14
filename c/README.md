# Pure C Solutions (Problems 1-200)

200 Project Euler solutions in pure C (C99/C11).
- Problems 1-100: translated from C++ solutions in `../cpp/`
- Problems 101-200: translated from Python solutions in `../python/`

## Build & Run

```bash
# Compile a single solution
gcc -O2 -o c/NNN c/NNN.c -lm

# Compile all 1-200
for f in c/*.c; do n="${f%.c}"; gcc -O2 -o "$n" "$f" -lm; done

# Run
./c/001
```

## Benchmark (2026-02-14)

### Problems 1-100
- **C total: 3.1s** vs Python total: 63.1s = **20.2x overall speedup**
- **100/100 correct** answers

### Problems 101-200
- **C total: 27.8s** — **100/100 correct** answers
- Slowest: P155 (6.2s), P153 (3.6s), P167 (3.5s)

### Combined 1-200
- **C total: ~31s** — **200/200 correct**

## Data Files

Some solutions read data files from `../data/`:
- P022: `p022_names.txt`
- P042: `p042_words.txt`
- P054: `p054_poker.txt`
- P059: `p059_cipher.txt`
- P067: `p067_triangle.txt`
- P079: `p079_keylog.txt`
- P081/082/083: `p081_matrix.txt`
- P089: `p089_roman.txt`
- P096: `p096_sudoku.txt`
- P098: `p098_words.txt`
- P099: `p099_base_exp.txt`
