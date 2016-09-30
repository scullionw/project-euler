from functools import reduce
from operator import mul

def factorial(n):
	return reduce(mul, range(1, n + 1), 1)

def main():
	print(sum(int(num) for num in str(factorial(100))))

if __name__ == '__main__':
	main()