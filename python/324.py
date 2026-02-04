"""Project Euler Problem 324 - Building a tower.

Find f(10^10000) mod 100000007, where f(n) is the number of ways
to fill a 3x3xn tower with 2x1x1 blocks.

Uses transition matrix with symmetry reduction to 46 states,
squaring trick to 23 states, then matrix exponentiation with numpy.
"""
import numpy as np

def solve():
    Q = 100000007
    K = 3
    KK = K * K  # 9
    FULL = (1 << KK) - 1

    def rotate(face):
        new = 0
        for i in range(K):
            for j in range(K):
                if face & (1 << (K * i + j)):
                    new |= 1 << (K * j + (K - 1 - i))
        return new

    def flip(face):
        new = 0
        for i in range(K):
            for j in range(K):
                if face & (1 << (K * i + j)):
                    new |= 1 << (K * i + (K - 1 - j))
        return new

    def face_class(one):
        transforms = set()
        f = one
        for _ in range(4):
            transforms.add(f)
            transforms.add(flip(f))
            f = rotate(f)
        return frozenset(transforms)

    def fill_layer(mask_cur, mask_out, results):
        inv = (~mask_cur) & FULL
        if inv == 0:
            results.append(mask_out)
            return
        pos = (inv & -inv).bit_length() - 1
        fill_layer(mask_cur | (1 << pos), mask_out | (1 << pos), results)
        row, col = divmod(pos, K)
        nei = pos + 1
        if col + 1 < K and not (mask_cur & (1 << nei)):
            fill_layer(mask_cur | (1 << pos) | (1 << nei), mask_out, results)
        nei = pos + K
        if row + 1 < K and not (mask_cur & (1 << nei)):
            fill_layer(mask_cur | (1 << pos) | (1 << nei), mask_out, results)

    raw_next = {}
    for mask_in in range(1 << KK):
        results = []
        fill_layer(mask_in, 0, results)
        raw_next[mask_in] = results

    raw_to_face = {}
    for s in range(1 << KK):
        raw_to_face[s] = face_class(s)

    from collections import deque
    face0 = raw_to_face[0]
    reachable_faces = set()
    queue = deque([face0])
    reachable_faces.add(face0)
    face_transitions = {}
    while queue:
        fc = queue.popleft()
        rep = min(fc)
        trans = {}
        for ns in raw_next[rep]:
            nfc = raw_to_face[ns]
            trans[nfc] = trans.get(nfc, 0) + 1
            if nfc not in reachable_faces:
                reachable_faces.add(nfc)
                queue.append(nfc)
        face_transitions[fc] = trans

    faces = sorted(reachable_faces, key=lambda f: min(f))
    face_idx = {f: i for i, f in enumerate(faces)}
    nf = len(faces)

    # Build transition matrix A
    A_mat = np.zeros((nf, nf), dtype=np.int64)
    for fc in faces:
        for nfc, cnt in face_transitions.get(fc, {}).items():
            A_mat[face_idx[nfc]][face_idx[fc]] = cnt % Q

    # Squaring trick: keep only even-popcount faces
    even_faces = [f for f in faces if bin(min(f)).count('1') % 2 == 0]
    even_idx = {f: i for i, f in enumerate(even_faces)}
    ne = len(even_faces)

    # Map even faces to their indices in the full matrix
    even_full_idx = [face_idx[f] for f in even_faces]

    # Compute A^2 restricted to even-popcount faces
    # A2[i][j] = sum_k A[even_i][k] * A[k][even_j]
    A_even_rows = A_mat[even_full_idx, :]  # ne x nf
    A_even_cols = A_mat[:, even_full_idx]  # nf x ne
    A2 = (A_even_rows @ A_even_cols) % Q

    # Matrix power mod Q using numpy
    def mat_mul_mod(A, B):
        # For 23x23 matrices with entries < Q (~10^8), product entries < 23 * Q^2 ~ 2.3*10^17
        # which fits in int64 (max ~9.2*10^18)
        return (A @ B) % Q

    def mat_pow_mod(M, exp):
        n = M.shape[0]
        result = np.eye(n, dtype=np.int64)
        base = M.copy()
        while exp > 0:
            if exp & 1:
                result = mat_mul_mod(result, base)
            base = mat_mul_mod(base, base)
            exp >>= 1
        return result

    exponent = 10**10000 // 2
    ei = even_idx[face0]
    powered = mat_pow_mod(A2, exponent)
    return int(powered[ei][ei])

if __name__ == "__main__":
    print(solve())
