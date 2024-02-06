use super::*;

#[skyline::hook(replace = L2CFighterCommon_status_ShieldBreakFly)]
unsafe extern "C" fn status_ShieldBreakFly_Main(fighter: &mut L2CFighterCommon) {
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            //ShieldBreakDown no longer causes fighters to drop prone and get back up, they use damage_hi_3 anim instead
            fighter.change_status(FIGHTER_STATUS_KIND_SHIELD_BREAK_DOWN.into(), true.into());
        } 
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_SHIELD_BREAK_FALL.into(), true.into());
        }
    }
    return;
}

#[skyline::hook(replace = L2CFighterCommon_status_pre_ShieldBreakFly)]
unsafe extern "C" fn status_ShieldBreakFly_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    
    StatusModule::init_settings(
        fighter.module_accessor, 
        smash::app::SituationKind(*SITUATION_KIND_GROUND), 
        *FIGHTER_KINETIC_TYPE_DAMAGE_GROUND,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32, 
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), 
        true, 
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, 
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, 
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor, 
        false,
        *FIGHTER_TREADED_KIND_DISABLE, 
        false, 
        false, 
        false, 
        0,
        *FIGHTER_STATUS_ATTR_DISABLE_SHIELD_RECOVERY as u32,
        0,
        0
    );
    
    return 0.into();
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_ShieldBreakFly_Main,
            status_ShieldBreakFly_pre,
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}