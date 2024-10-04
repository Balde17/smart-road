# Smart Road: Traffic Control Strategy and Simulation

## Objectives

Building upon the road_intersection raid from the Rust Piscine, the goal of this project is to develop a new traffic control strategy and simulation for autonomous vehicles (AVs) at intersections. This time, the intersection management must be smart and operate without traffic lights.

Overused intersections can cause severe traffic jams, wasting time, money, and increasing air pollution. According to the National Highway Traffic Safety Administration, about 96% of intersection-related crashes are due to driver error. Autonomous vehicles offer a promising solution to these issues. With their anticipated public availability in the next decade, new traffic strategies for AVs must be developed. Your task is to create an algorithm that enables AVs to navigate intersections without collisions and with minimal congestion.

## Instructions

### Intersection

We will focus on a cross intersection where each lane has a specific route:

- **r:** Turning right
- **s:** Straight ahead
- **l:** Turning left


### Vehicles

As all vehicles are autonomous, you need to implement their physics considering the following rules:

- AVs must follow the route of their lane and cannot change lanes or routes.
- AVs should have at least 3 different velocities, controlled by the smart intersection system.
- AVs must maintain a safety distance from other AVs to avoid collisions. The safety distance should be a strictly positive value.
- Implement vehicle physics with parameters: time, distance, and velocity.

### Animation

You must animate the movement of vehicles using assets. The vehicles should turn according to their route, meaning the image orientation should change to reflect the vehicle's direction.

### Commands

Implement the following commands for the simulation:

- **Arrow Up:** Generate vehicles from south to north.
- **Arrow Down:** Generate vehicles from north to south.
- **Arrow Right:** Generate vehicles from west to east.
- **Arrow Left:** Generate vehicles from east to west.
- **R:** Continually generate random vehicles.
- **Esc:** Exit the simulation and display statistics.

Ensure vehicles are not generated on top of each other when the same key is spammed.

### Statistics

Upon exiting the simulation, display the following statistics:

- Max number of vehicles that passed the intersection.
- Max velocity of all vehicles.
- Min velocity of all vehicles.
- Max time taken to pass the intersection.
- Min time taken to pass the intersection.
- Number of close calls (violations of the safe distance).

### Example

You can refer to an example [here](#).

### Bonus

Optional features to enhance the project:

- Create custom assets for vehicle animation.
- Add more statistics.
- Implement acceleration and deceleration physics.

## Learning Outcomes

This project will help you learn about:

- Rust programming language
- SDL2
- Animation
- Algorithm design
- Mathematics
- Event handling

## Authors

* [sbadiane](https://learn.zone01dakar.sn/git/sbadiane)
* [abdbalde](https://learn.zone01dakar.sn/git/abdbalde)

---

Feel free to reach out if you have any questions or need further assistance. Happy coding!
