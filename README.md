Kernel from scratch (KFS) series of projects at Ecole 42 !

### [documentation](https://jzck.github.io/kernel/bluesnow/index.html)

### dependencies

  - `nasm` compiles the bootcode
  - `ld` links the bootcode and rust binary
  - `grub-mkrescue` builds the iso
  - `xargo` builds rust code
  - `qemu` runs the iso

See `.travis.yml` to get an ubuntu environment ready  
on archlinux `pacman -S rustup make grub xorriso mtools binutils gcc qemu`

### rust setup

```
rustup override add nightly
rustup component add rust-src
cargo install xargo
```

### running

`make iso` generate the iso
`make qemu` runs the OS in a tmux window with `qemu` + `gdb`  

### todo

  - remove assembly for a pure rust entry point
  - replace grub with something lighter

### inspiration

  - [wiki.osdev.org](https://wiki.osdev.org) is a fucking goldmine
  - [Phil Opperman's "Writing an OS in rust"](https://os.phil-opp.com/)
  - [Redox kernel](https://github.com/redox/kernel)
