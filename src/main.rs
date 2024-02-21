use std::{fs::OpenOptions, os::fd::AsRawFd, ptr::write_volatile, thread, time::Duration};

const GPIO_BASE: i64 = 0x3F20_0000;
const GPFSEL2: isize = 0x08;
const GPSET0: isize = 0x1C;
const GPCLR0: isize = 0x28;

fn main() -> ! {
    let fd = OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/gpiomem")
        .expect("Erro ao abrir /dev/gpio");
    let gpio_base = unsafe {
        libc::mmap(
            std::ptr::null_mut(),
            164,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            fd.as_raw_fd(),
            GPIO_BASE,
        )
    };

    unsafe {
        write_volatile(gpio_base.offset(GPFSEL2) as *mut u32, 1 << 18);
    }

    loop {
        unsafe {
            write_volatile(gpio_base.offset(GPSET0) as *mut u32, 1 << 26);
        }
        thread::sleep(Duration::from_millis(500));
        unsafe {
            write_volatile(gpio_base.offset(GPCLR0) as *mut u32, 1 << 26);
        }
        thread::sleep(Duration::from_millis(500));
    }
}
