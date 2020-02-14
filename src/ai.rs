
enum PlayerState {
    InBattle,
    CanMove,
    Immobile,
    MenuOpen,
    TextBoxOpen,
}

// use as input to Movement AI 
// current map
// visited maps (vector)
// sprites iteracted with (vector)
//
// Then generate weights for each probability
//
// milestones ?
//
// use as input to battle AI
// Health of current pokemon
// Health of opponent pokemon

// Decisions become executed, can succeed or fail
enum MovementDecision {
    InteractWithPerson(sprite_number),
    GoToWarp(warp), 
    UseItem(item), 
    // CatchPokemon,
    // SwitchBoxPokemon,
}

enum BattleDecision {
    UseItem(item),
    Move(move_),
    Run,
    Switch(pokemon)
}
