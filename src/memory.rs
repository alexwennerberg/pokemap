// Definitions for relevant memory locations
// From https://github.com/pret/pokered/blob/master/wram.asm (and potentially other RAM files in
// that repo)
//

// Player coordinates
//
pub const PLAYER_X_COORD: u16 = 0xD362;
pub const PLAYER_Y_COORD: u16 = 0xD361;
// the id of the currently selected menu item
// the top item has id 0, the one below that has id 1, etc.
// note that the "top item" means the top item currently visible on the screen
// add this value to [wListScrollOffset] to get the item's position within the list
pub const CURRENT_MENU_ITEM: u16 = 0xCC26;

// lost battle, this is -1
// ; no battle, this is 0
// ; wild battle, this is 1
// ; trainer battle, this is 2
//    ds 1
pub const IS_IN_BATTLE: u16 = 0xD057;

// 1 if movement is disabled
pub const MOVEMENT_DISABLED: u16 = 0xcfc4;

pub const CURRENT_MAP: u16 = 0xD35E;

pub const MOVEMENT_SIMULATED: u16 = 0xcd38; // 0 if not simulated, some value otherwise

// // bagItems d31e
// // player money -- d347. 3 byte range I believe. create byte range function

// Sprite data -- 16 sprites with $10 bytes each
// c100 c200
