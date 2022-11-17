use crate::{global, ClientClass, PlayerFlag, PropKind, RecvTable};
use bevy::prelude::*;
use std::ffi::{CStr, OsStr};
use std::mem::MaybeUninit;
use std::slice;
use std::time::Duration;

pub use var::Var;

#[macro_use]
mod macros;
mod var;

fn panic(class: &'static str, var: &'static str) {
    panic!("networked variable {class}.{var} was not found");
}

pub macro read($class:ident, $struct:ident.$field:ident) {
    unsafe {
        global::with_app(|networked| {
            networked.$struct.$field.read($class);
        });
    }
}

pub macro write($class:ident, $struct:ident.$field:ident, $value:expr) {
    unsafe {
        global::with_app(|networked| {
            networked.$struct.$field.write($class, $value);
        });
    }
}

networked! {
    (BaseCombatCharacter, base_combat_character): b"DT_BaseCombatCharacter" {
        current_weapon: i32 = b"m_hActiveWeapon",
        weapons: i32 = b"m_hMyWeapons",
        wearables: i32 = b"m_hMyWearables",
    },
    (BaseCombatWeapon, base_combat_weapon): b"DT_BaseCombatWeapon" {
        owner: i32 = b"m_hOwner",
        magazine: i32 = b"m_iClip1",
        item_id_hi: i32 = b"m_iItemIDHigh",
        account_id: i32 = b"m_iAccountID",
        paint_kit: i32 = b"m_nFallbackPaintKit",
        wear: f32 = b"m_flFallbackWear",
        stat_track: i32 = b"m_nFallbackStatTrak",
        next_primary_attack: Duration = b"m_flNextPrimaryAttack",
        next_secondary_attack: Duration = b"m_flNextSecondaryAttack",
    },
    (BaseEntity, base_entity): b"DT_BaseEntity" {
        animation_time: Duration = b"m_flAnimTime",
        collision: i32 = b"m_Collision",
        name: Option<Box<OsStr>> = b"m_iName",
        model_index: i32 = b"m_nModelIndex",
        origin: Vec3 = b"m_vecOrigin",
        owner: i32 = b"m_hOwnerEntity",
        pending_team: i32 = b"m_iPendingTeamNum",
        render_mode: i32 = b"m_nRenderMode",
        rotation: Vec3 = b"m_angRotation",
        simulation_time: Duration = b"m_flSimulationTime",
        spotted: bool = b"m_bSpotted",
        team: i32 = b"m_iTeamNum",
    },
    (BasePlayer, base_player): b"DT_BasePlayer" {
        cycle: i32 = b"m_flCycle",
        eye_offset: Vec3 = b"m_vecViewOffset[0]",
        health: i32 = b"m_iHealth",
        is_dead: bool = b"deadflag",
        flags: PlayerFlag = b"m_fFlags",
        life_state: i32 = b"m_lifeState",
        location_name: Option<Box<OsStr>> = b"m_szLastPlaceName",
        model_scale: f32 = b"m_flModelScale",
        pose_parameters: f32 = b"m_flPoseParameter",
        sequence: i32 = b"m_nSequence",
        skin: i32 = b"m_nSkin",
        spectator_mode: i32 = b"m_iObserverMode",
        spectator_target: i32 = b"m_hObserverTarget",
        tick_base: u32 = b"m_nTickBase",
        velocity: Vec3 = b"m_vecVelocity[0]",
    },
    (CSPlayer, cs_player): b"DT_CSPlayer" {
        aim_punch: Vec3 = b"m_aimPunchAngle",
        armor_value: i32 = b"m_ArmorValue",
        flash_alpha: bool = b"m_flFlashMaxAlpha",
        has_defuser: bool = b"m_bHasDefuser",
        has_helmet: bool = b"m_bHasHelmet",
        is_immune: bool = b"m_bGunGameImmunity",
        is_scoped: bool = b"m_bIsScoped",
        lower_body_yaw: f32 = b"m_flLowerBodyYawTarget",
        money: i32 = b"m_iAccount",
        survival_team: i32 = b"m_nSurvivalTeam",
        view_punch: Vec3 = b"m_viewPunchAngle",
    },
    (CSPlayerResource, csplayer_resource): b"DT_CSPlayerResource" {
        active_coin_rank: i32 = b"m_nActiveCoinRank",
        bomb_carrier: i32 = b"m_iPlayerC4",
        bombsite_a_origin: Vec3 = b"m_bombsiteCenterA",
        bombsite_b_origin: Vec3 = b"m_bombsiteCenterB",
        bot_difficulty: i32 = b"m_iBotDifficulty",
        clan: i32 = b"m_szClan",
        competitive_ranking: i32 = b"m_iCompetitiveRanking",
        competitive_wins: i32 = b"m_iCompetitiveWins",
        controlling_bot: bool = b"m_bControllingBot",
        controlled_player: i32 = b"m_iControlledPlayer",
        controlled_by_player: i32 = b"m_iControlledByPlayer",
        has_helmet: bool = b"m_bHasHelmet",
        has_defuser: bool = b"m_bHasDefuser",
        hostage_rescue_origin: Vec3 = b"m_hostageRescueX",
        level: i32 = b"m_nPersonaDataPublicLevel",
        money_spent: i32 = b"m_iTotalCashSpent",
        money_spent_round: i32 = b"m_iCashSpentThisRound",
        music_id: i32 = b"m_nMusicID",
        mvps: i32 = b"m_iMVPs",
        next_map: i32 = b"m_bEndMatchNextMapAllVoted",
        next_map_votes: i32 = b"m_nEndMatchNextMapVotes",
        score: i32 = b"m_iScore",
        vip: i32 = b"m_iPlayerVIP",
    },
    (EnvTonemapController, env_tonemap_controller): b"DT_EnvTonemapController" {
        bloom_scale: f32 = b"m_flCustomBloomScale",
        bloom_scale_enabled: bool = b"m_bUseCustomBloomScale",
        exposure_end: f32 = b"m_flCustomAutoExposureMax",
        exposure_end_enabled: bool = b"m_bUseCustomAutoExposureMax",
        exposure_start: f32 = b"m_flCustomAutoExposureMin",
        exposure_start_enabled: bool = b"m_bUseCustomAutoExposureMin",
    },
    (FogController, fog_controller): b"DT_FogController" {
        alpha: f32 = b"m_fog.maxdensity",
        clip_distance: f32 = b"m_fog.farz",
        end: f32 = b"m_fog.end",
        is_enabled: bool = b"m_fog.enable",
        rgb: [u8; 3] = b"m_fog.colorPrimary",
        start: f32 = b"m_fog.start",
    },
    (PlayerResource, player_resource): b"DT_PlayerResource" {
        alive: bool = b"m_bAlive",
        assists: i32 = b"m_iAssists",
        connected: bool = b"m_bConnected",
        deaths: bool = b"m_iDeaths",
        health: i32 = b"m_iHealth",
        kills: i32 = b"m_iKills",
        pending_team: i32 = b"m_iPendingTeam",
        ping: i32 = b"m_iPing",
        team: i32 = b"m_iTeam",
    },
    (PlantedC4, planted_c4): b"DT_PlantedC4" {
        defuse_time: Duration = b"m_flDefuseCountDown",
        defuser: i32 = b"m_hBombDefuser",
        detonation_time: Duration = b"m_flC4Blow",
    },
    (WeaponCSBase, weapon_cs_base): b"DT_WeaponCSBase" {
        accuracy_penalty: f32 = b"m_fAccuracyPenalty",
        revolver_cock_time: Duration = b"m_flPostponeFireReadyTime",
    },
}
