use super::*;

//weapon
unsafe extern "C" fn needlemove(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;

    frame(lua_state, 1.);
        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 1.9, 60, 167, 0 ,0, 1.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        }
    frame(lua_state, 5.);
        if macros::is_excute(weapon)
        {
            macros::ATK_POWER(weapon, 0, 1.2);
        }
}

pub fn install(agent: &mut smashline::Agent) {
    let needle = &mut smashline::Agent::new("sheik_needle");

    needle.game_acmd("game_move", needlemove,);
}