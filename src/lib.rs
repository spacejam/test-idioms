pub struct MyObject {
    state: u64,
}

impl MyObject {
    pub fn new() -> MyObject {
        MyObject { state: 0 }
    }

    pub fn get_state(&self) -> u64 {
        self.state
    }

    pub fn apply(&mut self, input: u64) {
        self.state = input;

        assert_ne!(self.state, 0x1337_1337);
    }
}
