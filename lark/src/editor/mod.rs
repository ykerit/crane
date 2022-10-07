pub mod buffer;
pub mod document;
pub mod graphemes;

use std::collections::BTreeMap;

use self::document::{Document, DocumentId};

pub struct Editor {
    pub documents: BTreeMap<DocumentId, Document>,
}
