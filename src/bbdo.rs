use crate::event::Event;
use phf::phf_map;
use std::io::Read;
use std::{fs, str, vec};
use miniz_oxide::inflate::decompress_to_vec_zlib;

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

/// Bbdo object obtained from a file.
///
/// It is composed of `buffer` that is a vector of `u8` containing the full
/// content of the file used to construct it and of `offset` which is an index
/// to point to the current position in `buffer`.
///
pub struct Bbdo {
    buffer: std::vec::Vec<u8>,
    pub offset: usize,
}

impl Bbdo {
    /// Constructor of a Bbdo object from the file `filename`.
    ///
    /// Returns a `Bbdo` object.
    pub fn new(filename: &str) -> Bbdo {
        let mut f = fs::File::open(&filename).expect("no file found");
        let metadata = fs::metadata(&filename).expect("unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer).expect("buffer overflow");
        Bbdo {
            buffer: buffer,
            offset: 8,
        }
    }

    /// Gets the length of the Bbdo object.
    ///
    /// Returns the length of the Bbdo object.
    ///
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    fn get_bool(&mut self) -> bool {
        let size = 1;
        let v = u8::from_be_bytes(
            self.buffer[self.offset..(self.offset + size)]
                .try_into()
                .expect("slice with incorrect length"),
        );
        self.offset += size;
        if v == 1 {
            return true;
        } else if v == 0 {
            return false;
        } else {
            panic!("A boolean in bbdo format can only be 0 or 1");
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
        return v;
    }

    fn get_int32(&mut self) -> i32 {
        let size = 4;
        let v = i32::from_be_bytes(
            self.buffer[self.offset..(self.offset + size)]
                .try_into()
                .expect("slice with incorrect length"),
        );
        self.offset += size;
        return v;
    }

    fn get_int64(&mut self) -> i64 {
        let size = 8;
        let v = i64::from_be_bytes(
            self.buffer[self.offset..(self.offset + size)]
                .try_into()
                .expect("slice with incorrect length"),
        );
        self.offset += size;
        return v;
    }

    fn get_double(&mut self) -> f64 {
        let s = self.get_string();
        let v = s.parse::<f64>().unwrap();
        return v;
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
        return &v;
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

    pub fn is_compressed(&mut self) -> bool {
        let offset = self.offset;
        if self.offset >= self.len() {
            return false;
        }
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
        let verif_chksum = Bbdo::compute_crc16(&self.buffer[(offset + 2)..old_offset]);

        self.offset = offset;
        verif_chksum != chksum
    }

    pub fn deserialize(&mut self, compressed: &bool) -> serde_json::Value {
        if *compressed {
            let size = u32::from_be_bytes(
                self.buffer[self.offset..(self.offset + 4)]
                    .try_into()
                    .expect("slice with incorrect length"),
            );
            let d = decompress_to_vec_zlib(
                self.buffer[(self.offset + 8)..(self.offset + 8 + size as usize)]
                    .try_into()
                    .expect("slice with incorrect length"),
            )
            .unwrap();
            let mut event = Event::new(&d);
            let retval = event.deserialize();
            self.offset += 4 + size as usize;
            return retval;
        } else {
            let size = u16::from_be_bytes(
                self.buffer[(self.offset + 2)..(self.offset + 4)]
                    .try_into()
                    .expect("slice with incorrect length"),
            );
            let mut event = Event::new(&self.buffer[self.offset..(self.offset + 16 + size as usize)]);
            let retval = event.deserialize();
            self.offset += 16 + size as usize;
            return retval;
        }
    }
}
