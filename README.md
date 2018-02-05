Kernel from scratch (KFS) series of projects at Ecole 42 !

# compiling

### dependencies

  - `nasm` compiles the bootcode
  - `ld` links the bootcode
  - `grub-mkrescue` builds the iso
  - `xargo` builds rust code
  - `qemu` runs the iso

on archlinux `pacman -S make grub2 xorriso mtools binutils gcc qemu`

### rust setup

```
pacman -S rustup
rustup override add nightly
rustup component add rust-src
cargo install xargo
```

### docker
a standard development environment can be invoked:

```
docker run jzck/arch-kernel -it /usr/bin/zsh
```

clone the repo and `make iso`

# running

`make run` in your host operating system to launch qemu gtk window

# References

  - [Rust page on OSDev wiki](https://wiki.osdev.org/Rust)
  - [Writing an OS in rust](https://os.phil-opp.com/) extremely helpful to get things going on x86 and nightly rust
