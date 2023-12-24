use super::*;

#[fighter_frame( agent = FIGHTER_KIND_BAYONETTA )]
fn bayo_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
        
        //GA - Witch's Ascent
        // type: restriction lift
        //  after using afterburner kick once, hitting with dabk allows an additional use of upwards abk
        //   note: it does enable ladder combos, but with so many abk hits if the opponent doesn't sdi it's on them
        if status_kind == *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_U 
          && AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            VarModule::on_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
        }
        //allows one more abk after hitting with a second dabk
        if status_kind == *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D 
          && AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) 
          && VarModule::is_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON) {
            VarModule::on_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_CONFIRM);
        }
        if VarModule::is_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_CONFIRM) {
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_AIR_S_USED_COUNT);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S);
            if (status_kind == *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_U || status_kind == *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D) && MotionModule::frame(fighter.module_accessor) < 3.0 {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S);
                VarModule::off_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_CONFIRM);
                VarModule::off_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
                galeforce_apply_effect(&mut *fighter.module_accessor, 0.66);
            }
        }
        if situation_kind == *SITUATION_KIND_GROUND {
            VarModule::off_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_CONFIRM);
            VarModule::off_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
        }

        //remove gravity on bats startup. TODO: status? is there one?
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_BAYONETTA_STATUS_KIND_BATWITHIN {
            if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR && MotionModule::frame(fighter.module_accessor) <= 10. {
                KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }
            else {
                KineticModule::resume_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }
        }

        //dodge offset, port to status someday please
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S3 && !VarModule::is_flag(fighter.battle_object, bayonetta::status::flag::DODGE_OFFSET_FORBID) {
            if VarModule::is_flag(fighter.battle_object, bayonetta::instance::flag::DODGE_OFFSET) {
                if VarModule::get_int(fighter.battle_object, bayonetta::instance::int::DODGE_OFFSET_NUM) == 2 {
                    VarModule::set_int(fighter.battle_object, bayonetta::instance::int::DODGE_OFFSET_NUM, 0);
                    VarModule::off_flag(fighter.battle_object, bayonetta::instance::flag::DODGE_OFFSET);
                    StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S3, false);
                    MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40{hash: hash40("attack_s3_s2")}, 0.0, 0.0, 0.0);
                }
                if VarModule::get_int(fighter.battle_object, bayonetta::instance::int::DODGE_OFFSET_NUM) == 3 {
                    VarModule::set_int(fighter.battle_object, bayonetta::instance::int::DODGE_OFFSET_NUM, 0);
                    VarModule::off_flag(fighter.battle_object, bayonetta::instance::flag::DODGE_OFFSET);
                    StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S3, false);
                    MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40{hash: hash40("attack_s3_s3")}, 0.0, 0.0, 0.0);
                }
            }
            if VarModule::get_int(fighter.battle_object, bayonetta::instance::int::DODGE_OFFSET_NUM) != 0 {
                VarModule::on_flag(fighter.battle_object, bayonetta::instance::flag::DODGE_OFFSET);
                if ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor) >= 0.2 {
                    MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40{hash: hash40("escape_f")}, 0.0, 0.0, 0.0);
                    StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_F, false);
                }
                if ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor) < 0.2 && ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor) > -0.2 {
                    MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40{hash: hash40("escape")}, 0.0, 0.0, 0.0);
                    StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE, false);
                }
                if ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor) <= -0.2 {
                    MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40{hash: hash40("escape_b")}, 0.0, 0.0, 0.0);
                    StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_B, false);
                }
            }
        }
        if ![hash40("attack_s3_s"), hash40("attack_s3_s2"), hash40("escape_b"), hash40("escape"), hash40("escape_f")].contains(&curr_motion_kind) /*&& fighter.global_table[MOTION_FRAME].get_i32() == 14*/ {
            VarModule::set_int(fighter.battle_object, bayonetta::instance::int::DODGE_OFFSET_NUM, 0);
            VarModule::off_flag(fighter.battle_object, bayonetta::instance::flag::DODGE_OFFSET);
        }

        //not touching the jab finisher sound script with a 100ft long pole, and since i can't use original!() ...
        if curr_motion_kind == hash40("attack_100_end") && ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_BAYONETTA_GENERATE_ARTICLE_WICKEDWEAVEARM) {
            macros::PLAY_SE(fighter, Hash40::new("se_bayonetta_wecked_chargefull")); //WECKEEEED idk smash dev typo again
        }

        //removes wicked weave costume
        if is_status_damage(&mut *fighter.module_accessor) {
            smash_script::notify_event_msc_cmd!(fighter, 0x25fd66ecef as u64, 30, 0, 1);
        }

        //prevents landing prematurily when using up b. TODO: status script plz
        if [hash40("special_hi"), hash40("special_air_hi")].contains(&curr_motion_kind) {
            if fighter.global_table[MOTION_FRAME].get_i32() <= 34 {
                StatusModule::set_keep_situation_air(fighter.module_accessor, true);
                GroundModule::pass_floor(fighter.module_accessor);
            }
            else {
                StatusModule::set_keep_situation_air(fighter.module_accessor, false);
            }
        }
        
        //abk drift
        //if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 { //purple only for now
        //    if status_kind == *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_U 
        //      && fighter.global_table[MOTION_FRAME].get_i32() < 25
        //      && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
        //      && !is_hitlag(fighter.module_accessor) {
        //        let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
        //        if stick_y != 0.0 {
        //            KineticModule::add_speed_outside(fighter.module_accessor, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND, &Vector3f{ x: 0.0, y: 1.1125 * stick_y, z: 0.0});
        //        }
        //        let stick_x = ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor);
        //        if stick_x != 0.0 {
        //            KineticModule::add_speed_outside(fighter.module_accessor, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND, &Vector3f{ x: 1.11 * stick_x, y: 0.0, z: 0.0});
        //        }
        //    }
        //}
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        bayo_frame
    );
}