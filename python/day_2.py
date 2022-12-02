char_map = {
	"A": 0, "B": 1, "C": 2,
	"X": 0, "Y": 1, "Z": 2
}

def out(r, s):
	return (s - r + 1) % 3 * 3 + s + 1

lines = [[char_map[c] for c in game.split()] for game in open("input.txt").read().split("\n") if len(game) > 0]


print(sum([out(*game) for game in lines]))