use protocol::{OutStream, Write};

pub struct ProtocolVersion {
    major: i16,
    minor: i16,
    maintaince: i16,
}

impl Write for ProtocolVersion {
    fn write(&self, out: &OutStream) {
        out.write_i16(self.major);
        out.write_i16(self.minor);
        out.write_i16(self.maintaince);
    }
}
