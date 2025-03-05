use std::{collections::HashMap, time::Instant};

use chrono::{Duration, TimeDelta, Utc};
use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Note {
    id: String,
    created: chrono::DateTime<Utc>,
    last_updated: chrono::DateTime<Utc>,
    title: String,
    content: String,
    // embedding
    // etc..
}
impl Note {
    fn create(title: &str, content: &str) -> Self {
        let now = Utc::now();
        // fine for now
        let id = now.to_string();
        Self {
            id,
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
pub struct NotesProps {
    notes: Vec<Note>,
    categorization: Option<Categorization>,
}

#[component]
pub fn NotesComponent(mut props: NotesProps) -> Element {
    // tracing::warn!("props for notes component: {props:#?}");
    props
        .notes
        .sort_by(|a, b| a.last_updated.cmp(&b.last_updated));
    let mut categorized_notes_map = HashMap::<&'static str, Vec<&Note>>::new();
    match props.categorization {
        Some(Categorization::Content) => {}
        Some(Categorization::Description) => {}
        None => {
            props.notes.iter().for_each(|n| {
                let period = TimePeriod::from(n.last_updated);
                let key: &'static str = period.into();
                match categorized_notes_map.get_mut(&key) {
                    Some(ref mut v) => v.push(n),
                    None => {
                        let _ = categorized_notes_map.insert(key, vec![n]);
                    }
                }
            });
        }
    }
    // tracing::warn!("categorized notes : {categorized_notes_map:#?}");

    rsx!(
        div  {
            id: "notes",
            for (category, notes) in categorized_notes_map.iter() {
                div {
                    id: "{category}",
                    h1 {"{category}"}
                    for note in notes.iter() {
                        div {
                            id: note.id.as_ref(),
                            h2 {"{note.title}"}
                            p {"{note.content}"}
                        }
                    }
                }

            }
        }
    )
}

pub fn generate_mock_notes() -> Vec<Note> {
    let mock_titles = vec![
        "Meeting Notes",
        "Project Update",
        "Shopping List",
        "Daily Journal",
        "Workout Plan",
        "Recipe Ideas",
        "Coding Thoughts",
        "Travel Itinerary",
        "Book Summary",
        "Music Playlist",
        "Business Strategy",
        "Movie Watchlist",
        "Personal Goals",
        "Ideas & Inspiration",
        "Random Thoughts",
    ];

    let mock_contents = vec![
        "Discussed project roadmap and key deadlines.",
        "Updated the team on the latest features added.",
        "Milk, eggs, bread, and some fresh vegetables.",
        "Wrote about my experiences today and reflections.",
        "Planned a full-body workout for the week.",
        "Experimenting with a new pasta recipe.",
        "Thinking about optimizing the Redux store.",
        "Booked flights for the upcoming trip.",
        "Summarized key insights from the book I read.",
        "Compiled a list of favorite rock songs.",
        "Outlined marketing strategies for next quarter.",
        "Added some classic films to my watchlist.",
        "Set personal and career goals for the year.",
        "Captured new creative ideas for future projects.",
        "Just a collection of random musings and thoughts.",
    ];

    // Function to create notes based on specific periods
    let create_notes_for_period = |period: TimePeriod, count: usize| -> Vec<Note> {
        let period_duration = match period {
            TimePeriod::Today => Duration::minutes(0),
            TimePeriod::LastWeek => Duration::days(7),
            TimePeriod::LastMonth => Duration::days(30),
            TimePeriod::Older => Duration::days(365),
        };

        (0..count)
            .map(|index| {
                let mut n = Note::create(mock_titles[index], mock_contents[index]);
                let last_updated = n.created - period_duration;
                n.last_updated = last_updated;
                n
            })
            .collect()
    };

    // Generate notes for each period, 5 notes per period
    let mut mock_notes: Vec<Note> = Vec::new();
    mock_notes.extend(create_notes_for_period(TimePeriod::Today, 5));
    mock_notes.extend(create_notes_for_period(TimePeriod::LastWeek, 5));
    mock_notes.extend(create_notes_for_period(TimePeriod::LastMonth, 5));
    mock_notes.extend(create_notes_for_period(TimePeriod::Older, 5));

    mock_notes
}
