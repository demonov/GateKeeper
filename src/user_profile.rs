use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserProfile {
    id: i64,
    is_admin: bool,
    status: UserStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum UserStatus {
    Unknown,
    RegistrationInProgress(Vec<RegQuestion>),
    Registered,
    Banned,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegQuestion {
    answered: bool,
    question_text: String,
    reply: Reply,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Reply {
    Text(Vec<String>),
    TextWithMedia(Vec<String>, Vec<String>)
}