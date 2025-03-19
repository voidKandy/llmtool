use super::{CachedNotes, Note, NOTES_STORAGE};
use dioxus::prelude::*;
use dioxus_sdk::storage::use_persistent;

#[derive(Props, Clone, Debug, PartialEq)]
pub struct NoteEditProps {
    note: Signal<Note>,
}
#[component]
pub fn NoteEditComponent(props: NoteEditProps) -> Element {
    let is_edit = use_signal(|| false);
    rsx!()
}
