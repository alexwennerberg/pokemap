import os
from os import listdir
from os.path import isfile, join
from pathlib import Path

MAP_DATA_FOLDER = "pokered/maps"
# headers which contain tileset, connections
MAP_HEADER_FOLDER = "pokered/data/maps/headers"

map_constants = {}
block_data = {}
collision_tiles = {}
# Tile collisions
# Blocksets
tileset_rename = { 
"DOJO": "gym",
"MART": "pokecenter",
"FOREST_GATE": "gate",
"MUSEUM": "gate",
"REDS_HOUSE_2": "reds_house",
"REDS_HOUSE_1": "reds_house"
}

class Map:
	def __init__(self, map_header_file, map_data_file):
		text = map_header_file.read_text()
		lines = text.split("\n")
		self.mapname, self.map_id_string, self.tileset, connections = tuple(lines[1][12:].split(", "))
		self.height, self.width, self.map_id = map_constants[self.map_id_string]
		# TODO abstract out -- dont repeat for each file
		tileset = tileset_rename.get(self.tileset) or self.tileset.lower()
		block_data = Path("pokered/gfx/blocksets/{}.bst".format(tileset)).read_bytes()
	

def parse_constants():
	map_constants_text = Path("pokered/constants/map_constants.asm").read_text()
	lines = map_constants_text.split("\n")
	for line in lines:
		if line.startswith("\tmapconst "):
			map_id_string, height, width, map_id_hex = tuple(
				line[10:].replace(" ", "").replace(";",",").split(',')
			)
			map_constants[map_id_string] = (int(height), int(width), int(map_id_hex[1:], 16))

def parse_collision_tiles():
	map_constants_text = Path("pokered/data/tilesets/collision_tile_ids.asm").read_text()
	lines = map_constants_text.split("\n")
	for line in lines:
		if line.startswith("\tcoll_tiles"):
			print(line[12:].split(", "))
	return

def get_data():
	header_folder = Path(MAP_HEADER_FOLDER)
	header_files = [f for f in header_folder.iterdir()
		if f.name not in ["UndergroundPathRoute7Copy.asm"]
		and f.suffix == ".asm"]
	for header in header_files:
		data_file = Path("pokered/maps/{}.blk".format(header.stem))
		Map(header, data_file)

parse_constants()
parse_collision_tiles()
get_data()

# TODO -- get collision data and make full maps
# TODO -- get map connection/warp data
