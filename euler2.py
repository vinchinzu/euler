# coding: utf-8
import fibo

sum = 0

a = fibo.fib2(4000000)
for i in a:
     if i % 2 == 0:
         sum += i

		 
sum

