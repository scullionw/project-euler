from itertools import takewhile
# functional
def fibonnaci():
	a, b = 0, 1
	while True:
		yield a
		a, b = b, a + b

print(sum(filter(lambda x: x % 2 == 0, takewhile(lambda x: x < 4000000, fibonnaci()))))

# # iteration
# current_sum = 0
# a = 0
# b = 1
# ---0xFF
# while b <= 4000000:
# 	a, b = b, a + b
# 	if b % 2 == 0:
# 		current_sum += b
# print(current_sum)
