pub fn get_builtin_ascii_art(ascii_distro: &str) -> &'static str {
    let mut small_size = false;

    let dist = if let Some(stripped) = ascii_distro.strip_suffix("_small") {
        small_size = true;
        stripped.to_lowercase()
    } else {
        ascii_distro.to_lowercase()
    };

    println!("distro: {}", dist);

    match (dist.as_str(), small_size) {
        ("almalinux", false) | ("alma", false) => ALMALINUX,

        ("alpine", true) => ALPINE_SMALL,
        ("alpine", false) => ALPINE,

        ("amazon", true) => AMAZON_SMALL,
        ("amazon", false) => AMAZON,

        ("antix", _) => ANTI_X,

        ("arch", true) | ("archlinux", true) => ARCH_SMALL,
        ("arch", false) | ("archlinux", false) => ARCH,

        ("arcolinux", true) => ARCOLINUX_SMALL,
        ("arcolinux", false) => ARCOLINUX,

        ("bodhi", false) => BODHI,

        ("calculate", _) | ("calculateos", _) => CALCULATE,

        ("clearos", _) => CLEAROS,

        ("centos", true) => CENTOS_SMALL,
        ("centos", false) => CENTOS,

        ("debian", true) => DEBIAN_SMALL,
        ("debian", false) => DEBIAN,

        ("elementary", true) => ELEMENTARY_SMALL,
        ("elementary", false) => ELEMENTARY,

        ("endeavouros", true) => ENDEAVOUROS_SMALL,
        ("endeavouros", false) => ENDEAVOUROS,

        ("fedora", true) => FEDORA_SMALL,
        ("fedora", false) => FEDORA,

        ("garuda", _) => GARUDA,

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

        ("parrot", _) => PARROT,

        ("popos", true) | ("pop_os", true) => POP_OS_SMALL,
        ("popos", false) | ("pop_os", false) | ("pop!_os", false) => POP_OS,

        ("redhat", _) | ("red hat", _) | ("rhel", _) => REDHAT,

        ("rocky", true) => ROCKY_SMALL,
        ("rocky", false) => ROCKY,

        ("tails", _) => TAILS,

        ("ubuntu", true) => UBUNTU_SMALL,
        ("ubuntu", false) => UBUNTU,

        ("zorin", _) => ZORIN,

        _ => DEFAULT,
    }
}

const DEFAULT: &str = r#"
${c2}        #####
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

const ALMALINUX: &str = r#"
${c1}         'c:.
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

const ALPINE_SMALL: &str = r#"
${c1}   /\ /\
  /${c2}/ ${c1}\  \
 /${c2}/   ${c1}\  \
/${c2}//    ${c1}\  \
${c2}//      ${c1}\  \
         \"#;
const ALPINE: &str = r#"
${c1}       .hddddddddddddddddddddddh.
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

const AMAZON_SMALL: &str = r#"
${c1}
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
const AMAZON: &str = r#"
${c1}             `-/oydNNdyo:.`
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

const ANTI_X: &str = r#"
${c1}
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

const ARCH_SMALL: &str = r#"
${c1}      /\
     /  \
    /\   \
${c2}   /      \
  /   ,,   \
 /   |  |  -\
/_-''    ''-_\"#;
const ARCH: &str = r#"
${c1}                   -`
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

const ARCOLINUX_SMALL: &str = r#"
${c2}          A
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
const ARCOLINUX: &str = r#"
${c2}                    /-
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

const BODHI: &str = r#"
${c1}|           ${c2},,mmKKKKKKKKWm,,
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

const CALCULATE: &str = r#"
${c1}                              ......
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

const CENTOS_SMALL: &str = r#"
${c2} ____${c1}^${c4}____
${c2} |\  ${c1}|${c4}  /|
${c2} | \ ${c1}|${c4} / |
${c4}<---- ${c3}---->
${c3} | / ${c2}|${c1} \ |
${c3} |/__${c2}|${c1}__\|
${c2}     v"#;
const CENTOS: &str = r#"
${c1}                 ..
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

const CLEAROS: &str = r#"
${c1}             `.--::::::--.`
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

const DEBIAN_SMALL: &str = r#"
${c1}  _____
 /  __ \
|  /    |
|  \___-
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
              `""""#;

const ELEMENTARY_SMALL: &str = r#"
${c2}  _______
 / ____  \\
/  |  /  /\\
|__\\ /  / |
\\   /__/  /
 \\_______/"#;
const ELEMENTARY: &str = r#"
${c2}         eeeeeeeeeeeeeeeee
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

const ENDEAVOUROS_SMALL: &str = r#"
${c1}                ./${c2}o${c3}.
${c1}              `:${c2}oss+${c3}-
${c1}            `:+${c2}ssssso${c3}/.
${c1}          `-/o${c2}sssssssso${c3}/.
${c1}        `-:/+${c2}ssssssssssso${c3}/.
${c1}      .://+${c2}ssssssssssssssso${c3}+:
${c1}    .:///o${c2}ssssssssssssssssso${c3}+:
${c1}  `:////${c2}sssssssssssssssssssso${c3}++.
${c1}  `..-+${c2}oossssssssssssssssso${c3}++++/`
    `::::::::::::::::::::----``"#;
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
  `:::::::::::::::::::::::::------``"#;

const FEDORA_SMALL: &str = r#"
${c1}        ,'''''.
       |   ,.  |
       |  |  '_'
  ,....|  |..
.'  ,_;|   ..'
|  |   |  |
|  ',_,'  |
 '.     ,'
   '''''"#;
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
  '::cccccccccccccc::;,."#;

const GARUDA: &str = r#"
#{c3}                  __,,,,,,,_  
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

const KALI: &str = r#"
${c1}..............
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

const KDE: &str = r#"
${c1}             `..---+/---..`
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

const KUBUNTU: &str = r#"
${c1}           `.:/ossyyyysso/:.
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

const LINUXMINT_SMALL: &str = r#"
${c1} ___________
|_          \
  | ${c2}| _____ ${c1}|
  | ${c2}| | | | ${c1}|
  | ${c2}| | | | ${c1}|
  | ${c2}\__${c2}___/ ${c1}|
  \_________/"#;

const LINUXMINT_OLD: &str = r#"
${c1}MMMMMMMMMMMMMMMMMMMMMMMMMmds+.
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

const LINUXMINT: &str = r#"
${c2}             ...-:::::-...
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

const MX_SMALL: &str = r#"
${c3}    \\  /
     \\/
      \\
   /\/ \\
  /  \  /\
 /    \/  \
/__________\"#;
const MX: &str = r#"
${c3}MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMNMMMMMMMMM
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

const MANJARO_SMALL: &str = r#"
${c1}██████████████
█████████ ████
████      ████
████ ████ ████
████ ████ ████
████ ████ ████
████ ████ ████"#;
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
████████  ████████  ████████"#;

const NIXOS_SMALL: &str = r#"
${c1}  \\  \\ //
 ==\\__\\/ //
   //   \\//
==//     //==
 //\\___//
// /\\  \\==
  // \\  \\"#;
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
${c1}         .::::      ::::      ${c2}'::::."#;

const OPENSUSE_SMALL: &str = r#"
${c1}  _______
__|   __ \\
     / .\\ \\
     \\__/ |
   _______|
   \\_______
__________/"#;
const OPENSUSE: &str = r#"
${c2}           .;ldkO0000Okdl;.
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
const OPENSUSE_LEAP: &str = r#"
${c2}                 `-++:`
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
const OPENSUSE_TUMBLEWEED: &str = r#"
${c2}                                     ......
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

const PARROT: &str = r#"
${c1}  `:oho/-`
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

const POP_OS_SMALL: &str = r#"
${c1}______
\   _ \        __
 \ \ \ \      / /
  \ \_\ \    / /
   \  ___\  /_/
    \ \    _
   __\_\__(_)_
  (___________)`"#;
const POP_OS: &str = r#"
${c1}             /////////////
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

const REDHAT: &str = r#"
${c1}           .MMM..:MMMMMMM
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

const ROCKY_SMALL: &str = r#"
${c1}    `-/+++++++++/-.`
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

const ROCKY: &str = r#"
${c1}          __wgliliiligw_,
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

const TAILS: &str = r#"
${c1}      ``
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

const UBUNTU_SMALL: &str = r#"
${c1}         _
     ---(_)
 _/  ---  \
(_) |   |
  \  --- _/
     ---(_)"#;
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
            .-\+oossssoo+/-."#;

const ZORIN: &str = r#"
${c1}        `osssssssssssssssssssso`
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
