use serde_json::json;
use phf::phf_map;
use std::str;

pub enum Type {
    NONE,
    BOOL,
    SHORT,
    STR,
    DOUBLE,
    INT32,
    INT64,
    TIMESTAMP,
}

static EVENT: phf::Map<&'static str, (&'static str, &'static [(&'static str, Type)])> = phf_map! {
    "1:1" => ("NEB::Acknowledgement", &[("type", Type::SHORT),
                                        ("author", Type::STR),
                                        ("comment_data", Type::STR),
                                        ("deletion_time", Type::TIMESTAMP),
                                        ("entry_time", Type::TIMESTAMP),
                                        ("host_id", Type::INT32),
                                        ("instance_id", Type::INT32),
                                        ("sticky", Type::BOOL),
                                        ("notify_contacts", Type::BOOL),
                                        ("persistent_comment", Type::BOOL),
                                        ("service_id", Type::INT32),
                                        ("state", Type::SHORT),
                                        ]),
    "1:2" => ("NEB::Comment", &[("author", Type::STR),
                                ("type", Type::SHORT),
                                ("data", Type::STR),
                                ("deletion_time", Type::TIMESTAMP),
                                ("entry_time", Type::TIMESTAMP),
                                ("entry_type", Type::SHORT),
                                ("expires", Type::BOOL),
                                ("host_id", Type::INT32),
                                ("internal_id", Type::INT32),
                                ("persistent", Type::BOOL),
                                ("instance_id", Type::INT32),
                                ("service_id", Type::INT32),
                                ("source", Type::SHORT),
                                ]),
    "1:3" => ("NEB::CustomVariable", &[("enabled", Type::BOOL),
                                       ("host_id", Type::INT32),
                                       ("modified", Type::BOOL),
                                       ("name", Type::STR),
                                       ("service_id", Type::INT32),
                                       ("update_time", Type::TIMESTAMP),
                                       ("type", Type::SHORT),
                                       ("value", Type::STR),
                                       ("default_value", Type::STR),
                                       ]),
    "1:4" => ("NEB::CustomVariableStatus", &[("host_id", Type::INT32),
                                       ("modified", Type::BOOL),
                                       ("name", Type::STR),
                                       ("service_id", Type::INT32),
                                       ("update_time", Type::TIMESTAMP),
                                       ("value", Type::STR),
                                       ]),
    "1:8" => ("NEB::HostCheck", &[("active_checks_enabled", Type::BOOL),
                                  ("check_type", Type::SHORT),
                                  ("host_id", Type::INT32),
                                  ("next_check", Type::TIMESTAMP),
                                  ("command_line", Type::STR),
                                  ]),
    "1:19" => ("NEB::ServiceCheck", &[("active_checks_enabled", Type::BOOL),
                                  ("check_type", Type::SHORT),
                                  ("host_id", Type::INT32),
                                  ("next_check", Type::TIMESTAMP),
                                  ("service_id", Type::INT32),
                                  ("command_line", Type::STR),
                                  ]),
};

pub struct Event<'a> {
    buffer: &'a [u8],
    offset: usize,
}

impl Event<'_> {
    pub fn new(buffer : &[u8]) -> Event {
        Event { buffer: buffer, offset: 0}
    }

    fn get_bool(&mut self) -> bool {
        let size = 1;
        let v = u8::from_be_bytes(
            self.buffer[self.offset..(self.offset + size)]
                .try_into()
                .expect("slice with incorrect length"),
        );
        self.offset += size;
        match v {
            0 => false,
            1 => true,
            _ => {
            panic!("A boolean in bbdo format can only be 0 or 1"); }
        }
    }

    fn get_short(&mut self) -> i16 {
        let size = 2;
        let v = i16::from_be_bytes(
            self.buffer[self.offset..(self.offset + size)]
                .try_into()
                .expect("slice with incorrect length"),
        );
        self.offset += size;
        v
    }

    fn get_int32(&mut self) -> i32 {
        let size = 4;
        let v = i32::from_be_bytes(
            self.buffer[self.offset..(self.offset + size)]
                .try_into()
                .expect("slice with incorrect length"),
        );
        self.offset += size;
        v
    }

    fn get_int64(&mut self) -> i64 {
        let size = 8;
        let v = i64::from_be_bytes(
            self.buffer[self.offset..(self.offset + size)]
                .try_into()
                .expect("slice with incorrect length"),
        );
        self.offset += size;
        v
    }

    fn get_double(&mut self) -> f64 {
        let s = self.get_string();
        let v = s.parse::<f64>().unwrap();
        v
    }

    fn get_string(&mut self) -> &str {
        let mut i = self.offset;
        while self.buffer[i] != 0 {
            i += 1;
        }
        let v = match str::from_utf8(
            self.buffer[self.offset..i]
                .try_into()
                .expect("slice with incorrect length"),
        ) {
            Ok(v) => v,
            Err(e) => "Wrong UTF-8 string",
        };
        self.offset = i + 1;
        &v
    }

    fn compute_crc16(data: &[u8]) -> u16 {
        const CRC_TBL: [u16; 16] = [
            0x0000, 0x1081, 0x2102, 0x3183, 0x4204, 0x5285, 0x6306, 0x7387, 0x8408, 0x9489, 0xa50a,
            0xb58b, 0xc60c, 0xd68d, 0xe70e, 0xf78f,
        ];
        let mut crc: u16 = 0xffff;
        let mut data_len = 10;
        for c in data {
            let mut cc: u16 = *c as u16;
            crc = ((crc >> 4) & 0x0fff) ^ CRC_TBL[((crc ^ cc) & 15) as usize];
            cc >>= 4;
            crc = ((crc >> 4) & 0x0fff) ^ CRC_TBL[((crc ^ cc) & 15) as usize];
        }
        return !crc & 0xffff;
    }

    pub fn deserialize(&mut self) -> serde_json::Value {
            let offset = self.offset;
            let chksum = u16::from_be_bytes(
                self.buffer[self.offset..(self.offset + 2)]
                    .try_into()
                    .expect("slice with incorrect length"),
            );
            self.offset += 2;

            let size = u16::from_be_bytes(
                self.buffer[self.offset..(self.offset + 2)]
                    .try_into()
                    .expect("slice with incorrect length"),
            );
            self.offset += 2;

            let category = u16::from_be_bytes(
                self.buffer[self.offset..(self.offset + 2)]
                    .try_into()
                    .expect("slice with incorrect length"),
            );
            self.offset += 2;

            let element = u16::from_be_bytes(
                self.buffer[self.offset..(self.offset + 2)]
                    .try_into()
                    .expect("slice with incorrect length"),
            );
            self.offset += 2;

            let source_id = u32::from_be_bytes(
                self.buffer[self.offset..(self.offset + 4)]
                    .try_into()
                    .expect("slice with incorrect length"),
            );
            self.offset += 4;

            let dest_id = u32::from_be_bytes(
                self.buffer[self.offset..(self.offset + 4)]
                    .try_into()
                    .expect("slice with incorrect length"),
            );
            self.offset += 4;

            let old_offset = self.offset;
            let verif_chksum = Event::compute_crc16(&self.buffer[(offset + 2)..old_offset]);

            if verif_chksum != chksum {
                panic!("BBDO header unreadable");
            }

            let key = format!("{}:{}", category, element);

            let mut retval = json!([
                "unknown",
                {
                "chksum": chksum,
                "size": size,
                "category": category,
                "element": element,
                "source_id": source_id,
                "dest_id": dest_id,
            }]);

            if EVENT.contains_key(&key) {
                let d = EVENT[&key];
                retval[0] = d.0.into();
                let arr = d.1;
                for t in arr {
                    retval[1][t.0] = match t.1 {
                        Type::BOOL => self.get_bool().into(),
                        Type::SHORT => self.get_short().into(),
                        Type::STR => self.get_string().into(),
                        Type::DOUBLE => self.get_double().into(),
                        Type::INT32 => self.get_int32().into(),
                        Type::INT64 => self.get_int64().into(),
                        Type::TIMESTAMP => self.get_int64().into(),
                        _ => panic!("Should not arrive"),
                    }
                }
            }
            self.offset = old_offset + size as usize;
            retval
    }
}
