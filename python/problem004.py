# number = 0
# z = 0
# x = 0

# def is_palindrome(n):
#     number_string = str(n)
#     if str(n) == str(n)[::-1]:
#         return True
#     else:
#         return False


# for i in range(100, 1000):
#     for j in range(i, 1000):
#         if i * j > number and is_palindrome(i * j):
#             number = i * j
#             z = i
#             x = j

nums = max([(i * j, i, j) for i in range(100,1000) for j in range(i,1000) if str(i*j) == str(i*j)[::-1]], key=lambda item:item[0])

# print(number, z, x)
print(nums)
