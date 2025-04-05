use crate::components::notes::{list::Categorization, Note};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EmbeddingVec<const N: usize>([f32; N]);

impl<const N: usize> From<[f32; N]> for EmbeddingVec<N> {
    fn from(value: [f32; N]) -> Self {
        Self(value)
    }
}
impl<const N: usize> From<Vec<f32>> for EmbeddingVec<N> {
    fn from(value: Vec<f32>) -> Self {
        assert!(value.len() == N, "Vector is wrong size");
        let mut vec = [0.0; N];
        for i in 0..N {
            let val = value[i];
            vec[i] = val;
        }
        Self(vec)
    }
}

impl<const N: usize> std::ops::MulAssign<f32> for EmbeddingVec<N> {
    fn mul_assign(&mut self, rhs: f32) {
        self.0.iter_mut().for_each(|x| *x *= rhs);
    }
}
impl<const N: usize> std::ops::MulAssign<EmbeddingVec<N>> for EmbeddingVec<N> {
    fn mul_assign(&mut self, rhs: EmbeddingVec<N>) {
        self.0
            .iter_mut()
            .enumerate()
            .for_each(|(i, x)| *x *= rhs.0[i]);
    }
}

impl<const N: usize> std::ops::AddAssign<f32> for EmbeddingVec<N> {
    fn add_assign(&mut self, rhs: f32) {
        self.0.iter_mut().for_each(|x| *x += rhs);
    }
}

impl<const N: usize> std::ops::AddAssign<EmbeddingVec<N>> for EmbeddingVec<N> {
    fn add_assign(&mut self, rhs: EmbeddingVec<N>) {
        self.0
            .iter_mut()
            .enumerate()
            .for_each(|(i, x)| *x += rhs.0[i]);
    }
}

impl<const N: usize> std::ops::Mul<f32> for EmbeddingVec<N> {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        let mut vec = self;
        vec *= rhs;
        vec
    }
}

impl<const N: usize> EmbeddingVec<N> {
    fn euclidean_distance(&self, other: &Self) -> f32 {
        self.0
            .iter()
            .zip(other.0.iter())
            .map(|(x, y)| (x - y).powi(2))
            .sum::<f32>()
            .sqrt()
    }

    /// https://numpy.org/doc/2.1/reference/generated/numpy.average.html
    fn weighted_average(vectors: &[&Self], weights: Option<&[f32; N]>) -> Self {
        assert!(!vectors.is_empty(), "Vectors must not be empty");
        assert_eq!(
            vectors.len(),
            weights.and_then(|w| Some(w.len())).unwrap_or(vectors.len()),
            "Vectors and weights must be same length"
        );

        let mut sum = Self::from([0.0; N]);
        let mut total_weight = 0.0;

        for i in 0..vectors.len() {
            let vec = &vectors[i];
            let weight = weights.and_then(|w| Some(w[i]));
            sum += **vec * weight.unwrap_or(1.);
            total_weight += weight.unwrap_or(0.);
        }

        if total_weight != 0.0 {
            for val in &mut sum.0 {
                *val /= total_weight;
            }
        }

        sum
    }
}

fn categorize<const N: usize>(
    method: Categorization,
    mut model: super::ml::Model,
    notes: Vec<Note>,
) -> HashMap<String, Vec<Note>> {
    let mut map = HashMap::new();

    match method {
        Categorization::Description => {
            unimplemented!()
        }
        Categorization::Content => {
            // notes should definately have embeddings `on` them so we dont have to generate them everytime
            let note_contents: Vec<String> = notes.iter().map(|n| n.content.to_owned()).collect();
            println!("note contents: {note_contents:#?}");
            let embeds: Vec<EmbeddingVec<N>> = model
                .get_embeddings(super::ml::Params {
                    sentences: note_contents,
                    normalize_embeddings: false,
                })
                .expect("failed to embed notes' content")
                .into_iter()
                .map(|e| EmbeddingVec::<N>::from(e))
                .collect();

            println!("GOT EMBEDS!");
            let embedded_notes = notes
                .into_iter()
                .zip(embeds)
                .collect::<Vec<(Note, EmbeddingVec<N>)>>();

            let clusters = mean_shift_clustering(embedded_notes);
            // feed each cluster to an llm to create a category
            // for now ill just convert the usize to string
            for (k, notes_embs) in clusters {
                let notes = notes_embs.into_iter().map(|(n, _)| n).collect();
                map.insert(k.to_string(), notes);
            }
        }
    }
    map
}

/// The bandwidth parameter h determines the size of the neighborhood around each data point, directly influencing the clustering results.
/// *n* is the number of data points.
/// *d* is the number of dimensions in the dataset.
/// THIS IS ONLY *ONE* of many ways of defining bandwidth
/// Called bandwidth in some place, and called radius in others
/// I WILL FIX THIS DISCREPANCY LATER
fn bandwidth(n: usize, d: usize) -> f32 {
    let x: f32 = 1. / (d as f32 + 4.);
    (x * -1.) * n as f32
}

#[derive(Debug)]
struct MeanShift<const N: usize> {
    radius: f32,
    centroids: Option<Vec<EmbeddingVec<N>>>,
}

//// https://pythonprogramming.net/mean-shift-from-scratch-python-machine-learning-tutorial/
impl<const N: usize> MeanShift<N> {
    fn init(radius: f32) -> Self {
        Self {
            radius,
            centroids: None,
        }
    }

    fn fit(&mut self, data: Vec<EmbeddingVec<N>>) {
        let mut centroids = data.clone();
        loop {
            let mut new_centroids = vec![];
            for centroid in centroids.iter() {
                let mut in_bandwidth: Vec<&EmbeddingVec<N>> = vec![];
                for featureset in data.iter() {
                    let dist = EmbeddingVec::euclidean_distance(&featureset, &centroid);
                    println!("DIST: {dist:#?}");
                    if dist < self.radius {
                        in_bandwidth.push(featureset);
                    }
                }

                println!("{} IN BANDWIDTH", in_bandwidth.len());
                let new_centroid = EmbeddingVec::weighted_average(&in_bandwidth, None);
                if !new_centroids.iter().any(|v| *v == new_centroid) {
                    new_centroids.push(new_centroid);
                }
            }
            let prev_centroids: Vec<EmbeddingVec<N>> = centroids.drain(..).collect();

            centroids = new_centroids;

            let mut optimized = true;

            for (i, centroid) in centroids.iter().enumerate() {
                if prev_centroids[i] != *centroid {
                    optimized = false;
                }
                if !optimized {
                    break;
                }
            }

            if optimized {
                break;
            }
        }

        self.centroids = Some(centroids);
    }
}

//// https://www.datacamp.com/tutorial/mean-shift-clustering
/// **Initialization**: Start by considering each data point as a potential candidate for the cluster center.
/// **Density Estimation**: For each data point, define a window around it, called the radius, and then compute the mean of those data points within this radius.
/// **Shifting**: Shift each point to this mean position. This step moves the point towards the region of higher density.
/// **Convergence**: Repeat steps 2 and 3 iteratively until convergence, i.e., when the shift is smaller than a predefined threshold. The insignificant change in shift implies that the points have stabilized around the local maxima of the density function.
fn mean_shift_clustering<const N: usize>(
    embedded_notes: Vec<(Note, EmbeddingVec<N>)>,
) -> HashMap<usize, Vec<(Note, EmbeddingVec<N>)>> {
    // still messing with the best way to calculate this
    let bandwidth = f32::abs(bandwidth(embedded_notes.len(), N)) * 1000.;
    println!("BANDWIDTH: {bandwidth}");
    let mut msc = MeanShift::<N>::init(bandwidth);
    let vecs: Vec<EmbeddingVec<N>> = embedded_notes.iter().map(|(_, v)| v.clone()).collect();
    println!("FITTING");
    msc.fit(vecs);

    // labelled with numbers for now
    let labelled_centroids: HashMap<usize, EmbeddingVec<N>> =
        msc.centroids.unwrap().into_iter().enumerate().collect();
    println!("GOT {} LABELLED CENTROIDS", labelled_centroids.len());

    let mut clusters = HashMap::<usize, Vec<(Note, EmbeddingVec<N>)>>::new();
    for (note, vec) in embedded_notes {
        let cluster_id = {
            let mut current_min_dist = f32::MAX;
            let mut min_dist_label = Option::<usize>::None;

            for (i, centroid) in labelled_centroids.iter() {
                let dist = vec.euclidean_distance(&centroid);
                if dist < current_min_dist {
                    current_min_dist = dist;
                    min_dist_label = Some(*i);
                }
            }
            min_dist_label.unwrap_or({
                tracing::error!("GOT NO LABEL WHEN COMPARING NOTE MIN DISTS");
                *labelled_centroids
                    .keys()
                    .next()
                    .expect("No labelled centroids?")
            })
        };
        match clusters.get_mut(&cluster_id) {
            Some(v) => v.push((note, vec)),
            None => {
                let _ = clusters.insert(cluster_id, vec![(note, vec)]);
            }
        }
    }

    clusters
}

#[cfg(test)]
mod test {
    use crate::{
        components::notes::{list::generate_mock_notes, Note},
        services::ml::{fetch, get_model_info},
    };

    pub fn mock_notes() -> Vec<Note> {
        vec![
        // Cluster 1: Project & Work-related
        (
            "Meeting Notes",
            "Discussed project roadmap and key deadlines for the upcoming quarter.",
        ),
        (
            "Project Update",
            "Updated the team on the latest features added and upcoming tasks.",
        ),
        (
            "Business Strategy",
            "Outlined marketing strategies for next quarter to boost product visibility.",
        ),
        (
            "Coding Thoughts",
            "Thinking about optimizing the Redux store and improving performance.",
        ),
        (
            "Personal Goals",
            "Set personal and career goals for the year, including skill development and networking.",
        ),

        // Cluster 2: Food & Recipes
        (
            "Shopping List",
            "Milk, eggs, bread, and some fresh vegetables for the week.",
        ),
        (
            "Recipe Ideas",
            "Experimenting with a new pasta recipe that includes a creamy garlic sauce.",
        ),
        (
            "Workout Plan",
            "Planned a full-body workout for the week, including strength and cardio exercises.",
        ),

        // Cluster 3: Travel & Leisure
        (
            "Travel Itinerary",
            "Booked flights for the upcoming trip to Paris, with sightseeing activities planned.",
        ),
        (
            "Movie Watchlist",
            "Added some classic films to my watchlist, including 'The Godfather' and 'Casablanca'.",
        ),
        (
            "Music Playlist",
            "Compiled a list of favorite rock songs for the road trip, featuring Led Zeppelin and Pink Floyd.",
        ),

        // Cluster 4: Personal Reflection & Inspiration
        (
            "Daily Journal",
            "Wrote about my experiences today, reflecting on the challenges and personal growth.",
        ),
        (
            "Book Summary",
            "Summarized key insights from the book I read, focusing on self-improvement and habits.",
        ),
        (
            "Ideas & Inspiration",
            "Captured new creative ideas for future projects, including a possible photography series.",
        ),
        (
            "Random Thoughts",
            "Just a collection of random musings and thoughts on life and the universe.",
        ),
    ]
    .into_iter()
    .map(|(title, content)| Note::create(title, content))
    .collect()
    }

    #[tokio::test]
    async fn categorize() {
        let notes = mock_notes();
        let url =
            "https://huggingface.co/sentence-transformers/all-MiniLM-L6-v2/resolve/refs%2Fpr%2F21/";
        let info = get_model_info(url);
        let weights = fetch(&info.model_url).await;
        let tokenizer = fetch(&info.tokenizer_url).await;
        let config = fetch(&info.config_url).await;

        let model = crate::services::ml::Model::load(weights, tokenizer, config).unwrap();
        // 384 is how many dims the model's output embeddings are
        let cat = super::categorize::<384>(
            crate::components::notes::list::Categorization::Content,
            model,
            notes,
        );
        println!("{cat:#?}");
        assert!(false);
    }
}
