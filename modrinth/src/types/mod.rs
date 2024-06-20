pub mod project;
pub mod query;
pub mod version;

#[allow(clippy::trivially_copy_pass_by_ref)]
pub(in crate::types) fn is_zero(num: &u8) -> bool {
    *num == 0
}