use vga::*;
use alloc::collections::BTreeMap;

pub static mut CONSOLE: Console = self::Console::new();

lazy_static! {
    static ref COMMANDS: BTreeMap<&'static str, fn()> = {
        let mut commands = BTreeMap::new();
        commands.insert("help", self::help as fn());

        // ACPI
        commands.insert("acpi", ::acpi::info as fn());
        commands.insert("reboot", ::acpi::reboot as fn());
        commands.insert("shutdown", ::acpi::shutdown as fn());

        // time
        commands.insert("uptime", ::time::uptime as fn());

        // cpu
        use arch::x86;
        commands.insert("cpu", x86::devices::cpu::cpu_info as fn());
        commands.insert("regs", x86::regs as fn());
        commands.insert("int3", ::x86::instructions::interrupts::int3 as fn());

        //memory
        commands.insert("stack", ::memory::print_stack as fn());
        commands.insert("page_fault", ::memory::page_fault as fn());
        commands.insert("overflow", ::memory::overflow as fn());

        //pci
        commands.insert("lspci", ::pci::lspci as fn());
        commands
    };
}

fn help() {
    println!("The following commands are available:");
    for (key, _val) in COMMANDS.iter() {
        println!("{}", key);
    }
}

pub struct Console {
    command: [u8; 10],
    command_len: usize,
}

impl Console {
    pub const fn new() -> Console {
        Console {
            command: [b'\0'; 10],
            command_len: 0,
        }
    }

    pub fn init(&self) {
        set_color!();
        // print!("{}", format_args!("{: ^4000}", r#" "#));
        unsafe {
            // VGA.buffer_pos = 0;
            self.prompt();
            VGA.flush();
        }
    }

    pub fn backspace(&mut self) {
        if self.command_len > 0 {
            self.command_len -= 1;
            unsafe { VGA.erase_byte(); }
        }
    }

    pub fn prompt(&self) {
        set_color!(Blue);
        unsafe { VGA.write_str("> "); }
        set_color!();
        flush!();
    }

    pub fn keypress(&mut self, ascii: u8) {
        match ascii {
            b'\n' if self.command_len == 0 => {
                unsafe { VGA.write_byte(b'\n'); }
                self.prompt();
            }
            b'\n' => {
                unsafe { VGA.write_byte(b'\n'); }
                self.exec();
                self.command_len = 0;
                self.prompt();
            }
            // _ if self.command_len >= 10 => (),
            // byte if self.command_len == 0 && byte == b' ' => (),
            byte => {
                if self.command_len >= 10 {
                    return;
                };
                self.command[self.command_len] = byte;
                unsafe { VGA.write_byte(byte); }
                self.command_len += 1;
            }
        }
        unsafe { VGA.flush(); }
    }

    fn get_command(&self) -> Result<fn(), &'static str> {
        match core::str::from_utf8(&self.command) {
            Ok(y) => {
                if let Some(command) = COMMANDS.get(&y[..self.command_len]) {
                    Ok(*command)
                } else {
                    Err("Command not found, try help")
                }
            },
            Err(_) => Err("Command is not utf8"),
        }
    }

    pub fn exec(&self) {
        let command = self.get_command();
        match command {
            Err(msg) => println!("{}", msg),
            Ok(func) => (func)(),
        }
    }
}
