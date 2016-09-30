grid = open("problem011_data.txt").read().strip().split()
#grid = [i.lstrip("0") for i in grid]

#for i in range(len(grid)):
#   if grid[i] == "":
 #       grid[i] = "0"

grid = [[grid[y + x * 20] for y in range(20)] for x in range(20)]

largest = 0

# horizontal
for i in range(20):
    for j in range(17):
        product = 1
        for x in range(4):
            product *= int(grid[i][j + x])
        if product > largest:
            largest = product

# vertical
for i in range(17):
    for j in range(20):
        product = 1
        for x in range(4):
            product *= int(grid[i + x][j])
        if product > largest:
            largest = product

# diagonal right-down
for i in range(17):
    for j in range(17):
        product = 1
        for x in range(4):
            product *= int(grid[i + x][j + x])
        if product > largest:
            largest = product

# diagonal right-up
for i in range(3, 20):
    for j in range(17):
        product = 1
        for x in range(4):
            product *= int(grid[i - x][j + x])
        if product > largest:
            largest = product

print(largest)