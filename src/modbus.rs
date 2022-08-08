use crate::register::Registers;

pub struct ModbusCommunication {
    pub unit_id: i32,
    pub transaction_id: Vec<u8>,
    pub protocol_id: Vec<u8>,
    pub function_id: i32,
    pub raw: Vec<u8>,
}

impl ModbusCommunication {
    /**
     *
     * Example request:
     *
     * \x00\x01\x00\x00\x00\x06\x01\x03\x00\x6B\x00\x01
     *
     * \x00\x01 - transaction id
     * \x00\x00 - protocol id
     * \x00\x06 - number of bytes in the message (PDU = ProtocolDataUnit) to follow
     * \x01 - unit id
     * \x03 - function code
     * \x00\x6B - start address
     * \x00\x01 - holding registers quantity to return
     */
    pub fn parse_incomming(input: [u8; 12]) -> ModbusCommunication {
        let bytes = input;

        let transaction_id: &[u8] = &bytes[0..1];
        let protocol_id: &[u8] = &bytes[2..3];
        let unit_id = bytes[6] as i32;
        let function_code: i32 = bytes[7] as i32;

        ModbusCommunication {
            unit_id: unit_id,
            protocol_id: protocol_id.to_vec(),
            transaction_id: transaction_id.to_vec(),
            function_id: function_code,
            raw: bytes.to_vec(),
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:x?}", &self.raw)

        //format!("Credentials({})", &self.raw)
    }

    pub fn hangle_request() {}

    pub fn read_from_register(register_handler: Registers) {}

    pub fn response_holding_registers() {}
}
