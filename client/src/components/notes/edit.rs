use super::Note;
use chrono::{DateTime, Utc};
use dioxus::prelude::*;

// #[derive(Props, Clone, Debug, PartialEq)]
// pub struct NoteEditProps {
//     note: Note,
// }
#[component]
pub fn NoteEdit(current_note: Signal<Option<Note>>) -> Element {
    let mut is_edit = use_signal(|| false);
    let content_html = use_memo(move || {
        if let Some(note) = current_note() {
            return note.content;
        } else {
            return "".to_string();
        }
    });

    use_effect(move || {
        if is_edit() {
            document::eval("document.getElementById('edit-note-area').focus()");
        }
    });

    rsx! {
        if let Some(note) = current_note() {
            div {
                id: "note-edit",
                h1 { "{note.title}" },
                if is_edit() {
                    TextArea {
                        content: note.content,
                        oninput: move |event: FormEvent| current_note.set(Some(Note{
                            id: note.id,
                            created: note.created,
                            last_updated: Utc::now(),
                            title: note.title.clone(),
                            content: event.value()
                        })),
                        onblur: move |_| is_edit.set(!is_edit())
                    }
                } else {
                    Markdown {
                        content_html: content_html,
                        ondoubleclick: move |_| is_edit.set(!is_edit())
                    }
                },
            },
        } else
        {
            h1{ "no note selected" }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct TextAreaProps {
    content: String,
    oninput: EventHandler<FormEvent>,
    onblur: EventHandler<FocusEvent>,
}

#[component]
pub fn TextArea(props: TextAreaProps) -> Element {
    rsx! {
        textarea {
            id: "edit-note-area",
            resize: "none",
            value: "{props.content}",
            oninput: move |event| props.oninput.call(event),
            onblur: move |event| {
                props.onblur.call(event)
            }
        }
    }
}
#[derive(Props, PartialEq, Clone)]
pub struct MarkdownProps {
    content_html: String,
    ondoubleclick: EventHandler<MouseEvent>,
}

#[component]
pub fn Markdown(props: MarkdownProps) -> Element {
    rsx! {
        div {
            class: "note-markdown",
            dangerous_inner_html: "{props.content_html}",
            ondoubleclick: move |event| {
                props.ondoubleclick.call(event)
            }
        },
    }
}
