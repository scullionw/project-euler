#!/usr/bin/env python3
# coding: utf-8

import re
import sys

NUM2LETTER = {
	1:'one',
	2:'two',
	3:'three',
	4:'four',
	5:'five',
	6:'six',
	7:'seven',
	8:'eight',
	9:'nine',
	10:'ten',
	11:'eleven',
	12:'twelve',
	13:'thirteen',
	14:'fourteen',
	15:'fifteen',
	16:'sixteen',
	17:'seventeen',
	18:'eighteen',
	19:'nineteen',
	20:'twenty',
	30:'thirty',
	40:'forty',
	50:'fifty',
	60:'sixty',
	70:'seventy',
	80:'eighty',
	90:'ninety',
}

def word_from_number(number):
	"""Returns the written form of numbers between 1 and 1000"""
	str_num = str(number)
	len_num = len(str(number))
	
	if len_num == 1:
		return NUM2LETTER[number]
	elif len_num == 2:
		if number in NUM2LETTER:
			return NUM2LETTER[number]
		else:
			tens = NUM2LETTER[int(str_num[0]+'0')]
			ones = NUM2LETTER[int(str_num[1])]
			return tens + '-' + ones
	elif len_num == 3:
		hundreds = NUM2LETTER[int(str_num[0])] + ' hundred'
		if int(str_num[1:3]) == 0:
			return hundreds
		else:
			return hundreds + ' and ' + word_from_number(int(str_num[1:3]))

	elif len_num == 4:
		return 'one thousand'

def letter_counter(word_string):
	return len(re.sub('[-\s]','',word_string))

def total_letters(upto):
	list_of_numbers = [word_from_number(number) for number in range(1,upto + 1)]
	list_of_num_letters = map(letter_counter, list_of_numbers)
	return sum(list_of_num_letters)

def main():

	print(total_letters(1000))

if __name__ == '__main__':
	sys.exit(main())



