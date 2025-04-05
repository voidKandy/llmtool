pub mod edit;
pub mod list;
use chrono::{Duration, TimeDelta, Utc};
use dioxus::prelude::*;
use edit::NoteEdit;

use list::NotesSelectionList;

use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Note {
    /// hashed content
    pub id: u64,
    pub created: chrono::DateTime<Utc>,
    pub last_updated: chrono::DateTime<Utc>,
    pub title: String,
    pub content: String,
    // we should create something called a `NoteProp` instead of adding embeddings
    // `Note` could be in the shared lib and `NoteProp` will be in this crate
    // `NoteProp` will not contain embeddings, as they are too expensive
    // > I should look into using props with lifetimes in dioxus
    // embedding
    // etc..
}

impl Note {
    pub fn create(title: &str, content: &str) -> Self {
        let now = Utc::now();
        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        Self {
            id: hasher.finish(),
            created: now,
            last_updated: now,
            title: title.to_string(),
            content: content.to_string(),
        }
    }
}

pub type CachedNotes = HashMap<u64, Note>;
const NOTE_STYLES: Asset = asset!("/assets/styles/notes.css");

const NOTES_STORAGE: &str = "notes";

#[component]
pub fn NotesViewComponent() -> Element {
    let cached_notes = dioxus_sdk::storage::new_persistent(NOTES_STORAGE, || {
        tracing::warn!("generating");
        list::generate_mock_notes()
    });

    let current_note_id: Signal<Option<u64>> = use_signal(|| None);
    let current_note: Memo<Option<Note>> =
        use_memo(move || current_note_id().and_then(|id| cached_notes().get(&id).cloned()));

    let mut cloned_titles: Signal<Vec<Note>> = use_signal(|| vec![]);

    use_effect(move || {
        cloned_titles.set(
            cached_notes()
                .iter()
                .map(|(_, note)| note.to_owned())
                .collect::<Vec<Note>>(),
        );
    });

    rsx!(
        document::Link { rel: "stylesheet", href: NOTE_STYLES },
        div  {
            id: "notes-view",

            NotesSelectionList{
                // categorization: None,
                cloned_notes: cloned_titles(),
                current_note_id: current_note_id,
            }
            NoteEdit {
                current_note: current_note
            }

        }

    )
}
