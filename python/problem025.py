from itertools import islice, dropwhile

def fibonnaci():
	a, b = 1, 1
	while True:
		yield a
		a, b = b, a + b

print(list(islice(dropwhile(lambda x: len(str(x[1])) < 1000, enumerate(fibonnaci(), 1)), 1))[0][0])
