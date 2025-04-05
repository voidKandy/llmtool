
use super::Note;
use chrono::Duration;
use chrono::{TimeDelta, Utc};
use dioxus::dioxus_core;
use dioxus::html::p;
use dioxus::prelude::*;
use serde_json::error::Category;
use std::collections::HashMap;

// #[derive(Props, Clone, Debug, PartialEq)]
// struct NoteSelectionViewProps {
//     notes: ReadOnlySignal<HashMap<u64, Note>>,
//     current_note: Signal<Option<Note>>,
// }

#[component]
pub fn NotesSelectionList(
    // props: NoteSelectionViewProps,
    categorization: Option<Categorization>,
    cloned_notes: Vec<Note>,
    current_note_id: Signal<Option<u64>>,
) -> Element {
    let mut categorized_notes_map = use_signal(|| HashMap::new());

    use_effect(move || {
        let mut map: HashMap<&'static str, Vec<Note>> = HashMap::new();

        match categorization {
            Some(Categorization::Content) => {}
            Some(Categorization::Description) => {}
            None => {
                cloned_notes.iter().for_each(|n| {
                    let period = TimePeriod::from(n.last_updated);
                    let key: &'static str = period.into();
                    match map.get_mut(&key) {
                        Some(ref mut v) => v.push(n.clone()),
                        None => {
                            let _ = map.insert(key, vec![n.clone()]);
                        }
                    }
                });
            }
        }
        categorized_notes_map.set(map);
    });

    rsx!(
        ul {
        id: "notes-selection-view",
        for (category, notes) in categorized_notes_map().into_iter() {
            div {
                key: "{category}",
                class: "categorized-notes-selection",
                h1 {"{category}"},
                ul {
                    for note in notes.into_iter(){
                        li {
                            key: "note_{note.id}",
                            class:"note-list-item",
                            button {
                                class: "note-selector-button",
                                class: if current_note_id().is_some_and(|selected| selected == note.id) {
                                    "selected"
                                } else {
                                    ""
                                },
                                onclick: move |_| current_note_id.set(Some(note.id)),
                                "{note.title}",
                            }
                        },
                        }
                    }
            }
        }
    })

}

#[derive(Clone, Debug, PartialEq)]
pub enum Categorization {
    Content,
    Description,
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


pub fn generate_mock_notes() -> super::CachedNotes {
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

    let mut count = 0;
    // Function to create notes based on specific periods
    let mut create_notes_for_period = |period: TimePeriod, amt: usize| -> Vec<Note> {
        let period_duration = match period {
            TimePeriod::Today => Duration::minutes(0),
            TimePeriod::LastWeek => Duration::days(7),
            TimePeriod::LastMonth => Duration::days(30),
            TimePeriod::Older => Duration::days(365),
        };

        (0..amt)
            .map(|index| {
                let mut n = Note::create(mock_info[count + index].0, mock_info[count + index].1);
                count += 1;
                let last_updated = n.created - period_duration;
                n.last_updated = last_updated;
                n
            })
            .collect()
    };

    let amt = mock_info.len() / 4;
    // Generate notes for each period, 5 notes per period
    let mut mock_notes = HashMap::new();
    let mut all = create_notes_for_period(TimePeriod::Today, amt);
    all.extend(create_notes_for_period(TimePeriod::LastWeek, amt));
    all.extend(create_notes_for_period(TimePeriod::LastMonth, amt));
    all.extend(create_notes_for_period(TimePeriod::Older, amt));

    for n in all {
        mock_notes.insert(n.id, n);
    }

    mock_notes
}
