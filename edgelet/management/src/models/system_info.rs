/*
 * IoT Edge Management API
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: 2018-06-28
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfo {
    #[serde(rename = "osType")]
    os_type: String,
    #[serde(rename = "architecture")]
    architecture: String,
    #[serde(rename = "version")]
    version: String,
}

impl SystemInfo {
    pub fn new(os_type: String, architecture: String, version: String) -> SystemInfo {
        SystemInfo {
            os_type,
            architecture,
            version,
        }
    }

    pub fn set_os_type(&mut self, os_type: String) {
        self.os_type = os_type;
    }

    pub fn with_os_type(mut self, os_type: String) -> SystemInfo {
        self.os_type = os_type;
        self
    }

    pub fn os_type(&self) -> &String {
        &self.os_type
    }

    pub fn set_architecture(&mut self, architecture: String) {
        self.architecture = architecture;
    }

    pub fn with_architecture(mut self, architecture: String) -> SystemInfo {
        self.architecture = architecture;
        self
    }

    pub fn architecture(&self) -> &String {
        &self.architecture
    }

    pub fn set_version(&mut self, version: String) {
        self.version = version;
    }

    pub fn with_version(mut self, version: String) -> SystemInfo {
        self.version = version;
        self
    }

    pub fn version(&self) -> &String {
        &self.version
    }
}
