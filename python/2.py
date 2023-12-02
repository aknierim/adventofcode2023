def flatten(ls):
    for item in ls:
        try:
            yield from flatten(item)
        except TypeError:
            yield item


def day_2(input):
    return sum([([int(id) for id in lines.split(": ")[0].split() if id.isdigit()], [1 if 0 not in flatten([[0 if int(cont.split()[0]) >  {'red': 12, 'green': 13, 'blue': 14}.get(cont.split()[1], 0) else 1] for cont in res.split(", ")] for res in lines.split(": ")[1].split("; ")) else 0])[0][0] for lines in input.splitlines() if ([int(id) for id in lines.split(": ")[0].split() if id.isdigit()], [1 if 0 not in flatten([[0 if int(cont.split()[0]) >  {'red': 12, 'green': 13, 'blue': 14}.get(cont.split()[1], 0) else 1] for cont in res.split(", ")] for res in lines.split(": ")[1].split("; ")) else 0])[1][0] == 1])


def main():
    with open("input.txt", "r") as f:
        input = f.read()
    print(day_2(input))


if __name__ == "__main__":
    main()
