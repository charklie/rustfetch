use crate::fns::{get_os_release_pretty_name, uname_s};
use colored::Color;
use colored::Colorize;

const ARCH_LOGO: (&str, Color) = ("                  sMN-
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
/+.                                  -o.", Color::BrightBlue);
const DEBI_LOGO: (&str, Color) = (r#""#, Color::Red);
const FEDO_LOGO: (&str, Color) = (r#"end
eav
or
os
mabi
dshadsa
dsa
ds
Addsad
sad
sa
ds
Add"#, Color::Blue);
const ENDE_LOGO: (&str, Color) = (r#""#, Color::Magenta);
const VOID_LOGO: (&str, Color) = (r#""#, Color::BrightGreen);
const UBUN_LOGO: (&str, Color) = (r#""#, Color::BrightRed);
const SUSE_LOGO: (&str, Color) = (r#""#, Color::Green);
const RASP_LOGO: (&str, Color) = (r#""#, Color::Red);
const MINT_LOGO: (&str, Color) = (r#""#, Color::Green);
const MXLI_LOGO: (&str, Color) = (r#""#, Color::BrightGreen);
const GENT_LOGO: (&str, Color) = (r#""#, Color::Magenta);
const FUNT_LOGO: (&str, Color) = (r#""#, Color::Magenta);
const SLAC_LOGO: (&str, Color) = (r#""#, Color::BrightBlue);
const UWUN_LOGO: (&str, Color) = (r#""#, Color::BrightMagenta);
const NIXO_LOGO: (&str, Color) = (r#""#, Color::BrightBlue);
const VANI_LOGO: (&str, Color) = (r#""#, Color::BrightYellow);
const KALI_LOGO: (&str, Color) = (r#""#, Color::Blue);
const CACH_LOGO: (&str, Color) = (r#""#, Color::BrightGreen);
const NETB_LOGO: (&str, Color) = (r#""#, Color::BrightRed);
const FREE_LOGO: (&str, Color) = (r#""#, Color::Red);
const RUST_LOGO: (&str, Color) = (r#"R
U
S
T
L
O
GO
OG
get_os_release_pretty_nameDAS"#, Color::Red);

macro_rules! get_os {
    ($overriden_ascii:expr, $legacy:expr, $search_string:expr, $legacy_return_string:expr, $modern_return_string:expr) => {{
        let os_name = $overriden_ascii
            .clone()
            .map(|o| o.to_ascii_lowercase())
            .unwrap_or_else(|| {
                get_os_release_pretty_name(None, "ID")
                    .unwrap_or_default()
                    .to_ascii_lowercase()
            });
        let uname = uname_s($overriden_ascii.clone()).to_ascii_lowercase();
        let results = [os_name, uname].join("");

        if results.contains(&$search_string.to_ascii_lowercase()) {
            // if $legacy {
            //     $legacy_return_string.to_string()
            // } else {
            (match $search_string.to_ascii_lowercase().as_str() {
                "arch" => ARCH_LOGO,
                _ => RUST_LOGO,
            }.0.color($modern_return_string.1).to_string().as_str(), $modern_return_string.1)
            // }
        } else {
            RUST_LOGO
        }
    }};
}

pub fn ascii_test(legacy: bool) {
    let distros = [
        "Arch Linux",
        "Debian",
        "Fedora",
        "EndeavourOS",
        "Void",
        "Ubuntu",
        "Suse",
        "Raspbian",
        "Linux Mint",
        "MX Linux",
        "Gentoo",
        "Funtoo",
        "Slackware",
        "UwUntu",
        "NixOS",
        "VanillaOS",
        "Kali Linux",
        "CachyOS",
        "NetBSD",
        "FreeBSD",
        "Unknown distro",
    ];

    for i in distros {
        println!("\n{i}: \n{}", get_distro_ascii(legacy, Some(i.to_string())));
    }
}

pub fn get_distro_ascii(legacy: bool, overriden_ascii: Option<String>) -> String {
    let ascii_macros = vec![
        get_os!(overriden_ascii, legacy, "arch", "   ___               __ \n  / _ |  ____ ____  / / \n / __ | / __// __/ / _ \\\n/_/ |_|/_/   \\__/ /_//_/", ARCH_LOGO),
        get_os!(overriden_ascii, legacy, "debian", "   ___      __   _         \n  / _ \\___ / /  (_)__ ____ \n / // / -_) _ \\/ / _ `/ _ \\\n/____/\\__/_.__/_/\\_,_/_//_/", DEBI_LOGO),
        get_os!(overriden_ascii, legacy, "fedora", "   ____       __             \n  / __/__ ___/ /__  _______ _\n / _// -_) _  / _ \\/ __/ _ `/\n/_/  \\__/\\_,_/\\___/_/  \\_,_/", FEDO_LOGO),
        get_os!(overriden_ascii, legacy, "endeavour", "   ____        __                           \n  / __/__  ___/ /__ ___ __  _____  __ ______\n / _// _ \\/ _  / -_) _ `/ |/ / _ \\/ // / __/\n/___/_//_/\\_,_/\\__/\\_,_/|___/\\___/\\_,_/_/", ENDE_LOGO),
        get_os!(overriden_ascii, legacy, "void", "  _   __     _    __\n | | / /__  (_)__/ /\n | |/ / _ \\/ / _  / \n |___/\\___/_/\\_,_/", VOID_LOGO),
        get_os!(overriden_ascii, legacy, "ubuntu", "  __  ____             __      \n / / / / /  __ _____  / /___ __\n/ /_/ / _ \\/ // / _ \\/ __/ // /\n\\____/_.__/\\_,_/_//_/\\__/\\_,_/", UBUN_LOGO),
        get_os!(overriden_ascii, legacy, "suse", "  ____                ____            \n / __ \\___  ___ ___  / __/_ _____ ___ \n/ /_/ / _ \\/ -_) _ \\_\\ \\/ // (_-</ -_)\n\\____/ .__/\\__/_//_/___/\\_,_/___/\\__/ \n    /_/", SUSE_LOGO),
        get_os!(overriden_ascii, legacy, "rasp", "   ___                __   _         \n  / _ \\___ ____ ___  / /  (_)__ ____ \n / , _/ _ `(_-</ _ \\/ _ \\/ / _ `/ _ \\\n/_/|_|\\_,_/___/ .__/_.__/_/\\_,_/_//_/\n             /_/", RASP_LOGO),
        get_os!(overriden_ascii, legacy, "mint", "   __  ____      __ \n  /  |/  (_)__  / /_\n / /|_/ / / _ \\/ __/\n/_/  /_/_/_//_/\\__/", MINT_LOGO),
        get_os!(overriden_ascii, legacy, "mx", "   __  ____  __  __   _               \n  /  |/  / |/_/ / /  (_)__  __ ____ __\n / /|_/ />  <  / /__/ / _ \\/ // /\\ \\ /\n/_/  /_/_/|_| /____/_/_//_/\\_,_//_\\_\\", MXLI_LOGO),
        get_os!(overriden_ascii, legacy, "gentoo", "  _____         __          \n / ___/__ ___  / /____  ___ \n/ (_ / -_) _ \\/ __/ _ \\/ _ \\\n\\___/\\__/_//_/\\__/\\___/\\___/", GENT_LOGO),
        get_os!(overriden_ascii, legacy, "funtoo", "   ____          __          \n  / __/_ _____  / /____  ___ \n / _// // / _ \\/ __/ _ \\/ _ \n/_/  \\_,_/_//_/\\__/\\___/\\___/", FUNT_LOGO),
        get_os!(overriden_ascii, legacy, "slack", "   ______         __                      \n  / __/ /__ _____/ /___    _____ ________ \n _\\ \\/ / _ `/ __/  '_/ |/|/ / _ `/ __/ -_)\n/___/_/\\_,_/\\__/_/\\_\\|__,__/\\_,_/_/  \\__/ ", SLAC_LOGO),
        get_os!(overriden_ascii, legacy, "uwuntu", "  __  __       __  __     __      \n / / / /    __/ / / /__  / /___ __\n/ /_/ / |/|/ / /_/ / _ \\/ __/ // /\n\\____/|__,__/\\____/_//_/\\__/\\_,_/", UWUN_LOGO),
        get_os!(overriden_ascii, legacy, "nix", "   _  ___      ____  ____\n  / |/ (_)_ __/ __ \\/ __/\n /    / /\\ \\ / /_/ /\\ \\  \n/_/|_/_//_\\_\\\\____/___/", NIXO_LOGO),
        get_os!(overriden_ascii, legacy, "vanilla", "  _   __          _ ____    \n | | / /__ ____  (_) / /__ _\n | |/ / _ `/ _ \\/ / / / _ `/\n |___/\\_,_/_//_/_/_/_/\\_,_/", VANI_LOGO),
        get_os!(overriden_ascii, legacy, "kali", "   __ __     ___   \n  / //_/__ _/ (_)  \n / ,< / _ `/ / /   \n/_/|_|\\_,_/_/_/", KALI_LOGO),
        get_os!(overriden_ascii, legacy, "cachy", "  _____         __       \n / ___/__ _____/ /  __ __\n/ /__/ _ `/ __/ _ \\/ // /\n\\___/\\_,_/\\__/_//_/\\_, / \n                  /___/", CACH_LOGO),
        get_os!(overriden_ascii, legacy, "netbsd", "   _  __    __  ___  _______ \n  / |/ /__ / /_/ _ )/ __/ _ \\\n /    / -_) __/ _  |\\ \\/ // /\n/_/|_/\\__/\\__/____/___/____/ ", NETB_LOGO),
        get_os!(overriden_ascii, legacy, "freebsd", "   ___            ___  _______ \n  / _/______ ___ / _ )/ __/ _ \\\n / _/ __/ -_) -_) _  |\\ \\/ // /\n/_//_/  \\__/\\__/____/___/____/", FREE_LOGO)
    ];

    for i in ascii_macros {
        if !i.0.is_empty() {
            return i.0.to_string();
        }
    }

    "   ___      _____      __ \n  / _ \\___ / _/ /_____/ / \n / , _(_-</ _/ __/ __/ _ \\\n/_/|_/___/_/ \\__/\\__/_//_/".to_string()
}
