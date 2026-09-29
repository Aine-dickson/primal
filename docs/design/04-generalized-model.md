# Prismal
## Generalized Model

> **Purpose:** Define a domain-independent model capable of representing mathematical, physical, chemical, biological, computational, and abstract systems, together with their simulation, observation, interaction, visualization, and export.
>
> This document comes after the Test Case Catalogue, Capability / Case Matrix, and Capability Taxonomy.
>
> It intentionally does **not** define DSL syntax or implementation APIs.

---

# 1. The Central Idea

The system should model a **stateful system that can be observed, evolved, manipulated, and projected into representations**.

At the highest level:

```text
                    SYSTEM
                       │
              ┌────────┴────────┐
              │                  │
             STATE            BEHAVIOR
              │                  │
              └────────┬─────────┘
                       ↓
                    EVOLUTION
                       │
                       ↓
                  OBSERVATION
                       │
                       ↓
                  PROJECTION
                       │
                       ↓
                REPRESENTATION
                       │
                       ↓
                 INTERACTION
                       │
                       └──────────→ STATE
```

This creates a feedback loop:

```text
         ┌───────────────────────────────┐
         │                               │
         ↓                               │
       STATE → BEHAVIOR → NEW STATE      │
         │                               │
         ↓                               │
    OBSERVATION                          │
         ↓                               │
    REPRESENTATION                       │
         ↓                               │
     INTERACTION ────────────────────────┘
```

The model therefore isn't fundamentally a "scene."

It is a **system**.

A scene is one possible representation of that system.

---

# 2. System

The top-level abstraction is a `System`.

A system consists conceptually of:

```text
System
├── State
├── Entities
├── Relations
├── Variables
├── Parameters
├── Rules
├── Processes
├── Events
├── Constraints
├── Time
├── Space
├── Observations
└── Views
```

Not every system requires every component.

For example:

```text
f(x) = x²
```

may require:

```text
variables
expression
domain
view
```

but no dynamic process.

A projectile simulation may require nearly all of them.

---

# 3. State

State is the most fundamental runtime concept.

Let:

```text
S(t)
```

represent the state of a system at time `t`.

Conceptually:

```text
S(t) =
{
    entities,
    properties,
    relations,
    variables,
    fields,
    parameters,
    other state
}
```

The exact internal representation is an implementation question.

The important semantic property is:

> A system has a state that can be observed and can change.

---

# 4. Entity

An entity is an identifiable participant in the system.

Examples:

```text
ball
atom
electron
molecule
planet
particle
node
cell
population
machine
agent
```

An entity can have:

```text
identity
properties
relations
state
behavior
representation
```

An entity does not have to be physical.

For example:

```text
graph_node
```

is an entity.

So is:

```text
chemical_species
```

and potentially:

```text
algorithm_state
```

---

# 5. Properties

Entities and the system itself have properties.

Conceptually:

```text
entity.property = value
```

Examples:

```text
ball.mass
ball.position
ball.velocity

atom.charge
atom.energy

molecule.temperature

population.size
```

Properties may be:

### Constant

```text
mass = 2 kg
```

### Variable

```text
velocity(t)
```

### Derived

```text
speed = magnitude(velocity)
```

### Computed

```text
energy = ½mv²
```

### Stochastic

```text
decay_time ~ exponential(...)
```

### User-controlled

```text
gravity ← slider
```

This suggests that "property" should not necessarily mean merely "stored field."

It can represent a **named quantity associated with the model** whose value may have different origins.

---

# 6. Values

Values are the data carried by the model.

Potential value categories include:

```text
number
boolean
string
quantity
vector
matrix
tensor
point
geometry
set
sequence
distribution
function
expression
object/reference
```

Units and dimensions may be associated with quantities.

For example:

```text
5 m
20 m/s
300 K
```

should conceptually be different from dimensionless:

```text
5
20
300
```

even if the underlying numeric representation is related.

---

# 7. Expressions

An expression computes a value from other values.

For example:

```text
½ * mass * velocity²
```

or:

```text
distance(a.position, b.position)
```

or:

```text
sin(time)
```

An expression can depend on:

```text
constants
parameters
properties
state
time
randomness
other expressions
```

Therefore:

```text
expression(state, time) → value
```

is a useful conceptual abstraction.

---

# 8. Variables

Variables represent quantities whose values may change.

Examples:

```text
x
velocity
temperature
population
energy
concentration
```

A variable may belong to:

```text
system
entity
relation
field
process
```

This prevents the model from assuming that every variable belongs to an entity.

For example:

```text
temperature(x, y, t)
```

may be a field rather than a property of one entity.

---

# 9. Relations

Relations connect model elements.

Examples:

```text
atom A bonded_to atom B

planet orbits star

node A connected_to node B

particle A collides_with particle B
```

Relations can carry state:

```text
bond.order
edge.weight
connection.capacity
```

Relations may also change dynamically.

For example:

```text
A connected_to B
```

may become:

```text
A disconnected_from B
```

during a simulation.

---

# 10. Space

Space is a model capability rather than a rendering feature.

A system may define one or more spatial domains.

Conceptually:

```text
Space
├── coordinate system
├── dimensions
├── bounds
├── geometry
└── spatial relationships
```

A system may be:

```text
non-spatial
1D
2D
3D
higher-dimensional
abstract/topological
```

The runtime renderer may still be 2D.

Therefore:

```text
model space ≠ rendering space
```

---

# 11. Geometry

Geometry is the spatial structure of objects or regions.

Potential primitives include:

```text
point
line
ray
segment
circle
ellipse
polygon
curve
region
mesh
custom geometry
```

Geometry can be:

```text
static
dynamic
derived
procedural
user-defined
```

---

# 12. Fields

A field maps locations or other domains to values.

Conceptually:

```text
F(x, y, t) → value
```

Examples:

```text
temperature(x,y)
electric_field(x,y)
pressure(x,y,t)
probability_density(x,y)
```

A field does not require individual entities at every location.

This is important because a generalized model must support both:

```text
entity-centric systems
```

and:

```text
field-centric systems
```

---

# 13. Behavior

Behavior describes how a system can change.

A behavior can operate on:

```text
state
entity
relation
field
variable
```

Behavior can be represented through:

```text
rule
equation
function
process
event
constraint
algorithm
```

These are different mechanisms but belong to the same broader concept:

> **A mechanism that determines or constrains state evolution.**

---

# 14. Processes

A process describes ongoing evolution.

Conceptually:

```text
process(state, time) → state change
```

Examples:

```text
motion
diffusion
reaction
growth
heat transfer
decay
```

A process may be:

```text
continuous
discrete
deterministic
stochastic
```

or a combination.

---

# 15. Rules

Rules describe conditional behavior.

Conceptually:

```text
condition → action
```

Example:

```text
if velocity < 0
then ...
```

Rules are particularly useful for:

```text
agents
chemical reactions
state machines
games
algorithms
discrete simulations
```

---

# 16. Events

An event represents a discrete occurrence.

Examples:

```text
collision
decay
reaction
particle creation
particle destruction
button press
threshold crossing
state transition
```

Conceptually:

```text
event
├── condition
├── time
├── participants
└── effects
```

Events may be:

```text
scheduled
condition-triggered
random
externally triggered
```

---

# 17. State Transitions

A transition maps one state configuration into another.

```text
State A
   │
 transition
   ↓
State B
```

Transitions can be:

```text
deterministic
stochastic
instantaneous
time-dependent
event-triggered
user-triggered
```

This provides a common basis for:

```text
finite state machines
chemical reactions
radioactive decay
algorithm execution
agent behavior
```

---

# 18. Equations as Behavior

Equations should not merely be text displayed on screen.

An equation may serve as a governing relationship.

For example:

```text
F = ma
```

can constrain the evolution of:

```text
position
velocity
acceleration
```

Likewise:

```text
dN/dt = -λN
```

can govern radioactive decay.

Thus an equation may have two roles:

```text
equation
├── mathematical representation
└── computational/semantic relationship
```

These roles should remain distinguishable.

---

# 19. Constraints

A constraint restricts valid states.

Examples:

```text
distance(A,B) = L

x >= 0

energy_total = constant

charge_total = constant
```

A constraint can be:

```text
hard
soft
exact
approximate
geometric
mathematical
physical
logical
```

Constraints may either:

1. reject invalid states,
2. modify evolution to maintain validity, or
3. be reported as violations.

The precise semantics can be decided later.

---

# 20. Time

Time is part of system semantics.

The system has a conceptual simulation clock:

```text
t
```

State may therefore be represented as:

```text
S(t)
```

A model can use:

### Continuous time

```text
t ∈ ℝ
```

### Discrete time

```text
t₀, t₁, t₂...
```

### Event time

```text
t₁ < t₂ < t₃ ...
```

where events determine when state changes occur.

These mechanisms should be composable.

---

# 21. Evolution

The generalized system needs a concept of evolution:

```text
S₀ → S₁ → S₂ → ...
```

or:

```text
S(t)
```

Evolution can be generated through different mechanisms.

```text
Evolution
├── continuous equation
├── discrete rule
├── event
├── stochastic process
├── algorithm
└── hybrid combination
```

This is a major abstraction.

The runtime should not fundamentally care whether a state change came from Newton's laws or a cellular automaton.

---

# 22. Determinism and Randomness

A system may contain deterministic and stochastic components simultaneously.

For example:

```text
trajectory
    ↓
deterministic

radioactive decay
    ↓
stochastic
```

A generalized system therefore needs a source of controlled randomness:

```text
RandomSource
```

Conceptually:

```text
random_source(seed)
        ↓
random values
        ↓
stochastic behavior
```

This allows:

```text
random
```

and:

```text
reproducibly random
```

to coexist.

---

# 23. Experiment

An experiment is not necessarily another kind of model.

It is a controlled procedure over a model.

Conceptually:

```text
Experiment
├── initial state
├── parameters
├── intervention
├── execution
├── observations
└── results
```

For example:

```text
Experiment:
    gravity = 9.81
    velocity = 20
    angle = 45°
    run
    measure(range)
```

The same model can therefore support many experiments.

---

# 24. Observation

Observation extracts information from the system.

Conceptually:

```text
Observation(model_state) → data
```

Examples:

```text
measure velocity
measure temperature
count particles
sample concentration
record position
```

An observation can itself be computed.

For example:

```text
velocity
    ↓
magnitude
    ↓
speed
```

---

# 25. Measurement

A measurement is a specific observation with semantic meaning.

It may include:

```text
value
unit
location
time
uncertainty
source
```

This becomes important for educational and scientific applications.

---

# 26. Data

Observations can be accumulated into datasets.

```text
DataSet
├── observations
├── dimensions
├── units
├── timestamps
└── metadata
```

Datasets can then feed:

```text
graphs
statistics
tables
analysis
exports
```

This creates a clean path:

```text
MODEL
 ↓
OBSERVE
 ↓
DATA
 ↓
ANALYZE
 ↓
REPRESENT
```

---

# 27. Projection

A projection maps model information into a representation.

Conceptually:

```text
Projection(model_state) → representation_state
```

Examples:

```text
particle model
      ↓
spatial projection
      ↓
circles + labels

function
      ↓
graph projection
      ↓
curve + axes

field
      ↓
vector projection
      ↓
arrows

data
      ↓
statistical projection
      ↓
histogram
```

The projection is therefore the bridge between semantics and visualization.

---

# 28. Representation

A representation is the visual or conceptual manifestation of a projection.

Examples:

```text
shape
text
equation
graph
axis
particle
arrow
field visualization
timeline
table
histogram
diagram
```

Representations can themselves have properties:

```text
position
scale
opacity
style
visibility
label
```

This creates a distinction:

```text
model property
        vs
representation property
```

For example:

```text
particle.mass = 5 kg

particle_visual.radius = 20 px
```

These should not accidentally become the same thing.

---

# 29. View

A view observes one or more projections.

A system may have:

```text
View A → spatial
View B → graph
View C → equations
View D → statistics
```

All can observe:

```text
the same underlying model
```

This gives:

```text
                    MODEL
                      │
          ┌───────────┼────────────┐
          ↓           ↓            ↓
      Projection   Projection   Projection
          ↓           ↓            ↓
       Spatial       Graph      Equation
          │           │            │
          └───────────┼────────────┘
                      ↓
                   UI/View
```

---

# 30. Interaction

Interaction is an operation initiated externally.

Conceptually:

```text
Interaction
    ↓
operation
    ↓
model / simulation / view
```

Examples:

```text
change parameter
drag entity
press play
measure quantity
add particle
remove node
change viewpoint
```

Interaction may modify:

```text
model state
parameters
simulation execution
view state
```

These should remain conceptually distinct.

---

# 31. Animation

Animation changes representation over presentation time.

This is different from simulation.

For example:

```text
simulation time:
0 → 10 seconds
```

could be rendered as:

```text
presentation time:
0 → 60 seconds
```

Animation can therefore operate on:

```text
representation properties
transforms
appearance
visibility
sequence
timing
```

It can also respond to model events.

---

# 32. Presentation Time vs Simulation Time

The system should conceptually contain at least two clocks:

```text
simulation time
presentation time
```

Potentially also:

```text
wall-clock time
```

For example:

```text
Radioactive decay
simulation:     0 → 1000 years
presentation:   0 → 30 seconds
```

while interactive execution could run at:

```text
10 years / second
```

This separation enables both simulation and Manim-like rendering.

---

# 33. Interaction Feedback Loop

The full interactive runtime becomes:

```text
                 ┌───────────────┐
                 │     MODEL     │
                 └───────┬───────┘
                         │
                       STATE
                         │
                         ↓
                    EVOLUTION
                         │
                         ↓
                      STATE'
                         │
             ┌───────────┴───────────┐
             ↓                       ↓
        OBSERVATION              PROJECTION
             ↓                       ↓
           DATA                REPRESENTATION
             │                       │
             └───────────┬───────────┘
                         ↓
                       VIEW
                         │
                         ↓
                    USER INPUT
                         │
                         ↓
                    INTERACTION
                         │
                         └────────────→ MODEL
```

This is the core runtime loop.

---

# 34. The Model Is Not the Renderer

This distinction should be treated as fundamental.

A model could be rendered as:

```text
spatial scene
```

or:

```text
graph
```

or:

```text
equation
```

or:

```text
table
```

or:

```text
particle system
```

or all simultaneously.

Therefore:

```text
Model
≠
Scene
≠
View
≠
Renderer
```

---

# 35. Example: Projectile Motion

The generalized model can describe projectile motion without a `Projectile` primitive.

```text
Entities:
    ball

Properties:
    mass
    position
    velocity

Parameters:
    gravity
    initial_velocity
    launch_angle

Space:
    2D Cartesian

Behavior:
    dx/dt = velocity
    dv/dt = gravity

Time:
    continuous

Evolution:
    numerical integration

Observations:
    position(t)
    velocity(t)
    range
    maximum_height
```

Possible projections:

```text
spatial trajectory
velocity graph
position graph
equation display
measurement panel
```

One model, multiple representations.

---

# 36. Example: Radioactive Decay

The same generalized model can represent microscopic decay.

```text
Entities:
    atoms

Properties:
    isotope
    state

Behavior:
    stochastic transition

Event:
    decay

Transition:
    undecayed → decayed

Time:
    event-driven / discrete events

Randomness:
    seeded random source

Observation:
    number_remaining(t)

Data:
    decay observations

Projection:
    atom visualization
    population graph
    histogram
    half-life measurement
```

The macroscopic model can instead use:

```text
dN/dt = -λN
```

The generalized model does not need to decide that one is the "real" representation.

---

# 37. Example: Chemical Reaction

Consider:

```text
A + B → C
```

The model could contain:

```text
Entities:
    chemical species

Properties:
    concentration
    amount

Relations:
    reaction participation

Behavior:
    reaction rule

Parameters:
    rate constants

Dynamics:
    concentration evolution

Computation:
    numerical integration

Observation:
    concentration(t)

Projection:
    molecular diagram
    concentration graph
    equation
    statistics
```

A more microscopic version could instead represent individual molecules and stochastic reaction events.

The same generalized framework accommodates both.

---

# 38. Example: Electric Field

A field model might be:

```text
Charges
    ↓
electric field
    ↓
E(x,y)
```

The model contains:

```text
entities → charges
properties → charge values/positions
field → E(x,y)
equations → governing relationship
```

Possible projections:

```text
field arrows
field magnitude
equipotential contours
charge particles
equations
measurements
```

Again, no special "electric field scene" is required.

---

# 39. Example: Graph Algorithm

A graph algorithm may contain:

```text
Entities:
    nodes

Relations:
    edges

Properties:
    edge weight
    node state

Behavior:
    algorithm rules

State:
    visited/unvisited
    current node
    queue

Time:
    discrete steps

Observation:
    operation count
    path
    state transitions

Projection:
    graph
    algorithm state
    pseudocode
    statistics
```

This demonstrates that the generalized model is not inherently a physics engine.

---

# 40. Example: Pure Mathematics

Consider:

```text
f(x) = sin(x)
```

No simulation is necessary.

The system can contain:

```text
Function:
    f

Domain:
    x

Expression:
    sin(x)

Projection:
    graph

Representation:
    axes + curve + equation
```

Interaction could change:

```text
frequency
amplitude
phase
domain
```

This demonstrates:

```text
visualization without simulation
```

---

# 41. Example: Hybrid System

A sophisticated model could contain:

```text
particles
+
continuous field
+
discrete events
+
stochastic behavior
+
graphs
+
equations
+
user interaction
```

For example:

```text
particles
      ↓
produce field
      ↓
field affects particles
      ↓
particles collide
      ↓
collision generates event
      ↓
event changes state
      ↓
statistics record result
```

The generalized model must allow these components to interact.

---

# 42. Composition

Complex systems should be built from smaller systems.

Conceptually:

```text
System A
+
System B
+
coupling
=
System C
```

For example:

```text
Particle System
      +
Electric Field
      +
Measurement System
      =
Charged Particle Simulation
```

Composition should allow:

```text
shared state
shared time
coupled variables
events
data flow
constraints
```

---

# 43. Model Boundaries

Not everything should necessarily be global.

A system may contain subsystems:

```text
System
├── Physics
│   ├── particles
│   └── fields
│
├── Measurement
│   └── probes
│
└── Presentation
    ├── graph
    └── spatial view
```

This suggests that models should have explicit boundaries and interfaces.

---

# 44. Data Flow

Some systems are naturally described through transformations:

```text
input
  ↓
process
  ↓
output
```

Others are stateful:

```text
state
  ↓
evolution
  ↓
new state
```

The generalized model should support both.

Therefore a useful conceptual distinction is:

```text
FUNCTION
input → output

PROCESS
state → state'

EVENT
condition → discrete change

OBSERVATION
state → data
```

These concepts can interact without being collapsed into one abstraction.

---

# 45. Three Fundamental Directions

The model can now be viewed as three major directions.

## Forward

How the system evolves:

```text
STATE
 ↓
BEHAVIOR
 ↓
EVOLUTION
 ↓
STATE'
```

## Outward

How the system is observed:

```text
STATE
 ↓
OBSERVATION
 ↓
DATA
```

## Presentational

How information becomes visible:

```text
STATE / DATA
 ↓
PROJECTION
 ↓
REPRESENTATION
 ↓
VIEW
```

And interaction closes the loop:

```text
VIEW
 ↓
USER ACTION
 ↓
INTERACTION
 ↓
STATE / PARAMETERS
```

---

# 46. Generalized System Model

Putting everything together:

```text
                         SYSTEM
                            │
          ┌─────────────────┼──────────────────┐
          │                 │                  │
          ↓                 ↓                  ↓
        STATE            BEHAVIOR            CONTEXT
          │                 │                  │
    ┌─────┼─────┐     ┌─────┼──────┐      ┌───┴────┐
    ↓     ↓     ↓     ↓     ↓      ↓      ↓        ↓
 Entities Variables Relations Rules Equations Events Space Time
    │       │      │     │      │      │      │       │
    └───────┴──────┴─────┴──────┴──────┴──────┴───────┘
                            │
                            ↓
                         EVOLUTION
                            │
                            ↓
                      NEW STATE
                            │
                 ┌──────────┴──────────┐
                 ↓                     ↓
            OBSERVATION           PROJECTION
                 ↓                     ↓
               DATA              REPRESENTATION
                 │                     │
                 └──────────┬──────────┘
                            ↓
                           VIEW
                            │
                            ↓
                       INTERACTION
                            │
                            └──────────→ STATE
```

---

# 47. The Candidate Core Model

At this point, the generalized model can tentatively be reduced to:

```text
SYSTEM
│
├── STATE
│   ├── entities
│   ├── values
│   ├── variables
│   ├── relations
│   └── fields
│
├── BEHAVIOR
│   ├── functions
│   ├── rules
│   ├── equations
│   ├── processes
│   ├── events
│   └── constraints
│
├── CONTEXT
│   ├── space
│   └── time
│
├── COMPUTATION
│   └── evolution mechanisms
│
├── OBSERVATION
│   ├── measurements
│   └── datasets
│
├── PRESENTATION
│   ├── projections
│   ├── views
│   └── representations
│
└── INTERACTION
    ├── inputs
    └── operations
```

This is the current candidate, not the final architecture.

---

# 48. Fundamental Relationships

The generalized model can be expressed through a small number of relationships:

```text
Entity HAS Property

Entity RELATES_TO Entity

System HAS State

State CONTAINS Value

Expression DEPENDS_ON Value

Function MAPS Value → Value

Rule RESPONDS_TO Condition

Event CHANGES State

Process EVOLVES State

Equation CONSTRAINS/DEFINES Relationship

Constraint RESTRICTS State

Observation READS State

Projection MAPS State/Data → Representation

Interaction MODIFIES State/Parameters/View

View DISPLAYS Representation
```

This is more important than any eventual keyword names.

---

# 49. What Has Emerged

The investigation suggests that the system is not fundamentally:

```text
Manim + physics engine
```

It is closer to:

```text
General state/model system
+
mathematical computation
+
dynamic evolution
+
observation
+
projection system
+
interactive runtime
+
animation/presentation system
```

Manim-like animation becomes one capability built on top of this foundation.

Likewise, a physics engine becomes one possible evolution mechanism.

A graphing engine becomes one possible projection.

A statistics engine becomes one possible observation/analysis mechanism.

---

# 50. What We Still Must Validate

Before designing the DSL, this generalized model needs to survive adversarial testing.

The next validation should attempt to model at least:

```text
1.  f(x) = x²

2.  projectile motion

3.  pendulum

4.  collision simulation

5.  electric field

6.  wave propagation

7.  heat diffusion

8.  ideal gas

9.  microscopic radioactive decay

10. macroscopic radioactive decay

11. decay chain

12. chemical reaction

13. chemical equilibrium

14. molecular structure

15. population dynamics

16. predator-prey system

17. cellular automaton

18. graph traversal

19. sorting algorithm

20. stochastic particle simulation

21. hybrid particle + field system

22. multi-view educational experiment

23. pure explanatory animation

24. interactive parameter exploration
```

For each one we should ask:

> **Can the generalized model express this without introducing a domain-specific fundamental concept?**

If not, determine whether:

1. the generalized model is missing a genuinely fundamental capability,
2. the concept can be composed from existing capabilities, or
3. the concept belongs in a higher-level library rather than the core.

---

# 51. The Critical Test

The most important test is not:

> "Can we build a projectile simulation?"

It is:

> **"Can the same underlying model represent a projectile, a chemical reaction, a graph algorithm, a mathematical function, and radioactive decay without the model becoming a collection of unrelated special cases?"**

If yes, the abstraction is probably moving in the right direction.

If no, we return to the taxonomy and determine what capability is missing.

---

# 52. Next Step

Only after this generalized model is validated should we design the DSL.

The sequence becomes:

```text
01  TEST CASE CATALOGUE
        ↓
02  CAPABILITY / CASE MATRIX
        ↓
03  CAPABILITY TAXONOMY
        ↓
04  GENERALIZED MODEL              ← THIS DOCUMENT
        ↓
05  MODEL VALIDATION
        ↓
06  RUNTIME SEMANTICS
        ↓
07  DSL DESIGN
        ↓
08  RENDERING / PROJECTION API
        ↓
09  INTERACTION API
        ↓
10  IMPLEMENTATION
```

The **next artifact should therefore be Model Validation**, not DSL syntax.

That validation should deliberately try to break this model using the test cases and expose missing abstractions before we commit to the language design.
