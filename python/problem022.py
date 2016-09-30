with open('problem022_data.txt') as f:
	names = sorted([name.strip('"') for name in f.read().split(',')])

def alphapos(letter):
	return ord(letter.lower()) - 96

print(sum([sum([alphapos(letter) for letter in name]) * (index + 1) for index, name in enumerate(names)]))