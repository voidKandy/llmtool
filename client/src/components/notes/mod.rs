pub mod list;
pub mod selector;
use chrono::{Duration, TimeDelta, Utc};
use dioxus::prelude::*;
use selector::NoteSelector;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Note {
    pub id: String,
    pub created: chrono::DateTime<Utc>,
    pub last_updated: chrono::DateTime<Utc>,
    pub title: String,
    pub content: String,
    // embedding
    // etc..
}

impl Note {
    fn create(id: &str, title: &str, content: &str) -> Self {
        let now = Utc::now();
        // fine for now
        Self {
            id: id.to_string(),
            created: now,
            last_updated: now,
            title: title.to_string(),
            content: content.to_string(),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum TimePeriod {
    Today,
    LastWeek,
    LastMonth,
    Older,
}

impl Into<&'static str> for TimePeriod {
    fn into(self) -> &'static str {
        match self {
            Self::Today => "Today",
            Self::LastWeek => "Last 7 Days",
            Self::LastMonth => "Last 30 Days",
            Self::Older => "Older",
        }
    }
}

impl From<chrono::DateTime<Utc>> for TimePeriod {
    fn from(value: chrono::DateTime<Utc>) -> Self {
        let now = Utc::now();
        let one_day = TimeDelta::hours(24);
        let one_week = TimeDelta::days(7);
        let one_month = TimeDelta::days(30);
        let since = now.signed_duration_since(value);

        match since {
            _ if since <= one_day => Self::Today,
            _ if since <= one_week => Self::LastWeek,
            _ if since <= one_month => Self::LastMonth,
            _ => Self::Older,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Categorization {
    Content,
    Description,
}

#[derive(Props, Clone, Debug, PartialEq)]
pub struct NotesViewProps {
    categorization: Option<Categorization>,
}

pub const NOTE_STYLES: Asset = asset!("/assets/styles/notes.css");

#[component]
pub fn NotesViewComponent(props: NotesViewProps) -> Element {
    let cached_notes = dioxus_sdk::storage::use_persistent::<Vec<Note>>("notes", || {
        tracing::warn!("generating");
        generate_mock_notes()
    });

    let mut categorized_notes_map = HashMap::<&'static str, Vec<Note>>::new();
    match props.categorization {
        Some(Categorization::Content) => {}
        Some(Categorization::Description) => {}
        None => {
            cached_notes.iter().for_each(|n| {
                let period = TimePeriod::from(n.last_updated);
                let key: &'static str = period.into();
                match categorized_notes_map.get_mut(&key) {
                    Some(ref mut v) => v.push(n.clone()),
                    None => {
                        let _ = categorized_notes_map.insert(key, vec![n.clone()]);
                    }
                }
            });
        }
    }
    warn!("categorized: {categorized_notes_map}");
    let mut current_note = use_signal::<Option<Note>>(|| None);
    rsx!(
        document::Link { rel: "stylesheet", href: NOTE_STYLES },

        div  {
            id: "notes",
            div  {
                id: "notes-selection-container",
                for (category, notes) in categorized_notes_map.into_iter() {
                    div {
                        id: "{category}",
                        class: "categorized-notes-selection",
                        h1 {"{category}"}
                        for note in notes.into_iter() {
                            NoteSelector {
                                note: note.clone(),
                                selected: current_note.read().as_ref().is_some_and(|n| n == n),
                                onclick: move |_| {
                                    tracing::warn!("clicked!\n{note:#?}");
                                    current_note.set(Some(note.clone()));
                                },
                            }
                        }
                    },
                }
            },

            div {
                id: "note-view",
                if let Some(note) = current_note.read().as_ref() {
                    h1 {"{note.title}"}
                    p {"{note.content}"}
                } else {
                    h1{ "not note selected" }
                }
            },
        }

    )
}

pub fn generate_mock_notes() -> Vec<Note> {
    let mock_info = vec![
        (
            "Meeting Notes",
            "Discussed project roadmap and key deadlines.",
        ),
        (
            "Project Update",
            "Updated the team on the latest features added.",
        ),
        (
            "Shopping List",
            "Milk, eggs, bread, and some fresh vegetables.",
        ),
        (
            "Daily Journal",
            "Wrote about my experiences today and reflections.",
        ),
        ("Workout Plan", "Planned a full-body workout for the week."),
        ("Recipe Ideas", "Experimenting with a new pasta recipe."),
        (
            "Coding Thoughts",
            "Thinking about optimizing the Redux store.",
        ),
        ("Travel Itinerary", "Booked flights for the upcoming trip."),
        (
            "Book Summary",
            "Summarized key insights from the book I read.",
        ),
        ("Music Playlist", "Compiled a list of favorite rock songs."),
        (
            "Business Strategy",
            "Outlined marketing strategies for next quarter.",
        ),
        (
            "Movie Watchlist",
            "Added some classic films to my watchlist.",
        ),
        (
            "Personal Goals",
            "Set personal and career goals for the year.",
        ),
        (
            "Ideas & Inspiration",
            "Captured new creative ideas for future projects.",
        ),
        (
            "Random Thoughts",
            "Just a collection of random musings and thoughts.",
        ),
    ];

    let mut id = 0;
    // Function to create notes based on specific periods
    let mut create_notes_for_period = |period: TimePeriod, count: usize| -> Vec<Note> {
        let period_duration = match period {
            TimePeriod::Today => Duration::minutes(0),
            TimePeriod::LastWeek => Duration::days(7),
            TimePeriod::LastMonth => Duration::days(30),
            TimePeriod::Older => Duration::days(365),
        };

        (0..count)
            .map(|index| {
                let mut n = Note::create(
                    id.to_string().as_str(),
                    mock_info[id + index].0,
                    mock_info[id + index].1,
                );
                id += 1;
                let last_updated = n.created - period_duration;
                n.last_updated = last_updated;
                n
            })
            .collect()
    };

    let amt = mock_info.len() / 4;
    // Generate notes for each period, 5 notes per period
    let mut mock_notes: Vec<Note> = Vec::new();
    mock_notes.extend(create_notes_for_period(TimePeriod::Today, amt));
    mock_notes.extend(create_notes_for_period(TimePeriod::LastWeek, amt));
    mock_notes.extend(create_notes_for_period(TimePeriod::LastMonth, amt));
    mock_notes.extend(create_notes_for_period(TimePeriod::Older, amt));

    mock_notes
}
