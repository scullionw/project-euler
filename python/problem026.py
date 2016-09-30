from problem007 import sieve_d

def cyclesize(number):
	rems = []
	div = 10
	cycle_length = 0
	rem = None
	while divmod(div, number)[1] not in rems: # as soon as a remainder repeats, the same quotients will appear again
		div, rem = divmod(div, number)
		rems.append(rem)
		cycle_length += 1
		div = 10 * rem
	return cycle_length


def main():
	primes = sieve_d(1000)
	maxcycle = max([(prime, cyclesize(prime)) for prime in primes], key=lambda item:item[1])
	print(maxcycle)
	
if __name__ == '__main__':
	main()


# generator of quotients for 1/number
# def array_division(number):
# 	div = 10
# 	while div:
# 		div, rem = divmod(div, number)
# 		yield div
# 		div = 10 * rem

