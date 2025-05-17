pub fn get_builtin_distro_colors(distro: &str) -> &'static [u8] {
    let mut small_size = false;

    let dist = if let Some(stripped) = distro.strip_suffix("_small") {
        small_size = true;
        stripped.to_lowercase()
    } else {
        distro.to_lowercase()
    };

    if dist.contains("windows") {
        if dist.contains("11") {
            return &[4, 7];
        } else if dist.contains("10") || dist.contains("8") {
            return &[4, 7];
        } else {
            return &[1, 2, 4, 3];
        }
    }

    match (dist.as_str(), small_size) {
        ("almalinux", false) | ("alma", false) => &[1, 3, 4, 2, 6],

        ("alpine", true) => &[4, 7],
        ("alpine", false) => &[4, 5, 7, 6],

        ("alter", _) => &[6, 6],

        ("amazon", _) => &[3, 7],

        ("anarchy", _) => &[7],

        ("instantos", _) => &[4, 6],

        ("android", _) => &[2, 7],

        ("antix", false) => &[1, 7, 3],

        ("antergos", _) => &[4, 6],

        ("aosc os/retro", _) | ("aoscos/retro", _) | ("aosc os retro", _) => &[4, 7, 1, 3],

        ("aosc os", _) | ("aoscos", _) => &[4, 7, 1],

        ("arch", true) | ("archlinux", true) | ("arch linux", true) => &[6, 7, 1],
        ("arch", false) | ("archlinux", false) | ("arch linux", false) => &[6, 6, 7, 1],

        ("archbox", _) => &[2, 7, 1],

        ("archlabs", _) => &[6, 6, 7, 1],

        ("archcraft", _) => &[6, 6, 7, 1],

        ("arcolinux", _) => &[7, 4],

        ("apricity", _) => &[4, 7, 1],

        ("archstrike", _) => &[8, 6],

        ("AIX", _) => &[2, 7],

        ("bodhi", _) => &[7, 9, 2],

        ("calculate", false) | ("calculateos", false) => &[7, 3],

        ("clearos", false) => &[2],

        ("centos", _) => &[3, 2, 4, 5, 7],

        ("debian", _) => &[1, 3, 7],

        ("elementary", _) => &[4, 7, 1],

        ("endeavouros", _) => &[1, 5, 4],

        ("fedora", _) => &[4, 7, 1],

        ("hash", _) => &[1, 2, 3],

        ("garuda", _) => &[7, 7, 3, 7, 2, 4],

        ("kali", _) => &[4, 8],

        ("kde", _) => &[2, 7],

        ("kubuntu", _) => &[4, 7, 1],

        ("linuxmint", _)
        | ("linux mint", _)
        | ("linuxmintold", _)
        | ("linuxmint_old", _)
        | ("mintold", _)
        | ("mint_old", _)
        | ("mint", _) => &[2, 7],

        ("manjaro", _) => &[2, 7],

        ("mx", _) => &[4, 6, 7],

        ("nixos", _) => &[4, 6],

        ("suse", _)
        | ("opensuse", _)
        | ("open suse", _)
        | ("opensuse leap", _)
        | ("opensuse_leap", _)
        | ("opensuse tumbleweed", _)
        | ("opensuse_tumbleweed", _) => &[2, 7],

        ("parch", _) | ("parch linux", _) => &[4, 7, 1],

        ("parrot", _) | ("parrot linux", _) => &[6, 7],

        ("popos", _) | ("pop_os", _) | ("pop!_os", _) => &[6, 7],

        ("redhat", _) | ("red hat", _) | ("rhel", _) => &[1],

        ("rocky", _) => &[2],

        ("tails", _) => &[5, 7],

        ("ubuntu", _) => &[1, 7, 3],

        ("zorin", _) => &[4, 6],

        _ => &[7, 0, 3],
    }
}
