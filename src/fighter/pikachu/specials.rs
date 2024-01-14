use super::*;

//specials
unsafe extern "C" fn specials(fighter: &mut L2CAgentBase) {

        if macros::is_excute(fighter)
        {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
            VarModule::on_flag(fighter.module_accessor, commons::instance::flag::DISABLE_SPECIAL_S);
        }
    frame(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter)
        {
            JostleModule::set_status(fighter.module_accessor, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_SKULL_BASH_ATTACK_TRIGGER);
            macros::ATTACK(fighter, 0, 0, Hash40::new("rot"), 1.0, 361, 78, 0, 40, 4.0, 0.0, -0.7, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_SKULL_BASH_CALC_ATTACK_POWER);
            AttackModule::set_attack_keep_rumble(fighter.module_accessor, 0, true);
        }
    frame(fighter.lua_state_agent, 13.0);
        if macros::is_excute(fighter)
        {
            AttackModule::set_size(fighter.module_accessor, 0, 3.0);
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        }
    wait(fighter.lua_state_agent, 27.0);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_SKULL_BASH_BRAKE_TRIGGER);
        }
}

unsafe extern "C" fn specialhi2(fighter: &mut L2CAgentBase) {

    if macros::is_excute(fighter)
    {
        VarModule::off_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
        macros::ATTACK(fighter, 0, 0, Hash40::new("neck"), 3.0, 70, 50, 0, 80, 1.6, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
        JostleModule::set_status(fighter.module_accessor, false);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.game_acmd("game_specials", specials,);
    agent.game_acmd("game_specialhi2", specialhi2,);
    agent.game_acmd("game_specialairhi2", specialhi2,);
}