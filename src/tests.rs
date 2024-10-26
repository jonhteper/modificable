use super::*;
use chrono::{DateTime, Utc};
use derived::Ctor;

#[derive(Modificable, Ctor)]
pub struct User {
    _id: u8,

    #[setter]
    name: String,

    #[modifications]
    modifications: UserModifications,
}

struct UserModifications {
    created_at: DateTime<Utc>,
    edited_at: Option<DateTime<Utc>>,
    deleted_at: Option<DateTime<Utc>>,
}

impl UserModifications {
    fn created_now() -> Self {
        Self {
            created_at: Utc::now(),
            edited_at: None,
            deleted_at: None,
        }
    }
}

impl Modifications for UserModifications {
    fn set_created_now(&mut self) {
        self.created_at = Utc::now();
    }

    fn set_edited_now(&mut self) {
        self.edited_at = Some(Utc::now());
    }

    fn set_deleted_now(&mut self) {
        self.deleted_at = Some(Utc::now());
    }
}

#[test]
fn change_name_works() {
    let mut user = User::new(1, "John".to_string(), UserModifications::created_now());
    user.set_name("Johnny".to_string());

    assert!(user.modifications.edited_at.is_some());
}
