#![allow(clippy::wildcard_imports)]

use seed::*;
use seed::prelude::*;
//use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::simulation::Simulation;

pub struct Model {
	pub simulation_timer_handle: Option<StreamHandle>,
	pub steps: u64,

	pub simulation: Simulation,
	//pub canvas: ElRef<HtmlCanvasElement>,
}

impl Default for Model {
	fn default() -> Self {
		Model {
			steps: 0,
			simulation_timer_handle: None,
			// canvas: ElRef::default(),
			simulation: Simulation::default()
		}
	}
}

#[derive(Clone, Copy)]
pub enum Msg {
	RunSimulation,
	StepSimulation(u64),
	InterruptSimulation,
	ResetSimulation,
	SetStepsPerIteration(u64),

	SetPopulationSize(u64),
	SetBucketCount(u8),
	SetPopulationDistributionPercentage(usize, f32), // Set how much of the population goes into this bucket.
	SetPopulationTargetingProbability(usize, f32),
}

pub fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
	//let prefers_dark_mode = window().unwrap().match_media("(prefers-color-scheme: dark)").unwrap().unwrap().matches();

	Model::default()
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
	match msg {
		Msg::RunSimulation => {
			model.simulation_timer_handle = Some(get_simulation_timer_handle(orders, 100));
		}
		Msg::InterruptSimulation => {
			model.simulation_timer_handle = None; // Do we need to cancel this?
		}
		Msg::SetStepsPerIteration(steps_per_iteration) => {
			//model.simulation_speed = speed;
			if model.simulation_timer_handle.is_some() {
				model.simulation_timer_handle.replace(get_simulation_timer_handle(orders, steps_per_iteration));
			}
		}
		Msg::ResetSimulation => {

		}
		Msg::StepSimulation(step_count) => {}
		Msg::SetPopulationSize(new_pop_size) => {}
		Msg::SetBucketCount(new_bucket_count) => {}
		Msg::SetPopulationDistributionPercentage(bucket_idx, percentage) => {}
		Msg::SetPopulationTargetingProbability(bucket_idx, percentage) => {}
		//Msg::AddRandomPoint => {add_random_point(model);}
		//Msg::AddRandomPoints(n) => {(0..n).for_each(|_| add_random_point(model));}
	}
}


//
// Document + page interaction things:
//

fn get_simulation_timer_handle(orders: &mut impl Orders<Msg>, steps: u64) -> StreamHandle {
	orders.stream_with_handle(streams::interval(50, move || Msg::StepSimulation(steps)))
}

fn update_body_class(prefers_dark_mode: bool) {
	//let body = window().unwrap().document().unwrap().body().unwrap();
	//let body_class = body.class_name();

	if prefers_dark_mode {
		//body.set_class_name(format!("{} dark", body_class).as_str());
	} else {
		//body.set_class_name(body_class.replace("dark", "").as_str());
	}
}

//
// Unit Tests:
//

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_add() {

	}
}