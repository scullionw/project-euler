print(sum(number for number in range(2,6*9**5) if number == sum(int(n) ** 5 for n in str(number))))
