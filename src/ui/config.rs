use directories::ProjectDirs;
use serde_json as Json;
use std::fs;
use std::io;
use std::mem::MaybeUninit;
use std::path;
use std::sync::Once;

#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct Config {
    path: path::PathBuf,
    config: Json::Value,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
impl Config {
    pub fn instance() -> &'static mut Config {
        static mut CONF: MaybeUninit<Config> = MaybeUninit::uninit();
        static ONCE: Once = Once::new();

        let path = ProjectDirs::from("com", "Dahua", "FastBuildConfig");
        let path = path.unwrap().config_dir().to_owned();

        ONCE.call_once(|| unsafe {
            let mut config = Config {
                path,
                config: Json::Value::default(),
            };

            Config::readConfig(&mut config);
            CONF.write(config);
        });

        unsafe { &mut *CONF.as_mut_ptr() }
    }

    fn readConfig(&mut self) {
        if !self.path.exists() {
            fs::create_dir_all(&self.path).unwrap();
        }

        let configfile = self.path.as_path().join("config.json");
        if !configfile.exists() {
            fs::write(&configfile, "{}").unwrap();
        }

        let file = fs::File::open(&configfile).unwrap();
        let reader = io::BufReader::new(file);
        let content: Json::Value = serde_json::from_reader(reader).unwrap();
        self.config = content;
    }

    pub fn getLight(&mut self) -> bool {
        if let Some(flag) = self.config["LightTheme"].as_bool() {
            return flag;
        }

        self.config["LightTheme"] = Json::Value::Bool(true);
        self.save();
        return true;
    }

    pub fn setLight(&mut self, light: bool) {
        if let Some(_) = self.config["LightTheme"].as_bool() {
            self.config["LightTheme"] = Json::json!(light);
        } else {
            self.config["LightTheme"] = Json::json!(false);
        }
        self.save();
    }

    pub fn getEnvInfo(&mut self) -> (i32, Vec<(String, String)>) {
        let mut save = false;
        if let None = self.config["EnvInfo"].as_object() {
            self.config["EnvInfo"] = Json::json!({});
            save = true;
        }

        if let None = self.config["EnvInfo"]["Index"].as_i64() {
            self.config["EnvInfo"]["Index"] = Json::json!(-1);
            save = true;
        }

        if let None = self.config["EnvInfo"]["List"].as_array() {
            self.config["EnvInfo"]["List"] = Json::json!([]);
            save = true;
        }

        let index = self.config["EnvInfo"]["Index"].as_i64().unwrap();
        let list = self.config["EnvInfo"]["List"].as_array().unwrap();
        let mut info: (i32, Vec<(String, String)>) = (index as i32, Vec::new());
        for item in list {
            let id = item["ID"].as_str().unwrap();
            let address = item["Address"].as_str().unwrap();
            info.1.push((address.to_string(), id.to_string()));
        }

        if save {
            self.save();
        }

        info
    }

    pub fn setEnvList(&mut self, info: &(i32, Vec<(String, String)>)) {
        self.config["EnvInfo"]["Index"] = Json::json!(info.0);
        self.config["EnvInfo"]["List"] = Json::json!([]);

        let list = self.config["EnvInfo"]["List"].as_array_mut().unwrap();
        for value in &info.1 {
            let mut item = Json::json!({});
            item["Address"] = Json::json!(value.0);
            item["ID"] = Json::json!(value.1);
            list.push(item);
        }

        self.save();
    }

    fn save(&self) {
        let configfile = self.path.as_path().join("config.json");
        fs::write(&configfile, self.config.to_string()).unwrap();
    }
}
