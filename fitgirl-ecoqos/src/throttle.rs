use std::ffi::c_void;

use windows::Win32::{
    Foundation::CloseHandle,
    System::Threading::{
        IDLE_PRIORITY_CLASS, NORMAL_PRIORITY_CLASS, OpenProcess,
        PROCESS_POWER_THROTTLING_CURRENT_VERSION, PROCESS_POWER_THROTTLING_EXECUTION_SPEED,
        PROCESS_POWER_THROTTLING_STATE, PROCESS_SET_INFORMATION, ProcessPowerThrottling,
        SetPriorityClass, SetProcessInformation,
    },
};

pub const THROTTLE: PROCESS_POWER_THROTTLING_STATE = PROCESS_POWER_THROTTLING_STATE {
    Version: PROCESS_POWER_THROTTLING_CURRENT_VERSION,
    ControlMask: PROCESS_POWER_THROTTLING_EXECUTION_SPEED,
    StateMask: PROCESS_POWER_THROTTLING_EXECUTION_SPEED,
};

pub const UNTHROTTLE: PROCESS_POWER_THROTTLING_STATE = PROCESS_POWER_THROTTLING_STATE {
    Version: PROCESS_POWER_THROTTLING_CURRENT_VERSION,
    ControlMask: PROCESS_POWER_THROTTLING_EXECUTION_SPEED,
    StateMask: 0,
};

pub fn toggle_efficiency_mode(pid: u32, enable: bool) -> Result<(), windows_result::Error> {
    let new_state = if enable { THROTTLE } else { UNTHROTTLE };

    unsafe {
        let hprocess = OpenProcess(PROCESS_SET_INFORMATION, false, pid)?;
        let processinformationclass = ProcessPowerThrottling;

        let processinformation = &new_state as *const _ as *const c_void;
        let dwpriorityclass = if enable {
            IDLE_PRIORITY_CLASS
        } else {
            NORMAL_PRIORITY_CLASS
        };

        SetProcessInformation(
            hprocess,
            processinformationclass,
            processinformation,
            size_of::<PROCESS_POWER_THROTTLING_STATE>() as u32,
        )?;
        SetPriorityClass(hprocess, dwpriorityclass)?;

        CloseHandle(hprocess)?;
    }

    Ok(())
}
