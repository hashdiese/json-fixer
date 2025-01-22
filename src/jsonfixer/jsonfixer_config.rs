use crate::jsonfixer::jsonformatter::IndentStyle;

#[derive(Debug, Clone)]
pub struct JsonFixerConfig {
    pub preserve: bool,      // Keep whitesapces, keeps original format
    pub space_between: bool, // Adds one space after between key and value eg. {"key":"value"} to { "key" : "value" }
    /*
    Make it humain readable
    eg. {"key":"value", "key2": "value"} to
    {
        "key": "value",
        "key2": "value"
    }
     */
    pub beautify: bool,
    pub indent_style: IndentStyle,
    pub indent_size: usize,
    pub sort_keys: bool,
}

impl Default for JsonFixerConfig {
    fn default() -> Self {
        Self {
            preserve: false,
            space_between: false,
            beautify: false,
            indent_style: IndentStyle::Spaces,
            indent_size: 0,
            sort_keys: false,
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

    pub fn beautify(&self) -> bool {
        self.beautify && self.preserve == false
    }
}
