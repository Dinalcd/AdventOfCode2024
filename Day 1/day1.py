    f = open("input.txt", "r")
    Left = []
    Right = []

    for line in f.readlines():
        distance = line.split('   ')
        distance[-1] = distance[-1].strip()
        Left.append(int(distance[0]))
        Right.append(int(distance[1]))
        
    Left.sort()
    Right.sort()
    sum=0
    multiply=0
    for i in range(0, len(Left)):
        sum+= abs(Left[i]-Right[i])
        multiply+= (Right.count(Left[i])* Left[i])
        
        
    print(sum)
    print(multiply)