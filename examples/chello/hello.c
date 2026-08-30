// C program for the Xbox 360 (clang). DbgPrint import + a memory heartbeat so
// execution is observable via `xeedee getmem` even without a debug channel.
__attribute__((naked, used, section(".xthunks")))
unsigned int DbgPrint(const char *s) {
    __asm__ volatile(".long 0x01000003\n\t.long 0x60000000\n\t.long 0x60000000\n\t.long 0x4e800020\n\t");
}
__attribute__((used, aligned(4))) volatile unsigned int HEARTBEAT = 0x43480000u; // "CH.."
void real_main(void) {
    unsigned int i = 0;
    for (;;) {
        DbgPrint("chello: hello from C on the xbox 360, built with clang\n");
        HEARTBEAT = 0x43480000u | (i & 0xffff);
        i++;
        for (volatile int s = 0; s < 4000000; s++) { }
    }
}
__attribute__((naked, used, section(".text._start")))
void _start(void) { __asm__ volatile("bl real_main\n\t1: b 1b\n\t"); }
