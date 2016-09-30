from math import sqrt

original_number = 600851475143
number = original_number
factor = 3
maxfactor = sqrt(number)
largest_prime_factor = 0

while number > 1 and factor <= maxfactor:
    if number % factor == 0 :
        largest_prime_factor = factor
        while number % factor == 0:
        	number //= factor

    factor += 2


print(largest_prime_factor, number)