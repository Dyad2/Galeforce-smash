use super::*;
use crate::fighters::common::opff::common_fighter_frame;

//kirby cannot use varmodule for this yet because it gets cleared as the game ends.
static mut LAST_HAT : [i32; 9] = [1; 9];

unsafe extern "C" fn kirby_hats(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    
    //fighterspecializer stuff
    let instance_boma = mem::transmute::<&mut smash::app::BattleObjectModuleAccessor, &mut smash::app::FighterModuleAccessor>(&mut *fighter.module_accessor);
        
    //kirby hat in victory screen!
    if FighterManager::is_ready_go(singletons::FighterManager()) {
        LAST_HAT[entry_id as usize] = WorkModule::get_int(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA);
        //VarModule::set_int(fighter.battle_object, kirby::instance::int::LAST_HAT, WorkModule::get_int(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA));
    }
    if [hash40("win_1"), hash40("win_2"), hash40("win_3")].contains(&curr_motion_kind) && fighter.global_table[MOTION_FRAME].get_i32() == 1 {
        //let last_hat = VarModule::get_int(fighter.battle_object, kirby::instance::int::LAST_HAT);
        WorkModule::set_int(fighter.module_accessor, LAST_HAT[entry_id as usize], *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA);
        let copy_kind = FighterSpecializer_Kirby::get_copy_kind(instance_boma) as i32;
        if copy_kind != *FIGHTER_KIND_NONE {
            let copy_slot = FighterSpecializer_Kirby::get_copy_slot_no(instance_boma) as i32;
            FighterSpecializer_Kirby::copy_setup(instance_boma, copy_slot, smash::app::FighterKind(LAST_HAT[entry_id as usize]), false, false);
        }
    }
}

unsafe extern "C" fn kirby_hammer(fighter: &mut L2CFighterCommon) {    
    //faster hammer. why is this not acmd?
    if curr_motion_kind == hash40("special_air_s_start") || curr_motion_kind == hash40("special_s_start")  {
        MotionModule::set_rate(fighter.module_accessor, 1.5);
    }
}

//prevents dtilt from crossing up shields
unsafe extern "C" fn kirby_dtilt_stop_on_guard(fighter: &mut L2CFighterCommon) {
    if curr_motion_kind == hash40("attack_lw3") && AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
        let fighter_kinetic_energy_motion = mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        FighterKineticEnergyMotion::set_speed_mul(fighter_kinetic_energy_motion, 0.3);
    }
}

unsafe extern "C" fn kirby_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    kirby_hats(fighter);
    kirby_hammer(fighter);
    kirby_dtilt_stop_on_guard(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, kirby_frame);
}