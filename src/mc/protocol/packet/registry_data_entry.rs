use std::io;

use crate::error::KeisteenResult;
use crate::mc::nbt;
use crate::mc::protocol::packet::{PrefixedProtocolWrite, ProtocolWrite};
use crate::mc::types::Identifier;

#[derive(Debug)]
pub struct RegistryDataEntry {
    pub entry_id: Identifier,
    pub data: Option<nbt::NbtTag>,
}

impl ProtocolWrite for RegistryDataEntry {
    fn write<W: io::Write>(&self, writer: &mut W) -> KeisteenResult<()> {
        self.entry_id.write(writer)?;
        self.data.write_prefixed(writer)?;
        Ok(())
    }
}
