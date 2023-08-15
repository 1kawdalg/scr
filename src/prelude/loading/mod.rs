//! Load anything <i>(now only files)</i>
pub mod file;

use std::{fs::File, io::{Cursor, copy} };
use crate::prelude::{ rb, FileType };