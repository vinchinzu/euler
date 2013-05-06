#problem 3 project euler 
#Exercise 3 description:
#The prime factors of 13195 are 5, 7, 13 and 29.
#What is the largest prime factor of the number 600851475143 

n = 600851475143

def isprime(n)
n = (2..(n-1)).each {|x| return false if n%x ==0}
true
end

a=[]
ps =1
x=2

while ps < n
if n%x ==0 && isprime(x)
a<<x
ps *=x
end
x+=1
end

puts a.last
 

