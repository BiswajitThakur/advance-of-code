def read_file(path):
    f = open(path, "r")
    value = f.read()
    f.close()
    return value

def parse(value):
    arr = []
    for line in value.splitlines():
        line = line.split()
        arr.append((int(line[0]), int(line[1])))
    return arr

def filter(list):
    left = []
    right = []
    for item in list:
        left.append(item[0])
        right.append(item[1])
    left.sort()
    right.sort()
    return (left, right)

def calculate_distance(list):
    left = list[0]
    right = list[1]
    total = 0
    for i in range(left.__len__()):
        total += abs(left[i] - right[i])
    return total

def main():
    f = read_file("input.txt")
    total = calculate_distance(filter(parse(f)))
    print(total)

if __name__ == "__main__":
    main()
