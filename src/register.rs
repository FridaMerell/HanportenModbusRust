use sqlite::Connection;

pub struct Registers {
    db: Connection,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            db: sqlite::open(":memory:").unwrap(),
        }
    }

    pub fn get_available_registers(&self) {
        let query = "SELECT * FROM registers;";
        let result = self.db.execute(query);
    }

    pub fn listen_on_serial(&self) {
        loop {}
    }
}
