from problem007 import sieve_d, is_prime
from problem032 import pandigital, makenum
from itertools import permutations

def main():
	print(max(filter(is_prime, pandigitals(7))))

def pandigitals(digits):
	list_of_pandigital_lists = permutations([n for n in range(1, digits + 1)])
	return [makenum([str(el) for el in n]) for n in list_of_pandigital_lists]



if __name__ == '__main__':
	main()