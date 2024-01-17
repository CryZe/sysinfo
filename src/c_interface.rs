// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{Disks, Networks, Pid, Process, System};
use libc::{self, c_char, c_float, c_uint, c_void, size_t};
use std::ffi::CString;

/// on windows, libc has not include pid_t.
#[cfg(target_os = "windows")]
pub type PID = usize;

/// other platforms, use libc::pid_t
#[cfg(not(target_os = "windows"))]
pub type PID = libc::pid_t;

/// C string returned from `CString::into_raw`.
pub type RString = *const c_char;
/// Callback used by [`processes`][crate::System#method.processes].
pub type ProcessLoop = unsafe extern "C" fn(pid: PID, process: &Process, data: *mut c_void) -> bool;
/// Callback used by [`tasks`][crate::Process#method.tasks].
pub type ProcessPidLoop = unsafe extern "C" fn(pid: PID, data: *mut c_void) -> bool;

/// Equivalent of [`System::new()`][crate::System#method.new].
#[no_mangle]
pub extern "C" fn sysinfo_init() -> Box<System> {
    Box::new(System::new())
}

/// Equivalent of `System::drop()`. Important in C to cleanup memory.
#[no_mangle]
pub extern "C" fn sysinfo_destroy(_system: Box<System>) {}

/// Equivalent of [`System::refresh_memory()`][crate::System#method.refresh_memory].
#[no_mangle]
pub extern "C" fn sysinfo_refresh_memory(system: &mut System) {
    system.refresh_memory();
}

/// Equivalent of [`System::refresh_cpu_usage()`][crate::System#method.refresh_cpu_usage].
#[no_mangle]
pub extern "C" fn sysinfo_refresh_cpu(system: &mut System) {
    system.refresh_cpu_usage();
}

/// Equivalent of [`System::refresh_all()`][crate::System#method.refresh_all].
#[no_mangle]
pub extern "C" fn sysinfo_refresh_all(system: &mut System) {
    system.refresh_all();
}

/// Equivalent of [`System::refresh_processes()`][crate::System#method.refresh_processes].
#[no_mangle]
pub extern "C" fn sysinfo_refresh_processes(system: &mut System) {
    system.refresh_processes();
}

/// Equivalent of [`System::refresh_process()`][crate::System#method.refresh_process].
#[no_mangle]
pub extern "C" fn sysinfo_refresh_process(system: &mut System, pid: PID) {
    system.refresh_process(Pid(pid));
}

/// Equivalent of [`Disks::new()`][crate::Disks#method.new].
#[no_mangle]
pub extern "C" fn sysinfo_disks_init() -> Box<Disks> {
    Box::new(Disks::new())
}

/// Equivalent of `Disks::drop()`. Important in C to cleanup memory.
#[no_mangle]
pub extern "C" fn sysinfo_disks_destroy(_disks: Box<Disks>) {}

/// Equivalent of [`Disks::refresh()`][crate::Disks#method.refresh].
#[no_mangle]
pub extern "C" fn sysinfo_disks_refresh(disks: &mut Disks) {
    disks.refresh();
}

/// Equivalent of [`Disks::refresh_list()`][crate::Disks#method.refresh_list].
#[no_mangle]
pub extern "C" fn sysinfo_disks_refresh_list(disks: &mut Disks) {
    disks.refresh_list();
}

/// Equivalent of [`System::total_memory()`][crate::System#method.total_memory].
#[no_mangle]
pub extern "C" fn sysinfo_total_memory(system: &System) -> size_t {
    system.total_memory() as _
}

/// Equivalent of [`System::free_memory()`][crate::System#method.free_memory].
#[no_mangle]
pub extern "C" fn sysinfo_free_memory(system: &System) -> size_t {
    system.free_memory() as _
}

/// Equivalent of [`System::used_memory()`][crate::System#method.used_memory].
#[no_mangle]
pub extern "C" fn sysinfo_used_memory(system: &System) -> size_t {
    system.used_memory() as _
}

/// Equivalent of [`System::total_swap()`][crate::System#method.total_swap].
#[no_mangle]
pub extern "C" fn sysinfo_total_swap(system: &System) -> size_t {
    system.total_swap() as _
}

/// Equivalent of [`System::free_swap()`][crate::System#method.free_swap].
#[no_mangle]
pub extern "C" fn sysinfo_free_swap(system: &System) -> size_t {
    system.free_swap() as _
}

/// Equivalent of [`System::used_swap()`][crate::System#method.used_swap].
#[no_mangle]
pub extern "C" fn sysinfo_used_swap(system: &System) -> size_t {
    system.used_swap() as _
}

/// Equivalent of [`Networks::new()`][crate::Networks#method.new].
#[no_mangle]
pub extern "C" fn sysinfo_networks_init() -> Box<Networks> {
    Box::new(Networks::new())
}

/// Equivalent of `Networks::drop()`. Important in C to cleanup memory.
#[no_mangle]
pub extern "C" fn sysinfo_networks_destroy(_networks: Box<Networks>) {}

/// Equivalent of [`Networks::refresh_list()`][crate::Networks#method.refresh_list].
#[no_mangle]
pub extern "C" fn sysinfo_networks_refresh_list(networks: &mut Networks) {
    networks.refresh_list();
}

/// Equivalent of [`Networks::refresh()`][crate::Networks#method.refresh].
#[no_mangle]
pub extern "C" fn sysinfo_networks_refresh(networks: &mut Networks) {
    networks.refresh();
}

/// Equivalent of
/// `system::networks().iter().fold(0, |acc, (_, data)| acc + data.received() as size_t)`.
#[no_mangle]
pub extern "C" fn sysinfo_networks_received(networks: &Networks) -> size_t {
    networks.iter().fold(0, |acc: size_t, (_, data)| {
        acc.saturating_add(data.received() as size_t)
    })
}

/// Equivalent of
/// `system::networks().iter().fold(0, |acc, (_, data)| acc + data.transmitted() as size_t)`.
#[no_mangle]
pub extern "C" fn sysinfo_networks_transmitted(networks: &Networks) -> size_t {
    networks.iter().fold(0, |acc: size_t, (_, data)| {
        acc.saturating_add(data.transmitted() as size_t)
    })
}

/// Equivalent of [`System::cpus_usage()`][crate::System#method.cpus_usage].
///
/// * `length` will contain the number of CPU usage added into `procs`.
/// * `procs` will be allocated if it's null and will contain of CPU usage.
#[no_mangle]
pub unsafe extern "C" fn sysinfo_cpus_usage(
    system: &System,
    length: *mut c_uint,
    procs: *mut *mut c_float,
) {
    if procs.is_null() || length.is_null() {
        return;
    }

    let cpus = system.cpus();
    if (*procs).is_null() {
        (*procs) = libc::malloc(::std::mem::size_of::<c_float>() * cpus.len()) as *mut c_float;
    }
    for (pos, cpu) in cpus.iter().skip(1).enumerate() {
        (*(*procs).offset(pos as isize)) = cpu.cpu_usage();
    }
    *length = cpus.len() as c_uint - 1;
}

/// Equivalent of [`System::processes()`][crate::System#method.processes]. Returns an
/// array ended by a null pointer. Must be freed.
///
/// # ⚠️ WARNING ⚠️
///
/// While having this method returned processes, you should *never* call any refresh method!
#[no_mangle]
pub unsafe extern "C" fn sysinfo_processes(
    system: &System,
    fn_pointer: Option<ProcessLoop>,
    data: *mut c_void,
) -> size_t {
    if let Some(fn_pointer) = fn_pointer {
        let len = {
            let entries = system.processes();
            for (pid, process) in entries {
                if !fn_pointer(pid.0, process, data) {
                    break;
                }
            }
            entries.len() as size_t
        };
        len
    } else {
        0
    }
}

/// Equivalent of [`System::process()`][crate::System#method.process].
///
/// # ⚠️ WARNING ⚠️
///
/// While having this method returned process, you should *never* call any
/// refresh method!
#[no_mangle]
pub extern "C" fn sysinfo_process_by_pid(system: &System, pid: PID) -> Option<&Process> {
    system.process(Pid(pid))
}

/// Equivalent of iterating over [`Process::tasks()`][crate::Process#method.tasks].
///
/// # ⚠️ WARNING ⚠️
///
/// While having this method processes, you should *never* call any refresh method!
#[no_mangle]
pub unsafe extern "C" fn sysinfo_process_tasks(
    process: &Process,
    fn_pointer: Option<ProcessPidLoop>,
    data: *mut c_void,
) -> size_t {
    if let Some(fn_pointer) = fn_pointer {
        if let Some(tasks) = process.tasks() {
            for pid in tasks {
                if !fn_pointer(pid.0, data) {
                    break;
                }
            }
            tasks.len() as size_t
        } else {
            0
        }
    } else {
        0
    }
}

/// Equivalent of [`Process::pid()`][crate::Process#method.pid].
#[no_mangle]
pub extern "C" fn sysinfo_process_pid(process: &Process) -> PID {
    process.pid().0
}

/// Equivalent of [`Process::parent()`][crate::Process#method.parent].
///
/// In case there is no known parent, it returns `0`.
#[no_mangle]
pub extern "C" fn sysinfo_process_parent_pid(process: &Process) -> PID {
    process.parent().unwrap_or(Pid(0)).0
}

/// Equivalent of [`Process::cpu_usage()`][crate::Process#method.cpu_usage].
#[no_mangle]
pub extern "C" fn sysinfo_process_cpu_usage(process: &Process) -> c_float {
    process.cpu_usage()
}

/// Equivalent of [`Process::memory()`][crate::Process#method.memory].
#[no_mangle]
pub extern "C" fn sysinfo_process_memory(process: &Process) -> size_t {
    process.memory() as _
}

/// Equivalent of [`Process::virtual_memory()`][crate::Process#method.virtual_memory].
#[no_mangle]
pub extern "C" fn sysinfo_process_virtual_memory(process: &Process) -> size_t {
    process.virtual_memory() as _
}

/// Equivalent of [`Process::exe()`][crate::Process#method.exe].
#[no_mangle]
pub extern "C" fn sysinfo_process_executable_path(process: &Process) -> RString {
    if let Some(p) = process.exe().and_then(|exe| exe.to_str()) {
        if let Ok(c) = CString::new(p) {
            return c.into_raw();
        }
    }
    std::ptr::null()
}

/// Equivalent of [`Process::root()`][crate::Process#method.root].
#[no_mangle]
pub extern "C" fn sysinfo_process_root_directory(process: &Process) -> RString {
    if let Some(p) = process.root().and_then(|root| root.to_str()) {
        if let Ok(c) = CString::new(p) {
            return c.into_raw();
        }
    }
    std::ptr::null()
}

/// Equivalent of [`Process::cwd()`][crate::Process#method.cwd].
#[no_mangle]
pub extern "C" fn sysinfo_process_current_directory(process: &Process) -> RString {
    if let Some(p) = process.cwd().and_then(|cwd| cwd.to_str()) {
        if let Ok(c) = CString::new(p) {
            return c.into_raw();
        }
    }
    std::ptr::null()
}

/// Frees a C string created with `CString::into_raw()`.
#[no_mangle]
pub unsafe extern "C" fn sysinfo_rstring_free(s: RString) {
    if !s.is_null() {
        let _ = CString::from_raw(s.cast_mut());
    }
}

/// Equivalent of [`cpu::vendor_id()`].
#[no_mangle]
pub extern "C" fn sysinfo_cpu_vendor_id(system: &System) -> RString {
    if let Some(c) = system
        .cpus()
        .first()
        .and_then(|cpu| CString::new(cpu.vendor_id().as_encoded_bytes()).ok())
    {
        c.into_raw()
    } else {
        std::ptr::null()
    }
}

/// Equivalent of [`cpu::brand()`].
#[no_mangle]
pub extern "C" fn sysinfo_cpu_brand(system: &System) -> RString {
    if let Some(c) = system
        .cpus()
        .first()
        .and_then(|cpu| CString::new(cpu.brand().as_encoded_bytes()).ok())
    {
        c.into_raw()
    } else {
        std::ptr::null()
    }
}

/// Equivalent of [`system::physical_core_count()`].
#[no_mangle]
pub extern "C" fn sysinfo_cpu_physical_cores(system: &System) -> u32 {
    let count = system.physical_core_count().unwrap_or(0);
    count as u32
}

/// Equivalent of [`cpu::frequency()`].
#[no_mangle]
pub extern "C" fn sysinfo_cpu_frequency(system: &System) -> u64 {
    system
        .cpus()
        .first()
        .map(|cpu| cpu.frequency())
        .unwrap_or(0)
}

/// Equivalent of [`System::name()`][crate::System#method.name].
#[no_mangle]
pub extern "C" fn sysinfo_system_name() -> RString {
    if let Some(c) = System::name().and_then(|p| CString::new(p.as_encoded_bytes()).ok()) {
        c.into_raw()
    } else {
        std::ptr::null()
    }
}

/// Equivalent of [`System::version()`][crate::System#method.version].
#[no_mangle]
pub extern "C" fn sysinfo_system_version() -> RString {
    if let Some(c) = System::os_version().and_then(|c| CString::new(c.as_encoded_bytes()).ok()) {
        c.into_raw()
    } else {
        std::ptr::null()
    }
}

/// Equivalent of [`System::kernel_version()`][crate::System#method.kernel_version].
#[no_mangle]
pub extern "C" fn sysinfo_system_kernel_version() -> RString {
    if let Some(c) = System::kernel_version().and_then(|c| CString::new(c.as_encoded_bytes()).ok())
    {
        c.into_raw()
    } else {
        std::ptr::null()
    }
}

/// Equivalent of [`System::host_name()`][crate::System#method.host_name].
#[no_mangle]
pub extern "C" fn sysinfo_system_host_name() -> RString {
    if let Some(c) = System::host_name().and_then(|c| CString::new(c.as_encoded_bytes()).ok()) {
        c.into_raw()
    } else {
        std::ptr::null()
    }
}

/// Equivalent of [`System::long_os_version()`][crate::System#method.long_os_version].
#[no_mangle]
pub extern "C" fn sysinfo_system_long_version() -> RString {
    if let Some(c) = System::long_os_version().and_then(|c| CString::new(c.as_encoded_bytes()).ok())
    {
        c.into_raw()
    } else {
        std::ptr::null()
    }
}
