use ic_cdk::export::{
    candid::{CandidType, Deserialize},
    Principal,
};
use ic_cdk_macros::*;
use serde::Serialize;

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
struct TradeHistory {
    price: u128,
    decimals: u8,
    side: String,
    maker: Option<Principal>,
    taker: Option<Principal>,
    timestamp: u64,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
struct Subscriber {
    price: u128,
    decimals: u8,
    operator: String,
    side: String,
    maker: Option<Principal>,
    taker: Option<Principal>,
}

static mut PUBLISH_CANISTER_ID: Option<Principal> = None;

#[update]
async fn setup_subscribe(publish_id: Principal) -> String {
    ic_cdk::print("setup_subscribe called step 1");
    unsafe {
        PUBLISH_CANISTER_ID = Some(publish_id);
        let subscriber = Subscriber {
            price: 100000u128,
            decimals: 2u8,
            operator: ">".to_string(),
            side: "both".to_string(),
            maker: None,
            taker: None,
        };

        ic_cdk::print("setup_subscribe called step 2");
        let _call_result: Result<(String,), _> =
            ic_cdk::api::call::call(publish_id, "subscribe", (subscriber,)).await;

        ic_cdk::print(format!("setup_subscribe call finished"));
        //"ok".to_string()
        format!("result is  {}", _call_result.unwrap().0)
    }
}

#[update]
fn notify(trade: TradeHistory) ->i32{ 
    for x in 1..9 { 
            ic_cdk::print(format!(
                "{3}: subscriber {2} trade price {0},timestamp {1} ",
                trade.price,
                trade.timestamp,
                ic_cdk::id(),
                x
            ));
    }
    1
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
