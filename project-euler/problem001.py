sum = 0
integers = int(input(": "))
for i in range(integers+1):
	if i % 3 == 0:
		sum = sum + i
	elif i % 5 == 0:
		sum = sum + i
print(sum)
