def binary(number):
	remainder = True
	binary_list = []
	while number:
		number, remainder =  divmod(number, 2)
		binary_list.append(remainder)
	return ''.join([str(n) for n in reversed(binary_list)])

def palindrome(string):
	return string == string[::-1]

def main():
	print(sum([number for number in range(1000000) if palindrome(str(number)) and palindrome(binary(number))]))

if __name__ == '__main__':
	main()
