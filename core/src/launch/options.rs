// Conic Launcher
// Copyright 2022-2026 Broken-Deer and contributors. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-only

use crate::{
    account::Account,
    config::{
        launch::{Server, GC},
        read_config_file,
    },
    folder::MinecraftLocation,
    instance::Instance,
    DATA_LOCATION,
};

pub struct GameProfile {
    pub name: String,
    pub uuid: String,
}

pub struct LaunchOptions {
    /// User selected game profile.
    ///
    /// For game display name & uuid
    pub(crate) game_profile: GameProfile,

    pub(crate) properties: String,
    pub(crate) access_token: String,

    /// Min memory, this will add a jvm flag -XMS to the command result
    pub(crate) min_memory: usize,

    /// Max memory, this will add a jvm flag -Xmx to the command result
    pub(crate) max_memory: usize,

    /// Directly launch to a server. TODO: support 1.21.1
    pub(crate) server: Option<Server>,

    /// window width
    pub(crate) width: usize,

    /// window height
    pub(crate) height: usize,

    pub(crate) fullscreen: bool,

    /// User custom additional java virtual machine command line arguments.
    ///
    /// If this is empty, the `DEFAULT_EXTRA_JVM_ARGS` will be used.
    pub(crate) extra_jvm_args: String,

    /// User custom additional minecraft command line arguments.
    pub(crate) extra_mc_args: String,

    pub(crate) is_demo: bool,

    /// Add `-Dfml.ignoreInvalidMinecraftCertificates=true` to jvm argument
    pub(crate) ignore_invalid_minecraft_certificates: bool,

    /// Add `-Dfml.ignorePatchDiscrepancies=true` to jvm argument
    pub(crate) ignore_patch_discrepancies: bool,

    /// Add extra classpath
    pub(crate) extra_class_paths: String,

    /// Add other features flags
    pub(crate) extra_enabled_features: Vec<String>,

    // /// TODO: Support yushi's yggdrasil agent <https://github.com/to2mbn/authlib-injector/wiki>
    // pub(crate) yggdrasil_agent: Option<YggdrasilAgent>,
    pub(crate) gc: GC,

    pub(crate) minecraft_location: MinecraftLocation,
    pub(crate) launcher_name: String,

    pub wrap_command: String,

    pub execute_before_launch: String,

    pub execute_after_launch: String,
}

impl LaunchOptions {
    pub fn new(instance: &Instance, account: Account) -> Self {
        let global_config = read_config_file().launch;
        let launch_config = &instance.config.launch_config;
        Self {
            wrap_command: launch_config
                .wrap_command
                .clone()
                .unwrap_or(global_config.wrap_command),
            execute_before_launch: launch_config
                .execute_before_launch
                .clone()
                .unwrap_or(global_config.execute_before_launch),
            execute_after_launch: launch_config
                .execute_after_launch
                .clone()
                .unwrap_or(global_config.execute_after_launch),
            launcher_name: launch_config
                .launcher_name
                .clone()
                .unwrap_or(global_config.launcher_name),
            game_profile: GameProfile {
                name: account.profile.profile_name.clone(),
                uuid: account.profile.uuid.clone(),
            },
            access_token: account.access_token.clone().unwrap_or_default(),
            min_memory: launch_config.min_memory.unwrap_or(global_config.min_memory),
            max_memory: launch_config.max_memory.unwrap_or(global_config.max_memory),
            // TODO:
            server: launch_config.server.clone(),
            width: launch_config.width.unwrap_or(global_config.width),
            height: launch_config.height.unwrap_or(global_config.height),
            fullscreen: launch_config.fullscreen.unwrap_or(global_config.fullscreen),
            extra_jvm_args: launch_config
                .extra_jvm_args
                .clone()
                .unwrap_or(global_config.extra_jvm_args),
            extra_mc_args: launch_config
                .extra_mc_args
                .clone()
                .unwrap_or(global_config.extra_mc_args),
            is_demo: launch_config.is_demo.unwrap_or(global_config.is_demo),
            ignore_invalid_minecraft_certificates: launch_config
                .ignore_invalid_minecraft_certificates
                .unwrap_or(global_config.ignore_invalid_minecraft_certificates),
            ignore_patch_discrepancies: launch_config
                .ignore_patch_discrepancies
                .unwrap_or(global_config.ignore_patch_discrepancies),
            extra_class_paths: launch_config
                .extra_class_paths
                .clone()
                .unwrap_or(global_config.extra_class_paths),
            extra_enabled_features: vec![],
            gc: launch_config.gc.clone().unwrap_or(global_config.gc),
            minecraft_location: MinecraftLocation::new(&DATA_LOCATION.root),
            properties: "{}".to_string(),
        }
    }

    pub fn get_enabled_features(&self) -> Vec<String> {
        let mut result = vec!["has_custom_resolution".to_string()];
        if self.is_demo {
            result.push("is_demo_user".to_string())
        }
        result.extend(self.extra_enabled_features.clone());
        result
    }
}
