#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/cosmos/cosmos-rust/main/.images/cosmos.png"
)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![allow(
    rustdoc::bare_urls,
    rustdoc::broken_intra_doc_links,
    clippy::derive_partial_eq_without_eq
)]
#![forbid(unsafe_code)]
#![warn(trivial_casts, trivial_numeric_casts, unused_import_braces)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod traits;

pub use prost;
pub use tendermint_proto as tendermint;
pub use tendermint_proto::google::protobuf::{Any, Timestamp};

/// The version (commit hash) of the Cosmos SDK used when generating this library.
pub const COSMOS_SDK_VERSION: &str = include_str!("prost/cosmos-sdk/COSMOS_SDK_COMMIT");

/// Cosmos protobuf definitions.
pub mod cosmos {
    /// Authentication of accounts and transactions.
    pub mod auth {
        pub mod v1beta1 {
            include!("prost/cosmos-sdk/cosmos.auth.v1beta1.rs");
        }
    }

    /// Granting of arbitrary privileges from one account to another.
    pub mod authz {
        pub mod v1beta1 {
            include!("prost/cosmos-sdk/cosmos.authz.v1beta1.rs");
        }
    }

    /// Balances.
    pub mod bank {
        pub mod v1beta1 {
            include!("prost/cosmos-sdk/cosmos.bank.v1beta1.rs");
        }
    }

    /// Base functionality.
    pub mod base {
        /// Application BlockChain Interface (ABCI).
        ///
        /// Interface that defines the boundary between the replication engine
        /// (the blockchain), and the state machine (the application).
        pub mod abci {
            pub mod v1beta1 {
                include!("prost/cosmos-sdk/cosmos.base.abci.v1beta1.rs");
            }
        }

        /// Node requests.
        pub mod node {
            pub mod v1beta1 {
                include!("prost/cosmos-sdk/cosmos.base.node.v1beta1.rs");
            }
        }

        /// Query support.
        pub mod query {
            pub mod v1beta1 {
                include!("prost/cosmos-sdk/cosmos.base.query.v1beta1.rs");
            }
        }

        /// Reflection support.
        pub mod reflection {
            pub mod v1beta1 {
                include!("prost/cosmos-sdk/cosmos.base.reflection.v1beta1.rs");
            }

            pub mod v2alpha1 {
                include!("prost/cosmos-sdk/cosmos.base.reflection.v2alpha1.rs");
            }
        }

        /// Tendermint support.
        pub mod tendermint {
            pub mod v1beta1 {
                include!("prost/cosmos-sdk/cosmos.base.tendermint.v1beta1.rs");
            }
        }

        pub mod v1beta1 {
            include!("prost/cosmos-sdk/cosmos.base.v1beta1.rs");
        }
    }

    /// Benchmarking
    pub mod benchmark {
        pub mod v1 {
            include!("prost/cosmos-sdk/cosmos.benchmark.v1.rs");
        }
    }

    /// counter
    pub mod counter {
        pub mod v1 {
            include!("prost/cosmos-sdk/cosmos.counter.v1.rs");
        }
    }

    /// Crisis handling
    pub mod crisis {
        pub mod v1beta1 {
            include!("prost/cosmos-sdk/cosmos.crisis.v1beta1.rs");
        }
    }

    /// Cryptographic primitives.
    pub mod crypto {
        /// Multi-signature support.
        pub mod multisig {
            include!("prost/cosmos-sdk/cosmos.crypto.multisig.rs");
            pub mod v1beta1 {
                include!("prost/cosmos-sdk/cosmos.crypto.multisig.v1beta1.rs");
            }
        }
        pub mod ed25519 {
            include!("prost/cosmos-sdk/cosmos.crypto.ed25519.rs");
        }
        pub mod secp256k1 {
            include!("prost/cosmos-sdk/cosmos.crypto.secp256k1.rs");
        }
        pub mod secp256r1 {
            include!("prost/cosmos-sdk/cosmos.crypto.secp256r1.rs");
        }
    }

    /// Messages and services handling token distribution
    pub mod distribution {
        pub mod v1beta1 {
            include!("prost/cosmos-sdk/cosmos.distribution.v1beta1.rs");
        }
    }

    /// Messages and services handling evidence
    pub mod epochs {
        pub mod v1beta1 {
            include!("prost/cosmos-sdk/cosmos.epochs.v1beta1.rs");
        }
    }
    
    /// Messages and services handling evidence
    pub mod evidence {
        pub mod v1beta1 {
            include!("prost/cosmos-sdk/cosmos.evidence.v1beta1.rs");
        }
    }

    /// Allows accounts to grant fee allowances and to use fees from their accounts.
    pub mod feegrant {
        pub mod v1beta1 {
            include!("prost/cosmos-sdk/cosmos.feegrant.v1beta1.rs");
        }
    }

    /// Messages and services handling gentx's
    pub mod genutil {
        pub mod v1beta1 {
            include!("prost/cosmos-sdk/cosmos.genutil.v1beta1.rs");
        }
    }

    /// Messages and services handling governance
    pub mod gov {
        pub mod v1 {
            include!("prost/cosmos-sdk/cosmos.gov.v1.rs");
        }
        pub mod v1beta1 {
            include!("prost/cosmos-sdk/cosmos.gov.v1beta1.rs");
        }
    }

    /// Messages and services handling minting
    pub mod mint {
        pub mod v1beta1 {
            include!("prost/cosmos-sdk/cosmos.mint.v1beta1.rs");
        }
    }

    /// Messages and services handling chain parameters
    pub mod params {
        pub mod v1beta1 {
            include!("prost/cosmos-sdk/cosmos.params.v1beta1.rs");
        }
    }

    /// Messages and services handling chain parameters
    pub mod protocolpool {
        pub mod v1 {
            include!("prost/cosmos-sdk/cosmos.protocolpool.v1.rs");
        }
    }

    /// Handling slashing parameters and unjailing
    pub mod slashing {
        pub mod v1beta1 {
            include!("prost/cosmos-sdk/cosmos.slashing.v1beta1.rs");
        }
    }

    /// Proof-of-Stake layer for public blockchains.
    pub mod staking {
        pub mod v1beta1 {
            include!("prost/cosmos-sdk/cosmos.staking.v1beta1.rs");
        }
    }

    /// Transactions.
    pub mod tx {
        /// Transaction signing support.
        pub mod signing {
            pub mod v1beta1 {
                include!("prost/cosmos-sdk/cosmos.tx.signing.v1beta1.rs");
            }
        }

        pub mod v1beta1 {
            include!("prost/cosmos-sdk/cosmos.tx.v1beta1.rs");
        }
    }

    /// Services for the upgrade module.
    pub mod upgrade {
        pub mod v1beta1 {
            include!("prost/cosmos-sdk/cosmos.upgrade.v1beta1.rs");
        }
    }

    /// Services and tx's for the vesting module.
    pub mod vesting {
        pub mod v1beta1 {
            include!("prost/cosmos-sdk/cosmos.vesting.v1beta1.rs");
        }
    }
}

/// CosmWasm protobuf definitions.
#[cfg(feature = "cosmwasm")]
pub mod cosmwasm {
    /// Messages and services handling CosmWasm.
    pub mod wasm {
        pub mod v1 {
            include!("prost/wasmd/cosmwasm.wasm.v1.rs");
        }
    }
}
