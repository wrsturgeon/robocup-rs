pub struct GCHandler {
    pub current: crate::spl::c::RoboCupGameControlData,
}

impl crate::spl::interrupt::GCDataInterruptHandler for GCHandler {
    #[inline(always)]
    fn interrupt_packetNumber(&self) {
        // don't print--it's literally once a second and meaningless
    }
    #[inline(always)]
    fn interrupt_playersPerTeam(&self) {
        debug_println!(
            "GC: Players per team updated: {:#?}",
            self.current.playersPerTeam
        );
    }
    #[inline(always)]
    fn interrupt_competitionPhase(&self) {
        debug_println!(
            "GC: Competition phase updated: {:#?}",
            self.current.competitionPhase
        );
    }
    #[inline(always)]
    fn interrupt_competitionType(&self) {
        debug_println!(
            "GC: Competition type updated: {:#?}",
            self.current.competitionType
        );
    }
    #[inline(always)]
    fn interrupt_gamePhase(&self) {
        debug_println!("GC: Game phase updated: {:#?}", self.current.gamePhase);
    }
    #[inline(always)]
    fn interrupt_state(&self) {
        debug_println!("GC: State updated: {:#?}", self.current.state);
    }
    #[inline(always)]
    fn interrupt_setPlay(&self) {
        debug_println!("GC: Set-play updated: {:#?}", self.current.setPlay);
    }
    #[inline(always)]
    fn interrupt_firstHalf(&self) {
        debug_println!("GC: First-half updated: {:#?}", self.current.firstHalf);
    }
    #[inline(always)]
    fn interrupt_kickingTeam(&self) {
        debug_println!("GC: Kicking team updated: {:#?}", self.current.kickingTeam);
    }
    #[inline(always)]
    fn interrupt_secsRemaining(&self) {
        debug_println!(
            "GC: Seconds remaining updated: {:#?}",
            self.current.secsRemaining
        );
    }
    #[inline(always)]
    fn interrupt_secondaryTime(&self) {
        debug_println!(
            "GC: Secondary time updated: {:#?}",
            self.current.secondaryTime
        );
    }
    #[inline(always)]
    fn interrupt_teams(&self) {
        debug_println!("GC: Teams updated: {:#?}", self.current.teams);
    }
}
