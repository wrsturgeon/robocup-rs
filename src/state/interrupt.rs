#[inline(always)]
pub fn new_packet_number(_: u8) {
    // don't print--it's literally once a second and meaningless
}

#[allow(unused_variables)]
#[inline(always)]
pub fn new_players_per_team(value: u8) {
    debug_println!("GC: Players per team updated: {:#?}", value);
}

#[allow(unused_variables)]
#[inline(always)]
pub fn new_competition_phase(value: u8) {
    debug_println!("GC: Competition phase updated: {:#?}", value);
}

#[allow(unused_variables)]
#[inline(always)]
pub fn new_competition_type(value: u8) {
    debug_println!("GC: Competition type updated: {:#?}", value);
}

#[allow(unused_variables)]
#[inline(always)]
pub fn new_game_phase(value: u8) {
    debug_println!("GC: Game phase updated: {:#?}", value);
}

#[allow(unused_variables)]
#[inline(always)]
pub fn new_state(value: u8) {
    debug_println!("GC: State updated: {:#?}", value);
}

#[allow(unused_variables)]
#[inline(always)]
pub fn new_set_play(value: u8) {
    debug_println!("GC: Set-play updated: {:#?}", value);
}

#[allow(unused_variables)]
#[inline(always)]
pub fn new_first_half(value: u8) {
    debug_println!("GC: First-half updated: {:#?}", value);
}

#[allow(unused_variables)]
#[inline(always)]
pub fn new_kicking_team(value: u8) {
    debug_println!("GC: Kicking team updated: {:#?}", value);
}

#[allow(unused_variables)]
#[inline(always)]
pub fn new_secs_remaining(value: i16) {
    debug_println!("GC: Seconds remaining updated: {:#?}", value);
}

#[allow(unused_variables)]
#[inline(always)]
pub fn new_secondary_time(value: i16) {
    debug_println!("GC: Secondary time updated: {:#?}", value);
}

#[allow(unused_variables)]
#[inline(always)]
pub fn new_teams(value: [crate::spl::c::TeamInfo; 2]) {
    debug_println!("GC: Teams updated: {:#?}", value);
}
