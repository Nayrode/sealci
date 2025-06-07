// use std::fmt;
// 
// #[allow(dead_code)]
// pub enum Status {
//     Running,
//     Stopped,
//     Error(String),
//     Booting,
// }
// 
// impl fmt::Display for Status {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Status::Running => write!(f, "Running"),
//             Status::Stopped => write!(f, "Stopped"),
//             Status::Error(msg) => write!(f, "Error: {}", msg),
//             Status::Booting => write!(f, "Booting"),
//         }
//     }
// }