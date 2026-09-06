//! # Ferrox DataGrid (`ferrox-datagrid`)
//!
//! `ferrox-datagrid` bridges enterprise frontend table libraries (AG-Grid, MUI X DataGrid, TanStack Table) with Ferrox backend queries.
//! It automatically parses HTTP query strings into structured sort, filter, and page models ready for database translation.
//!
//! ## Key Features
//! - 📊 **AG-Grid Translator**: Parses AG-Grid filter models and multi-column sorting parameters.
//! - ⚛️ **MUI X & TanStack Support**: Seamless query parameter parsing for React/Angular grid components.
//! - 🛡️ **SQL Injection Safe**: Converts frontend filters into safe parametric database conditions.

pub mod models;

#[cfg(feature = "aggrid")]
pub mod parsers;

#[cfg(feature = "seaorm")]
pub mod translators;

pub use models::*;