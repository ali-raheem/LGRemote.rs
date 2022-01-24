use phf::{phf_map, Map};
use reqwest::{Client, Response};

/// Represents LG TV supporting 2012 era network controls
pub struct LGTV {
    // TVs IP address
    pub ip: String,
    // Port is a public field its set to "8080" by default.
    pub port: String,
    // Key will be set when passed to `pair_with_key`
    pub key: u32,
    // Set by `pair_with_key`
    pub paired: bool,
    // HTTP Client handle for reuse
    client: Client,
}

impl LGTV {
    /// Returns an LG TV instance at a particular IP address
    ///
    /// # Arguments
    ///
    /// * `ip` - A string slice containing the IP address
    ///
    /// # Examples
    ///
    /// ```
    /// use LGTV;
    /// 
    /// let TV = LGTV::new("192.168.1.250");
    /// ```
    pub fn new(ip: &str) -> LGTV {
        LGTV {
            ip: ip.to_string(),
            port: "8080".to_string(),
            client: reqwest::Client::new(),
            paired: false,
            key: 0,
        }
    }
    /// Tries to pair with a TV with a given key
    ///
    /// # Arguments
    ///
    /// * `key` - u32 containing TVs pairing key. Can be displayed on the TV with `display_pair_key()`
    ///
    pub async fn pair_with_key(&mut self, key: u32) -> Result<Response, reqwest::Error> {
        let auth_request = format!(
            "
            <?xml version=\"1.0\" encoding=\"utf-8\"?>
            <auth>
                <type>AuthReq</type>
                <value>{}</value>
            </auth>
        ",
            key
        );
        self.key = key;
        self.paired = true;
        let url = format!("http://{}:{}/roap/api/auth", self.ip, self.port);
        self.client
            .post(url)
            .header("Content-Type", "application/atom+xml")
            .body(auth_request)
            .send()
            .await
    }
    /// Requests TV displays it's pairing code on screen
    pub async fn display_pair_key(&mut self) -> Result<Response, reqwest::Error> {
        let display_pair_key = "
            <?xml version=\"1.0\" encoding=\"utf-8\"?>
            <auth>
                <type>AuthKeyReq</type>
            </auth>";
        let url = format!("http://{}:{}/roap/api/auth", self.ip, self.port);
        self.client
            .post(url)
            .header("Content-Type", "application/atom+xml")
            .body(display_pair_key)
            .send()
            .await
    }
    /// Sends a command to the TV
    ///
    /// # Arguments
    ///
    /// * `cmd_code` - u32 corresponding to a supported one of the TVs [COMMAND_CODES].
    ///
    pub async fn send_command(&mut self, cmd_code: u32) -> Result<Response, reqwest::Error> {
        let send_command = format!(
            "
            <?xml version=\"1.0\" encoding=\"utf-8\"?>
            <command>
                <name>HandleKeyInput</name>
                <value>{}</value>
            </command>
        ",
            cmd_code
        );

        let url = format!("http://{}:{}/roap/api/command", self.ip, self.port);

        self.client
            .post(url)
            .header("Content-Type", "application/atom+xml")
            .body(send_command)
            .send()
            .await
    }
}

/// Map of common names for commands and the corresponding code.
pub static COMMAND_CODES: Map<&'static str, u32> = phf_map! {
    "POWER" => 1,
    "NUM_0" => 2,
    "NUM_1" => 3,
    "NUM_2" => 4,
    "NUM_3" => 5,
    "NUM_4" => 6,
    "NUM_5" => 7,
    "NUM_6" => 8,
    "NUM_7" => 9,
    "NUM_8" => 10,
    "NUM_9" => 11,
    "UP" => 12,
    "DOWN" => 13,
    "LEFT" => 14,
    "RIGHT" => 15,
    "OK" => 20,
    "HOME" => 21,
    "MENU" => 22,
    "BACK" => 23,
    "VOLUME_UP" => 24,
    "VOLUME_DOWN" => 25,
    "MUTE" => 26,
    "CHANNEL_UP" => 27,
    "CHANNEL_DOWN" => 28,
    "BLUE" => 29,
    "GREEN" => 30,
    "RED" => 31,
    "YELLOW" => 32,
    "PLAY" => 33,
    "PAUSE" => 34,
    "STOP" => 35,
    "FF" => 36,
    "REW" => 37,
    "SKIP_FF" => 38,
    "SKIP_REW" => 39,
    "REC" => 40,
    "REC_LIST" => 41,
    "LIVE" => 43,
    "EPG" => 44,
    "INFO" => 45,
    "ASPECT" => 46,
    "EXT" => 47,
    "PIP" => 48,
    "SUBTITLE" => 49,
    "PROGRAM_LIST" => 50,
    "TEXT" => 51,
    "MARK" => 52,
    "3D" => 400,
    "3D_LR" => 401,
    "DASH" => 402,
    "PREV" => 403,
    "FAV" => 404,
    "QUICK_MENU" => 405,
    "TEXT_OPTION" => 406,
    "AUDIO_DESC" => 407,
    "NETCAST" => 408,
    "ENERGY_SAVE" => 409,
    "AV" => 410,
    "SIMPLINK" => 411,
    "EXIT" => 412,
    "RESERVED" => 413,
    "PIP_CHANNEL_UP" => 414,
    "PIP_CHANNEL_DOWN" => 415,
    "PIP_SWITCH" => 416,
    "APPS" => 417
};
