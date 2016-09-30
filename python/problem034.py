from math import factorial

def digit_factorial(n):
    return sum(factorial(int(digit)) for digit in str(n)) == n

def main():
    ans =  sum(n for n in range(3, 7 * factorial(9)) if digit_factorial(n))
    print(ans)
    
if __name__ == '__main__':
    main()
