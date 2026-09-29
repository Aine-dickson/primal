# Prismal
## 06 Runtime Semantics

### 1. Purpose

The refined core model defines **what exists** in the system:

- state
- objects
- values
- relations
- fields
- behaviors
- processes
- events
- observations
- projections
- views
- interactions
- experiments

Runtime semantics defines **what happens** when those things execute.

The central question is:

> Given a model, an initial state, a context, and external inputs, what precisely does the runtime do, in what order, and what state does it produce?

This document therefore defines the execution semantics of the system before designing the DSL syntax or implementation architecture.

---

# 2. Fundamental Runtime Principle

The runtime is a **state transition system with multiple coordinated clocks and execution domains**.

At the semantic level:

```text
Stateₙ
   │
   │  behavior + context + inputs
   ↓
Transition
   │
   ↓
Stateₙ₊₁
```

For continuous models:

```text
S(t₀) ───────── evolution ─────────→ S(t₁)
```

For discrete models:

```text
S₀ → S₁ → S₂ → S₃ → ...
```

For hybrid models:

```text
continuous evolution
       │
       ├── event
       ↓
discrete transition
       │
       ↓
continuous evolution
       │
       └── event
```

The runtime must therefore not assume that every model advances through identical fixed-size frames.

---

# 3. Three Different Clocks

A critical distinction is required between:

```text
SIMULATION TIME
PRESENTATION TIME
WALL-CLOCK TIME
```

### Simulation time

The time represented by the model.

Example:

```text
t = 37.5 seconds
```

A radioactive-decay simulation might represent:

```text
t = 10,000 years
```

### Presentation time

The time over which the result is displayed or animated.

For example:

```text
10,000 years of simulation
→
30 seconds of animation
```

### Wall-clock time

Actual execution time.

For example:

```text
10,000 simulation years
→
2.3 seconds CPU time
```

These must remain independent.

Therefore:

```text
simulation_time ≠ presentation_time ≠ wall_clock_time
```

The runtime may map between them, but they are semantically distinct.

---

# 4. Runtime State

The runtime maintains more than semantic model state.

```text
RUNTIME
├── Semantic State
├── Computational State
├── Presentation State
└── Execution State
```

## 4.1 Semantic State

What the model says currently exists.

Examples:

```text
ball.position
ball.velocity
temperature(x,y)
particle.alive
graph.nodes
graph.edges
```

This is the authoritative scientific/conceptual state.

---

## 4.2 Computational State

Internal state needed to calculate the next state.

Examples:

```text
integrator history
solver buffers
spatial indexes
cached derivatives
GPU buffers
numerical accumulators
event queues
random-generator state
```

Computational state must not silently become part of the scientific model.

---

## 4.3 Presentation State

State describing how the model is currently presented.

Examples:

```text
camera.position
camera.zoom
selected_object
visible_layers
layout
animation_progress
```

Changing camera zoom must not change the scientific model.

---

## 4.4 Execution State

State describing execution itself.

Examples:

```text
running
paused
stepping
completed
failed
waiting
replaying
```

This gives:

```text
Semantic State
      +
Computational State
      +
Presentation State
      +
Execution State
```

---

# 5. Authoritative State

The runtime must define which state is authoritative.

The rule is:

> Semantic state is authoritative. Computational and presentation states are derived execution structures.

For example:

```text
semantic:
particle.position = (2.0, 4.0)
```

A renderer may contain:

```text
GPU buffer → pixel coordinates
```

and a physics subsystem may contain:

```text
spatial hash → particle location
```

Neither replaces the semantic position.

This allows:

```text
CPU renderer
GPU renderer
web renderer
video renderer
```

to observe the same model.

---

# 6. State Immutability During Evaluation

A major runtime rule is needed:

> A state being evaluated must not be partially modified by arbitrary behaviors during the same evaluation phase.

Instead, conceptually:

```text
Sₙ
 │
 ├── evaluate behaviors
 │
 ├── produce changes
 │
 └── commit
      ↓
    Sₙ₊₁
```

This avoids order-dependent bugs such as:

```text
A reads x
B changes x
C reads x
```

where C sees a different value depending purely on execution order.

The default semantic model should therefore use:

```text
READ CURRENT STATE
        ↓
COMPUTE CHANGES
        ↓
VALIDATE
        ↓
COMMIT
```

rather than unrestricted immediate mutation.

---

# 7. The Transition

A transition can be represented conceptually as:

```text
Transition(
    state,
    context,
    inputs,
    dt
) → candidate_state
```

The runtime then performs:

```text
evaluate
   ↓
resolve
   ↓
validate
   ↓
commit
```

A transition therefore has four conceptual stages.

---

# 8. Evaluation

During evaluation, the runtime determines what should happen.

Examples:

```text
velocity = velocity + acceleration * dt
```

or:

```text
if distance(a,b) < radius:
    collision_event
```

or:

```text
sample Normal(0,1)
```

or:

```text
temperature[i] =
    average(neighbours[i])
```

Evaluation should normally be side-effect free with respect to committed semantic state.

---

# 9. Change Set

Behaviors conceptually produce a **change set**.

Example:

```text
CURRENT

ball.position = (0,0)
ball.velocity = (10,0)
```

Behavior evaluates:

```text
Δposition = (1,0)
```

Candidate change:

```text
ball.position ← (1,0)
```

Multiple behaviors can therefore produce:

```text
ChangeSet
├── ball.position ← ...
├── ball.velocity ← ...
├── particle.alive ← false
└── relation(A,B) ← removed
```

The runtime resolves these changes before committing them.

---

# 10. Multiple Behaviors Modifying the Same State

This is one of the most important semantic problems.

Suppose:

```text
Behavior A:
    x += 1

Behavior B:
    x *= 2
```

If executed sequentially:

```text
x = 1
A → 2
B → 4
```

but reversing them gives:

```text
x = 1
B → 2
A → 3
```

The runtime must not accidentally make execution order part of scientific meaning.

There are therefore several valid semantic categories.

## 10.1 Independent writes

Different behaviors modify different state.

```text
A → x
B → y
```

No conflict.

---

## 10.2 Compatible writes

Multiple behaviors contribute to the same quantity through a defined operation.

Example:

```text
force₁
force₂
force₃
```

The runtime can define:

```text
total_force = Σ forceᵢ
```

This is common in physics.

---

## 10.3 Exclusive writes

Only one behavior may own a state variable.

Example:

```text
position ← integrator_result
```

Another behavior attempting to directly replace it creates a conflict.

---

## 10.4 Constraint resolution

Multiple behaviors may propose values that must satisfy a constraint.

Example:

```text
A proposes x = 3
B requires x ≥ 5
```

A constraint mechanism may resolve this.

The exact resolution strategy belongs to runtime semantics.

---

## 10.5 Explicit conflict

If no valid resolution exists:

```text
CONFLICT
```

The runtime must not silently choose an arbitrary winner.

---

# 11. Dependency Graph

Behavior evaluation requires dependency analysis.

For example:

```text
position
   ↓
velocity
   ↓
kinetic_energy
```

and:

```text
temperature
   ↓
pressure
   ↓
gas_volume
```

The runtime can construct a dependency graph:

```text
A → B → C
```

and evaluate dependencies in a valid order.

However, cycles must be supported.

Example:

```text
A → B
↑   ↓
└── C
```

A coupled physical system can legitimately contain cyclic dependencies.

Therefore:

> A cycle is not automatically an error.

It may require:

- simultaneous solving
- iteration
- fixed-point convergence
- differential equation solving
- constraint solving
- event processing

---

# 12. Continuous Evolution

A continuous process conceptually defines:

```text
dS/dt = f(S,t)
```

For example:

```text
dx/dt = v
dv/dt = g
```

The semantic model defines the evolution law.

The runtime chooses an execution strategy.

For example:

```text
Euler
RK4
adaptive RK
implicit solver
GPU solver
```

The model does not become dependent on the solver.

Therefore:

```text
MODEL
dS/dt = f(S,t)

RUNTIME
solver = RK4
dt = 0.01
```

---

# 13. Numerical Integration

For a continuous process:

```text
S(t) → S(t + dt)
```

the runtime computes an approximation.

Conceptually:

```text
current state
      ↓
evaluate derivative
      ↓
integrator
      ↓
candidate state
      ↓
constraints / events
      ↓
commit
```

The runtime must be capable of exposing numerical metadata such as:

```text
step size
error estimate
tolerance
convergence
stability
precision
```

but these are execution concerns unless explicitly promoted into the model.

---

# 14. Variable Timestep

The runtime must not require:

```text
dt = constant
```

It should support:

```text
dt₁
dt₂
dt₃
...
```

For example:

```text
0.01
0.01
0.005
0.002
0.008
```

Adaptive solvers may change timestep based on estimated error.

The semantic meaning remains:

```text
S(t₀) → S(t₁)
```

regardless of the internal timestep strategy.

---

# 15. Discrete Evolution

A discrete process defines transitions such as:

```text
Sₙ₊₁ = F(Sₙ)
```

Example:

```text
cell.alive =
    neighbour_count >= 2
```

or:

```text
node.visited = true
```

A discrete step is therefore not necessarily a fixed amount of physical time.

It may represent:

```text
one algorithm iteration
one generation
one transaction
one cellular-automaton update
one logical state transition
```

The runtime should therefore distinguish:

```text
simulation step
```

from:

```text
time interval
```

---

# 16. Event-Driven Execution

Some systems are best represented by events.

Examples:

```text
collision
radioactive decay
chemical reaction
threshold crossing
network packet arrival
user click
object creation
object destruction
```

An event has conceptually:

```text
Event
├── trigger
├── time
├── priority
├── participants
└── transition
```

The runtime maintains an event schedule.

Conceptually:

```text
event queue

t=1.2 collision
t=1.8 decay
t=2.1 reaction
t=3.0 user intervention
```

The runtime advances to the next relevant event when using event-driven execution.

---

# 17. Simultaneous Events

Events can occur at the same simulation time.

Example:

```text
t = 5.0

collision A
decay B
threshold C
```

The runtime must define a deterministic policy.

The preferred semantic approach is:

```text
same-time events
       ↓
collect
       ↓
order by explicit semantics
       ↓
evaluate
       ↓
resolve conflicts
       ↓
commit
```

Events should not gain meaning merely because one happened to execute first on a particular machine.

---

# 18. Event Priority

Some systems need explicit priority.

For example:

```text
constraint enforcement
before
ordinary update
```

or:

```text
collision resolution
before
next free-flight interval
```

Therefore events may carry:

```text
priority
```

But priority must be semantic only when the model explicitly requires it.

Otherwise the runtime should avoid exposing arbitrary implementation ordering.

---

# 19. Stochastic Evolution

Random processes require controlled randomness.

Example:

```text
P(decay during dt) = 1 - e^(-λdt)
```

The runtime samples a random source.

Conceptually:

```text
Random Source
      ↓
Distribution
      ↓
Sample
      ↓
Event / Value
```

The random source has:

```text
algorithm
seed
state
```

This allows:

```text
seed = 42
```

to reproduce the same experiment.

---

# 20. Randomness and Deterministic Replay

A stochastic simulation can still be deterministic with respect to its random source.

For example:

```text
Model
+ Initial State
+ Parameters
+ Seed
+ Execution Configuration
```

can produce:

```text
Replay A ≈ Replay B
```

subject to the defined numerical/reproducibility guarantees.

This is essential for:

- debugging
- educational experiments
- recorded demonstrations
- testing
- comparing algorithms

---

# 21. Randomness in Parallel Execution

Parallel execution introduces an additional problem.

If random numbers are consumed according to thread scheduling:

```text
thread A → random₁
thread B → random₂
```

and scheduling changes, the simulation may diverge.

Therefore reproducible stochastic execution may require:

```text
deterministic random streams
```

or:

```text
entity-indexed random streams
event-indexed random streams
counter-based RNG
```

The runtime should define this as an execution policy rather than allowing accidental nondeterminism.

---

# 22. Hybrid Systems

Hybrid systems combine:

```text
continuous processes
+
discrete transitions
+
events
+
randomness
```

Example:

```text
projectile motion
      ↓
continuous integration
      ↓
collision event
      ↓
velocity transition
      ↓
continuous integration
```

The runtime loop becomes:

```text
advance continuous state
        ↓
detect events
        ↓
locate event time
        ↓
stop integration
        ↓
process event
        ↓
resolve constraints
        ↓
commit state
        ↓
resume continuous evolution
```

This is a fundamental runtime capability.

---

# 23. Event Detection During Continuous Evolution

An event may be defined by:

```text
distance(A,B) = r
```

but the solver may step from:

```text
t = 0.9
```

to:

```text
t = 1.0
```

without landing exactly on the collision.

Therefore event detection may require:

```text
event condition
      ↓
detect sign/threshold crossing
      ↓
locate event time
      ↓
integrate to event
```

This is important for accurate hybrid simulations.

---

# 24. Dynamic Object Creation and Destruction

The model allows:

```text
create(entity)
destroy(entity)
```

Examples:

```text
particle decay
cell division
chemical reaction
birth/death
network node creation
packet creation
```

These operations change the structure of state itself.

Therefore:

```text
Stateₙ
```

and

```text
Stateₙ₊₁
```

may contain different object sets.

Likewise relations may appear or disappear:

```text
relation(A,B)
```

can be created or removed dynamically.

---

# 25. Collections and Population Changes

A collection may evolve:

```text
Nₙ = 1000
Nₙ₊₁ = 997
```

without requiring fixed identity slots.

The runtime therefore needs semantic operations for:

```text
create
destroy
insert
remove
filter
query
group
aggregate
```

The implementation may use:

```text
arrays
arenas
maps
sparse sets
GPU buffers
```

but those are implementation details.

---

# 26. Constraints

Constraints participate in state transitions.

Example:

```text
x ≥ 0
```

or:

```text
distance(A,B) = L
```

A candidate state may therefore be:

```text
candidate
    ↓
constraint evaluation
    ↓
valid
```

or:

```text
candidate
    ↓
constraint violation
```

The runtime must distinguish:

```text
constraint violation
```

from:

```text
solver failure
```

and:

```text
runtime failure
```

---

# 27. Constraint Strategies

A constraint may be handled through:

```text
reject
correct
project
clamp
penalize
solve
report
```

For example:

```text
x ≥ 0
```

might be enforced by:

```text
x = max(x,0)
```

while a mechanical constraint might require a numerical solver.

The model specifies the constraint.

The execution strategy determines how it is enforced.

---

# 28. State Commit

A semantic state becomes authoritative only after successful commit.

Conceptually:

```text
Sₙ
 │
 ↓
evaluate
 │
 ↓
candidate S*
 │
 ↓
constraints
 │
 ↓
events
 │
 ↓
validation
 │
 ↓
COMMIT
 │
 ↓
Sₙ₊₁
```

Once committed:

```text
Sₙ₊₁
```

becomes the authoritative semantic state.

---

# 29. Failed Transitions

A transition may fail.

Examples:

```text
numerical divergence
constraint inconsistency
invalid operation
overflow
missing input
solver failure
```

The runtime should not partially commit the failed transition.

Default behavior:

```text
Sₙ
 ↓
failed transition
 ↓
remain at Sₙ
```

with an execution diagnostic.

This provides transactional semantics for state transitions.

---

# 30. Partial State

Not every failure means the entire state is invalid.

For example:

```text
temperature field → successfully computed

pressure field → solver failed
```

The runtime may represent:

```text
temperature = valid
pressure = unavailable
```

Hence the previously defined state validity categories matter:

```text
valid
invalid
unknown
unavailable
pending
```

Validity can exist at multiple granularities:

```text
system
object
property
field
dataset
observation
```

---

# 31. Observation Scheduling

Observations must have defined timing semantics.

Possible policies:

```text
observe before transition
observe after transition
observe every timestep
observe at fixed simulation intervals
observe at events
observe on demand
observe continuously
```

The default semantic pattern should be:

```text
Stateₙ
 ↓
transition
 ↓
Stateₙ₊₁
 ↓
observation
 ↓
data
```

unless an observation explicitly requests another phase.

---

# 32. Observation Does Not Mutate State

Ordinary observations should be read-only:

```text
Observation:
State → Data
```

They must not accidentally change:

```text
simulation state
```

A sensor that physically affects the model should instead be represented as:

```text
observation + intervention
```

or a domain-specific instrument model.

---

# 33. Sampling

Continuous fields cannot necessarily be observed everywhere.

For example:

```text
temperature(x,y)
```

may be sampled at:

```text
(x₁,y₁)
(x₂,y₂)
...
```

The observation system therefore supports:

```text
field
 ↓
sampling strategy
 ↓
dataset
```

Sampling may be:

```text
point
line
region
grid
random
adaptive
temporal
event-based
```

---

# 34. Projection Scheduling

After semantic state changes:

```text
Stateₙ → Stateₙ₊₁
```

dependent observations and projections may become invalid.

The runtime should therefore use dependency tracking:

```text
State
 ↓
Observation
 ↓
Projection
 ↓
Representation
```

Only affected portions need to update where possible.

For example:

```text
ball.velocity changes
```

may invalidate:

```text
velocity arrow
trajectory projection
kinetic-energy graph
```

but not:

```text
title text
```

---

# 35. Rendering Is Not Simulation

The simulation must not depend on rendering frequency.

For example:

```text
simulation:
1000 steps/sec
```

while:

```text
rendering:
60 frames/sec
```

The runtime can execute:

```text
simulation steps
simulation steps
simulation steps
...
render
...
```

The renderer observes committed state.

Therefore:

```text
simulation frequency ≠ render frequency
```

---

# 36. Rendering Between States

For smooth animation, the renderer may interpolate presentation state.

Suppose:

```text
Sₙ at t=1.00
Sₙ₊₁ at t=1.01
```

A frame may be rendered at:

```text
t=1.005
```

using interpolation.

This interpolation is a presentation mechanism unless the model explicitly defines interpolation semantics.

Therefore:

```text
simulation state
      ↓
presentation interpolation
      ↓
render frame
```

must not silently modify the model.

---

# 37. Fixed Simulation / Variable Rendering

A common runtime arrangement is:

```text
Simulation:
fixed timestep

Rendering:
variable frame rate
```

For example:

```text
simulation dt = 1/120 s
rendering ≈ 60 FPS
```

This is valid.

Likewise:

```text
simulation = event driven
rendering = 60 FPS
```

is valid.

---

# 38. Real-Time Mode

Interactive applications may attempt:

```text
simulation time ≈ wall-clock time
```

For example:

```text
1 second real time
≈
1 second simulation time
```

But this is an execution policy.

The model itself should not depend on real-time performance.

If execution cannot keep up:

```text
wall time
>
simulation processing time
```

the runtime must choose a policy:

```text
slow down simulation
drop presentation frames
run simulation faster
pause
warn
```

---

# 39. Accelerated and Slow Motion

The runtime may define:

```text
time_scale
```

such as:

```text
0.1×
1×
10×
1000×
```

This modifies the mapping between:

```text
simulation time
```

and:

```text
presentation/wall time
```

not the scientific equations themselves.

---

# 40. Interaction

User interaction enters the runtime as an external input.

Conceptually:

```text
USER INPUT
   ↓
INTERACTION
   ↓
ACTION
   ↓
STATE / PARAMETER / EXECUTION / VIEW
```

Examples:

```text
slider → parameter
drag → position
button → event
mouse click → selection
keyboard → execution control
measurement tool → observation
```

---

# 41. Interaction During Evolution

An interaction can occur between simulation transitions.

Example:

```text
Sₙ
 ↓
simulation
 ↓
Sₙ₊₁
 ↓
USER DRAGS BALL
 ↓
intervention
 ↓
Sₙ₊₁'
 ↓
simulation resumes
```

The intervention becomes part of the experiment history.

---

# 42. Intervention vs View Interaction

Not every interaction changes the model.

For example:

```text
zoom
pan
camera rotate
select object
```

may affect only presentation state.

Whereas:

```text
change mass
drag particle
add force
remove node
```

may affect semantic state.

Therefore interactions should identify their target domain:

```text
MODEL
PARAMETER
EXECUTION
VIEW
```

---

# 43. Pause, Step, Reset

Execution controls are runtime operations.

```text
pause
resume
step
reset
```

### Pause

Stops evolution without destroying state.

### Resume

Continues from current state.

### Step

Advances according to the model's defined stepping semantics.

A step might mean:

```text
one discrete transition
one fixed timestep
one event
one solver interval
```

depending on execution mode.

### Reset

Restores the experiment's initial state.

---

# 44. Snapshot

A snapshot captures sufficient information to restore an execution state.

Conceptually:

```text
Snapshot
├── semantic state
├── simulation time
├── relevant parameters
├── random state
├── computational state where required
└── execution metadata
```

Not every cache needs to be serialized.

A runtime may reconstruct computational state from semantic state.

---

# 45. Branching

From:

```text
Snapshot S
```

the user may create:

```text
       S
      / \
     A   B
```

Branch A:

```text
mass = 1kg
```

Branch B:

```text
mass = 2kg
```

Both share the same historical ancestor.

This makes interactive experimentation naturally representable.

---

# 46. Experiment History

The runtime can maintain:

```text
Experiment
   │
   ├── initial state
   ├── parameter changes
   ├── interactions
   ├── random seed
   ├── snapshots
   ├── branches
   ├── observations
   └── results
```

This enables:

```text
replay
undo
redo
branch
compare
record
```

without making the visual animation itself the source of truth.

---

# 47. Deterministic Replay

A replay should conceptually reconstruct:

```text
initial state
+
model version
+
parameters
+
random source
+
external inputs
+
execution configuration
```

and reproduce the same transition sequence within the runtime's defined numerical guarantees.

External nondeterministic inputs must therefore either be:

```text
recorded
```

or:

```text
excluded from deterministic replay guarantees
```

Examples:

```text
live sensor
network packet
wall-clock input
user interaction
```

may need event recording.

---

# 48. External Inputs

External inputs enter through an explicit boundary:

```text
EXTERNAL WORLD
      ↓
INPUT
      ↓
RUNTIME
      ↓
MODEL
```

Examples:

```text
mouse
keyboard
sensor
microcontroller
file
network
camera
live dataset
```

The runtime should timestamp or sequence external inputs when reproducibility matters.

---

# 49. Multiple Solvers

A system may contain multiple computational mechanisms.

Example:

```text
fluid subsystem → PDE solver
particle subsystem → particle integrator
reaction subsystem → ODE solver
random subsystem → stochastic process
```

They must coexist under a common runtime.

The runtime therefore needs:

```text
execution domains
```

and synchronization boundaries.

---

# 50. Coupled Solvers

Suppose:

```text
Particle System
      ↕
Electric Field
```

The particle system depends on the field:

```text
E(x) → force → particle motion
```

while particles generate the field:

```text
charges → E(x)
```

This creates:

```text
particles
   ↓
field
   ↓
forces
   ↓
particles
```

The runtime may need:

```text
iteration
```

until convergence or until a defined coupling step completes.

This is a runtime concern, not a requirement that the DSL expose a particular numerical method.

---

# 51. Synchronization Domains

Complex models may have different update rates.

For example:

```text
electromagnetic field: very small dt
particles: medium dt
population model: large dt
UI: 60 FPS
```

The runtime therefore needs synchronization points.

Conceptually:

```text
Subsystem A ──┐
              ├── synchronization barrier
Subsystem B ──┘
```

Synchronization should occur where semantic consistency requires it, not merely because rendering happens.

---

# 52. Parallel Execution

Independent computations may execute concurrently.

For example:

```text
particle 1 ─┐
particle 2 ─┤
particle 3 ─┼→ next state
particle 4 ─┤
particle 5 ─┘
```

provided their dependencies permit it.

The semantic result must not depend on arbitrary scheduling.

Therefore:

> Parallelism is an execution optimization unless the model explicitly contains concurrency semantics.

---

# 53. GPU Execution

GPU computation is similarly an execution strategy.

A model may say:

```text
heat(x,y,t)
```

while the runtime decides:

```text
CPU
GPU
CPU + GPU
```

The semantic model should remain unchanged.

GPU buffers belong to computational state.

---

# 54. Numerical Error

Numerical execution is approximate.

The runtime should distinguish:

```text
mathematical state
```

from:

```text
numerical approximation
```

For example:

```text
exact:
π

computed:
3.14159265...
```

and:

```text
exact model:
dN/dt = -λN

numerical state:
Nₙ
```

The runtime may expose:

```text
estimated error
relative error
absolute error
tolerance
residual
convergence status
```

---

# 55. Numerical Failure

A numerical computation may fail because of:

```text
divergence
overflow
underflow
NaN
singular matrix
non-convergence
unstable timestep
invalid domain
```

These should produce explicit runtime diagnostics.

The runtime must not silently convert:

```text
NaN
```

into:

```text
0
```

unless the model or execution policy explicitly specifies such behavior.

---

# 56. Semantic Failure vs Computational Failure

These must remain distinct.

### Semantic failure

The model itself is invalid.

Example:

```text
square_root(-1)
```

under a real-valued domain.

### Constraint failure

The resulting state violates a model constraint.

### Computational failure

The model is valid, but the selected execution strategy failed.

Example:

```text
solver did not converge
```

### Runtime failure

The execution environment failed.

Example:

```text
GPU device lost
```

### Presentation failure

The model executed correctly but could not be represented/rendered.

Example:

```text
unsupported representation
```

This distinction is important for debugging and authoring tools.

---

# 57. Runtime Pipeline

The complete conceptual runtime pipeline is:

```text
MODEL
  ↓
INITIALIZE
  ↓
INITIAL STATE
  ↓
SCHEDULE
  ↓
EVALUATE
  ↓
PRODUCE CHANGES
  ↓
RESOLVE
  ↓
VALIDATE
  ↓
COMMIT
  ↓
OBSERVE
  ↓
UPDATE PROJECTIONS
  ↓
PRESENT
  ↓
WAIT FOR NEXT TRANSITION / INPUT
```

For hybrid systems:

```text
             ┌───────────────┐
             │               ↓
INITIAL → CONTINUOUS → EVENT DETECTION
             ↑               │
             │               ↓
             └── COMMIT ← EVENT TRANSITION
```

---

# 58. General Runtime Cycle

A generalized runtime cycle can therefore be defined as:

```text
1. Determine next execution boundary
2. Gather external inputs
3. Evaluate applicable behaviors
4. Detect or process events
5. Produce candidate changes
6. Resolve competing changes
7. Apply constraints
8. Validate candidate state
9. Commit semantic state
10. Advance simulation time
11. Execute observations
12. Invalidate/update dependent projections
13. Update presentation
14. Record experiment history
15. Repeat
```

Not every model requires every phase.

For example:

```text
f(x) = x²
```

may require:

```text
evaluate
→ project
```

but no simulation cycle.

---

# 59. Static Models

The runtime must support models that never evolve.

Example:

```text
f(x) = x²
```

Execution can simply be:

```text
construct model
→ evaluate function
→ observe/project
→ render
```

No simulation clock is required.

This prevents the framework from becoming unnecessarily simulation-centric.

---

# 60. Pure Animation

A purely explanatory animation may bypass model evolution.

Example:

```text
circle appears
→ moves right
→ changes color
→ label appears
```

This is:

```text
presentation state
→ animation
→ presentation state
```

It does not require a scientific model.

The runtime should therefore support two fundamental execution modes:

```text
MODEL-DRIVEN
ANIMATION-DRIVEN
```

and a hybrid:

```text
MODEL + ANIMATION
```

---

# 61. Model-Driven Animation

A simulation may drive an animation:

```text
model.position
      ↓
projection
      ↓
screen position
```

while an author may independently animate:

```text
label.opacity
camera.zoom
highlight
```

Both can coexist.

---

# 62. Interaction + Simulation + Animation

The full interactive system becomes:

```text
                    ┌───────────────┐
                    │               │
                    ↓               │
              SEMANTIC STATE       │
                    │               │
          ┌─────────┼─────────┐     │
          ↓         ↓         ↓     │
      EVOLUTION  OBSERVATION  PROJECTION
          │         │         │
          ↓         ↓         ↓
       STATE'      DATA    REPRESENTATION
                              │
                              ↓
                             VIEW
                              │
                              ↓
                            USER
                              │
                              ↓
                         INTERACTION
                              │
                              └──────────→ STATE
```

Animation operates primarily on the presentation side:

```text
VIEW
 ↓
ANIMATION
 ↓
PRESENTATION STATE
```

---

# 63. Dependency Invalidation

The runtime should maintain dependency information.

Example:

```text
mass
velocity
   ↓
kinetic_energy
   ↓
energy_graph
```

If velocity changes:

```text
velocity changed
       ↓
kinetic_energy invalid
       ↓
energy_graph invalid
```

But:

```text
title
```

remains valid.

This permits incremental updates.

---

# 64. Lazy Evaluation

Not every derived value must be calculated immediately.

For example:

```text
kinetic_energy
```

may only be needed when:

```text
graph
```

requests it.

Therefore:

```text
derived value
```

can be evaluated lazily.

The runtime may use:

```text
dependency graph
+
cache
+
invalidation
```

to avoid unnecessary computation.

---

# 65. Caching

Computational caches may store:

```text
derived values
field samples
geometry
solver intermediates
render buffers
```

But cache contents are not automatically semantic state.

If a cache is lost:

```text
semantic model remains valid
```

and the runtime reconstructs the cache.

---

# 66. Transactional Semantics

A useful overarching principle emerges:

> A semantic transition behaves like a transaction.

Conceptually:

```text
BEGIN
   ↓
READ
   ↓
COMPUTE
   ↓
PROPOSE
   ↓
RESOLVE
   ↓
VALIDATE
   ↓
COMMIT
```

If validation fails:

```text
ROLLBACK
```

The previous committed semantic state remains authoritative.

This gives the runtime a stable foundation for:

- deterministic execution
- branching
- undo/redo
- replay
- debugging
- multi-view synchronization
- parallel execution

---

# 67. Runtime Determinism

There are three useful determinism levels.

### Strong deterministic

Same:

```text
model
state
inputs
execution configuration
```

produces exactly the same represented numerical state.

### Numerical deterministic

Results are equivalent within defined numerical tolerances.

### Non-deterministic

Execution may legitimately differ because of:

```text
uncontrolled randomness
external inputs
parallel race semantics
real-world data
```

The runtime should make the determinism level explicit.

---

# 68. Reproducibility Record

A reproducible experiment should be representable as:

```text
ReproducibilityRecord
├── model identity
├── model version
├── initial state
├── parameters
├── simulation clock
├── random source
├── random seed/state
├── execution strategy
├── numerical configuration
├── external input log
└── runtime version/configuration
```

This becomes particularly valuable for educational and scientific use.

---

# 69. Rendering and Simulation Decoupling

The complete relationship should be:

```text                MODEL
                  /       \
                 /         \
                ↓           ↓
          SIMULATION     OBSERVATION
              ↓              ↓
           STATE'           DATA
              ↓              ↓
              └──────┬───────┘
                     ↓
                 PROJECTION
                     ↓
               REPRESENTATION
                     ↓
                    VIEW
```

Rendering is therefore downstream of semantic state.

The renderer should never become the authoritative source of scientific state.

---

# 70. Runtime Modes

The semantics support several runtime modes.

```text
STATIC
DISCRETE
CONTINUOUS
EVENT_DRIVEN
STOCHASTIC
HYBRID
INTERACTIVE
ANIMATION
EXPERIMENT
REPLAY
```

These are execution modes, not separate foundational models.

A single system may combine several:

```text
HYBRID
├── continuous
├── discrete
├── event-driven
├── stochastic
├── interactive
└── multi-view
```

---

# 71. Minimal Runtime Kernel

Despite the large capability surface, the conceptual runtime kernel can remain small.

```text
RUNTIME KERNEL
├── State Store
├── Transition Engine
├── Dependency Engine
├── Event Scheduler
├── Clock
├── Random Source
├── Constraint/Validation Engine
├── Observation Scheduler
├── Projection Invalidation
└── Experiment History
```

Solvers, renderers, GPU execution, and domain engines plug into this kernel.

---

# 72. Execution Strategy as a Separate Layer

A model should not specify:

```text
"use RK4"
```

unless the author explicitly wants to control execution.

Instead:

```text
MODEL
dS/dt = f(S,t)

EXECUTION
solver = adaptive_rk
tolerance = ...
```

Likewise:

```text
MODEL
particle interaction
```

does not require:

```text
spatial_hash
```

or:

```text
BVH
```

to be part of the model.

---

# 73. Runtime Contract

The runtime can therefore be viewed as implementing a contract:

```text
Given:

M  = Model
S₀ = Initial State
C  = Context
I  = Inputs
E  = Execution Strategy

produce:

S₁, S₂...
O₁, O₂...
P₁, P₂...
```

where:

```text
S = semantic states
O = observations/data
P = presentation/projections
```

The execution strategy may change **how** the results are calculated without changing the intended model semantics.

---

# 74. The Most Important Runtime Invariants

The runtime should preserve these invariants:

```text
1. Semantic state is authoritative.

2. A transition does not partially commit semantic state.

3. Execution order must not introduce unintended model meaning.

4. Model semantics are independent of rendering frequency.

5. Simulation time is independent of presentation time.

6. Randomness is explicitly controllable when reproducibility is required.

7. Computational state is not automatically semantic state.

8. Presentation state is not semantic state.

9. Observation does not mutate the model unless explicitly defined as intervention.

10. Solver choice does not redefine the model.

11. Failed transitions do not silently corrupt committed state.

12. Dynamic creation/destruction is semantically supported.

13. Multiple representations may observe the same state.

14. Multiple computational strategies may execute the same model.

15. Domain concepts remain composable library abstractions rather than core assumptions.
```

---

# 75. Refined Architecture

The complete architecture is now:

```text
                    DOMAIN LIBRARIES
             ┌───────────────────────────┐
             │ Physics / Chemistry /     │
             │ Biology / CS / Mathematics│
             └─────────────┬─────────────┘
                           ↓
                    ┌─────────────┐
                    │    MODEL    │
                    │             │
                    │ values      │
                    │ objects     │
                    │ relations   │
                    │ fields      │
                    │ behaviors   │
                    │ constraints │
                    └──────┬──────┘
                           ↓
                    ┌─────────────┐
                    │   RUNTIME   │
                    │             │
                    │ state       │
                    │ transitions │
                    │ events      │
                    │ clocks      │
                    │ solvers     │
                    │ randomness  │
                    │ validation  │
                    │ observations│
                    │ replay      │
                    └──────┬──────┘
                           ↓
                    ┌─────────────┐
                    │ PROJECTION  │
                    │             │
                    │ mapping     │
                    │ dependency  │
                    │ invalidation│
                    └──────┬──────┘
                           ↓
                    ┌─────────────┐
                    │    VIEW     │
                    │             │
                    │ geometry    │
                    │ graph       │
                    │ equation    │
                    │ field       │
                    │ table       │
                    │ statistics  │
                    └──────┬──────┘
                           ↓
                    ┌─────────────┐
                    │ PRESENTATION│
                    │             │
                    │ animation   │
                    │ camera      │
                    │ interaction │
                    │ rendering   │
                    └─────────────┘
```

---

# 76. What Runtime Semantics Has Established

At this point we can state precisely:

### The model defines

```text
WHAT EXISTS
WHAT IT MEANS
WHAT RELATIONSHIPS HOLD
WHAT BEHAVIORS ARE DEFINED
```

### The runtime defines

```text
HOW STATE EVOLVES
WHEN IT EVOLVES
HOW EVENTS OCCUR
HOW RANDOMNESS IS CONTROLLED
HOW CONFLICTS ARE RESOLVED
HOW CONSTRAINTS ARE ENFORCED
HOW OBSERVATIONS ARE SCHEDULED
HOW STATE IS COMMITTED
HOW EXECUTION IS REPLAYED
```

### The presentation layer defines

```text
HOW STATE IS PROJECTED
HOW IT IS REPRESENTED
HOW IT IS ANIMATED
HOW IT IS RENDERED
```

This gives us a clean separation:

```text
             MEANING
                ↓
              MODEL
                ↓
             EXECUTION
                ↓
             RUNTIME
                ↓
          OBSERVATION /
           PROJECTION
                ↓
          PRESENTATION
```

---

# 77. The Resulting Semantic Stack

The project now has a progressively refined stack:

```text
CASE SPACE
    ↓
CAPABILITIES
    ↓
CAPABILITY TAXONOMY
    ↓
GENERALIZED MODEL
    ↓
MODEL VALIDATION
    ↓
GAP ANALYSIS
    ↓
REFINED CORE MODEL
    ↓
RUNTIME SEMANTICS
```

The next question is no longer:

> "What features should our DSL have?"

It is:

> **"How should these semantic concepts be expressed by an author?"**

That is the point at which the DSL can finally be designed.

Before syntax, however, the runtime semantics should be treated as the contract that the DSL ultimately compiles into.

Therefore the next artifact is:

```text
07 DSL Design
```

It should derive the language from the semantics rather than inventing syntax first.

Its key questions will be:

```text
How does an author define a model?

How does an author define state?

How are quantities and units written?

How are entities and collections declared?

How are relations represented?

How are fields defined?

How are equations expressed?

How are rules and events authored?

How are continuous processes declared?

How are stochastic processes expressed?

How are observations and datasets declared?

How are projections connected to model state?

How are views composed?

How are interactions declared?

How are experiments and branches authored?

How are animations expressed without confusing them with simulation?

How does the language allow domain libraries to extend the core?

And ultimately:

Can the resulting language express every case in the original catalogue
without making the language itself domain-specific?
```
