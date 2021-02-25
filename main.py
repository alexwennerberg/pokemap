from pyboy import PyBoy
import random
from pyboy.utils import WindowEvent
import memory as m

pyboy = PyBoy('roms/pokemon-blue.gb')
pyboy.set_emulation_speed(0)


def get_player_coordinates():
	return pyboy.get_memory_value(0xD361), pyboy.get_memory_value(0xD362)


def run_frames(count):
	for _ in range(count):
		pyboy.tick()

def walk(direction):
	if direction == "left":
		keydown = WindowEvent.PRESS_ARROW_LEFT
		keyup = WindowEvent.RELEASE_ARROW_LEFT
	if direction == "down":
		keydown = WindowEvent.PRESS_ARROW_DOWN
		keyup = WindowEvent.RELEASE_ARROW_DOWN
	if direction == "right":
		keydown = WindowEvent.PRESS_ARROW_RIGHT
		keyup = WindowEvent.RELEASE_ARROW_RIGHT
	if direction == "up":
		keydown = WindowEvent.PRESS_ARROW_UP
		keyup = WindowEvent.RELEASE_ARROW_UP
	pyboy.send_input(keydown)
	run_frames(9)
	pyboy.send_input(keyup)
	run_frames(12)


class Walker:
	def __init__(self):
		self.walk_plan = []
	def step(self):
		if len(self.walk_plan) > 0:
			walk(self.walk_plan.pop())
		else:
			for i in [random.choice(["down", "left", "up", "right"])] * random.randint(1,10):
				self.walk_plan.append(i)

def navigate_generic_menu():
	while 1:
		pyboy.send_input(WindowEvent.PRESS_BUTTON_A)
		run_frames(20)
		pyboy.send_input(WindowEvent.RELEASE_BUTTON_A)
		run_frames(20)
		if not pyboy.get_memory_value(0xCFC4): 
			return

def navigate_start_screen():
	for _ in range(1900):
		pyboy.send_input(WindowEvent.PRESS_BUTTON_A)
		pyboy.tick()
		pyboy.send_input(WindowEvent.RELEASE_BUTTON_A)
		pyboy.tick()

def run_battle():
	while 1:
		pyboy.send_input(WindowEvent.PRESS_BUTTON_A)
		run_frames(20)
		pyboy.send_input(WindowEvent.RELEASE_BUTTON_A)
		run_frames(20)
		if pyboy.get_memory_value(IS_IN_BATTLE) < 0:
			return

def main():
	navigate_start_screen()
	
	walker = Walker()

	while 1:
		if pyboy.get_memory_value(m.IS_IN_BATTLE):
			run_battle()
		# This byte indicates that sprite movement is disabled. We assume that this is true
		# IFF a text box is open. probably this is a bad assumption.
		if pyboy.get_memory_value(0xCFC4): 
			navigate_generic_menu()
		else:
			walker.step()

import signal

def int_handler(signal, frame):
	breakpoint()

signal.signal(signal.SIGINT, int_handler)

if __name__ == "__main__":
	main()
