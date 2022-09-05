f0 = open("/home/operaho/Code/Advent/D4/adv.txt", "r")
f1 = open("/home/operaho/Code/Advent/D4/instructions.txt", "r")

bingo, inst = f0.read().splitlines(), f1.read().splitlines()

for i in bingo:
	print(i.split(" "))


# snip
