from itertools import count, chain, islice
from functools import reduce
from operator import mul

def main():
	numbers = list(islice(chain.from_iterable(map(int, str(n)) for n in count(1)), 1000000))
	print(reduce(mul, [numbers[i] for i in [10**k - 1 for k in range(0,7)]]))

if __name__ == '__main__':
	main()