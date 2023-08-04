pub mod traits;
mod type_urls;

pub use prost;
pub use prost_types::Any;

/// Cosmos protobuf definitions.
pub mod cosmos {
    /// Base functionality.
    pub mod base {
        pub mod v1beta1 {
            include!("prost/cosmos-sdk/cosmos.base.v1beta1.rs");
        }
    }
}

/// IBC protobuf definitions.
pub mod ibc {
    /// IBC applications.
    pub mod applications {
        /// Transfer support.
        pub mod transfer {
            pub mod v1 {
                include!("prost/ibc-go/ibc.applications.transfer.v1.rs");
            }
        }
    }

    /// IBC core.
    pub mod core {
        /// IBC client.
        pub mod client {
            pub mod v1 {
                include!("prost/ibc-go/ibc.core.client.v1.rs");
            }
        }
    }
}
