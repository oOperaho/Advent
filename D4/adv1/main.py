f0 = open("/home/operaho/Code/Advent/D4/adv.txt", "r")
f1 = open("/home/operaho/Code/Advent/D4/instructions.txt", "r")

bingo, inst, l = f0.read().splitlines(), f1.read(), []

for i in bingo:
	if i != '':
		l.append(i)



