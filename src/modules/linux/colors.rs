pub fn get_builtin_distro_colors(distro: &str) -> &'static [u8] {
    let mut small_size = false;

    let dist = if let Some(stripped) = distro.strip_suffix("_small") {
        small_size = true;
        stripped.to_lowercase()
    } else {
        distro.to_lowercase()
    };

    match (dist.as_str(), small_size) {
        ("almalinux", false) | ("alma", false) => ALMALINUX,

        ("alpine", true) => ALPINE_SMALL,
        ("alpine", false) => ALPINE,

        ("amazon", _) => AMAZON,

        ("antix", false) => ANTI_X,

        ("arch", true) | ("archlinux", true) => ARCH_SMALL,
        ("arch", false) | ("archlinux", false) => ARCH,

        ("arcolinux", _) => ARCOLINUX,

        ("bodhi", false) => BODHI,

        ("calculate", false) | ("calculateos", false) => CALCULATE,

        ("clearos", false) => CLEAROS,

        ("centos", _) => CENTOS,

        ("debian", _) => DEBIAN,

        ("elementary", _) => ELEMENTARY,

        ("endeavouros", _) => ENDEAVOUROS,

        ("fedora", _) => FEDORA,

        ("garuda", _) => GARUDA,

        ("kali", _) => KALI,

        ("kde", _) => KDE,

        ("kubuntu", _) => KUBUNTU,

        ("linuxmint", _)
        | ("linux mint", _)
        | ("linuxmintold", _)
        | ("linuxmint_old", _)
        | ("mintold", _)
        | ("mint_old", _)
        | ("mint", _) => LINUXMINT,

        ("manjaro", _) => MANJARO,

        ("mx", _) => MX,

        ("nixos", _) => NIXOS,

        ("suse", _)
        | ("opensuse", _)
        | ("open suse", _)
        | ("opensuse leap", _)
        | ("opensuse_leap", _)
        | ("opensuse tumbleweed", _)
        | ("opensuse_tumbleweed", _) => OPENSUSE,

        ("parrot", _) => PARROT,

        ("popos", _) | ("pop_os", _) | ("pop!_os", _) => POP_OS,

        ("redhat", _) | ("red hat", _) | ("rhel", _) => REDHAT,

        ("rocky", _) => ROCKY,

        ("tails", _) => TAILS,

        ("ubuntu", _) => UBUNTU,

        ("zorin", _) => ZORIN,

        _ => DEFAULT,
    }
}

const DEFAULT: &[u8] = &[7, 0, 3];

const ALMALINUX: &[u8] = &[1, 3, 4, 2, 6];

const ALPINE: &[u8] = &[4, 5, 7, 6];
const ALPINE_SMALL: &[u8] = &[4, 7];

const AMAZON: &[u8] = &[3, 7];

const ANTI_X: &[u8] = &[1, 7, 3];

const ARCH: &[u8] = &[6, 6, 7, 1];
const ARCH_SMALL: &[u8] = &[6, 7, 1];

const ARCOLINUX: &[u8] = &[7, 4];

const BODHI: &[u8] = &[7, 9, 2];

const CALCULATE: &[u8] = &[7, 3];

const CENTOS: &[u8] = &[3, 2, 4, 5, 7];

const CLEAROS: &[u8] = &[2];

const DEBIAN: &[u8] = &[1, 3, 7];

const ELEMENTARY: &[u8] = &[4, 7, 1];

const ENDEAVOUROS: &[u8] = &[1, 5, 4];

const FEDORA: &[u8] = &[4, 7, 1];

const GARUDA: &[u8] = &[7, 7, 3, 7, 2, 4];

const KALI: &[u8] = &[4, 8];

const KDE: &[u8] = &[2, 7];

const KUBUNTU: &[u8] = &[4, 7, 1];

const LINUXMINT: &[u8] = &[2, 7];

const MANJARO: &[u8] = &[2, 7];

const MX: &[u8] = &[4, 6, 7];

const NIXOS: &[u8] = &[4, 6];

const OPENSUSE: &[u8] = &[2, 7];

const PARROT: &[u8] = &[6, 7];

const POP_OS: &[u8] = &[6, 7];

const REDHAT: &[u8] = &[1];

const ROCKY: &[u8] = &[2];

const TAILS: &[u8] = &[5, 7];

const UBUNTU: &[u8] = &[1, 7, 3];

const ZORIN: &[u8] = &[4, 6];
