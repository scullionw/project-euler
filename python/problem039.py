# for a given p, the maximum c is when one of the legs (a,b) is almost equal to c (other leg aproaches 0).
# a ~ c, b ~ 0, so c + c + 0 = p
# p ~ c / 2

# for a given p, the minimum c is when both legs (a,b) are equal:
# 2a^2 = c^2
# sqrt(2)a = c
# c/sqrt(2) + c/sqrt(2) + c = p
# c = p / 1+sqrt(2))

# the maximum value of the small side before they reverse is when they are equal
# so we only have to test until a = c / sqrt(2): 

from math import sqrt, ceil, floor

def right_triangle(a, b, c):
	return a**2 + b**2 == c**2

def main():
	pmin = 12 # smallest integer triangle is {3, 4, 5}
	pmax = 1000
	solutions = {p:[] for p in range(pmin, pmax + 1)}
	for p in range(pmin, pmax + 1):
		max_c = floor(p / 2) 
		min_c = ceil(p / (1 + sqrt(2)))
		for c in range(min_c, max_c + 1):
			for a in range(1, floor(c / sqrt(2)) + 1):
				b = p - c - a
				if right_triangle(a, b, c):
					solutions[p].append((a, b, c))
					
	print(max(solutions, key=lambda item: len(solutions[item])))

if __name__ == '__main__':
	main()
