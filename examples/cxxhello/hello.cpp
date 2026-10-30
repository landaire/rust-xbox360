// C++ program for the Xbox 360 (clang++). Like chello, it imports DbgPrint
// and stores a heartbeat that `xeedee getmem` can read.
extern "C" {
__attribute__((naked, used, section(".xthunks")))
unsigned int DbgPrint(const char *s) {
    __asm__ volatile(".long 0x01000003\n\t.long 0x60000000\n\t.long 0x60000000\n\t.long 0x4e800020\n\t");
}
}

__attribute__((used, aligned(4))) volatile unsigned int HEARTBEAT = 0x43580000u; // "CX.."

namespace {
template <typename T> class Counter {
public:
    explicit Counter(T start) : value_(start) {}
    T next() { return value_++; }

private:
    T value_;
};
} // namespace

extern "C" void real_main() {
    Counter<unsigned int> counter(0);
    for (;;) {
        DbgPrint("cxxhello: hello from C++ on the xbox 360, built with clang\n");
        HEARTBEAT = 0x43580000u | (counter.next() & 0xffff);
        for (volatile int s = 0; s < 4000000; s++) {
        }
    }
}

extern "C" __attribute__((naked, used, section(".text._start"))) void _start() {
    __asm__ volatile("bl real_main\n\t1: b 1b\n\t");
}
