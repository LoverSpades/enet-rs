use enet_sys::{
    enet_packet_create, enet_packet_destroy, ENetPacket, _ENetPacketFlag_ENET_PACKET_FLAG_RELIABLE,
    _ENetPacketFlag_ENET_PACKET_FLAG_UNSEQUENCED,
};

use crate::EnetFailure;

/// A packet that can be sent or retrieved on an ENet-connection.
pub struct EnetPacket {
    inner: *mut ENetPacket,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
/// Mode that can be set when transmitting a packet.
///
/// ENet does not support reliable but unsequenced packets.
pub enum PacketMode {
    /// The packet will be sent unreliably but sequenced (ENet default).
    UnreliableSequenced,
    /// The packet will be sent unreliably and unsequenced.
    UnreliableUnsequenced,
    /// The packet will be sent reliably and sequenced with other reliable packets.
    ReliableSequenced,
}

impl PacketMode {
    /// Returns whether this represents a reliable mode.
    pub fn is_reliable(&self) -> bool {
        match self {
            PacketMode::UnreliableSequenced => false,
            PacketMode::UnreliableUnsequenced => false,
            PacketMode::ReliableSequenced => true,
        }
    }

    /// Returns whether this represents a sequenced mode.
    pub fn is_sequenced(&self) -> bool {
        match self {
            PacketMode::UnreliableSequenced => true,
            PacketMode::UnreliableUnsequenced => false,
            PacketMode::ReliableSequenced => true,
        }
    }

    fn to_sys_flags(&self) -> u32 {
        match self {
            PacketMode::UnreliableSequenced => 0,
            PacketMode::UnreliableUnsequenced => _ENetPacketFlag_ENET_PACKET_FLAG_UNSEQUENCED,
            PacketMode::ReliableSequenced => _ENetPacketFlag_ENET_PACKET_FLAG_RELIABLE,
        }
    }
}

impl EnetPacket {
    pub fn new(data: &[u8], mode: PacketMode) -> Result<EnetPacket, EnetFailure> {
        let res = unsafe {
            enet_packet_create(data.as_ptr() as *const _, data.len(), mode.to_sys_flags())
        };

        if res.is_null() {
            return Err(EnetFailure(0));
        }

        Ok(EnetPacket::from_sys_packet(res))
    }

    pub(crate) fn from_sys_packet(inner: *mut ENetPacket) -> EnetPacket {
        EnetPacket { inner }
    }

    pub(crate) fn into_inner(self) -> *mut ENetPacket {
        self.inner
    }

    /// Returns a reference to the bytes inside this packet.
    pub fn data<'a>(&'a self) -> &'a [u8] {
        unsafe { std::slice::from_raw_parts((*self.inner).data, (*self.inner).dataLength) }
    }
}

impl Drop for EnetPacket {
    fn drop(&mut self) {
        unsafe {
            enet_packet_destroy(self.inner);
        }
    }
}
