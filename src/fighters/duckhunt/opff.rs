use super::*;
use crate::fighters::common::opff::common_fighter_frame;

//GA: Shoot the duck!
// When hitting an opponent, DuckHunt can command the gunman to shoot towards the duck. Foes should be careful, the gunman's aim is dodgy
// part of the code is in weapon's opff, another in function_hooks (get_param_float)
pub unsafe extern "C" fn duckhunt_galeforce_attack(fighter : &mut L2CFighterCommon) {
    let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
    
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_DUCKHUNT_GENERATE_ARTICLE_GUNMAN) && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
        if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 && !VarModule::is_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON) {
            VarModule::on_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
            galeforce_apply_effect(&mut *fighter.module_accessor, 0.5);
        }
    }
    else {
        VarModule::off_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
    }
}
unsafe extern "C" fn duckhunt_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    duckhunt_galeforce_attack(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, duckhunt_frame);
}