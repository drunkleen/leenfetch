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
        "Windows" => ARCH,
        _ => ARCH,
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
