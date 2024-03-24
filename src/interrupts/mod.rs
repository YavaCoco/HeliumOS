mod exceptions;
pub mod pic;

use self::exceptions::{
  div::div,
  gprot::gprot,
  lapic::{error::error, spurious::spurious, timer::timer},
  pagefault::page_fault,
};
use spin::RwLock;
use x86_64::structures::idt::InterruptDescriptorTable;

pub const ERROR_STACK_SIZE: usize = 2 * 1024;

#[repr(u8)]
pub enum Vectors {
  LocalApicError = 0xFD,
  LocalApicSpurious = 0xFF,
  LocalApicTimer = 0xF0,
}

impl Into<u8> for Vectors {
  fn into(self) -> u8 {
    self as u8
  }
}

static IDT: RwLock<InterruptDescriptorTable> =
  RwLock::new(InterruptDescriptorTable::new());

pub fn init() {
  let mut idt = IDT.write();

  idt.divide_error.set_handler_fn(div);
  idt.general_protection_fault.set_handler_fn(gprot);
  idt.page_fault.set_handler_fn(page_fault);

  idt[Vectors::LocalApicError as u8].set_handler_fn(error);
  idt[Vectors::LocalApicSpurious as u8].set_handler_fn(spurious);
  idt[Vectors::LocalApicTimer as u8].set_handler_fn(timer);
}

pub fn load() {
  let idt = IDT.read();
  unsafe { idt.load_unsafe() };
}
