# add numbers from bottom for p67, eg. 63 + max(04, 62)

with open('problem018_data.txt') as f:
	triangle = tuple([tuple(map(int,line.split())) for line in f.read().splitlines()])
size = len(triangle)

def mult_adj(row, column):
	if row == size - 1:
		return triangle[row][column]
	else:
		return triangle[row][column] + max(mult_adj(row + 1, column), mult_adj(row + 1, column + 1))
	
print(mult_adj(0,0))









