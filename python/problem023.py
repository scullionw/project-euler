from problem021 import d

def abundants(upto):
	return [n for n in range(1, upto + 1) if d(n) > n]

def abundant_sum_numbers(upto):
	abundant_nums = abundants(upto)
	sum_numbers = []
	while abundant_nums:
		current_num = abundant_nums.pop()
		sum_numbers.extend([current_num * 2] + [current_num + num for num in abundant_nums])
	return set(sum_numbers)

def main():
	maxn = 28123
	sum_numbers = abundant_sum_numbers(28123)	
	print(sum([number for number in range(1, 28124) if number not in sum_numbers]))

if __name__ == '__main__':
	main()