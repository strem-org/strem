//! # Spatio-Temporal Regular Expression Matcher
//!
//! The Spatio-Temporal Regular Expression Matching (STREM) tool is a
//! command-line tool that provides pattern matching against annotated perception
//! datastreams through the use of Spatial Regular Expressions (SpREs).

pub mod compiler;
pub mod config;
pub mod controller;
pub mod datastream;
pub mod matcher;
pub mod monitor;
pub mod symbolizer;
