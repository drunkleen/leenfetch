use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::str::FromStr;

use crate::modules::utils::{get_default_colors, get_distro_colors_order};

#[allow(dead_code)]
pub enum AsciiSize {
    Small,
    Large,
    Off,
}

impl FromStr for AsciiSize {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        match s.to_lowercase().as_str() {
            "small" => Ok(AsciiSize::Small),
            "large" => Ok(AsciiSize::Large),
            "off" => Ok(AsciiSize::Off),
            _ => Err(()),
        }
    }
}

impl ToString for AsciiSize {
    fn to_string(&self) -> String {
        match self {
            AsciiSize::Small => "small".to_string(),
            AsciiSize::Large => "large".to_string(),
            AsciiSize::Off => "off".to_string(),
        }
    }
}

pub fn get_ascii_and_colors(
    distro: &str,
    custom_path: Option<&str>,
    ascii_size: String,
) -> (String, HashMap<&'static str, &'static str>) {
    // 1. Try to load custom ASCII art from a file
    if let Some(path) = custom_path {
        if let Ok(content) = fs::read_to_string(Path::new(path)) {
            return (content, get_default_colors());
        }
    }

    // 2. Fallback to built-in ASCII art
    if ascii_size.eq_ignore_ascii_case("off") {
        return ("".to_string(), get_default_colors());
    }

    let (ascii_art, dist_color) = get_builtin_ascii_art(distro, &ascii_size);

    (ascii_art.to_string(), get_distro_colors_order(dist_color))
}

fn get_builtin_ascii_art(distro: &str, ascii_size: &str) -> (&'static str, &'static [u8]) {
    let small_size = ascii_size == "small";

    let key = distro.split(" ").collect::<Vec<&str>>()[0].to_lowercase();

    match (key.as_str(), small_size) {
        ("debian", true) => (DEBIAN_SMALL, DEBIAN_COLORS),
        ("debian", false) => (DEBIAN, DEBIAN_COLORS),
        ("endeavouros", true) => (ENDEAVOUROS_SMALL, ENDEAVOUROS_COLORS),
        ("endeavouros", false) => (ENDEAVOUROS, ENDEAVOUROS_COLORS),
        ("fedora", true) => (FEDORA_SMALL, FEDORA_COLORS),
        ("fedora", false) => (FEDORA, FEDORA_COLORS),
        ("manjaro", true) => (MANJARO_SMALL, MANJARO_COLORS),
        ("manjaro", false) => (MANJARO, MANJARO_COLORS),
        ("nixos", true) => (NIXOS_SMALL, NIXOS_COLORS),
        ("nixos", false) => (NIXOS, NIXOS_COLORS),
        ("ubuntu", true) => (UBUNTU_SMALL, UBUNTU_COLORS),
        ("ubuntu", false) => (UBUNTU, UBUNTU_COLORS),
        _ => (DEFAULT, &[1, 2, 3, 4, 5, 6, 7]),
    }
}

const DEFAULT: &str = r#"
Default
${reset}"#;

const DEBIAN_COLORS: &[u8] = &[1, 3, 7];

const DEBIAN_SMALL: &str = r#"
${c1}  _____
 /  __ \\
|  /    |
|  \\___-
-_
  --_
${reset}"#;

const DEBIAN: &str = r#"
${c2}       _,met$$$$$gg.
    ,g$$$$$$$$$$$$$$$P.
  ,g$$P"        """Y$$.".
 ,$$P'              `$$$.
',$$P       ,ggs.     `$$b:
`d$$'     ,$P"'   ${c1}.${c2}    $$$
 $$P      d$'     ${c1},${c2}    $$P
 $$:      $$.   ${c1}-${c2}    ,d$$'
 $$;      Y$b._   _,d$P'
 Y$$.    ${c1}`.${c2}`"Y$$$$P"'
${c2} `$$b      ${c1}"-.__
${c2}  `Y$$
   `Y$$.
     `$$b.
       `Y$$b.
          `"Y$b._
              `"""
${reset}"#;

const ENDEAVOUROS_COLORS: &[u8] = &[1, 5, 4];
const ENDEAVOUROS_SMALL: &str = r#"
       /\       
      /  \      
     / -- \     
    /      \    
   /   /\   \   
  /___/  \___\  
${reset}"#;

const ENDEAVOUROS: &str = r#"
${c1}                     ./${c2}o${c3}.
${c1}                   ./${c2}sssso${c3}-
${c1}                 `:${c2}osssssss+${c3}-
${c1}               `:+${c2}sssssssssso${c3}/.
${c1}             `-/o${c2}ssssssssssssso${c3}/.
${c1}           `-/+${c2}sssssssssssssssso${c3}+:`
${c1}         `-:/+${c2}sssssssssssssssssso${c3}+/.
${c1}       `.://o${c2}sssssssssssssssssssso${c3}++-
${c1}      .://+${c2}ssssssssssssssssssssssso${c3}++:
${c1}    .:///o${c2}ssssssssssssssssssssssssso${c3}++:
${c1}  `:////${c2}ssssssssssssssssssssssssssso${c3}+++.
${c1}`-////+${c2}ssssssssssssssssssssssssssso${c3}++++-
${c1} `..-+${c2}oosssssssssssssssssssssssso${c3}+++++/`
   ./++++++++++++++++++++++++++++++/:.
  `:::::::::::::::::::::::::------``
${reset}"#;

const FEDORA_COLORS: &[u8] = &[4, 7, 1];
const FEDORA_SMALL: &str = r#"
${c1}        ,'''''.
       |   ,.  |
       |  |  '_'
  ,....|  |..
.'  ,_;|   ..'
|  |   |  |
|  ',_,'  |
 '.     ,'
   '''''
${reset}"#;

const FEDORA: &str = r#"
${c1}             .',;::::;,'.
         .';:cccccccccccc:;,.
      .;cccccccccccccccccccccc;.
    .:cccccccccccccccccccccccccc:.
  .;ccccccccccccc;${c2}.:dddl:.${c1};ccccccc;.
 .:ccccccccccccc;${c2}OWMKOOXMWd${c1};ccccccc:.
.:ccccccccccccc;${c2}KMMc${c1};cc;${c2}xMMc${c1};ccccccc:.
,cccccccccccccc;${c2}MMM.${c1};cc;${c2};WW:${c1};cccccccc,
:cccccccccccccc;${c2}MMM.${c1};cccccccccccccccc:
:ccccccc;${c2}oxOOOo${c1};${c2}MMM0OOk.${c1};cccccccccccc:
cccccc;${c2}0MMKxdd:${c1};${c2}MMMkddc.${c1};cccccccccccc;
ccccc;${c2}XM0'${c1};cccc;${c2}MMM.${c1};cccccccccccccccc'
ccccc;${c2}MMo${c1};ccccc;${c2}MMW.${c1};ccccccccccccccc;
ccccc;${c2}0MNc.${c1}ccc${c2}.xMMd${c1};ccccccccccccccc;
cccccc;${c2}dNMWXXXWM0:${c1};cccccccccccccc:,
cccccccc;${c2}.:odl:.${c1};cccccccccccccc:,.
:cccccccccccccccccccccccccccc:'.
.:cccccccccccccccccccccc:;,..
  '::cccccccccccccc::;,.
${reset}"#;

const MANJARO_COLORS: &[u8] = &[2, 7];
const MANJARO_SMALL: &str = r#"
${c1}||||||||| ||||
||||||||| ||||
||||      ||||
|||| |||| ||||
|||| |||| ||||
|||| |||| ||||
|||| |||| ||||
${reset}"#;

const MANJARO: &str = r#"
${c1}██████████████████  ████████
██████████████████  ████████
██████████████████  ████████
██████████████████  ████████
████████            ████████
████████  ████████  ████████
████████  ████████  ████████
████████  ████████  ████████
████████  ████████  ████████
████████  ████████  ████████
████████  ████████  ████████
████████  ████████  ████████
████████  ████████  ████████
████████  ████████  ████████
${reset}"#;

const NIXOS_COLORS: &[u8] = &[4, 6];
const NIXOS_SMALL: &str = r#"
  ${c1}  \\\\  \\\\ //
 ==\\\\__\\\\/ //
   //   \\\\//
==//     //==
 //\\\\___//
// /\\\\  \\\\==
  // \\\\  \\\\
${reset}"#;

const NIXOS: &str = r#"
${c1}          ::::.    ${c2}':::::     ::::'
${c1}          ':::::    ${c2}':::::.  ::::'
${c1}            :::::     ${c2}'::::.:::::
${c1}      .......:::::..... ${c2}::::::::
${c1}     ::::::::::::::::::. ${c2}::::::    ${c1}::::.
    ::::::::::::::::::::: ${c2}:::::.  ${c1}.::::'
${c2}           .....           ::::' ${c1}:::::'
${c2}          :::::            '::' ${c1}:::::'
${c2} ........:::::               ' ${c1}:::::::::::.
${c2}:::::::::::::                 ${c1}:::::::::::::
${c2} ::::::::::: ${c1}..              ${c1}:::::
${c2}     .::::: ${c1}.:::            ${c1}:::::
${c2}    .:::::  ${c1}:::::          ${c1}'''''    ${c2}.....
    :::::   ${c1}':::::.  ${c2}......:::::::::::::'
     :::     ${c1}::::::. ${c2}':::::::::::::::::'
${c1}            .:::::::: ${c2}'::::::::::
${c1}           .::::''::::.     ${c2}'::::.
${c1}          .::::'   ::::.     ${c2}'::::.
${c1}         .::::      ::::      ${c2}'::::.
${reset}"#;

const UBUNTU_COLORS: &[u8] = &[1, 7, 3];

const UBUNTU_SMALL: &str = r#"
${c1}         _
     ---(_)
 _/  ---  \\
(_) |   |
  \\  --- _/
     ---(_)
${reset}"#;

const UBUNTU: &str = r#"
${c1}            .-/+oossssoo+\-.
        ´:+ssssssssssssssssss+:`
      -+ssssssssssssssssssyyssss+-
    .ossssssssssssssssss${c2}dMMMNy${c1}sssso.
   /sssssssssss${c2}hdmmNNmmyNMMMMh${c1}ssssss\
  +sssssssss${c2}hm${c1}yd${c2}MMMMMMMNddddy${c1}ssssssss+
 /ssssssss${c2}hNMMM${c1}yh${c2}hyyyyhmNMMMNh${c1}ssssssss\
.ssssssss${c2}dMMMNh${c1}ssssssssss${c2}hNMMMd${c1}ssssssss.
+ssss${c2}hhhyNMMNy${c1}ssssssssssss${c2}yNMMMy${c1}sssssss+
oss${c2}yNMMMNyMMh${c1}ssssssssssssss${c2}hmmmh${c1}ssssssso
oss${c2}yNMMMNyMMh${c1}sssssssssssssshmmmh${c1}ssssssso
+ssss${c2}hhhyNMMNy${c1}ssssssssssss${c2}yNMMMy${c1}sssssss+
.ssssssss${c2}dMMMNh${c1}ssssssssss${c2}hNMMMd${c1}ssssssss.
 \ssssssss${c2}hNMMM${c1}yh${c2}hyyyyhdNMMMNh${c1}ssssssss/
  +sssssssss${c2}dm${c1}yd${c2}MMMMMMMMddddy${c1}ssssssss+
   \sssssssssss${c2}hdmNNNNmyNMMMMh${c1}ssssss/
    .ossssssssssssssssss${c2}dMMMNy${c1}sssso.
      -+sssssssssssssssss${c2}yyy${c1}ssss+-
        `:+ssssssssssssssssss+:`
            .-\+oossssoo+/-.
${reset}"#;
