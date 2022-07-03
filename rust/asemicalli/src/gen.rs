use crate::models::{Attractor, Letter, Word};
use nannou::prelude::*;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, Normal, Uniform};
use std::collections::HashMap;

pub fn generate_words(
    input: &str,
    n_attractors_per_letter: usize,
    avg_mass: f32,
    width: f32,
    height: f32,
) -> Vec<Word> {
    let sentences: Vec<&str> = input.split('\n').collect();
    let max_letter_height = 0.75 * (height / sentences.len() as f32);
    let letter_height = max_letter_height.min(height / 5.0);

    let longest_sentence_length = sentences.iter().map(|s| s.len()).max().unwrap();
    let max_letter_width = width / longest_sentence_length as f32;
    let letter_width = max_letter_width.min(width / 10.0);

    let dict = generate_dictionary(
        input,
        n_attractors_per_letter,
        avg_mass,
        letter_width,
        letter_height,
    );

    let mut res: Vec<Word> = Vec::new();

    let mut current_y: f32 = height / 2.0 - 1.25 * letter_height;
    for sent in sentences {
        let mut current_x: f32 = -1.0 * width / 2.0;
        let words: Vec<&str> = sent.split(' ').collect();

        for word in words {
            let start_x = current_x;
            let mut attractors: Vec<Attractor> = Vec::new();

            for c in word.chars() {
                let repr = dict.get(&c).unwrap();

                for att in &repr.attractors {
                    attractors.push(Attractor {
                        pos: Point2::new(current_x + att.pos.x, current_y + att.pos.y),
                        mass: att.mass,
                    });
                }
                current_x += letter_width;
            }

            let word_width = word.len() as f32 * letter_width;
            res.push(Word {
                bounds: Rect::from_x_y_w_h(
                    start_x + word_width / 2.0,
                    current_y + letter_height / 2.0,
                    word_width,
                    letter_height,
                ),
                attractors: attractors,
            });

            current_x += letter_width;
        }

        current_y -= 2.0 * letter_height;
    }

    res
}

pub fn generate_dictionary(
    input: &str,
    n_attractors: usize,
    avg_mass: f32,
    letter_width: f32,
    letter_height: f32,
) -> HashMap<char, Letter> {
    let mut res = HashMap::new();

    for c in input.chars() {
        if res.contains_key(&c) {
            continue;
        }

        let attractors =
            generate_letter_attractors(&c, n_attractors, avg_mass, letter_width, letter_height);
        res.insert(
            c,
            Letter {
                attractors: attractors,
            },
        );
    }

    return res;
}

pub fn generate_letter_attractors(
    c: &char,
    n_attractors: usize,
    avg_mass: f32,
    width: f32,
    height: f32,
) -> Vec<Attractor> {
    let mut rng = ChaCha8Rng::seed_from_u64(*c as u64);

    let mass_distr = Normal::new(avg_mass, avg_mass / 100.0).unwrap();
    let x_distr = Uniform::new(0.0, width);
    let y_distr = Uniform::new(0.0, height);

    let mut results: Vec<Attractor> = Vec::with_capacity(n_attractors);

    for _ in 0..n_attractors {
        results.push(Attractor {
            pos: Point2::new(x_distr.sample(&mut rng), y_distr.sample(&mut rng)),
            mass: mass_distr.sample(&mut rng),
        })
    }

    results
}
