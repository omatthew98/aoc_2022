SYSTEM_SIZE = 70_000_000
REQUIRED_SIZE = 30_000_000

class FileNode:
	UNDER_THRESH = 100000

	def __init__(self, name="", size=None, parent_dir=None):
		self.name = name
		self.size = size
		self.children = []
		self.parent_node = parent_dir

	def add_child(self, name, size=None):
		new_node = FileNode(name=name, size=size, parent_dir=self)
		self.children.append(new_node)

	def find_child(self, name):
		return [x for x in self.children if x.name == name][0]

	def calc_size(self):
		if len(self.children) != 0:
			for child in self.children:
				child.calc_size()
			self.size = sum([child.size for child in self.children])
		elif self.size is None:
			self.size = 0

		return self.size

	def find_sum_size_under(self):
		if len(self.children) == 0:
			# case corresponds to file or directory with nothing in it
			return 0
		else:
			curr = self.size if self.size < self.UNDER_THRESH else 0
			below_sum = sum([child.find_sum_size_under() for child in self.children])
			return curr + below_sum

	def find_optimal_dir(self, min_required):
		# want to find smallest thing greater than the min required
		if len(self.children) == 0 or self.size < min_required:
			return SYSTEM_SIZE
		return min([self.size] + [child.find_optimal_dir(min_required) for child in self.children])



file = open("input.txt")

root = FileNode()
curr_dir = root

# build tree
for line in file.readlines():
	line = line.strip()

	if line == "$ cd /":
		curr_dir = root
	elif line == "$ ls":
		# parse results of ls line by line, no action until those lines read
		continue
	elif line == "$ cd ..":
		curr_dir = curr_dir.parent_node if curr_dir.parent_node is not None else curr_dir
	elif line.startswith("$ cd "):
		_, _, name = line.split()
		curr_dir = curr_dir.find_child(name)
	elif line.startswith("dir"):
		_, name = line.split()
		curr_dir.add_child(name)
	else:
		size, name = line.split()
		curr_dir.add_child(name, size=int(size))


root.calc_size()

print("root size:", root.size)
print("sum dirs under thresh:", root.find_sum_size_under())

min_required = REQUIRED_SIZE - (SYSTEM_SIZE - root.size)
print("min_required:", min_required)
print("smallest dir to delete:", root.find_optimal_dir(min_required))
