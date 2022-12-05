import copy

f = open("input", "r")
d = f.read()

setup, moves = d.split("\n\n")
setup_lines = setup.splitlines()

board = [[]]
for col in range(9):
  col_list = []
  for row in range(len(setup_lines) - 1):
    c = setup_lines[row][1+4*col]
    if c != ' ':
      col_list.insert(0, c)
  board.append(col_list)

# board = [
#   [],
#   ['G', 'D', 'V', 'Z', 'J', 'S', 'B'],
#   ['Z', 'S', 'M', 'G', 'V', 'P'],
#   ['C', 'L', 'B', 'S', 'W', 'T', 'Q', 'F'],
#   ['H', 'J', 'G', 'W', 'M', 'R', 'V', 'Q'],
#   ['C', 'L', 'S', 'N', 'F', 'M', 'D'],
#   ['R', 'G', 'C', 'D'],
#   ['H', 'G', 'T', 'R', 'J', 'D', 'S', 'Q'],
#   ['P', 'F', 'V'],
#   ['D', 'R', 'S', 'T', 'J'],
# ]
board2 = copy.deepcopy(board)

for line in moves.splitlines():
  _, count, _, fr, _, to = line.split(" ")
  count, fr, to = int(count), int(fr), int(to)

  # Part 1
  for i in range(count):
    item = board[fr].pop()
    board[to].append(item)

  # Part 2
  items = board2[fr][-count:]
  del board2[fr][-count:]
  board2[to].extend(items)

for i in range(1, len(board)):
  print(board[i][-1], end='')
print()
for i in range(1, len(board2)):
  print(board2[i][-1], end='')
print()
