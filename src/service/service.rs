use anyhow::anyhow;
use anyhow::Result as ServiceResult;
use winapi::um::{
    errhandlingapi, powrprof, processthreadsapi, securitybaseapi, winbase, winnt, winuser,
};

macro_rules! link_file_name {
    () => {
        "sleep-time.lnk"
    };
}

macro_rules! link_file_path {
    () => {
        concat!(
            r#"Microsoft\Windows\Start Menu\Programs\StartUp\"#,
            link_file_name!()
        )
    };
}

const LINK_FILE_PATH: &str = link_file_path!();

fn win32_string(value: &str) -> Vec<u16> {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    OsStr::new(value).encode_wide().chain(once(0)).collect()
}

fn aquire_shutdown_privilege() -> ServiceResult<()> {
    const WIN_BOOL_FALSE: winapi::shared::minwindef::BOOL = 0;
    unsafe {
        let mut token_handle: winnt::HANDLE = std::mem::MaybeUninit::zeroed().assume_init();
        let mut tkp: winnt::TOKEN_PRIVILEGES = std::mem::MaybeUninit::zeroed().assume_init();
        if processthreadsapi::OpenProcessToken(
            processthreadsapi::GetCurrentProcess(),
            winnt::TOKEN_ADJUST_PRIVILEGES | winnt::TOKEN_QUERY,
            &mut token_handle,
        ) == WIN_BOOL_FALSE
        {
            drop(token_handle);
            drop(tkp);
            return Err(anyhow!(
                "Failed to OpenProcessToken. code: {}",
                errhandlingapi::GetLastError()
            ));
        }
        winbase::LookupPrivilegeValueW(
            &winnt::UNICODE_NULL,
            winnt::SE_SHUTDOWN_NAME
                .encode_utf16()
                .collect::<Vec<u16>>()
                .as_ptr(),
            &mut (tkp.Privileges[0].Luid),
        );
        tkp.PrivilegeCount = 1; // one privilege to set
        tkp.Privileges[0].Attributes = winnt::SE_PRIVILEGE_ENABLED;
        if securitybaseapi::AdjustTokenPrivileges(
            token_handle,
            0,
            &mut tkp,
            0,
            winuser::WM_NULL as winnt::PTOKEN_PRIVILEGES,
            &mut 0,
        ) == WIN_BOOL_FALSE
        {
            drop(token_handle);
            drop(tkp);
            return Err(anyhow!(
                "Failed to AdjustTokenPrivileges. code: {}",
                errhandlingapi::GetLastError()
            ));
        }
        drop(token_handle);
        drop(tkp);
        return Ok(());
    }
}

pub fn enable_autorun() -> bool {
    // 当前用户专有的启动文件夹：%appdata%\Microsoft\Windows\Start Menu\Programs\StartUp
    // 所有用户有效的启动文件夹：%ProgramData%\Microsoft\Windows\Start Menu\Programs\StartUp
    #[cfg(windows)]
    if let Ok(Some(file_path)) = std::env::current_exe().as_ref().map(|path| path.to_str()) {
        if let Some(mut dest_path) = dirs::config_dir() {
            dest_path.push(LINK_FILE_PATH);
            return create_shortcut(file_path, dest_path.as_os_str().to_str().unwrap());
        }
    }
    return false;
}

fn create_shortcut(src_path: &str, dst_path: &str) -> bool {
    use std::{ptr::null_mut, u16};
    use winapi::ctypes::c_void;
    use winapi::shared::winerror::SUCCEEDED;
    use winapi::shared::wtypesbase::CLSCTX_INPROC_SERVER;
    use winapi::um::combaseapi::CoCreateInstance;
    use winapi::um::objbase::CoInitialize;
    use winapi::um::objidl::IPersistFile;
    use winapi::um::shobjidl_core::ShellLink;

    use winapi::Class;
    use winapi::{um::shobjidl_core::IShellLinkW, Interface};

    let mut is_ok = false;

    unsafe {
        CoInitialize(null_mut());
        let mut shell_link: *mut IShellLinkW = null_mut();
        let hr = CoCreateInstance(
            &ShellLink::uuidof(),
            null_mut(),
            CLSCTX_INPROC_SERVER,
            &IShellLinkW::uuidof(),
            &mut shell_link as *mut *mut _ as *mut *mut c_void,
        );
        if SUCCEEDED(hr) {
            let shell_link = &mut *shell_link;
            shell_link.SetPath(win32_string(src_path).as_ptr() as *const u16);

            let mut persist_file: *mut IPersistFile = null_mut();
            let hr = shell_link.QueryInterface(
                &IPersistFile::uuidof(),
                &mut persist_file as *mut *mut _ as *mut *mut c_void,
            );
            if SUCCEEDED(hr) {
                let persist_file = &mut *persist_file;
                persist_file.Save(win32_string(dst_path).as_ptr() as *const u16, 1);
                persist_file.Release();
                is_ok = true;
            }
            shell_link.Release();
        }
    }
    return is_ok;
}

fn get_link_file_path() -> Option<std::path::PathBuf> {
    let mut appdata_dir = dirs::config_dir().unwrap();
    appdata_dir.push(LINK_FILE_PATH);
    println!("{}", appdata_dir.to_str().unwrap());
    if appdata_dir.exists() {
        Some(appdata_dir)
    } else {
        None
    }
}

pub fn cancel_autorun() {
    if let Some(path) = get_link_file_path() {
        if let Err(_) = std::fs::remove_file(path) {
            println!("Failed to remove link file.");
        }
    }
}

pub fn is_enable_autorun() -> bool {
    get_link_file_path().map_or(false, |_| true)
}

pub fn shutdown_system() {
    let _ = aquire_shutdown_privilege();
    unsafe {
        powrprof::SetSuspendState(
            true as winnt::BOOLEAN,
            true as winnt::BOOLEAN,
            true as winnt::BOOLEAN,
        );
    }
}
