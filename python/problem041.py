from problem007 import sieve_d, is_prime
from problem032 import pandigital, makenum
from itertools import permutations

def main():
	# 8 and 9 pandigitals can't be prime because they can
	# all be divided by 3 since the sum of their digits can
	# be divided by 3.
	print(max(filter(is_prime, pandigitals(1, 7))))

def pandigitals(start, end):
	list_of_pandigital_lists = permutations([n for n in range(start, end + 1)])
	return [makenum([str(el) for el in n]) for n in list_of_pandigital_lists]



if __name__ == '__main__':
	main()