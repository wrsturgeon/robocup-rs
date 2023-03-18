#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
pub mod c;

pub mod cfg;
pub mod gated;

const _: () = assert!(c::GAMECONTROLLER_STRUCT_VERSION < u8::MAX as u32);
const _: () = assert!(c::GAMECONTROLLER_RETURN_STRUCT_VERSION < u8::MAX as u32);
const _: () = assert!((cfg::Team::UPennalizers as usize) < u8::MAX as usize);
pub const GC_DATA_HEADER: [i8; 4] = cstr_to_header(c::GAMECONTROLLER_STRUCT_HEADER);
pub const GC_DATA_VERSION: u8 = c::GAMECONTROLLER_STRUCT_VERSION as u8;
pub const GC_RETURN_HEADER: [i8; 4] = cstr_to_header(c::GAMECONTROLLER_RETURN_STRUCT_HEADER);
pub const GC_RETURN_VERSION: u8 = c::GAMECONTROLLER_RETURN_STRUCT_VERSION as u8;
pub const TEAM_NUMBER: u8 = cfg::Team::UPennalizers as u8;

impl PartialEq for c::TeamInfo {
    fn eq(&self, other: &c::TeamInfo) -> bool {
        self.teamNumber == other.teamNumber
            && self.fieldPlayerColour == other.fieldPlayerColour
            && self.goalkeeperColour == other.goalkeeperColour
            && self.goalkeeper == other.goalkeeper
            && self.score == other.score
            && self.penaltyShot == other.penaltyShot
            && self.singleShots == other.singleShots
            && self.messageBudget == other.messageBudget
    }
}

const fn cstr_to_header(cstr: &[u8; 5]) -> [i8; 4] {
    [cstr[0] as i8, cstr[1] as i8, cstr[2] as i8, cstr[3] as i8]
}
