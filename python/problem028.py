
def shellsum(size):
	if size == 1:
		return 1
	else:
		return sum([size**2 - i*(size - 1) for i in range(4)])


def spiralsum(maxsize):
	maxindex = (maxsize - 1) // 2
	return sum([shellsum(2*index + 1) for index in range(maxindex + 1)])

def main():
	print(spiralsum(1001))

if __name__ == '__main__':
	main()