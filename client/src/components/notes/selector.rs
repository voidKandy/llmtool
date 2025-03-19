use chrono::{Duration, TimeDelta, Utc};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::Note;

#[derive(Props, Clone, Debug, PartialEq)]
pub struct NoteSelectorViewProps {
    note: Note,
    selected: bool,
    onclick: EventHandler<MouseEvent>,
}

#[component]
pub fn NoteSelector(props: NoteSelectorViewProps) -> Element {
    let class = format!(
        "note-selection-button {}",
        if props.selected { "selected" } else { "" }
    );
    rsx!(
    button {
        id: "note_{props.note.id}",
        class: class,
        onclick:props.onclick,
        "{props.note.title}",
     }
    )
}
