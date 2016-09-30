with open('problem013_data.txt') as f:
	data = f.read().splitlines()

y = 0
length = 1
while len(str(y)) < 13:
	y = sum([int(number[0:length]) for number in data])
	length += 1
print(y)