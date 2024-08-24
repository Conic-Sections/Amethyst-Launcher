export default {
    globalSearch: {
        placeholder: "Search in Amethyst, or use commands",
    },
    sidebar: {
        game: "Game",
        settings: "Settings",
    },
    game: {
        latestRelease: "Latest Release",
        latestSnapshot: "Latest Snapshot",
        install: "Install Game",
        launch: "Launch Game",
        instances: "Instances",
        accounts: "Accounts",
        friends: "Friends",
        instanceManager: {
            view: {
                title: "Manage Instances",
                description: "Create, delete or modify your game instances",
            },
            create: {
                title: "Create Game Instances",
                description: "",
                instanceName: "Instance Name",
                nameRepeated: "Name cannot be repeated",
                createButton: "Create",
                chooseMinecraft: "Choose Minecraft Version",
            },
        },
        accountsManager: {},
        gameData: {
            saves: "Saves",
            savesCount: "No saves | One save | {count} saves",
            mods: "Mods",
            modsCount: "No mods | One mod | {count} mods",
            resourcepacks: "Resourcepacks",
            resourcepacksCount:
                "No resourcepacks | One resourcepack | {count} resourcepacks",
            shaderpacks: "Shaderpacks",
            shaderpacksCount:
                "No shaderpacks | One shaderpack | {count} shaderpacks",
            screenshots: "Screenshots",
            schematics: "Schematics",
            loading: "Loading...",
        },
        saves: {
            description:
                "You don't have any worlds, each instance uses an independent save folder. | You have one world, each instance uses an independent save folder. | You have {count} worlds, each instance uses an independent save folder.",
            showMap: "Select a world to preview it",
            allowCheat: "Allow Cheat",
            hardcore: "Hardcore",
        },
        resourcepacks: {
            description: "",
        },
    },
    settings: {
        general: {
            sidebar: "General",
            language: "Language",
            languageDesc: "It will take effect completely after reloading the page.",
            updateChannel: "Update Channel",
            updateChannelDesc: "Change update frequency",
            weekly: "Weekly",
            snapshot: "Snapshot",
            release: "Release",
            autoUpdate: "Auto Update",
            autoUpdateDesc:
                "Automatically download and install updates when available",
            checkUpdate: "Check Update",
        },
        game: {
            sidebar: "Game",
            jvmTitle: "JVM Settings",
            chooseJava: "Choose Java",
            selectedJava: "{selected}",
            addJava: "Install or Import Java",
            launchOptions: "Launch Options",
            launcherName: "Launcher Name",
            launcherNameDesc: "<i>I don't know what this is.</i>",
            processPriority: "Process Priority",
            processPriorityDesc:
                "Set the game process priority, only available on Linux.",
            processPriorityHigh: "High",
            processPriorityAboveNormal: "Above Normal",
            processPriorityNormal: "Normal",
            processPriorityBelowNormal: "Below Normal",
            processPriorityLow: "Low",
            worldName: "World Name",
            worldNameDesc: "Automatically enter the world after launch game.",
            worldNamePlaceholder: "The name of the world folder.",
            fullscreen: "Fullscreen",
            fullscreenDesc: "",
            windowSize: "Window Size",
            windowSizeDesc: "The initial size of the game window.",
            windowSizeWidth: "Width",
            windowSizeHeight: "Height",
            hideLauncherAfterLaunch: "Hide launcher after launch game",
            demo: "Demo mode",
            demoDesc:
                "Try the game for 100 minutes (5 game days) in a single world. If you haven't purchased the game, this option is forcefully enabled.",
        },
        advance: {
            sidebar: "Advance",
            exportLauncherLogs: "Export Launcher Log",
            exportLauncherLogsDesc:
                "Export launcher log to file for reporting problems.",
            reload: "Reload Window",
            launchArgs: "Advance Launch Options",
            gc: "GC",
            extraJVMArgs: "Extra JVM Arguments",
            extraJVMArgsDesc:
                "This will be put at the end of the default JVM arguments.",
            extraMinecraftArgs: "Extra Minecraft Arguments",
            extraMinecraftArgsDesc:
                "This will be put at the end of the default Minecraft arguments.",
            extraClassPaths: "Extra Class Paths",
            extraClassPathsDesc:
                "This will be put at the end of the default class paths.",
            executeBeforeLaunch: "Pre-launch Command",
            executeBeforeLaunchDesc:
                "This will be put at the beginning of the launch script.",
            wrapCommand: "Wrapper Command",
            wrapCommandDesc:
                "This will be put at the beginning of the launch command.",
            executeAfterLaunch: "Post-exit Command",
            executeAfterLaunchDesc:
                "This will be put at the end of the launch script.",
            ignoreInvalidMinecraftCertificates:
                "Ignore invalid Minecraft certificates",
            ignoreInvalidMinecraftCertificatesDesc:
                "Add <code>-Dfml.ignoreInvalidMinecraftCertificates=true</code> to JVM arguments.",
            ignorePatchDiscrepancies: "Ignore Path Discrepancies",
            ignorePatchDiscrepanciesDesc:
                "Add <code>-Dfml.ignorePatchDiscrepancies=true</code> to JVM arguments.",
            lwjglSettings: "LWJGL Settings",
        },
        appearance: {
            sidebar: "Appearance",
        },
        download: {
            sidebar: "Download",
            maxConnections: "Max Connections",
            maxConnectionsDesc:
                "<strong>Too many connections may decrease the download speed,</strong> and your download speed may be affected by your ISP and download servers.",
            maxDownloadSpeed: "Limit Download Speed",
            maxDownloadSpeedDesc:
                "Set the limit of download speed in B/s. 0 for no limit.",
            mirrorServer: "Mirror Server",
            mirrorServerDesc: "Download game files from mirror server.",
            proxy: "Proxy",
            useSystemProxy: "Use System Proxy",
        },
        accessibility: {
            sidebar: "Easy to Use",
            extraFeatures: "Extra Features",
            releaseReminder: "Release Reminder",
            releaseReminderDesc: "Remind me when the release is updated",
            snapshotReminder: "Snapshot Reminder",
            snapshotReminderDesc: "Remind me when the snapshot is updated",
            hideLatestRelease: 'Hide "Latest Release"',
            hideLatestReleaseDesc: 'Hide "Latest Release" instance',
            hideLatestSnapshot: 'Hide "Latest Snapshot"',
            hideLatestSnapshotDesc: 'Hide "Latest Snapshot" instance',
            changeGameLanguage: "Auto Change Game Language",
            changeGameLanguageDesc:
                "Change the game language to the same as the launcher's language when launching the game at first time.",
            accessibility: "Accessibility",
            disableAllAnimations: "Disable All Animations",

            disableAllAnimationsDesc:
                "Blinking and flashing animation can be problematic for people with cognitive concerns such as Attention Deficit Hyperactivity Disorder (ADHD). Additionally, certain kinds of motion can be a trigger for Vestibular disorders, epilepsy, and migraine and Scotopic sensitivity.",
            highContrastMode: "High Contrast Mode",
            highContrastModeDesc:
                "Enable high contrast mode for current theme (if it's available).",
        },
        extend: {
            sidebar: "Extend",
        },
        about: {
            sidebar: "About",
            report: "Report an Issue",
            reportDesc: "Open an issue on GitHub, please upload launcher log.",
            sponsorTitle: "Amethyst Sponsor",
            sponsorDesc:
                "Amethyst Launcher is a free, open-source, cross-platform Minecraft launcher. The program is built by Broken_Deer and other contributors. Your support will help us.",
            thanks: "Thanks",
            thirdPartyLibraries: "Third-Party Libraries",
        },
    },
};