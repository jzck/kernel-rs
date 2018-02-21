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

`make run` runs the OS in a tmux window with `qemu` + `gdb`  
`make iso` generate an iso to run wherever

# References

  - [wiki.osdev.org](https://wiki.osdev.org) is a fucking goldmine
  - [wiki.osdev.org/Rust](https://wiki.osdev.org/Rust) everything rust related to OSes
  - [Writing an OS in rust](https://os.phil-opp.com/) extremely helpful to get things going on x86 and nightly rust
  - [Rust OS comparison](https://github.com/flosse/rust-os-comparison) roundup of current projects
