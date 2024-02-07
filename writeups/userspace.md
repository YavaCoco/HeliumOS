Start: Jan 29, 2024.
End:
# Objective
Have the ability to load a user space program, provide a syste call for printing to kterm
and to exit the program.

# Progress
- [X] Load flat binary to user space addresses, and be able to call it
- [X] Switch CPU to user mode
- [ ] Implement print & exit syscalls
- [ ] Load ELF executable
- [ ] Setup build system to make this process streamlined

# Initial steps
First thing I want to do is remove the entire file system, I will replace the initrd
reading with its own system, the filesystem will only be used by user space programs, not
the kernel itself.

Now that is done, I will make a dummy flat binary executable that just contains some mov
instructions, call it in kernel mode, and mark the first step as done.

And that is done, the only "difficulty" I had is that I was trying to map a virtual address
to another virtual address, this stupid thing cost me an hour.

What I have now is just a binary that moves 42 into rax then returns, and in effect it's like
an int function that returns 42, and I can see its result. I would really love to make it print something
but calls use relative addresses, and the 2G where I load the flat binary is very far from where kterm_print would be
and I don't want to spend time making trampolines, so I will just make it print after I switch to user mode
and implement system calls.
For now, I can move to the next step, switching to usermode, this probably would make the OS crash since ret won't be enough
to return to the kernel.

I spent yesterday night reading a lot about stack switching etc, and I believe the best way to move forward is to remove
kernel shell. Having the kernel shell requires me to keep track of where the stack was left etc, what I would like is to
simply after initialization, launch the userspace init process and the kernel init program can disappear, the kernel shell
is technically part of the init program. I want to configure the interrupts to use stack switching and start the stack from zero
all over again. The init process can itself be the shell once the required system calls are implemented.

There is a problem with this, what if a processor receives two interrupts? I can simply disable interrupts while we're
handling one, but what about NMIs? a very lazy solution is to have a lock, when an interrupt happens it locks and any
interrupt that can bypass the mask, but then I should use assembly to lock to make sure I don't push anything to the stack
while locking, and I am imagining I will have some global structure per CPU that will contain the lock and then I will have to
access it from assembly which sounds like a lot of work, and god forbids the event where I change the layout of the structure but
forget to update the assembly code, I will be having some very fun time.

What really makes me want to ensure I have interrupt stacks is that apparently sysexit does not restore the stack to userspace
which means the kernel must do it before doing the sysret, which leaves a short frame where an interrupt can happen while the
stack is in userspace.

I am having a little trouble trying to figure out where to start, as such i decided to do the obvious first, create GDT entries
for userspace code.

While doing that I realized I was probably on drugs when I implmented the GDT, I am using the GDT entries of 32 bit systems and
trying to setup a 64 bit system with them. I have no clue how this did not crash, I am suspecting that since the GDT dropped a lot
of its functions in x86_64, it simply didn't matter much, anyway, I am fixing that now.

NO NO NO NO NO, I was not drunk, GDT entries in x86-64 still take up 8 bytes in size, it is TSS entries that take the place of two
items in the GDT instead.

It took me more time than what it should have but its done. I was in such a rush to do it quickly I forgot to initialize half the
fields of the TSS entry in the GDT, and spent quite some time debugging the issue.

I tried running the current state of the code with 2 parallel processors, and after initialization is done, for some reason memory
initialization is ran again, idk by which processor, then Proc 1 receives a #GP, Proc 0 continues to run normally, invalid commands
are handled propely, `version` and `clear` also work fine, but `exec` causes it to receive a #GP as well.

It's not only memory initialization, it's the entire _start function, and I found the issue, I removed the continuous halt after
initialization is done, so the secondary proc returned from `proc_ignition_wait` back to `_start` and messed up everything. Now
everything is fine again.

The rest of the implementation was pretty straighforward, I did not face many issues, none that were particularly hard, I fixed a bug with
`mem_alloc_vblock`, and now, I can call userspace code with CPL 3, then syscall back to the kernel with CPL 0. The registers ES, FS and GS still use
CPL 3 in user space code, the fix is probably easy, i'm just too lazy to fix it now especially that I don't care about security, neither do I know i
those registers can even be used in amd64.

Anyways, I will mark the switch CPU to user mode step as done. Loading an elf file should be easy.