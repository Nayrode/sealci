# Scheduler Architecture

## Contributors

- DE MARTINO Giada Flora
- DURAT Mathias
- PLANCHE Benoit
- TCHILINGUIRIAN Théo

## Lexicon

- Action: a CI atomic unit. It contains infrastructure, environment and commands to execute.
- Agent: an agent is a computing node registered with the scheduler.
- Agent pool: the set of all registered agents. It is a scheduler's entire knowledge about available computing resources.
- Scheduling: selection of an agent on which to execute an action.
- Action status: the state of the execution of an action (running, successful, failed).

## The "What", "Why" and "How" of the scheduler

### What?

The scheduler receives a stream of CI actions (CI job units) as its main input. It also tracks a set of CI agents (registered computing nodes) that provide a dynamic resource pool, where the CI actions will be executed.

The main role of a scheduler instance is to select CI agents on which to run the received actions. This selection of an agent and distribution of a action on this agent is called 'scheduling'.

A scheduler:

- Must be functional even without any registered agents. When in such state, the scheduler rejects all incoming requests from the controller.
- Can receive more actions than it has registered agents.
- Must always know the current state / capacity (memory, CPU) of each registered agent.
- Distributes actions to agents based on their resource capacities and current load (memory and CPU).
- Schedule actions in order, i.e. in the same order that it received them.

- The creation and startup of agents is out of the scheduler's scope.

### Why?

Schedulers abstract the CI cluster resource management away from the controller, by tracking all registered agents and their available computing resources.

This allows for a clean separation of duties between the controller and the scheduler. The former is responsible for managing the CI jobs themselves, independently from the actual resources that evolve within the CI cluster.  
Schedulers thus allow for an efficient distribution of load between computing resources.

### How?

A scheduler has a pool of available agents. Each agent is connected through a continuous connection to monitor their resource capacities.

During scheduling of an action, the changes of states are reported the same way the logs are reported through a stream of response message to the action request. The logs are forwarded from the agent to the controller with action identification (interfaces defined in .proto files). The scheduler is not in charge of the log interpretation.

- Agents connect to the scheduler.
- The scheduler does not persist state; if it fails, agents have to reconnect and resubmit their state information.

- The scheduler knows currently registered agents as a pool of computing resources.
- The scheduler is mostly stateless, and agents must attempt to reconnect to it if the connection is lost between them and the scheduler.
- A gRPC connection exists between an agent and a scheduler, to report health state and resource capacities to the scheduler.
- If an agent disconnects, it is removed from the resource pool. Any pending actions from a disconnected agent must be re-scheduled to another available agent.

- An agent receives actions to execute from the scheduler through a gRPC interface.
- If the action execution stage changes, the agent reports the new stage of the action with a message in the return stream of an action request.
- The execution logs are sent to the controller through a return stream of an action request. The logs are never treated by the scheduler and only forwarded from the agent to the controller.
