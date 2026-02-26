use std::io;

pub(crate) trait PositionExtension {
    fn position(&mut self) -> u64;
}

impl<T: std::io::Seek> PositionExtension for T {
    fn position(&mut self) -> u64 {
        self.stream_position().expect("Unable to get current steam position")
    }
}

pub trait Serializable {
    type Version;

    // deserialize from binary format into rust struct.
    // Note: _version is passed as there's some differences in
    // behavior depending on which version of hermes is used.
    fn deserialize<R>(r: &mut R, _version: u32) -> Self
    where
        R: io::Read + io::BufRead + io::Seek;

    fn serialize<W>(&self, w: &mut W)
    where
        W: std::io::Write + std::io::Seek;

    fn size(&self) -> usize;
}