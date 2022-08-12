pub use toml; // Re-export toml so the user can use it

use anyhow::{Result, bail};
use toml::Value;

mod util;

#[cfg(test)]
mod tests {
    use crate::{Settings, SettingOption};
    use anyhow::{Result};
    
    #[test]
    fn settings_create() -> Result<()> {

        let filename = "Test_1.toml";

        let s = Settings::init(filename)?;
        
        s.read_settings_to_string()?;

        s.write_comment("testing!")?;

        std::fs::remove_file(filename)?;
        Ok(())
    }

    #[test]
    fn add_setting() -> Result<()> {
        let filename = "Test_2.toml";

        let s = Settings::init(filename)?;

        s.add_header("test")?;
        s.add_setting(SettingOption{ name: "option".to_string(), value: "bool".to_string(), default_value: Some("true".to_string()) }, None)?;
        s.add_setting(SettingOption{ name: "option_default".to_string(), value: "bool".to_string(), default_value: None }, Some(2))?;

        s.add_setting(SettingOption{ name: "option_num".to_string(), value: "number".to_string(), default_value: Some("5".to_string()) }, None)?;
        s.add_setting(SettingOption{ name: "option_num_default".to_string(), value: "number".to_string(), default_value: None }, Some(2))?;

        s.add_setting(SettingOption{ name: "option_string".to_string(), value: "string".to_string(), default_value: Some("test".to_owned()) }, None)?;
        s.add_setting(SettingOption{ name: "option_string_default".to_string(), value: "string".to_string(), default_value: None }, Some(2))?;
        

        std::fs::remove_file(filename)?;
        Ok(())
    }

    #[test]
    fn add_settings() -> Result<()> {

        let filename = "Test_3.toml";

        let s = Settings::init(filename)?;

        let test = [
            SettingOption{ name: "option_bool".to_string(), value: "bool".to_string(), default_value: Some("true".to_string()) },
            SettingOption{ name: "option_bool_default".to_string(), value: "bool".to_string(), default_value: None },
            SettingOption{ name: "option_num".to_string(), value: "number".to_string(), default_value: Some("5".to_string()) },
            SettingOption{ name: "option_num_default".to_string(), value: "number".to_string(), default_value: None },
            SettingOption{ name: "option_string".to_string(), value: "string".to_string(), default_value: Some("test".to_owned()) },
            SettingOption{ name: "option_string_default".to_string(), value: "string".to_string(), default_value: None },
        ];

        s.add_settings(test.to_vec(), None)?;


        // This code will always fail!
        /* 
        for t in test {

            let v = t.value.clone().to_lowercase();

            let value = if v == "bool" {
                match t.default_value { Some(o) => o, None => "false".to_string(), }

            } else if v == "number" { match t.default_value { Some(o) => o, None => 0.to_string(), } 

            } else if v == "string" { match t.default_value { Some(o) => format!(r#""{}""#,o), None => r#""""#.to_string(), } }

            else { r#"''"#.to_string() };

            println!("{:?}",s.read_settings_file()?);

            println!("{}: {:?} - {}",t.name,s.read_settings_file()?[t.name.clone()].as_str(),value.as_str());

            assert_eq!(s.read_settings_file()?[t.name].as_str(), Some(value.as_str()))
        }

        */

        std::fs::remove_file(filename)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct SettingOption {
    pub name: String,
    pub value: String,
    pub default_value: Option<String>, // Default For Bool is `false`
}

#[derive(Debug, Clone)]
pub struct Settings {
    path: String
}

impl Settings {
    pub fn init(settings_file_path: &str) -> Result<Self> {
        if settings_file_path.contains("toml") {

            util::write_file(None, settings_file_path.clone())?;

        } else { bail!("Setting File must be TOML!") }

        Ok( Self { path: settings_file_path.to_string() } )
    }

    pub fn read_settings_to_string(&self) -> Result<String> {
        Ok(util::read_file(&self.path)?)
    }

    pub fn write_comment(&self, comment: &str) -> Result<()> {

        let data = format!("# {}\n",comment);

        util::write_file(Some(data), &self.path)?;

        Ok(())
    }

    pub fn add_header(&self, header_name: &str) -> Result<()> {
        
        let header = format!("[{}]\n",header_name);
        util::write_file(Some(header), &self.path)?;

        Ok(())
    }

    pub fn add_setting(&self, setting: SettingOption, spaces: Option<usize>) -> Result<()> {

        let v = setting.value.clone().to_lowercase();

        let value = if v == "bool" {
            match setting.default_value { Some(o) => o, None => "false".to_string(), }

        } else if v == "number" { match setting.default_value { Some(o) => o, None => 0.to_string(), } 

        } else if v == "string" { match setting.default_value { Some(o) => format!(r#""{}""#,o), None => r#""""#.to_string(), } }

        else { r#"''"#.to_string() };
        
        let f = format!(r#"{} = {}"#,setting.name.replace(" ", "_"), value);

        let new_space = "\n".repeat(spaces.unwrap_or(1));

        let f = format!("{}{}",f,new_space);

        util::write_file(Some(f), &self.path)?;
        Ok(())
    }

    pub fn add_settings(&self, settings: Vec<SettingOption>, spaces: Option<usize>) -> Result<()> {

        for setting in settings { self.add_setting(setting, spaces)?; }
        Ok(())
    }

    pub fn read_settings_file(&self) -> Result<Value> { Ok(util::read_file(&self.path)?.parse::<Value>()?) }
}