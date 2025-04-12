pub mod edit;
pub mod list;
use chrono::{Duration, TimeDelta, Utc};
use dioxus::{html::li, prelude::*};
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
    let mut notes_list: Signal<Vec<Note>> = use_signal(|| vec![]);
    let current_note: Signal<Option<Note>> = use_signal(|| None);

    use_effect(move || {
        notes_list.set(
            cached_notes()
                .iter()
                .map(|(_, note)| note.to_owned())
                .collect::<Vec<Note>>(),
        );
    });

    use_effect(move || {
        if let Some(note) = current_note() {
            let edited_list = notes_list
                .peek()
                .iter()
                .map(|note_iter| {
                    if note_iter.id == note.id {
                        note.clone()
                    } else {
                        note_iter.clone()
                    }
                })
                .collect::<Vec<Note>>();
            notes_list.set(edited_list);
        };
    });

    rsx!(
        document::Link { rel: "stylesheet", href: NOTE_STYLES },
        div  {
            id: "notes-view",
            NotesSelectionList{
                // categorization: None,
                notes: notes_list,
                current_note: current_note,
            }
            NoteEdit {
                current_note: current_note
            }
        }
    )
}
