#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub mod c;

pub mod cfg;
pub mod diff;
pub mod interrupt;

const _: () = assert!(c::GAMECONTROLLER_STRUCT_VERSION < u8::MAX.into());
const _: () = assert!(c::GAMECONTROLLER_RETURN_STRUCT_VERSION < u8::MAX.into());
const _: () = assert!((cfg::Team::UPennalizers as usize) < u8::MAX.into());
pub const GC_DATA_HEADER: [i8; 4] = cstr_to_header(c::GAMECONTROLLER_STRUCT_HEADER);
pub const GC_DATA_VERSION: u8 = c::GAMECONTROLLER_STRUCT_VERSION as u8;
pub const GC_RETURN_HEADER: [i8; 4] = cstr_to_header(c::GAMECONTROLLER_RETURN_STRUCT_HEADER);
pub const GC_RETURN_VERSION: u8 = c::GAMECONTROLLER_RETURN_STRUCT_VERSION as u8;
pub const TEAM_NUMBER: u8 = cfg::Team::UPennalizers as u8;

impl c::RoboCupGameControlData {
    pub const fn new() -> Self {
        Self {
            header: GC_DATA_HEADER,
            version: c::GAMECONTROLLER_STRUCT_VERSION as u8,
            packetNumber: 0,
            playersPerTeam: 5,
            competitionPhase: 0,
            competitionType: 0,
            gamePhase: 0,
            state: 0,
            setPlay: 0,
            firstHalf: 0,
            kickingTeam: 0,
            secsRemaining: i16::MAX,
            secondaryTime: 0,
            teams: [c::TeamInfo::new(), c::TeamInfo::new()],
        }
    }
}

impl c::TeamInfo {
    pub const fn new() -> Self {
        Self {
            teamNumber: 0,
            fieldPlayerColour: 0,
            goalkeeperColour: 0,
            goalkeeper: 0,
            score: 0,
            penaltyShot: 0,
            singleShots: 0,
            messageBudget: u16::MAX,
            players: [
                c::RobotInfo::new(),
                c::RobotInfo::new(),
                c::RobotInfo::new(),
                c::RobotInfo::new(),
                c::RobotInfo::new(),
                c::RobotInfo::new(),
                c::RobotInfo::new(),
                c::RobotInfo::new(),
                c::RobotInfo::new(),
                c::RobotInfo::new(),
                c::RobotInfo::new(),
                c::RobotInfo::new(),
                c::RobotInfo::new(),
                c::RobotInfo::new(),
                c::RobotInfo::new(),
                c::RobotInfo::new(),
                c::RobotInfo::new(),
                c::RobotInfo::new(),
                c::RobotInfo::new(),
                c::RobotInfo::new(),
            ],
        }
    }
}

impl c::RobotInfo {
    pub const fn new() -> Self {
        Self {
            penalty: 0,
            secsTillUnpenalised: 0,
        }
    }
}

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
