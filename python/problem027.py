from problem007 import *
from itertools import count

def quadratics(a, b):
	def quadratic(n):
		return n**2 + a*n + b 
	return quadratic

def max_consecutive_primes(quadratic, primes):
	integer = count()
	n = next(integer)
	prime_length = 0
	x = quadratic(n)
	while x in primes:
		prime_length += 1
		n = next(integer)
		x = quadratic(n)
	return prime_length

def main():
	primes = list(sieve_d(200000)) # 149002 doesnt work lol?!
	n = 1000
	a_s = [a for a in range(-n + 1, n) if not a % 2 == 0]
	b_s = [b for b in range(-n, n + 1) if b in primes]
	print(len(a_s), len(b_s))
	coefficients = [(a, b, a * b) for a in a_s for b in b_s]
	max_prime_length = 0
	max_product = 0
	for coef in coefficients:
		prime_length = max_consecutive_primes(quadratics(coef[0], coef[1]), primes)
		if prime_length > max_prime_length:
			max_prime_length = prime_length
			final_coefs = coef
	print(max_prime_length, final_coefs)

if __name__ == '__main__':
	main()