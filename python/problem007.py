import itertools
from tools import timer
from math import sqrt

def sieve_gen(n): # will
    candidates = [[True, x] for x in range(2, n + 1)]
    for index, candidate in enumerate(candidates):
        if candidate[0]:
            yield candidate[1]
            mark_from = candidate[1] ** 2
            if mark_from > n:
                for possible_prime in candidates[index + 1:]:
                    if possible_prime[0]:
                        yield possible_prime[1]
                break #return [prime[1] for prime in candidates if prime[0]]
            mark_indexes = [marker_index - 2 for marker_index in range(mark_from, n + 1, candidate[1])]
            for marker_index in mark_indexes:
                candidates[marker_index][0] = False
    # return [prime[1] for prime in candidates if prime[0]]
# generate list of only odd numbers to save half the memory

def sieve(n): # will
    candidates = [[True, x] for x in range(2, n + 1)]
    for candidate in candidates:
        if candidate[0]:
            mark_from = candidate[1] ** 2
            if mark_from > n:
                return [prime[1] for prime in candidates if prime[0]]
            mark_indexes = [marker_index - 2 for marker_index in range(mark_from, n + 1, candidate[1])]
            for marker_index in mark_indexes:
                candidates[marker_index][0] = False
# fastest
# returns bogus primes for n over 4096........len(sieve_d(n)) changes drastically for n over 4096 but not 6000! WTF FIXED: Dictionaries arent sorted!

def sieve_d(n): # will
    candidates = {x:True for x in range(3, n + 1, 2)} # generate all candidates up to n, skipping even numbers
    sorted_candidates = [x for x in range(3, n + 1, 2)]
    for candidate in sorted_candidates:
        if candidates[candidate]: # if candidate is still a candidate
            mark_from = candidate ** 2 # mark from the square of the candidate, multiples under are marked already
            if mark_from > n: # if the mark is bigger than our biggest candidate, the search is over, return all candidates left
                return [2] + [prime for prime in sorted_candidates if candidates[prime]] # also return 2, the only even prime
            for composites in range(mark_from, n + 1, candidate):
                if not composites % 2 == 0: # if number is not even, because evens aren't in the dictionary to begin with
                    candidates[composites] = False

def is_prime(n):
    if n == 1:
        return False
    if n < 4:
        return True
    if n % 2 == 0 or n % 3 == 0:
        return False
    number = n
    divisor = 3
    while number > 1 and divisor < sqrt(n):
        while number % divisor == 0:
            number //= divisor
        divisor += 2
    return number == n


def eratosthenes(): # from web (infinite)
    '''Yields the sequence of prime numbers via the Sieve of Eratosthenes.'''
    D = {  }  # map each composite integer to its first-found prime factor
    for q in itertools.count(2):     # q gets 2, 3, 4, 5, ... ad infinitum
        p = D.pop(q, None)
        if p is None:
            # q not a key in D, so q is prime, therefore, yield it
            yield q
            # mark q squared as not-prime (with q as first-found prime factor)
            D[q*q] = q
        else:
            # let x <- smallest (N*p)+q which wasn't yet known to be composite
            # we just learned x is composite, with p first-found prime factor,
            # since p is the first-found prime factor of q -- find and mark it
            x = p + q
            while x in D:
                x += p
            D[x] = p

def main():
    print(list(itertools.islice(sieve_d(1000000),10001))[-1])
if __name__ == '__main__':
    main()


