use crate::{global, ClientClass, PlayerFlag, PropKind, RecvTable, Tick};
use bevy::prelude::*;
use std::ffi::{CStr, OsStr};
use std::mem::MaybeUninit;
use std::time::Duration;

pub use var::Var;

#[macro_use]
mod macros;
mod var;

fn panic(table: &'static str, var: &'static str) {
    tracing::error!("networked variable {table}.{var} was not found");
    tracing::error!("create an issue at https://github.com/elysian6969/elysium");
    tracing::error!("or join our discord https://discord.gg/4F3x3eaTDn");

    std::process::exit(1);
}

pub macro addr($base:expr, $struct:ident.$field:ident) {{
    #[allow(unused_unsafe)]
    unsafe {
        global::with_app(|app| {
            let networked = app.world.resource::<Networked>();

            networked.$struct.$field.addr($base)
        })
    }
}}

pub macro read($base:expr, $struct:ident.$field:ident) {{
    #[allow(unused_unsafe)]
    unsafe {
        global::with_app(|app| {
            let networked = app.world.resource::<Networked>();

            networked.$struct.$field.read($base)
        })
    }
}}

pub macro write($base:expr, $struct:ident.$field:ident, $value:expr) {{
    #[allow(unused_unsafe)]
    unsafe {
        global::with_app(|app| {
            let networked = app.world.resource::<Networked>();

            networked.$struct.$field.write($base, $value);
        });
    }
}}

networked! {
    (BaseCombatCharacter, base_combat_character): b"DT_BaseCombatCharacter" {
        current_weapon: i32 = b"m_hActiveWeapon",
        weapons: i32 = b"m_hMyWeapons",
        wearables: i32 = b"m_hMyWearables",
    },
    (BaseCombatWeapon, base_combat_weapon): b"DT_BaseCombatWeapon" {
        owner: i32 = b"m_hOwner",
        magazine: i32 = b"m_iClip1",
    },
    (BaseEntity, base_entity): b"DT_BaseEntity" {
        name: Option<Box<OsStr>> = b"m_iName",
        model_index: i32 = b"m_nModelIndex",
        origin: Vec3 = b"m_vecOrigin",
        render_mode: i32 = b"m_nRenderMode",
        rotation: Vec3 = b"m_angRotation",
        simulation_time: Duration = b"m_flSimulationTime",
        spotted: bool = b"m_bSpotted",
        team: i32 = b"m_iTeamNum",
    },
    (BasePlayer, base_player): b"DT_BasePlayer" {
        flags: PlayerFlag = b"m_fFlags",
        health: i32 = b"m_iHealth",
        is_dead: bool = b"deadflag",
        life_state: i32 = b"m_lifeState",
        location_name: Option<Box<OsStr>> = b"m_szLastPlaceName",
        spectator_mode: i32 = b"m_iObserverMode",
        spectator_target: i32 = b"m_hObserverTarget",
        tick_base: Tick = b"m_nTickBase",
        velocity: Vec3 = b"m_vecVelocity[0]",
    },
    (CSPlayer, cs_player): b"DT_CSPlayer" {
        aim_punch: Vec3 = b"m_aimPunchAngle",
        armor_value: i32 = b"m_ArmorValue",
        has_defuser: bool = b"m_bHasDefuser",
        has_heavy_armor: bool = b"m_bHasHeavyArmor",
        has_helmet: bool = b"m_bHasHelmet",
        has_moved_since_spawn: bool = b"m_bHasMovedSinceSpawn",
        is_defusing: bool = b"m_bIsDefusing",
        is_grabing_hostage: bool = b"m_bIsGrabbingHostage",
        is_immune: bool = b"m_bGunGameImmunity",
        is_in_bomb_area: bool = b"m_bInBombZone",
        is_in_buy_area: bool = b"m_bInBuyZone",
        is_in_hostage_rescue_area: bool = b"m_bInHostageRescueZone",
        is_not_in_defuse_area: bool = b"m_bInNoDefuseArea",
        is_rescuing: bool = b"m_bIsRescuing",
        is_scoped: bool = b"m_bIsScoped",
        lower_body_yaw: f32 = b"m_flLowerBodyYawTarget",
        max_flash_alpha: f32 = b"m_flFlashMaxAlpha",
        money: i32 = b"m_iAccount",
        survival_team: i32 = b"m_nSurvivalTeam",
        view_punch: Vec3 = b"m_viewPunchAngle",
        view_angle: Vec3 = b"m_angEyeAngles",
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
    (PlantedC4, planted_c4): b"DT_PlantedC4" {
        defuse_time: Duration = b"m_flDefuseCountDown",
        defuser: i32 = b"m_hBombDefuser",
        detonation_time: Duration = b"m_flC4Blow",
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
    },
    (WeaponCSBase, weapon_cs_base): b"DT_WeaponCSBase" {
        accuracy_penalty: f32 = b"m_fAccuracyPenalty",
        revolver_cock_time: Duration = b"m_flPostponeFireReadyTime",
    },
}
