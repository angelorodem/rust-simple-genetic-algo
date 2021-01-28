# Rust example Genetic Algorithm
A simple project for learning Rust.

It creates a random population with 64 characteristics that range from 0 to 255,
the objective that defines the Agent performance is given by the sum of all it's characteristics,
the 64 agents that have the sum of parameters closest to the TARGET objective have higher fitness and will be mated to generate a new population,
the new population will have the crossed characteristics of the best-selected agents,
then there is some mutation to prevent overfitting and introduce novelty.
