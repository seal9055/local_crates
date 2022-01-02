use std::mem;
use std::ffi::CString;
use std::ptr::null_mut;

use winapi::shared::minwindef::{LPVOID, DWORD, FALSE};
use winapi::um::{
    memoryapi::{VirtualAllocEx, WriteProcessMemory},
    processthreadsapi::{OpenProcess, CreateRemoteThread},
    libloaderapi::{GetModuleHandleA, GetProcAddress},
    winnt::{
        PROCESS_CREATE_THREAD,
        PROCESS_VM_OPERATION,
        PROCESS_VM_WRITE,
        MEM_RESERVE,
        MEM_COMMIT,
        PAGE_READWRITE,
    },
};

/// Get process address of specified function {func} in specified module {mod}
unsafe fn get_func_addr(module: &str, func: &str) -> u64 {
    let module = CString::new(module).unwrap();
    let func = CString::new(func).unwrap();
    let module_handle = GetModuleHandleA(module.as_ptr());

    GetProcAddress(module_handle, func.as_ptr()) as u64
}

/// Inject dll specified by {dll_path} into process with the pid of {pid}
pub unsafe fn inject_dll(pid: u32, dll_path: &str) -> Option<()> {
    let load_lib_addr = get_func_addr("Kernel32.dll", "LoadLibraryA");
    let dll_path = CString::new(dll_path).unwrap();
    let dll_path_len = dll_path.as_bytes_with_nul().len();

    let proc = OpenProcess(PROCESS_CREATE_THREAD | PROCESS_VM_OPERATION | PROCESS_VM_WRITE, FALSE, pid);
    let va_path = VirtualAllocEx(proc, null_mut(), dll_path_len, MEM_RESERVE | MEM_COMMIT, PAGE_READWRITE);
    WriteProcessMemory(proc, va_path, dll_path.as_ptr() as LPVOID, dll_path_len, null_mut());

    type ThreadStartRoutine = unsafe extern "system" fn(LPVOID) -> DWORD;
    let start_routine: ThreadStartRoutine = mem::transmute(load_lib_addr);

    CreateRemoteThread(proc, null_mut(), 0, Some(start_routine), va_path, 0, null_mut());
    Some(())
}