use colored::Color;
use colored::Colorize;
use rayon::iter::ParallelIterator;
use rayon::str::ParallelString;
use std::env;
use std::io::Write;
use std::mem;
use tabwriter::TabWriter;

mod ascii;
mod color_config;
mod fns;
mod info_config;

use crate::ascii::*;
use crate::color_config::*;
use crate::fns::*;
use crate::info_config::*;

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let mut overriden_ascii: Option<String> = None;
    let mut info_custom_config: Option<String> = None;
    let mut color_custom_config: Option<String> = None;
    let mut get_only_info: Option<String> = None;
    let mut legacy: bool = false;
    let mut use_info_custom_config = true;
    let mut use_color_custom_config = true;
    let mut margin: i8 = 1;

    for count in 0..args.len() {
        match args[count].to_lowercase().as_str() {
            "-h" | "--help" | "--usage" => return help(),
            "-v" | "--version" => {
                return println!("Rsftch {}\nMade by charklie", VERSION.unwrap_or_default())
            }
            "--ignore-color-config" => use_color_custom_config = false,
            "--ignore-info-config" => use_info_custom_config = false,
            "--ignore-config" => {
                use_color_custom_config = false;
                use_info_custom_config = false;
            }
            "-l" | "--legacy" => legacy = true,
            "-m" | "--margin" => {
                if count + 1 < args.len() {
                    margin = args[count + 1].parse().unwrap();
                } else {
                    println!("[{}] Missing argument for margin.\n", "ERROR".red());
                    return help();
                }
            }
            "--info" => {
                if count + 1 < args.len() && !args[count + 1].starts_with("-") {
                    get_only_info = Some(mem::take(&mut args[count + 1]));
                } else {
                    println!("[{}] Missing argument for info.\n", "ERROR".red());
                    return help();
                }
            }
            "-o" | "--override" => {
                if count + 1 < args.len() && !args[count + 1].starts_with("-") {
                    overriden_ascii = Some(mem::take(&mut args[count + 1]));
                } else {
                    println!(
                        "[{}] Missing argument for override, showing all possible.",
                        "WARNING".yellow()
                    );
                    return println!("{:?}\n{:?}", ascii_test(true), ascii_test(false));
                }
            }
            "-i" | "--info-config" => {
                if count + 1 < args.len() && !args[count + 1].starts_with("-") {
                    info_custom_config = Some(mem::take(&mut args[count + 1]));
                } else {
                    println!(
                        "[{}] Missing argument for custom info config file.\n",
                        "ERROR".red()
                    );
                    return help();
                }
            }
            "-c" | "--color-config" => {
                if count + 1 < args.len() && !args[count + 1].starts_with("-") {
                    color_custom_config = Some(mem::take(&mut args[count + 1]));
                } else {
                    println!(
                        "[{}] Missing argument for custom color config file.\n",
                        "ERROR".red()
                    );
                    return help();
                }
            }
            _ => {}
        };
    }

    println!(
        "{}",
        info(
            overriden_ascii,
            margin,
            use_info_custom_config,
            !use_color_custom_config,
            info_custom_config,
            color_custom_config,
            get_only_info,
            legacy
        )
    );
}

#[derive(Clone, Debug)]
struct InfoItem {
    title: &'static str,
    alignment_space: i8,
    icon: &'static str,
    value: String,
}

fn print_ascii(
    ascii_art: String,
    color: Color,
    overriden_colors: bool,
    custom_color_config_file: Option<String>,
) -> String {
    if !overriden_colors {
        ascii_art.color(color).bold().to_string()
    } else {
        print_ascii(
            ascii_art,
            get_color_config(
                "color0".to_string(),
                !overriden_colors,
                custom_color_config_file.clone(),
            ),
            false,
            custom_color_config_file,
        )
    }
}

fn print_data(infos: &InfoItem, color: Color, connector: &'static str) -> String {
    let arrow = "~>";
    let coloreds = (
        connector.color(color).to_string(),
        infos.icon.color(color).to_string(),
        arrow.color(color).to_string(),
    );
    let alignment_space = " ".repeat(infos.alignment_space as usize);

    format!(
        "{}{}  {}{} {}  {}",
        coloreds.0, coloreds.1, infos.title, alignment_space, coloreds.2, infos.value
    )
    .to_string()
}

fn info(
    overriden_ascii: Option<String>,
    margin: i8,
    use_custom_info_config: bool,
    use_custom_color_config: bool,
    custom_info_config_file: Option<String>,
    custom_color_config_file: Option<String>,
    get_only_info: Option<String>,
    legacy: bool,
) -> String {
    let distro = InfoItem {
        title: "distro",
        alignment_space: 2,
        icon: "",
        value: get_os_release_pretty_name(overriden_ascii.clone(), "NAME")
            .unwrap_or(uname_s(overriden_ascii.clone())),
    };

    let hostname = InfoItem {
        title: "host",
        alignment_space: 4,
        icon: "󱩛",
        value: uname_n(),
    };

    let shell = InfoItem {
        title: "shell",
        alignment_space: 3,
        icon: "",
        value: shell_name(),
    };

    let kernel = InfoItem {
        title: "kernel",
        alignment_space: 2,
        icon: "",
        value: uname_r(),
    };

    let packs = InfoItem {
        title: "packs",
        alignment_space: 3,
        icon: "󰿺",
        value: get_packages(),
    };

    let user = InfoItem {
        title: "user",
        alignment_space: 4,
        icon: "",
        value: whoami(),
    };

    let term = InfoItem {
        title: "term",
        alignment_space: 4,
        icon: "",
        value: get_terminal(),
    };

    let de = InfoItem {
        title: "de/wm",
        alignment_space: 3,
        icon: "",
        value: get_wm(),
    };

    let cpu = InfoItem {
        title: "cpu",
        alignment_space: 5,
        icon: "󰍛",
        value: get_cpu_info(),
    };

    let mem = InfoItem {
        title: "mem",
        alignment_space: 5,
        icon: "",
        value: get_mem(),
    };

    let res = InfoItem {
        title: "res",
        alignment_space: 5,
        icon: "",
        value: get_res(),
    };

    let uptime = InfoItem {
        title: "uptime",
        alignment_space: 2,
        icon: "󰄉",
        value: match get_uptime() {
            Err(_err) => "".to_string(),
            Ok(time) => time,
        },
    };

    let gpu = InfoItem {
        title: "gpu",
        alignment_space: 5,
        icon: "󰍹",
        value: match get_gpu_info() {
            Err(_err) => "".to_string(),
            Ok(gpu_info) => gpu_info,
        },
    };

    let disk = InfoItem {
        title: "disk",
        alignment_space: 4,
        icon: "",
        value: get_disk_usage(),
    };

    let timezone = InfoItem {
        title: "timezone",
        alignment_space: 0,
        icon: "󰥔",
        value: get_timezone(),
    };

    let empty = InfoItem {
        title: "empty",
        alignment_space: 0,
        icon: "",
        value: String::new(),
    };

    let parse_info = |name: String| {
        return match name.to_lowercase().as_str() {
            "os" | "distro" => &distro,
            "host" | "hostname" => &hostname,
            "shell" => &shell,
            "kernel" => &kernel,
            "packs" | "packages" => &packs,
            "user" | "username" => &user,
            "term" | "terminal" => &term,
            "de" | "dewm" | "wm" => &de,
            "cpu" | "processor" => &cpu,
            "gpu" | "graphics" => &gpu,
            "mem" | "memory" => &mem,
            "uptime" => &uptime,
            "res" | "display" | "resolution" => &res,
            "time" | "timezone" => &timezone,
            "disk" | "diskusage" => &disk,
            _ => &empty,
        };
    };

    let parse_json_lists = |set| {
        let mut info_set: Vec<InfoItem> = vec![];
        for i in get_info(set, use_custom_info_config, custom_info_config_file.clone()) {
            info_set.push(parse_info(i).clone());
        }
        info_set
    };

    if get_only_info.is_some() {
        return format!(
            "{}",
            parse_info(get_only_info.unwrap()).value.trim_matches('"')
        );
    }

    let info_set1 = parse_json_lists("info1");
    let info_set2 = parse_json_lists("info2");
    let info_set3 = parse_json_lists("info3");

    let margin_spaces = " ".repeat(margin as usize);
    let distroascii = print_ascii(
        get_distro_ascii(legacy, overriden_ascii),
        get_color_config(
            "color0".to_string(),
            use_custom_color_config,
            custom_color_config_file.clone(),
        ),
        !use_custom_color_config,
        custom_color_config_file.clone(),
    );
    let infos1 = (1, info_set1);
    let infos2 = (2, info_set2);
    let infos3 = (3, info_set3);
    let mut info_sets = vec![infos1, infos2, infos3];
    let mut complete_data = Vec::new();

    for infos in info_sets.iter_mut() {
        complete_data.push(loop_over_data(
            &mut infos.1,
            margin_spaces.clone(),
            infos.0,
            use_custom_color_config,
            custom_color_config_file.clone(),
        ) + "\n");
    }

    match legacy {
        true => {
            println!("{}{}", distroascii, complete_data.join("\n"));
        }
        false => {
            side_by_side(distroascii.as_str(), complete_data.join("\n").as_str());
        }
    }

    String::new()
}


fn side_by_side(str1: &str, str2: &str) {
    let lines1: Vec<&str> = str1.lines().collect();
    let lines2: Vec<&str> = str2.lines().collect();
    let max_lines = lines1.len().max(lines2.len());
    let mut max_left_width = 0;
    
    for line in &lines1 {
        max_left_width = max_left_width.max(line.len());
    }

    for i in 0..max_lines {
        let left = lines1.get(i).unwrap_or(&"").trim_end();
        let right = lines2.get(i).unwrap_or(&"").trim_end();
        println!("{:<width$}{}", left, right.trim(), width = max_left_width);
    }
}


fn loop_over_data(
    list: &mut Vec<InfoItem>,
    margin: String,
    section: i8,
    use_custom_config: bool,
    custom_color_config_file: Option<String>,
) -> String {
    list.retain(|s| !s.value.is_empty());
    let len = list.len();
    let mut data = Vec::new();

    for (idx, item) in list.clone().iter().enumerate() {
        let color = get_color_config(
            format!("color{}", section),
            use_custom_config,
            custom_color_config_file.clone(),
        );

        let connector = if idx == 0 {
            "╭─"
        } else if idx == len - 1 {
            "╰─"
        } else {
            "├─"
        };

        data.push(format!("{}{}", margin, print_data(item, color, connector)));
    }

    data.join("\n")
}
