from problem032 import pandigital, makenum

def conpro(integer, size=9):
	through = 2
	concatenated_product = []
	while len(concatenated_product) < size:
		concatenated_product = ''.join([str(integer * n) for n in range(1, through + 1)])
		through += 1
	if len(concatenated_product) == size:
		return concatenated_product
	else:
		return None

def main():
	print(max([(integer, conpro(integer)) for integer in range(10000) if pandigital(conpro(integer), 9)], key=lambda item: item[1]))

if __name__ == '__main__':
	main()
