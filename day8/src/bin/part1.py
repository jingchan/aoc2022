lines = [
    "30373",
    "25512",
    "65332",
    "33549",
    "35390",
]

trees = []
for line in lines:
    row = []
    for c in line:
        row.append(int(c))
    trees.append(row)

visible = [[False] * len(row) for row in trees]

h, w = len(trees), len(trees[0])
for i in range(0, h):
    for j in range(0, w):
        if j == 0 or trees[i][j - 1] < trees[i][j]:
            visible[i][j] = True
        else:
            break
for i in range(0, h):
    for j in range(0, w):
        if j == 0 or trees[h - i - 1][w - j - 2] < trees[h - i - 1][w - j - 1]:
            visible[h - i - 1][w - j - 1] = True
        else:
            break
for j in range(0, h):
    for i in range(0, w):
        if i == 0 or trees[i - 1][j] < trees[i][j]:
            visible[i][j] = True
        else:
            break
for j in range(0, w):
    for i in range(0, h):
        if i == 0 or trees[h - i - 2][w - j - 1] < trees[h - i - 1][w - j - 1]:
            visible[h - i - 1][w - j - 1] = True
        else:
            break

count = 0
for i in range(0, h):
    for j in range(0, w):
        if visible[i][j]:
            count += 1
            print("X", end="")
        else:
            print(" ", end="")
    print()

print("Answer:", count)
