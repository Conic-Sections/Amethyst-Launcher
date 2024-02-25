/*
 * Amethyst Launcher Core
 * Copyright (C) 2023 Broken-Deer <old_driver__@outlook.com> and contributors
 *
 * This program is free software, you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ForgeVersionListItem {
    pub _id: String,
    pub build: u32,
    pub __v: u32,
    pub version: String,
    pub modified: String,
    pub mcversion: String,
    pub files: Vec<ForgeInstallerFile>,
    pub branch: Option<Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ForgeInstallerFile {
    pub format: String,
    pub category: String,
    pub hash: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ForgeVersionList(Vec<ForgeVersionListItem>);

impl ForgeVersionList {
    pub async fn new() -> Result<Self> {
        Ok(reqwest::get("https://bmclapi2.bangbang93.com/forge/list/0")
            .await?
            .json::<Self>()
            .await?)
    }

    pub async fn from_mcversion(mcversion: &str) -> Result<Self> {
        Ok(reqwest::get(format!(
            "https://bmclapi2.bangbang93.com/forge/minecraft/{mcversion}"
        ))
        .await?
        .json::<Self>()
        .await?)
    }
}

// #[tokio::test]
// async fn test123123123() {
//     // let minecraft_version = "1.20.1"
//     let forge_version_list = ForgeVersionList::new().await.unwrap();
//     let a = serde_json::to_string_pretty(&forge_version_list).unwrap();
//     tokio::fs::write("1.json", a).await.unwrap();
//     println!("{:#?}", forge_version_list);
// }
