mod debugtest;
mod tools;
mod vmtest;

pub struct SandboxDetector {
    detect_debug: bool,
    detect_vmware: bool,
    detect_hyperv: bool,
    detect_qemu: bool,
    detect_common_vms: bool,
}

impl Default for SandboxDetector {
    fn default() -> Self {
        SandboxDetector {
            detect_debug: true,
            detect_vmware: true,
            detect_hyperv: true,
            detect_qemu: true,
            detect_common_vms: true,
        }
    }
}

impl SandboxDetector {
    pub fn detect(&self) -> bool {
        let mut flag = false;

        if self.detect_debug {
            flag |= debugtest::is_debugging();
        }
        if self.detect_vmware {
            flag |= vmtest::check_vmware();
        }
        if self.detect_hyperv {
            todo!();
            // flag |= todo!();
        }
        if self.detect_qemu {
            flag |= vmtest::check_qemu();
        }
        if self.detect_common_vms {
            flag |= vmtest::check_common_vm();
        }

        flag
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_sd() {
        let s: SandboxDetector = Default::default();

        assert!(s.detect_debug);
    }

    #[test]
    fn test_detect() {
        let s: SandboxDetector = Default::default();

        assert!(!s.detect())
    }
}
