use derive_builder::Builder;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::api::{authenticated::orders::types::OrderRaw, endpoint::Endpoint};

use super::types::Order;

#[derive(Debug, Builder, Serialize)]
pub struct CancelOrder {
    id: u64,
}

impl CancelOrder {
    pub fn builder() -> CancelOrderBuilder {
        CancelOrderBuilder::default()
    }
}

impl Endpoint for CancelOrder {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> String {
        String::from("v2/auth/w/order/cancel")
    }

    fn is_authenticated(&self) -> bool {
        true
    }

    fn body(&self) -> Option<(&'static str, Vec<u8>)> {
        let body = serde_json::to_string(self).unwrap();
        Some(("application/json", body.into_bytes()))
    }
}

#[derive(Debug)]
pub struct CancelOrderResp {
    pub mts: u64,
    pub ty: String,
    pub message_id: u64,
    pub order: Order,
    pub code: u64,
    pub status: String,
    pub text: String,
}

impl<'de> Deserialize<'de> for CancelOrderResp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct CancelOrderRawResp(u64, String, u64, Option<()>, OrderRaw, u64, String, String);

        impl From<CancelOrderRawResp> for CancelOrderResp {
            fn from(value: CancelOrderRawResp) -> Self {
                let CancelOrderRawResp(mts, ty, message_id, _, order, code, status, text) = value;

                Self {
                    mts,
                    ty,
                    message_id,
                    order: order.into(),
                    code,
                    status,
                    text,
                }
            }
        }

        let raw = CancelOrderRawResp::deserialize(deserializer)?;
        Ok(raw.into())
    }
}
