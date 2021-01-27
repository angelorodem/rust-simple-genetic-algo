use std::usize;

use rand_distr::{Normal, Distribution};
use rand::Rng;

static OFFSPRINGS: i32 = 3;
static RELATIONS: i32 =  32;
static AGENTS: i32 = RELATIONS*OFFSPRINGS*2;

static PARAMETERS: i32 = 64;
static MUTATION_COEF: f32 = 5.0;
static MUTATION_PROB: f32 = 0.10;
static GENERATIONS: i32 = 500;

static TARGET: i32 = 16320;

// Gaussian noise
fn noise() -> i32 {
    let normal = Normal::new(0.0, MUTATION_COEF).unwrap();
    normal.sample(&mut rand::thread_rng()).floor() as i32
}

#[derive(Default, Debug)]
struct Agent {
    parameters: Vec<i32>,
    score: i32,
}


fn mutate_offspring(agent: &mut Agent){
    let mut rng = rand::thread_rng();
    let parameters: &mut Vec<i32> = &mut agent.parameters;

    for i in 0..PARAMETERS as usize {
        if rng.gen::<f32>() <=  MUTATION_PROB {
            parameters[i] += noise();
        }
    }

}


fn cross_over(agent_a: &Agent, agent_b: &Agent) -> Vec<Agent>{
    let mut rng = rand::thread_rng();
    let cross_point :i32 = rng.gen_range(4..PARAMETERS - 5);

    let mut a_new_parameters: Vec<i32> = Vec::new();
    let mut b_new_parameters: Vec<i32> = Vec::new();

    for i in 0..cross_point as usize{
        a_new_parameters.push(agent_a.parameters[i]);
        b_new_parameters.push(agent_b.parameters[i]);
    }

    for i in cross_point as usize..PARAMETERS as usize {
        a_new_parameters.push(agent_b.parameters[i]);
        b_new_parameters.push(agent_a.parameters[i]);
    }

    let new_agent_a = Agent{
        parameters: a_new_parameters, ..Default::default()
    };

    let new_agent_b = Agent{
        parameters: b_new_parameters, ..Default::default()
    };

    vec![new_agent_a, new_agent_b]
}


fn agent_score(agent: &mut Agent){  // FitScore
    let mut sum = 0;
    for i in 0..PARAMETERS as usize {
        sum += agent.parameters[i];
    }

    agent.score = i32::abs(TARGET - sum);
}


fn initial_population() -> Vec<Agent> {
    let mut population : Vec<Agent> = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..AGENTS {
        let mut agent_parameters: Vec<i32> = Vec::new();

        for _ in 0..PARAMETERS as usize {
            agent_parameters.push(rng.gen_range(-255..255))
        }


        let new_agent = Agent {
            parameters: agent_parameters, ..Default::default()
        };

        population.push(new_agent)
    }

    population
}

fn main() {
    let mut population: Vec<Agent> =  initial_population(); 
    let mut next_population: Vec<Agent> = Vec::new();

    let mut needed_gens = 0;
    for gen in 0..GENERATIONS {

        for i in 0..population.len() {
            agent_score(&mut population[i])            
        }

        population.sort_by( |a, b| a.score.cmp(&b.score));

        // Print performance
        let mut error = 0;
        for i in 0..RELATIONS as usize {
            error += population[i].score;            
        }
        

        println!("---------- Generation: {:?}, Mean error: {:?}", gen, error/RELATIONS);

        if gen >= GENERATIONS -1 || population[0].score == 0 {
            needed_gens = gen;
            break;
        }

        // Mate and mutate
        for i in 0..RELATIONS as usize {
            for _ in 0..OFFSPRINGS {
                let mut offsprings = cross_over(&population[i], &population[i+1]);
                for offspring in 0..offsprings.len() {
                    mutate_offspring(&mut offsprings[offspring]);
                }

                next_population.append(&mut offsprings);
            }     
        }        

        std::mem::swap(&mut population, &mut next_population);
        next_population.clear();

    }

    println!("The best agent that sum of parameters reaches close to the target {:?} is:", TARGET);
    println!("{:?}", population[0].parameters);
    println!("With error of delta: {:?}", population[0].score);
    println!("Found Chad after {:?} generations", needed_gens);


}
