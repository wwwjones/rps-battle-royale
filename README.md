This project is a little rock paper scissors simulation built in rust.

The implementation uses the NPC Engine (https://github.com/ethz-gtc/npc-engine) as a starting point, but ditches the MCTS planning algorithm in favor of a homemade tree algorithm. 
The MCTS algorithm in the NPC Engine works wonders for a small number of agents, though performance does not scale well with the large amount of agents wanted for this project.
Where the MCTS algorithm employs dynamic exploration and exploitation, this planner is built to try ALL possibilities for one agent for a few steps, while greedily executing actions of other agents to attempt a similar simulation-based approach.
This planner is therefore more myopic in its approach, though its purpose is not to show great intelligence for a few agents, but rather middle-of-the-road intelligence for many.

Currently, visualization is done in the terminal, but a wgpu implementation is in the works.

This project is very much a work in progress!

![image](https://github.com/wwwjones/rps-battle-royale/assets/97795524/0005464b-bd77-4711-8a25-19b1f9c1f4e5)
