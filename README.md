_____________________________________________
![maxresdefault](https://github.com/user-attachments/assets/e00aef3a-4bf6-47de-b8d0-7029aa9e3b1b)
______________________________________________
# PINJECT
PINJECT is a seamless process injector for Linux, which inject your payload directly into other running process. It restore normal execution flow of the target program.

## Working
PINJECT uses simple method to inject payload into the running process.
  1) Pass the target process name you want to inject.<br>
  2) PINJECT itself recognize the pid of the target process and attach to it.<br>
  3) It identifies offset of RIP register.<br>
  4) And, overwrites existing bytes of RIP with shellcode/payload.<br>
  5) Then, it restore the original instruction of target program.<br>
  
<code>For testing phase i use shellcode that prints "Injected: ar.p" but can use bind-shell or reverse-shell with no exit syscall</code>

## Usage
```bash
:$ git clone https://github.com/0x00snape/pinject.git
:$ cd pinject
:$ cargo build --release
```

## Disclaimer 
Works with target process having proper write permissions.

## License
This project is licensed under [MIT](https://github.com/0x00snape/pinject/blob/main/LICENSE)
