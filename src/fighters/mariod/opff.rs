use super::*;
use crate::fighters::common::opff::common_fighter_frame;

unsafe extern "C" fn dr_neutral_air(fighter: &mut L2CFighterCommon) {
    let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    
    if curr_motion_kind == hash40("attack_air_n") {
        let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL).round();
        let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL).round();
        println!("mariod speed_x {}", speed_x);
        println!("mariod speed_y {}", speed_y);
    
        let damage_calc = if speed_x.abs() > speed_y.abs() {
           5.0 + (4.0 * speed_x.abs())
        }
        else {
           5.0 + (4.0 * speed_y.abs())
        };
    
        if VarModule::get_int(fighter.module_accessor, mariod::instance::int::GA_MEDECINE_TIMER) >= 0 {
            VarModule::set_float(fighter.module_accessor, mariod::instance::float::ATTACK_AIR_N_DAMAGE, damage_calc.clamp(1.0, 13.0));
        }
        else {
            VarModule::set_float(fighter.module_accessor, mariod::instance::float::ATTACK_AIR_N_DAMAGE, damage_calc.clamp(1.0, 11.0));
        }
        
        let damage_total = VarModule::get_float(fighter.module_accessor, mariod::instance::float::ATTACK_AIR_N_DAMAGE);
        if damage_total < 7. {
            VarModule::set_int(fighter.module_accessor, mariod::instance::int::ATTACK_AIR_N_SOUND_LEVEL, *ATTACK_SOUND_LEVEL_S);
        }
        if damage_total >= 7. && damage_total < 11. {
            VarModule::set_int(fighter.module_accessor, mariod::instance::int::ATTACK_AIR_N_SOUND_LEVEL, *ATTACK_SOUND_LEVEL_M);
        }
        if damage_total >= 11. {
            VarModule::set_int(fighter.module_accessor, mariod::instance::int::ATTACK_AIR_N_SOUND_LEVEL, *ATTACK_SOUND_LEVEL_L);
        }
    
        if MotionModule::frame(fighter.module_accessor) >= 3. && MotionModule::frame(fighter.module_accessor) < 27. {
            let battle_object = fighter.module_accessor;
            macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), VarModule::get_float(battle_object, mariod::instance::float::ATTACK_AIR_N_DAMAGE), 361, 93, 0, 20, 4.0, 0.8, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), VarModule::get_int(battle_object, mariod::instance::int::ATTACK_AIR_N_SOUND_LEVEL), *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneel"), VarModule::get_float(battle_object, mariod::instance::float::ATTACK_AIR_N_DAMAGE), 361, 93, 0, 20, 4.0, 0.8, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), VarModule::get_int(battle_object, mariod::instance::int::ATTACK_AIR_N_SOUND_LEVEL), *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
        }
    }
}
        
//GA - The good Medecine
// hit an opponent with a pill at close range to gain a speed buff. the buff is lost after a short duration or after being grabbed.
//  the rest of the GA code is in common opff, it's opponent-side
unsafe extern "C" fn dr_galeforce_attack(fighter: &mut L2CFighterCommon) {
    mariod_buff_effect(fighter);
    if VarModule::is_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON) && VarModule::get_int(fighter.module_accessor, mariod::instance::int::GA_MEDECINE_TIMER) <= 0 {
        galeforce_apply_effect(&mut *fighter.module_accessor, 0.66);
        VarModule::set_int(fighter.module_accessor, mariod::instance::int::GA_MEDECINE_TIMER, 1500);
        VarModule::off_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
    }
    if VarModule::get_int(fighter.module_accessor, mariod::instance::int::GA_MEDECINE_TIMER) >= 0 {
        VarModule::sub_int(fighter.module_accessor, mariod::instance::int::GA_MEDECINE_TIMER, 1);
    }
    if is_status_grabbed(&mut *fighter.module_accessor) {
        VarModule::set_int(fighter.module_accessor, mariod::instance::int::GA_MEDECINE_TIMER, -1);
    }
    
}

unsafe extern "C" fn dr_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    dr_neutral_air(fighter);
    dr_galeforce_attack(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, dr_frame);
}