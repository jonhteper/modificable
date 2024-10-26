pub use proc_modificable::Modificable;

#[cfg(feature = "testing")]
#[cfg(test)]
mod tests;

pub trait Modifications {
    fn set_created_now(&mut self);
    fn set_edited_now(&mut self);
    fn set_deleted_now(&mut self);
}

#[cfg(feature = "modifications_impl")]
#[derive(Clone, Debug)]
pub struct ModificationsMetadata<T> {
    created_at: T,
    edited_at: Option<T>,
    deleted_at: Option<T>,
}

#[cfg(feature = "modifications_utc")]
mod utc {
    use crate::{Modifications, ModificationsMetadata};

    impl<T> ModificationsMetadata<T>
    where
        T: Clone + From<chrono::DateTime<chrono::Utc>>,
    {
        pub fn created_now() -> Self {
            Self {
                created_at: T::from(chrono::Utc::now()),
                edited_at: None,
                deleted_at: None,
            }
        }
    }

    impl<T> Modifications for ModificationsMetadata<T>
    where
        T: Clone + From<chrono::DateTime<chrono::Utc>>,
    {
        fn set_created_now(&mut self) {
            self.created_at = T::from(chrono::Utc::now());
        }

        fn set_edited_now(&mut self) {
            self.edited_at = Some(T::from(chrono::Utc::now()));
        }

        fn set_deleted_now(&mut self) {
            self.deleted_at = Some(T::from(chrono::Utc::now()));
        }
    }
}

#[cfg(feature = "modifications_local")]
mod local {
    use crate::{Modifications, ModificationsMetadata};
    use chrono::Local;

    impl<T> ModificationsMetadata<T>
    where
        T: Clone + From<chrono::DateTime<Local>>,
    {
        pub fn created_now() -> Self {
            Self {
                created_at: T::from(Local::now()),
                edited_at: None,
                deleted_at: None,
            }
        }
    }

    impl<T> Modifications for ModificationsMetadata<T>
    where
        T: Clone + From<chrono::DateTime<Local>>,
    {
        fn set_created_now(&mut self) {
            self.created_at = T::from(Local::now());
        }

        fn set_edited_now(&mut self) {
            self.edited_at = Some(T::from(Local::now()));
        }

        fn set_deleted_now(&mut self) {
            self.deleted_at = Some(T::from(Local::now()));
        }
    }
}
