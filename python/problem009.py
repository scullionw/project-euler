__author__ = 'williamscullion'

# A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
#
# a^2 + b^2 = c^2
# For example, 32^2 + 42^2 = 9 + 16 = 25 = 52.
#
# There exists exactly one Pythagorean triplet for which a + b + c = 1000.
# Find the product abc.

# a ** 2 + b**2 == c ** 2
# a + b + c == 1000

# (a + b - 1000) ** 2 = c ** 2
# (a + b - 1000) ** 2 == a ** 2 + b ** 2
# a**2 + a*b -1000*a + a*b + b**2  - 1000*b -1000*a - 1000*b + 1000**2 == a**2 + b**2
# 2*a*b - 2000*b - 2000*a + 1000000 == 0
#



# for a in range(1, 1001):
#     for b in range(a, 1001):
#         if 2 * a * b - 2000 * b - 2000 * a + 1000000 == 0:
#             ans = a * b * (1000 - a - b)
#             print("Found!")
#             break
#     else:
#         continue
#     break

# print(ans)
print([a * b * (1000 - a - b) for a in range(1, 1001) for b in range(a, 1001) if not 2 * a * b - 2000 * b - 2000 * a + 1000000])
