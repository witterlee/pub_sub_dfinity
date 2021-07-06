use ic_cdk::export::{candid::{CandidType}};


#[derive(Debug,CandidType)]
pub enum PubSubError {
    ApplicationError { code: i32, msg: String },
    Unknown
}

// impl CandidType for PubSubError {
//     fn _ty() -> Type {
//         Type::Variant(vec![
//             // Make sure the field id is sorted by idl_hash
//             Field {
//                 id: Label::Named("ApplicationError".to_owned()),
//                 ty: ic_types::Text,
//             },
//             Field {
//                 id: Label::Named("Unknown".to_owned()),
//                 ty: ic_types::Text,
//             },
//         ])
//     }
//     fn idl_serialize<S>(&self, serializer: S) -> Result<(), S::Error>
//     where
//         S: Serializer,
//     {
//         match *self {
//             PubSubError::ApplicationError(ref code,ref msg) => {
//                 serializer.serialize_text(format!("code:{0}, msg {1},",code, msg))
//             }
//             Result::Err(ref e) => {
//                 serializer.serialize_text("Unknown".to_string())
//             }
//         }
//     }
// }
