_____________________________________________
![maxresdefault](https://github.com/user-attachments/assets/e00aef3a-4bf6-47de-b8d0-7029aa9e3b1b)
______________________________________________
# PINJECT
PINJECT is a seamless process injector for Linux, which inject your payload directly into other running process. It restore normal execution flow of the target program after payload injected.

## Working
PINJECT uses simple method to inject payload into the running process.
  1) Pass the target process name you want to inject.<br>
  2) PINJECT itself recognize the pid of the target process and attach to it.<br>
  3) It sets tracer option (PTRACE_O_TRACEFORK) to trace forks into target process.<br>
  4) Saves the original registers, RIP and instruction.<br>
  5) Inject shellcode that call fork() syscall.<br>
  6) Catch that injected fork() syscall with getevent() for getting the child pid CPID.<br>
  7) And, overwrites existing bytes of RIP with payload on that child process.<br>
  8) Then, it restore the original instruction of target program.<br>
  
<code>For testing phase i use shellcode that prints "Injected: ar.p" but can use bind-shell or reverse-shell</code>

## Usage
```bash
:$ git clone https://github.com/0x00snape/pinject.git
:$ cd pinject
:$ cargo build --release
```


## Disclaimer 
Works with target process having proper write permissions.

## POC
_____________________________________________
![pocpinject](https://github.com/user-attachments/assets/3eebe7a5-54f6-4ae2-9e79-7cfc261462e4)
______________________________________________

## License
This project is licensed under [MIT](https://github.com/0x00snape/pinject/blob/main/LICENSE)
