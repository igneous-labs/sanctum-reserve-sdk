#![macro_use]

use borsh::{BorshDeserialize, BorshSerialize};

macro_rules! inherent_borsh_ser {
    () => {
        /// Expose borsh serialization as inherent method
        /// so that it can still be used
        /// even with conflicting versions of borsh in downstream projects.
        ///
        /// And also pass writer by value just to be more inline with rust standards
        pub fn borsh_ser<W: borsh::io::Write>(&self, mut writer: W) -> borsh::io::Result<()> {
            <Self as borsh::BorshSerialize>::serialize(self, &mut writer)
        }
    };
}

macro_rules! inherent_borsh_de {
    () => {
        /// Expose borsh deserialization as inherent method
        /// so that it can still be used
        /// even with conflicting versions of borsh in downstream projects.
        ///
        /// And also pass reader by value just to be more inline with rust standards
        pub fn borsh_de<R: borsh::io::Read>(mut reader: R) -> borsh::io::Result<Self> {
            <Self as borsh::BorshDeserialize>::deserialize_reader(&mut reader)
        }
    };
}

macro_rules! inherent_borsh_serde {
    () => {
        inherent_borsh_ser!();
        inherent_borsh_de!();
    };
}

// TODO: idk why `pub(crate) use` is not necessary for the `inherent_borsh_*` macros above,
// probably due to module depth > 1

pub trait AnchorAccount: borsh::BorshDeserialize + borsh::BorshSerialize {
    const DISCM: [u8; 8];

    fn ser<W: borsh::io::Write>(&self, mut writer: W) -> borsh::io::Result<()> {
        Self::DISCM.serialize(&mut writer)?;
        self.serialize(&mut writer)
    }

    fn de<R: borsh::io::Read>(mut reader: R) -> borsh::io::Result<Self> {
        let discm = <[u8; 8]>::deserialize_reader(&mut reader)?;
        if discm != Self::DISCM {
            return Err(borsh::io::Error::new(
                borsh::io::ErrorKind::InvalidData,
                "invalid anchor account discriminant",
            ));
        }
        Self::deserialize_reader(&mut reader)
    }
}

macro_rules! inherent_anchor_ser {
    () => {
        pub fn anchor_ser<W: borsh::io::Write>(&self, mut writer: W) -> borsh::io::Result<()> {
            <Self as AnchorAccount>::ser(self, &mut writer)
        }
    };
}

macro_rules! inherent_anchor_de {
    () => {
        pub fn anchor_de<R: borsh::io::Read>(mut reader: R) -> borsh::io::Result<Self> {
            <Self as AnchorAccount>::de(&mut reader)
        }
    };
}

macro_rules! inherent_anchor_serde {
    () => {
        inherent_anchor_ser!();
        inherent_anchor_de!();
    };
}
