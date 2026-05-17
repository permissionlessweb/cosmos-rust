use super::AccessConfig;
use crate::{proto, tx::Msg, AccountId, Error, ErrorReport, Result};
use tendermint::Hash;

/// MsgStoreCode submit Wasm code to the system
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgStoreCode {
    /// Sender is the that actor that signed the messages
    pub sender: AccountId,

    /// WASMByteCode can be raw or gzip compressed
    pub wasm_byte_code: Vec<u8>,

    /// InstantiatePermission access control to apply on contract creation,
    /// optional
    pub instantiate_permission: Option<AccessConfig>,
}

impl Msg for MsgStoreCode {
    type Proto = proto::cosmwasm::wasm::v1::MsgStoreCode;
}

impl TryFrom<proto::cosmwasm::wasm::v1::MsgStoreCode> for MsgStoreCode {
    type Error = ErrorReport;

    fn try_from(proto: proto::cosmwasm::wasm::v1::MsgStoreCode) -> Result<MsgStoreCode> {
        Ok(MsgStoreCode {
            sender: proto.sender.parse()?,
            wasm_byte_code: proto.wasm_byte_code,
            instantiate_permission: proto
                .instantiate_permission
                .map(TryFrom::try_from)
                .transpose()?,
        })
    }
}

impl From<MsgStoreCode> for proto::cosmwasm::wasm::v1::MsgStoreCode {
    fn from(msg: MsgStoreCode) -> proto::cosmwasm::wasm::v1::MsgStoreCode {
        proto::cosmwasm::wasm::v1::MsgStoreCode {
            sender: msg.sender.to_string(),
            wasm_byte_code: msg.wasm_byte_code,
            instantiate_permission: msg.instantiate_permission.map(Into::into),
        }
    }
}

/// MsgStoreCodeResponse returns store result data.
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgStoreCodeResponse {
    /// CodeID is the reference to the stored WASM code
    pub code_id: u64,

    /// Checksum is the sha256 hash of the stored code
    pub checksum: Hash,
}

impl Msg for MsgStoreCodeResponse {
    type Proto = proto::cosmwasm::wasm::v1::MsgStoreCodeResponse;
}

impl TryFrom<proto::cosmwasm::wasm::v1::MsgStoreCodeResponse> for MsgStoreCodeResponse {
    type Error = ErrorReport;

    fn try_from(
        proto: proto::cosmwasm::wasm::v1::MsgStoreCodeResponse,
    ) -> Result<MsgStoreCodeResponse> {
        Ok(MsgStoreCodeResponse {
            code_id: proto.code_id,
            checksum: Hash::Sha256(proto.checksum.try_into().map_err(|_| Error::Crypto)?),
        })
    }
}

impl From<MsgStoreCodeResponse> for proto::cosmwasm::wasm::v1::MsgStoreCodeResponse {
    fn from(msg: MsgStoreCodeResponse) -> proto::cosmwasm::wasm::v1::MsgStoreCodeResponse {
        proto::cosmwasm::wasm::v1::MsgStoreCodeResponse {
            code_id: msg.code_id,
            checksum: msg.checksum.as_bytes().into(),
        }
    }
}

/// MsgStoreCode submit Wasm code to the system
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgStoreCircuit {
    /// Sender is the that actor that signed the messages
    pub sender: AccountId,

    /// circuit_binary
    pub circuit_binary: Vec<u8>,

    /// InstantiatePermission access control to apply on contract creation,
    /// optional
    pub instantiate_permission: Option<AccessConfig>,
}

impl Msg for MsgStoreCircuit {
    type Proto = proto::cosmwasm::wasm::v1::MsgStoreCircuit;
}

impl TryFrom<proto::cosmwasm::wasm::v1::MsgStoreCircuit> for MsgStoreCircuit {
    type Error = ErrorReport;

    fn try_from(proto: proto::cosmwasm::wasm::v1::MsgStoreCircuit) -> Result<MsgStoreCircuit> {
        Ok(MsgStoreCircuit {
            sender: proto.sender.parse()?,
            circuit_binary: proto.circuit_binary,
            instantiate_permission: proto
                .instantiate_permission
                .map(TryFrom::try_from)
                .transpose()?,
        })
    }
}

impl From<MsgStoreCircuit> for proto::cosmwasm::wasm::v1::MsgStoreCircuit {
    fn from(msg: MsgStoreCircuit) -> proto::cosmwasm::wasm::v1::MsgStoreCircuit {
        proto::cosmwasm::wasm::v1::MsgStoreCircuit {
            sender: msg.sender.to_string(),
            circuit_binary: msg.circuit_binary,
            instantiate_permission: msg.instantiate_permission.map(Into::into),
        }
    }
}

/// MsgStoreCircuitResponse returns store result data.
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgStoreCircuitResponse {
    /// CodeID is the reference to the stored WASM code
    pub zk_id: u64,

    /// Checksum is the sha256 hash of the stored code
    pub checksum: Hash,
}

impl Msg for MsgStoreCircuitResponse {
    type Proto = proto::cosmwasm::wasm::v1::MsgStoreCircuitResponse;
}

impl TryFrom<proto::cosmwasm::wasm::v1::MsgStoreCircuitResponse> for MsgStoreCircuitResponse {
    type Error = ErrorReport;

    fn try_from(
        proto: proto::cosmwasm::wasm::v1::MsgStoreCircuitResponse,
    ) -> Result<MsgStoreCircuitResponse> {
        Ok(MsgStoreCircuitResponse {
            zk_id: proto.zk_id,
            checksum: Hash::Sha256(proto.checksum.try_into().map_err(|_| Error::Crypto)?),
        })
    }
}

impl From<MsgStoreCircuitResponse> for proto::cosmwasm::wasm::v1::MsgStoreCircuitResponse {
    fn from(msg: MsgStoreCircuitResponse) -> proto::cosmwasm::wasm::v1::MsgStoreCircuitResponse {
        proto::cosmwasm::wasm::v1::MsgStoreCircuitResponse {
            zk_id: msg.zk_id,
            checksum: msg.checksum.as_bytes().into(),
        }
    }
}
