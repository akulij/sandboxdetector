const DISK_SIZE_MIN: u32 = 60; // GB

#[cfg(windows)]
pub fn check_common_vm() -> bool {
    let mut flag = false;

    flag |= check_disk_size();
    flag |= check_sleep_patch();
    flag |= check_cpu_count();
    flag |= check_native_vhd_boot();

    flag
}

#[cfg(windows)]
pub fn check_vmware() -> bool {
    let mut flag = false;

    flag |= check_vmwaretools_regkey();
    flag |= check_vmware_hkeys();

    flag
}

#[cfg(windows)]
pub fn check_qemu() -> bool {
    let mut flag = false;

    flag |= check_qemu_hkeys();

    flag
}

#[cfg(windows)]
pub fn check_hyperv() -> bool {
    todo!();
}

#[cfg(windows)]
pub fn check_vbox() -> bool {
    let mut flag = false;

    flag |= check_vbox_hkeys();
    flag |= check_vbox_regkey();

    flag
}

fn check_vmwaretools_regkey() -> bool {
    use winapi::um::winreg::HKEY_LOCAL_MACHINE;

    crate::tools::regkey_exists(HKEY_LOCAL_MACHINE, "SOFTWARE\\VMware, Inc.\\VMware Tools")
}

fn check_vmware_hkeys() -> bool {
    use crate::tools::regkey_value_contains;
    use winapi::um::winreg::HKEY_LOCAL_MACHINE;

    for i in 0..=2 {
        if regkey_value_contains(
            HKEY_LOCAL_MACHINE,
            (String::from("HARDWARE\\DEVICEMAP\\Scsi\\Scsi Port ")
                + i.to_string().as_str()
                + "\\Scsi Bus 0\\Target Id 0\\Logical Unit Id 0")
                .as_str(),
            "Identifier",
            "VMWARE",
        )
        .unwrap_or(false)
        {
            return true;
        }
    }

    return false;
}

fn check_qemu_hkeys() -> bool {
    use crate::tools::regkey_value_contains;
    use winapi::um::winreg::HKEY_LOCAL_MACHINE;

    let mut flag = false;

    flag |= regkey_value_contains(
        HKEY_LOCAL_MACHINE,
        "HARDWARE\\DEVICEMAP\\Scsi\\Scsi Port 0\\Scsi Bus 0\\Target Id 0\\Logical Unit Id 0",
        "Identifier",
        "QEMU",
    )
    .unwrap_or(false);
    flag |= regkey_value_contains(
        HKEY_LOCAL_MACHINE,
        "HARDWARE\\Description\\System",
        "SystemBiosVersion",
        "QEMU",
    )
    .unwrap_or(false);

    flag
}

fn check_vbox_regkey() -> bool {
    use winapi::um::winreg::HKEY_LOCAL_MACHINE;

    let mut flag = false;

    flag |= crate::tools::regkey_exists(HKEY_LOCAL_MACHINE, "HARDWARE\\ACPI\\DSDT\\VBOX__");
    flag |= crate::tools::regkey_exists(HKEY_LOCAL_MACHINE, "HARDWARE\\ACPI\\FADT\\VBOX__");
    flag |= crate::tools::regkey_exists(HKEY_LOCAL_MACHINE, "HARDWARE\\ACPI\\RSDT\\VBOX__");
    flag |= crate::tools::regkey_exists(HKEY_LOCAL_MACHINE, "SOFTWARE\\Oracle\\VirtualBox Guest Additions");
    flag |= crate::tools::regkey_exists(HKEY_LOCAL_MACHINE, "SYSTEM\\ControlSet001\\Services\\VBoxGuest");
    flag |= crate::tools::regkey_exists(HKEY_LOCAL_MACHINE, "SYSTEM\\ControlSet001\\Services\\VBoxMouse");
    flag |= crate::tools::regkey_exists(HKEY_LOCAL_MACHINE, "SYSTEM\\ControlSet001\\Services\\VBoxService");
    flag |= crate::tools::regkey_exists(HKEY_LOCAL_MACHINE, "SYSTEM\\ControlSet001\\Services\\VBoxSF");
    flag |= crate::tools::regkey_exists(HKEY_LOCAL_MACHINE, "SYSTEM\\ControlSet001\\Services\\VBoxVideo");

    flag
}

fn check_vbox_hkeys() -> bool {
    use crate::tools::regkey_value_contains;
    use winapi::um::winreg::HKEY_LOCAL_MACHINE;

    let mut flag = false;

    flag |= regkey_value_contains(
        HKEY_LOCAL_MACHINE,
        "HARDWARE\\DEVICEMAP\\Scsi\\Scsi Port 0\\Scsi Bus 0\\Target Id 0\\Logical Unit Id 0",
        "Identifier",
        "VBOX",
    )
    .unwrap_or(false);
    flag |= regkey_value_contains(
        HKEY_LOCAL_MACHINE,
        "HARDWARE\\Description\\System",
        "SystemBiosVersion",
        "VBOX",
    )
    .unwrap_or(false);
    flag |= regkey_value_contains(
        HKEY_LOCAL_MACHINE,
        "HARDWARE\\Description\\System",
        "SystemBiosVersion",
        "VIRTUALBOX",
    )
    .unwrap_or(false);
    flag |= regkey_value_contains(
        HKEY_LOCAL_MACHINE,
        "HARDWARE\\Description\\System",
        "SystemBiosDate",
        "06/23/99",
    )
    .unwrap_or(false);


    flag
}

fn calc_disk_gb_size(bps: u32, spc: u32, clusters: u32) -> f32 {
    bps as f32 / 1024.0 * spc as f32 / 1024.0 * clusters as f32 / 1024.0
}
fn check_disk_size() -> bool {
    use winapi::um::fileapi::GetDiskFreeSpaceA;

    let mut sectors: u32 = 0;
    let mut bytes_per_sector: u32 = 0;
    let mut free_clusters: u32 = 0;
    let mut total_clusters: u32 = 0;

    unsafe {
        if GetDiskFreeSpaceA(
            "C:\\\0".as_ptr() as *const i8,
            &mut sectors,
            &mut bytes_per_sector,
            &mut free_clusters,
            &mut total_clusters,
        ) != 0
        {
            calc_disk_gb_size(bytes_per_sector, sectors, total_clusters) <= DISK_SIZE_MIN as f32
        } else {
            false
        }
    }
}

fn check_sleep_patch() -> bool {
    use winapi::um::synchapi::Sleep;
    use winapi::um::sysinfoapi::GetTickCount;

    unsafe {
        let start_time = GetTickCount();

        Sleep(500);

        GetTickCount() - start_time <= 450
    }
}

fn check_cpu_count() -> bool {
    use winapi::um::sysinfoapi::{GetSystemInfo, SYSTEM_INFO};

    let mut sys_info: SYSTEM_INFO = unsafe { std::mem::zeroed() };

    unsafe {
        GetSystemInfo(&mut sys_info);
    }

    sys_info.dwNumberOfProcessors < 2
}

fn check_native_vhd_boot() -> bool {
    use winapi::um::libloaderapi::GetModuleHandleA;
    use winapi::um::libloaderapi::GetProcAddress;

    let mut is_native = false;

    unsafe {
        let func_native = GetProcAddress(
            GetModuleHandleA("kernel32".as_ptr() as *const i8),
            "IsNativeVhdBoot".as_ptr() as *const i8,
        );
        if func_native as u32 != 0 {
            let fnptr = func_native as *const ();
            let fnptr: fn(&mut bool) -> () = std::mem::transmute(fnptr);
            fnptr(&mut is_native);
        }

        is_native
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vhd() {
        assert!(!check_native_vhd_boot());
    }
    #[test]
    fn test_disk_size() {
        assert!(!check_disk_size());
    }

    #[test]
    fn test_qemu() {
        assert!(!check_qemu());
    }
}
