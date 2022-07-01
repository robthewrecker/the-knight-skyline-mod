use smash::hash40;
use smash::phx::Hash40;
use smash::phx::Vector3f;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::BattleObjectModuleAccessor;
use smash::app::sv_animcmd;
use smash::app::lua_bind::*;
use smash::app::lua_bind::EffectModule;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smashline::*;
use smash_script::*;
use smash_script::macros;
use smash_script::macros::EFFECT_ALPHA;

static mut ENTRY_ID : usize = 0;

#[fighter_frame( agent = FIGHTER_KIND_TOONLINK )]
fn toonlink_frame(fighter: &mut L2CFighterCommon) {
	unsafe {
		let boma = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let status_kind = StatusModule::status_kind(boma);
		ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		
		// Allows DAir to bounce off enemies //
		if MotionModule::motion_kind(boma) == hash40("attack_air_lw") {
			if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) == true  {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP, true);
			}
		}
		
		if MotionModule::motion_kind(boma) == hash40("special_air_lw") {
			if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND	{
				MotionModule::change_motion(boma, Hash40::new("special_lw"), 10.0, 1.0, false, 0.0, false, false);
			}		
		}
}}


/////////		ACMD Scripts		\\\\\\\\\

// Jump //
#[acmd_script(
	agent = "toonlink",
	scripts = [ "sound_jumpfront", "sound_jumpback" ],
	category = ACMD_SOUND )]
unsafe fn knight_jump_sound(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_toonlink_jump01"));
    }
}

// Aerial Jump //
#[acmd_script(
	agent = "toonlink",
	scripts = [ "sound_jumpaerialfront", "sound_jumpaerialback" ],
	category = ACMD_SOUND )]
unsafe fn knight_djump_sound(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_toonlink_jumpround"));
    }
}

//		Jab 1 + 2		//
#[acmd_script(
	agent = "toonlink",
	scripts = [ "game_attack11", "game_attack12" ],
	category = ACMD_GAME )]
unsafe fn knight_jab12_game(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.4, 361, 10, 0, 10, 4.8, 0.0, 6.0, 17.0, Some(0.0), Some(6.0), Some(2.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
    }
	sv_animcmd::frame(fighter.lua_state_agent, 9.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
	sv_animcmd::frame(fighter.lua_state_agent, 14.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO);
    }
}

//	Jab 3
#[acmd_script(
	agent = "toonlink",
	script = "game_attack13",
	category = ACMD_GAME )]
unsafe fn knight_jab3_game(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.4, 361, 10, 0, 10, 4.8, 0.0, 6.0, 17.0, Some(0.0), Some(6.0), Some(2.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
    }
}

//		Forward Tilt		//
#[acmd_script(
	agent = "toonlink",
	script = "game_attacks3",
	category = ACMD_GAME )]
unsafe fn knight_ftilt_game(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("sword2"), 8.5, 361, 86, 0, 41, 4.7, 5.0, 0.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 1, 0, Hash40::new("sword2"), 8.5, 361, 86, 0, 41, 4.7, 1.0, 0.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
    }
}

//		Up Tilt		//
#[acmd_script(
	agent = "toonlink",
	script = "game_attackhi3",
	category = ACMD_GAME )]
unsafe fn knight_utilt_game(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("sword2"), 7.5, 95, 102, 0, 41, 3.4, 5.0, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 1, 0, Hash40::new("sword2"), 7.5, 85, 102, 0, 41, 3.4, 1.0, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 4.0);
}

#[acmd_script(
	agent = "toonlink",
	script = "effect_attackhi3",
	category = ACMD_EFFECT )]
unsafe fn knight_utilt_effect(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
	if macros::is_excute(fighter) {
        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("toonlink_sword1"), Hash40::new("toonlink_sword2"), 4, Hash40::new("sword1"), 0.5, 0.0, -0.5, Hash40::new("sword1"), 10.57, -0.13, -0.12, true, Hash40::new("toonlink_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.5, 0.2);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
        macros::AFTER_IMAGE_OFF(fighter, 2);
    }
}

//		Down Tilt		//
#[acmd_script(
	agent = "toonlink",
	script = "game_attacklw3",
	category = ACMD_GAME )]
unsafe fn knight_dtilt_game(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.5, 30, 40, 0, 50, 2.7, 0.0, 2.5, 13.0, Some(0.0), Some(3.1), Some(9.2), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
		AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
    }
}

//		Up Smash		//
#[acmd_script(
	agent = "toonlink",
	script = "game_attackhi4",
	category = ACMD_GAME )]
unsafe fn knight_usmash_game(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
	sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.5, 100, 84, 0, 45, 3.5, 0.0, 6.0, 0.0, Some(0.0), Some(28.5), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_ENERGY);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 6.5, 100, 84, 0, 45, 3.5, 0.0, 6.5, 4.5, Some(0.0), Some(22.5), Some(8.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_ENERGY);
		macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 6.5, 100, 84, 0, 45, 3.5, 0.0, 6.5, -4.5, Some(0.0), Some(22.5), Some(-8.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_ENERGY);
		macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
    }
	sv_animcmd::frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.5, 100, 84, 0, 45, 3.5, 0.0, 6.0, 0.0, Some(0.0), Some(24.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_ENERGY);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 6.5, 100, 84, 0, 45, 3.5, 0.0, 6.5, 2.0, Some(0.0), Some(17.5), Some(5.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_ENERGY);
		macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 6.5, 100, 84, 0, 45, 3.5, 0.0, 6.5, -2.0, Some(0.0), Some(17.5), Some(-5.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_ENERGY);
		macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
    }
	sv_animcmd::frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
    }
	sv_animcmd::frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.5, 100, 84, 0, 45, 3.5, 0.0, 6.0, 0.0, Some(0.0), Some(24.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_ENERGY);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 6.5, 100, 84, 0, 45, 3.5, 0.0, 6.5, 2.0, Some(0.0), Some(17.5), Some(5.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_ENERGY);
		macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 6.5, 100, 84, 0, 45, 3.5, 0.0, 6.5, -2.0, Some(0.0), Some(17.5), Some(-5.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_ENERGY);
		macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 27.0);
    if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
    }
}

#[acmd_script(
	agent = "toonlink",
	scripts = [ "sound_attackhi4" ],
	category = ACMD_SOUND )]
unsafe fn knight_usmash_sound(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
		macros::STOP_SE(fighter, Hash40::new("se_common_smash_start"));
    }
    sv_animcmd::wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_toonlink_win01_01"));
    }
}

//		Forward Smash		//

#[acmd_script(
	agent = "toonlink",
	script = "game_attacks4",
	category = ACMD_GAME )]
unsafe fn knight_fsmash_game(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
	sv_animcmd::frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.5, 30, 104, 0, 35, 7.0, 0.0, 5.0, 23.5, Some(0.0), Some(10.5), Some(4.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script(
	agent = "toonlink",
	scripts = [ "sound_attacks4" ],
	category = ACMD_SOUND )]
unsafe fn knight_fsmash_sound(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
		macros::STOP_SE(fighter, Hash40::new("se_common_smash_start"));
    }
	sv_animcmd::wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_toonlink_appeal_h01_win01"));
    }
}

//		Down Smash		//
#[acmd_script(
	agent = "toonlink",
	script = "game_attacklw4",
	category = ACMD_GAME )]
unsafe fn knight_dsmash_game(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
	sv_animcmd::frame(fighter.lua_state_agent, 21.0);
	if ControlModule::get_stick_x(fighter.module_accessor) > 0.5 {
			macros::SET_SPEED_EX(fighter, (2.0) * (PostureModule::lr(fighter.module_accessor)), 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
	}
	else if ControlModule::get_stick_x(fighter.module_accessor) < -0.5 {
			macros::SET_SPEED_EX(fighter, (-2.0) * (PostureModule::lr(fighter.module_accessor)), 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
	}
	else {
		macros::SET_SPEED_EX(fighter, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
	}
    if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.8, 180, 100, 50, 30, 4.8, 0.0, 6.0, 18.0, Some(0.0), Some(6.0), Some(2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 24.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
    }
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.8, 180, 100, 50, 30, 4.0, 0.0, 11.0, 7.0, Some(0.0), Some(11.0), Some(-7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
	sv_animcmd::frame(fighter.lua_state_agent, 27.0);
    if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
    }
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.8, 180, 100, 50, 30, 4.8, 0.0, 6.0, -18.0, Some(0.0), Some(6.0), Some(-2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
	sv_animcmd::frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
    }
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.8, 180, 100, 50, 30, 4.0, 0.0, 2.5, 7.0, Some(0.0), Some(2.5), Some(-7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
	sv_animcmd::frame(fighter.lua_state_agent, 33.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
    }
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.8, 180, 100, 50, 30, 4.8, 0.0, 6.0, 18.0, Some(0.0), Some(6.0), Some(2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
	sv_animcmd::frame(fighter.lua_state_agent, 39.0);
    if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
    }
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.8, 180, 100, 50, 30, 4.8, 0.0, 11.0, 7.0, Some(0.0), Some(11.0), Some(-7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
	sv_animcmd::frame(fighter.lua_state_agent, 42.0);
    if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
    }
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.8, 180, 100, 50, 30, 4.8, 0.0, 6.0, -18.0, Some(0.0), Some(6.0), Some(-2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
	sv_animcmd::frame(fighter.lua_state_agent, 45.0);
    if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
    }
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.8, 180, 160, 50, 30, 4.0, 0.0, 2.5, 7.0, Some(0.0), Some(2.5), Some(-7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
	sv_animcmd::frame(fighter.lua_state_agent, 47.0);
    if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		macros::SET_SPEED_EX(fighter, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
}

#[acmd_script(
	agent = "toonlink",
	scripts = [ "sound_attacklw4" ],
	category = ACMD_SOUND )]
unsafe fn knight_dsmash_sound(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
		macros::STOP_SE(fighter, Hash40::new("se_common_smash_start"));
    }
	sv_animcmd::wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_toonlink_special_n01"));
    }
}

//		Neutral Aerial		//
#[acmd_script(
	agent = "toonlink",
	script = "game_attackairn",
	category = ACMD_GAME )]
unsafe fn knight_attackairn_game(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
	sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("bust"), 4.0, 42, 85, 0, 32, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
	sv_animcmd::wait(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		macros::ATTACK(fighter, 0, 0, Hash40::new("bust"), 8.5, 42, 85, 0, 32, 8.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
	}
    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 30.0);
	if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//		Forward Aerial		//
#[acmd_script(
	agent = "toonlink",
	script = "game_attackairf",
	category = ACMD_GAME )]
unsafe fn knight_attackairf_game(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.5, 10, 50, 0, 50, 4.8, 0.0, 6.0, 17.0, Some(0.0), Some(6.0), Some(2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 21.0);
	if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//		Up Aerial		//
#[acmd_script(
	agent = "toonlink",
	script = "game_attackairhi",
	category = ACMD_GAME )]
unsafe fn knight_attackairhi_game(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.6, 361, 10, 0, 10, 4.8, 0.0, 8.0, 0.0, Some(0.0), Some(18.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 22.0);
	if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//		Down Aerial		//
#[acmd_script(
	agent = "toonlink",
	script = "game_attackairlw",
	category = ACMD_GAME )]
unsafe fn knight_attackairlw_game(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.4, 94, 25, 0, 15, 4.8, 0.0, -2.0, 0.0, Some(0.0), Some(-8.0), Some(0.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 34.0);
	if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//		Neutral Special		//

	// Ground //
	
#[acmd_script(
	agent = "toonlink",
	script = "game_specialnstart",
	category = ACMD_GAME )]
unsafe fn knight_nspecial_game(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE);
	}
}

#[acmd_script(
	agent = "toonlink",
	script = "game_specialnend",
	category = ACMD_GAME )]
unsafe fn knight_nspecialend_game(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("fingerr23"), 6.6, 361, 10, 0, 10, 4.8, 0.0, 8.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_ENERGY);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 37.0);
    if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

	// Air //
	
#[acmd_script(
	agent = "toonlink",
	script = "game_specialairnstart",
	category = ACMD_GAME )]
unsafe fn knight_nspecialair_game(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE);
	}
}

#[acmd_script(
	agent = "toonlink",
	script = "game_specialairnend",
	category = ACMD_GAME )]
unsafe fn knight_nspecialairend_game(fighter: &mut L2CAgentBase) {
	let boma = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
		KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
	}
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("fingerr23"), 6.6, 361, 10, 0, 10, 4.8, 0.0, 8.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_ENERGY);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 37.0);
    if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		KineticModule::resume_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
	}
}

//		Side Special		//

	// Ground //
#[acmd_script(
	agent = "toonlink",
	script = "game_specials1",
	category = ACMD_GAME )]
unsafe fn knight_sidespecial_game(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOOMERANG, ArticleOperationTarget(0));
    }
	sv_animcmd::frame(fighter.lua_state_agent, 11.0);
	if macros::is_excute(fighter) {
		macros::SET_SPEED_EX(fighter, 1.2, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 15.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.3, 361, 99, 0, 30, 2.8, 0.0, 7.5, 24.0, Some(0.0), Some(7.5), Some(2.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_death"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
}

	// Air	//
#[acmd_script(
	agent = "toonlink",
	script = "game_specialairs1",
	category = ACMD_GAME )]
unsafe fn knight_sidespecialair_game(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOOMERANG, ArticleOperationTarget(0));
    }
}

//		Up Special		//

	// Ground //
#[acmd_script(
	agent = "toonlink",
	script = "game_specialhi",
	category = ACMD_GAME )]
unsafe fn knight_upspecial_game(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
	if ControlModule::get_stick_x(fighter.module_accessor) > 0.5 {
		if ControlModule::get_stick_y(fighter.module_accessor) > 0.2 {
			macros::SET_SPEED_EX(fighter, (12.0) * (PostureModule::lr(fighter.module_accessor)), 8.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
		}
		else {
			macros::SET_SPEED_EX(fighter, (12.0) * (PostureModule::lr(fighter.module_accessor)), 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
		}
	}
	else if ControlModule::get_stick_x(fighter.module_accessor) < -0.5 {
		if ControlModule::get_stick_y(fighter.module_accessor) > 0.2 {
			macros::SET_SPEED_EX(fighter, (-12.0) * (PostureModule::lr(fighter.module_accessor)), 8.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
		}
		else {
			macros::SET_SPEED_EX(fighter, (-12.0) * (PostureModule::lr(fighter.module_accessor)), 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
		}
	}
	else {
		macros::SET_SPEED_EX(fighter, 0.0, 12.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
	}
}

	// Air	//


//		Down Special		//

	//	Ground	//
#[acmd_script(
	agent = "toonlink",
	script = "game_speciallw",
	category = ACMD_GAME )]
unsafe fn knight_downspecial_game(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.4, 94, 25, 0, 15, 4.8, 0.0, -2.0, 0.0, Some(0.0), Some(-8.0), Some(0.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
 
    }
	sv_animcmd::frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 361, 99, 0, 30, 2.8, 0.0, 1.5, -20.0, Some(0.0), Some(1.5), Some(20.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_ENERGY);
		macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
    }
	sv_animcmd::frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
	}
}

	//	Air	//
#[acmd_script(
	agent = "toonlink",
	script = "game_specialairlw",
	category = ACMD_GAME )]
unsafe fn knight_downspecialair_game(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_LANDING_CLEAR_SPEED);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
		macros::SET_SPEED_EX(fighter, 0.0, 2.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
		JostleModule::set_status(fighter.module_accessor, false);
		KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
	sv_animcmd::frame(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_TOONLINKBOMB, ArticleOperationTarget(0));
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 17.0);
	if macros::is_excute(fighter) {
		JostleModule::set_status(fighter.module_accessor, true);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
		macros::SET_SPEED_EX(fighter, 0.0, -5.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.6, 60, 95, 0, 20, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
	}
}


//		Up Taunt		//

#[acmd_script(
	agent = "toonlink",
	scripts = [ "effect_appealhil", "effect_appealhir" ],
	category = ACMD_EFFECT )]
unsafe fn knight_appealhi_effect(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("toonlink_sword1"), Hash40::new("toonlink_sword2"), 4, Hash40::new("sword1"), 0.5, 0.0, -0.5, Hash40::new("sword1"), 10.57, -0.13, -0.12, true, Hash40::new("toonlink_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.5, 0.2);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 14.0);
	if macros::is_excute(fighter) {
        macros::AFTER_IMAGE_OFF(fighter, 2);
    }
}

#[acmd_script(
	agent = "toonlink",
	scripts = [ "sound_appealhil", "sound_appealhir" ],
	category = ACMD_SOUND )]
unsafe fn knight_appealhi_sound(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_toonlink_appeal_h01"));
    }
}

//		Side Taunt		//

#[acmd_script(
	agent = "toonlink",
	scripts = [ "effect_appealsl", "effect_appealsr" ],
	category = ACMD_EFFECT )]
unsafe fn knight_appeals_effect2(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
		EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 2.3, 7, 5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 360, true, 1);
    }
}

#[acmd_script(
	agent = "toonlink_fairy",
	scripts = [ "effect_appealsl", "effect_appealsr" ],
	category = ACMD_EFFECT )]
unsafe fn knight_appeals_effect(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
		
    }
}

//		Empty ACMD's		//

#[acmd_script(
	agent = "toonlink",
	scripts = [ "effect_landingairlw", "effect_specialn" ],
	category = ACMD_EFFECT )]
unsafe fn knight_empty_effect(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
		
    }
}

pub fn install() {
	
	smashline::install_agent_frames!(
        toonlink_frame
    );
	
    smashline::install_acmd_scripts!(
		knight_jump_sound,
		
		knight_djump_sound,
	
        knight_jab12_game,

		knight_jab3_game,
		
		knight_ftilt_game,
		
		knight_utilt_game,
		knight_utilt_effect,
		
		knight_dtilt_game,
		
		knight_usmash_game,
		knight_usmash_sound,
		
		knight_fsmash_game,
		knight_fsmash_sound,
		
		knight_dsmash_game,
		knight_dsmash_sound,
		
		knight_attackairn_game,
		
		knight_attackairf_game,
		
		knight_attackairhi_game,
		
		knight_attackairlw_game,
		
		knight_nspecial_game,
		
		knight_nspecialend_game,
		
		knight_nspecialair_game,
		
		knight_nspecialairend_game,
		
		knight_sidespecial_game,
		
		knight_sidespecialair_game,
		
		knight_upspecial_game,
		
		knight_downspecial_game,
		
		knight_downspecialair_game,
		
		knight_appealhi_effect,
		knight_appealhi_sound,
		
		knight_appeals_effect,
		knight_appeals_effect2,
		
		knight_empty_effect
    );
}