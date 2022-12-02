rint(sum(sorted([sum([int(cal) for cal in elf.split("\n")]) for elf in open("input.txt").read().split("\n\n") if len(elf) > 0])[-3:]))
