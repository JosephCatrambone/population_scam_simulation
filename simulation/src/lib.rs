use rand::{rngs::SmallRng, Rng, SeedableRng};

#[derive(Clone)]
pub struct Population {
	pub population_size: u64,
	pub num_buckets: u8,
	pub age_distribution: Vec<f32>, // What percent of the population goes into each distribution?
	pub targeting_probability: Vec<f32>, // What population is targeted most often by scammers?
	pub exposure_probability: Vec<f32>, // Which population is more likely to be online?
	pub scam_susceptibility: Vec<f32>, // 0 -> never falls for scams.  1 -> always falls for scams.
	pub report_probability: Vec<f32>, // 0 -> never reports.  1 -> always reports.
}

pub struct Outcome {
	pub attempts: Vec<u64>,
	pub successes: Vec<u64>,
	pub reports: Vec<u64>,
}

pub struct Simulation {
	rng: SmallRng,
	population: Population,
	experiment: Outcome,
	previous_outcomes: Vec<(Population, Outcome)>,
}

impl Default for Population {
	fn default() -> Self {
		Self {
			// These are the numbers for the US population.
			population_size: 340_000_000,
			num_buckets: 10,
			age_distribution: vec![
				0.1231609713, // [0-10)
				0.128488451, // 10-20
				0.137290508, // 20-30
				0.1337265227, // 30-40
				0.1231023937, // 40-50,
				0.128001331, // 50-60,
				0.1169949068, // 60-70,
				0.07241426087, // 70-80
				0.03682065385, // 80-90
				0.0 // 90-100
			],
			targeting_probability: vec![0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, ],
			exposure_probability: vec![0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, ],
			scam_susceptibility: vec![0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, ],
			report_probability: vec![0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, ],
		}
	}
}

impl Outcome {
	fn new(num_buckets: u8) -> Self {
		let mut attempts = Vec::with_capacity(num_buckets as usize);
		let mut successes = Vec::with_capacity(num_buckets as usize);
		let mut reports = Vec::with_capacity(num_buckets as usize);
		for _ in 0..num_buckets {
			attempts.push(0);
			successes.push(0);
			reports.push(0);
		}

		Outcome { attempts, successes, reports }
	}
}

impl Default for Simulation {
	fn default() -> Self {
		let pop = Population::default();
		Self {
			rng: SmallRng::from_entropy(),
			experiment: Outcome::new(pop.num_buckets),
			population: pop,
			previous_outcomes: vec![],
		}
	}
}

impl Simulation {
	pub fn get_bucket_count(&self) -> u8 {
		self.population.num_buckets
	}

	pub fn step(self: &mut Self) {
		// Simulate one attempted scam.

		// The likelyhood of an individual receiving a scam attempt is the joint probability of
		// being targeted, being exposed, and being inside of a given distribution.
		let joint_pop_probability = join_and_normalize_probabilities(vec![
			&self.population.age_distribution,
			&self.population.targeting_probability,
			&self.population.exposure_probability,
		]);
		let targeted_age_group = sample_from_distribution(&joint_pop_probability);

		// We have a scam attempt targeting group X.
		self.experiment.attempts[targeted_age_group] += 1;
		// The group was targeted and a person was exposed.  Do they fall for it?
		if !self.rng.gen_bool(self.population.scam_susceptibility[targeted_age_group] as f64) {
			return; // Individual exposed, but they didn't fall for it.
		}
		// They did fall for it:
		self.experiment.successes[targeted_age_group] += 1;

		// Do they report it?
		if !self.rng.gen_bool(self.population.report_probability[targeted_age_group] as f64) {
			return; // No report.
		}
		// Yes.  File report.
		self.experiment.reports[targeted_age_group] += 1;
	}
}

/// Sum each of the distributions and renormalize the total.
/// Normalizes each distribution before adding, then normalizes again at the end.
pub fn join_and_normalize_probabilities(distributions: Vec<&Vec<f32>>) -> Vec<f32> {
	if distributions.is_empty() {
		return vec![];
	}

	// Perform the normalized dot between all of these distributions.
	let mut accumulator: Vec<f32> = normalize_distribution(distributions.first().unwrap());

	for dist in distributions.iter().skip(1) {
		assert_eq!(dist.len(), accumulator.len());
		let normalized = normalize_distribution(dist);
		for (idx, value) in normalized.iter().enumerate() {
			accumulator[idx] += value;
		}
	}

	normalize_distribution(&accumulator)
}

/// Return a vector whose elements sum to one.
/// Panics if a vector is empty
pub fn normalize_distribution(dist: &Vec<f32>) -> Vec<f32> {
	let sum = dist.iter().fold(0.0f32, |acc, element|{ acc + *element});
	assert!(sum != 0.0 && !sum.is_nan() && !sum.is_infinite());
	dist.iter().map(|v|{ v / sum}).collect()
}

/// Samples from a distribution, returning the weighted index of the selected bucket.
/// If we happen to pass [0.1, 0.8, 0.1], the value '1' will be returned 80% of the time.
/// The distribution does not need to be normalized.  If the values total to a value greater than
/// 1.0, the distribution is automatically renormalized internally.
///
/// Values cannot be negative.  Using negative probabilities will result in unknown but probably wrong behavior.
///
/// If the distribution is all zeros or contains NaN/Inf, returns 0.
///
/// If the distribution is empty, panics.
///
/// There may be floating point issues with this technique.  If all of the values are infinitesimal but nonzero, we may only return the first element.
/// If one value is near infinity but not infinite, we may only return that value.
///
/// # Arguments
///
/// * `dist` - An array of float 32s greater than or equal to zero.
///
pub fn sample_from_distribution(dist: &Vec<f32>) -> usize {
	assert!(dist.len() > 0);

	// Hey, I just noticed 'choose_weighted_mut'!

	let mut energy = dist.iter().fold(0.0f32, |accumulator, element| {accumulator + element});

	if energy.is_nan() || energy.is_infinite() || energy == 0.0 {
		return 0;
	}

	for (bin_idx, elem) in dist.iter().enumerate() {
		if energy < *elem {
			return bin_idx;
		}
		energy -= *elem;
	}

	dist.len()-1
}

//
// Unit Tests:
//

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_sanity_e2e() {
		//assert_eq!(add(1, 2), 3);
		let mut sim = Simulation::default();
		sim.step();
	}

	#[test]
	fn test_distribution_sample() {
		let dist = vec![0.01, 0.99];
		let mut counts = [0, 0];
		for _ in 0..100000 {
			counts[sample_from_distribution(&dist)] += 1;
		}
		assert!(counts[0] < 10000);
		assert!(counts[1] > 70000);
	}
}