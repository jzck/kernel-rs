Kernel from scratch (KFS) series of projects at Ecole 42 !

### [documentation](https://jzck.github.io/kernel/bluesnow/index.html)

# building

`git submodule udpate --init`  

  - `nasm` compiles the bootcode
  - `ld` links the bootcode and rust binary
  - `grub-mkrescue` builds the iso (need xorriso and mtools)
  - `xargo` builds rust code

See `.travis.yml` to get an ubuntu environment ready  
on archlinux `pacman -S rustup make grub xorriso mtools binutils gcc qemu`  
on voidlinux `xbps-install -S rustup make grub xorriso mtools binutils gcc qemu nasm`


#### rust setup

We build on nightly channel because of some cool features not yet in stable.  
We need the rust sources to build with xargo for cross-compiling to custom platform.  

```
rustup component add rust-src
rustup override add nightly
rustup default nightly
cargo install xargo
```

# running

  - `make iso` builds a bootable iso with grub
  - `make qemu` runs the iso, 
  - `make qemu-reload` reloads the CD

# todo

  - remove assembly for a pure rust entry point
  - replace grub with something lighter	(no bootloader at all with `qemu -kernel` ?)

# inspiration

  - [wiki.osdev.org](https://wiki.osdev.org) is a fucking goldmine
  - [Phil Opperman's "Writing an OS in rust"](https://os.phil-opp.com/)
  - [Redox kernel](https://github.com/redox/kernel)
