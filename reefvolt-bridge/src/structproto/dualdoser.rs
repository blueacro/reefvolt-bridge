use packed_struct::prelude::*;

#[derive(PackedStruct)]
#[packed_struct(endian = "lsb", bit_numbering = "msb0")]
pub struct CommandSetOutputs {
    #[packed_field(ty = "enum", element_size_bytes = "1")]
    command: Command,
    pump1: u8,
    pump2: u8,
}

impl CommandSetOutputs {
    pub fn new(pump1: u8, pump2: u8) -> Self {
        Self {
            command: Command::SetOutputs,
            pump1,
            pump2,
        }
    }
}

#[derive(PackedStruct)]
#[packed_struct(endian = "lsb", bit_numbering = "msb0")]
pub struct CommandRead {
    #[packed_field(ty = "enum", element_size_bytes = "1")]
    command: Command,
}

impl Default for CommandRead {
    fn default() -> Self {
        Self {
            command: Command::Read,
        }
    }
}

#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum Command {
    SetOutputs = 1,
    Read = 2,
    BootloaderEntry = 0xF0,
}

#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum Response {
    Status = 0,
}

#[derive(PackedStruct, Debug)]
#[packed_struct(endian = "lsb", bit_numbering = "msb0")]
pub struct ResponseStatus {
    #[packed_field(element_size_bytes = "1")]
    pump1: u8,
    #[packed_field(element_size_bytes = "1")]
    pump2: u8,
    #[packed_field(element_size_bits = "6")]
    _rest_float_status: u8,
    #[packed_field(element_size_bits = "1")]
    float_status2: bool,
    #[packed_field(element_size_bits = "1")]
    float_status1: bool,
    analog_float_status: u8,
    power_status: u8,
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn decode_response() {
        let response: Vec<u8> = vec![100, 100, 0x2, 8, 1];
        let result = ResponseStatus::unpack_from_slice(response.as_ref()).unwrap();
        assert_eq!(result.float_status1, false);
        assert_eq!(result.float_status2, true);
        assert_eq!(result.power_status, 1);
    }
}
