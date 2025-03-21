// use std::sync::{Arc, RwLock};

// pub type SharedState = Arc<RwLock<State>>;
// pub struct State {
//     pub db: tinydb::Database<DBType>,
// }

// const DBNAME: &str = "test";
// impl State {
//     pub fn init() -> Self {
//         let db = tinydb::Database::<DBType>::new(DBNAME, None, true);
//         Self { db }
//     }
// }

// #[derive(Debug, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
// pub enum DBType {
//     Some(Vec<Vec<String>>),
// }
// impl From<Vec<Vec<f32>>> for DBType {
//     fn from(value: Vec<Vec<f32>>) -> Self {
//         let mut out: Vec<Vec<String>> = vec![];
//         for vec in value {
//             out.push(
//                 vec.into_iter()
//                     .map(|fl| {
//                         let mut buffer = ryu::Buffer::new();
//                         let printed = buffer.format(fl);
//                         printed.to_string()
//                     })
//                     .collect(),
//             );
//         }
//         Self::Some(out)
//     }
// }
