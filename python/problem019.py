import itertools

def months(starting_month='Jan'):
	month_names = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec']
	index = month_names.index(starting_month)
	rotated_month_names = month_names[index:] + month_names[0:index]
	yield from itertools.cycle(rotated_month_names)

def days(starting_day='Mon'):
	day_names = ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun']
	index = day_names.index(starting_day)
	rotated_day_names = day_names[index:] + day_names[0:index]
	yield from itertools.cycle(rotated_day_names)

def years(starting_year=1900):
	yield from (starting_year + n for n in itertools.count())

def dates(starting_date=1, month='Jan', year=1900):
	big_months = ['Jan', 'Mar', 'May', 'Jul', 'Aug', 'Oct', 'Dec']
	small_months = ['Apr', 'Jun', 'Sep', 'Nov']
	if month in big_months:
		maxdays = 31
	elif month in small_months:
		maxdays = 30
	else: # Feb
		if (year % 4 == 0) and (year % 400 == 0 or not year % 100 == 0):
			maxdays = 29
		else:
			maxdays = 28
	date_numbers = list(range(1, maxdays + 1))
	index = date_numbers.index(starting_date)
	rotated_date_numbers = date_numbers[index:] + date_numbers[0:index]
	yield from (date for date in rotated_date_numbers)

def time(n_days):
	current_day = 'Mon'
	current_date = 1
	current_month = 'Jan'
	current_year = 1900
	day = days(current_day)
	date = dates(current_date, current_month, current_year)
	month = months(current_month)
	year = years(current_year)
	current_day = next(day)
	current_date = next(date)
	current_month = next(month)
	current_year = next(year)
	time_sequence = []
	for day_n in range(n_days + 1):
		time_sequence.append((current_day, current_date, current_month, current_year))
		current_day = next(day)
		try:
			current_date = next(date)
		except StopIteration:
			current_month = next(month)
			date = dates(1 , current_month, current_year)
			current_date = next(date)
			if current_month == 'Jan' and current_date == 1:
				current_year = next(year)
	return time_sequence

period = time(36889)

century = [day for day in period if day[3] >= 1901 and day[3] <= 2000]
print(len([day for day in century if day[0] == 'Sun' and day[1] == 1]))
