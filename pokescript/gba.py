from pyboy import PyBoy

class Game:
	def __init__(self):
		return

	def start():
		pyboy = PyBoy('ROMs/gamerom.gb')
		while not pyboy.tick():
			pass
