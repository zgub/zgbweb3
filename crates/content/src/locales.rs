//! The Fluent catalogs, compiled in (D-013, D-032). The app turns them into
//! bundles; nothing is fetched or served at run time.

pub const EN: &str = include_str!("../locales/en.ftl");
pub const SK: &str = include_str!("../locales/sk.ftl");
