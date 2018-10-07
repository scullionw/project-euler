from tools import timer

@timer
def compute(bound):
    total = 0
    for i in range(1, bound):
        if i % 3 == 0 or i % 5 == 0:
            total += i
    return total

@timer
def compute_fp(bound):
    return sum(filter(lambda x: x % 3 == 0 or x % 5 == 0, range(1, bound)))

def main():
    print(compute_fn(100000000))

if __name__ == '__main__':
    main()
