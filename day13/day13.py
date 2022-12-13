import functools

f = open("input", "r")
d = f.read()

# d = "[1,1,3,1,1]\n[1,1,5,1,1]\n\n[[1],[2,3,4]]\n[[1],4]\n\n[9]\n[[8,7,6]]\n\n[[4,4],4,4]\n[[4,4],4,4,4]\n\n[7,7,7,7]\n[7,7,7]\n\n[]\n[3]\n\n[[[]]]\n[[]]\n\n[1,[2,[3,[4,[5,6,7]]]],8,9]\n[1,[2,[3,[4,[5,6,0]]]],8,9]\n"

def compare(lhs_item, rhs_item):
    if type(lhs_item) == int and type(rhs_item) == int:
        if lhs_item < rhs_item:
            return 1
        elif lhs_item == rhs_item:
            return 0
        else:
            return -1
    elif type(lhs_item) == list and type(rhs_item) == list:
        i = 0
        while i < min(len(lhs_item), len(rhs_item)):
            new_lhs_item = lhs_item[i]
            new_rhs_item = rhs_item[i]
            res = compare(new_lhs_item, new_rhs_item)
            if res != 0:
                return res
            i += 1
        if len(lhs_item) == i and len(rhs_item) > i:
            return 1
        elif len(lhs_item) == i and len(rhs_item) == i:
            return 0
        else:
            return -1
    else:
        if type(lhs_item) == list:
            rhs_item = [rhs_item]
        else:
            lhs_item = [lhs_item]
        return compare(lhs_item, rhs_item)

result = 0
for i, pair in enumerate(d.split("\n\n")):
    lhs_str, rhs_str = pair.splitlines()

    lhs = eval(lhs_str)
    rhs = eval(rhs_str)

    if compare(lhs, rhs) == 1:
        result += (i+1)

print(result)

messages = []
for packet in d.splitlines():
    if len(packet) == 0:
        continue
    messages.append(eval(packet))
messages.append([[2]])
messages.append([[6]])

def bubble_sort(arr):
    n = len(arr)
    swapped = False
    for i in range(n-1):
        for j in range(0, n-i-1):
            if compare(arr[j], arr[j + 1]) == -1:
                swapped = True
                arr[j], arr[j + 1] = arr[j + 1], arr[j]
        if not swapped:
            return

bubble_sort(messages)

print((messages.index([[2]])+1)*(messages.index([[6]])+1))
