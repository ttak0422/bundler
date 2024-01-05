use crate::content::common::Target;

pub trait FromTarget<T>: Sized {
    fn from_target(value: T, target: &Target) -> Self;
}
