from math import sqrt
from itertools import combinations

def pent(pent):
    n = (1 + sqrt(1 + 24 * pent)) / 6
    return n == int(n)

def pents_upto(k):
    return [(n * (3 * n - 1)) // 2 for n in range(1, k + 1)]

def main():
   print(sorted([hi - lo for lo, hi in combinations(pents_upto(2500), 2) if pent(hi - lo) and pent(lo + hi)])[0])

if __name__ == '__main__':
    main()
