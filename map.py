import os
from os import listdir
from os.path import isfile, join
from pathlib import Path

MAP_DATA_FOLDER = "pokered/maps"
# headers which contain tileset, connections
MAP_HEADER_FOLDER = "pokered/data/maps/headers"
# Constants, which contain map id and size
MAP_CONSTANTS_FILE = "pokered/constants/map_constants.asm"

map_constants = {}
# Tile collisions
# Blocksets

class Map:
	def __init__(self, map_header_file, map_data_file):
		text = map_header_file.read_text()
		lines = text.split("\n")
		self.mapname, self.map_id_string, self.tileset, connections = tuple(lines[1][12:].split(", "))
		self.height, self.width, self.map_id = map_constants[self.map_id_string]
		return
	

def parse_constants():
	map_constants_text = Path(MAP_CONSTANTS_FILE).read_text()
	lines = map_constants_text.split("\n")
	for line in lines:
		if line.startswith("\tmapconst "):
			map_id_string, height, width, map_id_hex = tuple(
				line[10:].replace(" ", "").replace(";",",").split(',')
			)
			map_constants[map_id_string] = (int(height), int(width), int(map_id_hex[1:], 16))

def get_data():
	header_folder = Path(MAP_HEADER_FOLDER)
	header_files = [f for f in header_folder.iterdir()
		if f.name not in ["UndergroundPathRoute7Copy.asm"]
		and f.suffix == ".asm"]
	for header in header_files:
		data_file = Path("pokered/maps/{}.blk".format(header.stem))
		Map(header, data_file)

parse_constants()
get_data()
