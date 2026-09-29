# Interactive Scientific Visualization DSL
## Capability Taxonomy

> **Purpose:** Consolidate the recurring capabilities identified in the Test Case Catalogue and Capability / Case Matrix into a domain-independent taxonomy.
>
> This document describes **what the system must be capable of expressing**, not how those capabilities are exposed in the final DSL.

---

# 1. Design Principle

The system should not be organized primarily around scientific subjects.

It should not fundamentally have:

```text
physics/
chemistry/
mathematics/
biology/
computer_science/
```

Instead, domains should emerge from combinations of general capabilities.

For example:

```text
Projectile motion
=
entities
+ spatial state
+ vectors
+ equations
+ continuous dynamics
+ numerical integration
+ visualization
```

while:

```text
Radioactive decay
=
entities
+ discrete events
+ stochastic transitions
+ probability
+ time
+ population statistics
+ visualization
```

and:

```text
Chemical reaction
=
entities
+ state
+ quantities
+ reactions
+ rates
+ differential equations
+ numerical integration
+ visualization
```

The goal is therefore to identify the underlying common vocabulary.

---

# 2. Top-Level Capability Taxonomy

The emerging taxonomy is:

```text
SYSTEM
│
├── MODEL
│   ├── Entity
│   ├── Property
│   ├── Relation
│   ├── State
│   ├── Parameter
│   ├── Constraint
│   └── Composition
│
├── MATHEMATICS
│   ├── Quantity
│   ├── Expression
│   ├── Function
│   ├── Equation
│   ├── Vector
│   ├── Matrix / Tensor
│   ├── Set / Collection
│   ├── Probability
│   └── Statistics
│
├── DYNAMICS
│   ├── Rule
│   ├── Transition
│   ├── Process
│   ├── Event
│   ├── Continuous Evolution
│   ├── Discrete Evolution
│   ├── Feedback
│   └── Stochasticity
│
├── SPACE
│   ├── Coordinate System
│   ├── Position
│   ├── Geometry
│   ├── Transform
│   ├── Distance
│   ├── Direction
│   ├── Region
│   └── Spatial Relationship
│
├── TIME
│   ├── Clock
│   ├── Duration
│   ├── Instant
│   ├── Timestep
│   ├── Event Scheduling
│   ├── Playback
│   └── Time Scaling
│
├── COMPUTATION
│   ├── Evaluation
│   ├── Numerical Integration
│   ├── Numerical Solving
│   ├── Symbolic Transformation
│   ├── Iteration
│   ├── Sampling
│   ├── Aggregation
│   ├── Optimization
│   └── Constraint Solving
│
├── REPRESENTATION
│   ├── View
│   ├── Projection
│   ├── Geometry Renderer
│   ├── Graph Renderer
│   ├── Diagram Renderer
│   ├── Symbolic Renderer
│   ├── Field Renderer
│   ├── Statistical Renderer
│   ├── Timeline Renderer
│   └── Composite View
│
├── INTERACTION
│   ├── Input
│   ├── Control
│   ├── Manipulation
│   ├── Measurement
│   ├── Inspection
│   ├── Experiment
│   ├── Branching
│   └── State Management
│
├── OBSERVATION
│   ├── Measurement
│   ├── Probe
│   ├── Derived Quantity
│   ├── Sampling
│   ├── Recording
│   └── Analysis
│
├── ANIMATION
│   ├── Property Animation
│   ├── Transformation
│   ├── Appearance
│   ├── Temporal Sequencing
│   ├── Synchronization
│   └── Transition
│
├── EDUCATION
│   ├── Annotation
│   ├── Explanation
│   ├── Guided Activity
│   ├── Experiment
│   ├── Prediction
│   ├── Assessment
│   └── Narration
│
└── OUTPUT
    ├── Interactive Runtime
    ├── Image
    ├── Animation
    ├── Video
    ├── Data
    └── Presentation
```

---

# 3. MODEL

The model represents **what exists and what is true**.

## 3.1 Entity

An entity is an identifiable object or conceptual participant.

Examples:

```text
ball
electron
atom
molecule
planet
vehicle
node
particle
population
function
```

An entity does not necessarily have to be spatial.

---

## 3.2 Property

A property is a value associated with something.

Examples:

```text
mass
charge
velocity
temperature
position
name
population
energy
radius
color
```

Properties may be:

- constant
- variable
- derived
- computed
- user-controlled
- stochastic

---

## 3.3 Relation

A relation connects things.

Examples:

```text
particle → collides_with → particle

atom → bonded_to → atom

node → connected_to → node

planet → orbits → star

object → contained_by → region
```

Relations may themselves have properties.

---

## 3.4 State

State represents the condition of a model at a particular point.

Conceptually:

```text
State(t) = {
    entities,
    properties,
    relations,
    fields,
    variables
}
```

The simulation evolves state.

---

## 3.5 Parameter

A parameter is an externally controllable value that influences the model.

Examples:

```text
gravity = 9.81
mass = 2
initial_velocity = 20
temperature = 300
reaction_rate = ...
```

Parameters may be changed interactively.

---

## 3.6 Constraint

A constraint specifies something that must remain true.

Examples:

```text
distance(a, b) = L

x >= 0

mass_total = constant

charge_total = constant
```

Constraints may be:

- geometric
- physical
- mathematical
- logical
- user-defined

---

## 3.7 Composition

Models must be composable.

A model may contain:

```text
system
 ├── subsystem
 │    ├── entity
 │    └── process
 │
 └── subsystem
      ├── entity
      └── process
```

This enables complex simulations to be constructed from smaller models.

---

# 4. MATHEMATICS

Mathematics should be a first-class capability rather than something embedded only inside physics.

## 4.1 Quantity

A quantity has a value and potentially additional semantic information.

Examples:

```text
5
3.14
-2
5 m
20 m/s
300 K
```

The system may eventually need:

```text
value
unit
dimension
precision
domain
```

---

## 4.2 Expression

Expressions combine values and operations.

Examples:

```text
2 + 3

m * v

sin(x)

x² + 2x + 1

distance(a, b)
```

Expressions may depend on model state.

---

## 4.3 Function

A function maps inputs to outputs.

```text
f(x) = x²
```

or:

```text
velocity(time) → velocity
```

Functions may be:

- analytical
- numerical
- user-defined
- piecewise
- stochastic
- state-dependent

---

## 4.4 Equation

An equation expresses a relationship.

```text
F = ma

x(t) = x₀ + vt

PV = nRT
```

Equations may be:

- explanatory
- computational
- constraints
- governing laws
- displayed representations

These roles should remain distinguishable conceptually.

---

## 4.5 Vector

Vectors represent quantities with direction and magnitude.

Examples:

```text
position
velocity
acceleration
force
electric field
magnetic field
```

Operations include:

```text
addition
subtraction
scaling
dot product
cross product
magnitude
normalization
projection
```

---

## 4.6 Matrix / Tensor

Required for cases involving:

- linear algebra
- transformations
- systems of equations
- numerical simulation
- multidimensional data
- tensor fields

---

## 4.7 Probability

Probability represents uncertainty and stochastic behavior.

Capabilities include:

```text
random variable
distribution
sampling
probability
conditional probability
expectation
variance
```

---

## 4.8 Statistics

Statistics operates on observations or generated data.

Examples:

```text
mean
median
variance
histogram
distribution
correlation
regression
confidence interval
```

This becomes particularly important for simulations such as radioactive decay and particle systems.

---

# 5. DYNAMICS

Dynamics describes **how state changes**.

## 5.1 Rule

A rule maps conditions to behavior.

```text
IF condition
THEN action
```

Rules are useful for:

- agents
- reactions
- state machines
- discrete systems
- educational interactions

---

## 5.2 Transition

A transition changes an entity or system from one state to another.

```text
READY → RUNNING

UNDECAYED → DECAYED

LIQUID → GAS
```

---

## 5.3 Process

A process describes ongoing change.

Examples:

```text
diffusion
reaction
growth
decay
motion
heat transfer
```

---

## 5.4 Event

An event represents a discrete occurrence.

Examples:

```text
collision
radioactive decay
button press
particle creation
particle destruction
reaction
threshold crossing
```

Events may trigger other events.

---

## 5.5 Continuous Evolution

A system may evolve continuously:

```text
dx/dt = v
dv/dt = a
```

This implies numerical integration or another evolution mechanism.

---

## 5.6 Discrete Evolution

A system may instead evolve through steps:

```text
state₀
  ↓
state₁
  ↓
state₂
```

Examples include:

- cellular automata
- algorithms
- discrete simulations
- state machines

---

## 5.7 Feedback

The output of a process can affect its future input.

```text
state
 ↓
process
 ↓
output
 ↓
feeds back into state
```

This is fundamental to control systems, populations, ecosystems, and many physical systems.

---

## 5.8 Stochasticity

Randomness must be a first-class model capability.

Possible sources include:

```text
random initial state
random property
random event time
random transition
random measurement
random interaction
```

A stochastic model should support reproducibility through controlled randomness/seeding.

---

# 6. SPACE

Space is independent of rendering.

## 6.1 Coordinate System

A model may define:

```text
Cartesian
polar
relative
custom
```

A rendering system may map that coordinate system onto the 2D viewport.

---

## 6.2 Position

An entity may have a position:

```text
position(entity, t)
```

Position does not necessarily imply that every entity must be rendered as a shape.

---

## 6.3 Geometry

Geometry describes spatial form.

Capabilities include:

```text
point
line
segment
circle
ellipse
polygon
curve
region
custom shape
```

---

## 6.4 Transform

Transforms include:

```text
translation
rotation
scale
reflection
shear
affine transformation
```

---

## 6.5 Spatial Relationship

Examples:

```text
inside
outside
intersects
touches
near
far
above
below
left_of
connected_to
```

These relationships can drive rules and interactions.

---

# 7. TIME

Time should be modeled independently from animation.

This distinction is crucial.

```text
simulation time ≠ wall-clock time ≠ presentation time
```

A simulation might run:

```text
0 → 10 seconds
```

while being presented over:

```text
60 seconds of video
```

or interactively at:

```text
2× speed
```

---

## 7.1 Simulation Clock

The clock represents model time.

---

## 7.2 Timestep

Defines how continuous/discrete evolution advances.

Possible models:

```text
fixed timestep
variable timestep
adaptive timestep
```

---

## 7.3 Event Scheduling

Events may occur at specific times or in response to conditions.

---

## 7.4 Playback

Playback controls presentation:

```text
play
pause
seek
step
reverse
speed
```

Not every simulation needs all of these.

---

## 7.5 Time Scaling

The same model may be:

```text
slowed down
accelerated
paused
scrubbed
```

without changing its underlying semantics.

---

# 8. COMPUTATION

Computation determines how model relationships are evaluated.

## 8.1 Evaluation

Evaluate expressions against the current state.

```text
energy = 0.5 * mass * velocity²
```

---

## 8.2 Numerical Integration

Required for many continuous dynamical systems.

Potential methods eventually include:

```text
Euler
RK2
RK4
adaptive methods
symplectic methods
```

The taxonomy does not yet prescribe which methods must exist.

---

## 8.3 Numerical Solving

Examples:

```text
solve(f(x) = 0)

solve(system_of_equations)
```

---

## 8.4 Symbolic Transformation

Examples:

```text
differentiate(f)
integrate(f)
simplify(expression)
substitute(expression, value)
```

---

## 8.5 Iteration

Repeated computation:

```text
for each step
for each entity
until condition
```

---

## 8.6 Sampling

Generate observations from:

```text
probability distributions
simulations
fields
functions
measurements
```

---

## 8.7 Aggregation

Convert many observations into useful information:

```text
mean
sum
count
histogram
distribution
minimum
maximum
```

---

## 8.8 Optimization

Find parameters satisfying an objective.

Potential uses:

```text
fit a curve
minimize energy
maximize efficiency
find trajectory
```

---

## 8.9 Constraint Solving

Find states satisfying constraints.

---

# 9. REPRESENTATION

Representation answers:

> **How should model information be presented?**

It does not define the model.

---

## 9.1 View

A view observes some portion of model state.

Examples:

```text
world view
graph view
equation view
statistics view
timeline view
```

---

## 9.2 Projection

A projection maps model information into a representation.

Conceptually:

```text
Model
  ↓
Projection
  ↓
Representation
```

For example:

```text
velocity(t)
      ↓
plot projection
      ↓
velocity graph
```

---

## 9.3 Geometry Representation

Maps model entities to visual geometry.

---

## 9.4 Graph Representation

Maps quantities/relationships to:

```text
axes
lines
points
curves
nodes
edges
```

---

## 9.5 Symbolic Representation

Displays:

```text
equations
expressions
variables
values
units
```

---

## 9.6 Field Representation

Maps distributed values to:

```text
vectors
arrows
contours
density
color/intensity
streamlines
```

---

## 9.7 Statistical Representation

Maps data to:

```text
histograms
distributions
scatter plots
bars
summary values
```

---

## 9.8 Composite View

Multiple projections can coexist.

```text
┌───────────────────────┐
│     Spatial View      │
├───────────┬───────────┤
│ Equation  │   Graph   │
├───────────┴───────────┤
│      Measurements     │
└───────────────────────┘
```

All views may observe the same model state.

---

# 10. INTERACTION

Interaction modifies or observes the model.

## 10.1 Input

Examples:

```text
keyboard
mouse
touch
controller
UI controls
```

---

## 10.2 Control

Controls modify parameters or execution.

```text
slider
button
toggle
dropdown
playback control
```

---

## 10.3 Manipulation

Users may directly manipulate model entities.

```text
drag particle
move object
change vector
construct molecule
connect nodes
```

---

## 10.4 Measurement

Interaction can request a value from the model.

```text
measure(distance(a,b))
measure(velocity)
measure(temperature)
```

---

## 10.5 Inspection

Inspect an entity or state without necessarily modifying it.

---

## 10.6 Experiment

An experiment defines:

```text
initial conditions
parameters
procedure
observations
results
```

This provides a bridge between simulation and educational/scientific experimentation.

---

## 10.7 Branching

A user may explore:

```text
State A
 ├── experiment 1
 └── experiment 2
```

without destroying the original state.

---

## 10.8 State Management

Capabilities include:

```text
save
restore
reset
checkpoint
replay
branch
```

---

# 11. OBSERVATION

Observation deserves its own capability family because **the observer is not necessarily part of the model**.

A simulation may contain:

```text
MODEL
  ↓
OBSERVATION
  ↓
MEASUREMENT
  ↓
REPRESENTATION
```

This distinction is important for scientific simulations.

---

## 11.1 Measurement

Extract a quantity from the model.

---

## 11.2 Probe

Observe a particular:

```text
entity
location
region
time
property
```

---

## 11.3 Derived Quantity

Measurements may themselves be computed.

```text
speed = magnitude(velocity)

kinetic_energy = 0.5 * m * v²
```

---

## 11.4 Recording

Observations may be collected through time.

```text
t       velocity
0       0
1       4
2       8
...
```

---

## 11.5 Analysis

Recorded data can become the input to statistical or graphical representations.

---

# 12. ANIMATION

Animation is a presentation capability, not necessarily a simulation capability.

This distinction allows:

```text
simulation
```

and:

```text
pure explanatory animation
```

to coexist.

---

## 12.1 Property Animation

Animate a property:

```text
opacity
position
rotation
scale
```

---

## 12.2 Transformation

Animate transformations of representations.

---

## 12.3 Appearance

Animate:

```text
creation
destruction
highlighting
emphasis
visibility
```

---

## 12.4 Sequencing

Represent a sequence:

```text
A → B → C
```

---

## 12.5 Synchronization

Synchronize animation with:

```text
simulation time
events
equations
measurements
narration
```

---

# 13. EDUCATION

Education should be layered above the core model rather than contaminating the underlying simulation semantics.

## 13.1 Annotation

Add:

```text
labels
arrows
callouts
definitions
highlights
```

---

## 13.2 Explanation

Associate explanatory content with:

```text
entities
properties
events
equations
steps
observations
```

---

## 13.3 Guided Activity

A sequence of learner actions:

```text
observe
predict
change parameter
run
measure
compare
conclude
```

---

## 13.4 Prediction

Allow a learner to make a prediction before executing the model.

---

## 13.5 Assessment

Compare learner interaction/results against expected conditions.

---

## 13.6 Narration

Synchronize spoken content with:

```text
simulation time
animation
events
visual highlights
```

---

# 14. OUTPUT

The model should not be tied to one output target.

Potential outputs include:

```text
interactive application
web/WASM application
image
animation
video
GIF
frame sequence
data
graph
presentation
educational lesson
```

The same authored model may produce several outputs.

---

# 15. Capability Relationships

The capabilities are not independent.

A more useful conceptual dependency graph is:

```text
                    MODEL
                      │
          ┌───────────┼────────────┐
          ↓           ↓            ↓
     MATHEMATICS   SPACE       RELATIONS
          │           │            │
          └───────────┼────────────┘
                      ↓
                   DYNAMICS
                      │
               ┌──────┼──────┐
               ↓      ↓      ↓
             TIME   EVENTS  RULES
               │      │      │
               └──────┼──────┘
                      ↓
                 COMPUTATION
                      │
                      ↓
                   STATE
                      │
          ┌───────────┴───────────┐
          ↓                       ↓
     OBSERVATION             INTERACTION
          │                       │
          └───────────┬───────────┘
                      ↓
                REPRESENTATION
                      │
          ┌───────────┼────────────┐
          ↓           ↓            ↓
      ANIMATION    EDUCATION     OUTPUT
```

This is not yet the final architecture. It is a conceptual relationship map.

---

# 16. Primitive vs Derived Capabilities

An important distinction is beginning to emerge.

Some capabilities are likely **fundamental**:

```text
value
entity
property
relation
state
expression
time
event
rule
space
measurement
view
input
```

Others may be constructed from these:

```text
velocity
acceleration
kinetic energy
radioactive decay
chemical reaction
graph
histogram
pendulum
projectile
```

For example:

```text
velocity
=
change(position) / change(time)
```

Therefore velocity may not need to be a primitive of the DSL.

Likewise:

```text
half_life
```

could potentially be represented through a decay process and statistical observation rather than being a special primitive.

This distinction will be extremely important when designing the generalized model.

---

# 17. Semantic vs Computational vs Presentation Capabilities

Another major separation is:

```text
SEMANTIC
What exists / what is true?

COMPUTATIONAL
How is it calculated/evolved?

PRESENTATIONAL
How is it shown?
```

For example:

```text
Particle
    │
    ├── Semantic:
    │      mass
    │      position
    │      charge
    │
    ├── Computational:
    │      forces
    │      integration
    │      collision
    │
    └── Presentation:
           circle
           label
           velocity arrow
           trajectory graph
```

This separation should be preserved.

---

# 18. Model vs View

The most important architectural relationship emerging from the taxonomy is:

```text
             MODEL
               │
               │ state
               ↓
        ┌──────────────┐
        │  PROJECTION  │
        └──────────────┘
               │
       ┌───────┼────────┐
       ↓       ↓        ↓
    Spatial   Graph   Equation
       │       │        │
       └───────┼────────┘
               ↓
          Presentation
```

A change to the model should propagate to all relevant views.

Likewise, interaction with a view should be able to map back to model operations where appropriate.

---

# 19. Simulation vs Animation

The taxonomy also establishes two distinct paths.

## Simulation

```text
MODEL
 ↓
DYNAMICS
 ↓
TIME
 ↓
COMPUTATION
 ↓
STATE
 ↓
REPRESENTATION
```

## Explanatory Animation

```text
AUTHOR INTENT
 ↓
ANIMATION
 ↓
REPRESENTATION
```

They can also be combined:

```text
MODEL
 ↓
SIMULATION
 ↓
STATE
 ↓
ANIMATION / PRESENTATION
```

This allows the system to support Manim-like explanatory animation without requiring every scene to be a physical simulation.

---

# 20. Static Mathematical Visualization

The system must also support cases where there is no simulation.

For example:

```text
f(x) = x²
```

may simply produce:

```text
equation
   +
coordinate system
   +
graph
   +
annotations
```

No evolving physical state is required.

Therefore:

```text
simulation ≠ prerequisite for visualization
```

---

# 21. Hybrid Models

A generalized system must allow capabilities to coexist.

Example:

```text
Chemical reaction simulation

Entities
  ↓
Molecules

Relations
  ↓
Bonds

Properties
  ↓
Energy / concentration

Dynamics
  ↓
Reaction rules

Time
  ↓
Continuous evolution

Stochasticity
  ↓
Random reaction events

Computation
  ↓
Numerical integration + sampling

Observation
  ↓
Concentration measurements

Representation
  ├── molecules
  ├── concentration graph
  ├── equation
  └── statistics
```

No single modeling paradigm captures the entire case.

---

# 22. Domain Construction

Domains can therefore be viewed as **capability compositions**.

For example:

```text
PHYSICS
=
mathematics
+ space
+ dynamics
+ time
+ measurement
```

```text
CHEMISTRY
=
entities
+ relations
+ properties
+ reactions
+ dynamics
+ probability
+ statistics
```

```text
COMPUTER SCIENCE
=
entities
+ state
+ transitions
+ relations
+ discrete time
+ algorithms
+ graphs
```

These are conceptual compositions, not mandatory namespaces.

---

# 23. What Should NOT Become a Primitive Yet

At this stage, avoid prematurely creating primitives such as:

```text
particle
atom
electron
ball
projectile
molecule
radioactive_atom
pendulum
chemical_reaction
gravity
```

Some of these may eventually become convenient library abstractions, but they should not be assumed to be fundamental to the generalized system.

Instead:

```text
particle
```

may emerge from:

```text
entity
+ position
+ geometry
+ properties
+ dynamics
```

while:

```text
molecule
```

may emerge from:

```text
entities
+ relations
+ properties
+ geometry
```

This keeps the core general.

---

# 24. Emerging Core

The current investigation suggests a surprisingly small conceptual center:

```text
ENTITY
PROPERTY
RELATION
STATE
VALUE
EXPRESSION
FUNCTION
RULE
EVENT
TIME
SPACE
PROCESS
PARAMETER
CONSTRAINT
OBSERVATION
PROJECTION
INTERACTION
```

Everything else may potentially be constructed around these.

However, this is **not yet the final primitive set**.

The next phase must test whether these concepts can actually express the entire test-case catalogue.

---

# 25. Next Artifact: Generalized Model

The next question is therefore no longer:

> "What features should the DSL have?"

It is:

> **"What generalized computational model can represent all of these capabilities?"**

The next artifact should investigate:

```text
             GENERALIZED MODEL
                    │
        ┌───────────┼───────────┐
        ↓           ↓           ↓
      STATE       BEHAVIOR     SPACE
        │           │           │
        └───────────┼───────────┘
                    ↓
                  TIME
                    ↓
               EVOLUTION
                    ↓
                 OBSERVE
                    ↓
               PROJECT / VIEW
                    ↓
              INTERACT / EXPORT
```

That model should then be challenged against the original test cases.

Only after that validation should the project move toward actual DSL syntax.
