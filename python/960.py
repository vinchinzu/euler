#!/usr/bin/env python3
r"""
Project Euler Problem 960 - Stone Pile Game

Problem:
- Start with n piles, each containing n-1 stones
- Each turn: remove exactly n stones total from any two piles
- Add min(a,b) to score where a,b are stones removed from each pile
- Goal: empty all piles (if stuck, score = 0)
- F(n) = sum of final scores for all successful sequences

Given:
- F(3) = 12
- F(4) = 360
- F(8) = 16785941760

Find: F(100) mod 10^9+7
There are $n$ distinct piles of stones, each of size $n-1$. Starting with an initial score of $0$, the following procedure is repeated:
1. Choose any two piles and remove exactly $n$ stones in total from the two piles.
2. If the number of stones removed from the two piles were $a$ and $b$, add $\min(a,b)$ to the score.
If all piles are eventually emptied, the current score is confirmed as final. However, if one gets "stuck" and cannot empty all piles, the current score is discarded, resulting in a final score of $0$.
Three example sequences of turns are illustrated below for $n=4$, with each tuple representing pile sizes as one proceeds, and with additions to the score indicated above the arrows.
$$
\begin{align}
&(3,3,3,3)\xrightarrow{+1}(0,3,2,3)\xrightarrow{+1}(0,3,1,0)\xrightarrow{+1}(0,0,0,0)&:\quad\text{final score }=3\\
&(3,3,3,3)\xrightarrow{+1}(3,0,3,2)\xrightarrow{+2}(1,0,3,0)\xrightarrow{+1}(0,0,0,0)&:\quad\text{final score }=4\\
&(3,3,3,3)\xrightarrow{+2}(1,3,1,3)\xrightarrow{+1}(1,2,1,0)\rightarrow\text{stuck!}&:\quad\text{final score }=0
\end{align}
$$
Define $F(n)$ to be the sum of the final scores achieved for every sequence of turns which successfully empty all piles.
You are given $F(3)=12$, $F(4)=360$, and $F(8)=16785941760$.
Find $F(100)$. Give your answer modulo $10^9+7$
"""

import math
from functools import lru_cache
from typing import List, Tuple, Set


MOD = 10**9 + 7


def create_initial_state(n: int) -> List[int]:
    """Create initial game state with n piles of n-1 stones each."""
    return [n - 1] * n


def is_complete(state: List[int]) -> bool:
    """Check if all piles are empty."""
    return all(pile == 0 for pile in state)


def is_stuck(state: List[int], n: int) -> bool:
    """
    Check if the game is stuck (no valid moves possible).
    A state is stuck if it's not complete and no valid moves exist.
    """
    if is_complete(state):
        return False

    # Check if any valid move exists
    for i in range(len(state)):
        for j in range(i + 1, len(state)):
            # Try all ways to split n stones between piles i and j
            for a in range(min(state[i], n) + 1):
                b = n - a
                if b >= 0 and b <= state[j]:
                    return False  # Found a valid move
    return True


def generate_moves(state: List[int], n: int) -> List[Tuple[int, int, int, int]]:
    """
    Generate all valid moves from current state.
    Returns list of (pile1_idx, pile2_idx, stones_from_pile1, stones_from_pile2).
    """
    moves = []
    for i in range(len(state)):
        for j in range(i + 1, len(state)):
            # Try all ways to remove exactly n stones from piles i and j
            for a in range(min(state[i], n) + 1):
                b = n - a
                if 0 <= b <= state[j]:
                    moves.append((i, j, a, b))
    return moves


def calculate_score_addition(a: int, b: int) -> int:
    """Calculate the score addition for removing a and b stones."""
    return min(a, b)


def apply_move(state: List[int], pile1_idx: int, pile2_idx: int, a: int, b: int) -> Tuple[int, ...]:
    """
    Apply a move to the state and return new state as tuple.
    Returns tuple for hashability.
    """
    new_state = list(state)
    new_state[pile1_idx] -= a
    new_state[pile2_idx] -= b
    return tuple(sorted(new_state, reverse=True))  # Sort for canonical form


def power(a, b):
    return pow(a, b, MOD)


def inverse(n):
    return power(n, MOD - 2)


def nCr_mod(n, r, fact, invFact):
    if r < 0 or r > n:
        return 0
    num = fact[n]
    den = (invFact[r] * invFact[n - r]) % MOD
    return (num * den) % MOD


def calculate_f_formula(n: int, use_mod: bool = False) -> int:
    """
    Calculate F(n) using the analytic formula.
    F(n) = (n-1)!/2 * sum_{k=1}^{n-1} [C(n,k) * k^(k-1) * (n-k)^(n-k-1) * min(k, n-k)]
    """
    if n < 3:
        # For n=2: F(2): 2 piles of 1. Move remove 2. 1+1=2. Score min(1,1)=1.
        # Formula: k=1. C(2,1)*1^0*1^0*min(1,1) = 2*1*1*1 = 2.
        # (2-1)!/2 * 2 = 1. Correct.
        pass

    if not use_mod:
        # Exact calculation using integer arithmetic
        total_sum = 0
        for k in range(1, n):
            term = math.comb(n, k)
            if k == 1:
                term *= 1
            else:
                term *= pow(k, k - 1)

            if n - k == 1:
                term *= 1
            else:
                term *= pow(n - k, n - k - 1)

            term *= min(k, n - k)
            total_sum += term

        result = math.factorial(n - 1) * total_sum // 2
        return result
    else:
        # Modular calculation
        fact = [1] * (n + 1)
        invFact = [1] * (n + 1)
        for i in range(1, n + 1):
            fact[i] = (fact[i - 1] * i) % MOD

        invFact[n] = inverse(fact[n])
        for i in range(n - 1, -1, -1):
            invFact[i] = (invFact[i + 1] * (i + 1)) % MOD

        total_sum = 0
        for k in range(1, n):
            term = nCr_mod(n, k, fact, invFact)
            term = (term * power(k, k - 1)) % MOD
            term = (term * power(n - k, n - k - 1)) % MOD
            term = (term * min(k, n - k)) % MOD
            total_sum = (total_sum + term) % MOD

        result = (fact[n - 1] * inverse(2)) % MOD
        result = (result * total_sum) % MOD

        return result


def compute_f(n: int, use_mod: bool = False) -> int:
    """
    Compute F(n) - sum of all final scores for all successful sequences.
    If use_mod is True, computes result modulo MOD.
    """
    return calculate_f_formula(n, use_mod)


def solve_optimized(n: int, use_mod: bool = True) -> int:
    """
    Wrapper for optimized solver (now using formula).
    """
    return calculate_f_formula(n, use_mod)


def solve() -> int:
    """Solve F(100) mod 10^9+7."""
    result = calculate_f_formula(100, use_mod=True)
    return result


def main():
    """Main entry point."""
    # Output only the numeric result
    result = solve()
    print(result)


if __name__ == "__main__":
    main()
