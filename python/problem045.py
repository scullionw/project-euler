import math
import itertools

def pent_num(pent):
    n = (1 + math.sqrt(1 + 24 * pent)) / 6
    return n == int(n)

def hexa_num(hexa):
    n = (1 + math.sqrt(1 + 8 * hexa)) / 4
    return n == int(n)

def triangles():
    return ((n * (n + 1)) // 2 for n in itertools.count(1))

def main():
    print(list(itertools.islice((tri for tri in triangles() if pent_num(tri) and hexa_num(tri)), 3)))

if __name__ == '__main__':
    main()