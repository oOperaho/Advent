f0 = open("/home/operaho/Code/Advent/D4/adv.txt", "r")
f1 = open("/home/operaho/Code/Advent/D4/instructions.txt", "r")

bingo, inst, l, fnl, c0 = f0.read().splitlines(), f1.read(), [], [], 0

for i in bingo:
	if i != '':
		l.append(i)


for j in range(0, len(l)):
	
	
	c0 = c0 + 5
