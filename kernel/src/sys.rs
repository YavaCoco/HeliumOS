use core::{arch::asm, panic::PanicInfo};

use x86_64::instructions::{hlt, interrupts};

use crate::{println, proc::apic};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  println!("[Proc {}] {}", apic::id(), info);

  interrupts::disable();
  loop {
    hlt();
  }
}

pub fn pause() {
  unsafe {
    asm! {
      "pause"
    }
  }
}

pub fn event_loop() -> ! {
  unsafe {
    asm! {
      "sti",
      "2:",
      "hlt",
      "jmp 2b"
    }
  }
  unreachable!()
}