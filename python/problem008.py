from functools import reduce
data = "".join(open("problem008_data.txt").read().splitlines())

largest_product = 0

for count in range(len(data)-13):  # -13 or ?

    thirteen_product = 1
    for i in range(13):
        thirteen_product *= int(data[count+i])
    if thirteen_product > largest_product:
        largest_product = thirteen_product

print(largest_product)

# FP
print(max([reduce(lambda x, y: int(x) * int(y), data[i:i+13]) for i in range(len(data) - 13)]))
