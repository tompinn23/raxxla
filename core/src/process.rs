use std::os::windows::prelude::*;
use std::path::Path;
use windows::core::{PCWSTR, PWSTR};
use windows::Win32::Foundation::*;
use windows::Win32::Security::Authorization::{ConvertSidToStringSidW, GetNamedSecurityInfoW, SE_FILE_OBJECT};
use windows::Win32::Security::*;
use windows::Win32::System::Diagnostics::Debug::{FormatMessageW, FORMAT_MESSAGE_FROM_SYSTEM, FORMAT_MESSAGE_IGNORE_INSERTS};
use windows::Win32::System::ProcessStatus::EnumProcesses;
use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcess, OpenProcessToken, QueryFullProcessImageNameW, PROCESS_NAME_WIN32, PROCESS_QUERY_LIMITED_INFORMATION};

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

// pub(crate) fn self_owner() -> Option<String> {
//     let mut token = HANDLE::default();
//     unsafe {
//         if !OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token).is_ok() {
//             log::error!("failed to get named security info: {}", last_error_string());
//             return None;
//         }
//     }
//
//     let mut size = 0;
//     unsafe {
//         let _ = GetTokenInformation(token, TokenUser, None, 0, &mut size);
//     }
//
//     unsafe {
//         if size == 0 {
//             let _ = CloseHandle(token);
//             return None;
//         }
//     }
//
//     let mut buf = vec![0u8; size as usize];
//     let usr = buf.as_mut_ptr() as *mut TOKEN_USER;
//
//
//     let result = unsafe {
//         let res = if GetTokenInformation(token, TokenUser, Some(usr as *mut _), size, &mut size).is_ok() {
//             let mut sid_ptr: PWSTR = PWSTR::default();
//             if ConvertSidToStringSidW((*usr).User.Sid, &mut sid_ptr).is_ok() {
//                 let s = if !sid_ptr.is_null() {
//                     // Convert PWSTR to Rust String
//                     let len = (0..).take_while(|&i| *sid_ptr.0.add(i) != 0).count();
//                     let slice = std::slice::from_raw_parts(sid_ptr.0, len);
//                     Some(String::from_utf16_lossy(slice))
//                 } else {
//                     None
//                 };
//
//                 // Free the allocated string
//                 LocalFree(Some(HLOCAL(sid_ptr.0 as _)));
//                 s
//             } else {
//                 None
//             }
//         } else { None };
//         let _ = CloseHandle(token);
//         res
//     };
//
//     result
// }

fn process_owner(pid: u32) -> Option<String> {
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
            let res = if GetTokenInformation(token, TokenUser, Some(usr as *mut _), size, &mut size).is_ok() {
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
            } else { None };
            let _ = CloseHandle(token);
            res
        };
        let _ = CloseHandle(handle);
        result
    }
}

fn find_processes() -> Vec<u32> {
    let mut ret = Vec::new();
    unsafe {
        let mut pids = vec![0u32; 4096];
        let mut needed = 0;
        if !EnumProcesses(pids.as_mut_ptr(), (pids.len() * size_of::<u32>()) as u32, &mut needed).is_ok() {
            return vec![];
        }

        let count = needed as usize / size_of::<u32>();

        for &pid in &pids[..count] {
            if pid == 0 { continue; }

            let handle = match OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) {
                Ok(v) => v,
                Err(_) => continue,
            };

            let mut buf = vec![0u16; 260];
            let mut size = buf.len() as u32;

            if !QueryFullProcessImageNameW(handle, PROCESS_NAME_WIN32, PWSTR(buf.as_mut_ptr()), &mut size).is_ok() {
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