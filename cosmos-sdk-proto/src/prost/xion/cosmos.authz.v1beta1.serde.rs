// @generated
#[cfg(feature = "serde")]
impl serde::Serialize for GenericAuthorization {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.msg.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmos.authz.v1beta1.GenericAuthorization", len)?;
        if !self.msg.is_empty() {
            struct_ser.serialize_field("msg", &self.msg)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for GenericAuthorization {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["msg"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Msg,
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
                            "msg" => Ok(GeneratedField::Msg),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GenericAuthorization;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct cosmos.authz.v1beta1.GenericAuthorization")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<GenericAuthorization, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut msg__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Msg => {
                            if msg__.is_some() {
                                return Err(serde::de::Error::duplicate_field("msg"));
                            }
                            msg__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GenericAuthorization {
                    msg: msg__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmos.authz.v1beta1.GenericAuthorization",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for Grant {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.authorization.is_some() {
            len += 1;
        }
        if self.expiration.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cosmos.authz.v1beta1.Grant", len)?;
        if let Some(v) = self.authorization.as_ref() {
            struct_ser.serialize_field("authorization", v)?;
        }
        if let Some(v) = self.expiration.as_ref() {
            struct_ser.serialize_field("expiration", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Grant {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["authorization", "expiration"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Authorization,
            Expiration,
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
                            "authorization" => Ok(GeneratedField::Authorization),
                            "expiration" => Ok(GeneratedField::Expiration),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Grant;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct cosmos.authz.v1beta1.Grant")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<Grant, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut authorization__ = None;
                let mut expiration__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Authorization => {
                            if authorization__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authorization"));
                            }
                            authorization__ = map_.next_value()?;
                        }
                        GeneratedField::Expiration => {
                            if expiration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expiration"));
                            }
                            expiration__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Grant {
                    authorization: authorization__,
                    expiration: expiration__,
                })
            }
        }
        deserializer.deserialize_struct("cosmos.authz.v1beta1.Grant", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for GrantAuthorization {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.granter.is_empty() {
            len += 1;
        }
        if !self.grantee.is_empty() {
            len += 1;
        }
        if self.authorization.is_some() {
            len += 1;
        }
        if self.expiration.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmos.authz.v1beta1.GrantAuthorization", len)?;
        if !self.granter.is_empty() {
            struct_ser.serialize_field("granter", &self.granter)?;
        }
        if !self.grantee.is_empty() {
            struct_ser.serialize_field("grantee", &self.grantee)?;
        }
        if let Some(v) = self.authorization.as_ref() {
            struct_ser.serialize_field("authorization", v)?;
        }
        if let Some(v) = self.expiration.as_ref() {
            struct_ser.serialize_field("expiration", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for GrantAuthorization {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["granter", "grantee", "authorization", "expiration"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Granter,
            Grantee,
            Authorization,
            Expiration,
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
                            "granter" => Ok(GeneratedField::Granter),
                            "grantee" => Ok(GeneratedField::Grantee),
                            "authorization" => Ok(GeneratedField::Authorization),
                            "expiration" => Ok(GeneratedField::Expiration),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GrantAuthorization;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct cosmos.authz.v1beta1.GrantAuthorization")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<GrantAuthorization, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut granter__ = None;
                let mut grantee__ = None;
                let mut authorization__ = None;
                let mut expiration__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Granter => {
                            if granter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("granter"));
                            }
                            granter__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Grantee => {
                            if grantee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grantee"));
                            }
                            grantee__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Authorization => {
                            if authorization__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authorization"));
                            }
                            authorization__ = map_.next_value()?;
                        }
                        GeneratedField::Expiration => {
                            if expiration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expiration"));
                            }
                            expiration__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GrantAuthorization {
                    granter: granter__.unwrap_or_default(),
                    grantee: grantee__.unwrap_or_default(),
                    authorization: authorization__,
                    expiration: expiration__,
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmos.authz.v1beta1.GrantAuthorization",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for GrantQueueItem {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.msg_type_urls.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmos.authz.v1beta1.GrantQueueItem", len)?;
        if !self.msg_type_urls.is_empty() {
            struct_ser.serialize_field("msgTypeUrls", &self.msg_type_urls)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for GrantQueueItem {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["msg_type_urls", "msgTypeUrls"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MsgTypeUrls,
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
                            "msgTypeUrls" | "msg_type_urls" => Ok(GeneratedField::MsgTypeUrls),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GrantQueueItem;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct cosmos.authz.v1beta1.GrantQueueItem")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<GrantQueueItem, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut msg_type_urls__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MsgTypeUrls => {
                            if msg_type_urls__.is_some() {
                                return Err(serde::de::Error::duplicate_field("msgTypeUrls"));
                            }
                            msg_type_urls__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GrantQueueItem {
                    msg_type_urls: msg_type_urls__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmos.authz.v1beta1.GrantQueueItem",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
