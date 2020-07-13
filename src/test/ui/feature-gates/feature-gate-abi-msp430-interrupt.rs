// Test that the MSP430 interrupt ABI cannot be used when msp430_interrupt
// feature gate is not used.

// ignore-riscv64 msp430 is not supported

extern "msp430-interrupt" fn foo() {}
//~^ ERROR msp430-interrupt ABI is experimental and subject to change

fn main() {
    foo();
}
