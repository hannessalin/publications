import math

integer = int(input(": "))
x = int(math.sqrt(integer))
lst = []
for i in range(x):
	lst.append(1)
print("sqrt is ",x)

for i in range(2,x):
	for j in range(i,x):
		if j % i == 0 and j != i:
			lst[j] = 0

print(len(lst))
