use std::path::Path;

#[cfg(target_os = "windows")]
use windows::{file_owner, find_processes, process_owner};

#[cfg(target_os = "linux")]
use linux::{file_owner, find_processes, process_owner};

pub fn running(journal_dir: &str) -> bool {
    let dir_sid = match file_owner(Path::new(journal_dir)) {
        Some(d) => d,
        None => return false,
    };
    let pids = find_processes();

    for pid in pids {
        let psid = match process_owner(pid) {
            Some(psid) => psid,
            None => continue,
        };
        if dir_sid == psid {
            return true;
        }
    }
    false
}

#[cfg(target_os = "windows")]
mod windows {
    use std::os::windows::prelude::*;
    use std::path::Path;
    use windows::Win32::Foundation::*;
    use windows::Win32::Security::Authorization::{
        ConvertSidToStringSidW, GetNamedSecurityInfoW, SE_FILE_OBJECT,
    };
    use windows::Win32::Security::*;
    use windows::Win32::System::Diagnostics::Debug::{
        FORMAT_MESSAGE_FROM_SYSTEM, FORMAT_MESSAGE_IGNORE_INSERTS, FormatMessageW,
    };
    use windows::Win32::System::ProcessStatus::EnumProcesses;
    use windows::Win32::System::Threading::{
        OpenProcess, OpenProcessToken, PROCESS_NAME_WIN32, PROCESS_QUERY_LIMITED_INFORMATION,
        QueryFullProcessImageNameW,
    };
    use windows::core::{PCWSTR, PWSTR};

    fn error_string(code: WIN32_ERROR) -> String {
        let mut buffer: [u16; 512] = [0; 512];

        unsafe {
            let len = FormatMessageW(
                FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS,
                None,
                code.0,
                0,
                PWSTR(buffer.as_mut_ptr()),
                buffer.len() as u32,
                None,
            );

            String::from_utf16_lossy(&buffer[..len as usize])
        }
    }

    // fn last_error_string() -> String {
    //     let code = unsafe { GetLastError() };
    //     error_string(code)
    // }
    pub(crate) fn file_owner(path: &Path) -> Option<String> {
        let mut sd: PSECURITY_DESCRIPTOR = PSECURITY_DESCRIPTOR::default();
        let mut owner: PSID = PSID::default();

        let wpath: Vec<u16> = path.as_os_str().encode_wide().chain(Some(0)).collect();

        let err: WIN32_ERROR = unsafe {
            GetNamedSecurityInfoW(
                PCWSTR(wpath.as_ptr()),
                SE_FILE_OBJECT,
                OWNER_SECURITY_INFORMATION,
                Some(&mut owner as *mut _),
                None,
                None,
                None,
                &mut sd as *mut _,
            )
        };
        if err != ERROR_SUCCESS {
            log::error!("failed to get named security info: {}", error_string(err));
            return None;
        }

        let mut sid_ptr: PWSTR = PWSTR::default();

        let name = unsafe {
            if ConvertSidToStringSidW(owner, &mut sid_ptr).is_ok() {
                let s = if !sid_ptr.is_null() {
                    // Convert PWSTR to Rust String
                    let len = (0..).take_while(|&i| *sid_ptr.0.add(i) != 0).count();
                    let slice = std::slice::from_raw_parts(sid_ptr.0, len);
                    Some(String::from_utf16_lossy(slice))
                } else {
                    None
                };

                // Free the allocated string
                LocalFree(Some(HLOCAL(sid_ptr.0 as _)));
                s
            } else {
                None
            }
        };
        unsafe {
            if !sd.0.is_null() {
                LocalFree(Some(HLOCAL(sd.0 as _)));
            }
        }
        name
    }

    pub(crate) fn process_owner(pid: u32) -> Option<String> {
        unsafe {
            let handle = match OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) {
                Ok(v) => v,
                Err(_) => return None,
            };

            let mut token = HANDLE::default();
            if !OpenProcessToken(handle, TOKEN_QUERY, &mut token).is_ok() {
                let _ = CloseHandle(handle);
                return None;
            }

            let mut size = 0;
            let _ = GetTokenInformation(token, TokenUser, None, 0, &mut size);

            if size == 0 {
                let _ = CloseHandle(token);
                return None;
            }

            let mut buf = vec![0u8; size as usize];
            let usr = buf.as_mut_ptr() as *mut TOKEN_USER;

            let result = {
                let res =
                    if GetTokenInformation(token, TokenUser, Some(usr as *mut _), size, &mut size)
                        .is_ok()
                    {
                        let mut sid_ptr: PWSTR = PWSTR::default();
                        if ConvertSidToStringSidW((*usr).User.Sid, &mut sid_ptr).is_ok() {
                            let s = if !sid_ptr.is_null() {
                                // Convert PWSTR to Rust String
                                let len = (0..).take_while(|&i| *sid_ptr.0.add(i) != 0).count();
                                let slice = std::slice::from_raw_parts(sid_ptr.0, len);
                                Some(String::from_utf16_lossy(slice))
                            } else {
                                None
                            };

                            // Free the allocated string
                            LocalFree(Some(HLOCAL(sid_ptr.0 as _)));
                            s
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                let _ = CloseHandle(token);
                res
            };
            let _ = CloseHandle(handle);
            result
        }
    }

    pub(crate) fn find_processes() -> Vec<u32> {
        let mut ret = Vec::new();
        unsafe {
            let mut pids = vec![0u32; 4096];
            let mut needed = 0;
            if !EnumProcesses(
                pids.as_mut_ptr(),
                (pids.len() * size_of::<u32>()) as u32,
                &mut needed,
            )
            .is_ok()
            {
                return vec![];
            }

            let count = needed as usize / size_of::<u32>();

            for &pid in &pids[..count] {
                if pid == 0 {
                    continue;
                }

                let handle = match OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) {
                    Ok(v) => v,
                    Err(_) => continue,
                };

                let mut buf = vec![0u16; 260];
                let mut size = buf.len() as u32;

                if !QueryFullProcessImageNameW(
                    handle,
                    PROCESS_NAME_WIN32,
                    PWSTR(buf.as_mut_ptr()),
                    &mut size,
                )
                .is_ok()
                {
                    let _ = CloseHandle(handle);
                    continue;
                }

                let exe_name = String::from_utf16_lossy(&buf[..size as usize]);
                if exe_name.to_lowercase().contains("elitedangerous") {
                    ret.push(pid);
                }

                let _ = CloseHandle(handle);
            }
        }
        ret
    }
}

#[cfg(target_os = "linux")]
mod linux {
    use std::fs;
    use std::os::unix::fs::MetadataExt;
    use std::path::Path;

    pub(crate) fn file_owner(path: &Path) -> Option<u32> {
        fs::metadata(path).ok().map(|m| m.uid())
    }

    pub(crate) fn process_owner(pid: u32) -> Option<u32> {
        let status = fs::read_to_string(format!("/proc/{}/status", pid)).ok()?;

        for line in status.lines() {
            if line.starts_with("Uid:") {
                // Format: Uid:    real    effective    saved    fs
                let parts: Vec<&str> = line.split_whitespace().collect();
                return parts.get(1)?.parse().ok(); // real UID
            }
        }
        None
    }

    pub(crate) fn find_processes() -> Vec<u32> {
        let mut pids = Vec::new();

        let entries = match fs::read_dir("/proc") {
            Ok(e) => e,
            Err(_) => return pids,
        };

        for entry in entries.flatten() {
            let name = entry.file_name();
            let name = name.to_string_lossy();

            if let Ok(pid) = name.parse::<u32>() {
                let exe = fs::read_link(format!("/proc/{}/exe", pid)).ok();
                if let Some(path) = exe {
                    if let Some(name) = path.file_name() {
                        if name
                            .to_string_lossy()
                            .to_lowercase()
                            .contains("elitedangerous")
                        {
                            pids.push(pid);
                        }
                    }
                }
            }
        }

        pids
    }
}
