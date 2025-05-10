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
    println!("ascii size small? {}", small_size);

    let key = distro.to_ascii_lowercase();

    match (key.as_str(), small_size) {
        // ("arch", true) => ARCH_SMALL,
        // ("arch", false) => ARCH,
        // ("debian", true) => DEBIAN_SMALL,
        // ("debian", false) => DEBIAN,
        // ("endeavouros", true) => ENDEAVOUROS_SMALL,
        // ("endeavouros", false) => ENDEAVOUROS,
        // ("fedora", true) => FEDORA_SMALL,
        // ("fedora", false) => FEDORA,
        // ("manjaro", true) => MANJARO_SMALL,
        // ("manjaro", false) => MANJARO,
        // ("nixos", true) => NIXOS_SMALL,
        // ("nixos", false) => NIXOS,
        ("ubuntu", true) => (UBUNTU_SMALL, UBUNTU_COLORS),
        ("ubuntu", false) => (UBUNTU, UBUNTU_COLORS),
        _ => (DEFAULT, &[0, 1, 2, 3, 4, 5, 6, 7]),
    }
}

const DEFAULT: &str = r#"
Default
${reset}"#;

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

const ENDEAVOUROS_SMALL: &str = r#"
       /\       
      /  \      
     / -- \     
    /      \    
   /   /\   \   
  /___/  \___\  
${reset}"#;

const ENDEAVOUROS: &str = r#"
${bold.c2}                     ./${bold.c6}o${bold.c5}.
${bold.c2}                   ./${bold.c6}sssso${bold.c5}-
${bold.c2}                 `:${bold.c6}osssssss+${bold.c5}-
${bold.c2}               `:+${bold.c6}sssssssssso${bold.c5}/.
${bold.c2}             `-/o${bold.c6}ssssssssssssso${bold.c5}/.
${bold.c2}           `-/+${bold.c6}sssssssssssssssso${bold.c5}+:`
${bold.c2}         `-:/+${bold.c6}sssssssssssssssssso${bold.c5}+/.
${bold.c2}       `.://o${bold.c6}sssssssssssssssssssso${bold.c5}++-
${bold.c2}      .://+${bold.c6}ssssssssssssssssssssssso${bold.c5}++:
${bold.c2}    .:///o${bold.c6}ssssssssssssssssssssssssso${bold.c5}++:
${bold.c2}  `:////${bold.c6}ssssssssssssssssssssssssssso${bold.c5}+++.
${bold.c2}`-////+${bold.c6}ssssssssssssssssssssssssssso${bold.c5}++++-
${bold.c2} `..-+${bold.c6}oosssssssssssssssssssssssso${bold.c5}+++++/`
   ./++++++++++++++++++++++++++++++/:.
  `:::::::::::::::::::::::::------``
${reset}"#;

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
${c1}        ´:+ssssssssssssssssss+:`
${c1}      -+ssssssssssssssssssyyssss+-
${c1}    .ossssssssssssssssss${c2}dMMMNy${c1}sssso.
${c1}   /sssssssssss${c2}hdmmNNmmyNMMMMh${c1}ssssss\
${c1}  +sssssssss${c2}hm${c1}yd${c2}MMMMMMMNddddy${c1}ssssssss+
${c1} /ssssssss${c2}hNMMM${c1}yh${c2}hyyyyhmNMMMNh${c1}ssssssss\
${c1}.ssssssss${c2}dMMMNh${c1}ssssssssss${c2}hNMMMd${c1}ssssssss.
${c1}+ssss${c2}hhhyNMMNy${c1}ssssssssssss${c2}yNMMMy${c1}sssssss+
${c1}oss${c2}yNMMMNyMMh${c1}ssssssssssssss${c2}hmmmh${c1}ssssssso
${c1}oss${c2}yNMMMNyMMh${c1}sssssssssssssshmmmh${c1}ssssssso
${c1}+ssss${c2}hhhyNMMNy${c1}ssssssssssss${c2}yNMMMy${c1}sssssss+
${c1}.ssssssss${c2}dMMMNh${c1}ssssssssss${c2}hNMMMd${c1}ssssssss.
${c1} \ssssssss${c2}hNMMM${c1}yh${c2}hyyyyhdNMMMNh${c1}ssssssss/
${c1}  +sssssssss${c2}dm${c1}yd${c2}MMMMMMMMddddy${c1}ssssssss+
${c1}   \sssssssssss${c2}hdmNNNNmyNMMMMh${c1}ssssss/
${c1}    .ossssssssssssssssss${c2}dMMMNy${c1}sssso.
${c1}      -+sssssssssssssssss${c2}yyy${c1}ssss+-
${c1}        `:+ssssssssssssssssss+:`
${c1}            .-\+oossssoo+/-.
${reset}"#;
