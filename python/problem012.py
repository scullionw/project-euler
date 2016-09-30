import itertools
from functools import reduce
from operator import mul
from problem007 import eratosthenes

def triangle_numbers():
	num_list = []
	for natural in itertools.count(start=1):
		num_list.append(natural)
		yield sum(num_list)

def triangle_numbers2():
	n = 1
	while True:
		yield ((n+1)*n)//2
		n += 1

# using tau divisors formula where n_divisors = d(N) = (a+1)(b+1)...for N= p^a*q^b, where p and q are prime factorizations.
def num_divisors(number):
	divisors = {}
	current_divisor = 2
	while number > 1:
		times = 0
		while number % current_divisor == 0:
			times += 1
			number //= current_divisor 
		divisors[current_divisor] = times
		current_divisor += 1
	return reduce(mul, [x + 1 for x in divisors.values()], 1)

def num_divisors2(number):
	divisors = {}
	primes = eratosthenes()
	current_divisor = next(primes)
	while number > 1:
		times = 0
		while number % current_divisor == 0:
			times += 1
			number //= current_divisor 
		divisors[current_divisor] = times
		current_divisor = next(primes)
	return reduce(mul, [x + 1 for x in divisors.values()], 1)

print(next(itertools.islice((x for x in triangle_numbers() if num_divisors(x) > 500), 1)))

