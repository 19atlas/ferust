pub struct SystemInfos {
    pub os_name: String,
    pub kernel: String,
    pub username: String,
    pub hostname: String,
    pub shell: String,
    pub uptime: String
}
pub mod sys {
    use crate::kynk::SystemInfos;
    use std::process::Command;
    pub fn init() -> SystemInfos {
        SystemInfos {
            os_name: get_os(),
            kernel: get_kernel(),
            username: get_username(),
            hostname: get_hostname(),
            shell: get_shell(),
            uptime: get_syszaman()
        }
    }

    pub fn get_os() -> String {
        #[cfg(target_os = "windows")]
        return "Windows".to_string();
        #[cfg(target_os = "macos")]
        return "macOS".to_string();
        #[cfg(target_os = "ios")]
        return "iOS".to_string();
        #[cfg(target_os = "linux")]
        return get_linux_distro("/etc/os-release");
        #[cfg(target_os = "android")]
        return "Android".to_string();
        #[cfg(target_os = "freebsd")]
        return "FreeBSD".to_string();
        #[cfg(target_os = "dragonfly")]
        return "DragonflyBSD".to_string();
        #[cfg(target_os = "openbsd")]
        return "OpenBSD".to_string();
        #[cfg(target_os = "netbsd")]
        return "NetBSD".to_string();
        #[cfg(target = "unix")]
        return "Unix".to_string();
    }

    pub fn get_linux_distro(file: &str) -> String {
        if std::path::Path::new(file).exists() {
            if let Ok(lines) = crate::ekhizmet::yardimlar::read_lines(file) {
                for line in lines {
                    if let Ok(ip) = line {
                        if ip.starts_with('P') {
                            if ip.contains("PRETTY_NAME=\"") {
                                return ip.replace("PRETTY_NAME=", "").replace("\"", "");
                            }
                        }
                    }
                }
            }
        }
        "GNU/Linux".to_string()
    }

    pub fn get_kernel() -> String {
        #[cfg(target_os = "windows")]
        return "NT".to_string();
        #[cfg(target_os = "macos")]
        return "XNU".to_string();
        #[cfg(target_os = "ios")]
        return "XNU".to_string();
        #[cfg(target_os = "android")] return get_kernel_version();
        #[cfg(target_os = "freebsd")]
        return "BSD".to_string();
        #[cfg(target_os = "dragonfly")]
        return "BSD".to_string();
        #[cfg(target_os = "openbsd")]
        return "BSD".to_string();
        #[cfg(target_os = "netbsd")]
        return "BSD".to_string();
        #[cfg(target = "unix")]
        return "Unix".to_string();
        #[cfg(target_os = "linux")] return get_kernel_version();
    
    }

    pub fn get_kernel_version() -> String {
        let krl_vr = Command::new("uname").arg("-r").output();
        let krl_vr = match krl_vr {
            Ok(x) => {
                let rev_kernel_ver: String =
                    String::from_utf8(x.stdout).unwrap().chars().rev().collect();
                let rev_kernel_ver = rev_kernel_ver
                    .split("-")
                    .last()
                    .unwrap()
                    .chars()
                    .rev()
                    .collect();

                rev_kernel_ver
            }
            Err(_) => "Bilinmeyen".to_string(),
        };
        krl_vr
    }

    pub fn get_username() -> String {
        std::env::var(if cfg!(target_os = "linux") {
            "USER"
        } else {
            "USERNAME"
        })
        .unwrap()
    }

    pub fn get_hostname() -> String {
        std::env::var(if cfg!(target_os = "linux") {
            "HOSTNAME"
        } else {
            "COMPUTERNAME"
        })
        .unwrap()
    }

    pub fn get_shell() -> String {
        #[cfg(target_os = "linux")]
        use std::env;
        let shell_var = "SHELL";
        match env::var(shell_var) {
            Ok(mut val) => {
                val = val.replace("/", " ");
                val.split(" ").last().unwrap().to_string()
            }
            Err(_) => "Bilinmeyen".to_string(), 
        }
    }

    pub fn get_syszaman() -> String {
        //`uptime -p` command.
        let up_time = Command::new("uptime").arg("-p").output();
    
        let up_time = match up_time {
            Ok(x) => {
                let time = String::from_utf8(x.stdout)
                    .unwrap()
                    .replace("hours", "saat")
                    .replace("hour", "saat")
                    .replace("minutes", "dakkikadir")
                    .replace("minute", "dakkikadir")
                    .replace("days", "g??n")
                    .replace("day", "g??n")
                    .replace("up ", "");
                time
            }
            Err(_) => "Unknown".to_string(),
        };
    
        let up_time = up_time.replace("\n", ""); // Remove any newline character
    
        up_time
    }
    

    pub fn is_unix_like() -> bool {
        return if cfg!(target_os = "linux")
            || cfg!(target_os = "freebsd")
            || cfg!(target_os = "openbsd")
            || cfg!(target_os = "macos")
            || cfg!(target_os = "ios")
            || cfg!(target_os = "dragonfly")
            || cfg!(target_os = "netbsd")
        {
            true
        } else {
            false
        };
    }
}