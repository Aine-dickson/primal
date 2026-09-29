# Prismal
## 05 Model Validation

### Purpose

This document validates the generalized model defined in `04-generalized-model.md`.

The goal is not to prove that every case can already be implemented. The goal is to determine whether the generalized model is **expressively sufficient** for the case space identified in the Test Case Catalogue and Capability/Case Matrix.

For every test case, we ask:

> **Can this case be expressed using the generalized model without introducing a new domain-specific fundamental concept?**

If not, the missing capability must be classified as one of:

1. **Fundamental capability missing from the generalized model**
2. **Composable capability that can be constructed from existing primitives**
3. **Higher-level domain/library concept that should not belong in the core model**
4. **Representation/runtime capability missing rather than model capability**
5. **Computation capability missing rather than semantic model capability**

The distinction is important. A failure to express projectile motion does not automatically mean that `Projectile` should become a primitive. It may simply mean that vectors, differential equations, numerical integration, and spatial entities have not been composed correctly.

---

# 1. Validation Criteria

A test case passes validation when the generalized model can describe:

### 1.1 Semantic structure

What exists?

- entities
- properties
- values
- relations
- variables
- fields
- parameters
- constraints

### 1.2 Behavior

What determines what happens?

- functions
- equations
- rules
- processes
- events
- constraints

### 1.3 Evolution

How does state change?

- continuous evolution
- discrete transitions
- event-driven changes
- stochastic changes
- hybrid evolution

### 1.4 Context

Where and when does it exist?

- space
- coordinate systems
- time
- simulation clock
- spatial relationships

### 1.5 Observation

What can be measured or extracted?

- measurements
- probes
- derived quantities
- datasets
- statistics

### 1.6 Presentation

How can the model be shown?

- geometry
- graph
- diagram
- equation
- text
- particles
- field visualization
- timeline
- table
- statistical visualization
- multiple synchronized views

### 1.7 Interaction

How can the user affect or investigate it?

- parameters
- controls
- manipulation
- measurement
- experimentation
- stepping
- branching
- state restoration

### 1.8 Computation

How is the model evaluated?

- analytical evaluation
- numerical evaluation
- integration
- solving
- iteration
- sampling
- aggregation
- optimization
- constraint solving

---

# 2. Validation Classification

Each test case receives one of the following classifications.

| Classification | Meaning |
|---|---|
| **PASS** | Fully expressible using the current generalized model |
| **PASS: COMPOSED** | Expressible by composing existing primitives |
| **PASS: LIBRARY** | Requires a useful domain abstraction, but not a core primitive |
| **PARTIAL** | Core semantics are expressible but an important computational/runtime capability is missing |
| **FAIL: CORE** | A genuinely fundamental modeling capability is missing |
| **FAIL: COMPUTATION** | Model is expressible, but required computation is absent |
| **FAIL: REPRESENTATION** | Model is expressible, but required visualization is absent |
| **FAIL: INTERACTION** | Model is expressible, but required interaction mechanism is absent |

---

# 3. Validation Test 1 Pure Mathematical Function

## Case

\[
f(x)=x^2
\]

Domain:

\[
x\in[-5,5]
\]

Desired presentation:

- coordinate axes
- curve
- equation
- optional moving point
- optional tangent
- optional derivative

## Model

```text
System
├── Function f
│   └── f(x) = x²
├── Domain
│   └── [-5, 5]
└── Observation
    └── f(x)
```

The function is an existing mathematical primitive.

No simulation is required.

The graph is a projection:

```text
Function + Domain
        ↓
    Projection
        ↓
    Graph View
```

## Result

**PASS: COMPOSED**

Important finding:

> The system must not require a simulation loop for static mathematical visualization.

A static function is a valid model.

---

# 4. Validation Test 2 Projectile Motion

## Case

A projectile is launched with initial velocity \(v_0\) at angle \(\theta\).

Ignoring air resistance:

\[
\frac{d\mathbf{x}}{dt}=\mathbf{v}
\]

\[
\frac{d\mathbf{v}}{dt}=\mathbf{g}
\]

## Model

```text
Entity
└── projectile

Properties
├── position : Vector2
├── velocity : Vector2
└── mass : Quantity

Parameters
├── initial_velocity
├── launch_angle
└── gravity

Space
└── Cartesian2D

Behavior
├── dx/dt = velocity
└── dv/dt = gravity

Computation
└── numerical integration
```

Representations:

```text
Model
├── spatial trajectory
├── velocity vector
├── equation
├── position-time graph
└── measurement readout
```

## Result

**PASS: COMPOSED**

`Projectile` does not need to be a fundamental primitive.

It is simply:

```text
Entity
+ position
+ velocity
+ acceleration
+ equations
+ numerical integration
```

A library may provide a convenient `projectile()` abstraction, but the core does not need it.

---

# 5. Validation Test 3 Pendulum

## Case

A pendulum with angle \(\theta\).

Idealized equation:

\[
\frac{d^2\theta}{dt^2}
=
-\frac{g}{L}\sin\theta
\]

## Model

```text
Entity
└── pendulum

Properties
├── angle
├── angular_velocity
└── length

Parameters
├── gravity
└── mass

Behavior
└── differential equation

Computation
└── numerical integration
```

Geometry can be derived from:

```text
pivot
+
length
+
angle
```

## Result

**PASS: COMPOSED**

No `Pendulum` primitive is required.

---

# 6. Validation Test 4 Collision Simulation

## Case

Two moving bodies collide.

Required capabilities:

- position
- velocity
- mass
- geometry
- collision detection
- event
- state transition
- conservation equations

## Model

```text
Entities
├── A
└── B

Properties
├── position
├── velocity
├── mass
└── geometry

Event
└── collision(A, B)

Rule
└── collision response

Constraints
├── momentum conservation
└── optional energy conservation
```

## Result

**PASS: COMPOSED**

However, this exposes an important distinction:

> Collision detection is a computational capability, while collision itself is a model event.

Therefore:

```text
Event
    ↓
requires
    ↓
Geometric relation / collision detection
```

Collision detection should not automatically become a semantic primitive called `CollisionEngine`.

---

# 7. Validation Test 5 Electric Field

## Case

Electric charges produce a field:

\[
\mathbf{E}(\mathbf{x})
=
\frac{1}{4\pi\epsilon_0}
\frac{q}{r^2}\hat{\mathbf r}
\]

## Model

```text
Entities
└── charges

Properties
├── position
└── charge

Field
└── E(x, y)

Expression
└── Coulomb field equation
```

The field exists over space independently of the visual representation.

Possible projections:

```text
E(x,y)
├── vector arrows
├── magnitude heatmap
├── field lines
├── equipotential contours
└── measurement probe
```

## Result

**PASS: COMPOSED**

This validates that fields must be first-class model constructs.

---

# 8. Validation Test 6 Wave Propagation

## Case

A wave described by:

\[
\frac{\partial^2 u}{\partial t^2}
=
c^2
\frac{\partial^2 u}{\partial x^2}
\]

## Model

```text
Field
└── u(x,t)

Parameter
└── c

Equation
└── wave equation

Computation
└── numerical PDE solver
```

Representations:

- spatial waveform
- surface
- heatmap
- time graph
- animated wave
- probe measurement

## Result

**PASS: SEMANTICALLY**

**PARTIAL: COMPUTATION**

The generalized model can describe the field and equation, but a practical implementation requires a PDE/numerical-field computation capability.

Important conclusion:

> A missing numerical solver is not a missing model primitive.

---

# 9. Validation Test 7 Heat Diffusion

## Case

\[
\frac{\partial T}{\partial t}
=
\alpha\nabla^2T
\]

## Model

```text
Field
└── temperature(x,y,t)

Parameter
└── diffusivity

Equation
└── heat equation

Constraints
└── boundary conditions
```

## Result

**PASS: SEMANTICALLY**

**PARTIAL: COMPUTATION**

This exposes the need for:

- spatial discretization
- PDE solving
- boundary conditions
- numerical stability handling

These belong primarily to the computation/runtime layer.

---

# 10. Validation Test 8 Ideal Gas

## Case

A collection of particles in a container.

Possible models:

### Microscopic model

```text
Particles
├── position
├── velocity
└── mass

Interactions
└── collisions

Boundary
└── container
```

### Macroscopic model

```text
Variables
├── pressure
├── volume
├── temperature
└── amount

Equation
└── PV = nRT
```

### Statistical model

```text
Particle observations
        ↓
velocity distribution
        ↓
temperature / statistics
```

## Result

**PASS: COMPOSED**

Critical finding:

> Multiple models may represent the same scientific concept.

The system must not force microscopic and macroscopic descriptions into one model.

---

# 11. Validation Test 9 Microscopic Radioactive Decay

## Case

A collection of unstable nuclei.

Each nucleus has a probability of decaying.

A stochastic event occurs:

```text
nucleus
    ↓
decay event
    ↓
new state
```

## Model

```text
Entities
└── nuclei

Property
└── unstable

Parameter
└── decay_constant λ

Process
└── stochastic decay

Event
└── decay(nucleus)

Randomness
└── seeded RNG
```

## Result

**PASS: COMPOSED**

This validates first-class:

- stochastic processes
- events
- randomness
- reproducible seeds

---

# 12. Validation Test 10 Macroscopic Radioactive Decay

## Case

\[
\frac{dN}{dt}=-\lambda N
\]

Solution:

\[
N(t)=N_0e^{-\lambda t}
\]

## Model

```text
Variable
└── N

Parameter
└── λ

Equation
└── dN/dt = -λN
```

No individual nuclei are required.

## Result

**PASS: COMPOSED**

Together, Tests 9 and 10 establish:

> Deterministic and stochastic models must both be first-class, while neither should be assumed to be the universal representation of a concept.

---

# 13. Validation Test 11 Radioactive Decay Chain

## Case

\[
A\rightarrow B\rightarrow C
\]

Each transition may have its own decay constant.

## Model

```text
Entities / Species
├── A
├── B
└── C

Relations
├── A → B
└── B → C

Processes
├── decay(A)
└── decay(B)

Parameters
├── λA
└── λB
```

## Result

**PASS: COMPOSED**

This validates dynamic relations and multiple coupled processes.

---

# 14. Validation Test 12 Chemical Reaction

## Case

\[
A+B\rightarrow C
\]

Possible rate law:

\[
r=k[A][B]
\]

## Model

```text
Entities / Species
├── A
├── B
└── C

Relation
└── reaction

Parameters
└── k

Expression
└── r = k[A][B]

Process
└── concentration evolution
```

## Result

**PASS: COMPOSED**

A `Reaction` object may be a library abstraction, but the underlying model is:

```text
entities
+
relations
+
quantities
+
expressions
+
processes
```

---

# 15. Validation Test 13 Chemical Equilibrium

## Case

\[
A+B\rightleftharpoons C
\]

with forward and reverse processes.

## Model

```text
Processes
├── forward reaction
└── reverse reaction

Parameters
├── k_forward
└── k_reverse

State
└── concentrations

Observation
└── concentration over time
```

## Result

**PASS: COMPOSED**

This validates coupled processes and equilibrium behavior.

---

# 16. Validation Test 14 Molecular Structure

## Case

A molecule such as water.

```text
O
|
H-H
```

Conceptually:

```text
Entities
├── atoms
└── bonds

Properties
├── element
├── position
└── charge

Relations
└── bond
```

Representation:

```text
atoms → circles
bonds → lines
labels → text
```

## Result

**PASS: COMPOSED**

This validates graph-like relations between entities.

---

# 17. Validation Test 15 Population Dynamics

## Case

\[
\frac{dP}{dt}=rP
\]

or logistic growth:

\[
\frac{dP}{dt}
=
rP\left(1-\frac{P}{K}\right)
\]

## Model

```text
Variable
└── population

Parameters
├── r
└── K

Equation
└── population evolution
```

## Result

**PASS: COMPOSED**

No biological primitive is required.

---

# 18. Validation Test 16 Predator-Prey System

## Case

Lotka-Volterra equations:

\[
\frac{dx}{dt}=\alpha x-\beta xy
\]

\[
\frac{dy}{dt}=\delta xy-\gamma y
\]

## Model

```text
Variables
├── prey
└── predator

Parameters
├── α
├── β
├── γ
└── δ

Equations
├── prey evolution
└── predator evolution
```

Representations:

- population graph
- phase portrait
- particle/agent approximation
- parameter exploration

## Result

**PASS: COMPOSED**

Again, `PredatorPrey` belongs at the domain/library level.

---

# 19. Validation Test 17 Cellular Automaton

## Case

A grid whose cells change according to local rules.

For example:

```text
cell state
    ↓
neighbor states
    ↓
rule
    ↓
next cell state
```

## Model

```text
Space
└── discrete grid

Entities
└── cells

Property
└── state

Relation
└── neighborhood

Rule
└── transition function

Time
└── discrete timestep
```

## Result

**PASS: COMPOSED**

This validates:

- discrete space
- discrete time
- local relations
- rule-based evolution

---

# 20. Validation Test 18 Graph Traversal

## Case

Breadth-first search or depth-first search.

```text
Graph
├── nodes
└── edges

Algorithm state
├── current node
├── visited
└── frontier
```

At each step:

```text
state
 ↓
algorithm rule
 ↓
new state
```

## Result

**PASS: COMPOSED**

Important result:

> The generalized model is not inherently a physics model.

Algorithms can be modeled as discrete state transitions.

---

# 21. Validation Test 19 Sorting Algorithm

## Case

Visualize:

```text
[5, 2, 8, 1, 3]
```

being sorted.

## Model

```text
State
└── sequence

Algorithm
└── transition rules

Step
└── discrete evolution

Observation
└── comparisons/swaps
```

Representation:

```text
array
├── values
├── indices
└── highlighted elements
```

## Result

**PASS: COMPOSED**

This strongly validates the generalized state-transition model.

---

# 22. Validation Test 20 Stochastic Particle Simulation

## Case

Particles undergo random motion.

For example:

```text
position(t + Δt)
=
position(t)
+
random_displacement
```

## Model

```text
Entities
└── particles

Properties
└── position

Process
└── stochastic motion

Randomness
└── seeded RNG

Time
└── discrete timestep
```

## Result

**PASS: COMPOSED**

---

# 23. Validation Test 21 Hybrid Particle + Field System

## Case

Charged particles moving through an electromagnetic field.

```text
Particles
      ↓
charge/current
      ↓
Field
      ↓
force on particles
      ↓
Particle evolution
      ↓
new field
```

## Model

```text
Entities
└── particles

Fields
├── electric field
└── magnetic field

Relations
└── particle ↔ field coupling

Equations
├── field equations
└── particle equations

Processes
├── field evolution
└── particle evolution
```

## Result

**PASS: COMPOSED**

This is a critical validation.

The generalized model supports systems where:

```text
entity-based dynamics
+
field-based dynamics
+
coupling
```

occur simultaneously.

---

# 24. Validation Test 22 Multi-View Educational Experiment

## Case

A student changes the launch angle of a projectile.

The system simultaneously displays:

```text
             Spatial view
                  │
                  ├── projectile
                  └── trajectory

Model ────────────┤

                  ├── velocity graph
                  ├── equation
                  ├── numerical values
                  └── prediction question
```

The student:

1. changes angle
2. predicts range
3. runs simulation
4. measures result
5. compares prediction
6. repeats experiment

## Result

**PASS: COMPOSED**

This validates:

- multiple views
- synchronized projections
- interaction
- experiment procedures
- observations
- data
- educational structures

---

# 25. Validation Test 23 Pure Explanatory Animation

## Case

Explain the Pythagorean theorem without simulating a physical system.

Example sequence:

```text
triangle appears
        ↓
sides highlight
        ↓
squares appear
        ↓
areas transform
        ↓
equation appears
```

There may be no evolving scientific state.

## Model

```text
Mathematical content
+
Representations
+
Animation timeline
```

## Result

**PASS: COMPOSED**

This confirms:

> Animation is not synonymous with simulation.

The runtime must support both:

```text
simulation-driven animation
```

and

```text
author-driven animation
```

---

# 26. Validation Test 24 Interactive Parameter Exploration

## Case

A user changes:

```text
gravity = 9.81
angle = 45°
velocity = 20 m/s
```

The simulation updates.

## Model

```text
Parameters
├── gravity
├── angle
└── velocity

Interaction
└── parameter modification

Evolution
└── recompute model

Observation
└── range / height / trajectory
```

## Result

**PASS: COMPOSED**

This validates parameterized experiments.

---

# 27. Summary Matrix

| Test Case | Model | Evolution | Computation | Representation | Result |
|---|---:|---:|---:|---:|---|
| Function plot | ✓ | - | ✓ | ✓ | **PASS** |
| Projectile | ✓ | ✓ | ✓ | ✓ | **PASS** |
| Pendulum | ✓ | ✓ | ✓ | ✓ | **PASS** |
| Collision | ✓ | ✓ | ✓ | ✓ | **PASS** |
| Electric field | ✓ | ✓/- | ✓ | ✓ | **PASS** |
| Wave | ✓ | ✓ | △ | ✓ | **PARTIAL** |
| Heat diffusion | ✓ | ✓ | △ | ✓ | **PARTIAL** |
| Ideal gas | ✓ | ✓ | ✓ | ✓ | **PASS** |
| Microscopic decay | ✓ | ✓ | ✓ | ✓ | **PASS** |
| Macroscopic decay | ✓ | ✓ | ✓ | ✓ | **PASS** |
| Decay chain | ✓ | ✓ | ✓ | ✓ | **PASS** |
| Chemical reaction | ✓ | ✓ | ✓ | ✓ | **PASS** |
| Chemical equilibrium | ✓ | ✓ | ✓ | ✓ | **PASS** |
| Molecular structure | ✓ | - | - | ✓ | **PASS** |
| Population dynamics | ✓ | ✓ | ✓ | ✓ | **PASS** |
| Predator-prey | ✓ | ✓ | ✓ | ✓ | **PASS** |
| Cellular automaton | ✓ | ✓ | ✓ | ✓ | **PASS** |
| Graph traversal | ✓ | ✓ | ✓ | ✓ | **PASS** |
| Sorting | ✓ | ✓ | ✓ | ✓ | **PASS** |
| Stochastic particles | ✓ | ✓ | ✓ | ✓ | **PASS** |
| Particle + field | ✓ | ✓ | △ | ✓ | **PASS / COMPUTATION-DEPENDENT** |
| Educational experiment | ✓ | ✓ | ✓ | ✓ | **PASS** |
| Explanatory animation | ✓ | - | - | ✓ | **PASS** |
| Parameter exploration | ✓ | ✓ | ✓ | ✓ | **PASS** |

Legend:

```text
✓  supported by generalized model
not required
△  requires a specialized computational capability
```

---

# 28. What the Validation Reveals

The test cases reveal several important properties of the generalized model.

## 28.1 Domain objects do not need to be core primitives

These can all be constructed:

```text
Projectile
Pendulum
Molecule
Reaction
Predator
Prey
Nucleus
Cell
Particle
Graph Node
Algorithm State
```

from more general concepts.

Therefore they should initially be treated as **libraries/domain abstractions**, not fundamental DSL primitives.

---

# 29. Fields Must Be First-Class

The validation would be incomplete without first-class fields.

Examples include:

```text
temperature(x,y,t)
pressure(x,y,t)
electric_field(x,y,t)
magnetic_field(x,y,t)
velocity(x,y,t)
probability_density(x,y,t)
```

A field is fundamentally different from a collection of independent entities.

Therefore:

> **Field is a core model capability.**

---

# 30. Relations Must Be First-Class

Relations appear in:

- chemical bonds
- graph edges
- reaction pathways
- predator-prey interactions
- network connections
- constraints
- particle interactions
- parent/child structures
- dependencies

Relations may also contain state:

```text
edge.weight
bond.order
connection.capacity
```

Therefore:

> **Relation is a core model capability.**

---

# 31. Events Must Be First-Class

Events occur in:

- collisions
- radioactive decay
- chemical reactions
- threshold crossings
- object creation
- object destruction
- algorithm steps
- user interaction

An event is not simply an animation.

It is a semantic occurrence that may change state.

Therefore:

> **Event is a core model capability.**

---

# 32. Randomness Must Be Explicit

Stochastic cases require:

```text
random variable
random process
probability distribution
seed
sampling
```

Randomness must not be an invisible side effect of the runtime.

For reproducibility:

```text
experiment
+
seed
+
initial state
+
parameters
```

should be sufficient to reproduce a run, where the underlying computation permits deterministic replay.

Therefore:

> **Controlled randomness is a core runtime capability.**

---

# 33. Time Must Be More General Than Timesteps

The validation includes:

- static systems
- continuous systems
- discrete systems
- event-driven systems
- stochastic processes
- accelerated simulations
- explanatory animations

Therefore the model cannot assume:

```text
t = frame_number
```

or:

```text
one frame = one simulation step
```

Instead:

```text
simulation time
```

and:

```text
presentation time
```

must remain distinct.

---

# 34. Computation Is a Separate Layer

Several cases expose a distinction between:

```text
"What does the model mean?"
```

and:

```text
"How do we calculate it?"
```

For example:

```text
heat equation
```

may be semantically valid even when a PDE solver is unavailable.

Likewise:

```text
differential equation
```

does not imply a specific numerical integration algorithm.

Therefore:

```text
MODEL
   ↓
mathematical semantics
   ↓
COMPUTATION STRATEGY
   ↓
STATE EVOLUTION
```

should remain conceptually separate.

---

# 35. Representation Is Also Separate

The same model may produce:

```text
spatial scene
graph
equation
table
timeline
histogram
phase portrait
particle visualization
field visualization
text explanation
```

Therefore:

> A representation must not define the model.

A useful conceptual chain remains:

```text
MODEL
  ↓
OBSERVATION / DATA
  ↓
PROJECTION
  ↓
REPRESENTATION
  ↓
VIEW
```

---

# 36. One Model Can Have Multiple Synchronized Views

For example:

```text
             ┌── spatial scene
             │
Model ───────┼── equation
             │
             ├── graph
             │
             ├── measurement
             │
             └── statistics
```

All views can observe the same evolving state.

This is essential for educational visualization.

A student's changing parameter should therefore propagate through the model rather than requiring each view to implement its own simulation.

---

# 37. The Model Is Not Necessarily Spatial

The validation includes:

- function plots
- graph algorithms
- sorting
- state machines
- statistics
- abstract mathematical structures

Therefore:

```text
System ≠ Scene
```

and:

```text
Space = optional model capability
```

A system can exist entirely in abstract state space.

---

# 38. The Model Is Not Necessarily Dynamic

The validation includes:

- static geometry
- equations
- molecular structures
- function plots
- diagrams
- explanatory compositions

Therefore:

```text
System ≠ Simulation
```

A simulation is one mode of system evolution.

---

# 39. The Model Is Not Necessarily Entity-Based

The validation contains:

### Entity-centric

```text
particles
atoms
agents
nodes
cells
```

### Field-centric

```text
temperature(x,y)
electric_field(x,y)
pressure(x,y)
```

### Equation-centric

```text
f(x)
dN/dt
PV=nRT
```

### Rule-centric

```text
cellular automata
algorithms
state machines
```

### Hybrid

```text
particles + fields + equations + events
```

Therefore no single one of these can be the universal foundation.

---

# 40. New Fundamental Requirement: Collections

The validation repeatedly requires collections:

```text
particles
atoms
cells
nodes
edges
species
samples
datasets
```

A collection is more than an ordinary entity when the system needs:

- bulk operations
- filtering
- mapping
- aggregation
- dynamic creation/destruction
- population statistics
- iteration

This suggests that **collection/population semantics** should be explicitly investigated as a core capability.

Candidate abstraction:

```text
Collection<T>
```

with operations such as:

```text
filter
map
reduce
count
sample
aggregate
```

However, this should be validated further before declaring it fundamental.

---

# 41. New Fundamental Requirement: Units and Dimensions

Scientific models repeatedly use:

```text
meters
seconds
kilograms
joules
volts
amperes
moles
kelvin
```

and derived quantities:

```text
velocity = distance / time
force = mass × acceleration
```

Therefore the model may need semantic dimensionality.

Example:

```text
distance : Length
time     : Time
velocity : Length / Time
```

This should be investigated as a **core mathematical/scientific capability**, rather than treated merely as display formatting.

---

# 42. New Fundamental Requirement: Domains and Index Spaces

Several models require different kinds of domains:

```text
x ∈ [-1,1]
```

```text
grid[i,j]
```

```text
particle ∈ population
```

```text
node ∈ graph
```

```text
x ∈ ℝ²
```

This suggests the generalized model needs a more explicit notion of:

```text
Domain / Index Space
```

A domain may be:

- continuous
- discrete
- finite
- infinite
- spatial
- abstract
- parameterized

This deserves validation before DSL design.

---

# 43. New Fundamental Requirement: Initial and Boundary Conditions

Differential equations and field simulations require information beyond the equation itself.

For example:

\[
\frac{\partial T}{\partial t}
=
\alpha\nabla^2T
\]

requires:

```text
initial condition
+
boundary conditions
```

Therefore constraints alone may not sufficiently describe all computational problem specifications.

Candidate concepts:

```text
InitialCondition
BoundaryCondition
```

These may ultimately be specializations of a more general constraint/condition mechanism.

This should be investigated rather than immediately promoted to primitives.

---

# 44. New Fundamental Requirement: Discretization

Continuous models frequently need to be computed using discrete approximations.

For example:

```text
continuous field
        ↓
spatial grid
        ↓
finite differences
        ↓
discrete numerical state
```

The semantic model and computational representation may therefore differ.

This leads to an important distinction:

```text
Semantic model
      ↓
Computational model
      ↓
Runtime state
```

A continuous PDE does not necessarily execute as a continuous data structure.

---

# 45. New Fundamental Requirement: Approximation and Error

Numerical models introduce:

```text
approximation
error
tolerance
stability
convergence
precision
```

For example:

```text
exact equation
       ↓
numerical solver
       ↓
approximate result
```

The runtime therefore needs mechanisms for describing or controlling numerical approximation.

This is primarily a **computation/runtime capability**, not a domain-specific semantic primitive.

---

# 46. Important Separation: Semantic State vs Runtime State

Validation suggests that there may need to be more than one notion of "state."

### Semantic state

What the model means:

```text
particle.position
temperature(x,y)
population
```

### Computational state

What the solver stores:

```text
grid buffers
solver coefficients
integration history
random generator state
iteration state
```

### Presentation state

What the view stores:

```text
camera position
zoom
selected object
layout
opacity
animation progress
```

These should not be conflated.

Conceptually:

```text
SEMANTIC STATE
      ↓
COMPUTATIONAL STATE
      ↓
OBSERVATION
      ↓
PRESENTATION STATE
```

---

# 47. Important Separation: Simulation State vs Experiment State

An experiment may contain:

```text
initial state
parameters
random seed
interventions
simulation run
observations
results
```

Therefore an experiment is not merely a simulation.

It is a higher-level procedure around a model.

Candidate structure:

```text
Experiment
├── model
├── initial_state
├── parameters
├── interventions
├── execution
├── observations
└── results
```

This is likely a library/runtime abstraction built from core concepts.

---

# 48. Important Separation: Model vs Domain Knowledge

The core system should know about:

```text
entity
property
relation
field
equation
rule
process
event
constraint
space
time
observation
projection
```

It should not necessarily know about:

```text
gravity
atom
molecule
DNA
predator
prey
projectile
electron
pendulum
Ohm's law
Newton's law
```

These can be defined as domain libraries.

For example:

```text
physics.gravity
chemistry.reaction
biology.population
cs.graph
math.calculus
```

This keeps the core domain-independent.

---

# 49. Validation Result

The generalized model successfully expresses the broad validation set without requiring obvious domain-specific fundamental primitives.

The strongest result is that the following can all fit the same foundation:

```text
f(x)
     ↓
equation
     ↓
particle
     ↓
field
     ↓
chemical system
     ↓
population
     ↓
cellular automaton
     ↓
graph algorithm
     ↓
educational experiment
     ↓
explanatory animation
```

This indicates that the generalized model is viable as a **domain-independent foundation**.

However, several capabilities require further specification.

---

# 50. Current Core Model After Validation

The generalized model should now be considered approximately:

```text
SYSTEM
│
├── STATE
│   ├── entities
│   ├── values
│   ├── variables
│   ├── relations
│   ├── fields
│   └── collections
│
├── BEHAVIOR
│   ├── functions
│   ├── expressions
│   ├── equations
│   ├── rules
│   ├── processes
│   ├── events
│   └── constraints
│
├── CONTEXT
│   ├── space
│   ├── domains
│   └── time
│
├── COMPUTATION
│   ├── evaluation
│   ├── integration
│   ├── solving
│   ├── iteration
│   ├── sampling
│   ├── aggregation
│   ├── discretization
│   └── numerical approximation
│
├── OBSERVATION
│   ├── measurements
│   ├── probes
│   ├── derived quantities
│   └── datasets
│
├── PRESENTATION
│   ├── projections
│   ├── representations
│   └── views
│
├── INTERACTION
│   ├── inputs
│   ├── controls
│   ├── manipulation
│   ├── experimentation
│   └── state management
│
└── ANIMATION
    ├── property animation
    ├── sequencing
    ├── transitions
    └── synchronization
```

---

# 51. Capabilities That Are Now Strong Candidates for Core

Validation has strengthened the case for:

```text
Entity
Value
Property
Variable
Relation
Collection
Field

Expression
Function
Equation
Rule
Process
Event
Constraint

Space
Domain
Time

Observation
Measurement
Dataset

Projection
Representation
View

Parameter
Randomness
```

---

# 52. Capabilities That Should Remain Runtime/Computation Features

These should not automatically become semantic primitives:

```text
ODE solver
PDE solver
collision detector
numerical integrator
linear algebra engine
root finder
optimizer
Monte Carlo engine
GPU execution
parallel execution
spatial indexing
```

They are mechanisms for executing models.

The same semantic model may potentially be executed by different computational strategies.

---

# 53. Capabilities That Should Likely Be Libraries

Examples:

```text
Projectile
Pendulum
Spring
Planet
Atom
Molecule
ChemicalReaction
ElectricCircuit
Wave
Population
PredatorPrey
Graph
BinaryTree
SortingAlgorithm
CellularAutomaton
```

These can provide convenience while remaining expressible through the core model.

---

# 54. Remaining Validation Questions

The current tests do not yet fully validate:

### Mathematical structures

- complex numbers
- tensors
- symbolic algebra
- integrals
- derivatives
- limits
- sets
- proofs
- logic

### Geometry

- arbitrary curves
- topology
- transformations
- intersections
- clipping
- constructive geometry

### Computation

- symbolic solving
- optimization
- large-scale numerical computation
- GPU execution
- distributed computation
- automatic differentiation

### Advanced simulation

- fluid dynamics
- rigid-body systems
- deformable bodies
- finite-element models
- constrained mechanics
- agent-based systems
- multi-scale models

### Data

- large datasets
- streaming data
- time-series analysis
- statistical inference
- uncertainty propagation

### Interaction

- direct manipulation
- construction tools
- measurement instruments
- interactive experimentation
- collaborative state

### Presentation

- arbitrary diagrams
- layout systems
- 3D model projected into 2D
- synchronized multi-panel views
- accessibility
- responsive presentation

These should form the next round of validation.

---

# 55. Most Important Architectural Finding

The validation strongly suggests the system should not be designed as:

```text
DSL
 ↓
Scene graph
 ↓
Renderer
```

Instead, the architecture should conceptually be closer to:

```text
                 ┌───────────────┐
                 │    MODEL      │
                 └───────┬───────┘
                         │
              ┌──────────┴──────────┐
              ↓                     ↓
         COMPUTATION            OBSERVATION
              │                     │
              ↓                     ↓
          STATE EVOLUTION         DATA
              │                     │
              └──────────┬──────────┘
                         ↓
                    PROJECTION
                         ↓
                  REPRESENTATION
                         ↓
                       VIEW
                         ↓
                    INTERACTION
                         │
                         └────────→ MODEL
```

Animation can operate across the presentation side without being confused with the underlying scientific evolution.

---

# 56. Validation Conclusion

The generalized model survives the initial 24-case validation.

No major domain-specific primitive was required.

The failures and partial results primarily reveal missing **computational mechanisms**, not fundamental semantic concepts.

The strongest candidates for additional core investigation are:

```text
1. Collections
2. Units and dimensions
3. Domains / index spaces
4. Initial and boundary conditions
5. Discretization
6. Numerical approximation and error
7. Semantic vs computational state
8. Symbolic mathematics
9. Constraints
10. Uncertainty / distributions
```

These should be tested before freezing the core model.

Therefore the next step should **not yet be DSL syntax**.

The next step should be a deeper **Model Gap Analysis / Extended Validation**, specifically designed around the capabilities that the first validation exposed as uncertain.

Only after those gaps are resolved should the project move to:

```text
Generalized Model
       ↓
Validation
       ↓
Gap Analysis
       ↓
Refined Core Model
       ↓
Runtime Semantics
       ↓
DSL Design
```

The DSL should be derived from the validated model, not the other way around.
