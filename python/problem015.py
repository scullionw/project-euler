# 11111111110000000000 How many ways can we put ones into 40 buckets? 40 choose 20
from math import factorial
def nchoosek(n,k):
	return factorial(n)//(factorial(k) * factorial(n - k))



print(nchoosek(40,20))