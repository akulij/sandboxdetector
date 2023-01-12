#[cfg(windows)]
pub fn is_debugging() -> bool {
    let mut debuggin_flag = false;

    debuggin_flag |= is_debugger_present();

    debuggin_flag
}

fn is_debugger_present() -> bool {
    use winapi::um::debugapi::IsDebuggerPresent;

    unsafe { IsDebuggerPresent() != 0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_debugger_present() {
        assert!(!is_debugger_present());
    }
}
