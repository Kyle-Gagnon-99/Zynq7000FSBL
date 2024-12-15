use register_macro::register;

#[register(32, u32)]
struct UartControlRegisterTest {
    #[bits(0:2, default = 0b11)]
    pub control: u8,
}

fn main() {
    let reg = UartControlRegisterTest { control: 0 };
    let test = UartControlRegisterTest::test_control();
}
