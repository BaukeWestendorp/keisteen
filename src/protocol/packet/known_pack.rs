use std::io;

use crate::error::KeisteenResult;
use crate::protocol::packet::{ProtocolRead, ProtocolWrite};

#[derive(Debug)]
pub struct KnownPack {
    pub namespace: String,
    pub id: String,
    pub version: String,
}

impl ProtocolRead for KnownPack {
    fn read_from<R: io::Read>(reader: &mut R) -> KeisteenResult<Self> {
        Ok(Self {
            namespace: String::read_from(reader)?,
            id: String::read_from(reader)?,
            version: String::read_from(reader)?,
        })
    }
}

impl ProtocolWrite for KnownPack {
    fn write<W: io::Write>(&self, writer: &mut W) -> KeisteenResult<()> {
        self.namespace.write(writer)?;
        self.id.write(writer)?;
        self.version.write(writer)?;
        Ok(())
    }
}
