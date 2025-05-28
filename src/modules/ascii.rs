pub fn get_builtin_ascii_art(ascii_distro: &str) -> &'static str {
    let mut small_size = false;

    let dist = if let Some(stripped) = ascii_distro.strip_suffix("_small") {
        small_size = true;
        stripped.to_lowercase()
    } else {
        ascii_distro.to_lowercase()
    };

    if dist.contains("windows") {
        if dist.contains("11") {
            return WINDOWS_11;
        } else if dist.contains("10") || dist.contains("8") {
            return WINDOWS_10;
        } else {
            return WINDOWS;
        }
    }

    match (dist.as_str(), small_size) {
        ("almalinux", false) | ("alma", false) => ALMALINUX,

        ("alpine", true) => ALPINE_SMALL,
        ("alpine", false) => ALPINE,

        ("alter", _) => ALTER,

        ("android", true) => ANDROID_SMALL,
        ("android", false) => ANDROID,

        ("amazon", true) => AMAZON_SMALL,
        ("amazon", false) => AMAZON,

        ("anarchy", _) => ANARCHY,

        ("instantos", _) => INSTANT_OS,

        ("antix", _) => ANTI_X,

        ("antergos", _) => ANTERGOS,

        ("aosc os/retro", _) | ("aoscos/retro", _) | ("aosc os retro", _) => AOSC_OS_RETRO,
        ("aosc os", _) | ("aoscos", _) => AOSC_OS,

        ("arch", true) | ("archlinux", true) | ("arch linux", true) => ARCH_SMALL,
        ("arch", false) | ("archlinux", false) | ("arch linux", false) => ARCH,

        ("archbox", _) => ARCH_BOX,

        ("archlabs", _) => ARCH_LABS,

        ("archcraft", _) => ARCH_CRAFT,

        ("arcolinux", true) => ARCOLINUX_SMALL,
        ("arcolinux", false) => ARCOLINUX,

        ("apricity", _) => APRICITY,

        ("archstrike", _) => ARCH_STRIKE,

        ("archmerge", _) => ARCH_MERGE,

        ("artix", _) => ARTIX,

        ("arya", _) => ARYA,

        ("asteroidos", _) | ("asteroid", _) => ASTEROIDOS,

        ("aix", _) => AIX,

        ("bedrock", _) => BEDROCK,

        ("bitrig", _) => BITRIG,

        ("blackarch", _) | ("black arch", _) => BLACK_ARCH,

        ("blag", _) => BLAG,

        ("blankon", _) | ("blank on", _) => BLANK_ON,

        ("bluelight", _) => BLUE_LIGHT,

        ("bodhi", _) => BODHI,

        ("bonsai", _) => BONSAI,

        ("bsd", _) => BSD,

        ("bunsenlabs", _) | ("bunsen labs", _) => BUNSEN_LABS,

        ("calculate", _) | ("calculateos", _) => CALCULATE,

        ("carbs", _) => CARBS,

        ("cbl-mariner", _) | ("cbl mariner", _) => CBL_MARINER,

        ("celos", _) | ("cel os", _) => CEL_OS,

        ("clearos", _) => CLEAROS,

        ("centos", true) => CENTOS_SMALL,
        ("centos", false) => CENTOS,

        ("chakra", _) => CHAKRA,

        ("chaletos", _) | ("chalet os", _) => CHALET_OS,

        ("chapeau", _) => CHEAPEAU,

        ("chrom", _) | ("chrome", _) | ("chromeos", _) | ("chrome os", _) => CHROME,

        ("cleanjaro", true) => CLEANJARO_SMALL,
        ("cleanjaro", false) => CLEANJARO,

        ("clear linux os", _)
        | ("clearlinuxos", _)
        | ("clear_linux", _)
        | ("clear_linux_os", _) => CLEAR_OS,

        ("clover", _) => CLOVER,

        ("condres", _) => CONDRES,

        ("container Linux by coreos", _) | ("container_linux", _) | ("container", _) => {
            CONTAINER_LINUX
        }

        ("crux", true) => CRUX_SMALL,
        ("crux", false) => CRUX,

        ("crystal linux", _) | ("crystal", _) => CRYSTAL_LINUX,

        ("cucumber", _) => CUCUMBER,

        ("cyberos", _) | ("cyber os", _) => CYBER_OS,

        ("dahlia", _) => DAHLIA,

        ("debian", true) => DEBIAN_SMALL,
        ("debian", false) => DEBIAN,

        ("deepin", _) => DEEPIN,

        ("desaos", _) => DESAOS,

        ("devuan", _) => DEVUAN,

        ("dracos", _) => DRACOS,

        ("darkos", _) | ("dark os", _) => DARK_OS,

        ("dragonfly_old", _) => DRAGONFLY_OLD,
        ("dragonfly", true) => DRAGONFLY_SMALL,
        ("dragonfly", false) => DRAGONFLY,

        ("drauger", _) => DRAUGER,

        ("elementary", true) => ELEMENTARY_SMALL,
        ("elementary", false) => ELEMENTARY,

        ("endeavouros", true) => ENDEAVOUROS_SMALL,
        ("endeavouros", false) => ENDEAVOUROS,

        ("fedora", true) => FEDORA_SMALL,
        ("fedora", false) => FEDORA,

        ("garuda", _) => GARUDA,

        ("hash", _) => HASH,

        ("kali", _) => KALI,

        ("kde", _) => KDE,

        ("kubuntu", _) => KUBUNTU,

        ("linuxmint", true) | ("mint", true) | ("linux mint", true) => LINUXMINT_SMALL,
        ("linuxmint", false) | ("mint", false) | ("linux mint", false) => LINUXMINT,
        ("linuxmint_old", false)
        | ("linuxmintold", false)
        | ("mintold", false)
        | ("mint_old", false)
        | ("mint old", false) => LINUXMINT_OLD,

        ("manjaro", true) => MANJARO_SMALL,
        ("manjaro", false) => MANJARO,

        ("mx", true) => MX_SMALL,
        ("mx", false) => MX,

        ("nixos", true) => NIXOS_SMALL,
        ("nixos", false) => NIXOS,

        ("opensuse", true) | ("suse", true) => OPENSUSE_SMALL,
        ("opensuse", false) | ("open suse", false) | ("suse", false) => OPENSUSE,
        ("opensuse leap", _) | ("opensuse_leap", _) => OPENSUSE_LEAP,
        ("opensuse tumbleweed", _) | ("opensuse_tumbleweed", _) => OPENSUSE_TUMBLEWEED,

        ("parch", _) | ("parch linux", _) => PARCH,

        ("parrot", _) | ("parrot linux", _) => PARROT,

        ("popos", true) | ("pop_os", true) => POP_OS_SMALL,
        ("popos", false) | ("pop_os", false) | ("pop!_os", false) => POP_OS,

        ("redhat", _) | ("red hat", _) | ("rhel", _) => REDHAT,

        ("rocky", true) => ROCKY_SMALL,
        ("rocky", false) => ROCKY,

        ("tails", _) => TAILS,

        ("ubuntu", true) => UBUNTU_SMALL,
        ("ubuntu", false) => UBUNTU,

        ("zorin", _) => ZORIN,

        ("windows 11", _) | ("windows11", _) => WINDOWS_11,
        ("windows 10", _) | ("windows10", _) | ("windows 8", _) | ("windows8", _) => WINDOWS_10,
        ("windows", _) => WINDOWS,

        _ => DEFAULT,
    }
}

const DEFAULT: &str = r#"${c2}        #####
${c2}       #######
${c2}       ##${c1}O${c2}#${c1}O${c2}##
${c2}       #${c3}#####${c2}#
${c2}     ##${c1}##${c3}###${c1}##${c2}##
${c2}    #${c1}##########${c2}##
${c2}   #${c1}############${c2}##
${c2}   #${c1}############${c2}###
${c3}  ##${c2}#${c1}###########${c2}##${c3}#
${c3}######${c2}#${c1}#######${c2}#${c3}######
${c3}#######${c2}#${c1}#####${c2}#${c3}#######
${c3}  #####${c2}#######${c3}#####"#;

const ALMALINUX: &str = r#"${c1}         'c:.
${c1}        lkkkx, ..       ${c2}..   ,cc,
${c1}        okkkk:ckkx'  ${c2}.lxkkx.okkkkd
${c1}        .:llcokkx'  ${c2}:kkkxkko:xkkd,
${c1}      .xkkkkdood:  ${c2};kx,  .lkxlll;
${c1}       xkkx.       ${c2}xk'     xkkkkk:
${c1}       'xkx.       ${c2}xd      .....,.
${c3}      .. ${c1}:xkl'     ${c2}:c      ..''..
${c3}    .dkx'  ${c1}.:ldl:'. ${c2}'  ${c4}':lollldkkxo;
${c3}  .''lkkko'                     ${c4}ckkkx.
${c3}'xkkkd:kkd.       ..  ${c5};'        ${c4}:kkxo.
${c3},xkkkd;kk'      ,d;    ${c5}ld.   ${c4}':dkd::cc,
${c3} .,,.;xkko'.';lxo.      ${c5}dx,  ${c4}:kkk'xkkkkc
${c3}     'dkkkkkxo:.        ${c5};kx  ${c4}.kkk:;xkkd.
${c3}       .....   ${c5}.;dk:.   ${c5}lkk.  ${c4}:;,
             ${c5}:kkkkkkkdoxkkx
              ,c,,;;;:xkkd.
                ;kkkkl...
                ;kkkkl
                 ,od;"#;

const ALPINE_SMALL: &str = r#"${c1}   /\ /\
  /${c2}/ ${c1}\  \
 /${c2}/   ${c1}\  \
/${c2}//    ${c1}\  \
${c2}//      ${c1}\  \
         \"#;
const ALPINE: &str = r#"${c1}       .hddddddddddddddddddddddh.
      :dddddddddddddddddddddddddd:
     /dddddddddddddddddddddddddddd/
    +dddddddddddddddddddddddddddddd+
  `sdddddddddddddddddddddddddddddddds`
 `ydddddddddddd++hdddddddddddddddddddy`
.hddddddddddd+`  `+ddddh:-sdddddddddddh.
hdddddddddd+`      `+y:    .sddddddddddh
ddddddddh+`   `//`   `.`     -sddddddddd
ddddddh+`   `/hddh/`   `:s-    -sddddddd
ddddh+`   `/+/dddddh/`   `+s-    -sddddd
ddd+`   `/o` :dddddddh/`   `oy-    .yddd
hdddyo+ohddyosdddddddddho+oydddy++ohdddh
.hddddddddddddddddddddddddddddddddddddh.
 `yddddddddddddddddddddddddddddddddddy`
  `sdddddddddddddddddddddddddddddddds`
    +dddddddddddddddddddddddddddddd+
     /dddddddddddddddddddddddddddd/
      :dddddddddddddddddddddddddd:
       .hddddddddddddddddddddddh."#;

const ALTER: &str = r#"${c1}                      %,
                    ^WWWw
                   'wwwwww
                  !wwwwwwww
                 #`wwwwwwwww
                @wwwwwwwwwwww
               wwwwwwwwwwwwwww
              wwwwwwwwwwwwwwwww
             wwwwwwwwwwwwwwwwwww
            wwwwwwwwwwwwwwwwwwww,
           w~1i.wwwwwwwwwwwwwwwww,
         3~:~1lli.wwwwwwwwwwwwwwww.
        :~~:~?ttttzwwwwwwwwwwwwwwww
       #<~:~~~~?llllltO-.wwwwwwwwwww
      #~:~~:~:~~?ltlltlttO-.wwwwwwwww
     @~:~~:~:~:~~(zttlltltlOda.wwwwwww
    @~:~~: ~:~~:~:(zltlltlO    a,wwwwww
   8~~:~~:~~~~:~~~~_1ltltu          ,www
  5~~:~~:~~:~~:~~:~~~_1ltq             N,,
 g~:~~:~~~:~~:~~:~:~~~~1q                N,"#;

const ANARCHY: &str = r#"                         ${c2}..${c1}
                        ${c2}..${c1}
                      ${c2}:..${c1}
                    ${c2}:+++.${c1}
              .:::++${c2}++++${c1}+::.
          .:+######${c2}++++${c1}######+:.
       .+#########${c2}+++++${c1}##########:.
     .+##########${c2}+++++++${c1}##${c2}+${c1}#########+.
    +###########${c2}+++++++++${c1}############:
   +##########${c2}++++++${c1}#${c2}++++${c1}#${c2}+${c1}###########+
  +###########${c2}+++++${c1}###${c2}++++${c1}#${c2}+${c1}###########+
 :##########${c2}+${c1}#${c2}++++${c1}####${c2}++++${c1}#${c2}+${c1}############:
 ###########${c2}+++++${c1}#####${c2}+++++${c1}#${c2}+${c1}###${c2}++${c1}######+
.##########${c2}++++++${c1}#####${c2}++++++++++++${c1}#######.
.##########${c2}+++++++++++++++++++${c1}###########.
 #####${c2}++++++++++++++${c1}###${c2}++++++++${c1}#########+
 :###${c2}++++++++++${c1}#########${c2}+++++++${c1}#########:
  +######${c2}+++++${c1}##########${c2}++++++++${c1}#######+
   +####${c2}+++++${c1}###########${c2}+++++++++${c1}#####+
    :##${c2}++++++${c1}############${c2}++++++++++${c1}##:
     .${c2}++++++${c1}#############${c2}++++++++++${c1}+.
      :${c2}++++${c1}###############${c2}+++++++${c1}::
     .${c2}++. .:+${c1}##############${c2}+++++++${c1}..
     ${c2}.:.${c1}      ..::++++++::..:${c2}++++${c1}+.
     ${c2}.${c1}                       ${c2}.:+++${c1}.
                                ${c2}.:${c1}:
                                   ${c2}..${c1}
                                    ${c2}..${c1}"#;

const ANDROID_SMALL: &str = r#"${c1}  ;,           ,;
   ';,.-----.,;'
  ,'           ',
 /    O     O    \\
|                 |
'-----------------'"#;
const ANDROID: &str = r#"${c1}         -o          o-
          +hydNNNNdyh+
        +mMMMMMMMMMMMMm+
      `dMM${c2}m:${c1}NMMMMMMN${c2}:m${c1}MMd`
      hMMMMMMMMMMMMMMMMMMh
  ..  yyyyyyyyyyyyyyyyyyyy  ..
.mMMm`MMMMMMMMMMMMMMMMMMMM`mMMm.
:MMMM-MMMMMMMMMMMMMMMMMMMM-MMMM:
:MMMM-MMMMMMMMMMMMMMMMMMMM-MMMM:
:MMMM-MMMMMMMMMMMMMMMMMMMM-MMMM:
:MMMM-MMMMMMMMMMMMMMMMMMMM-MMMM:
-MMMM-MMMMMMMMMMMMMMMMMMMM-MMMM-
 +yy+ MMMMMMMMMMMMMMMMMMMM +yy+
      mMMMMMMMMMMMMMMMMMMm
      `/++MMMMh++hMMMM++/`
          MMMMo  oMMMM
          MMMMo  oMMMM
          oNMm-  -mMNs"#;

const AMAZON_SMALL: &str = r#"${c1}
        -/o:.
     -+hNNMMNNho-
.``  -/+shNhs+/-  `.
dNms+:. `.Md.` .:+shdd
ddhs+:..      ..:+shdd
dMMMMMMNds  odNMMMMMMd
dMMMMMMMMh  yMMMMMMMMd
.:+ydNMMMh  yMMMNdy+:.
   `.:+shh  yNhs+:``
     `-+sy  ss+:`"#;
const AMAZON: &str = r#"${c1}             `-/oydNNdyo:.`
      `.:+shmMMMMMMMMMMMMMMmhs+:.`
    -+hNNMMMMMMMMMMMMMMMMMMMMMMNNho-
.``      -/+shmNNMMMMMMNNmhs+/-      ``.
dNmhs+:.       `.:/oo/:.`       .:+shmNd
dMMMMMMMNdhs+:..        ..:+shdNMMMMMMMd
dMMMMMMMMMMMMMMNds    odNMMMMMMMMMMMMMMd
dMMMMMMMMMMMMMMMMh    yMMMMMMMMMMMMMMMMd
dMMMMMMMMMMMMMMMMh    yMMMMMMMMMMMMMMMMd
dMMMMMMMMMMMMMMMMh    yMMMMMMMMMMMMMMMMd
dMMMMMMMMMMMMMMMMh    yMMMMMMMMMMMMMMMMd
dMMMMMMMMMMMMMMMMh    yMMMMMMMMMMMMMMMMd
dMMMMMMMMMMMMMMMMh    yMMMMMMMMMMMMMMMMd
dMMMMMMMMMMMMMMMMh    yMMMMMMMMMMMMMMMMd
dMMMMMMMMMMMMMMMMh    yMMMMMMMMMMMMMMMMd
dMMMMMMMMMMMMMMMMh    yMMMMMMMMMMMMMMMMd
.:+ydNMMMMMMMMMMMh    yMMMMMMMMMMMNdy+:.
     `.:+shNMMMMMh    yMMMMMNhs+:``
            `-+shy    shs+:`"#;

const INSTANT_OS: &str = r#"${c1}
     'cx0XWWMMWNKOd:'.
  .;kNMMMMMMMMMMMMMWNKd'
 'kNMMMMMMWNNNWMMMMMMMMXo.
,0MMMMMW0o;'..,:dKWMMMMMWx.
OMMMMMXl.        .xNMMMMMNo
WMMMMNl           .kWWMMMMO'
MMMMMX;            oNWMMMMK,
NMMMMWo           .OWMMMMMK,
kWMMMMNd.        ,kWMMMMMMK,
'kWMMMMWXxl:;;:okNMMMMMMMMK,
 .oXMMMMMMMWWWMMMMMMMMMMMMK,
   'oKWMMMMMMMMMMMMMMMMMMMK,
     .;lxOKXXXXXXXXXXXXXXXO;......
          ................,d0000000kd:.
                          .kMMMMMMMMMW0;
                          .kMMMMMMMMMMMX
                          .xMMMMMMMMMMMW
                           cXMMMMMMMMMM0
                            :0WMMMMMMNx,
                             .o0NMWNOc."#;

const ANTI_X: &str = r#"${c1}
                    \
         , - ~ ^ ~ - \        /
     , '              \ ' ,  /
   ,                   \   '/
  ,                     \  / ,
 ,___,                   \/   ,
 /   |   _  _  _|_ o     /\   ,
|,   |  / |/ |  |  |    /  \  ,
 \,_/\_/  |  |_/|_/|_/_/    \,
   ,                  /     ,\
     ,               /  , '   \
      ' - , _ _ _ ,  '"#;

const ANTERGOS: &str = r#"${c2}              `.-/::/-``
            .-/osssssssso/.
           :osyysssssssyyys+-
        `.+yyyysssssssssyyyyy+.
       `/syyyyyssssssssssyyyyys-`
      `/yhyyyyysss${c1}++${c2}ssosyyyyhhy/`
     .ohhhyyyys${c1}o++/+o${c2}so${c1}+${c2}syy${c1}+${c2}shhhho.
    .shhhhys${c1}oo++//+${c2}sss${c1}+++${c2}yyy${c1}+s${c2}hhhhs.
   -yhhhhs${c1}+++++++o${c2}ssso${c1}+++${c2}yyy${c1}s+o${c2}hhddy:
  -yddhhy${c1}o+++++o${c2}syyss${c1}++++${c2}yyy${c1}yooy${c2}hdddy-
 .yddddhs${c1}o++o${c2}syyyyys${c1}+++++${c2}yyhh${c1}sos${c2}hddddy`
`odddddhyosyhyyyyyy${c1}++++++${c2}yhhhyosddddddo
.dmdddddhhhhhhhyyyo${c1}+++++${c2}shhhhhohddddmmh.
ddmmdddddhhhhhhhso${c1}++++++${c2}yhhhhhhdddddmmdy
dmmmdddddddhhhyso${c1}++++++${c2}shhhhhddddddmmmmh
-dmmmdddddddhhys${c1}o++++o${c2}shhhhdddddddmmmmd-
.smmmmddddddddhhhhhhhhhdddddddddmmmms.
   `+ydmmmdddddddddddddddddddmmmmdy/.
      `.:+ooyyddddddddddddyyso+:.`"#;

const AOSC_OS_RETRO: &str = r#"${c2}          .........
     ...................
   .....................${c1}################${c2}
 ..............     ....${c1}################${c2}
..............       ...${c1}################${c2}
.............         ..${c1}****************${c2}
............     .     .${c1}****************${c2}
...........     ...     ${c1}................${c2}
..........     .....     ${c1}...............${c2}
.........     .......     ...
 .${c3}......                   ${c2}.
  ${c3}.....      .....${c2}....    ${c4}...........
  ${c3}....      ......${c2}.       ${c4}...........
  ${c3}...      .......        ${c4}...........
  ${c3}................        ${c4}***********
  ${c3}................        ${c4}###########
  ${c3}****************
  ${c3}################"#;

const AOSC_OS: &str = r#"${c2}             .:+syhhhhys+:.
         .ohNMMMMMMMMMMMMMMNho.
      `+mMMMMMMMMMMmdmNMMMMMMMMm+`
     +NMMMMMMMMMMMM/   `./smMMMMMN+
   .mMMMMMMMMMMMMMMo        -yMMMMMm.
  :NMMMMMMMMMMMMMMMs          .hMMMMN:
 .NMMMMhmMMMMMMMMMMm+/-         oMMMMN.
 dMMMMs  ./ymMMMMMMMMMMNy.       sMMMMd
-MMMMN`      oMMMMMMMMMMMN:      `NMMMM-
/MMMMh       NMMMMMMMMMMMMm       hMMMM/
/MMMMh       NMMMMMMMMMMMMm       hMMMM/
-MMMMN`      :MMMMMMMMMMMMy.     `NMMMM-
 dMMMMs       .yNMMMMMMMMMMMNy/. sMMMMd
 .NMMMMo         -/+sMMMMMMMMMMMmMMMMN.
  :NMMMMh.          .MMMMMMMMMMMMMMMN:
   .mMMMMMy-         NMMMMMMMMMMMMMm.
     +NMMMMMms/.`    mMMMMMMMMMMMN+
      `+mMMMMMMMMNmddMMMMMMMMMMm+`
         .ohNMMMMMMMMMMMMMMNho.
             .:+syhhhhys+:."#;

const ARCH_SMALL: &str = r#"${c1}      /\
     /  \
    /\   \
${c2}   /      \
  /   ,,   \
 /   |  |  -\
/_-''    ''-_\"#;
const ARCH: &str = r#"${c1}                   -`
                  .o+`
                 `ooo/
                `+oooo:
               `+oooooo:
               -+oooooo+:
             `/:-:++oooo+:
            `/++++/+++++++:
           `/++++++++++++++:
          `/+++o${c2}oooooooo${c1}oooo/`
${c2}         ${c1}./${c2}ooosssso++osssssso${c1}+`
${c2}        .oossssso-````/ossssss+`
       -osssssso.      :ssssssso.
      :osssssss/        osssso+++.
     /ossssssss/        +ssssooo/-
   `/ossssso+/:-        -:/+osssso+-
  `+sso+:-`                 `.-/+oso:
 `++:.                           `-/+/
 .`                                 `/"#;

const ARCH_BOX: &str = r#"${c1}              ...:+oh/:::..
         ..-/oshhhhhh`   `::::-.
     .:/ohhhhhhhhhhhh`        `-::::.
 .+shhhhhhhhhhhhhhhhh`             `.::-.
 /`-:+shhhhhhhhhhhhhh`            .-/+shh
 /      .:/ohhhhhhhhh`       .:/ohhhhhhhh
 /           `-:+shhh`  ..:+shhhhhhhhhhhh
 /                 .:ohhhhhhhhhhhhhhhhhhh
 /                  `hhhhhhhhhhhhhhhhhhhh
 /                  `hhhhhhhhhhhhhhhhhhhh
 /                  `hhhhhhhhhhhhhhhhhhhh
 /                  `hhhhhhhhhhhhhhhhhhhh
 /      .+o+        `hhhhhhhhhhhhhhhhhhhh
 /     -hhhhh       `hhhhhhhhhhhhhhhhhhhh
 /     ohhhhho      `hhhhhhhhhhhhhhhhhhhh
 /:::+`hhhhoos`     `hhhhhhhhhhhhhhhhhs+`
    `--/:`   /:     `hhhhhhhhhhhho/-
             -/:.   `hhhhhhs+:-`
                ::::/ho/-`"#;

const ARCH_LABS: &str = r#"${c1}                     'c'
                    'kKk,
                   .dKKKx.
                  .oKXKXKd.
                 .l0XXXXKKo.
                 c0KXXXXKX0l.
                :0XKKOxxOKX0l.
               :OXKOc. .c0XX0l.
              :OK0o. ${c4}...${c1}'dKKX0l.
             :OX0c  ${c4};xOx'${c1}'dKXX0l.
            :0KKo.${c4}.o0XXKd'.${c1}lKXX0l.
           c0XKd.${c4}.oKXXXXKd..${c1}oKKX0l.
         .c0XKk;${c4}.l0K0OO0XKd..${c1}oKXXKo.
        .l0XXXk:${c4},dKx,.'l0XKo.${c1}.kXXXKo.
       .o0XXXX0d,${c4}:x;   .oKKx'${c1}.dXKXXKd.
      .oKXXXXKK0c.${c4};.    :00c'${c1}cOXXXXXKd.
     .dKXXXXXXXXk,${c4}.     cKx'${c1}'xKXXXXXXKx'
    'xKXXXXK0kdl:.     ${c4}.ok; ${c1}.cdk0KKXXXKx'
   'xKK0koc,..         ${c4}'c, ${c1}    ..,cok0KKk,
  ,xko:'.             ${c4}.. ${c1}           .':okx;
 .,'.                                   .',."#;

const ARCH_CRAFT: &str = r#"${c1}                   -m:
                  :NMM+      .+
                 +MMMMMo    -NMy
                sMMMMMMMy  -MMMMh`
               yMMMMMMMMMd` oMMMMd`
             `dMMMMMMMMMMMm. /MMMMm-
            .mMMMMMm-dMMMMMN- :NMMMN:
           -NMMMMMd`  yMMMMMN: .mMMMM/
          :NMMMMMy     sMMMMMM+ `dMMMMo
         +MMMMMMs       +MMMMMMs `hMMMMy
        oMMMMMMMds-      :NMMMMMy  sMMMMh`
       yMMMMMNoydMMmo`    -NMMMMMd` +MMMMd.
     `dMMMMMN-   `:yNNs`   .mMMMMMm. /MMMMm-
    .mMMMMMm.        :hN/   `dMMMMMN- -NMMMN:
   -NMMMMMd`           -hh`  `yMMMMMN: .mMMMM/
  :NMMMMMy         `s`   :h.   oMMMMMM+ `-----
 +MMMMMMo         .dMm.   `o.   +MMMMMMo
sMMMMMM+         .mMMMN:    :`   :NMMMMMy"#;

const ARCOLINUX_SMALL: &str = r#"${c2}          A
         ooo
        ooooo
       ooooooo
      ooooooooo
     ooooo ooooo
    ooooo   ooooo
   ooooo     ooooo
  ooooo  ${c1}<oooooooo>${c2}
 ooooo      ${c1}<oooooo>${c2}
ooooo          ${c1}<oooo>${c2}"#;
const ARCOLINUX: &str = r#"${c2}                    /-
                   ooo:
                  yoooo/
                 yooooooo
                yooooooooo
               yooooooooooo
             .yooooooooooooo
            .oooooooooooooooo
           .oooooooarcoooooooo
          .ooooooooo-oooooooooo
         .ooooooooo-  oooooooooo
        :ooooooooo.    :ooooooooo
       :ooooooooo.      :ooooooooo
      :oooarcooo         .oooarcooo
     :ooooooooy           .ooooooooo
    :ooooooooo   ${c1}/ooooooooooooooooooo${c2}
   :ooooooooo      ${c1}.-ooooooooooooooooo.${c2}
  ooooooooo-             ${c1}-ooooooooooooo.${c2}
 ooooooooo-                 ${c1}.-oooooooooo.${c2}
ooooooooo.                     ${c1}-ooooooooo${c2}"#;

const APRICITY: &str = r#"${c2}                                    ./o-
          ``...``              `:. -/:
     `-+ymNMMMMMNmho-`      :sdNNm/
   `+dMMMMMMMMMMMMMMMmo` sh:.:::-
  /mMMMMMMMMMMMMMMMMMMMm/`sNd/
 oMMMMMMMMMMMMMMMMMMMMMMMs -`
:MMMMMMMMMMMMMMMMMMMMMMMMM/
NMMMMMMMMMMMMMMMMMMMMMMMMMd
MMMMMMMmdmMMMMMMMMMMMMMMMMd
MMMMMMy` .mMMMMMMMMMMMmho:`
MMMMMMNo/sMMMMMMMNdy+-.`-/
MMMMMMMMMMMMNdy+:.`.:ohmm:
MMMMMMMmhs+-.`.:+ymNMMMy.
MMMMMM/`.-/ohmNMMMMMMy-
MMMMMMNmNNMMMMMMMMmo.
MMMMMMMMMMMMMMMms:`
MMMMMMMMMMNds/.
dhhyys+/-`"#;

const ARCH_STRIKE: &str = r#"${c1}                   *   
                  **.
                 ****
                ******
                *******
              ** *******
             **** *******
            ${c1}****${c2}_____${c1}***${c2}/${c1}*
           ***${c2}/${c1}*******${c2}//${c1}***
          **${c2}/${c1}********${c2}///${c1}*${c2}/${c1}**
         **${c2}/${c1}*******${c2}////${c1}***${c2}/${c1}**
        **${c2}/${c1}****${c2}//////.,${c1}****${c2}/${c1}**
       ***${c2}/${c1}*****${c2}/////////${c1}**${c2}/${c1}***
      ****${c2}/${c1}****    ${c2}/////${c1}***${c2}/${c1}****
     ******${c2}/${c1}***  ${c2}////   ${c1}**${c2}/${c1}******
    ********${c2}/${c1}* ${c2}///      ${c1}*${c2}/${c1}********
  ,******     ${c2}// ______ /    ${c1}******,"#;

const ARCH_MERGE: &str = r#"$${c1}                  y:
                  sMN-
                 +MMMm`
                /MMMMMd`
               :NMMMMMMy
              -NMMMMMMMMs
             .NMMMMMMMMMM+
            .mMMMMMMMMMMMM+
            oNMMMMMMMMMMMMM+
          `+:-+NMMMMMMMMMMMM+
          .sNMNhNMMMMMMMMMMMM/
        `hho/sNMMMMMMMMMMMMMMM/
       `.`omMMmMMMMMMMMMMMMMMMM+
      .mMNdshMMMMd+::oNMMMMMMMMMo
     .mMMMMMMMMM+     `yMMMMMMMMMs
    .NMMMMMMMMM/        yMMMMMMMMMy
   -NMMMMMMMMMh         `mNMMMMMMMMd`
  /NMMMNds+:.`             `-/oymMMMm.
 +Mmy/.                          `:smN:
/+.                                  -o."#;

const ARTIX: &str = r#"${c1}                   '
                  'o'
                 'ooo'
                'ooxoo'
               'ooxxxoo'
              'oookkxxoo'
             'oiioxkkxxoo'
            ':;:iiiioxxxoo'
               `'.;::ioxxoo'
          '-.      `':;jiooo'
         'oooio-..     `'i:io'
        'ooooxxxxoio:,.   `'-;'
       'ooooxxxxxkkxoooIi:-.  `'
      'ooooxxxxxkkkkxoiiiiiji'
     'ooooxxxxxkxxoiiii:'`     .i'
    'ooooxxxxxoi:::'`       .;ioxo'
   'ooooxooi::'`         .:iiixkxxo'
  'ooooi:'`                `'';ioxxo'
 'i:'`                          '':io'
'`                                   `'"#;

const ARYA: &str = r#"${c1}                `oyyy/${c2}-yyyyyy+
${c1}               -syyyy/${c2}-yyyyyy+
${c1}              .syyyyy/${c2}-yyyyyy+
${c1}              :yyyyyy/${c2}-yyyyyy+
${c1}           `/ :yyyyyy/${c2}-yyyyyy+
${c1}          .+s :yyyyyy/${c2}-yyyyyy+
${c1}         .oys :yyyyyy/${c2}-yyyyyy+
${c1}        -oyys :yyyyyy/${c2}-yyyyyy+
${c1}       :syyys :yyyyyy/${c2}-yyyyyy+
${c1}      /syyyys :yyyyyy/${c2}-yyyyyy+
${c1}     +yyyyyys :yyyyyy/${c2}-yyyyyy+
${c1}   .oyyyyyyo. :yyyyyy/${c2}-yyyyyy+ ---------
${c1}  .syyyyyy+`  :yyyyyy/${c2}-yyyyy+-+syyyyyyyy
${c1} -syyyyyy/    :yyyyyy/${c2}-yyys:.syyyyyyyyyy
${c1}:syyyyyy/     :yyyyyy/${c2}-yyo.:syyyyyyyyyyy"#;

const ASTEROIDOS: &str = r#"${c1}                    ***
${c1}                   *****
${c1}                **********
${c1}              ***************
${c1}           *///****////****////.
${c2}         (/////// /////// ///////(
${c2}      /(((((//*     //,     //((((((.
${c2}    (((((((((((     (((        ((((((((
${c2} *(((((((((((((((((((((((        ((((((((
${c3}    (((((#(((((((#(((((        ((#(((((
${c3}     (#(#(#####(#(#,       ####(#(#
${c3}         #########        ########
${c3}           /########   ########
${c4}              #######%#######
${c4}                (#%%%%%%%#
${c4}                   %%%%%
${c4}                    %%%"#;

const AIX: &str = r#"${c1}           `:+ssssossossss+-`
        .oys///oyhddddhyo///sy+.
      /yo:+hNNNNNNNNNNNNNNNNh+:oy/
    :h/:yNNNNNNNNNNNNNNNNNNNNNNy-+h:
  `ys.yNNNNNNNNNNNNNNNNNNNNNNNNNNy.ys
 `h+-mNNNNNNNNNNNNNNNNNNNNNNNNNNNNm-oh
 h+-NNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNN.oy
/d`mNNNNNNN/::mNNNd::m+:/dNNNo::dNNNd`m:
h//NNNNNNN: . .NNNh  mNo  od. -dNNNNN:+y
N.sNNNNNN+ -N/ -NNh  mNNd.   sNNNNNNNo-m
N.sNNNNNs  +oo  /Nh  mNNs` ` /mNNNNNNo-m
h//NNNNh  ossss` +h  md- .hm/ `sNNNNN:+y
:d`mNNN+/yNNNNNd//y//h//oNNNNy//sNNNd`m-
 yo-NNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNm.ss
 `h+-mNNNNNNNNNNNNNNNNNNNNNNNNNNNNm-oy
   sy.yNNNNNNNNNNNNNNNNNNNNNNNNNNs.yo
    :h+-yNNNNNNNNNNNNNNNNNNNNNNs-oh-
      :ys:/yNNNNNNNNNNNNNNNmy/:sy:
        .+ys///osyhhhhys+///sy+.
            -/osssossossso/-"#;

const BEDROCK: &str = r#"${c1}--------------------------------------
--------------------------------------
--------------------------------------
---${c2}\\\\\\\\\\\\${c1}-----------------------
----${c2}\\\      \\\${c1}----------------------
-----${c2}\\\      \\\${c1}---------------------
------${c2}\\\      \\\\\\\\\\\\\\\\\${c1}------
-------${c2}\\\                    \\\${c1}-----
--------${c2}\\\                    \\\${c1}----
---------${c2}\\\        ______      \\\${c1}---
----------${c2}\\\                   ///${c1}---
-----------${c2}\\\                 ///${c1}----
------------${c2}\\\               ///${c1}-----
-------------${c2}\\\////////////////${c1}------
--------------------------------------
--------------------------------------
--------------------------------------"#;

const BITRIG: &str = r#"${c1}   `hMMMMN+
   -MMo-dMd`
   oMN- oMN`
   yMd  /NM:
  .mMmyyhMMs
  :NMMMhsmMh
  +MNhNNoyMm-
  hMd.-hMNMN:
  mMmsssmMMMo
 .MMdyyhNMMMd
 oMN.`/dMddMN`
 yMm/hNm+./MM/
.dMMMmo.``.NMo
:NMMMNmmmmmMMh
/MN/-------oNN:
hMd.       .dMh
sm/         /ms"#;

const BLACK_ARCH: &str = r#"${c3}                   00
                   11
                  ====${c1}
                  .${c3}//${c1}
                 `o${c3}//${c1}:
                `+o${c3}//${c1}o:
               `+oo${c3}//${c1}oo:
               -+oo${c3}//${c1}oo+:
             `/:-:+${c3}//${c1}ooo+:
            `/+++++${c3}//${c1}+++++:
           `/++++++${c3}//${c1}++++++:
          `/+++o${c2}ooo${c3}//${c2}ooo${c1}oooo/`
${c2}         ${c1}./${c2}ooosssso${c3}//${c2}osssssso${c1}+`
${c2}        .oossssso-`${c3}//${c1}`/ossssss+`
       -osssssso.  ${c3}//${c1}  :ssssssso.
      :osssssss/   ${c3}//${c1}   osssso+++.
     /ossssssss/   ${c3}//${c1}   +ssssooo/-
   `/ossssso+/:-   ${c3}//${c1}   -:/+osssso+-
  `+sso+:-`        ${c3}//${c1}       `.-/+oso:
 `++:.             ${c3}//${c1}            `-/+/
 .`                ${c3}/${c1}                `/"#;

const BLAG: &str = r#"${c1}             d
            ,MK:
            xMMMX:
           .NMMMMMX;
           lMMMMMMMM0clodkO0KXWW:
           KMMMMMMMMMMMMMMMMMMX'
      .;d0NMMMMMMMMMMMMMMMMMMK.
 .;dONMMMMMMMMMMMMMMMMMMMMMMx
'dKMMMMMMMMMMMMMMMMMMMMMMMMl
   .:xKWMMMMMMMMMMMMMMMMMMM0.
       .:xNMMMMMMMMMMMMMMMMMK.
          lMMMMMMMMMMMMMMMMMMK.
          ,MMMMMMMMWkOXWMMMMMM0
          .NMMMMMNd.     `':ldko
           OMMMK:
           oWk,
           ;:"#;

const BLANK_ON: &str = r#"${c2}        `./ohdNMMMMNmho+.` ${c1}       .+oo:`
${c2}      -smMMMMMMMMMMMMMMMMmy-`    ${c1}`yyyyy+
${c2}   `:dMMMMMMMMMMMMMMMMMMMMMMd/`  ${c1}`yyyyys
${c2}  .hMMMMMMMNmhso/++symNMMMMMMMh- ${c1}`yyyyys
${c2} -mMMMMMMms-`         -omMMMMMMN-${c1}.yyyyys
${c2}.mMMMMMMy.              .yMMMMMMm:${c1}yyyyys
${c2}sMMMMMMy                 `sMMMMMMh${c1}yyyyys
${c2}NMMMMMN:                  .NMMMMMN${c1}yyyyys
${c2}MMMMMMm.                   NMMMMMN${c1}yyyyys
${c2}hMMMMMM+                  /MMMMMMN${c1}yyyyys
${c2}:NMMMMMN:                :mMMMMMM+${c1}yyyyys
${c2} oMMMMMMNs-            .sNMMMMMMs.${c1}yyyyys
${c2}  +MMMMMMMNho:.`  `.:ohNMMMMMMNo ${c1}`yyyyys
${c2}   -hMMMMMMMMNNNmmNNNMMMMMMMMh-  ${c1}`yyyyys
${c2}     :yNMMMMMMMMMMMMMMMMMMNy:`   ${c1}`yyyyys
${c2}       .:sdNMMMMMMMMMMNds/.      ${c1}`yyyyyo
${c2}           `.:/++++/:.`           ${c1}:oys+."#;

const BLUE_LIGHT: &str = r#"${c1}              oMMNMMMMMMMMMMMMMMMMMMMMMM
              oMMMMMMMMMMMMMMMMMMMMMMMMM
              oMMMMMMMMMMMMMMMMMMMMMMMMM
              oMMMMMMMMMMMMMMMMMMMMMMMMM
              -+++++++++++++++++++++++mM${c2}
             ```````````````````````..${c1}dM${c2}
           ```````````````````````....${c1}dM${c2}
         ```````````````````````......${c1}dM${c2}
       ```````````````````````........${c1}dM${c2}
     ```````````````````````..........${c1}dM${c2}
   ```````````````````````............${c1}dM${c2}
.::::::::::::::::::::::-..............${c1}dM${c2}
 `-+yyyyyyyyyyyyyyyyyyyo............${c1}+mMM${c2}
     -+yyyyyyyyyyyyyyyyo..........${c1}+mMMMM${c2}
        ./syyyyyyyyyyyyo........${c1}+mMMMMMM${c2}
           ./oyyyyyyyyyo......${c1}+mMMMMMMMM${c2}
              omdyyyyyyo....${c1}+mMMMMMMMMMM${c2}
              ${c1}oMMM${c2}mdhyyo..${c1}+mMMMMMMMMMMMM
              oNNNNNNm${c2}dso${c1}mMMMMMMMMMMMMMM"#;

const BODHI: &str = r#"${c1}|           ${c2},,mmKKKKKKKKWm,,
 ${c1}'      ${c2},aKKP${c1}LL**********|L*${c2}TKp,
   ${c1}t  ${c2}aKP${c1}L**```          ```**L${c2}*Kp
    IX${c1}EL${c3}L,wwww,              ${c1}``*||${c2}Kp
  ,#P${c1}L|${c3}KKKpPP@IPPTKmw,          ${c1}`*||${c2}K
 ,K${c1}LL*${c3}{KKKKKKPPb$KPhpKKPKp        ${c1}`||${c2}K
 #${c1}PL  ${c3}!KKKKKKPhKPPP$KKEhKKKKp      ${c1}`||${c2}K
!H${c1}L*   ${c3}1KKKKKKKphKbPKKKKKK$KKp      ${c1}`|I${c2}W
$${c1}bL     ${c3}KKKKKKKKBQKhKbKKKKKKKK       ${c1}|I${c2}N
$${c1}bL     ${c3}!KKKKKKKKKKNKKKKKKKPP`       ${c1}|I${c2}b
TH${c1}L*     ${c3}TKKKKKK##KKKN@KKKK^         ${c1}|I${c2}M
 K@${c1}L      ${c3}*KKKKKKKKKKKEKE5          ${c1}||${c2}K
 `NL${c1}L      ${c3}`KKKKKKKKKK"```|L       ${c1}||${c2}#P
  `K@${c1}LL       ${c3}`"**"`        ${c1}'.   :||${c2}#P
    Yp${c1}LL                      ${c1}' |L${c2}$M`
     `Tp${c1}pLL,                ,|||${c2}p'L
        "Kpp${c1}LL++,.,    ,,|||$${c2}#K*   ${c1}'.
           ${c2}`"MKWpppppppp#KM"`        ${c1}`h,"#;

const BONSAI: &str = r#"${c2}   ,####,
   ${c2}#######,  ${c2},#####,
   ${c2}#####',#  ${c2}'######
    ${c2}''###'${c3}';,,,'${c2}###'
   ${c3}       ,;  ''''
   ${c3}      ;;;   ${c2},#####,
   ${c3}     ;;;'  ,,;${c2};;###
   ${c3}     ';;;;''${c2}'####'
   ${c3}      ;;;
   ${c3}   ,.;;';'',,,
   ${c3}  '     '
${c1} #
 #                        O
 ##, ,##,',##, ,##  ,#,   ,
 # # #  # #''# #,,  # #   #
 '#' '##' #  #  ,,# '##;, #"#;

const BSD: &str = r#"${c1}             ,        ,
            /(        )`
            \ \___   / |
            /- _  `-/  '
           (${c2}/\/ \ ${c1}\   /\
           ${c2}/ /   | `    ${c1}\
           ${c3}O O   ${c2}) ${c1}/    |
           ${c2}`-^--'${c1}`<     '
          (_.)  _  )   /
           `.___/`    /
             `-----' /
${c4}<----.     __ / __   \
${c4}<----|====${c1}O)))${c4}==${c1}) \) /${c4}====|
<----'    ${c1}`--' `.__,' \
             |        |
              \       /       /\
         ${c5}______${c1}( (_  / \______/
       ${c5},'  ,-----'   |
       `--{__________)"#;

const BUNSEN_LABS: &str = r#"${c1}        `++
      -yMMs
    `yMMMMN`
   -NMMMMMMm.
  :MMMMMMMMMN-
 .NMMMMMMMMMMM/
 yMMMMMMMMMMMMM/
`MMMMMMNMMMMMMMN.
-MMMMN+ /mMMMMMMy
-MMMm`   `dMMMMMM
`MMN.     .NMMMMM.
 hMy       yMMMMM`
 -Mo       +MMMMN
  /o       +MMMMs
           +MMMN`
           hMMM:
          `NMM/
          +MN:
          mh.
         -/"#;

const CALCULATE: &str = r#"${c1}                              ......
                           ,,+++++++,.
                         .,,,....,,,${c2}+**+,,.${c1}
                       ............,${c2}++++,,,${c1}
                      ...............
                    ......,,,........
                  .....+*#####+,,,*+.
              .....,*###############,..,,,,,,..
           ......,*#################*..,,,,,..,,,..
         .,,....*####################+***+,,,,...,++,
       .,,..,..*#####################*,
     ,+,.+*..*#######################.
   ,+,,+*+..,########################*
.,++++++.  ..+##**###################+
.....      ..+##***#################*.
           .,.*#*****##############*.
           ..,,*********#####****+.
     ${c2}.,++*****+++${c1}*****************${c2}+++++,.${c1}
      ${c2},++++++**+++++${c1}***********${c2}+++++++++,${c1}
     ${c2}.,,,,++++,..  .,,,,,.....,+++,.,,${c1}"#;

const CARBS: &str = r#"${c2}             ..........
          ..,;:ccccccc:;'..
       ..,clllc:;;;;;:cllc,.
      .,cllc,...     ..';;'.
     .;lol;..           ..
    .,lol;.
    .coo:.
   .'lol,.
   .,lol,.
   .,lol,.
    'col;.
    .:ooc'.
    .'col:.
     .'cllc'..          .''.
      ..:lolc,'.......',cll,.
        ..;cllllccccclllc;'.
          ...',;;;;;;,,...
                ....."#;

const CBL_MARINER: &str = r#"${c1}                    .
                  :-  .
                :==. .=:
              :===:  -==:
            :-===:  .====:
          :-====-   -=====:
         -======   :=======:
        -======.  .=========:
       -======:   -==========.
      -======-    -===========.
     :======-      :===========.
    :=======.       .-==========.
   :=======:          -==========.
  :=======-            :==========.
 :=======-              .-========-
:--------.                :========-
                    ..:::--=========-
            ..::---================-=-"#;

const CEL_OS: &str = r#"${c4}                     .,cmmmmmmmmmmmc,.
                .,cmMMMMMMMMMMMMMMMMMMMMmc.
             .cMMMMMMMMMMMMMMMMMMMMMMMMMMMmc.
           .cMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMc.
         ,:MMM ${c3}####################################${c4}
        cMMMMMMmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmc.
       .MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM.
      .MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMc
      "******************************MMMMMMMMMMMMMc:
${c3}#################################### ${c4}MMMMMMMMMMMMMc
      "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM:
       "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM"
       'MMMMMMMMM*******************************:
        \"MMMMMM ${c3}#####################################
         ${c4}`:MMMMMMmmmmmmmmmmmmmmmmmmmmmmmmmmmmm;
           `"MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM"
             `":MMMMMMMMMMMMMMMMMMMMMMMMM;'
                `":MMMMMMMMMMMMMMMMMMM:"
                     "************""#;

const CENTOS_SMALL: &str = r#"${c2} ____${c1}^${c4}____
${c2} |\  ${c1}|${c4}  /|
${c2} | \ ${c1}|${c4} / |
${c4}<---- ${c3}---->
${c3} | / ${c2}|${c1} \ |
${c3} |/__${c2}|${c1}__\|
${c2}     v"#;
const CENTOS: &str = r#"${c1}                 ..
               .PLTJ.
              <><><><>
     ${c2}KKSSV' 4KKK ${c1}LJ${c4} KKKL.'VSSKK
     ${c2}KKV' 4KKKKK ${c1}LJ${c4} KKKKAL 'VKK
     ${c2}V' ' 'VKKKK ${c1}LJ${c4} KKKKV' ' 'V
     ${c2}.4MA.' 'VKK ${c1}LJ${c4} KKV' '.4Mb.
${c4}   . ${c2}KKKKKA.' 'V ${c1}LJ${c4} V' '.4KKKKK ${c3}.
${c4} .4D ${c2}KKKKKKKA.'' ${c1}LJ${c4} ''.4KKKKKKK ${c3}FA.
${c4}<QDD ++++++++++++  ${c3}++++++++++++ GFD>
${c4} 'VD ${c3}KKKKKKKK'.. ${c2}LJ ${c1}..'KKKKKKKK ${c3}FV
${c4}   ' ${c3}VKKKKK'. .4 ${c2}LJ ${c1}K. .'KKKKKV ${c3}'
     ${c3} 'VK'. .4KK ${c2}LJ ${c1}KKA. .'KV'
     ${c3}A. . .4KKKK ${c2}LJ ${c1}KKKKA. . .4
     ${c3}KKA. 'KKKKK ${c2}LJ ${c1}KKKKK' .4KK
     ${c3}KKSSA. VKKK ${c2}LJ ${c1}KKKV .4SSKK
${c2}              <><><><>
               'MKKM'
                 ''"#;

const CHAKRA: &str = r#"${c1}     _ _ _        "kkkkkkkk.
   ,kkkkkkkk.,    'kkkkkkkkk,
   ,kkkkkkkkkkkk., 'kkkkkkkkk.
  ,kkkkkkkkkkkkkkkk,'kkkkkkkk,
 ,kkkkkkkkkkkkkkkkkkk'kkkkkkk.
  "''"''',;::,,"''kkk''kkkkk;   __
      ,kkkkkkkkkk, "k''kkkkk' ,kkkk
    ,kkkkkkk' ., ' .: 'kkkk',kkkkkk
  ,kkkkkkkk'.k'   ,  ,kkkk;kkkkkkkkk
 ,kkkkkkkk';kk 'k  "'k',kkkkkkkkkkkk
.kkkkkkkkk.kkkk.'kkkkkkkkkkkkkkkkkk'
;kkkkkkkk''kkkkkk;'kkkkkkkkkkkkk''
'kkkkkkk; 'kkkkkkkk.,""''"''""
  ''kkkk;  'kkkkkkkkkk.,
     ';'    'kkkkkkkkkkkk.,
             ';kkkkkkkkkk'
               ';kkkkkk'
                  "''""#;

const CHALET_OS: &str = r#"${c1}             `.//+osso+/:``
         `/sdNNmhyssssydmNNdo:`
       :hNmy+-`          .-+hNNs-
     /mMh/`       `+:`       `+dMd:
   .hMd-        -sNNMNo.  /yyy  /mMs`
  -NM+       `/dMd/--omNh::dMM   `yMd`
 .NN+      .sNNs:/dMNy:/hNmo/s     yMd`
 hMs    `/hNd+-smMMMMMMd+:omNy-    `dMo
:NM.  .omMy:/hNMMMMMMMMMMNy:/hMd+`  :Md`
/Md` `sm+.omMMMMMMMMMMMMMMMMd/-sm+  .MN:
/Md`      MMMMMMMMMMMMMMMMMMMN      .MN:
:NN.      MMMMMMm....--NMMMMMN      -Mm.
`dMo      MMMMMMd      mMMMMMN      hMs
 -MN:     MMMMMMd      mMMMMMN     oMm`
  :NM:    MMMMMMd      mMMMMMN    +Mm-
   -mMy.  mmmmmmh      dmmmmmh  -hMh.
     oNNs-                    :yMm/
      .+mMdo:`            `:smMd/`
         -ohNNmhsoo++osshmNNh+.
            `./+syyhhyys+:``"#;

const CHEAPEAU: &str = r#"${c1}               .-/-.
            ////////.
          ////////${c2}y+${c1}//.
        ////////${c2}mMN${c1}/////.
      ////////${c2}mMN+${c1}////////.
    ////////////////////////.
  /////////+${c2}shhddhyo${c1}+////////.
 ////////${c2}ymMNmdhhdmNNdo${c1}///////.
///////+${c2}mMms${c1}////////${c2}hNMh${c1}///////.
///////${c2}NMm+${c1}//////////${c2}sMMh${c1}///////
//////${c2}oMMNmmmmmmmmmmmmMMm${c1}///////
//////${c2}+MMmssssssssssssss+${c1}///////
`//////${c2}yMMy${c1}////////////////////
 `//////${c2}smMNhso++oydNm${c1}////////
  `///////${c2}ohmNMMMNNdy+${c1}///////
    `//////////${c2}++${c1}//////////
       `////////////////.
           -////////-"#;

const CHROME: &str = r#"${c2}            .,:loool:,.
        .,coooooooooooooc,.
     .,lllllllllllllllllllll,.
    ;ccccccccccccccccccccccccc;
${c1}  '${c2}ccccccccccccccccccccccccccccc.
${c1} ,oo${c2}c::::::::okO${c5}000${c3}0OOkkkkkkkkkkk:
${c1}.ooool${c2};;;;:x${c5}K0${c4}kxxxxxk${c5}0X${c3}K0000000000.
${c1}:oooool${c2};,;O${c5}K${c4}ddddddddddd${c5}KX${c3}000000000d
${c1}lllllool${c2};l${c5}N${c4}dllllllllllld${c5}N${c3}K000000000
${c1}lllllllll${c2}o${c5}M${c4}dccccccccccco${c5}W${c3}K000000000
${c1};cllllllllX${c5}X${c4}c:::::::::c${c5}0X${c3}000000000d
${c1}.ccccllllllO${c5}Nk${c4}c;,,,;cx${c5}KK${c3}0000000000.
${c1} .cccccclllllxOO${c5}OOO${c1}Okx${c3}O0000000000;
${c1}  .:ccccccccllllllllo${c3}O0000000OOO,
${c1}    ,:ccccccccclllcd${c3}0000OOOOOOl.
${c1}      '::ccccccccc${c3}dOOOOOOOkx:.
${c1}        ..,::cccc${c3}xOOOkkko;.
${c1}            ..,:${c3}dOkxl:."#;

const CLEANJARO_SMALL: &str = r#"${c1}█████ ██████████
█████ ██████████
█████
█████
█████
████████████████
████████████████"#;

const CLEANJARO: &str = r#"${c1}███████▌ ████████████████
███████▌ ████████████████
███████▌ ████████████████
███████▌
███████▌
███████▌
███████▌
███████▌
█████████████████████████
█████████████████████████
█████████████████████████
▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀"#;

const CLEAROS: &str = r#"${c1}             `.--::::::--.`
         .-:////////////////:-.
      `-////////////////////////-`
     -////////////////////////////-
   `//////////////-..-//////////////`
  ./////////////:      ://///////////.
 `//////:..-////:      :////-..-//////`
 ://////`    -///:.``.:///-`    ://///:
`///////:.     -////////-`    `:///////`
.//:--////:.     -////-`    `:////--://.
./:    .////:.     --`    `:////-    :/.
`//-`    .////:.        `:////-    `-//`
 :///-`    .////:.    `:////-    `-///:
 `/////-`    -///:    :///-    `-/////`
  `//////-   `///:    :///`   .//////`
   `:////:   `///:    :///`   -////:`
     .://:   `///:    :///`   -//:.
       .::   `///:    :///`   -:.
             `///:    :///`
              `...    ...`"#;

const CLEAR_OS: &str = r#"${c1}          BBB
       BBBBBBBBB
     BBBBBBBBBBBBBBB
   BBBBBBBBBBBBBBBBBBBB
   BBBBBBBBBBB         BBB
  BBBBBBBB${c2}YYYYY
${c1}  BBBBBBBB${c2}YYYYYY
${c1}  BBBBBBBB${c2}YYYYYYY
${c1}  BBBBBBBBB${c2}YYYYY${c3}W
${c4} GG${c1}BBBBBBBY${c2}YYYY${c3}WWW
${c4} GGG${c1}BBBBBBB${c2}YY${c3}WWWWWWWW
${c4} GGGGGG${c1}BBBBBB${c3}WWWWWWWW
${c4} GGGGGGGG${c1}BBBB${c3}WWWWWWWW
${c4}GGGGGGGGGGG${c1}BBB${c3}WWWWWWW
${c4}GGGGGGGGGGGGG${c1}B${c3}WWWWWW
${c4}GGGGGGGG${c3}WWWWWWWWWWW
${c4}GG${c3}WWWWWWWWWWWWWWWW
 WWWWWWWWWWWWWWWW
      WWWWWWWWWW
          WWW"#;

const CLOVER: &str = r#"${c1}               `omo``omo`
             `oNMMMNNMMMNo`
           `oNMMMMMMMMMMMMNo`
          oNMMMMMMMMMMMMMMMMNo
          `sNMMMMMMMMMMMMMMNs`
     `omo`  `sNMMMMMMMMMMNs`  `omo`
   `oNMMMNo`  `sNMMMMMMNs`  `oNMMMNo`
 `oNMMMMMMMNo`  `oNMMNs`  `oNMMMMMMMNo`
oNMMMMMMMMMMMNo`  `sy`  `oNMMMMMMMMMMMNo
`sNMMMMMMMMMMMMNo.${c2}oNNs${c1}.oNMMMMMMMMMMMMNs`
`oNMMMMMMMMMMMMNs.${c2}oNNs${c1}.oNMMMMMMMMMMMMNo`
oNMMMMMMMMMMMNs`  `sy`  `oNMMMMMMMMMMMNo
 `oNMMMMMMMNs`  `oNMMNo`  `oNMMMMMMMNs`
   `oNMMMNs`  `sNMMMMMMNs`  `oNMMMNs`
     `oNs`  `sNMMMMMMMMMMNs`  `oNs`
          `sNMMMMMMMMMMMMMMNs`
          +NMMMMMMMMMMMMMMMMNo
           `oNMMMMMMMMMMMMNo`
             `oNMMMNNMMMNs`
               `omo``oNs`"#;

const CONDRES: &str = r#"${c1}syyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy+${c3}.+.
${c1}`oyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy+${c3}:++.
${c2}/o${c1}+oyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy/${c3}oo++.
${c2}/y+${c1}syyyyyyyyyyyyyyyyyyyyyyyyyyyyy${c3}+ooo++.
${c2}/hy+${c1}oyyyhhhhhhhhhhhhhhyyyyyyyyy${c3}+oo+++++.
${c2}/hhh+${c1}shhhhhdddddhhhhhhhyyyyyyy${c3}+oo++++++.
${c2}/hhdd+${c1}oddddddddddddhhhhhyyyys${c3}+oo+++++++.
${c2}/hhddd+${c1}odmmmdddddddhhhhyyyy${c3}+ooo++++++++.
${c2}/hhdddmo${c1}odmmmdddddhhhhhyyy${c3}+oooo++++++++.
${c2}/hdddmmms${c1}/dmdddddhhhhyyys${c3}+oooo+++++++++.
${c2}/hddddmmmy${c1}/hdddhhhhyyyyo${c3}+oooo++++++++++:
${c2}/hhdddmmmmy${c1}:yhhhhyyyyy+${c3}+oooo+++++++++++:
${c2}/hhddddddddy${c1}-syyyyyys+${c3}ooooo++++++++++++:
${c2}/hhhddddddddy${c1}-+yyyy+${c3}/ooooo+++++++++++++:
${c2}/hhhhhdddddhhy${c1}./yo:${c3}+oooooo+++++++++++++/
${c2}/hhhhhhhhhhhhhy${c1}:-.${c3}+sooooo+++++++++++///:
${c2}:sssssssssssso++${c1}${c3}`:/:--------.````````"#;

const CONTAINER_LINUX: &str = r#"${c1}                .....
          .';:cccccccc:;'.
        ':ccccclc${c3}lllllllll${c1}cc:.
     .;cccccccc${c3}lllllllllllllll${c1}c,
    ;clllccccc${c3}llllllllllllllllll${c1}c,
  .cllclccccc${c3}lllll${c2}lll${c3}llllllllllll${c1}c:
  ccclclcccc${c3}cllll${c2}kWMMNKk${c3}llllllllll${c1}c:
 :ccclclcccc${c3}llll${c2}oWMMMMMMWO${c3}lllllllll${c1}c,
.ccllllllccc${c3}clll${c2}OMMMMMMMMM0${c3}lllllllll${c1}c
.lllllclcccc${c3}llll${c2}KMMMMMMMMMMo${c3}llllllll${c1}c.
.lllllllcccc${c3}clll${c2}KMMMMMMMMN0${c3}lllllllll${c1}c.
.cclllllcccc${c3}lllld${c2}xkkxxdo${c3}llllllllllc${c1}lc
 :cccllllllcccc${c3}lllccllllcclccc${c1}cccccc;
 .ccclllllllcccccccc${c3}lll${c1}ccccclccccccc
  .cllllllllllclcccclccclccllllcllc
    :cllllllllccclcllllllllllllcc;
     .cccccccccccccclcccccccccc:.
       .;cccclccccccllllllccc,.
          .';ccccclllccc:;..
                ....."#;

const CRUX_SMALL: &str = r#"${c1}    ___
   (${c3}.· ${c1}|
   (${c2}<> ${c1}|
  / ${c3}__  ${c1}\
 ( ${c3}/  \ ${c1}/|
${c2}_${c1}/\ ${c3}__)${c1}/${c2}_${c1})
${c2}\/${c1}-____${c2}\/"#;

const CRUX: &str = r#"${c1}         odddd
      oddxkkkxxdoo
     ddcoddxxxdoool
     xdclodod  olol
     xoc  xdd  olol
     xdc  ${c2}k00${c1}Okdlol
     xxd${c2}kOKKKOkd${c1}ldd
     xdco${c2}xOkdlo${c1}dldd
     ddc:cl${c2}lll${c1}oooodo
   odxxdd${c3}xkO000kx${c1}ooxdo
  oxdd${c3}x0NMMMMMMWW0od${c1}kkxo
 oooxd${c3}0WMMMMMMMMMW0o${c1}dxkx
docldkXW${c3}MMMMMMMWWN${c1}Odolco
xx${c2}dx${c1}kxxOKN${c3}WMMWN${c1}0xdoxo::c
${c2}xOkkO${c1}0oo${c3}odOW${c2}WW${c1}XkdodOxc:l
${c2}dkkkxkkk${c3}OKX${c2}NNNX0Oxx${c1}xc:cd
${c2} odxxdx${c3}xllod${c2}ddooxx${c1}dc:ldo
${c2}   lodd${c1}dolccc${c2}ccox${c1}xoloo"#;

const CRYSTAL_LINUX: &str = r#"${c1}                        mysssym
${c1}                      mysssym
${c1}                    mysssym
${c1}                  mysssym
${c1}                mysssyd
${c1}              mysssyd    N
${c1}            mysssyd    mysym
${c1}          mysssyd      dysssym
${c1}        mysssyd          dysssym
${c1}      mysssyd              dysssym
${c1}      mysssyd              dysssym
${c1}        mysssyd          dysssym
${c1}          mysssyd      dysssym
${c1}            mysym    dysssym
${c1}              N    dysssym
${c1}                 dysssym
${c1}               dysssym
${c1}             dysssym
${c1}           dysssym
${c1}         dysssym"#;

const CUCUMBER: &str = r#"${c1}           `.-://++++++//:-.`
        `:/+//${c2}::--------${c1}:://+/:`
      -++/:${c2}----..........----${c1}:/++-
    .++:${c2}---...........-......---${c1}:++.
   /+:${c2}---....-::/:/--//:::-....---${c1}:+/
 `++:${c2}--.....:---::/--/::---:.....--${c1}:++`
 /+:${c2}--.....--.--::::-/::--.--.....--${c1}:+/
-o:${c2}--.......-:::://--/:::::-.......--${c1}:o-
/+:${c2}--...-:-::---:::..:::---:--:-...--${c1}:+/
o/:${c2}-...-:.:.-/:::......::/:.--.:-...-${c1}:/o
o/${c2}--...::-:/::/:-......-::::::-/-...-${c1}:/o
/+:${c2}--..-/:/:::--:::..:::--::////-..--${c1}:+/
-o:${c2}--...----::/:::/--/:::::-----...--${c1}:o-
 /+:${c2}--....://:::.:/--/:.::://:....--${c1}:+/
 `++:${c2}--...-:::.--.:..:.--.:/:-...--${c1}:++`
   /+:${c2}---....----:-..-:----....---${c1}:+/
    .++:${c2}---..................---${c1}:++.
      -/+/:${c2}----..........----${c1}:/+/-
        `:/+//${c2}::--------:::${c1}/+/:`
           `.-://++++++//:-.`"#;

const CYBER_OS: &str = r#"${c3}             !M$EEEEEEEEEEEP
            .MMMMM000000Nr.
            ${c3}&MMMMMM${c2}MMMMMMMMMMMMM9
           ${c3}~MMM${c1}MMMM${c2}MMMMMMMMMMMMC
      ${c1}"    ${c3}M${c1}MMMMMMM${c2}MMMMMMMMMMs
    ${c1}iM${c2}MMM&&${c1}MMMMMMMM${c2}MMMMMMMM\\
   ${c1}BMMM${c2}MMMMM${c1}MMMMMMM${c2}MMMMMM${c3}"
  ${c1}9MMMMM${c2}MMMMMMM${c1}MMMM${c2}MMMM${c3}MMMf-
        ${c2}sMMMMMMMM${c1}MM${c2}M${c3}MMMMMMMMM3_
         ${c2}+ffffffff${c1}P${c3}MMMMMMMMMMMM0
                    ${c2}CMMMMMMMMMMM
                      }MMMMMMMMM
                        ~MMMMMMM
                          "RMMMM
                            .PMB"#;

const DAHLIA: &str = r#"${c1}
                  .#.
                *%@@@%*
        .,,,,,(&@@@@@@@&/,,,,,.
       ,#@@@@@@@@@@@@@@@@@@@@@#.
       ,#@@@@@@@&#///#&@@@@@@@#.
     ,/%&@@@@@%/,    .,(%@@@@@&#/.
   *#&@@@@@@#,.         .*#@@@@@@&#,
 .&@@@@@@@@@(            .(@@@@@@@@@&&.
#@@@@@@@@@@(               )@@@@@@@@@@@#
 °@@@@@@@@@@(            .(@@@@@@@@@@@°
   *%@@@@@@@(.           ,#@@@@@@@%*
     ,(&@@@@@@%*.     ./%@@@@@@%(,
       ,#@@@@@@@&(***(&@@@@@@@#.
       ,#@@@@@@@@@@@@@@@@@@@@@#.
        ,*****#&@@@@@@@&(*****,
               ,/%@@@%/.
                  ,#,"#;

const DEBIAN_SMALL: &str = r#"${c1}  _____
 /  __ \
|  /    |
|  \___-
-_
  --_
${reset}"#;
const DEBIAN: &str = r#"${c2}       _,met$$$$$gg.
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
              `""""#;

const DEEPIN: &str = r#"${c1}             ............
         .';;;;;.       .,;,.
      .,;;;;;;;.       ';;;;;;;.
    .;::::::::'     .,::;;,''''',.
   ,'.::::::::    .;;'.          ';
  ;'  'cccccc,   ,' :: '..        .:
 ,,    :ccccc.  ;: .c, '' :.       ,;
.l.     cllll' ., .lc  :; .l'       l.
.c       :lllc  ;cl:  .l' .ll.      :'
.l        'looc. .   ,o:  'oo'      c,
.o.         .:ool::coc'  .ooo'      o.
 ::            .....   .;dddo      ;c
  l:...            .';lddddo.     ,o
   lxxxxxdoolllodxxxxxxxxxc      :l
    ,dxxxxxxxxxxxxxxxxxxl.     'o,
      ,dkkkkkkkkkkkkko;.    .;o;
        .;okkkkkdl;.    .,cl:.
            .,:cccccccc:,."#;

const DESAOS: &str = r#"${c1}███████████████████████
███████████████████████
███████████████████████
███████████████████████
████████               ███████
████████               ███████
████████               ███████
████████               ███████
████████               ███████
████████               ███████
████████               ███████
██████████████████████████████
██████████████████████████████
████████████████████████
████████████████████████
████████████████████████"#;

const DEVUAN: &str = r#"${c1}   ..,,;;;::;,..
           `':ddd;:,.
                 `'dPPd:,.
                     `:b$$b`.
                        'P$$$d`
                         .$$$$$`
                         ;$$$$$P
                      .:P$$$$$$`
                  .,:b$$$$$$$;'
             .,:dP$$$$$$$$b:'
      .,:;db$$$$$$$$$$Pd'`
 ,db$$$$$$$$$$$$$$b:'`
:$$$$$$$$$$$$b:'`
 `$$$$$bd:''`
   `'''`"#;

const DRACOS: &str = r#"${c1}       `-:/-
          -os:
            -os/`
              :sy+-`
               `/yyyy+.
                 `+yyyyo-
                   `/yyyys:
`:osssoooo++-        +yyyyyy/`
   ./yyyyyyo         yo`:syyyy+.
      -oyyy+         +-   :yyyyyo-
        `:sy:        `.    `/yyyyys:
           ./o/.`           .oyyso+oo:`
              :+oo+//::::///:-.`     `.`"#;

const DARK_OS: &str = r#"${c3}       ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⠢⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
${c1}⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣶⠋⡆⢹⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
${c5}⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡆⢀⣤⢛⠛⣠⣿⠀⡏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
${c6}⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣶⣿⠟⣡⠊⣠⣾⣿⠃⣠⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
${c2}⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣴⣯⣿⠀⠊⣤⣿⣿⣿⠃⣴⣧⣄⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
${c1}⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣤⣶⣿⣿⡟⣠⣶⣿⣿⣿⢋⣤⠿⠛⠉⢁⣭⣽⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
${c4}  ⠀⠀⠀⠀⠀⠀ ⠀⣠⠖⡭⢉⣿⣯⣿⣯⣿⣿⣿⣟⣧⠛⢉⣤⣶⣾⣿⣿⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
${c5}⠀⠀⠀⠀⠀⠀⠀⠀⣴⣫⠓⢱⣯⣿⢿⠋⠛⢛⠟⠯⠶⢟⣿⣯⣿⣿⣿⣿⣿⣿⣦⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
${c2}⠀⠀⠀⠀⠀⠀⢀⡮⢁⣴⣿⣿⣿⠖⣠⠐⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠉⠉⠛⠛⠛⢿⣶⣄⠀⠀⠀⠀⠀⠀⠀
${c3}⠀⠀⠀⠀⢀⣤⣷⣿⣿⠿⢛⣭⠒⠉⠀⠀⠀⣀⣀⣄⣤⣤⣴⣶⣶⣶⣿⣿⣿⣿⣿⠿⠋⠁⠀⠀⠀⠀⠀⠀⠀⠀
${c1}⠀⢀⣶⠏⠟⠝⠉⢀⣤⣿⣿⣶⣾⣿⣿⣿⣿⣿⣿⣟⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣧⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
${c6}⢴⣯⣤⣶⣿⣿⣿⣿⣿⡿⣿⣯⠉⠉⠉⠉⠀⠀⠀⠈⣿⡀⣟⣿⣿⢿⣿⣿⣿⣿⣿⣦⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
${c5}⠀⠀⠀⠉⠛⣿⣧⠀⣆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⠃⣿⣿⣯⣿⣦⡀⠀⠉⠻⣿⣦⠀⠀⠀⠀⠀⠀⠀⠀⠀
${c3}⠀⠀⠀⠀⠀⠀⠉⢿⣮⣦⠀⠀⠀⠀⠀⠀⠀⠀⠀⣼⣿⠀⣯⠉⠉⠛⢿⣿⣷⣄⠀⠈⢻⣆⠀⠀⠀⠀⠀⠀⠀⠀
${c2}⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠢⠀⠀⠀⠀⠀⠀⠀⢀⢡⠃⣾⣿⣿⣦⠀⠀⠀⠙⢿⣿⣤⠀⠙⣄⠀⠀⠀⠀⠀⠀⠀
${c6}⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⢋⡟⢠⣿⣿⣿⠋⢿⣄⠀⠀⠀⠈⡄⠙⣶⣈⡄⠀⠀⠀⠀⠀⠀
${c1}⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠐⠚⢲⣿⠀⣾⣿⣿⠁⠀⠀⠉⢷⡀⠀⠀⣇⠀⠀⠈⠻⡀⠀⠀⠀⠀⠀
${c4}⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢢⣀⣿⡏⠀⣿⡿⠀⠀⠀⠀⠀⠀⠙⣦⠀⢧⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
${c3}⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠿⣧⣾⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⣮⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
${c5}⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠙⠛"#;

const DRAGONFLY_OLD: &str = r#"  ${c1}                   .-.
                 ${c3} ()${c1}I${c3}()
            ${c1} "==.__:-:__.=="
            "==.__/~|~\__.=="
            "==._(  Y  )_.=="
 ${c2}.-'~~""~=--...,__${c1}\/|\/${c2}__,...--=~""~~'-.
(               ..=${c1}\\=${c1}/${c2}=..               )
 `'-.        ,.-"`;${c1}/=\\${c2};"-.,_        .-'`
     `~"-=-~` .-~` ${c1}|=|${c2} `~-. `~-=-"~`
          .-~`    /${c1}|=|${c2}\    `~-.
       .~`       / ${c1}|=|${c2} \       `~.
   .-~`        .'  ${c1}|=|${c2}  `.        `~-.
 (`     _,.-="`  ${c1}  |=|${c2}    `"=-.,_     `)
  `~"~"`        ${c1}   |=|${c2}           `"~"~`
                 ${c1}  /=\\
                   \\=/
                    ^"#;

const DRAGONFLY_SMALL: &str = r#"${c2}   ,${c1}_${c2},
('-_${c1}|${c2}_-')
 >--${c1}|${c2}--<
(_-'${c1}|${c2}'-_)
    ${c1}|
    |
    |"#;

const DRAGONFLY: &str = r#"${c2},--,           ${c1}|           ${c2},--,
${c2}|   `-,       ${c1},^,       ${c2},-'   |
${c2} `,    `-,   ${c3}(/ \)   ${c2},-'    ,'
${c2}   `-,    `-,${c1}/   \${c2},-'    ,-'
${c2}      `------${c1}(   )${c2}------'
${c2}  ,----------${c1}(   )${c2}----------,
${c2} |        _,-${c1}(   )${c2}-,_        |
${c2}  `-,__,-'   ${c1}\   /${c2}   `-,__,-'
${c1}              | |
              | |
              | |
              | |
              | |
              | |
              `|'"#;

const DRAUGER: &str = r#"${c1}                  -``-
                `:+``+:`
               `/++``++/.
              .++/.  ./++.
             :++/`    `/++:
           `/++:        :++/`
          ./+/-          -/+/.
         -++/.            ./++-
        :++:`              `:++:
      `/++-                  -++/`
     ./++.                    ./+/.
    -++/`                      `/++-
   :++:`                        `:++:
 `/++-                            -++/`
.:-.`..............................`.-:.
`.-/++++++++++++++++++++++++++++++++/-.`"#;

const ELEMENTARY_SMALL: &str = r#"${c2}  _______
 / ____  \\
/  |  /  /\\
|__\\ /  / |
\\   /__/  /
 \\_______/"#;
const ELEMENTARY: &str = r#"${c2}         eeeeeeeeeeeeeeeee
      eeeeeeeeeeeeeeeeeeeeeee
    eeeee  eeeeeeeeeeee   eeeee
  eeee   eeeee       eee     eeee
 eeee   eeee          eee     eeee
eee    eee            eee       eee
eee   eee            eee        eee
ee    eee           eeee       eeee
ee    eee         eeeee      eeeeee
ee    eee       eeeee      eeeee ee
eee   eeee   eeeeee      eeeee  eee
eee    eeeeeeeeee     eeeeee    eee
 eeeeeeeeeeeeeeeeeeeeeeee    eeeee
  eeeeeeee eeeeeeeeeeee      eeee
    eeeee                 eeeee
      eeeeeee         eeeeeee
         eeeeeeeeeeeeeeeee"#;

const ENDEAVOUROS_SMALL: &str = r#"${c1}                ./${c2}o${c3}.
${c1}              `:${c2}oss+${c3}-
${c1}            `:+${c2}ssssso${c3}/.
${c1}          `-/o${c2}sssssssso${c3}/.
${c1}        `-:/+${c2}ssssssssssso${c3}/.
${c1}      .://+${c2}ssssssssssssssso${c3}+:
${c1}    .:///o${c2}ssssssssssssssssso${c3}+:
${c1}  `:////${c2}sssssssssssssssssssso${c3}++.
${c1}  `..-+${c2}oossssssssssssssssso${c3}++++/`
    `::::::::::::::::::::----``"#;
const ENDEAVOUROS: &str = r#"${c1}                     ./${c2}o${c3}.
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
  `:::::::::::::::::::::::::------``"#;

const FEDORA_SMALL: &str = r#"${c1}        ,'''''.
       |   ,.  |
       |  |  '_'
  ,....|  |..
.'  ,_;|   ..'
|  |   |  |
|  ',_,'  |
 '.     ,'
   '''''"#;
const FEDORA: &str = r#"${c1}             .',;::::;,'.
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
  '::cccccccccccccc::;,."#;

const GARUDA: &str = r#"#{c3}                  __,,,,,,,_  
            _╓╗╣╫╠╠╠╠╠╠╠╠╠╠╠╠╠╕╗╗┐_ 
         ╥╢╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╥,
       ╗╠╠╠╠╠╠╠╝╜╜╜╜╝╢╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠┐
      ╣╠╠╠╠╠╠╠╠╢╣╢╗╕ , `"╘╠╠╠╠╠╠╠╠╠╠╠╠╠╠╔╥_
    ╒╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╕╙╥╥╜   `"╜╠╬╠╠╠╠╠╠╠╠╠╠╠╥,
    ╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╗╥╥╥╥╗╗╬╠╠╠╠╠╠╠╝╙╠╠╣╠╠╠╠╢┐
   ╣╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╥╬╣╠╠╠╠╠╠╠╠╗
  ╒╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╗
  ╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠
  ╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╬     ```"╜╝╢╠╠╡
 ╒╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╣,         ╘╠╪
 ╞╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╢┐        ╜ 
 `╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╗  
 ,╬╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠"╕ 
 ╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╗ 
 ╝^╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╝╣╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╠╡ 
  ╔╜`╞┘╢╛╜ ╡╢╠"╚╠╠╜╝┌╞╞"╢╠╠╠╠╠╠╠╠╠╠╣╩╢╪ 
     ╜╒"   `╜    `      ╜╙╕  └╣╠╠╠╠╕ ╞╙╖
                                ╠╠╠
                                 ╜"#;

const HASH: &str = r#"${c1}

      +   ######   +
    ###   ######   ###
  #####   ######   #####
 ######   ######   ######

####### '"\###### '"\########
#######   ######   ########
#######   ######   ########

 ###### '"\###### '"\######
  #####   ######   #####
    ###   ######   ###
      ~   ######   ~"#;

const KALI: &str = r#"${c1}..............
            ..,;:ccc,.
          ......''';lxO.
.....''''..........,:ld;
           .';;;:::;,,.x,
      ..'''.            0Xxoc:,.  ...
  ....                ,ONkc;,;cokOdc',.
 .                   OMo           ':${c2}dd${c1}o.
                    dMc               :OO;
                    0M.                 .:o.
                    ;Wd
                     ;XO,
                       ,d0Odlc;,..
                           ..',;:cdOOd::,.
                                    .:d;.':;.
                                       'd,  .'
                                         ;l   ..
                                          .o
                                            c
                                            .'
                                             ."#;

const KDE: &str = r#"${c1}             `..---+/---..`
         `---.``   ``   `.---.`
      .--.`        ``        `-:-.
    `:/:     `.----//----.`     :/-
   .:.    `---`          `--.`    .:`
  .:`   `--`                .:-    `:.
 `/    `:.      `.-::-.`      -:`   `/`
 /.    /.     `:++++++++:`     .:    .:
`/    .:     `+++++++++++/      /`   `+`
/+`   --     .++++++++++++`     :.   .+:
`/    .:     `+++++++++++/      /`   `+`
 /`    /.     `:++++++++:`     .:    .:
 ./    `:.      `.:::-.`      -:`   `/`
  .:`   `--`                .:-    `:.
   .:.    `---`          `--.`    .:`
    `:/:     `.----//----.`     :/-
      .-:.`        ``        `-:-.
         `---.``   ``   `.---.`
             `..---+/---..`"#;

const KUBUNTU: &str = r#"${c1}           `.:/ossyyyysso/:.
        .:oyyyyyyyyyyyyyyyyyyo:`
      -oyyyyyyyo${c2}dMMy${c1}yyyyyyysyyyyo-
    -syyyyyyyyyy${c2}dMMy${c1}oyyyy${c2}dmMMy${c1}yyyys-
   oyyys${c2}dMy${c1}syyyy${c2}dMMMMMMMMMMMMMy${c1}yyyyyyo
 `oyyyy${c2}dMMMMy${c1}syysoooooo${c2}dMMMMy${c1}yyyyyyyyo`
 oyyyyyy${c2}dMMMMy${c1}yyyyyyyyyyys${c2}dMMy${c1}sssssyyyo
-yyyyyyyy${c2}dMy${c1}syyyyyyyyyyyyyys${c2}dMMMMMy${c1}syyy-
oyyyysoo${c2}dMy${c1}yyyyyyyyyyyyyyyyyy${c2}dMMMMy${c1}syyyo
yyys${c2}dMMMMMy${c1}yyyyyyyyyyyyyyyyyysosyyyyyyyy
yyys${c2}dMMMMMy${c1}yyyyyyyyyyyyyyyyyyyyyyyyyyyyy
oyyyyysos${c2}dy${c1}yyyyyyyyyyyyyyyyyy${c2}dMMMMy${c1}syyyo
-yyyyyyyy${c2}dMy${c1}syyyyyyyyyyyyyys${c2}dMMMMMy${c1}syyy-
 oyyyyyy${c2}dMMMy${c1}syyyyyyyyyyys${c2}dMMy${c1}oyyyoyyyo
 `oyyyy${c2}dMMMy${c1}syyyoooooo${c2}dMMMMy${c1}oyyyyyyyyo
   oyyysyyoyyyys${c2}dMMMMMMMMMMMy${c1}yyyyyyyo
    -syyyyyyyyy${c2}dMMMy${c1}syyy${c2}dMMMy${c1}syyyys-
      -oyyyyyyy${c2}dMMy${c1}yyyyyysosyyyyo-
        ./oyyyyyyyyyyyyyyyyyyo/.
           `.:/oosyyyysso/:.`"#;

const LINUXMINT_SMALL: &str = r#"${c1} ___________
|_          \
  | ${c2}| _____ ${c1}|
  | ${c2}| | | | ${c1}|
  | ${c2}| | | | ${c1}|
  | ${c2}\__${c2}___/ ${c1}|
  \_________/"#;

const LINUXMINT_OLD: &str = r#"${c1}MMMMMMMMMMMMMMMMMMMMMMMMMmds+.
MMm----::-://////////////oymNMd+`
MMd      ${c2}/++                ${c1}-sNMd:
MMNso/`  ${c2}dMM    `.::-. .-::.` ${c1}.hMN:
ddddMMh  ${c2}dMM   :hNMNMNhNMNMNh: ${c1}`NMm
    NMm  ${c2}dMM  .NMN/-+MMM+-/NMN` ${c1}dMM
    NMm  ${c2}dMM  -MMm  `MMM   dMM. ${c1}dMM
    NMm  ${c2}dMM  -MMm  `MMM   dMM. ${c1}dMM
    NMm  ${c2}dMM  .mmd  `mmm   yMM. ${c1}dMM
    NMm  ${c2}dMM`  ..`   ...   ydm. ${c1}dMM
    hMM- ${c2}+MMd/-------...-:sdds  ${c1}dMM
    -NMm- ${c2}:hNMNNNmdddddddddy/`  ${c1}dMM
     -dMNs-${c2}``-::::-------.``    ${c1}dMM
      `/dMNmy+/:-------------:/yMMM
         ./ydNMMMMMMMMMMMMMMMMMMMMM
            .MMMMMMMMMMMMMMMMMMM"#;

const LINUXMINT: &str = r#"${c2}             ...-:::::-...
${c2}          .-MMMMMMMMMMMMMMM-.
      .-MMMM${c1}`..-:::::::-..`${c2}MMMM-.
    .:MMMM${c1}.:MMMMMMMMMMMMMMM:.${c2}MMMM:.
   -MMM${c1}-M---MMMMMMMMMMMMMMMMMMM.${c2}MMM-
 `:MMM${c1}:MM`  :MMMM:....::-...-MMMM:${c2}MMM:`
 :MMM${c1}:MMM`  :MM:`  ``    ``  `:MMM:${c2}MMM:
.MMM${c1}.MMMM`  :MM.  -MM.  .MM-  `MMMM.${c2}MMM.
:MMM${c1}:MMMM`  :MM.  -MM-  .MM:  `MMMM-${c2}MMM:
:MMM${c1}:MMMM`  :MM.  -MM-  .MM:  `MMMM:${c2}MMM:
:MMM${c1}:MMMM`  :MM.  -MM-  .MM:  `MMMM-${c2}MMM:
.MMM${c1}.MMMM`  :MM:--:MM:--:MM:  `MMMM.${c2}MMM.
 :MMM${c1}:MMM-  `-MMMMMMMMMMMM-`  -MMM-${c2}MMM:
  :MMM${c1}:MMM:`                `:MMM:${c2}MMM:
   .MMM${c1}.MMMM:--------------:MMMM.${c2}MMM.
     '-MMMM${c1}.-MMMMMMMMMMMMMMM-.${c2}MMMM-'
       '.-MMMM${c1}``--:::::--``${c2}MMMM-.'
${c2}            '-MMMMMMMMMMMMM-'
${c2}               ``-:::::-``"#;

const MX_SMALL: &str = r#"${c3}    \\  /
     \\/
      \\
   /\/ \\
  /  \  /\
 /    \/  \
/__________\"#;
const MX: &str = r#"${c3}MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMNMMMMMMMMM
MMMMMMMMMMNs..yMMMMMMMMMMMMMm: +NMMMMMMM
MMMMMMMMMN+    :mMMMMMMMMMNo` -dMMMMMMMM
MMMMMMMMMMMs.   `oNMMMMMMh- `sNMMMMMMMMM
MMMMMMMMMMMMN/    -hMMMN+  :dMMMMMMMMMMM
MMMMMMMMMMMMMMh-    +ms. .sMMMMMMMMMMMMM
MMMMMMMMMMMMMMMN+`   `  +NMMMMMMMMMMMMMM
MMMMMMMMMMMMMMNMMd:    .dMMMMMMMMMMMMMMM
MMMMMMMMMMMMm/-hMd-     `sNMMMMMMMMMMMMM
MMMMMMMMMMNo`   -` :h/    -dMMMMMMMMMMMM
MMMMMMMMMd:       /NMMh-   `+NMMMMMMMMMM
MMMMMMMNo`         :mMMN+`   `-hMMMMMMMM
MMMMMMh.            `oNMMd:    `/mMMMMMM
MMMMm/                -hMd-      `sNMMMM
MMNs`                   -          :dMMM
Mm:                                 `oMM
MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM"#;

const MANJARO_SMALL: &str = r#"${c1}██████████████
█████████ ████
████      ████
████ ████ ████
████ ████ ████
████ ████ ████
████ ████ ████"#;
const MANJARO: &str = r#"${c1}██████████████████  ████████
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
████████  ████████  ████████"#;

const NIXOS_SMALL: &str = r#"${c1}  \\  \\ //
 ==\\__\\/ //
   //   \\//
==//     //==
 //\\___//
// /\\  \\==
  // \\  \\"#;
const NIXOS: &str = r#"${c1}          ::::.    ${c2}':::::     ::::'
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
${c1}         .::::      ::::      ${c2}'::::."#;

const OPENSUSE_SMALL: &str = r#"${c1}  _______
__|   __ \\
     / .\\ \\
     \\__/ |
   _______|
   \\_______
__________/"#;
const OPENSUSE: &str = r#"${c2}           .;ldkO0000Okdl;.
       .;d00xl:^''''''^:ok00d;.
     .d00l'                'o00d.
   .d0Kd'${c1}  Okxol:;,.          ${c2}:O0d.
  .OK${c1}KKK0kOKKKKKKKKKKOxo:,      ${c2}lKO.
 ,0K${c1}KKKKKKKKKKKKKKK0P^${c2},,,${c1}^dx:${c2}    ;00,
.OK${c1}KKKKKKKKKKKKKKKk'${c2}.oOPPb.${c1}'0k.${c2}   cKO.
:KK${c1}KKKKKKKKKKKKKKK: ${c2}kKx..dd ${c1}lKd${c2}   'OK:
dKK${c1}KKKKKKKKKOx0KKKd ${c2}^0KKKO' ${c1}kKKc${c2}   dKd
dKK${c1}KKKKKKKKKK;.;oOKx,..${c2}^${c1}..;kKKK0.${c2}  dKd
:KK${c1}KKKKKKKKKK0o;...^cdxxOK0O/^^'  ${c2}.0K:
 kKK${c1}KKKKKKKKKKKKK0x;,,......,;od  ${c2}lKk
 '0K${c1}KKKKKKKKKKKKKKKKKKKK00KKOo^  ${c2}c00'
  'kK${c1}KKOxddxkOO00000Okxoc;''   ${c2}.dKk'
    l0Ko.                    .c00l'
     'l0Kk:.              .;xK0l'
        'lkK0xl:;,,,,;:ldO0kl'
            '^:ldxkkkkxdl:^'"#;
const OPENSUSE_LEAP: &str = r#"${c2}                 `-++:`
               ./oooooo/-
            `:oooooooooooo:.
          -+oooooooooooooooo+-`
       ./oooooooooooooooooooooo/-
      :oooooooooooooooooooooooooo:
    `  `-+oooooooooooooooooooo/-   `
 `:oo/-   .:ooooooooooooooo+:`  `-+oo/.
`/oooooo:.   -/oooooooooo/.   ./oooooo/.
  `:+ooooo+-`  `:+oooo+-   `:oooooo+:`
     .:oooooo/.   .::`   -+oooooo/.
        -/oooooo:.    ./oooooo+-
          `:+ooooo+-:+oooooo:`
             ./oooooooooo/.
                -/oooo+:`
                  `:/."#;
const OPENSUSE_TUMBLEWEED: &str = r#"${c2}                                     ......
     .,cdxxxoc,.               .:kKMMMNWMMMNk:.
    cKMMN0OOOKWMMXo. ;        ;0MWk:.      .:OMMk.
  ;WMK;.       .lKMMNM,     :NMK,             .OMW;
 cMW;            'WMMMN   ,XMK,                 oMM'
.MMc               ..;l. xMN:                    KM0
'MM.                   'NMO                      oMM
.MM,                 .kMMl                       xMN
 KM0               .kMM0. .dl:,..               .WMd
 .XM0.           ,OMMK,    OMMMK.              .XMK
   oWMO:.    .;xNMMk,       NNNMKl.          .xWMx
     :ONMMNXMMMKx;          .  ,xNMWKkxllox0NMWk,
         .....                    .:dOOXXKOxl,"#;

const PARCH: &str = r#"‌${c1}             ,:lodddd.
           .:clooood.
         ;clllooooc
       ;cclllllloo
      .cccccllllll
    .   ,cccclllll
   ':::;; ccccclll;
  .:::cccccccccccll;
  ;::::ccccllllllcll:
 .;::::cccclllloool::;
 ;;;::::cccclllolc::::;.
 ;;;::::cccclllccc:::::;.
 ;;;::::cccclccccc::::::;.
 ;;;;::::::llcccccc:::::'
 ;;;;:; ,clllccccccc::
 .;;  .cllllllcccccc::;::::'
     .'''''''''',:lddoooolll
    '.....'''',cdddooooollll
   ........':oddddoooolllllc
    ....';ldddddooooolllllc:
      ,cdddddddooooollllccc
       :ddddddoooolllllccc
         ;ddooooolllllcc.
            :ooollllc.
                c'
"#;

const PARROT: &str = r#"${c1}  `:oho/-`
`mMMMMMMMMMMMNmmdhy-
 dMMMMMMMMMMMMMMMMMMs`
 +MMsohNMMMMMMMMMMMMMm/
 .My   .+dMMMMMMMMMMMMMh.
  +       :NMMMMMMMMMMMMNo
           `yMMMMMMMMMMMMMm:
             /NMMMMMMMMMMMMMy`
              .hMMMMMMMMMMMMMN+
                  ``-NMMMMMMMMMd-
                     /MMMMMMMMMMMs`
                      mMMMMMMMsyNMN/
                      +MMMMMMMo  :sNh.
                      `NMMMMMMm     -o/
                       oMMMMMMM.
                       `NMMMMMM+
                        +MMd/NMh
                         mMm -mN`
                         /MM  `h:
                          dM`   .
                          :M-
                           d:
                           -+
                            -"#;

const POP_OS_SMALL: &str = r#"${c1}______
\   _ \        __
 \ \ \ \      / /
  \ \_\ \    / /
   \  ___\  /_/
    \ \    _
   __\_\__(_)_
  (___________)`"#;
const POP_OS: &str = r#"${c1}             /////////////
         /////////////////////
      ///////${c2}*767${c1}////////////////
    //////${c2}7676767676*${c1}//////////////
   /////${c2}76767${c1}//${c2}7676767${c1}//////////////
  /////${c2}767676${c1}///${c2}*76767${c1}///////////////
 ///////${c2}767676${c1}///${c2}76767${c1}.///${c2}7676*${c1}///////
/////////${c2}767676${c1}//${c2}76767${c1}///${c2}767676${c1}////////
//////////${c2}76767676767${c1}////${c2}76767${c1}/////////
///////////${c2}76767676${c1}//////${c2}7676${c1}//////////
////////////,${c2}7676${c1},///////${c2}767${c1}///////////
/////////////*${c2}7676${c1}///////${c2}76${c1}////////////
///////////////${c2}7676${c1}////////////////////
 ///////////////${c2}7676${c1}///${c2}767${c1}////////////
  //////////////////////${c2}'${c1}////////////
   //////${c2}.7676767676767676767,${c1}//////
    /////${c2}767676767676767676767${c1}/////
      ///////////////////////////
         /////////////////////
             /////////////"#;

const REDHAT: &str = r#"${c1}           .MMM..:MMMMMMM
          MMMMMMMMMMMMMMMMMM
          MMMMMMMMMMMMMMMMMMMM.
         MMMMMMMMMMMMMMMMMMMMMM
        ,MMMMMMMMMMMMMMMMMMMMMM:
        MMMMMMMMMMMMMMMMMMMMMMMM
  .MMMM'  MMMMMMMMMMMMMMMMMMMMMM
 MMMMMM    `MMMMMMMMMMMMMMMMMMMM.
MMMMMMMM      MMMMMMMMMMMMMMMMMM .
MMMMMMMMM.       `MMMMMMMMMMMMM' MM.
MMMMMMMMMMM.                     MMMM
`MMMMMMMMMMMMM.                 ,MMMMM.
 `MMMMMMMMMMMMMMMMM.          ,MMMMMMMM.
    MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
      MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM:
         MMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
            `MMMMMMMMMMMMMMMMMMMMMMMM:
                ``MMMMMMMMMMMMMMMMM'"#;

const ROCKY_SMALL: &str = r#"${c1}    `-/+++++++++/-.`
 `-+++++++++++++++++-`
.+++++++++++++++++++++.
-+++++++++++++++++++++++.
+++++++++++++++/-/+++++++
+++++++++++++/.   ./+++++
+++++++++++:.       ./+++
+++++++++:`   `:/:`   .:/
-++++++:`   .:+++++:`
 .+++-`   ./+++++++++:`
  `-`   ./+++++++++++-
       -+++++++++:-.`"#;

const ROCKY: &str = r#"${c1}          __wgliliiligw_,
       _williiiiiiliilililw,
     _%iiiiiilililiiiiiiiiiii_
   .Qliiiililiiiiiiililililiilm.
  _iiiiiliiiiiililiiiiiiiiiiliil,
 .lililiiilililiiiilililililiiiii,
_liiiiiiliiiiiiiliiiiiF{iiiiiilili,
jliililiiilililiiili@`  ~ililiiiiiL
iiiliiiiliiiiiiili>`      ~liililii
liliiiliiilililii`         -9liiiil
iiiiiliiliiiiii~             "4lili
4ililiiiiilil~|      -w,       )4lf
-liiiiililiF'       _liig,       )'
 )iiiliii@`       _QIililig,
  )iiii>`       .Qliliiiililw
   )<>~       .mliiiiiliiiiiil,
            _gllilililiililii~
           giliiiiiiiiiiiiT`
          -^~$ililili@~~'"#;

const TAILS: &str = r#"${c1}      ``
  ./yhNh
syy/Nshh         `:o/
N:dsNshh  █   `ohNMMd
N-/+Nshh      `yMMMMd
N-yhMshh       yMMMMd
N-s:hshh  █    yMMMMd so//.
N-oyNsyh       yMMMMd d  Mms.
N:hohhhd:.     yMMMMd  syMMM+
Nsyh+-..+y+-   yMMMMd   :mMM+
+hy-      -ss/`yMMMM     `+d+
  :sy/.     ./yNMMMMm      ``
    .+ys- `:+hNMMMMMMy/`
      `hNmmMMMMMMMMMMMMdo.
       dMMMMMMMMMMMMMMMMMNh:
       +hMMMMMMMMMMMMMMMMMmy.
         -oNMMMMMMMMMMmy+.`
           `:yNMMMds/.`
              .//`"#;

const UBUNTU_SMALL: &str = r#"${c1}         _
     ---(_)
 _/  ---  \
(_) |   |
  \  --- _/
     ---(_)"#;
const UBUNTU: &str = r#"${c1}            .-/+oossssoo+\-.
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
            .-\+oossssoo+/-."#;

const ZORIN: &str = r#"${c1}        `osssssssssssssssssssso`
       .osssssssssssssssssssssso.
      .+oooooooooooooooooooooooo+.


  `::::::::::::::::::::::.         .:`
 `+ssssssssssssssssss+:.`     `.:+ssso`
.ossssssssssssssso/.       `-+ossssssso.
ssssssssssssso/-`      `-/osssssssssssss
.ossssssso/-`      .-/ossssssssssssssso.
 `+sss+:.      `.:+ssssssssssssssssss+`
  `:.         .::::::::::::::::::::::`


      .+oooooooooooooooooooooooo+.
       -osssssssssssssssssssssso-
        `osssssssssssssssssssso`"#;

// =====================================================================
// =====================================================================
//                              WINDOWS
// =====================================================================
// =====================================================================

const WINDOWS: &str = r#"${c1}        ,.=:!!t3Z3z.,
       :tt:::tt333EE3
${c1}       Et:::ztt33EEEL${c2} @Ee.,      ..,
${c1}      ;tt:::tt333EE7${c2} ;EEEEEEttttt33#
${c1}     :Et:::zt333EEQ.${c2} $EEEEEttttt33QL
${c1}     it::::tt333EEF${c2} @EEEEEEttttt33F
${c1}    ;3=*^```"*4EEV${c2} :EEEEEEttttt33@.
${c3}    ,.=::::!t=., ${c1}`${c2} @EEEEEEtttz33QF
${c3}   ;::::::::zt33)${c2}   "4EEEtttji3P*
${c3}  :t::::::::tt33.${c4}:Z3z..${c2}  ``${c4} ,..g.
${c3}  i::::::::zt33F${c4} AEEEtttt::::ztF
${c3} ;:::::::::t33V${c4} ;EEEttttt::::t3
${c3} E::::::::zt33L${c4} @EEEtttt::::z3F
${c3}{3=*^```"*4E3)${c4} ;EEEtttt:::::tZ`
${c3}             `${c4} :EEEEtttt::::z7
                 "VEzjt:;;z>*`"#;

const WINDOWS_10: &str = r#"${c1}                                ..,
                    ....,,:;+ccMMMM
      ....++mm  cMMMMMMMMMMMMMMMMMM
,ccMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM

MMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM
`'ccMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM
      `'\nnMMM  :ccMMMMMMMMMMMMMMMM
                       ````'\***cMM
                                 ``"#;

const WINDOWS_11: &str = r#"${c1}████████████████  ████████████████
████████████████  ████████████████
████████████████  ████████████████
████████████████  ████████████████
████████████████  ████████████████
████████████████  ████████████████
████████████████  ████████████████

████████████████  ████████████████
████████████████  ████████████████
████████████████  ████████████████
████████████████  ████████████████
████████████████  ████████████████
████████████████  ████████████████
████████████████  ████████████████"#;
