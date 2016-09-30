from problem007 import is_prime, sieve_d
from itertools import islice

def truncable_prime(prime):
	num = str(prime)
	l2r = [int(num[n + 1::]) for n in range(len(str(num)) - 1)]
	r2l = [int(num[0:n + 1]) for n in range(len(str(num)) - 1)]
	return all(map(is_prime, l2r)) and all(map(is_prime, r2l))

def main():
	print(sum(islice((prime for prime in sieve_d(1000000) if truncable_prime(prime)), 4, 15)))	

if __name__ == '__main__':
	main()