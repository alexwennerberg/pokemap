# Definitions for relevant memory locations
# https://github.com/pret/pokered/blob/7ecfd641fb47e4a6ed475302bc1deac17473e704/wram.asm
# Player coordinates

PLAYER_X_COORD = 0xD362
PLAYER_Y_COORD = 0xD361
# the id of the currently selected menu item
# // the top item has id 0, the one below that has id 1, etc.
# // note that the "top item" means the top item currently visible on the screen
# // add this value to [wListScrollOffset] to get the item's position within the list
CURRENT_MENU_ITEM = 0xCC26

# // lost battle, this is -1
# // ; no battle, this is 0
# // ; wild battle, this is 1
# // ; trainer battle, this is 2
# //    ds 1
IS_IN_BATTLE = 0xD057
MOVEMENT_DISABLED= 0xcfc4

CURRENT_MAP = 0xD35E;

MOVEMENT_SIMULATED = 0xcd38; # // 0 if not simulated, some value otherwise
TEXT_BOX_ID = 0xd125

# // // bagItems d31e
# // // player money -- d347. 3 byte range I believe. create byte range function

# // Sprite data -- 16 sprites with $10 bytes each
# // c100 c200
