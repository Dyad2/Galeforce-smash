use super::*;

#[fighter_frame( agent = FIGHTER_KIND_EDGE )]
fn bangs_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        //let status = StatusModule::status_kind(fighter.module_accessor);

        //GA - Sliding jump
            //type: ressource spending
            //When sephiroth has OWA, he can suppress it to force dark orbs to explode early and combo
        if !is_operation_cpu(fighter.module_accessor) {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED)
              && VarModule::is_flag(fighter.battle_object, edge::instance::flag::FLARE_EXISTS) {
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) {
                    //println!("explosion? :)");
                    galeforce_apply_effect(&mut *fighter.module_accessor, 1.0);
                    VarModule::on_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
                    VarModule::set_int(fighter.battle_object, commons::instance::int::FRAME_COUNTER, 3600);
                }
            }
            if VarModule::get_int(fighter.battle_object, commons::instance::int::FRAME_COUNTER) > 0 {
                VarModule::off_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_EDGE_WING_STATE_ACTIVATE_OFF);
                VarModule::add_int(fighter.battle_object, commons::instance::int::FRAME_COUNTER, -1);
            }
        }
    }
}

#[smashline::weapon_frame_callback]
pub fn flare2_callback(weapon: &mut smash::lua2cpp::L2CFighterBase) {
    unsafe { 
        let object_kind = get_kind(&mut *weapon.module_accessor);
        if object_kind != WEAPON_KIND_EDGE_FLARE2 {
            return
        }
        let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        let bangs = get_battle_object_from_id(owner_id);
        //println!("flare 2 exists");
        VarModule::on_flag(bangs, edge::instance::flag::FLARE_EXISTS);
        if VarModule::is_flag(bangs, commons::instance::flag::GALEFORCE_ATTACK_ON) {
            VarModule::off_flag(bangs, commons::instance::flag::GALEFORCE_ATTACK_ON);
            StatusModule::change_status_request(weapon.module_accessor, *WEAPON_EDGE_FLARE2_STATUS_KIND_END, false);
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        bangs_frame
    );
    smashline::install_agent_frame_callback!(flare2_callback);
}