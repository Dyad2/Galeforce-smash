use super::*;

#[fighter_frame( agent = FIGHTER_KIND_MIIGUNNER )]
fn lame_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);

        if curr_motion_kind == hash40("attack_air_hi") {
            let offset = Vector2f {x: 0.0, y: 4.0 };
            GroundModule::set_rhombus_offset(fighter.module_accessor, &offset);
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        lame_frame
    );
}