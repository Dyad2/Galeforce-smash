use super::*;

//=================================================================
//== ECB ADJUSTMENTS
//== Author: 
//         original: FaultyPine
//         modifications: Dyad/
//              changelist:
//                  Bayonetta's ecb was adjusted in abk to prevent it from clipping the floor so much
//                  fixed bug related to training mode reset
//                  prevented lucina's ecb to be shifted while in aether's falling motion
//                  partly restored zelda's phantom back to 9.0.1 behavior
//=================================================================
pub unsafe fn run(fighter : &mut L2CFighterCommon) {
    let object_kind = get_kind(&mut *fighter.module_accessor);
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let prev_status_kind: i32 = StatusModule::prev_status_kind(fighter.module_accessor, 0);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let mut max_offset: f32 = 0.;

    // Statuses for regular ECB
    let vanilla_ecb = ([
                        *FIGHTER_STATUS_KIND_CAPTURE_PULLED,
                        *FIGHTER_STATUS_KIND_CAPTURE_WAIT,
                        *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE,
                        *FIGHTER_STATUS_KIND_CAPTURE_CUT,
                        *FIGHTER_STATUS_KIND_THROWN,
                        *FIGHTER_STATUS_KIND_SWALLOWED,
                        *FIGHTER_STATUS_KIND_SWALLOWED_CANCEL,
                        *FIGHTER_STATUS_KIND_SWALLOWED_CANCELED,
                        *FIGHTER_STATUS_KIND_SWALLOWED_CAPTURE
                       ].contains(&prev_status_kind))
                    || ([
                        *FIGHTER_STATUS_KIND_CAPTURE_PULLED,
                        *FIGHTER_STATUS_KIND_CAPTURE_WAIT,
                        *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE,
                        *FIGHTER_STATUS_KIND_CAPTURE_CUT,
                        *FIGHTER_STATUS_KIND_PASS,
                        *FIGHTER_STATUS_KIND_ENTRY,
                        *FIGHTER_STATUS_KIND_THROWN,
                        *FIGHTER_STATUS_KIND_SWALLOWED,
                        *FIGHTER_STATUS_KIND_SWALLOWED_CANCEL,
                        *FIGHTER_STATUS_KIND_SWALLOWED_CANCELED,
                        *FIGHTER_STATUS_KIND_SWALLOWED_CAPTURE,
                        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
                        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
                        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, 
                        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
                        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
                        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
                        *FIGHTER_STATUS_KIND_DAMAGE_FALL,
                        *FIGHTER_STATUS_KIND_TREAD_DAMAGE_AIR,
                        *FIGHTER_STATUS_KIND_BURY,
                        *FIGHTER_STATUS_KIND_BURY_WAIT
                       ].contains(&status_kind))
                    || (WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_AIR))
                    || (WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_GROUND))
                    || (WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_Y) >= 0.66);

    if is_fighter(&mut *fighter.module_accessor) {
        if !FighterManager::is_ready_go(singletons::FighterManager()) {
            GroundModule::set_rhombus_offset(fighter.module_accessor,  &Vector2f{x : 0.0, y : 0.0});
            VarModule::set_float(fighter.battle_object, commons::instance::float::ECB_OFFSET_Y, 0.0);
            return;
        }

        if !vanilla_ecb {
            if situation_kind == *SITUATION_KIND_AIR {
                if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR) > 10 {
                    // Small
                    let group1: &[i32] = &[*FIGHTER_KIND_KIRBY,*FIGHTER_KIND_PIKACHU,*FIGHTER_KIND_NESS,*FIGHTER_KIND_PURIN,*FIGHTER_KIND_GAMEWATCH,*FIGHTER_KIND_POPO,*FIGHTER_KIND_NANA,*FIGHTER_KIND_PICHU,*FIGHTER_KIND_METAKNIGHT,*FIGHTER_KIND_WARIO,*FIGHTER_KIND_PZENIGAME,*FIGHTER_KIND_PFUSHIGISOU,*FIGHTER_KIND_LUCAS,*FIGHTER_KIND_PIKMIN,*FIGHTER_KIND_TOONLINK,*FIGHTER_KIND_DUCKHUNT,*FIGHTER_KIND_MURABITO,*FIGHTER_KIND_SHIZUE];
                    // Medium
                    let group2: &[i32] = &[*FIGHTER_KIND_MARIO, *FIGHTER_KIND_YOSHI, *FIGHTER_KIND_LUIGI, *FIGHTER_KIND_MARIOD, *FIGHTER_KIND_PLIZARDON, *FIGHTER_KIND_DIDDY, *FIGHTER_KIND_DEDEDE, *FIGHTER_KIND_ROCKMAN, *FIGHTER_KIND_GEKKOUGA, *FIGHTER_KIND_PACMAN, *FIGHTER_KIND_KOOPAJR, *FIGHTER_KIND_PACKUN, *FIGHTER_KIND_MIIFIGHTER, *FIGHTER_KIND_MIISWORDSMAN, *FIGHTER_KIND_MIIGUNNER, *FIGHTER_KIND_PACKUN, *FIGHTER_KIND_BUDDY, *FIGHTER_KIND_INKLING];
                    // medium large
                    let group3: &[i32] = &[*FIGHTER_KIND_YOUNGLINK];
                    // Large
                    let group4: &[i32] = &[*FIGHTER_KIND_FOX, *FIGHTER_KIND_FALCO, *FIGHTER_KIND_DAISY, *FIGHTER_KIND_PEACH, *FIGHTER_KIND_SHEIK, *FIGHTER_KIND_ZELDA, *FIGHTER_KIND_MARTH, *FIGHTER_KIND_LUCINA, *FIGHTER_KIND_MASTER, *FIGHTER_KIND_MEWTWO, *FIGHTER_KIND_PIT, *FIGHTER_KIND_PITB, *FIGHTER_KIND_SONIC, *FIGHTER_KIND_LUCARIO, *FIGHTER_KIND_ROBOT, *FIGHTER_KIND_WOLF, *FIGHTER_KIND_KROOL, *FIGHTER_KIND_GAOGAEN, *FIGHTER_KIND_EFLAME, *FIGHTER_KIND_ELIGHT, *FIGHTER_KIND_TRAIL];
                    // X-Large
                    let group5: &[i32] = &[*FIGHTER_KIND_DONKEY, *FIGHTER_KIND_CAPTAIN, *FIGHTER_KIND_KOOPA, *FIGHTER_KIND_ROY, *FIGHTER_KIND_CHROM, *FIGHTER_KIND_LITTLEMAC, *FIGHTER_KIND_SZEROSUIT, *FIGHTER_KIND_SNAKE, *FIGHTER_KIND_IKE, *FIGHTER_KIND_WIIFIT, *FIGHTER_KIND_ROSETTA, *FIGHTER_KIND_REFLET, *FIGHTER_KIND_SHULK, *FIGHTER_KIND_RYU, *FIGHTER_KIND_KEN, *FIGHTER_KIND_CLOUD, *FIGHTER_KIND_RIDLEY, *FIGHTER_KIND_SIMON, *FIGHTER_KIND_RICHTER, *FIGHTER_KIND_JACK, *FIGHTER_KIND_BRAVE, *FIGHTER_KIND_DOLLY, *FIGHTER_KIND_TANTAN, *FIGHTER_KIND_SAMUS, *FIGHTER_KIND_SAMUSD];
                    // XX-Large
                    let group6: &[i32] = &[*FIGHTER_KIND_BAYONETTA, *FIGHTER_KIND_DEMON, *FIGHTER_KIND_EDGE, *FIGHTER_KIND_GANON, *FIGHTER_KIND_KAMUI, *FIGHTER_KIND_LINK, *FIGHTER_KIND_PALUTENA];
                    // others
                    let group7: &[i32] = &[*FIGHTER_KIND_PICKEL];

                    max_offset = match object_kind {
                        y if group1.contains(&y) => 2.,
                        y if group2.contains(&y) => 3.5,
                        y if group3.contains(&y) => 4.,
                        y if group4.contains(&y) => 4.5,
                        y if group5.contains(&y) => 5.,
                        y if group6.contains(&y) => 5.5,
                        y if group7.contains(&y) => 0.0,
                        _ => max_offset,
                    };

                    // if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
                    //     max_offset -= 0.2;
                    // }
                    //fix so bayo's side b doesn't go through the stage so much
                    if object_kind == *FIGHTER_KIND_BAYONETTA && status_kind == *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_U {
                        max_offset = 4.5;
                    }
                    //lucina lands earlier while falling with up b
                    else if object_kind == *FIGHTER_KIND_LUCINA && [hash40("special_hi_3"), hash40("landing_fall_special")].contains(&curr_motion_kind) {
                        max_offset = 0.;
                    }
                }
                else {
                    max_offset = 0.;
                }
            }
            else if situation_kind == *SITUATION_KIND_GROUND {
                max_offset = 0.;
            }
            // else {
            //     max_offset = VarModule::get_float(fighter.battle_object, commons::instance::float::ECB_OFFSET_Y);
            // }
            VarModule::set_float(fighter.battle_object, commons::instance::float::ECB_OFFSET_Y, max_offset);
            GroundModule::set_rhombus_offset(fighter.module_accessor, &Vector2f{x : 0.0, y : VarModule::get_float(fighter.battle_object, commons::instance::float::ECB_OFFSET_Y)});
        }
        else if VarModule::get_float(fighter.battle_object, commons::instance::float::ECB_OFFSET_Y) != 0.0 {
            VarModule::set_float(fighter.battle_object, commons::instance::float::ECB_OFFSET_Y, 0.0);
            GroundModule::set_rhombus_offset(fighter.module_accessor, &Vector2f{x : 0.0, y : 0.0});
        }
    }
}