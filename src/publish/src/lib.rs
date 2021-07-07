use futures::channel::oneshot;
use futures::stream::{self, StreamExt};

use ic_cdk::export::{
    candid::{CandidType, Deserialize},
    Principal,
};
use ic_cdk::storage;
use ic_cdk_macros::*;
use serde::Serialize;
use std::collections::BTreeMap;

type SubscriberStore = BTreeMap<Principal, Subscriber>;

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
struct TradeHistory {
    price: u128,
    decimals: u8,
    side: String,
    maker: Principal,
    taker: Principal,
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

#[update]
fn subscribe(subscriber: Subscriber) -> String {
    ic_cdk::print(format!("calling publish subscribe method!"));
    let subscriber_principal_id = ic_cdk::caller();
    let subscriber_store = storage::get_mut::<SubscriberStore>();
    let exist_subscriber = subscriber_store.contains_key(&subscriber_principal_id);
    if exist_subscriber {
        ic_cdk::print(format!("{} has subscribed!", &subscriber_principal_id));
        "subscribed".to_string()
    } else {
        ic_cdk::print(format!(
            "subscriber ({0}) before insert! {1} subscriber now",
            &subscriber_principal_id,
            subscriber_store.len()
        ));

        subscriber_store.insert(subscriber_principal_id, subscriber);
        ic_cdk::print(format!(
            "subscriber inserted!{ } subscriber now",
            subscriber_store.len()
        ));
        "succeed".to_string()
    }
}

#[update]
async fn generate_random_trade() {
    let subscriber_store = storage::get_mut::<SubscriberStore>();
    let subscriber_ids: Vec<Principal> = subscriber_store.keys().cloned().collect();
    
    stream::iter(subscriber_ids)
        .for_each_concurrent(/* limit */ 10, |k| async move{
            let trade = TradeHistory {
                price: 1000u128,
                decimals: 4u8,
                side: "sell".to_string(),
                maker: Principal::anonymous(),
                taker: Principal::anonymous(),
                timestamp: 1625488881u64,
            };
            let _call_result: Result<(String,), _> =
                ic_cdk::api::call::call(k, "notify", (trade,)).await;
        })
        .await;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
