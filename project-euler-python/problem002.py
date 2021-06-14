current = 2
last = 1
sum = 2
limit = 4000000
next = 0

while next <= limit:
	next = last+current
	if next % 2 == 0:
		sum = sum + next
	last = current
	current = next
	print(next)
print("sum of even terms: ",sum)
