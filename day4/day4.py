f = open("input", "r")
d = f.read()

overlap = 0
overlap2 = 0
for line in d.splitlines():
  parts = line.split(",")
  range1 = list(map(lambda x: int(x), parts[0].split("-")))
  range2 = list(map(lambda x: int(x), parts[1].split("-")))

  if range1[0] >= range2[0] and range1[1] <= range2[1]:
    overlap += 1
  elif range2[0] >= range1[0] and range2[1] <= range1[1]:
    overlap += 1

  if range1[0] >= range2[0] and range1[0] <= range2[1]:
    overlap2 += 1
  elif range2[0] >= range1[0] and range2[0] <= range1[1]:
    overlap2 += 1
  elif range1[1] >= range2[0] and range1[1] <= range2[1]:
    overlap2 += 1
  elif range2[1] >= range1[0] and range2[1] <= range1[1]:
    overlap2 += 1

print(overlap)
print(overlap2)
