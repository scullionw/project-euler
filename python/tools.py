from functools import wraps
from time import process_time

def timer(func):
	@wraps(func)
	def wrapper(*args, **kwargs):
		start = process_time()
		func_ret = func(*args, **kwargs)
		end = process_time()
		seconds = end - start
		milliseconds = seconds * 1000
		print('Runtime: {0:.2f} seconds | {1:.2f} ms'.format(seconds, milliseconds))
		return func_ret
	return wrapper