use chrono::prelude::*;
use rand::prelude::*;
use rand::Rng;
use randomwalk::generators::NormalGenerator;

pub fn walk() {
    println!("Walk called");
    let mut rng = rand_hc::Hc128Rng::from_entropy();

    let now = || chrono::Utc::now();
    let utc_now = now();
    let norm_dist = rand_distr::Normal::new(0.0, 1.0).unwrap();
    let exp_dist = rand_distr::Exp::new(1.0 / 180.0).unwrap();

    // Setup a random walk with zero mean. Since we'll be
    // adding the value of the walk to the shaping functions,
    // using a walk with zero mean ensures that the walk will
    // wander around but will keep returning to the vicinity
    // of the shaping function. Reducing the variance will
    // keep the walk closer to the shaping function and
    // make its form more obvious.

    let mut normal_gen = NormalGenerator::new(100.0, 250.0, 0.1, utc_now.timestamp() as u64);

    // The walk will consist of shaped transitions between
    // random points. The start variable represent the
    // starting point of a transition, which is usually
    // the 'current' time and value and the target variables
    // represent the end point of a transition - the value
    // the walk need to walk to and the time at which it
    // needs to be there.

    let mut current_time = utc_now.timestamp_millis() as f64;
    let mut current_value = 0.0;

    let mut start_time = current_time;
    let mut start_value = 100.0 + rng.sample(norm_dist) * 10.0;

    let mut target_time = current_time + rng.sample(exp_dist) + 5.0;
    let mut target_value = 100.0 + rng.sample(norm_dist) * 10.0;

    for i in 1..10 {
        current_time = current_time + (i as f64 * 500f64);

        if current_time > target_time {
            start_time = current_time;
            start_value = current_value;

            target_time = current_time + rng.sample(exp_dist) + 5.0;
            target_value = 100.0 + rng.sample(norm_dist) * 10.0;
        }

        current_value = normal_gen
            .next_interpolant(current_time, target_time, 0.0)
            .unwrap();

        let normalized_time = (current_time - start_time) / (target_time - start_time);

        // linear
        // let shaping_value = target_value + normalized_time + start_value * (1.0 - normalized_time);

        // smoothstep
        // let shaping_value = start_value + (target_value - start_value) * (3.0 * normalized_time * normalized_time - 2.0 * normalized_time * normalized_time * normalized_time);

        // quarter
        let shaping_value = start_value
            + (target_value - start_value)
                * (1.0 - (1.0 - normalized_time) * (1.0 - normalized_time)).sqrt();

        println!("walk {:#.2}", current_value + shaping_value,);
    }
}
