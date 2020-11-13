import sys

data = []
for line in sys.stdin:
    data.append(line.split())
amount = int(data[0][0])


total_min_temp = 0
total_max_temp = 0
total_min_flow = 0
total_max_flow = 0

faucets = int(data[0][0])

for i in range(1, faucets+1):
    min_flow = int(data[i][1])
    max_flow = int(data[i][2])
    temp = int(data[i][0])
    total_min_flow = total_min_flow + min_flow
    total_max_flow = total_max_flow + max_flow
    total_max_temp = total_max_temp + temp*max_flow
    total_min_temp = total_min_temp + temp*min_flow

weighted_min_temp = (total_min_temp) / total_min_flow
weighted_max_temp = (total_max_temp) / total_max_flow


recipies = int(data[faucets+1][0])


for i in range(faucets+2, recipies+faucets+2):
    temp = int(data[i][0])
    flow = int(data[i][1])
    if(flow > total_max_flow or flow < total_min_flow):
        print("no")
        continue
    
    min_temp = 0
    max_temp = 0
    min_flow = 0
    max_flow = 0
    for j in range(1, faucets+1):
        f_temp = int(data[i][0])
        f_min_flow = int(data[j][1])
        f_max_flow = int(data[j][2])
        if (f_temp > temp):
            print(1)
            min_flow = min_flow + f_min_flow
            max_flow = max_flow + f_max_flow
            min_temp = min_temp + f_min_flow * f_temp
            max_temp = max_temp + f_max_flow * f_temp
            continue
        print(f_temp)
        print(temp)
        if (f_temp < temp):
            print(1)
            min_flow = max_flow + f_max_flow
            max_flow = min_flow + f_min_flow
            min_temp = min_temp + f_max_flow * f_temp
            max_temp = max_temp + f_min_flow * f_temp
            continue
        min_flow = min_flow + f_max_flow
        max_flow = max_flow + f_max_flow
        min_temp = min_temp + f_max_flow * f_temp
        max_temp = max_temp + f_max_flow * f_temp
    if(min_temp/min_flow <= temp and max_temp/max_flow >= temp):
        print("yes")
    else:
        print("no")


