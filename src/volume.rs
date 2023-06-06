use std;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct UsageData {
    pub Size: i64,
    pub RefCount: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct ClusterVolume {
    pub Debug: String,
    pub CreatedAt: String,
    pub UpdatedAt: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Volume {
    pub Name: String,
    pub Driver: String,
    pub Mountpoint: String,
    pub CreatedAt: String,
    pub Labels: Option<HashMap<String, String>>,
    pub Scope: String,
    pub ClusterVolume: ClusterVolume,
    pub Options: DriverOpts,
    pub UsageData: UsageData,
}

impl Clone for Volume {
    fn clone(&self) -> Self {
        Volume {
            Name: self.Name.clone(),
            Scope: self.Scope.clone(),
            Driver: self.Driver.clone(),
            Options: self.Options.clone(),
            Labels: self.Labels.clone(),
            Mountpoint: self.Mountpoint.clone(),
            CreatedAt: self.CreatedAt.clone(),
            ClusterVolume: self.ClusterVolume.clone(),
            UsageData: self.UsageData.clone(),
        }
    }
}

impl Clone for ClusterVolume {
    fn clone(&self) -> Self {
        ClusterVolume {
            Debug: self.Debug.clone(),
            CreatedAt: self.CreatedAt.clone(),
            UpdatedAt: self.UpdatedAt.clone(),
        }
    }
}

impl Clone for UsageData {
    fn clone(&self) -> Self {
        UsageData {
            Size: self.Size,
            RefCount: self.RefCount,
        }
    }
}

impl std::fmt::Display for Volume {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.Name)
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct DriverOpts {
    pub device: String,
    pub o: String,
    #[serde(rename = "type")]
    pub driver_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct VolumeCreate {
    pub Name: String,
    pub Driver: Option<String>,
    pub DriverOpts: Option<DriverOpts>,
    pub Labels: Option<HashMap<String, String>>,
}

impl Clone for DriverOpts {
    fn clone(&self) -> Self {
        DriverOpts {
            device: self.device.clone(),
            o: self.o.clone(),
            driver_type: self.driver_type.clone(),
        }
    }
}

impl Clone for VolumeCreate {
    fn clone(&self) -> Self {
        VolumeCreate {
            Name: self.Name.clone(),
            Driver: self.Driver.clone(),
            DriverOpts: self.DriverOpts.clone(),
            Labels: self.Labels.clone(),
        }
    }
}

impl std::fmt::Display for VolumeCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.Name)
    }
}
