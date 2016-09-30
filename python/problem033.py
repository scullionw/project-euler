from operator import mul
from functools import reduce
from problem021 import divisors_will as divisors
from tools import timer

def curious(a, b):
	if a % 10 == 0 or b % 10 == 0: # trivial case
		return False
	aprime = ''.join(digit for digit in str(a) if digit not in str(b))
	bprime = ''.join(digit for digit in str(b) if digit not in str(a))
	if aprime and bprime: # make sure we don't divide by zero
		aprime, bprime = map(int, [aprime, bprime])
	else:
		return False
	return (aprime/bprime) == a/b and aprime != a and bprime != b

def gcd_will(a, b):
	return max(set(divisors(a)) & set(divisors(b)))

@timer
def main():
	curious_fractions = [(a, b) for a in range(10,100) for b in range(a, 100) if curious(a, b)]
	numerator, denominator = [reduce(mul, numbers, 1) for numbers in zip(*curious_fractions)]
	print(denominator // gcd_will(numerator,denominator))

if __name__ == '__main__':
	main()