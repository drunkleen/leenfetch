use std::fs;
use std::path::Path;

pub fn get_ascii_art(distro: &str, custom_path: Option<&str>) -> String {
    if let Some(path_str) = custom_path {
        let path = Path::new(path_str);
        if path.exists() {
            if let Ok(content) = fs::read_to_string(path) {
                return content;
            }
        }
    }

    let distro = distro.to_lowercase();
    let ascii = match distro.as_str() {
        "arch" => ARCH,
        "ubuntu" => UBUNTU,
        "debian" => DEBIAN,
        "fedora" => FEDORA,
        "manjaro" => MANJARO,
        "nixos" => NIXOS,
        "void" => VOID,
        _ => DEFAULT,
    };

    ascii.to_string()
}

// Sample ASCII art for each distro
const ARCH: &str = r#"
       /\       
      /  \      
     / -- \     
    /      \    
   /   /\   \   
  /___/  \___\  
"#;

const UBUNTU: &str = r#"
         _        
     ---(_)       
 _/  ---  \_     
(_) |   | (_)    
  \__|_|__/      
"#;

const DEBIAN: &str = r#"
   _____          
  /  __ \         
 |  /  \/ ___ _ __ 
 | |    / _ \ '__|
 | \__/\  __/ |   
  \____/\___|_|   
"#;

const FEDORA: &str = r#"
   ______        
  / ____/___ ___ 
 / / __/ __ `__ \
/ /_/ / / / / / /
\____/_/ /_/ /_/ 
"#;

const MANJARO: &str = r#"
███████████████ 
███        ███ 
███  ████  ███ 
███  ████  ███ 
███  ████  ███ 
"#;

const NIXOS: &str = r#"
       .:         
    .:::           
 :::::            
:::::::.          
 ':::::'          
"#;

const VOID: &str = r#"
 _______         
|__   __|        
   | | ___  ___  
   | |/ _ \/ _ \ 
   | |  __/ (_) |
   |_|\___|\___/ 
"#;

const DEFAULT: &str = r#"
  _____ 
 /     \
| () () |
 \  ^  /
  |||||
  |||||
"#;
