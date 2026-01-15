P = 900497239

N = 10000

four_n = pow(4, N, P)

two_n = pow(2, N, P)

three_inv = pow(3, P - 2, P)

s = (four_n + 2) * three_inv - two_n

s %= P

print(s)