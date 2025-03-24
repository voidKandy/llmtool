pub mod edit;
pub mod list;
use chrono::{Duration, TimeDelta, Utc};
use dioxus::prelude::*;
use edit::NoteEdit;
use list::NotesSelectionView;
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
    fn create(title: &str, content: &str) -> Self {
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

pub const NOTES_STORAGE: &str = "notes";
#[derive(Props, Clone, Debug, PartialEq)]
pub struct NotesViewProps {}
#[component]
pub fn NotesViewComponent(props: NotesViewProps) -> Element {
    let cached_notes = dioxus_sdk::storage::new_persistent(NOTES_STORAGE, || {
        tracing::warn!("generating");
        list::generate_mock_notes()
    });
    let mut current_note: Signal<Option<Note>> = use_signal(|| None);

    // let current_note_id: Signal<Option<u64>> = use_signal(|| Option::<u64>::None);
    // //title content
    // let current_note: Option<Note> = cached_notes
    //     .read()
    //     // im assuming 0 wont return a note id so this should be changed later
    //     .get(&current_note_id.read().clone().unwrap_or(0))
    //     .cloned();

    rsx!(
        document::Link { rel: "stylesheet", href: NOTE_STYLES },
        div  {
            id: "notes-view",
            NotesSelectionView{
                notes: cached_notes,
                current_note: current_note,
            }
            if let Some(note) = current_note() {
                NoteEdit{ note: note }
            } else
            {
                h1{ "no note selected" }
            }
        }

    )
}
