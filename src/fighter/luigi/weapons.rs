use super::*;

//weapons
unsafe extern "C" fn plunger_aircatch(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent)
    {
        macros::ATTACK(agent, 0, 0, Hash40::new("plungerrope26"), 5.0, 361, 100, 0, 10, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
    }
}

unsafe extern "C" fn fireballregular(weapon: &mut L2CAgentBase) {    
    frame(weapon.lua_state_agent, 1.);
        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 6.0, 361, 22, 0, 12, 2.7, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -3, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            AttackModule::set_add_reaction_frame_revised(weapon.module_accessor, 0, 5.0, false);
            AttackModule::enable_safe_pos(weapon.module_accessor);
        }
}

pub fn install() {
    let plunger = &mut smashline::Agent::new("luigi_plunger");
    let fireball = &mut smashline::Agent::new("luigi_fireball");

    plunger.game_acmd("game_aircatch", plunger_aircatch,);
    fireball.game_acmd("game_regular", fireballregular,);
}