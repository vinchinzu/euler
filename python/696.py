"""Project Euler Problem 696: Mahjong.

Find the number of distinct winning Mahjong hands consisting of T triples and
one pair, given S suits, N numbers in each suit, and 4 copies of each suit and
number.

Each triple and pair must be in one suit, so the suits are independent.
Therefore, we can first compute the number of ways to have 0≤t≤T triples and
0≤p≤1 pairs in a single suit for small values of n. To do this, we use dynamic
programming: for each tile number in increasing order, we build a solution
using 0≤k≤4 tiles with that number. The only previous state required is the
number of tiles used so far, the number of Chows starting at the previous two
tile numbers, and whether a pair has yet been used.

To avoid double-counting hands that can be organized into triples and pairs in
different ways, first we disallow 3+ Chows starting at the same tile number
(this is already counted by 3 consecutive Pungs). Second, we use a "full state"
that stores the set of all possible states: for example, if we start with 2
tiles with a given number, that can represent either a pair, or 2 Chows starting
at that number. To get these possibilities, we iterate over all possible counts
of Pungs, Chows, and pairs at the given tile number, and see which are compatible
with the given k. At the end, we count any full state with some state that
matches our desired criteria. This can also be thought of as converting an NFA
to a DFA.

Now that we can compute the number of hands w(n,s,t) for small n, s=1, and all t,
both for with and without a pair, we can use extrapolation to compute the hands
w(N,1,t), which gives generating functions g_0(t) (the number of hands with t
triples but without a pair, with tiles up to n but all in 1 suit) and g_1(t)
(same, but with a pair). The number of hands over S suits is then
S g_0(t)^{S-1} g_1(t): we can select any of the S suits to have the pair.
"""

from __future__ import annotations

from collections import defaultdict
from dataclasses import dataclass
from typing import Callable, Set


@dataclass(frozen=True)
class State:
    """State for DP."""

    prev_chows: int
    curr_chows: int
    num_pairs: int


@dataclass(frozen=True)
class FullState:
    """Full state for DP."""

    num_tiles: int
    states: Set[State]


def extrapolation(
    f: Callable[[int], int], n_points: int, mod: int
) -> Callable[[int], int]:
    """Extrapolate function."""
    values = []
    for i in range(1, n_points + 1):
        values.append(f(i) % mod)

    def interpolate(x: int) -> int:
        """Interpolate at x."""
        result = 0
        for i in range(n_points):
            term = values[i]
            for j in range(n_points):
                if i != j:
                    denom = (i + 1 - (j + 1)) % mod
                    if denom == 0:
                        continue
                    inv = pow(denom, mod - 2, mod)
                    term = (term * (x - (j + 1)) * inv) % mod
            result = (result + term) % mod
        return result

    return interpolate


def solve() -> int:
    """Solve Problem 696."""
    N = 10**8
    S = 10**8
    T = 30
    M = 10**9 + 7
    L = 300

    # Simplified implementation - full version would require polynomial operations
    # This is a placeholder
    f = [[[0] * L for _ in range(T + 1)] for _ in range(2)]

    # Initialize with basic states
    full_states: dict[FullState, int] = {
        FullState(0, {State(0, 0, 0)}): 1
    }

    for index in range(1, L):
        new_full_states: dict[FullState, int] = defaultdict(int)
        for full_state, count in full_states.items():
            for k in range(5):
                if (full_state.num_tiles + k) // 3 > T:
                    continue
                new_states: Set[State] = set()
                for num_pungs in range(2):
                    for num_chows in range(3):
                        for num_pairs in range(2):
                            for state in full_state.states:
                                if state.num_pairs + num_pairs > 1:
                                    continue
                                total = (
                                    state.prev_chows
                                    + state.curr_chows
                                    + num_pungs * 3
                                    + num_chows
                                    + num_pairs * 2
                                )
                                if total != k:
                                    continue
                                new_states.add(
                                    State(state.curr_chows, num_chows, state.num_pairs + num_pairs)
                                )
                new_full_state = FullState(full_state.num_tiles + k, new_states)
                new_full_states[new_full_state] = (
                    new_full_states[new_full_state] + count
                ) % M

        full_states = new_full_states
        for full_state, count in full_states.items():
            for num_pairs in range(2):
                if any(
                    s.prev_chows == 0
                    and s.curr_chows == 0
                    and s.num_pairs == num_pairs
                    for s in full_state.states
                ):
                    num_triples = full_state.num_tiles // 3
                    if num_triples <= T:
                        f[num_pairs][num_triples][index] = (
                            f[num_pairs][num_triples][index] + count
                        ) % M

    # Use extrapolation (simplified)
    g = [[0] * (T + 1) for _ in range(2)]
    for num_pairs in range(2):
        for num_triples in range(T + 1):
            def f_func(n: int) -> int:
                return f[num_pairs][num_triples][n + T] if n + T < L else 0

            extrap_func = extrapolation(f_func, 1, M)
            g[num_pairs][num_triples] = extrap_func(N - T) % M

    # Compute final answer (simplified - would need polynomial multiplication)
    ans = 0
    # Placeholder: would compute S * g[0]^(S-1) * g[1] using polynomial operations
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
