// @generated
#[cfg(feature = "serde")]
impl serde::Serialize for AddHostZoneProposal {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if self.host_chain_config.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("xion.feeabs.v1beta1.AddHostZoneProposal", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if let Some(v) = self.host_chain_config.as_ref() {
            struct_ser.serialize_field("hostChainConfig", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for AddHostZoneProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "host_chain_config",
            "hostChainConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            HostChainConfig,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "hostChainConfig" | "host_chain_config" => {
                                Ok(GeneratedField::HostChainConfig)
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
            type Value = AddHostZoneProposal;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.feeabs.v1beta1.AddHostZoneProposal")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<AddHostZoneProposal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut host_chain_config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HostChainConfig => {
                            if host_chain_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostChainConfig"));
                            }
                            host_chain_config__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AddHostZoneProposal {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    host_chain_config: host_chain_config__,
                })
            }
        }
        deserializer.deserialize_struct(
            "xion.feeabs.v1beta1.AddHostZoneProposal",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for CosmosQuery {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.requests.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("xion.feeabs.v1beta1.CosmosQuery", len)?;
        if !self.requests.is_empty() {
            struct_ser.serialize_field("requests", &self.requests)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for CosmosQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["requests"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Requests,
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
                            "requests" => Ok(GeneratedField::Requests),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CosmosQuery;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.feeabs.v1beta1.CosmosQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<CosmosQuery, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut requests__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Requests => {
                            if requests__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requests"));
                            }
                            requests__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CosmosQuery {
                    requests: requests__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("xion.feeabs.v1beta1.CosmosQuery", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for CosmosResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.responses.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("xion.feeabs.v1beta1.CosmosResponse", len)?;
        if !self.responses.is_empty() {
            struct_ser.serialize_field("responses", &self.responses)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for CosmosResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["responses"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Responses,
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
                            "responses" => Ok(GeneratedField::Responses),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CosmosResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.feeabs.v1beta1.CosmosResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<CosmosResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut responses__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Responses => {
                            if responses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responses"));
                            }
                            responses__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CosmosResponse {
                    responses: responses__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "xion.feeabs.v1beta1.CosmosResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for DeleteHostZoneProposal {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.ibc_denom.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("xion.feeabs.v1beta1.DeleteHostZoneProposal", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.ibc_denom.is_empty() {
            struct_ser.serialize_field("ibcDenom", &self.ibc_denom)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for DeleteHostZoneProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["title", "description", "ibc_denom", "ibcDenom"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            IbcDenom,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "ibcDenom" | "ibc_denom" => Ok(GeneratedField::IbcDenom),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteHostZoneProposal;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.feeabs.v1beta1.DeleteHostZoneProposal")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<DeleteHostZoneProposal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut ibc_denom__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IbcDenom => {
                            if ibc_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ibcDenom"));
                            }
                            ibc_denom__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeleteHostZoneProposal {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    ibc_denom: ibc_denom__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "xion.feeabs.v1beta1.DeleteHostZoneProposal",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for EpochInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.identifier.is_empty() {
            len += 1;
        }
        if self.start_time.is_some() {
            len += 1;
        }
        if self.duration.is_some() {
            len += 1;
        }
        if self.current_epoch != 0 {
            len += 1;
        }
        if self.current_epoch_start_time.is_some() {
            len += 1;
        }
        if self.epoch_counting_started {
            len += 1;
        }
        if self.current_epoch_start_height != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("xion.feeabs.v1beta1.EpochInfo", len)?;
        if !self.identifier.is_empty() {
            struct_ser.serialize_field("identifier", &self.identifier)?;
        }
        if let Some(v) = self.start_time.as_ref() {
            struct_ser.serialize_field("startTime", v)?;
        }
        if let Some(v) = self.duration.as_ref() {
            struct_ser.serialize_field("duration", v)?;
        }
        if self.current_epoch != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "currentEpoch",
                alloc::string::ToString::to_string(&self.current_epoch).as_str(),
            )?;
        }
        if let Some(v) = self.current_epoch_start_time.as_ref() {
            struct_ser.serialize_field("currentEpochStartTime", v)?;
        }
        if self.epoch_counting_started {
            struct_ser.serialize_field("epochCountingStarted", &self.epoch_counting_started)?;
        }
        if self.current_epoch_start_height != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "currentEpochStartHeight",
                alloc::string::ToString::to_string(&self.current_epoch_start_height).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for EpochInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "identifier",
            "start_time",
            "startTime",
            "duration",
            "current_epoch",
            "currentEpoch",
            "current_epoch_start_time",
            "currentEpochStartTime",
            "epoch_counting_started",
            "epochCountingStarted",
            "current_epoch_start_height",
            "currentEpochStartHeight",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Identifier,
            StartTime,
            Duration,
            CurrentEpoch,
            CurrentEpochStartTime,
            EpochCountingStarted,
            CurrentEpochStartHeight,
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
                            "identifier" => Ok(GeneratedField::Identifier),
                            "startTime" | "start_time" => Ok(GeneratedField::StartTime),
                            "duration" => Ok(GeneratedField::Duration),
                            "currentEpoch" | "current_epoch" => Ok(GeneratedField::CurrentEpoch),
                            "currentEpochStartTime" | "current_epoch_start_time" => {
                                Ok(GeneratedField::CurrentEpochStartTime)
                            }
                            "epochCountingStarted" | "epoch_counting_started" => {
                                Ok(GeneratedField::EpochCountingStarted)
                            }
                            "currentEpochStartHeight" | "current_epoch_start_height" => {
                                Ok(GeneratedField::CurrentEpochStartHeight)
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
            type Value = EpochInfo;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.feeabs.v1beta1.EpochInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<EpochInfo, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut identifier__ = None;
                let mut start_time__ = None;
                let mut duration__ = None;
                let mut current_epoch__ = None;
                let mut current_epoch_start_time__ = None;
                let mut epoch_counting_started__ = None;
                let mut current_epoch_start_height__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Identifier => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            identifier__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StartTime => {
                            if start_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startTime"));
                            }
                            start_time__ = map_.next_value()?;
                        }
                        GeneratedField::Duration => {
                            if duration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("duration"));
                            }
                            duration__ = map_.next_value()?;
                        }
                        GeneratedField::CurrentEpoch => {
                            if current_epoch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("currentEpoch"));
                            }
                            current_epoch__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::CurrentEpochStartTime => {
                            if current_epoch_start_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "currentEpochStartTime",
                                ));
                            }
                            current_epoch_start_time__ = map_.next_value()?;
                        }
                        GeneratedField::EpochCountingStarted => {
                            if epoch_counting_started__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "epochCountingStarted",
                                ));
                            }
                            epoch_counting_started__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CurrentEpochStartHeight => {
                            if current_epoch_start_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "currentEpochStartHeight",
                                ));
                            }
                            current_epoch_start_height__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(EpochInfo {
                    identifier: identifier__.unwrap_or_default(),
                    start_time: start_time__,
                    duration: duration__,
                    current_epoch: current_epoch__.unwrap_or_default(),
                    current_epoch_start_time: current_epoch_start_time__,
                    epoch_counting_started: epoch_counting_started__.unwrap_or_default(),
                    current_epoch_start_height: current_epoch_start_height__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("xion.feeabs.v1beta1.EpochInfo", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for ExponentialBackoff {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.jump != 0 {
            len += 1;
        }
        if self.future_epoch != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("xion.feeabs.v1beta1.ExponentialBackoff", len)?;
        if self.jump != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "jump",
                alloc::string::ToString::to_string(&self.jump).as_str(),
            )?;
        }
        if self.future_epoch != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "futureEpoch",
                alloc::string::ToString::to_string(&self.future_epoch).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ExponentialBackoff {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["jump", "future_epoch", "futureEpoch"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Jump,
            FutureEpoch,
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
                            "jump" => Ok(GeneratedField::Jump),
                            "futureEpoch" | "future_epoch" => Ok(GeneratedField::FutureEpoch),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExponentialBackoff;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.feeabs.v1beta1.ExponentialBackoff")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<ExponentialBackoff, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut jump__ = None;
                let mut future_epoch__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Jump => {
                            if jump__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jump"));
                            }
                            jump__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::FutureEpoch => {
                            if future_epoch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("futureEpoch"));
                            }
                            future_epoch__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(ExponentialBackoff {
                    jump: jump__.unwrap_or_default(),
                    future_epoch: future_epoch__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "xion.feeabs.v1beta1.ExponentialBackoff",
            FIELDS,
            GeneratedVisitor,
        )
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
        if !self.epochs.is_empty() {
            len += 1;
        }
        if !self.port_id.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("xion.feeabs.v1beta1.GenesisState", len)?;
        if let Some(v) = self.params.as_ref() {
            struct_ser.serialize_field("params", v)?;
        }
        if !self.epochs.is_empty() {
            struct_ser.serialize_field("epochs", &self.epochs)?;
        }
        if !self.port_id.is_empty() {
            struct_ser.serialize_field("portId", &self.port_id)?;
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
        const FIELDS: &[&str] = &["params", "epochs", "port_id", "portId"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Params,
            Epochs,
            PortId,
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
                            "epochs" => Ok(GeneratedField::Epochs),
                            "portId" | "port_id" => Ok(GeneratedField::PortId),
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
                formatter.write_str("struct xion.feeabs.v1beta1.GenesisState")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<GenesisState, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut params__ = None;
                let mut epochs__ = None;
                let mut port_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = map_.next_value()?;
                        }
                        GeneratedField::Epochs => {
                            if epochs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("epochs"));
                            }
                            epochs__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PortId => {
                            if port_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("portId"));
                            }
                            port_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GenesisState {
                    params: params__,
                    epochs: epochs__.unwrap_or_default(),
                    port_id: port_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "xion.feeabs.v1beta1.GenesisState",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for HostChainFeeAbsConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.ibc_denom.is_empty() {
            len += 1;
        }
        if !self.osmosis_pool_token_denom_in.is_empty() {
            len += 1;
        }
        if self.pool_id != 0 {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("xion.feeabs.v1beta1.HostChainFeeAbsConfig", len)?;
        if !self.ibc_denom.is_empty() {
            struct_ser.serialize_field("ibcDenom", &self.ibc_denom)?;
        }
        if !self.osmosis_pool_token_denom_in.is_empty() {
            struct_ser
                .serialize_field("osmosisPoolTokenDenomIn", &self.osmosis_pool_token_denom_in)?;
        }
        if self.pool_id != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "poolId",
                alloc::string::ToString::to_string(&self.pool_id).as_str(),
            )?;
        }
        if self.status != 0 {
            let v = HostChainFeeAbsStatus::try_from(self.status).map_err(|_| {
                serde::ser::Error::custom(alloc::format!("Invalid variant {}", self.status))
            })?;
            struct_ser.serialize_field("status", &v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for HostChainFeeAbsConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ibc_denom",
            "ibcDenom",
            "osmosis_pool_token_denom_in",
            "osmosisPoolTokenDenomIn",
            "pool_id",
            "poolId",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IbcDenom,
            OsmosisPoolTokenDenomIn,
            PoolId,
            Status,
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
                            "ibcDenom" | "ibc_denom" => Ok(GeneratedField::IbcDenom),
                            "osmosisPoolTokenDenomIn" | "osmosis_pool_token_denom_in" => {
                                Ok(GeneratedField::OsmosisPoolTokenDenomIn)
                            }
                            "poolId" | "pool_id" => Ok(GeneratedField::PoolId),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostChainFeeAbsConfig;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.feeabs.v1beta1.HostChainFeeAbsConfig")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<HostChainFeeAbsConfig, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut ibc_denom__ = None;
                let mut osmosis_pool_token_denom_in__ = None;
                let mut pool_id__ = None;
                let mut status__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::IbcDenom => {
                            if ibc_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ibcDenom"));
                            }
                            ibc_denom__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OsmosisPoolTokenDenomIn => {
                            if osmosis_pool_token_denom_in__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "osmosisPoolTokenDenomIn",
                                ));
                            }
                            osmosis_pool_token_denom_in__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PoolId => {
                            if pool_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("poolId"));
                            }
                            pool_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<HostChainFeeAbsStatus>()? as i32);
                        }
                    }
                }
                Ok(HostChainFeeAbsConfig {
                    ibc_denom: ibc_denom__.unwrap_or_default(),
                    osmosis_pool_token_denom_in: osmosis_pool_token_denom_in__.unwrap_or_default(),
                    pool_id: pool_id__.unwrap_or_default(),
                    status: status__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "xion.feeabs.v1beta1.HostChainFeeAbsConfig",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for HostChainFeeAbsStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "HOST_CHAIN_FEE_ABS_STATUS_UNSPECIFIED",
            Self::Updated => "HOST_CHAIN_FEE_ABS_STATUS_UPDATED",
            Self::Outdated => "HOST_CHAIN_FEE_ABS_STATUS_OUTDATED",
            Self::Frozen => "HOST_CHAIN_FEE_ABS_STATUS_FROZEN",
        };
        serializer.serialize_str(variant)
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for HostChainFeeAbsStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "HOST_CHAIN_FEE_ABS_STATUS_UNSPECIFIED",
            "HOST_CHAIN_FEE_ABS_STATUS_UPDATED",
            "HOST_CHAIN_FEE_ABS_STATUS_OUTDATED",
            "HOST_CHAIN_FEE_ABS_STATUS_FROZEN",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostChainFeeAbsStatus;

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
                    "HOST_CHAIN_FEE_ABS_STATUS_UNSPECIFIED" => {
                        Ok(HostChainFeeAbsStatus::Unspecified)
                    }
                    "HOST_CHAIN_FEE_ABS_STATUS_UPDATED" => Ok(HostChainFeeAbsStatus::Updated),
                    "HOST_CHAIN_FEE_ABS_STATUS_OUTDATED" => Ok(HostChainFeeAbsStatus::Outdated),
                    "HOST_CHAIN_FEE_ABS_STATUS_FROZEN" => Ok(HostChainFeeAbsStatus::Frozen),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for InterchainQueryPacketAck {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.data.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("xion.feeabs.v1beta1.InterchainQueryPacketAck", len)?;
        if !self.data.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser
                .serialize_field("data", pbjson::private::base64::encode(&self.data).as_str())?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for InterchainQueryPacketAck {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["data"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Data,
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
                            "data" => Ok(GeneratedField::Data),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InterchainQueryPacketAck;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.feeabs.v1beta1.InterchainQueryPacketAck")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<InterchainQueryPacketAck, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(InterchainQueryPacketAck {
                    data: data__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "xion.feeabs.v1beta1.InterchainQueryPacketAck",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for InterchainQueryPacketData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.data.is_empty() {
            len += 1;
        }
        if !self.memo.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("xion.feeabs.v1beta1.InterchainQueryPacketData", len)?;
        if !self.data.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser
                .serialize_field("data", pbjson::private::base64::encode(&self.data).as_str())?;
        }
        if !self.memo.is_empty() {
            struct_ser.serialize_field("memo", &self.memo)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for InterchainQueryPacketData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["data", "memo"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Data,
            Memo,
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
                            "data" => Ok(GeneratedField::Data),
                            "memo" => Ok(GeneratedField::Memo),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InterchainQueryPacketData;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.feeabs.v1beta1.InterchainQueryPacketData")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<InterchainQueryPacketData, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut data__ = None;
                let mut memo__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Memo => {
                            if memo__.is_some() {
                                return Err(serde::de::Error::duplicate_field("memo"));
                            }
                            memo__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(InterchainQueryPacketData {
                    data: data__.unwrap_or_default(),
                    memo: memo__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "xion.feeabs.v1beta1.InterchainQueryPacketData",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for InterchainQueryRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.data.is_empty() {
            len += 1;
        }
        if !self.path.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("xion.feeabs.v1beta1.InterchainQueryRequest", len)?;
        if !self.data.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser
                .serialize_field("data", pbjson::private::base64::encode(&self.data).as_str())?;
        }
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for InterchainQueryRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["data", "path"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Data,
            Path,
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
                            "data" => Ok(GeneratedField::Data),
                            "path" => Ok(GeneratedField::Path),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InterchainQueryRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.feeabs.v1beta1.InterchainQueryRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<InterchainQueryRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut data__ = None;
                let mut path__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(InterchainQueryRequest {
                    data: data__.unwrap_or_default(),
                    path: path__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "xion.feeabs.v1beta1.InterchainQueryRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for InterchainQueryRequestPacket {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.requests.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("xion.feeabs.v1beta1.InterchainQueryRequestPacket", len)?;
        if !self.requests.is_empty() {
            struct_ser.serialize_field("requests", &self.requests)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for InterchainQueryRequestPacket {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["requests"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Requests,
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
                            "requests" => Ok(GeneratedField::Requests),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InterchainQueryRequestPacket;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.feeabs.v1beta1.InterchainQueryRequestPacket")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<InterchainQueryRequestPacket, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut requests__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Requests => {
                            if requests__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requests"));
                            }
                            requests__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(InterchainQueryRequestPacket {
                    requests: requests__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "xion.feeabs.v1beta1.InterchainQueryRequestPacket",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgAddHostZone {
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
        if self.host_chain_config.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("xion.feeabs.v1beta1.MsgAddHostZone", len)?;
        if !self.authority.is_empty() {
            struct_ser.serialize_field("authority", &self.authority)?;
        }
        if let Some(v) = self.host_chain_config.as_ref() {
            struct_ser.serialize_field("hostChainConfig", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgAddHostZone {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["authority", "host_chain_config", "hostChainConfig"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Authority,
            HostChainConfig,
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
                            "hostChainConfig" | "host_chain_config" => {
                                Ok(GeneratedField::HostChainConfig)
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
            type Value = MsgAddHostZone;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.feeabs.v1beta1.MsgAddHostZone")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgAddHostZone, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut authority__ = None;
                let mut host_chain_config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Authority => {
                            if authority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            authority__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HostChainConfig => {
                            if host_chain_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostChainConfig"));
                            }
                            host_chain_config__ = map_.next_value()?;
                        }
                    }
                }
                Ok(MsgAddHostZone {
                    authority: authority__.unwrap_or_default(),
                    host_chain_config: host_chain_config__,
                })
            }
        }
        deserializer.deserialize_struct(
            "xion.feeabs.v1beta1.MsgAddHostZone",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgAddHostZoneResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("xion.feeabs.v1beta1.MsgAddHostZoneResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgAddHostZoneResponse {
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
            type Value = MsgAddHostZoneResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.feeabs.v1beta1.MsgAddHostZoneResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<MsgAddHostZoneResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgAddHostZoneResponse {})
            }
        }
        deserializer.deserialize_struct(
            "xion.feeabs.v1beta1.MsgAddHostZoneResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgFundFeeAbsModuleAccount {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sender.is_empty() {
            len += 1;
        }
        if !self.amount.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("xion.feeabs.v1beta1.MsgFundFeeAbsModuleAccount", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgFundFeeAbsModuleAccount {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["sender", "amount"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            Amount,
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
                            "sender" => Ok(GeneratedField::Sender),
                            "amount" => Ok(GeneratedField::Amount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgFundFeeAbsModuleAccount;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.feeabs.v1beta1.MsgFundFeeAbsModuleAccount")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<MsgFundFeeAbsModuleAccount, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut amount__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgFundFeeAbsModuleAccount {
                    sender: sender__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "xion.feeabs.v1beta1.MsgFundFeeAbsModuleAccount",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgFundFeeAbsModuleAccountResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct(
            "xion.feeabs.v1beta1.MsgFundFeeAbsModuleAccountResponse",
            len,
        )?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgFundFeeAbsModuleAccountResponse {
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
            type Value = MsgFundFeeAbsModuleAccountResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.feeabs.v1beta1.MsgFundFeeAbsModuleAccountResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<MsgFundFeeAbsModuleAccountResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgFundFeeAbsModuleAccountResponse {})
            }
        }
        deserializer.deserialize_struct(
            "xion.feeabs.v1beta1.MsgFundFeeAbsModuleAccountResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgRemoveHostZone {
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
        if !self.ibc_denom.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("xion.feeabs.v1beta1.MsgRemoveHostZone", len)?;
        if !self.authority.is_empty() {
            struct_ser.serialize_field("authority", &self.authority)?;
        }
        if !self.ibc_denom.is_empty() {
            struct_ser.serialize_field("ibcDenom", &self.ibc_denom)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgRemoveHostZone {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["authority", "ibc_denom", "ibcDenom"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Authority,
            IbcDenom,
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
                            "ibcDenom" | "ibc_denom" => Ok(GeneratedField::IbcDenom),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgRemoveHostZone;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.feeabs.v1beta1.MsgRemoveHostZone")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgRemoveHostZone, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut authority__ = None;
                let mut ibc_denom__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Authority => {
                            if authority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            authority__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IbcDenom => {
                            if ibc_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ibcDenom"));
                            }
                            ibc_denom__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgRemoveHostZone {
                    authority: authority__.unwrap_or_default(),
                    ibc_denom: ibc_denom__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "xion.feeabs.v1beta1.MsgRemoveHostZone",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgRemoveHostZoneResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("xion.feeabs.v1beta1.MsgRemoveHostZoneResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgRemoveHostZoneResponse {
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
            type Value = MsgRemoveHostZoneResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.feeabs.v1beta1.MsgRemoveHostZoneResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<MsgRemoveHostZoneResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgRemoveHostZoneResponse {})
            }
        }
        deserializer.deserialize_struct(
            "xion.feeabs.v1beta1.MsgRemoveHostZoneResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgSendQueryIbcDenomTwap {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sender.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("xion.feeabs.v1beta1.MsgSendQueryIbcDenomTWAP", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgSendQueryIbcDenomTwap {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["sender"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
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
                            "sender" => Ok(GeneratedField::Sender),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgSendQueryIbcDenomTwap;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.feeabs.v1beta1.MsgSendQueryIbcDenomTWAP")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<MsgSendQueryIbcDenomTwap, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgSendQueryIbcDenomTwap {
                    sender: sender__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "xion.feeabs.v1beta1.MsgSendQueryIbcDenomTWAP",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgSendQueryIbcDenomTwapResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct("xion.feeabs.v1beta1.MsgSendQueryIbcDenomTWAPResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgSendQueryIbcDenomTwapResponse {
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
            type Value = MsgSendQueryIbcDenomTwapResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.feeabs.v1beta1.MsgSendQueryIbcDenomTWAPResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<MsgSendQueryIbcDenomTwapResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgSendQueryIbcDenomTwapResponse {})
            }
        }
        deserializer.deserialize_struct(
            "xion.feeabs.v1beta1.MsgSendQueryIbcDenomTWAPResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgSwapCrossChain {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sender.is_empty() {
            len += 1;
        }
        if !self.ibc_denom.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("xion.feeabs.v1beta1.MsgSwapCrossChain", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.ibc_denom.is_empty() {
            struct_ser.serialize_field("ibcDenom", &self.ibc_denom)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgSwapCrossChain {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["sender", "ibc_denom", "ibcDenom"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            IbcDenom,
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
                            "sender" => Ok(GeneratedField::Sender),
                            "ibcDenom" | "ibc_denom" => Ok(GeneratedField::IbcDenom),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgSwapCrossChain;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.feeabs.v1beta1.MsgSwapCrossChain")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgSwapCrossChain, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut ibc_denom__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IbcDenom => {
                            if ibc_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ibcDenom"));
                            }
                            ibc_denom__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgSwapCrossChain {
                    sender: sender__.unwrap_or_default(),
                    ibc_denom: ibc_denom__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "xion.feeabs.v1beta1.MsgSwapCrossChain",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgSwapCrossChainResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("xion.feeabs.v1beta1.MsgSwapCrossChainResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgSwapCrossChainResponse {
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
            type Value = MsgSwapCrossChainResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.feeabs.v1beta1.MsgSwapCrossChainResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<MsgSwapCrossChainResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgSwapCrossChainResponse {})
            }
        }
        deserializer.deserialize_struct(
            "xion.feeabs.v1beta1.MsgSwapCrossChainResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgUpdateHostZone {
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
        if self.host_chain_config.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("xion.feeabs.v1beta1.MsgUpdateHostZone", len)?;
        if !self.authority.is_empty() {
            struct_ser.serialize_field("authority", &self.authority)?;
        }
        if let Some(v) = self.host_chain_config.as_ref() {
            struct_ser.serialize_field("hostChainConfig", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgUpdateHostZone {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["authority", "host_chain_config", "hostChainConfig"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Authority,
            HostChainConfig,
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
                            "hostChainConfig" | "host_chain_config" => {
                                Ok(GeneratedField::HostChainConfig)
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
            type Value = MsgUpdateHostZone;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.feeabs.v1beta1.MsgUpdateHostZone")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgUpdateHostZone, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut authority__ = None;
                let mut host_chain_config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Authority => {
                            if authority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            authority__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HostChainConfig => {
                            if host_chain_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostChainConfig"));
                            }
                            host_chain_config__ = map_.next_value()?;
                        }
                    }
                }
                Ok(MsgUpdateHostZone {
                    authority: authority__.unwrap_or_default(),
                    host_chain_config: host_chain_config__,
                })
            }
        }
        deserializer.deserialize_struct(
            "xion.feeabs.v1beta1.MsgUpdateHostZone",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgUpdateHostZoneResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("xion.feeabs.v1beta1.MsgUpdateHostZoneResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgUpdateHostZoneResponse {
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
            type Value = MsgUpdateHostZoneResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.feeabs.v1beta1.MsgUpdateHostZoneResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<MsgUpdateHostZoneResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUpdateHostZoneResponse {})
            }
        }
        deserializer.deserialize_struct(
            "xion.feeabs.v1beta1.MsgUpdateHostZoneResponse",
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
        let mut struct_ser =
            serializer.serialize_struct("xion.feeabs.v1beta1.MsgUpdateParams", len)?;
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
                formatter.write_str("struct xion.feeabs.v1beta1.MsgUpdateParams")
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
        deserializer.deserialize_struct(
            "xion.feeabs.v1beta1.MsgUpdateParams",
            FIELDS,
            GeneratedVisitor,
        )
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
            serializer.serialize_struct("xion.feeabs.v1beta1.MsgUpdateParamsResponse", len)?;
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
                formatter.write_str("struct xion.feeabs.v1beta1.MsgUpdateParamsResponse")
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
            "xion.feeabs.v1beta1.MsgUpdateParamsResponse",
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
        if !self.native_ibced_in_osmosis.is_empty() {
            len += 1;
        }
        if !self.osmosis_query_twap_path.is_empty() {
            len += 1;
        }
        if !self.chain_name.is_empty() {
            len += 1;
        }
        if !self.ibc_transfer_channel.is_empty() {
            len += 1;
        }
        if !self.ibc_query_icq_channel.is_empty() {
            len += 1;
        }
        if !self.osmosis_crosschain_swap_address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("xion.feeabs.v1beta1.Params", len)?;
        if !self.native_ibced_in_osmosis.is_empty() {
            struct_ser.serialize_field("nativeIbcedInOsmosis", &self.native_ibced_in_osmosis)?;
        }
        if !self.osmosis_query_twap_path.is_empty() {
            struct_ser.serialize_field("osmosisQueryTwapPath", &self.osmosis_query_twap_path)?;
        }
        if !self.chain_name.is_empty() {
            struct_ser.serialize_field("chainName", &self.chain_name)?;
        }
        if !self.ibc_transfer_channel.is_empty() {
            struct_ser.serialize_field("ibcTransferChannel", &self.ibc_transfer_channel)?;
        }
        if !self.ibc_query_icq_channel.is_empty() {
            struct_ser.serialize_field("ibcQueryIcqChannel", &self.ibc_query_icq_channel)?;
        }
        if !self.osmosis_crosschain_swap_address.is_empty() {
            struct_ser.serialize_field(
                "osmosisCrosschainSwapAddress",
                &self.osmosis_crosschain_swap_address,
            )?;
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
            "native_ibced_in_osmosis",
            "nativeIbcedInOsmosis",
            "osmosis_query_twap_path",
            "osmosisQueryTwapPath",
            "chain_name",
            "chainName",
            "ibc_transfer_channel",
            "ibcTransferChannel",
            "ibc_query_icq_channel",
            "ibcQueryIcqChannel",
            "osmosis_crosschain_swap_address",
            "osmosisCrosschainSwapAddress",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NativeIbcedInOsmosis,
            OsmosisQueryTwapPath,
            ChainName,
            IbcTransferChannel,
            IbcQueryIcqChannel,
            OsmosisCrosschainSwapAddress,
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
                            "nativeIbcedInOsmosis" | "native_ibced_in_osmosis" => {
                                Ok(GeneratedField::NativeIbcedInOsmosis)
                            }
                            "osmosisQueryTwapPath" | "osmosis_query_twap_path" => {
                                Ok(GeneratedField::OsmosisQueryTwapPath)
                            }
                            "chainName" | "chain_name" => Ok(GeneratedField::ChainName),
                            "ibcTransferChannel" | "ibc_transfer_channel" => {
                                Ok(GeneratedField::IbcTransferChannel)
                            }
                            "ibcQueryIcqChannel" | "ibc_query_icq_channel" => {
                                Ok(GeneratedField::IbcQueryIcqChannel)
                            }
                            "osmosisCrosschainSwapAddress" | "osmosis_crosschain_swap_address" => {
                                Ok(GeneratedField::OsmosisCrosschainSwapAddress)
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
                formatter.write_str("struct xion.feeabs.v1beta1.Params")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<Params, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut native_ibced_in_osmosis__ = None;
                let mut osmosis_query_twap_path__ = None;
                let mut chain_name__ = None;
                let mut ibc_transfer_channel__ = None;
                let mut ibc_query_icq_channel__ = None;
                let mut osmosis_crosschain_swap_address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NativeIbcedInOsmosis => {
                            if native_ibced_in_osmosis__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "nativeIbcedInOsmosis",
                                ));
                            }
                            native_ibced_in_osmosis__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OsmosisQueryTwapPath => {
                            if osmosis_query_twap_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "osmosisQueryTwapPath",
                                ));
                            }
                            osmosis_query_twap_path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ChainName => {
                            if chain_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainName"));
                            }
                            chain_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IbcTransferChannel => {
                            if ibc_transfer_channel__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "ibcTransferChannel",
                                ));
                            }
                            ibc_transfer_channel__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IbcQueryIcqChannel => {
                            if ibc_query_icq_channel__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "ibcQueryIcqChannel",
                                ));
                            }
                            ibc_query_icq_channel__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OsmosisCrosschainSwapAddress => {
                            if osmosis_crosschain_swap_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "osmosisCrosschainSwapAddress",
                                ));
                            }
                            osmosis_crosschain_swap_address__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Params {
                    native_ibced_in_osmosis: native_ibced_in_osmosis__.unwrap_or_default(),
                    osmosis_query_twap_path: osmosis_query_twap_path__.unwrap_or_default(),
                    chain_name: chain_name__.unwrap_or_default(),
                    ibc_transfer_channel: ibc_transfer_channel__.unwrap_or_default(),
                    ibc_query_icq_channel: ibc_query_icq_channel__.unwrap_or_default(),
                    osmosis_crosschain_swap_address: osmosis_crosschain_swap_address__
                        .unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("xion.feeabs.v1beta1.Params", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryAllHostChainConfigRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct("xion.feeabs.v1beta1.QueryAllHostChainConfigRequest", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryAllHostChainConfigRequest {
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
            type Value = QueryAllHostChainConfigRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.feeabs.v1beta1.QueryAllHostChainConfigRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryAllHostChainConfigRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryAllHostChainConfigRequest {})
            }
        }
        deserializer.deserialize_struct(
            "xion.feeabs.v1beta1.QueryAllHostChainConfigRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryAllHostChainConfigResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.all_host_chain_config.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("xion.feeabs.v1beta1.QueryAllHostChainConfigResponse", len)?;
        if !self.all_host_chain_config.is_empty() {
            struct_ser.serialize_field("allHostChainConfig", &self.all_host_chain_config)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryAllHostChainConfigResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["all_host_chain_config", "allHostChainConfig"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AllHostChainConfig,
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
                            "allHostChainConfig" | "all_host_chain_config" => {
                                Ok(GeneratedField::AllHostChainConfig)
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
            type Value = QueryAllHostChainConfigResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.feeabs.v1beta1.QueryAllHostChainConfigResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryAllHostChainConfigResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut all_host_chain_config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AllHostChainConfig => {
                            if all_host_chain_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "allHostChainConfig",
                                ));
                            }
                            all_host_chain_config__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryAllHostChainConfigResponse {
                    all_host_chain_config: all_host_chain_config__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "xion.feeabs.v1beta1.QueryAllHostChainConfigResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryArithmeticTwapToNowRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.pool_id != 0 {
            len += 1;
        }
        if !self.base_asset.is_empty() {
            len += 1;
        }
        if !self.quote_asset.is_empty() {
            len += 1;
        }
        if self.start_time.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("xion.feeabs.v1beta1.QueryArithmeticTwapToNowRequest", len)?;
        if self.pool_id != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "poolId",
                alloc::string::ToString::to_string(&self.pool_id).as_str(),
            )?;
        }
        if !self.base_asset.is_empty() {
            struct_ser.serialize_field("baseAsset", &self.base_asset)?;
        }
        if !self.quote_asset.is_empty() {
            struct_ser.serialize_field("quoteAsset", &self.quote_asset)?;
        }
        if let Some(v) = self.start_time.as_ref() {
            struct_ser.serialize_field("startTime", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryArithmeticTwapToNowRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pool_id",
            "poolId",
            "base_asset",
            "baseAsset",
            "quote_asset",
            "quoteAsset",
            "start_time",
            "startTime",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PoolId,
            BaseAsset,
            QuoteAsset,
            StartTime,
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
                            "poolId" | "pool_id" => Ok(GeneratedField::PoolId),
                            "baseAsset" | "base_asset" => Ok(GeneratedField::BaseAsset),
                            "quoteAsset" | "quote_asset" => Ok(GeneratedField::QuoteAsset),
                            "startTime" | "start_time" => Ok(GeneratedField::StartTime),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryArithmeticTwapToNowRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.feeabs.v1beta1.QueryArithmeticTwapToNowRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryArithmeticTwapToNowRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut pool_id__ = None;
                let mut base_asset__ = None;
                let mut quote_asset__ = None;
                let mut start_time__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PoolId => {
                            if pool_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("poolId"));
                            }
                            pool_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::BaseAsset => {
                            if base_asset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("baseAsset"));
                            }
                            base_asset__ = Some(map_.next_value()?);
                        }
                        GeneratedField::QuoteAsset => {
                            if quote_asset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quoteAsset"));
                            }
                            quote_asset__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StartTime => {
                            if start_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startTime"));
                            }
                            start_time__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryArithmeticTwapToNowRequest {
                    pool_id: pool_id__.unwrap_or_default(),
                    base_asset: base_asset__.unwrap_or_default(),
                    quote_asset: quote_asset__.unwrap_or_default(),
                    start_time: start_time__,
                })
            }
        }
        deserializer.deserialize_struct(
            "xion.feeabs.v1beta1.QueryArithmeticTwapToNowRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryArithmeticTwapToNowResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.arithmetic_twap.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("xion.feeabs.v1beta1.QueryArithmeticTwapToNowResponse", len)?;
        if !self.arithmetic_twap.is_empty() {
            struct_ser.serialize_field("arithmeticTwap", &self.arithmetic_twap)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryArithmeticTwapToNowResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["arithmetic_twap", "arithmeticTwap"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ArithmeticTwap,
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
                            "arithmeticTwap" | "arithmetic_twap" => {
                                Ok(GeneratedField::ArithmeticTwap)
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
            type Value = QueryArithmeticTwapToNowResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.feeabs.v1beta1.QueryArithmeticTwapToNowResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryArithmeticTwapToNowResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut arithmetic_twap__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ArithmeticTwap => {
                            if arithmetic_twap__.is_some() {
                                return Err(serde::de::Error::duplicate_field("arithmeticTwap"));
                            }
                            arithmetic_twap__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryArithmeticTwapToNowResponse {
                    arithmetic_twap: arithmetic_twap__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "xion.feeabs.v1beta1.QueryArithmeticTwapToNowResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryFeeabsModuleBalancesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct("xion.feeabs.v1beta1.QueryFeeabsModuleBalancesRequest", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryFeeabsModuleBalancesRequest {
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
            type Value = QueryFeeabsModuleBalancesRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.feeabs.v1beta1.QueryFeeabsModuleBalancesRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryFeeabsModuleBalancesRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryFeeabsModuleBalancesRequest {})
            }
        }
        deserializer.deserialize_struct(
            "xion.feeabs.v1beta1.QueryFeeabsModuleBalancesRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryFeeabsModuleBalancesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.balances.is_empty() {
            len += 1;
        }
        if !self.address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("xion.feeabs.v1beta1.QueryFeeabsModuleBalancesResponse", len)?;
        if !self.balances.is_empty() {
            struct_ser.serialize_field("balances", &self.balances)?;
        }
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryFeeabsModuleBalancesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["balances", "address"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Balances,
            Address,
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
                            "balances" => Ok(GeneratedField::Balances),
                            "address" => Ok(GeneratedField::Address),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryFeeabsModuleBalancesResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.feeabs.v1beta1.QueryFeeabsModuleBalancesResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryFeeabsModuleBalancesResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut balances__ = None;
                let mut address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Balances => {
                            if balances__.is_some() {
                                return Err(serde::de::Error::duplicate_field("balances"));
                            }
                            balances__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryFeeabsModuleBalancesResponse {
                    balances: balances__.unwrap_or_default(),
                    address: address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "xion.feeabs.v1beta1.QueryFeeabsModuleBalancesResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryHostChainConfigRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.ibc_denom.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("xion.feeabs.v1beta1.QueryHostChainConfigRequest", len)?;
        if !self.ibc_denom.is_empty() {
            struct_ser.serialize_field("ibcDenom", &self.ibc_denom)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryHostChainConfigRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["ibc_denom", "ibcDenom"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IbcDenom,
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
                            "ibcDenom" | "ibc_denom" => Ok(GeneratedField::IbcDenom),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryHostChainConfigRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.feeabs.v1beta1.QueryHostChainConfigRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryHostChainConfigRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut ibc_denom__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::IbcDenom => {
                            if ibc_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ibcDenom"));
                            }
                            ibc_denom__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryHostChainConfigRequest {
                    ibc_denom: ibc_denom__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "xion.feeabs.v1beta1.QueryHostChainConfigRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryHostChainConfigResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.host_chain_config.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("xion.feeabs.v1beta1.QueryHostChainConfigResponse", len)?;
        if let Some(v) = self.host_chain_config.as_ref() {
            struct_ser.serialize_field("hostChainConfig", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryHostChainConfigResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["host_chain_config", "hostChainConfig"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HostChainConfig,
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
                            "hostChainConfig" | "host_chain_config" => {
                                Ok(GeneratedField::HostChainConfig)
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
            type Value = QueryHostChainConfigResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.feeabs.v1beta1.QueryHostChainConfigResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryHostChainConfigResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut host_chain_config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::HostChainConfig => {
                            if host_chain_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostChainConfig"));
                            }
                            host_chain_config__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryHostChainConfigResponse {
                    host_chain_config: host_chain_config__,
                })
            }
        }
        deserializer.deserialize_struct(
            "xion.feeabs.v1beta1.QueryHostChainConfigResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryOsmosisArithmeticTwapRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.ibc_denom.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("xion.feeabs.v1beta1.QueryOsmosisArithmeticTwapRequest", len)?;
        if !self.ibc_denom.is_empty() {
            struct_ser.serialize_field("ibcDenom", &self.ibc_denom)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryOsmosisArithmeticTwapRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["ibc_denom", "ibcDenom"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IbcDenom,
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
                            "ibcDenom" | "ibc_denom" => Ok(GeneratedField::IbcDenom),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryOsmosisArithmeticTwapRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.feeabs.v1beta1.QueryOsmosisArithmeticTwapRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryOsmosisArithmeticTwapRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut ibc_denom__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::IbcDenom => {
                            if ibc_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ibcDenom"));
                            }
                            ibc_denom__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryOsmosisArithmeticTwapRequest {
                    ibc_denom: ibc_denom__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "xion.feeabs.v1beta1.QueryOsmosisArithmeticTwapRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryOsmosisArithmeticTwapResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.arithmetic_twap.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "xion.feeabs.v1beta1.QueryOsmosisArithmeticTwapResponse",
            len,
        )?;
        if !self.arithmetic_twap.is_empty() {
            struct_ser.serialize_field("arithmeticTwap", &self.arithmetic_twap)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryOsmosisArithmeticTwapResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["arithmetic_twap", "arithmeticTwap"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ArithmeticTwap,
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
                            "arithmeticTwap" | "arithmetic_twap" => {
                                Ok(GeneratedField::ArithmeticTwap)
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
            type Value = QueryOsmosisArithmeticTwapResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.feeabs.v1beta1.QueryOsmosisArithmeticTwapResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryOsmosisArithmeticTwapResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut arithmetic_twap__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ArithmeticTwap => {
                            if arithmetic_twap__.is_some() {
                                return Err(serde::de::Error::duplicate_field("arithmeticTwap"));
                            }
                            arithmetic_twap__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryOsmosisArithmeticTwapResponse {
                    arithmetic_twap: arithmetic_twap__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "xion.feeabs.v1beta1.QueryOsmosisArithmeticTwapResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for SetHostZoneProposal {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if self.host_chain_config.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("xion.feeabs.v1beta1.SetHostZoneProposal", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if let Some(v) = self.host_chain_config.as_ref() {
            struct_ser.serialize_field("hostChainConfig", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for SetHostZoneProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "host_chain_config",
            "hostChainConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            HostChainConfig,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "hostChainConfig" | "host_chain_config" => {
                                Ok(GeneratedField::HostChainConfig)
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
            type Value = SetHostZoneProposal;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct xion.feeabs.v1beta1.SetHostZoneProposal")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<SetHostZoneProposal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut host_chain_config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HostChainConfig => {
                            if host_chain_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostChainConfig"));
                            }
                            host_chain_config__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SetHostZoneProposal {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    host_chain_config: host_chain_config__,
                })
            }
        }
        deserializer.deserialize_struct(
            "xion.feeabs.v1beta1.SetHostZoneProposal",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
