def pandigital(number, through):
	if number is None:
		return False
	if len(str(number)) != through:
		return False
	return set(map(int,str(number))) == set(n for n in range(1, through + 1))

def makenum(num_list):
	return int(''.join([str(n) for n in num_list]))

def main():
	n2_3 = [a * b for a in range(1, 10) for b in range(1000, 10000) if pandigital(makenum([a,b,a*b]), 9)]
	n3_2 = [a * b for a in range(10, 100) for b in range(100, 999) if pandigital(makenum([a,b,a*b]), 9)]
	print(sum(set(n3_2 + n2_3)))

if __name__ == '__main__':
	main()