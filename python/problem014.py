def collatz(n):
	yield n
	while n > 1:
		if n % 2 == 0:
			n //= 2
		else:
			n = 5*n + 1
		print(n)
		yield n

def longest_collatz_fp(upto):
	return max([(len(list(collatz(n))), n) for n in range(1, upto + 1)], key=lambda item:item[0])


def longest_collatz(n):
	cache = {}
	for n in range(1, n + 1):
		i = n
		length = 0
		while i not in cache:
			if i % 2 == 0:
				i //= 2
			else:
				i = 3*i + 1
			length += 1
			if i == 1:
				cache[n] = length
		cache[n] = length + cache[i]

	return max(cache, key=lambda k: cache[k])

# print(longest_collatz(1000000))
print(list(collatz(52)))
