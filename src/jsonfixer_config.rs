

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialze))]
pub struct JsonFixerConfig {
    pub preserve : bool, // Keep whitesapces, keeps original format
    pub space_after_key: bool, // Adds one space after : eg. {"key":"value"} to {"key": "value"}
    pub space_between: bool,  // Adds one space after between key and value eg. {"key":"value"} to { "key" : "value" }
    /*
    Make it humain readable
    eg. {"key":"value", "key2": "value"} to 
    { 
        "key": "value",
        "key2": "value"
    }
     */
    pub beautify: bool,
}

impl Default for JsonFixerConfig{
    
    fn default() -> Self{
        Self{
            preserve: false,
            space_after_key: false,
            space_between: false,
            beautify:false,
        }
    }
}

impl JsonFixerConfig {
    
    pub fn preserve(&self) -> bool {
        self.preserve
    }

    pub fn space_between(&self) -> bool {
        self.space_between && self.preserve == false
    }

    pub fn space_after_key(&self) -> bool {
        self.space_after_key && self.preserve == false && self.space_between == false 
    }
}