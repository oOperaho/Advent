file = open("/home/operaho/Advent/D3/adv.txt", "r")
f, l, out, c = file.read().splitlines(), [], [], 0

while len(f) > 1:
	l.clear()
	out.clear()

	if c == 12:
		c = c*0

	for i in f:
		l.append(i[c])

	l0 = l.count("0")
	l1 = l.count("1")

	if l0 > l1:
		for j in f:
			if j[c] == "1": # Change this to find the other binary
				out.append(j)
		f = out[:]
	if l1 > l0:
		for j in f:
			if j[c] == "0": # Change this to find the other binary
				out.append(j)
		f = out[:]
	if l0 == l1:
		for j in f:
			if j[c] == "0": # Change this to find the other binary
				out.append(j)
		f = out[:]

	c = c + 1

print(f)
