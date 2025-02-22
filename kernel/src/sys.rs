use crate::{dev::framebuffer::debug_set_pixel, println, proc::apic};
use core::{arch::asm, panic::PanicInfo};
use x86_64::instructions::{hlt, interrupts};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  interrupts::disable();
  let id = apic::id();
  debug_set_pixel(30, 30, (255, 0, 0).into());
  debug_set_pixel(30, 30 + id + 1, (255, 0, 0).into());
  println!("Panic Info: {}", info);
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
  // in the event_loop we will simply halt, but before, we should sit in the bottom of the stack
  // This is because if we don't, the stack will keep growing with every timer tick, the timer tick will
  // call event_loop again, which will grow the stack again, and so on
  unsafe {
    asm! {
      "swapgs",
      "mov rsp, gs:[8]",
      "swapgs",
      "sti",
      "2:",
      "hlt",
      "jmp 2b"
    }
  }
  unreachable!()
}
