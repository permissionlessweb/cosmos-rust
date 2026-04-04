// @generated
#[cfg(feature = "serde")]
impl serde::Serialize for AllowedMsgAllowance {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.allowance.is_some() {
            len += 1;
        }
        if !self.allowed_messages.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmos.feegrant.v1beta1.AllowedMsgAllowance", len)?;
        if let Some(v) = self.allowance.as_ref() {
            struct_ser.serialize_field("allowance", v)?;
        }
        if !self.allowed_messages.is_empty() {
            struct_ser.serialize_field("allowedMessages", &self.allowed_messages)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for AllowedMsgAllowance {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["allowance", "allowed_messages", "allowedMessages"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Allowance,
            AllowedMessages,
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
                            "allowance" => Ok(GeneratedField::Allowance),
                            "allowedMessages" | "allowed_messages" => {
                                Ok(GeneratedField::AllowedMessages)
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
            type Value = AllowedMsgAllowance;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct cosmos.feegrant.v1beta1.AllowedMsgAllowance")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<AllowedMsgAllowance, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut allowance__ = None;
                let mut allowed_messages__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Allowance => {
                            if allowance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowance"));
                            }
                            allowance__ = map_.next_value()?;
                        }
                        GeneratedField::AllowedMessages => {
                            if allowed_messages__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowedMessages"));
                            }
                            allowed_messages__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AllowedMsgAllowance {
                    allowance: allowance__,
                    allowed_messages: allowed_messages__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmos.feegrant.v1beta1.AllowedMsgAllowance",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for BasicAllowance {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.spend_limit.is_empty() {
            len += 1;
        }
        if self.expiration.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmos.feegrant.v1beta1.BasicAllowance", len)?;
        if !self.spend_limit.is_empty() {
            struct_ser.serialize_field("spendLimit", &self.spend_limit)?;
        }
        if let Some(v) = self.expiration.as_ref() {
            struct_ser.serialize_field("expiration", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for BasicAllowance {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["spend_limit", "spendLimit", "expiration"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SpendLimit,
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
                            "spendLimit" | "spend_limit" => Ok(GeneratedField::SpendLimit),
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
            type Value = BasicAllowance;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct cosmos.feegrant.v1beta1.BasicAllowance")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<BasicAllowance, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut spend_limit__ = None;
                let mut expiration__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SpendLimit => {
                            if spend_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spendLimit"));
                            }
                            spend_limit__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Expiration => {
                            if expiration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expiration"));
                            }
                            expiration__ = map_.next_value()?;
                        }
                    }
                }
                Ok(BasicAllowance {
                    spend_limit: spend_limit__.unwrap_or_default(),
                    expiration: expiration__,
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmos.feegrant.v1beta1.BasicAllowance",
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
        if !self.granter.is_empty() {
            len += 1;
        }
        if !self.grantee.is_empty() {
            len += 1;
        }
        if self.allowance.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cosmos.feegrant.v1beta1.Grant", len)?;
        if !self.granter.is_empty() {
            struct_ser.serialize_field("granter", &self.granter)?;
        }
        if !self.grantee.is_empty() {
            struct_ser.serialize_field("grantee", &self.grantee)?;
        }
        if let Some(v) = self.allowance.as_ref() {
            struct_ser.serialize_field("allowance", v)?;
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
        const FIELDS: &[&str] = &["granter", "grantee", "allowance"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Granter,
            Grantee,
            Allowance,
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
                            "allowance" => Ok(GeneratedField::Allowance),
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
                formatter.write_str("struct cosmos.feegrant.v1beta1.Grant")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<Grant, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut granter__ = None;
                let mut grantee__ = None;
                let mut allowance__ = None;
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
                        GeneratedField::Allowance => {
                            if allowance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowance"));
                            }
                            allowance__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Grant {
                    granter: granter__.unwrap_or_default(),
                    grantee: grantee__.unwrap_or_default(),
                    allowance: allowance__,
                })
            }
        }
        deserializer.deserialize_struct("cosmos.feegrant.v1beta1.Grant", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for PeriodicAllowance {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.basic.is_some() {
            len += 1;
        }
        if self.period.is_some() {
            len += 1;
        }
        if !self.period_spend_limit.is_empty() {
            len += 1;
        }
        if !self.period_can_spend.is_empty() {
            len += 1;
        }
        if self.period_reset.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmos.feegrant.v1beta1.PeriodicAllowance", len)?;
        if let Some(v) = self.basic.as_ref() {
            struct_ser.serialize_field("basic", v)?;
        }
        if let Some(v) = self.period.as_ref() {
            struct_ser.serialize_field("period", v)?;
        }
        if !self.period_spend_limit.is_empty() {
            struct_ser.serialize_field("periodSpendLimit", &self.period_spend_limit)?;
        }
        if !self.period_can_spend.is_empty() {
            struct_ser.serialize_field("periodCanSpend", &self.period_can_spend)?;
        }
        if let Some(v) = self.period_reset.as_ref() {
            struct_ser.serialize_field("periodReset", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for PeriodicAllowance {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "basic",
            "period",
            "period_spend_limit",
            "periodSpendLimit",
            "period_can_spend",
            "periodCanSpend",
            "period_reset",
            "periodReset",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Basic,
            Period,
            PeriodSpendLimit,
            PeriodCanSpend,
            PeriodReset,
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
                            "basic" => Ok(GeneratedField::Basic),
                            "period" => Ok(GeneratedField::Period),
                            "periodSpendLimit" | "period_spend_limit" => {
                                Ok(GeneratedField::PeriodSpendLimit)
                            }
                            "periodCanSpend" | "period_can_spend" => {
                                Ok(GeneratedField::PeriodCanSpend)
                            }
                            "periodReset" | "period_reset" => Ok(GeneratedField::PeriodReset),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PeriodicAllowance;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct cosmos.feegrant.v1beta1.PeriodicAllowance")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<PeriodicAllowance, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut basic__ = None;
                let mut period__ = None;
                let mut period_spend_limit__ = None;
                let mut period_can_spend__ = None;
                let mut period_reset__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Basic => {
                            if basic__.is_some() {
                                return Err(serde::de::Error::duplicate_field("basic"));
                            }
                            basic__ = map_.next_value()?;
                        }
                        GeneratedField::Period => {
                            if period__.is_some() {
                                return Err(serde::de::Error::duplicate_field("period"));
                            }
                            period__ = map_.next_value()?;
                        }
                        GeneratedField::PeriodSpendLimit => {
                            if period_spend_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("periodSpendLimit"));
                            }
                            period_spend_limit__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PeriodCanSpend => {
                            if period_can_spend__.is_some() {
                                return Err(serde::de::Error::duplicate_field("periodCanSpend"));
                            }
                            period_can_spend__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PeriodReset => {
                            if period_reset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("periodReset"));
                            }
                            period_reset__ = map_.next_value()?;
                        }
                    }
                }
                Ok(PeriodicAllowance {
                    basic: basic__,
                    period: period__,
                    period_spend_limit: period_spend_limit__.unwrap_or_default(),
                    period_can_spend: period_can_spend__.unwrap_or_default(),
                    period_reset: period_reset__,
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmos.feegrant.v1beta1.PeriodicAllowance",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
