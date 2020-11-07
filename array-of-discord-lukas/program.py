import sys

data = []
for line in sys.stdin:
    data.append(line.split())
amount = int(data[0][0])


newSet = data[1]
sorted = True

for i in range(0, amount):
    current = current = int(newSet[i])
    if(i != 0 and sorted):
        prev = int(newSet[i-1])
        prev_integers = list(str(prev))
        current_integers = list(str(current))
        if(int(prev)>0 and (int(current_integers[0]) == 1 and all(int(current_integers[i]) == 0 for i in range(1, len(current_integers))) or len(current_integers)==1)):
            sorted = False
            current_integers = ['0']
            newSet[i] = ("".join(current_integers))
            continue
        if (len(prev_integers) == len(current_integers)):
            for j in range (0, len(prev_integers)):
                if (int(prev_integers[j]) < 9):

                    temp_prev = prev_integers.copy()
                    temp_prev[j] = str(9)
                    if (int("".join(temp_prev)) > int("".join(current_integers))):
                        newSet[i-1] = "".join(temp_prev)
                        sorted = False
                        break
                if (int(current_integers[j]) > 0):
                    temp_curr = current_integers.copy()
                    temp_curr[j] = "0" if j > 0 else "1"
                    if (int("".join(prev_integers)) > int("".join(temp_curr))):
                        newSet[i] = ("".join(temp_curr))
                        sorted = False
                        break
        if(not sorted):
            continue
    newSet[i] = (current)

if(sorted):
    print("impossible")
else:
    desired_array = [int(numeric_string) for numeric_string in newSet]
    print(*desired_array)