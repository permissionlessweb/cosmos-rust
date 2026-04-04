// @generated
#[cfg(feature = "serde")]
impl serde::Serialize for AuthenticateResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.verified {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("xion.dkim.v1.AuthenticateResponse", len)?;
        if self.verified {
            struct_ser.serialize_field("verified", &self.verified)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for AuthenticateResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["verified"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Verified,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "verified" => Ok(GeneratedField::Verified),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AuthenticateResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.dkim.v1.AuthenticateResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<AuthenticateResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut verified__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Verified => {
                            if verified__.is_some() {
                                return Err(serde::de::Error::duplicate_field("verified"));
                            }
                            verified__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AuthenticateResponse {
                    verified: verified__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "xion.dkim.v1.AuthenticateResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for DkimPubKey {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.domain.is_empty() {
            len += 1;
        }
        if !self.pub_key.is_empty() {
            len += 1;
        }
        if !self.poseidon_hash.is_empty() {
            len += 1;
        }
        if !self.selector.is_empty() {
            len += 1;
        }
        if self.version != 0 {
            len += 1;
        }
        if self.key_type != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("xion.dkim.v1.DkimPubKey", len)?;
        if !self.domain.is_empty() {
            struct_ser.serialize_field("domain", &self.domain)?;
        }
        if !self.pub_key.is_empty() {
            struct_ser.serialize_field("pubKey", &self.pub_key)?;
        }
        if !self.poseidon_hash.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "poseidonHash",
                pbjson::private::base64::encode(&self.poseidon_hash).as_str(),
            )?;
        }
        if !self.selector.is_empty() {
            struct_ser.serialize_field("selector", &self.selector)?;
        }
        if self.version != 0 {
            let v = Version::try_from(self.version).map_err(|_| {
                serde::ser::Error::custom(alloc::format!("Invalid variant {}", self.version))
            })?;
            struct_ser.serialize_field("version", &v)?;
        }
        if self.key_type != 0 {
            let v = KeyType::try_from(self.key_type).map_err(|_| {
                serde::ser::Error::custom(alloc::format!("Invalid variant {}", self.key_type))
            })?;
            struct_ser.serialize_field("keyType", &v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for DkimPubKey {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "domain",
            "pub_key",
            "pubKey",
            "poseidon_hash",
            "poseidonHash",
            "selector",
            "version",
            "key_type",
            "keyType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Domain,
            PubKey,
            PoseidonHash,
            Selector,
            Version,
            KeyType,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "domain" => Ok(GeneratedField::Domain),
                            "pubKey" | "pub_key" => Ok(GeneratedField::PubKey),
                            "poseidonHash" | "poseidon_hash" => Ok(GeneratedField::PoseidonHash),
                            "selector" => Ok(GeneratedField::Selector),
                            "version" => Ok(GeneratedField::Version),
                            "keyType" | "key_type" => Ok(GeneratedField::KeyType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DkimPubKey;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.dkim.v1.DkimPubKey")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<DkimPubKey, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut domain__ = None;
                let mut pub_key__ = None;
                let mut poseidon_hash__ = None;
                let mut selector__ = None;
                let mut version__ = None;
                let mut key_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Domain => {
                            if domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domain"));
                            }
                            domain__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PubKey => {
                            if pub_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pubKey"));
                            }
                            pub_key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PoseidonHash => {
                            if poseidon_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("poseidonHash"));
                            }
                            poseidon_hash__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Selector => {
                            if selector__.is_some() {
                                return Err(serde::de::Error::duplicate_field("selector"));
                            }
                            selector__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = Some(map_.next_value::<Version>()? as i32);
                        }
                        GeneratedField::KeyType => {
                            if key_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyType"));
                            }
                            key_type__ = Some(map_.next_value::<KeyType>()? as i32);
                        }
                    }
                }
                Ok(DkimPubKey {
                    domain: domain__.unwrap_or_default(),
                    pub_key: pub_key__.unwrap_or_default(),
                    poseidon_hash: poseidon_hash__.unwrap_or_default(),
                    selector: selector__.unwrap_or_default(),
                    version: version__.unwrap_or_default(),
                    key_type: key_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("xion.dkim.v1.DkimPubKey", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for GenesisState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.params.is_some() {
            len += 1;
        }
        if !self.dkim_pubkeys.is_empty() {
            len += 1;
        }
        if !self.revoked_pubkeys.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("xion.dkim.v1.GenesisState", len)?;
        if let Some(v) = self.params.as_ref() {
            struct_ser.serialize_field("params", v)?;
        }
        if !self.dkim_pubkeys.is_empty() {
            struct_ser.serialize_field("dkimPubkeys", &self.dkim_pubkeys)?;
        }
        if !self.revoked_pubkeys.is_empty() {
            struct_ser.serialize_field("revokedPubkeys", &self.revoked_pubkeys)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for GenesisState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "params",
            "dkim_pubkeys",
            "dkimPubkeys",
            "revoked_pubkeys",
            "revokedPubkeys",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Params,
            DkimPubkeys,
            RevokedPubkeys,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "params" => Ok(GeneratedField::Params),
                            "dkimPubkeys" | "dkim_pubkeys" => Ok(GeneratedField::DkimPubkeys),
                            "revokedPubkeys" | "revoked_pubkeys" => {
                                Ok(GeneratedField::RevokedPubkeys)
                            }
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GenesisState;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.dkim.v1.GenesisState")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<GenesisState, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut params__ = None;
                let mut dkim_pubkeys__ = None;
                let mut revoked_pubkeys__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = map_.next_value()?;
                        }
                        GeneratedField::DkimPubkeys => {
                            if dkim_pubkeys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dkimPubkeys"));
                            }
                            dkim_pubkeys__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RevokedPubkeys => {
                            if revoked_pubkeys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("revokedPubkeys"));
                            }
                            revoked_pubkeys__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GenesisState {
                    params: params__,
                    dkim_pubkeys: dkim_pubkeys__.unwrap_or_default(),
                    revoked_pubkeys: revoked_pubkeys__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("xion.dkim.v1.GenesisState", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for IndexRange {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.start != 0 {
            len += 1;
        }
        if self.end != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("xion.dkim.v1.IndexRange", len)?;
        if self.start != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "start",
                alloc::string::ToString::to_string(&self.start).as_str(),
            )?;
        }
        if self.end != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "end",
                alloc::string::ToString::to_string(&self.end).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for IndexRange {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["start", "end"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Start,
            End,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "start" => Ok(GeneratedField::Start),
                            "end" => Ok(GeneratedField::End),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IndexRange;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.dkim.v1.IndexRange")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<IndexRange, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut start__ = None;
                let mut end__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Start => {
                            if start__.is_some() {
                                return Err(serde::de::Error::duplicate_field("start"));
                            }
                            start__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::End => {
                            if end__.is_some() {
                                return Err(serde::de::Error::duplicate_field("end"));
                            }
                            end__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(IndexRange {
                    start: start__.unwrap_or_default(),
                    end: end__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("xion.dkim.v1.IndexRange", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for KeyType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::RsaUnspecified => "KEY_TYPE_RSA_UNSPECIFIED",
        };
        serializer.serialize_str(variant)
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for KeyType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["KEY_TYPE_RSA_UNSPECIFIED"];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = KeyType;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "KEY_TYPE_RSA_UNSPECIFIED" => Ok(KeyType::RsaUnspecified),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgAddDkimPubKeys {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.authority.is_empty() {
            len += 1;
        }
        if !self.dkim_pubkeys.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("xion.dkim.v1.MsgAddDkimPubKeys", len)?;
        if !self.authority.is_empty() {
            struct_ser.serialize_field("authority", &self.authority)?;
        }
        if !self.dkim_pubkeys.is_empty() {
            struct_ser.serialize_field("dkimPubkeys", &self.dkim_pubkeys)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgAddDkimPubKeys {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["authority", "dkim_pubkeys", "dkimPubkeys"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Authority,
            DkimPubkeys,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "authority" => Ok(GeneratedField::Authority),
                            "dkimPubkeys" | "dkim_pubkeys" => Ok(GeneratedField::DkimPubkeys),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgAddDkimPubKeys;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.dkim.v1.MsgAddDkimPubKeys")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgAddDkimPubKeys, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut authority__ = None;
                let mut dkim_pubkeys__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Authority => {
                            if authority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            authority__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DkimPubkeys => {
                            if dkim_pubkeys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dkimPubkeys"));
                            }
                            dkim_pubkeys__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgAddDkimPubKeys {
                    authority: authority__.unwrap_or_default(),
                    dkim_pubkeys: dkim_pubkeys__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("xion.dkim.v1.MsgAddDkimPubKeys", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgAddDkimPubKeysResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("xion.dkim.v1.MsgAddDkimPubKeysResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgAddDkimPubKeysResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgAddDkimPubKeysResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.dkim.v1.MsgAddDkimPubKeysResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<MsgAddDkimPubKeysResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgAddDkimPubKeysResponse {})
            }
        }
        deserializer.deserialize_struct(
            "xion.dkim.v1.MsgAddDkimPubKeysResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgRemoveDkimPubKey {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.authority.is_empty() {
            len += 1;
        }
        if !self.selector.is_empty() {
            len += 1;
        }
        if !self.domain.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("xion.dkim.v1.MsgRemoveDkimPubKey", len)?;
        if !self.authority.is_empty() {
            struct_ser.serialize_field("authority", &self.authority)?;
        }
        if !self.selector.is_empty() {
            struct_ser.serialize_field("selector", &self.selector)?;
        }
        if !self.domain.is_empty() {
            struct_ser.serialize_field("domain", &self.domain)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgRemoveDkimPubKey {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["authority", "selector", "domain"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Authority,
            Selector,
            Domain,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "authority" => Ok(GeneratedField::Authority),
                            "selector" => Ok(GeneratedField::Selector),
                            "domain" => Ok(GeneratedField::Domain),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgRemoveDkimPubKey;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.dkim.v1.MsgRemoveDkimPubKey")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<MsgRemoveDkimPubKey, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut authority__ = None;
                let mut selector__ = None;
                let mut domain__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Authority => {
                            if authority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            authority__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Selector => {
                            if selector__.is_some() {
                                return Err(serde::de::Error::duplicate_field("selector"));
                            }
                            selector__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Domain => {
                            if domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domain"));
                            }
                            domain__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgRemoveDkimPubKey {
                    authority: authority__.unwrap_or_default(),
                    selector: selector__.unwrap_or_default(),
                    domain: domain__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "xion.dkim.v1.MsgRemoveDkimPubKey",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgRemoveDkimPubKeyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("xion.dkim.v1.MsgRemoveDkimPubKeyResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgRemoveDkimPubKeyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgRemoveDkimPubKeyResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.dkim.v1.MsgRemoveDkimPubKeyResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<MsgRemoveDkimPubKeyResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgRemoveDkimPubKeyResponse {})
            }
        }
        deserializer.deserialize_struct(
            "xion.dkim.v1.MsgRemoveDkimPubKeyResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgRevokeDkimPubKey {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.signer.is_empty() {
            len += 1;
        }
        if !self.domain.is_empty() {
            len += 1;
        }
        if !self.priv_key.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("xion.dkim.v1.MsgRevokeDkimPubKey", len)?;
        if !self.signer.is_empty() {
            struct_ser.serialize_field("signer", &self.signer)?;
        }
        if !self.domain.is_empty() {
            struct_ser.serialize_field("domain", &self.domain)?;
        }
        if !self.priv_key.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "privKey",
                pbjson::private::base64::encode(&self.priv_key).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgRevokeDkimPubKey {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["signer", "domain", "priv_key", "privKey"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Signer,
            Domain,
            PrivKey,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "signer" => Ok(GeneratedField::Signer),
                            "domain" => Ok(GeneratedField::Domain),
                            "privKey" | "priv_key" => Ok(GeneratedField::PrivKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgRevokeDkimPubKey;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.dkim.v1.MsgRevokeDkimPubKey")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<MsgRevokeDkimPubKey, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut signer__ = None;
                let mut domain__ = None;
                let mut priv_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Signer => {
                            if signer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signer"));
                            }
                            signer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Domain => {
                            if domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domain"));
                            }
                            domain__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PrivKey => {
                            if priv_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("privKey"));
                            }
                            priv_key__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(MsgRevokeDkimPubKey {
                    signer: signer__.unwrap_or_default(),
                    domain: domain__.unwrap_or_default(),
                    priv_key: priv_key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "xion.dkim.v1.MsgRevokeDkimPubKey",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgRevokeDkimPubKeyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("xion.dkim.v1.MsgRevokeDkimPubKeyResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgRevokeDkimPubKeyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgRevokeDkimPubKeyResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.dkim.v1.MsgRevokeDkimPubKeyResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<MsgRevokeDkimPubKeyResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgRevokeDkimPubKeyResponse {})
            }
        }
        deserializer.deserialize_struct(
            "xion.dkim.v1.MsgRevokeDkimPubKeyResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgUpdateParams {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.authority.is_empty() {
            len += 1;
        }
        if self.params.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("xion.dkim.v1.MsgUpdateParams", len)?;
        if !self.authority.is_empty() {
            struct_ser.serialize_field("authority", &self.authority)?;
        }
        if let Some(v) = self.params.as_ref() {
            struct_ser.serialize_field("params", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgUpdateParams {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["authority", "params"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Authority,
            Params,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "authority" => Ok(GeneratedField::Authority),
                            "params" => Ok(GeneratedField::Params),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgUpdateParams;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.dkim.v1.MsgUpdateParams")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgUpdateParams, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut authority__ = None;
                let mut params__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Authority => {
                            if authority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            authority__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = map_.next_value()?;
                        }
                    }
                }
                Ok(MsgUpdateParams {
                    authority: authority__.unwrap_or_default(),
                    params: params__,
                })
            }
        }
        deserializer.deserialize_struct("xion.dkim.v1.MsgUpdateParams", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgUpdateParamsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("xion.dkim.v1.MsgUpdateParamsResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgUpdateParamsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgUpdateParamsResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.dkim.v1.MsgUpdateParamsResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<MsgUpdateParamsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUpdateParamsResponse {})
            }
        }
        deserializer.deserialize_struct(
            "xion.dkim.v1.MsgUpdateParamsResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for Params {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.vkey_identifier != 0 {
            len += 1;
        }
        if self.max_pubkey_size_bytes != 0 {
            len += 1;
        }
        if self.public_input_indices.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("xion.dkim.v1.Params", len)?;
        if self.vkey_identifier != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "vkeyIdentifier",
                alloc::string::ToString::to_string(&self.vkey_identifier).as_str(),
            )?;
        }
        if self.max_pubkey_size_bytes != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "maxPubkeySizeBytes",
                alloc::string::ToString::to_string(&self.max_pubkey_size_bytes).as_str(),
            )?;
        }
        if let Some(v) = self.public_input_indices.as_ref() {
            struct_ser.serialize_field("publicInputIndices", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Params {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "vkey_identifier",
            "vkeyIdentifier",
            "max_pubkey_size_bytes",
            "maxPubkeySizeBytes",
            "public_input_indices",
            "publicInputIndices",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            VkeyIdentifier,
            MaxPubkeySizeBytes,
            PublicInputIndices,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "vkeyIdentifier" | "vkey_identifier" => {
                                Ok(GeneratedField::VkeyIdentifier)
                            }
                            "maxPubkeySizeBytes" | "max_pubkey_size_bytes" => {
                                Ok(GeneratedField::MaxPubkeySizeBytes)
                            }
                            "publicInputIndices" | "public_input_indices" => {
                                Ok(GeneratedField::PublicInputIndices)
                            }
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Params;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.dkim.v1.Params")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<Params, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut vkey_identifier__ = None;
                let mut max_pubkey_size_bytes__ = None;
                let mut public_input_indices__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::VkeyIdentifier => {
                            if vkey_identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vkeyIdentifier"));
                            }
                            vkey_identifier__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::MaxPubkeySizeBytes => {
                            if max_pubkey_size_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "maxPubkeySizeBytes",
                                ));
                            }
                            max_pubkey_size_bytes__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::PublicInputIndices => {
                            if public_input_indices__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "publicInputIndices",
                                ));
                            }
                            public_input_indices__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Params {
                    vkey_identifier: vkey_identifier__.unwrap_or_default(),
                    max_pubkey_size_bytes: max_pubkey_size_bytes__.unwrap_or_default(),
                    public_input_indices: public_input_indices__,
                })
            }
        }
        deserializer.deserialize_struct("xion.dkim.v1.Params", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for PublicInputIndices {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.min_length != 0 {
            len += 1;
        }
        if self.email_hash_index != 0 {
            len += 1;
        }
        if self.dkim_domain_range.is_some() {
            len += 1;
        }
        if self.dkim_hash_index != 0 {
            len += 1;
        }
        if self.tx_bytes_range.is_some() {
            len += 1;
        }
        if self.email_host_range.is_some() {
            len += 1;
        }
        if self.email_subject_range.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("xion.dkim.v1.PublicInputIndices", len)?;
        if self.min_length != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "minLength",
                alloc::string::ToString::to_string(&self.min_length).as_str(),
            )?;
        }
        if self.email_hash_index != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "emailHashIndex",
                alloc::string::ToString::to_string(&self.email_hash_index).as_str(),
            )?;
        }
        if let Some(v) = self.dkim_domain_range.as_ref() {
            struct_ser.serialize_field("dkimDomainRange", v)?;
        }
        if self.dkim_hash_index != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "dkimHashIndex",
                alloc::string::ToString::to_string(&self.dkim_hash_index).as_str(),
            )?;
        }
        if let Some(v) = self.tx_bytes_range.as_ref() {
            struct_ser.serialize_field("txBytesRange", v)?;
        }
        if let Some(v) = self.email_host_range.as_ref() {
            struct_ser.serialize_field("emailHostRange", v)?;
        }
        if let Some(v) = self.email_subject_range.as_ref() {
            struct_ser.serialize_field("emailSubjectRange", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for PublicInputIndices {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "min_length",
            "minLength",
            "email_hash_index",
            "emailHashIndex",
            "dkim_domain_range",
            "dkimDomainRange",
            "dkim_hash_index",
            "dkimHashIndex",
            "tx_bytes_range",
            "txBytesRange",
            "email_host_range",
            "emailHostRange",
            "email_subject_range",
            "emailSubjectRange",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MinLength,
            EmailHashIndex,
            DkimDomainRange,
            DkimHashIndex,
            TxBytesRange,
            EmailHostRange,
            EmailSubjectRange,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "minLength" | "min_length" => Ok(GeneratedField::MinLength),
                            "emailHashIndex" | "email_hash_index" => {
                                Ok(GeneratedField::EmailHashIndex)
                            }
                            "dkimDomainRange" | "dkim_domain_range" => {
                                Ok(GeneratedField::DkimDomainRange)
                            }
                            "dkimHashIndex" | "dkim_hash_index" => {
                                Ok(GeneratedField::DkimHashIndex)
                            }
                            "txBytesRange" | "tx_bytes_range" => Ok(GeneratedField::TxBytesRange),
                            "emailHostRange" | "email_host_range" => {
                                Ok(GeneratedField::EmailHostRange)
                            }
                            "emailSubjectRange" | "email_subject_range" => {
                                Ok(GeneratedField::EmailSubjectRange)
                            }
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PublicInputIndices;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.dkim.v1.PublicInputIndices")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<PublicInputIndices, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut min_length__ = None;
                let mut email_hash_index__ = None;
                let mut dkim_domain_range__ = None;
                let mut dkim_hash_index__ = None;
                let mut tx_bytes_range__ = None;
                let mut email_host_range__ = None;
                let mut email_subject_range__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MinLength => {
                            if min_length__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minLength"));
                            }
                            min_length__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::EmailHashIndex => {
                            if email_hash_index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("emailHashIndex"));
                            }
                            email_hash_index__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::DkimDomainRange => {
                            if dkim_domain_range__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dkimDomainRange"));
                            }
                            dkim_domain_range__ = map_.next_value()?;
                        }
                        GeneratedField::DkimHashIndex => {
                            if dkim_hash_index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dkimHashIndex"));
                            }
                            dkim_hash_index__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::TxBytesRange => {
                            if tx_bytes_range__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txBytesRange"));
                            }
                            tx_bytes_range__ = map_.next_value()?;
                        }
                        GeneratedField::EmailHostRange => {
                            if email_host_range__.is_some() {
                                return Err(serde::de::Error::duplicate_field("emailHostRange"));
                            }
                            email_host_range__ = map_.next_value()?;
                        }
                        GeneratedField::EmailSubjectRange => {
                            if email_subject_range__.is_some() {
                                return Err(serde::de::Error::duplicate_field("emailSubjectRange"));
                            }
                            email_subject_range__ = map_.next_value()?;
                        }
                    }
                }
                Ok(PublicInputIndices {
                    min_length: min_length__.unwrap_or_default(),
                    email_hash_index: email_hash_index__.unwrap_or_default(),
                    dkim_domain_range: dkim_domain_range__,
                    dkim_hash_index: dkim_hash_index__.unwrap_or_default(),
                    tx_bytes_range: tx_bytes_range__,
                    email_host_range: email_host_range__,
                    email_subject_range: email_subject_range__,
                })
            }
        }
        deserializer.deserialize_struct("xion.dkim.v1.PublicInputIndices", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryAuthenticateRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tx_bytes.is_empty() {
            len += 1;
        }
        if !self.email_hash.is_empty() {
            len += 1;
        }
        if !self.proof.is_empty() {
            len += 1;
        }
        if !self.public_inputs.is_empty() {
            len += 1;
        }
        if !self.allowed_email_hosts.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("xion.dkim.v1.QueryAuthenticateRequest", len)?;
        if !self.tx_bytes.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "txBytes",
                pbjson::private::base64::encode(&self.tx_bytes).as_str(),
            )?;
        }
        if !self.email_hash.is_empty() {
            struct_ser.serialize_field("emailHash", &self.email_hash)?;
        }
        if !self.proof.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "proof",
                pbjson::private::base64::encode(&self.proof).as_str(),
            )?;
        }
        if !self.public_inputs.is_empty() {
            struct_ser.serialize_field("publicInputs", &self.public_inputs)?;
        }
        if !self.allowed_email_hosts.is_empty() {
            struct_ser.serialize_field("allowedEmailHosts", &self.allowed_email_hosts)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryAuthenticateRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tx_bytes",
            "txBytes",
            "email_hash",
            "emailHash",
            "proof",
            "public_inputs",
            "publicInputs",
            "allowed_email_hosts",
            "allowedEmailHosts",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TxBytes,
            EmailHash,
            Proof,
            PublicInputs,
            AllowedEmailHosts,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "txBytes" | "tx_bytes" => Ok(GeneratedField::TxBytes),
                            "emailHash" | "email_hash" => Ok(GeneratedField::EmailHash),
                            "proof" => Ok(GeneratedField::Proof),
                            "publicInputs" | "public_inputs" => Ok(GeneratedField::PublicInputs),
                            "allowedEmailHosts" | "allowed_email_hosts" => {
                                Ok(GeneratedField::AllowedEmailHosts)
                            }
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryAuthenticateRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.dkim.v1.QueryAuthenticateRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryAuthenticateRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut tx_bytes__ = None;
                let mut email_hash__ = None;
                let mut proof__ = None;
                let mut public_inputs__ = None;
                let mut allowed_email_hosts__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TxBytes => {
                            if tx_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txBytes"));
                            }
                            tx_bytes__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::EmailHash => {
                            if email_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("emailHash"));
                            }
                            email_hash__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Proof => {
                            if proof__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proof"));
                            }
                            proof__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::PublicInputs => {
                            if public_inputs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("publicInputs"));
                            }
                            public_inputs__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AllowedEmailHosts => {
                            if allowed_email_hosts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowedEmailHosts"));
                            }
                            allowed_email_hosts__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryAuthenticateRequest {
                    tx_bytes: tx_bytes__.unwrap_or_default(),
                    email_hash: email_hash__.unwrap_or_default(),
                    proof: proof__.unwrap_or_default(),
                    public_inputs: public_inputs__.unwrap_or_default(),
                    allowed_email_hosts: allowed_email_hosts__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "xion.dkim.v1.QueryAuthenticateRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryDkimPubKeyRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.selector.is_empty() {
            len += 1;
        }
        if !self.domain.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("xion.dkim.v1.QueryDkimPubKeyRequest", len)?;
        if !self.selector.is_empty() {
            struct_ser.serialize_field("selector", &self.selector)?;
        }
        if !self.domain.is_empty() {
            struct_ser.serialize_field("domain", &self.domain)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryDkimPubKeyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["selector", "domain"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Selector,
            Domain,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "selector" => Ok(GeneratedField::Selector),
                            "domain" => Ok(GeneratedField::Domain),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryDkimPubKeyRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.dkim.v1.QueryDkimPubKeyRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryDkimPubKeyRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut selector__ = None;
                let mut domain__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Selector => {
                            if selector__.is_some() {
                                return Err(serde::de::Error::duplicate_field("selector"));
                            }
                            selector__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Domain => {
                            if domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domain"));
                            }
                            domain__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryDkimPubKeyRequest {
                    selector: selector__.unwrap_or_default(),
                    domain: domain__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "xion.dkim.v1.QueryDkimPubKeyRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryDkimPubKeyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.dkim_pub_key.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("xion.dkim.v1.QueryDkimPubKeyResponse", len)?;
        if let Some(v) = self.dkim_pub_key.as_ref() {
            struct_ser.serialize_field("dkimPubKey", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryDkimPubKeyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["dkim_pub_key", "dkimPubKey"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DkimPubKey,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "dkimPubKey" | "dkim_pub_key" => Ok(GeneratedField::DkimPubKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryDkimPubKeyResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.dkim.v1.QueryDkimPubKeyResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryDkimPubKeyResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut dkim_pub_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DkimPubKey => {
                            if dkim_pub_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dkimPubKey"));
                            }
                            dkim_pub_key__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryDkimPubKeyResponse {
                    dkim_pub_key: dkim_pub_key__,
                })
            }
        }
        deserializer.deserialize_struct(
            "xion.dkim.v1.QueryDkimPubKeyResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryDkimPubKeysRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.selector.is_empty() {
            len += 1;
        }
        if !self.domain.is_empty() {
            len += 1;
        }
        if !self.poseidon_hash.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("xion.dkim.v1.QueryDkimPubKeysRequest", len)?;
        if !self.selector.is_empty() {
            struct_ser.serialize_field("selector", &self.selector)?;
        }
        if !self.domain.is_empty() {
            struct_ser.serialize_field("domain", &self.domain)?;
        }
        if !self.poseidon_hash.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "poseidonHash",
                pbjson::private::base64::encode(&self.poseidon_hash).as_str(),
            )?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryDkimPubKeysRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "selector",
            "domain",
            "poseidon_hash",
            "poseidonHash",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Selector,
            Domain,
            PoseidonHash,
            Pagination,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "selector" => Ok(GeneratedField::Selector),
                            "domain" => Ok(GeneratedField::Domain),
                            "poseidonHash" | "poseidon_hash" => Ok(GeneratedField::PoseidonHash),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryDkimPubKeysRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.dkim.v1.QueryDkimPubKeysRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryDkimPubKeysRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut selector__ = None;
                let mut domain__ = None;
                let mut poseidon_hash__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Selector => {
                            if selector__.is_some() {
                                return Err(serde::de::Error::duplicate_field("selector"));
                            }
                            selector__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Domain => {
                            if domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domain"));
                            }
                            domain__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PoseidonHash => {
                            if poseidon_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("poseidonHash"));
                            }
                            poseidon_hash__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryDkimPubKeysRequest {
                    selector: selector__.unwrap_or_default(),
                    domain: domain__.unwrap_or_default(),
                    poseidon_hash: poseidon_hash__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "xion.dkim.v1.QueryDkimPubKeysRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryDkimPubKeysResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.dkim_pub_keys.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("xion.dkim.v1.QueryDkimPubKeysResponse", len)?;
        if !self.dkim_pub_keys.is_empty() {
            struct_ser.serialize_field("dkimPubKeys", &self.dkim_pub_keys)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryDkimPubKeysResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["dkim_pub_keys", "dkimPubKeys", "pagination"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DkimPubKeys,
            Pagination,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "dkimPubKeys" | "dkim_pub_keys" => Ok(GeneratedField::DkimPubKeys),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryDkimPubKeysResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.dkim.v1.QueryDkimPubKeysResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryDkimPubKeysResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut dkim_pub_keys__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DkimPubKeys => {
                            if dkim_pub_keys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dkimPubKeys"));
                            }
                            dkim_pub_keys__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryDkimPubKeysResponse {
                    dkim_pub_keys: dkim_pub_keys__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "xion.dkim.v1.QueryDkimPubKeysResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryParamsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("xion.dkim.v1.QueryParamsRequest", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryParamsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryParamsRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.dkim.v1.QueryParamsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryParamsRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryParamsRequest {})
            }
        }
        deserializer.deserialize_struct("xion.dkim.v1.QueryParamsRequest", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryParamsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.params.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("xion.dkim.v1.QueryParamsResponse", len)?;
        if let Some(v) = self.params.as_ref() {
            struct_ser.serialize_field("params", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryParamsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["params"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Params,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "params" => Ok(GeneratedField::Params),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryParamsResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.dkim.v1.QueryParamsResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryParamsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut params__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryParamsResponse { params: params__ })
            }
        }
        deserializer.deserialize_struct(
            "xion.dkim.v1.QueryParamsResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for Version {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Dkim1Unspecified => "VERSION_DKIM1_UNSPECIFIED",
        };
        serializer.serialize_str(variant)
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Version {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["VERSION_DKIM1_UNSPECIFIED"];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Version;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "VERSION_DKIM1_UNSPECIFIED" => Ok(Version::Dkim1Unspecified),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
