def ways(total, coins):
	if total in [0, 1] or len(coins) == 1:
		return 1
	else:
		coins_left = coins[:-1]
		coin = coins[-1]
		return sum(ways(total - (coin * x), coins_left) for x in range(1 + total // coin))

def main():
	coins = [1, 2, 5, 10, 20, 50, 100, 200]
	print(ways(200, coins))

if __name__ == '__main__':
	main()