from problem007 import sieve_d, is_prime

def all_rotations(number):
	num_str = str(number)
	expanded_number = (num_str*2)[0:-1]
	len_num = len(num_str)
	return [int(expanded_number[i:i+len_num]) for i in range(len_num)] # list of rotated number

def circular_prime(n):
	return all(map(is_prime, all_rotations(n)))

def main():
	print(len([prime for prime in sieve_d(1000000) if circular_prime(prime)]))

if __name__ == '__main__':
	main()

