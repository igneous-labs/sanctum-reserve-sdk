#![macro_use]

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
