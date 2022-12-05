f = open("input", "r")
d = f.read()

def prio(x):
  if ord(x) >= ord('a') and ord(x) <= ord('z'):
    return ord(x) - ord('a') + 1
  else:
    return ord(x) - ord('A') + 27

score = 0
for line in d.splitlines():
  n = len(line)
  a = line[:n//2]
  b = line[n//2:]

  for c in a:
    i = b.find(c)
    if i != -1:
      score += prio(c)
      break

print(score)

score = 0
lines = d.splitlines()
for i in range(0, len(lines), 3):
  l1 = lines[i+0]
  l2 = lines[i+1]
  l3 = lines[i+2]

  for c in l1:
    f2 = l2.find(c)
    f3 = l3.find(c)
    if f2 != -1 and f3 != -1:
      score += prio(c)
      break

print(score)
