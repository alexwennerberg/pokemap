from pyboy import PyBoy
from pyboy.utils import WindowEvent

pyboy = PyBoy('roms/pokemon-blue.gb')
pyboy.set_emulation_speed(0)
pyboy.get_memory_value(0xD086)

def run_frames(count):
	for _ in range(count):
		pyboy.tick()

def walk(direction, test):
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
	run_frames(test)
	pyboy.send_input(keyup)
	run_frames(test)

for _ in range(3250):
	pyboy.send_input(WindowEvent.PRESS_BUTTON_A)
	pyboy.tick()
	pyboy.send_input(WindowEvent.RELEASE_BUTTON_A)
	pyboy.tick()

breakpoint()
while 1:
	for d in ["down", "left", "up", "right"]:
		walk(d)
