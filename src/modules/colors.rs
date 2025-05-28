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

        ("archmerge", _) => &[6, 6, 7, 1],

        ("artix", _) => &[6, 6, 7, 1],

        ("arya", _) => &[2, 1],

        ("asteroidos", _) | ("asteroid", _) => &[1, 2, 2, 2],

        ("AIX", _) => &[2, 7],

        ("bedrock", _) => &[8, 7],

        ("bitrig", _) => &[2, 7],

        ("blackarch", _) | ("black arch", _) => &[1, 1, 0, 1],

        ("blag", _) => &[5, 7],

        ("blankon", _) | ("blank on", _) => &[1, 7, 3],

        ("bluelight", _) => &[7, 4],

        ("bodhi", _) => &[7, 9, 2],

        ("bonsai", _) => &[6, 2, 3],

        ("bsd", _) => &[1, 7, 4, 3, 6],

        ("bunsenlabs", _) | ("bunsen labs", _) => &[7],

        ("calculate", false) | ("calculateos", false) => &[7, 3],

        ("carbs", _) => &[4, 5, 4, 4, 4, 4],

        ("cbl-mariner", _) | ("cbl mariner", _) => &[6],

        ("celos", _) | ("cel os", _) => &[4, 6, 0, 5],

        ("clearos", false) => &[2],

        ("centos", _) => &[3, 2, 4, 5, 7],

        ("chakra", _) => &[4, 5, 7, 6],

        ("chaletos", _) | ("chalet os", _) => &[4, 7, 1],

        ("chapeau", _) => &[2, 7],

        ("chrom", _) | ("chrome", _) | ("chromeos", _) | ("chrome os", _) => &[2, 1, 3, 4, 7],

        ("cleanjaro", _) => &[7, 7],

        ("clear linux os", _)
        | ("clearlinuxos", _)
        | ("clear_linux", _)
        | ("clear_linux_os", _) => &[4, 3, 7, 6],

        ("clover", _) => &[2, 6],

        ("condres", _) => &[2, 3, 6],

        ("container Linux by coreos", _) | ("container_linux", _) | ("container", _) => &[4, 7, 1],

        ("crux", _) => &[4, 5, 7, 6],

        ("crystal linux", _) | ("crystal", _) => &[13, 5],

        ("cucumber", _) => &[2, 3],

        ("cyberos", _) | ("cyber os", _) => &[4, 5, 6],

        // ("name", _) => &[],
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
