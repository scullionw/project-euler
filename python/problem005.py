start_number = 20
up_to = start_number
stop = False

while not stop:

    x = 11

    while x < up_to and (start_number % x == 0):
        x += 1

    if x < (up_to):
        start_number += up_to
    else:
        stop = True


print(start_number)

# make faster FP code!
# y = 20
# while not all(map(lambda x: y % x == 0, range(2, 21))):
# 	y += 20
# print(y)
