from itertools import takewhile, count

with open('problem042_data.txt') as f:
	words = f.read().strip("\"").split("\",\"")
	
def triangle(n):
	return n * (n + 1) // 2

def count_word(word):
	return sum([letter_pos(letter) for letter in word])

def letter_pos(letter):
	return ord(letter) - 64

def main():
	largest = 26 * len(max(words, key=len))
	triangles = list(takewhile(lambda x: x <= largest, (triangle(n) for n in count(1))))
	print(len([word for word in words if count_word(word) in triangles]))

if __name__ == '__main__':
	main()