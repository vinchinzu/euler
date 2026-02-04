"""Project Euler Problem 300 - Protein Folding

Find the average maximum number of H-H contact points in an optimal folding
of a random protein string of length 15.

Approach (following the Java reference):
1. Enumerate all self-avoiding walks (SAWs) of length 15, using symmetry
   reduction (fix first step east, second step not south).
2. Encode each walk as a "contact bitset" over even-odd position pairs:
   bit (7*i + j) is set if positions 2i and 2j+1 are lattice-adjacent.
3. Deduplicate and prune dominated bitsets (remove any bitset that is a
   strict subset of another).
4. For each of 2^15 protein masks, compute the protein's contact bitset
   and find the maximum popcount(protein_bs AND folding_bs).
5. Return the average of maximum contacts across all masks.
"""

import numpy as np


def solve() -> float:
    N = 15
    HALF = N // 2  # 7
    DIRS = ((1, 0), (-1, 0), (0, 1), (0, -1))

    # Step 1: Enumerate SAWs and collect unique contact bitsets
    GRID_SIZE = 2 * N + 1
    OFFSET = N
    grid = [[-1] * GRID_SIZE for _ in range(GRID_SIZE)]

    contact_bitsets = set()

    def dfs(step, x, y, bitset):
        if step == N:
            contact_bitsets.add(bitset)
            return
        if step == 1:
            dirs = ((1, 0),)
        elif step == 2:
            dirs = ((1, 0), (0, 1))
        else:
            dirs = DIRS
        for dx, dy in dirs:
            nx, ny = x + dx, y + dy
            gx, gy = nx + OFFSET, ny + OFFSET
            if grid[gy][gx] >= 0:
                continue
            new_bits = 0
            for ddx, ddy in DIRS:
                nnx, nny = nx + ddx, ny + ddy
                ggx, ggy = nnx + OFFSET, nny + OFFSET
                if 0 <= ggx < GRID_SIZE and 0 <= ggy < GRID_SIZE:
                    prev = grid[ggy][ggx]
                    if prev < 0:
                        continue
                    if step % 2 == 0 and prev % 2 == 1:
                        new_bits |= 1 << (HALF * (step // 2) + prev // 2)
                    elif step % 2 == 1 and prev % 2 == 0:
                        new_bits |= 1 << (HALF * (prev // 2) + step // 2)
            grid[gy][gx] = step
            dfs(step + 1, nx, ny, bitset | new_bits)
            grid[gy][gx] = -1

    grid[OFFSET][OFFSET] = 0
    dfs(1, 0, 0, 0)

    # Step 2: Prune dominated bitsets
    bitsets_list = sorted(contact_bitsets)
    pruned = []
    for i in range(len(bitsets_list)):
        bs = bitsets_list[i]
        dominated = False
        for j in range(i + 1, len(bitsets_list)):
            if (bs & bitsets_list[j]) == bs:
                dominated = True
                break
        if not dominated:
            pruned.append(bs)

    # Step 3: Precompute protein contact bitsets for all 2^N protein masks
    protein_bs_arr = np.zeros(1 << N, dtype=np.int64)
    for protein in range(1 << N):
        even_bits = 0
        for i in range(8):  # positions 0,2,4,6,8,10,12,14
            if protein & (1 << (2 * i)):
                even_bits |= 1 << i
        odd_bits = 0
        for j in range(7):  # positions 1,3,5,7,9,11,13
            if protein & (1 << (2 * j + 1)):
                odd_bits |= 1 << j
        bs = 0
        for i in range(8):
            if even_bits & (1 << i):
                bs |= odd_bits << (7 * i)
        protein_bs_arr[protein] = bs

    # Step 4: For each protein, find max popcount(protein_bs & folding_bs)
    folding_bs = np.array(pruned, dtype=np.int64)

    def popcount32(x):
        """Vectorized popcount for arrays of uint32."""
        x = x - ((x >> 1) & np.uint32(0x55555555))
        x = (x & np.uint32(0x33333333)) + ((x >> 2) & np.uint32(0x33333333))
        x = (x + (x >> 4)) & np.uint32(0x0F0F0F0F)
        return ((x * np.uint32(0x01010101)) >> 24).astype(np.int32)

    sum_contacts = 0
    batch_size = 1024
    for start in range(0, 1 << N, batch_size):
        end = min(start + batch_size, 1 << N)
        pbs = protein_bs_arr[start:end]
        max_contacts = np.zeros(end - start, dtype=np.int32)
        for fbs in folding_bs:
            anded = pbs & fbs
            lo = (anded & 0xFFFFFFFF).astype(np.uint32)
            hi = ((anded >> 32) & 0xFFFFFFFF).astype(np.uint32)
            count = popcount32(lo) + popcount32(hi)
            np.maximum(max_contacts, count, out=max_contacts)
        sum_contacts += int(np.sum(max_contacts))

    return sum_contacts / (1 << N)


if __name__ == "__main__":
    print(solve())
