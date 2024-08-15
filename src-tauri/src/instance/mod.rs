use crate::core::{folder::MinecraftLocation, version::VersionManifest, Download};
use crate::{
    game_data::mods::ResolvedMod,
    game_data::saves::level::LevelData,
    install::{
        fabric::LoaderArtifactList,
        generate_download_info,
        quilt::{version_list::get_quilt_version_list_from_mcversion, QuiltVersion},
    },
};
use crate::{Storage, DATA_LOCATION, HTTP_CLIENT, MAIN_WINDOW};
use base64::{engine::general_purpose, Engine};
use futures::StreamExt;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::Duration;
use tauri::Emitter;
use tauri_plugin_http::reqwest;
use tokio::{fs, io::AsyncWriteExt};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InstanceConfig {
    pub name: String,
    pub runtime: InstanceRuntime,
    pub group: Option<Vec<String>>,
}

impl InstanceConfig {
    pub fn new(instance_name: &str, minecraft_version: &str) -> Self {
        Self {
            name: instance_name.to_string(),
            runtime: InstanceRuntime {
                minecraft: minecraft_version.to_string(),
                fabric: "".to_string(),
                forge: "".to_string(),
                quilt: "".to_string(),
                optifine: "".to_string(),
            },
            group: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InstanceRuntime {
    pub minecraft: String,
    pub fabric: String,
    pub forge: String,
    pub quilt: String,
    pub optifine: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Instance {
    pub config: InstanceConfig,
    pub installed: bool,
}

#[tauri::command(async)]
pub async fn get_minecraft_version_list() -> Option<VersionManifest> {
    VersionManifest::new().await.ok()
}

#[tauri::command(async)]
pub async fn get_fabric_version_list(mcversion: String) -> Option<LoaderArtifactList> {
    LoaderArtifactList::from_mcversion(&mcversion).await.ok()
}

#[tauri::command(async)]
pub async fn get_forge_version_list(_mcversion: String) -> Option<LoaderArtifactList> /* Option<ForgeVersionList> */
{
    // ForgeVersionList::from_mcversion(&mcversion).await.ok()
    None
}

#[tauri::command(async)]
pub async fn get_quilt_version_list(mcversion: String) -> Option<Vec<QuiltVersion>> {
    get_quilt_version_list_from_mcversion(None, &mcversion)
        .await
        .ok()
}

#[tauri::command(async)]
pub async fn create_instance(instance_name: String, config: InstanceConfig) -> Option<()> {
    async fn create_instance(instance_name: String, config: InstanceConfig) -> anyhow::Result<()> {
        let data_location = DATA_LOCATION.get().unwrap();
        let instance_root = data_location.get_instance_root(&instance_name);
        let config_file = instance_root.join("instance.toml");
        fs::create_dir_all(config_file.parent().ok_or(anyhow::anyhow!("Path Error"))?).await?;
        fs::write(config_file, toml::to_string_pretty(&config)?).await?;
        Ok(())
    }
    create_instance(instance_name, config).await.ok()
}

#[tauri::command(async)]
pub async fn check_repeated_instance_name(instance_name: String) -> bool {
    let instance_root = DATA_LOCATION
        .get()
        .unwrap()
        .get_instance_root(&instance_name);
    let config = match get_instance_config_from_string(&instance_name).await {
        Ok(x) => x,
        Err(_) => return false,
    };
    let folder_name = match instance_root.file_name() {
        None => return true,
        Some(x) => x,
    }
    .to_string_lossy()
    .to_string();
    (config.name == instance_name) || (folder_name == instance_name)
}

#[tauri::command(async)]
pub async fn scan_instances_folder() -> Option<Vec<Instance>> {
    println!("Scanning Instances Folder...");
    async fn scan() -> anyhow::Result<Vec<Instance>> {
        let datafolder_path = DATA_LOCATION.get().unwrap();
        let instances_folder = &datafolder_path.instances;
        let mut folder_entries = tokio::fs::read_dir(instances_folder).await?;
        let mut results = Vec::new();
        while let Some(entry) = folder_entries.next_entry().await? {
            let file_type = match entry.file_type().await {
                Err(_) => continue,
                Ok(file_type) => file_type,
            };
            if !file_type.is_dir() {
                continue;
            }
            let path = entry.path();
            let folder_name = match path.file_name() {
                None => continue,
                Some(x) => x,
            }
            .to_string_lossy()
            .to_string();
            let instance_config = path.join("instance.toml");
            let metadata = match instance_config.metadata() {
                Err(_) => continue,
                Ok(result) => result,
            };
            if metadata.len() > 2000000 || !instance_config.is_file() {
                continue;
            }
            let config_content = match fs::read_to_string(instance_config).await {
                Err(_) => continue,
                Ok(content) => content,
            };
            let config = match toml::from_str::<InstanceConfig>(&config_content) {
                Ok(config) => config,
                Err(_) => continue,
            };
            let fabric = config.runtime.fabric.as_str();
            let forge = config.runtime.forge.as_str();
            let quilt = config.runtime.quilt.as_str();
            if (!fabric.is_empty() && !forge.is_empty())
                || (!forge.is_empty() && !quilt.is_empty())
                || (!quilt.is_empty() && !fabric.is_empty())
            {
                continue;
            }
            if folder_name != config.name {
                continue;
            }
            results.push(Instance {
                config,
                installed: fs::File::open(path.join(".aml-ok")).await.is_ok(),
            })
        }
        println!("Done");
        Ok(results)
    }
    scan().await.ok()
}
// TODO: watcher
//
// #[tauri::command]
// pub fn watch_instances_folder() {
//     let main_window = MAIN_WINDOW.get().unwrap().clone();
//     println!("Watching instances folder...");
//     thread::spawn(move || {
//         let (tx, rx) = channel();
//
//         // Create a watcher object, delivering debounced events.
//         // The notification back-end is selected based on the platform.
//         let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();
//
//         std::fs::create_dir_all(&DATA_LOCATION.get().unwrap().instances).unwrap();
//         // Add a path to be watched. All files and directories at that path and
//         // below will be monitored for changes.
//         watcher
//             .watch(
//                 &DATA_LOCATION.get().unwrap().instances,
//                 RecursiveMode::Recursive,
//             )
//             .unwrap();
//
//         loop {
//             match rx.recv() {
//                 Ok(event) => {
//                     main_window
//                         .emit("instances_changed", format!("{:#?}", event))
//                         .unwrap();
//                 }
//                 Err(e) => println!("watch error: {:?}", e),
//             }
//         }
//     });
// }

#[tauri::command]
pub fn set_current_instance(instance_name: String, storage: tauri::State<Storage>) {
    let mut current_instance = storage.current_instance.lock().unwrap();
    println!("Setting current instance to {}", instance_name);
    *current_instance = instance_name;
}

#[tauri::command(async)]
pub async fn scan_mod_folder(
    storage: tauri::State<'_, Storage>,
) -> std::result::Result<Vec<ResolvedMod>, ()> {
    let instance_name = storage.current_instance.lock().unwrap().clone();

    async fn scan(
        instance_name: String,
        storage: tauri::State<'_, Storage>,
    ) -> anyhow::Result<Vec<ResolvedMod>> {
        let data_location = DATA_LOCATION.get().unwrap();
        let mod_packs_root = data_location.get_modpacks_root(&instance_name);

        fs::create_dir_all(&mod_packs_root).await?;

        let mut folder_entries = tokio::fs::read_dir(mod_packs_root).await?;
        let mut results = Vec::new();
        while let Some(entry) = folder_entries.next_entry().await? {
            let file_type = match entry.file_type().await {
                Err(_) => continue,
                Ok(file_type) => file_type,
            };
            let active_instance = storage.current_instance.lock().unwrap().clone();
            if active_instance != instance_name {
                return Err(anyhow::anyhow!("stopped")); // if user change the active instance, stop scanning
            }
            if !file_type.is_file() {
                continue;
            }
            let path = entry.path();
            if path.metadata().is_err() {
                continue;
            }
            let parser_task =
                tokio::task::spawn_blocking(|| crate::game_data::mods::parse_mod(path));

            results.push(match parser_task.await {
                Err(_) => continue,
                Ok(result) => match result {
                    Err(_) => continue,
                    Ok(result) => result,
                },
            });
        }
        Ok(results)
    }
    match scan(instance_name, storage).await {
        Ok(results) => Ok(results
            .into_iter()
            .filter(|v| v.version.is_some())
            .collect()),
        Err(_) => Err(()),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Saves {
    pub icon: String,
    pub level_data: LevelData,
    pub dir_name: String,
}

#[tauri::command(async)]
pub async fn scan_saves_folder(
    storage: tauri::State<'_, Storage>,
) -> std::result::Result<Vec<Saves>, ()> {
    let instance_name = storage.current_instance.lock().unwrap().clone();

    async fn scan(
        instance_name: String,
        storage: tauri::State<'_, Storage>,
    ) -> anyhow::Result<Vec<Saves>> {
        let data_location = DATA_LOCATION.get().unwrap();
        let saves_root = data_location.get_saves_root(&instance_name);

        fs::create_dir_all(&saves_root).await?;

        let mut folder_entries = tokio::fs::read_dir(saves_root).await?;
        let mut results = Vec::new();
        while let Some(entry) = folder_entries.next_entry().await? {
            let file_type = match entry.file_type().await {
                Err(_) => continue,
                Ok(file_type) => file_type,
            };
            let active_instance = storage.current_instance.lock().unwrap().clone();
            if active_instance != instance_name {
                return Err(anyhow::anyhow!("stopped")); // if user change the active instance, stop scanning
            }
            if !file_type.is_dir() {
                continue;
            }
            let path = entry.path();
            if path.metadata().is_err() {
                continue;
            }
            let level_path = path.join("level.dat");
            let parser_task = tokio::task::spawn_blocking(|| {
                crate::game_data::saves::level::get_level_data(level_path)
            });
            let icon_path = path.join("icon.png");
            let icon = match fs::read(icon_path).await {
                Err(_) => "".to_string(),
                Ok(content) => format!(
                    "data:image/png;base64,{}",
                    general_purpose::STANDARD_NO_PAD.encode(content)
                ),
            };
            let level_data = match parser_task.await {
                Err(_) => continue,
                Ok(result) => match result {
                    Err(_) => continue,
                    Ok(result) => result,
                },
            };
            results.push(Saves {
                icon,
                level_data,
                dir_name: path.file_name().unwrap().to_string_lossy().to_string(),
            });
        }
        Ok(results)
    }
    match scan(instance_name, storage).await {
        Ok(results) => Ok(results),
        Err(_) => Err(()),
    }
}

#[tauri::command(async)]
pub async fn get_instance_config(
    storage: tauri::State<'_, Storage>,
) -> std::result::Result<InstanceConfig, ()> {
    let instance_name = storage.current_instance.lock().unwrap().clone();
    let config_path = DATA_LOCATION
        .get()
        .unwrap()
        .get_instance_root(&instance_name)
        .join("instance.toml");
    match config_path.metadata() {
        Ok(_) => {
            let config_content = match fs::read_to_string(config_path).await {
                Err(_) => return Err(()),
                Ok(content) => content,
            };
            match toml::from_str::<InstanceConfig>(&config_content) {
                Ok(config) => Ok(config),
                Err(_) => Err(()),
            }
        }
        Err(_) => Err(()),
    }
}

#[tauri::command(async)]
pub async fn get_instance_config_from_string(instance_name: &str) -> Result<InstanceConfig, ()> {
    let config_path = DATA_LOCATION
        .get()
        .unwrap()
        .get_instance_root(instance_name)
        .join("instance.toml");
    match config_path.metadata() {
        Ok(_) => {
            let config_content = match fs::read_to_string(config_path).await {
                Err(_) => return Err(()),
                Ok(content) => content,
            };
            match toml::from_str::<InstanceConfig>(&config_content) {
                Ok(config) => Ok(config),
                Err(_) => Err(()),
            }
        }
        Err(_) => Err(()),
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DownloadProgress {
    pub completed: usize,
    pub total: usize,
    pub step: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DownloadError {
    pub step: usize,
}

async fn download_files(downloads: Vec<Download>) {
    let main_window = MAIN_WINDOW.get().unwrap();
    main_window
        .emit(
            "install_progress",
            DownloadProgress {
                completed: 0,
                total: 0,
                step: 2,
            },
        )
        .unwrap();
    let counter: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
    let downloads: Vec<_> = downloads
        .into_par_iter()
        .filter(|download| {
            if std::fs::metadata(&download.file).is_err() {
                return true;
            }
            let mut file = match std::fs::File::open(&download.file) {
                Ok(file) => file,
                Err(_) => {
                    return true;
                }
            };
            if download.sha1.is_none() {
                return false;
            };
            let file_hash = calculate_sha1_from_read(&mut file);
            counter.clone().fetch_add(1, Ordering::SeqCst);
            main_window
                .emit(
                    "install_progress",
                    DownloadProgress {
                        completed: counter.load(Ordering::SeqCst),
                        total: 0,
                        step: 2,
                    },
                )
                .unwrap();
            &file_hash != download.sha1.as_ref().unwrap()
        })
        .collect();
    let counter: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
    let total = downloads.len();
    let client = HTTP_CLIENT.get().unwrap();
    let speed_counter: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
    let speed_counter_clone = speed_counter.clone();
    let (tx, rx) = mpsc::channel();
    let speed_thread = thread::spawn(move || {
        let counter = speed_counter_clone;
        loop {
            let message = rx.try_recv();
            if message == Ok("terminate") {
                break;
            }
            thread::sleep(Duration::from_millis(2000));
            main_window
                .emit("download_speed", counter.load(Ordering::SeqCst))
                .unwrap();
            counter.store(0, Ordering::SeqCst);
        }
    });

    futures::stream::iter(downloads)
        .map(|task| {
            let counter = counter.clone();
            let speed_counter = speed_counter.clone();
            async move {
                let mut retries = 0;
                let task = task;
                loop {
                    let speed_counter = speed_counter.clone();
                    if download_file(client, &task, &counter, &speed_counter).await.is_ok() {
                        break;
                    }
                    println!("Downloaded failed: {}, retrying...", &task.url);
                    if retries >= 5 {
                        MAIN_WINDOW
                            .get()
                            .unwrap()
                            .emit("install_error", "")
                            .unwrap();
                        break;
                    }
                    retries += 1;
                }
            }
        })
        .buffer_unordered(100)
        .for_each_concurrent(None, |_| async {
            let counter = counter.clone().load(Ordering::SeqCst);
            // println!("Progress: {counter} / {total}");
            main_window
                .emit(
                    "install_progress",
                    DownloadProgress {
                        completed: counter,
                        total,
                        step: 3,
                    },
                )
                .unwrap();
        })
        .await;
    tx.send("terminate").unwrap();
    speed_thread.join().unwrap();
}

pub async fn download_file(
    client: &reqwest::Client,
    task: &Download,
    counter: &Arc<AtomicUsize>,
    speed_counter: &Arc<AtomicUsize>,
) -> anyhow::Result<()> {
    let file_path = task.file.clone();
    fs::create_dir_all(file_path.parent().ok_or(anyhow::Error::msg(
        "Unknown Error in instance/mod.rs".to_string(),
    ))?)
    .await?;
    let mut response = client.get(task.url.clone()).send().await?;
    let mut file = fs::File::create(&file_path).await?;
    while let Some(chunk) = response.chunk().await? {
        file.write_all(&chunk).await?;
        speed_counter.fetch_add(chunk.len(), Ordering::SeqCst);
    }
    counter.fetch_add(1, Ordering::SeqCst);
    Ok(())
}

#[tauri::command(async)]
pub async fn install(storage: tauri::State<'_, Storage>) -> std::result::Result<(), ()> {
    let main_window = MAIN_WINDOW.get().unwrap();
    main_window
        .emit(
            "install_progress",
            DownloadProgress {
                completed: 0,
                total: 0,
                step: 1,
            },
        )
        .unwrap();
    let active_instance = storage.current_instance.lock().unwrap().clone();
    let data_location = DATA_LOCATION.get().unwrap();
    let instance_config = match get_instance_config_from_string(&active_instance).await {
        Ok(x) => x,
        Err(_) => {
            main_window
                .emit("install_error", DownloadError { step: 1 })
                .unwrap();
            return Err(());
        }
    };

    let runtime = instance_config.runtime;
    let download_list = match generate_download_info(
        &runtime.minecraft,
        MinecraftLocation::new(&data_location.root),
    )
    .await
    {
        Ok(x) => x,
        Err(_) => {
            main_window
                .emit("install_error", DownloadError { step: 1 })
                .unwrap();
            return Err(());
        }
    };

    download_files(download_list).await;
    let mut lock_file = fs::File::create(
        data_location
            .get_instance_root(active_instance)
            .join(".aml-ok"),
    )
    .await
    .unwrap();
    lock_file.write_all(b"ok").await.unwrap();
    main_window.emit("install_success", "").unwrap();
    Ok(())
}

pub async fn update_latest_instance() {
    let version_list = VersionManifest::new().await;
    if version_list.is_err() {
        return;
    };
    let version_list = version_list.unwrap();
    if !check_repeated_instance_name("Latest Release".to_string()).await {
        create_instance(
            "Latest Release".to_string(),
            InstanceConfig::new("Latest Release", &version_list.latest.release),
        )
        .await;
    };
    if !check_repeated_instance_name("Latest Snapshot".to_string()).await {
        create_instance(
            "Latest Snapshot".to_string(),
            InstanceConfig::new("Latest Snapshot", &version_list.latest.snapshot),
        )
        .await;
    };
}

fn calculate_sha1_from_read<R: Read>(source: &mut R) -> String {
    let mut hasher = sha1_smol::Sha1::new();
    let mut buffer = [0; 1024];
    loop {
        let bytes_read = source.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }
    hasher.digest().to_string()
}
