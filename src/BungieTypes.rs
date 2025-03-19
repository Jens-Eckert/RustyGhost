use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct GenericResponse<T: Clone> {
    pub Response: Option<T>,
    pub ErrorCode: i32,
    pub ThrottleSeconds: i32,
    pub ErrorStatus: String,
    pub Message: String,
    pub MessageData: HashMap<String, String>,
    pub DetailedErrorTrace: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Destiny2Manifest {
    pub version: String,
    pub mobileAssetContentPath: String,
    pub mobileGearAssetDataBases: Vec<Destiny2ConfigGearAssetDataBaseDefinition>,
    pub mobileWorldContentPaths: HashMap<String, String>,
    pub jsonWorldContentPaths: HashMap<String, String>,
    pub jsonWorldComponentContentPaths: HashMap<String, HashMap<String, String>>,
    pub mobileClanBannerDatabasePath: String,
    pub mobileGearCDN: HashMap<String, String>,
    pub iconImagePyramidInfo: Vec<Destiny2ConfigImagePyramidEntry>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Destiny2ConfigGearAssetDataBaseDefinition {
    pub version: i32,
    pub path: String,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Destiny2ConfigImagePyramidEntry {
    pub name: String,
    pub factor: f32,
}
