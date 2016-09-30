from functools import lru_cache
from math import ceil, sqrt

# fast 0.1s (100x faster)
@lru_cache(maxsize=None)
def d(n):
	return sum(proper_divisors_will(n))

def proper_divisors_will(n):
	if n < 1:
		return []
	sqrt_n = sqrt(n)
	lowers = [divisor for divisor in range(1, ceil(sqrt_n)) if n % divisor == 0]
	biggers = [n // divisor for divisor in lowers[1:]]
	if sqrt_n == int(sqrt_n):
		return lowers + [int(sqrt_n)] + biggers
	else:
		return lowers + biggers

def divisors_will(n):
	return proper_divisors_will(n) + [n]


def main():
	print(sum([n for dn, n in ((d(n), n) for n in range(1,10001)) if n == d(dn) and n != dn]))

if __name__ == '__main__':
	main()
# simple 10s
# def d(n):
# 	return sum([divisor for divisor in range(1, n) if n % divisor == 0])

# print(sum([n for n in range(1,10001)) if n == d(d(n)) and n != d(n)]))



