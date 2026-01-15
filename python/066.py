#!/usr/bin/env python3
import math

max_x = 0
result_d = 0
limit = 1000

for d in range(2, limit + 1):
    sqrt_d_int = int(math.isqrt(d))
    if sqrt_d_int * sqrt_d_int == d:
        continue

    m_i = 0
    d_i = 1
    a_0 = int(math.isqrt(d))
    a_i = a_0

    p_km2 = 0
    p_km1 = 1
    q_km2 = 1
    q_km1 = 0

    while True:
        current_p = a_i * p_km1 + p_km2
        current_q = a_i * q_km1 + q_km2

        if current_p * current_p - d * current_q * current_q == 1:
            if current_p > max_x:
                max_x = current_p
                result_d = d
            break

        p_km2 = p_km1
        p_km1 = current_p
        q_km2 = q_km1
        q_km1 = current_q

        m_next = d_i * a_i - m_i
        d_next = (d - m_next * m_next) // d_i
        a_next = (a_0 + m_next) // d_next

        m_i = m_next
        d_i = d_next
        a_i = a_next

print(result_d)
