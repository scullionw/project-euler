from problem041 import pandigitals

def substrings(number, start, end, size):
	str_number = str(number)
	return [int(''.join(str_number[n:n + size])) for n in range(start, end + 1)]


def main():
	print(sum([n for n in pandigitals(0, 9) if all([sub % prime == 0 for sub, prime in zip(substrings(n, 1, 7, 3), [2, 3, 5, 7, 11, 13, 17])])]))


if __name__ == '__main__':
	main()