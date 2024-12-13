use register_macro::register;

#[register(32, u32)]
struct UartControlRegisterTest {
    pub control: u8,
}

fn main() {
    let reg = UartControlRegisterTest { control: 0 };
    reg.please_work();
}
