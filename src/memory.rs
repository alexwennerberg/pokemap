// Definitions for relevant memory locations
// From https://github.com/pret/pokered/blob/master/wram.asm (and potentially other RAM files in
// that repo)
//


// Player coordinates
//
const PLAYER_X_COORD = 0xD362;
const PLAYER_Y_COORD = 0xD361;

// bagItems d31e
// player money -- d347. 3 byte range I believe. create byte range function

// Sprite data -- 16 sprites with $10 bytes each 
// c100 c200
