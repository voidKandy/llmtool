pub mod list;
use chrono::{Duration, TimeDelta, Utc};
use dioxus::prelude::*;
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

#[derive(Props, Clone, Debug, PartialEq)]
pub struct NotesViewProps {}
#[component]
pub fn NotesViewComponent(props: NotesViewProps) -> Element {
    let cached_notes = dioxus_sdk::storage::new_persistent("notes", || {
        tracing::warn!("generating");
        list::generate_mock_notes()
    });
    let current_note_id: Signal<Option<u64>> = use_signal(|| Option::<u64>::None);
    //title content
    let current_note_info: Option<(String, String)> = cached_notes
        .read()
        // im assuming 0 wont return a note id so this should be changed later
        .get(&current_note_id.read().clone().unwrap_or(0))
        .as_ref()
        .and_then(|n| Some((n.title.to_owned(), n.content.to_owned())));

    rsx!(
        document::Link { rel: "stylesheet", href: NOTE_STYLES },
        div  {
            id: "notes",
            NotesSelectionView{
                notes: cached_notes,
                 current_note_id: current_note_id
             }
        div {
            id: "note-view",
            if let Some((title, content)) = current_note_info {
                h1 {"{title}"}
                p {"{content}"}
            } else {
                h1{ "not note selected" }
            }
        },
        }

    )
}

// #[derive(Props, Clone, Debug, PartialEq)]
// pub struct NotesViewProps {}
// #[component]
// pub fn NotesViewComponent(props: NotesViewProps) -> Element {}
