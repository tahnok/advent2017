string = input()

total = 0

for index, char in enumerate(string):
    if char == string[(index + 1) % len(string)]:
        total = total + int(char)

print(total)
