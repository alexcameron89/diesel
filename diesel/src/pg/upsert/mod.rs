mod on_conflict_actions;
mod on_conflict_clause;
mod on_conflict_extension;
mod on_conflict_target;

pub use self::on_conflict_actions::{do_nothing, do_update, excluded};
#[allow(deprecated)]
#[cfg(feature = "with-deprecated")]
pub use self::on_conflict_extension::DeprecatedOnConflictExtension;
// pub use self::on_conflict_extension::OnConflictExtension;
pub use self::on_conflict_target::on_constraint;
