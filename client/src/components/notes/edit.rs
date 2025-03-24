use super::Note;
use dioxus::prelude::*;

#[derive(Props, Clone, Debug, PartialEq)]
pub struct NoteEditProps {
    note: Note,
}

#[component]
pub fn NoteEdit(props: NoteEditProps) -> Element {
    let mut is_edit = use_signal(|| false);
    let mut content = use_signal(|| props.note.content);
    let content_html = use_memo(move || markdown::to_html(content().as_str()));

    use_effect(move || {
        if is_edit() {
            document::eval("document.getElementById('edit-note-area').focus()");
        }
    });

    rsx! {
        div {
            id: "note-edit",
            h1 { "{props.note.title}" },
            if is_edit() {
                TextArea {
                    content: content,
                    oninput: move |event: FormEvent| content.set(event.value()),
                    onblur: move |_| is_edit.set(!is_edit())
                }
            } else {
                Markdown {
                    content_html: content_html,
                    ondoubleclick: move |_| is_edit.set(!is_edit())
                }
            },
        },
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
                tracing::warn!("blurred textarea");
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
                tracing::warn!("double clicked body!");
                props.ondoubleclick.call(event)
            }
        },
    }
}
