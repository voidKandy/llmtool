use super::{CachedNotes, Note, NOTES_STORAGE};
use dioxus::prelude::*;
use dioxus_sdk::storage::use_persistent;
use global_attributes::dangerous_inner_html;

#[derive(Props, Clone, Debug, PartialEq)]
pub struct NoteEditProps {
    note: Note,
}

#[component]
pub fn NoteEdit(props: NoteEditProps) -> Element {
    let mut is_edit_sig = use_signal(|| false);
    // let content_html = markdown::to_html(&props.note.content);
    let content_html = markdown::to_html(
        "# Header\n*italic*\n**bold**\n> sidenote\n\n### smaller header\n`something`\n",
    );
    let is_edit: bool = is_edit_sig.read().clone();
    use_effect(move || {
        if is_edit {
            document::eval("document.getElementById('edit-note-area').focus()");
        }
    });

    rsx!(
         div {
           id: "note-edit",
           h1 { "{props.note.title}" },
           if is_edit {
                 textarea {
                     id: "edit-note-area",
                     onblur: move |_| {
                         tracing::warn!("blurred textarea");
                         let mut w = is_edit_sig.write();
                         *w = false;
                         drop(w);
                     },
                     resize: "none",
                     "{props.note.content}",
                 }
             } else {
                 div {
                     class: "note-markdown",
                     ondoubleclick: move |_| {
                         tracing::warn!("double clicked body!");
                         let mut w = is_edit_sig.write();
                         *w = true;
                         drop(w);
                     },
                     dangerous_inner_html: "{content_html}"
                 },
             },
    },
     )
}
