use crate::event::Event;
use miniz_oxide::inflate::decompress_to_vec_zlib;
use std::io::Read;
use std::{fs, str, vec};

/// Bbdo object obtained from a file.
///
/// It is composed of `buffer` that is a vector of `u8` containing the full
/// content of the file used to construct it and of `offset` which is an index
/// to point to the current position in `buffer`.
///
pub struct Bbdo<'a> {
    buffer: std::vec::Vec<u8>,
    pub offset: usize,
    output: &'a str,
}

impl Bbdo<'_> {
    /// Constructor of a Bbdo object from the file `filename`.
    ///
    /// Returns a `Bbdo` object.
    pub fn new<'a>(filename: &'a str, output: &'a str) -> Bbdo<'a> {
        let mut f = fs::File::open(&filename).expect("no file found");
        let metadata = fs::metadata(&filename).expect("unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer).expect("buffer overflow");
        Bbdo {
            buffer: buffer,
            offset: 8,
            output: output,
        }
    }

    /// Gets the length of the Bbdo object.
    ///
    /// Returns the length of the Bbdo object.
    ///
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    fn compute_crc16(data: &[u8]) -> u16 {
        const CRC_TBL: [u16; 16] = [
            0x0000, 0x1081, 0x2102, 0x3183, 0x4204, 0x5285, 0x6306, 0x7387, 0x8408, 0x9489, 0xa50a,
            0xb58b, 0xc60c, 0xd68d, 0xe70e, 0xf78f,
        ];
        let mut crc: u16 = 0xffff;
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
        let verif_chksum = Bbdo::compute_crc16(&self.buffer[(offset + 2)..(offset + 16)]);
        verif_chksum != chksum
    }

    pub fn deserialize(
        &mut self,
        compressed: &bool,
        filter_event: &i32,
        deprecated: &bool,
    ) -> Result<serde_json::Value, &'static str> {
        if *compressed {
            let size = u32::from_be_bytes(
                self.buffer[self.offset..(self.offset + 4)]
                    .try_into()
                    .expect("slice with incorrect length"),
            );
            let d = decompress_to_vec_zlib(
                self.buffer[(self.offset + 8)..(self.offset + 4 + size as usize)]
                    .try_into()
                    .expect("slice with incorrect length"),
            )
            .unwrap();
            let mut event = Event::new(&d, *deprecated);
            let retval = event.deserialize(filter_event);
            self.offset += 4 + size as usize;
            return retval;
        } else {
            let size = u16::from_be_bytes(
                self.buffer[(self.offset + 2)..(self.offset + 4)]
                    .try_into()
                    .expect("slice with incorrect length"),
            );
            let mut event = Event::new(
                &self.buffer[self.offset..(self.offset + 16 + size as usize)],
                *deprecated,
            );
            let retval = event.deserialize(filter_event);
            self.offset += 16 + size as usize;
            return retval;
        }
    }
}
