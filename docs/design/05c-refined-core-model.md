# Interactive Scientific Visualization DSL
## 05c Refined Core Model

### Purpose

This document consolidates the results of:

- `01-test-case-catalogue.md`
- `02-capability-case-matrix.md`
- `03-capability-taxonomy.md`
- `04-generalized-model.md`
- `05-model-validation.md`
- `05b-model-gap-analysis.md`

The purpose is to establish a **refined semantic core** before defining runtime semantics or DSL syntax.

The central question is:

> What is the smallest sufficiently expressive set of concepts from which the systems, simulations, experiments, visualizations, and educational experiences in the case catalogue can be constructed?

The goal is not minimality at all costs.

The goal is:

> **A small, coherent core that is expressive enough to avoid domain-specific assumptions while providing strong composition and extension mechanisms.**

---

# 1. Refined Foundational Principle

The system models:

> **Stateful structures, relationships, quantities, behaviors, and observations that may exist in domains, evolve through time, and be projected into representations.**

The fundamental architecture is:

```text
                         MODEL
                           │
                 ┌─────────┴─────────┐
                 ↓                   ↓
              CONTEXT             STATE
                 │                   │
            space / time       values / entities
                 │              relations / fields
                 │              collections
                 │                   │
                 └─────────┬─────────┘
                           ↓
                        BEHAVIOR
                           │
             ┌─────────────┼─────────────┐
             ↓             ↓             ↓
          functions     equations      events
             │             │             │
             └─────────────┼─────────────┘
                           ↓
                       EVOLUTION
                           │
                           ↓
                       OBSERVATION
                           │
                           ↓
                          DATA
                           │
                           ↓
                       PROJECTION
                           │
                           ↓
                    REPRESENTATION
                           │
                           ↓
                          VIEW
                           │
                           ↓
                     INTERACTION
                           │
                           └────────────→ MODEL
```

This architecture deliberately separates:

```text
meaning
execution
observation
presentation
interaction
```

---

# 2. The Refined Core

The core is organized into ten semantic areas:

```text
CORE
├── VALUES
├── DOMAINS
├── OBJECTS
├── STATE
├── RELATIONS
├── BEHAVIOR
├── CONTEXT
├── OBSERVATION
├── PROJECTION
└── COMPOSITION
```

Some capabilities such as time, randomness, constraints, and data cut across several of these areas.

---

# 3. VALUES

A **Value** is anything that can participate in the semantic model.

Candidate value categories:

```text
VALUE
├── Boolean
├── Integer
├── Real
├── Complex
├── Quantity
├── Vector
├── Matrix
├── Tensor
├── Point
├── Geometry
├── Set
├── Sequence
├── Distribution
├── Function
├── Expression
├── Equation
├── Reference
└── Data
```

This is a semantic classification rather than necessarily a literal implementation inheritance hierarchy.

---

# 4. Primitive Values

The mathematical foundation begins with primitive values.

```text
Boolean
Integer
Real
Complex
```

These provide the foundation for:

- arithmetic
- logic
- comparisons
- indexing
- symbolic expressions
- simulation parameters

---

# 5. Quantities

A scientific quantity combines a magnitude with dimensional meaning.

Conceptually:

```text
Quantity
├── magnitude
├── unit
└── dimension
```

Example:

```text
distance = 5 m
```

contains:

```text
magnitude = 5
unit = metre
dimension = Length
```

While:

```text
time = 5 s
```

has:

```text
magnitude = 5
unit = second
dimension = Time
```

---

# 6. Units and Dimensions

Units and dimensions are distinct.

```text
metre
centimetre
kilometre
```

are units.

```text
Length
Time
Mass
Temperature
```

are dimensions.

Thus:

```text
1 m
100 cm
```

have different units but the same dimension.

Dimensional arithmetic should be supported.

For example:

```text
Length / Time
    → Velocity
```

and:

```text
Mass × Length / Time²
    → Force
```

This provides an important scientific correctness mechanism.

---

# 7. Mathematical Structures

The core must support mathematical structures beyond scalar values.

## Vectors

```text
Vector2
Vector3
VectorN
```

or, more generally:

```text
Vector<Domain>
```

## Matrices

```text
Matrix(rows, columns)
```

## Tensors

Generalized multi-dimensional arrays with mathematical semantics.

These support:

- linear algebra
- mechanics
- fields
- transformations
- numerical methods
- machine-learning-like models
- scientific computation

---

# 8. Sets and Sequences

The core must distinguish between:

```text
Set
```

and:

```text
Sequence
```

A set is unordered.

A sequence has ordering.

Examples:

```text
graph nodes → set
particle population → collection
time samples → sequence
array values → sequence
```

This distinction becomes important for algorithms and data.

---

# 9. Functions

A function maps values from one domain to another.

Conceptually:

```text
Function
├── input domain
├── output domain
└── mapping
```

Examples:

```text
f : Real → Real
```

```text
velocity : Time → Vector
```

```text
field : Position → Vector
```

```text
algorithm_step : State → State
```

The core must not restrict functions to scalar mathematics.

---

# 10. Expressions

An expression represents a value-producing computation.

Example:

```text
x² + 2x + 1
```

An expression may depend on:

```text
constants
parameters
variables
properties
time
other expressions
functions
random variables
```

Expressions may be evaluated numerically or transformed symbolically.

---

# 11. Equations

An equation represents a mathematical relationship.

Examples:

```text
F = ma
```

```text
PV = nRT
```

```text
dN/dt = -λN
```

An equation must not automatically imply a particular solver.

It may serve as:

```text
definition
constraint
relationship
evolution law
displayed mathematical statement
```

Therefore equation semantics require an explicit role/context.

---

# 12. Predicates

A predicate is an expression whose result is Boolean.

Examples:

```text
velocity > 10 m/s
```

```text
distance(A,B) < r
```

```text
node.visited == false
```

Predicates are important because they drive:

- rules
- events
- filtering
- constraints
- queries
- conditional behavior

---

# 13. Distributions

Probability distributions are mathematical objects.

Examples:

```text
Normal(μ, σ)
Poisson(λ)
Binomial(n, p)
Uniform(a,b)
```

A distribution may be:

```text
sampled
observed
plotted
combined
transformed
analysed
```

This distinguishes a probability model from the random-number generator used to sample it.

---

# 14. Randomness

Randomness is a separate computational concept.

Conceptually:

```text
Random Source
├── algorithm
├── seed
└── state
```

A stochastic model can then use:

```text
distribution
+
random source
```

to produce samples or events.

The seed must be controllable for reproducibility.

---

# 15. DOMAINS

A **Domain** describes the space over which something is defined.

Examples:

```text
x ∈ [-1,1]
```

```text
i ∈ {0...,99}
```

```text
(x,y) ∈ ℝ²
```

```text
node ∈ Graph
```

```text
t ∈ [0,10]
```

Domains may be:

```text
continuous
discrete
finite
infinite
spatial
temporal
categorical
topological
abstract
```

Domain is distinct from value.

---

# 16. Space as a Domain

Space is a specialized domain describing spatial structure.

Possible spaces include:

```text
1D
2D
3D
N-dimensional
polar
cylindrical
spherical
abstract metric space
topological space
```

The core should not assume Euclidean 2D space.

A 2D renderer is merely one possible projection target.

---

# 17. Time as a Domain/Context

Time is similarly distinct from presentation frames.

The semantic model may define:

```text
t ∈ [0,10 s]
```

while presentation may use:

```text
30 seconds of video
```

These are different quantities.

Therefore:

```text
simulation time
```

and:

```text
presentation time
```

must remain separate.

---

# 18. OBJECTS

The generalized model previously used `Entity`.

The refined model broadens this slightly.

An **Object** is an identifiable semantic element that can participate in the model.

Examples:

```text
particle
atom
node
cell
equation
function
region
subsystem
```

Not every object is physical.

---

# 19. Identity

Every persistent object that can be referenced needs identity.

Conceptually:

```text
Object
├── identity
└── content
```

Identity allows:

```text
probe → particle A
relation → node B
view → entity C
event → object D
```

to remain meaningful even if collections reorder.

Identity is semantic, not visual.

---

# 20. Entities

An Entity is an object representing an identifiable participant in a model.

Examples:

```text
ball
atom
electron
node
cell
agent
vehicle
```

An entity can have:

```text
properties
state
relations
behavior
geometry
identity
```

---

# 21. Properties

A property associates a value with an object.

Examples:

```text
particle.mass
particle.position
atom.charge
node.visited
cell.temperature
```

A property may be:

```text
constant
variable
derived
computed
stochastic
user-controlled
```

---

# 22. Variables

A variable is a value whose state may change.

Variables can belong to:

```text
system
object
relation
field
process
experiment
```

Examples:

```text
position(t)
temperature(t)
population(t)
energy(t)
```

---

# 23. Derived Values

A derived value is computed from other values.

Example:

```text
velocity = derivative(position)
```

```text
kinetic_energy = 1/2 m v²
```

Derived values need not be stored independently.

This creates a distinction between:

```text
stored state
```

and:

```text
derived state
```

---

# 24. Collections

Collections represent groups of objects or values.

Conceptually:

```text
Collection<T>
```

A collection may support:

```text
create
destroy
filter
map
reduce
count
group
sort
sample
aggregate
```

Collections may be:

```text
static
dynamic
ordered
unordered
finite
population-like
```

The exact implementation remains a runtime concern.

---

# 25. Populations

A population is a higher-level interpretation of a collection.

Examples:

```text
radioactive nuclei
bacteria
predators
particles
students
```

Population-level quantities can include:

```text
count
mean
variance
distribution
density
```

`Population` is likely a library-level abstraction over collections.

---

# 26. RELATIONS

A Relation connects semantic objects.

Examples:

```text
bond(A,B)
edge(A,B)
connected(A,B)
parent(A,B)
interacts(A,B)
```

A relation may contain properties:

```text
edge.weight
bond.order
connection.capacity
```

---

# 27. Relation Lifecycle

Relations may be:

```text
static
dynamic
derived
conditional
```

A reaction can create or destroy relations.

A graph can add or remove edges.

A molecule can break a bond.

Therefore relation topology can evolve independently of ordinary property values.

---

# 28. Fields

A Field associates values with positions or elements of a domain.

General form:

```text
F : Domain → Value
```

Examples:

```text
temperature(x,y)
electric_field(x,y)
pressure(x,y)
velocity(x,y)
probability_density(x,y)
```

Fields are fundamentally different from ordinary collections of entities and remain first-class.

---

# 29. Stored vs Derived Fields

A field may be:

```text
stored
computed
sampled
interpolated
cached
```

For example:

```text
E(x,y)
```

may be computed on demand.

The semantic meaning should remain independent of its computational storage.

---

# 30. STATE

State describes the current configuration of a system.

Conceptually:

```text
State
├── objects
├── values
├── variables
├── relations
├── fields
├── collections
└── contextual state
```

State may be:

```text
initial
current
snapshot
derived
invalid
partial
```

---

# 31. Validity

The model must distinguish:

```text
valid
invalid
unknown
unavailable
pending
```

For example:

```text
division by zero
negative concentration
invalid geometry
constraint violation
missing external data
```

must not all be represented as ordinary values.

---

# 32. State Transition

A state transition maps one state into another.

Conceptually:

```text
State
  ↓
transition
  ↓
State'
```

A transition may be:

```text
deterministic
stochastic
continuous approximation
discrete
event-triggered
user-triggered
```

---

# 33. BEHAVIOR

Behavior describes how values or state are determined or changed.

Core behavioral concepts:

```text
Behavior
├── Function
├── Expression
├── Equation
├── Rule
├── Process
├── Event
└── Constraint
```

These should remain distinct because they have different semantic roles.

---

# 34. Rules

A rule expresses conditional behavior.

Conceptually:

```text
condition
    ↓
action/state change
```

Example:

```text
if distance(A,B) < r
    → collision
```

Rules depend on predicates and state transitions.

---

# 35. Processes

A process represents ongoing evolution.

Examples:

```text
motion
diffusion
reaction
growth
decay
heat transfer
```

A process may be:

```text
continuous
discrete
deterministic
stochastic
```

---

# 36. Events

An event represents a discrete occurrence.

Examples:

```text
collision
decay
reaction
threshold crossing
node creation
node deletion
button press
```

Events may be triggered by:

```text
time
predicate
random process
external input
user interaction
```

---

# 37. Constraints

A constraint restricts valid states or relationships.

Examples:

```text
x ≥ 0
```

```text
distance(A,B) = r
```

```text
mass_total = constant
```

Constraints may be:

```text
hard
soft
equality
inequality
geometric
physical
logical
boundary
```

Their enforcement strategy belongs to computation/runtime semantics.

---

# 38. Behavior Does Not Necessarily Mean Evolution

A function may simply calculate:

```text
f(x)
```

without changing state.

An equation may define a relationship without directly executing a transition.

A rule may describe a possible transition.

Therefore:

```text
Behavior
```

and:

```text
Evolution
```

must remain distinct.

---

# 39. CONTEXT

Context describes the environment in which a model exists.

Core contextual dimensions:

```text
Context
├── Domain
├── Space
├── Time
├── Parameters
└── External Inputs
```

Not every model requires every context.

---

# 40. Parameters

Parameters are values intentionally exposed as model configuration.

Examples:

```text
gravity = 9.81 m/s²
mass = 2 kg
decay_constant = λ
temperature = 300 K
```

Parameters may be changed between or during experiments.

---

# 41. Initial Conditions

Initial conditions specify the starting state.

They do not need to be a completely separate primitive.

Conceptually:

```text
Model
+
Initial State
```

defines the starting configuration.

---

# 42. Boundary Conditions

Boundary conditions restrict a field or process at a domain boundary.

They can be represented through:

```text
domain
+
constraint
+
boundary relation
```

A specialized API may make them convenient, but the semantic foundation remains generic.

---

# 43. External Inputs

A model may receive information from outside itself.

Examples:

```text
sensor
user
network
file
camera
microcontroller
live dataset
```

External input should remain distinguishable from ordinary internal state.

---

# 44. COMPOSITION

Composition allows systems to be constructed from systems.

A subsystem can contain:

```text
state
behavior
context
observations
interfaces
```

Example:

```text
System
├── Mechanical subsystem
├── Electrical subsystem
└── Thermal subsystem
```

---

# 45. Interfaces

A subsystem may expose:

```text
inputs
outputs
parameters
observations
```

while keeping internal state private.

This allows domain systems to be composed without requiring one giant global model.

---

# 46. Coupling

Subsystems may be coupled through:

```text
shared values
dependencies
relations
events
functions
equations
data flow
```

Example:

```text
Electrical subsystem
        ↓
       power
        ↓
Thermal subsystem
        ↓
   temperature
        ↓
Mechanical subsystem
```

---

# 47. Dependencies

A dependency means one semantic value or behavior depends on another.

Example:

```text
position
   ↓
velocity
   ↓
kinetic energy
```

Dependencies support:

- evaluation
- invalidation
- scheduling
- synchronization
- incremental recomputation

Dependencies may be:

```text
acyclic
cyclic
temporal
event-driven
```

---

# 48. Cyclic Dependencies

A cycle is not automatically an error.

For example:

```text
particle
   ↓
field
   ↓
force
   ↓
particle
```

may represent a coupled physical system.

The runtime must distinguish:

```text
simple dependency evaluation
```

from:

```text
coupled numerical evolution
```

---

# 49. OBSERVATION

Observation extracts information from model state.

General form:

```text
State
  ↓
Observation
  ↓
Data
```

Examples:

```text
measure position
measure temperature
count particles
sample field
observe energy
```

---

# 50. Measurement

A measurement may include:

```text
value
unit
timestamp
uncertainty
resolution
sampling information
instrument model
```

This allows realistic experiments without changing the underlying model.

---

# 51. Observation vs Intervention

These are deliberately different.

### Observation

```text
State → Information
```

### Intervention

```text
Action → State'
```

An interaction may perform either.

This distinction is important for experimental semantics.

---

# 52. Data

Data represents observations and results.

Conceptually:

```text
Dataset
├── values
├── dimensions
├── axes
├── units
├── timestamps
├── metadata
├── uncertainty
└── provenance
```

Examples:

```text
x(t)
T(x,y,t)
N(t)
velocity distribution
```

---

# 53. Analysis

Data can be transformed into derived data.

```text
Data
 ↓
Analysis
 ↓
Derived Data
```

Examples:

```text
mean
variance
derivative
integral
FFT
regression
histogram
correlation
```

Basic analysis belongs to computational infrastructure; specialized analyses may be libraries.

---

# 54. PROJECTION

Projection is the mapping from semantic information to something representable.

General form:

```text
Source
  ↓
Projection
  ↓
Representation
```

Sources can include:

```text
state
entity
property
relation
field
equation
event
dataset
derived data
```

---

# 55. Representation

A representation is how projected information is manifested.

Examples:

```text
circle
line
arrow
text
equation
graph
histogram
timeline
field arrows
heatmap
diagram
table
```

Representation is not the underlying model.

---

# 56. View

A view displays one or more representations.

A view may have presentation state:

```text
camera
zoom
layout
selection
visibility
scale
```

Presentation state must remain distinct from model state.

---

# 57. Projection Direction

Projection normally flows:

```text
Model/Data
    ↓
Representation
```

But interactive projections may also allow:

```text
Representation
    ↓
Model/Data
```

This must be explicitly declared.

Not every representation is editable.

---

# 58. Bidirectional Projection

Example:

```text
model.position
      ↓
screen position
```

A user drags the visual:

```text
screen position
      ↓
model.position
```

This enables interactive geometry and parameter manipulation.

---

# 59. Multiple Projections

One model may have many simultaneous projections:

```text
                    ┌── spatial view
                    ├── equation
Model ──────────────┼── graph
                    ├── table
                    ├── statistics
                    └── measurement
```

All may observe the same state.

---

# 60. Synchronization

When state changes:

```text
State'
```

all dependent observations and projections should update consistently.

This requires:

```text
dependency tracking
+
observation scheduling
+
projection invalidation
```

The exact execution strategy belongs to runtime semantics.

---

# 61. INTERACTION

Interaction represents external operations applied to the system.

Examples:

```text
play
pause
step
reset
change parameter
drag object
measure
select
construct
intervene
branch
restore
```

Interaction may affect:

```text
model state
parameters
simulation execution
view state
presentation
```

---

# 62. Interaction Is Not the Same as Animation

Dragging a particle may modify model state.

Moving a camera does not.

Fading a label does not.

Therefore interaction targets must be explicit.

---

# 63. Animation

Animation belongs primarily to presentation.

It changes presentation properties over presentation time:

```text
position
opacity
scale
rotation
style
visibility
camera
```

Animation may be:

```text
author-driven
state-driven
event-driven
interaction-driven
```

---

# 64. Simulation vs Animation

Simulation:

```text
Model
 ↓
Evolution
 ↓
State'
```

Animation:

```text
Presentation state
 ↓
Temporal transformation
 ↓
Presentation state'
```

They can be synchronized but must not be conflated.

---

# 65. Simulation Time vs Presentation Time

A simulation may represent:

```text
1,000 years
```

while a video lasts:

```text
30 seconds
```

Therefore:

```text
simulation_time ≠ presentation_time
```

A mapping between them can be established by the presentation layer.

---

# 66. MODEL TRANSFORMATION

Models may be transformed for computation or presentation.

Examples:

```text
continuous model
      ↓
discretization
```

```text
microscopic model
      ↓
aggregation
      ↓
macroscopic model
```

```text
3D model
      ↓
projection
      ↓
2D representation
```

These transformations should not alter the meaning of the original model unless explicitly intended.

---

# 67. Computational Representation

A semantic model may have several computational representations.

Example:

```text
Field E(x,y)
```

could be represented computationally as:

```text
function
grid
mesh
sparse structure
GPU buffer
```

The semantic field remains the same concept.

---

# 68. Semantic State vs Computational State

This distinction is now explicit.

### Semantic state

```text
particle.position
temperature(x,y)
population
```

### Computational state

```text
grid buffers
solver history
integration state
cache
GPU buffers
RNG state
```

### Presentation state

```text
camera
zoom
selection
layout
animation progress
```

These are different state spaces.

---

# 69. Runtime State

The eventual runtime may therefore maintain:

```text
RUNTIME
├── Semantic State
├── Computational State
└── Presentation State
```

with controlled relationships between them.

---

# 70. Scientific Model vs Execution Strategy

The semantic model describes:

```text
what the system means
```

while execution strategy determines:

```text
how it is calculated
```

For example:

```text
differential equation
```

could be solved by:

```text
Euler
Runge-Kutta
Verlet
analytical solution
custom solver
```

The semantic model should not depend on one.

---

# 71. Numerical Approximation

Numerical computation introduces:

```text
precision
step size
tolerance
error
convergence
stability
```

These belong to execution configuration.

The semantic model may remain exact while its execution is approximate.

---

# 72. Determinism

A model can be deterministic or stochastic.

A deterministic execution should ideally produce reproducible results given equivalent:

```text
model
initial state
parameters
execution configuration
```

A stochastic execution additionally requires:

```text
random source
seed
```

---

# 73. Reproducibility

An experiment should be reproducible through a record containing enough information to recreate the run.

Candidate provenance:

```text
model identity/version
initial state
parameters
random seed
execution strategy
numerical configuration
external inputs
```

This is primarily runtime/data infrastructure.

---

# 74. Experiment

An experiment is a procedure performed on a model.

Conceptually:

```text
Experiment
├── model
├── initial state
├── parameters
├── interventions
├── execution
├── observations
└── results
```

Experiment is therefore a higher-level construct composed from core concepts.

---

# 75. Snapshot

A snapshot captures a state at a particular point.

```text
Snapshot
├── semantic state
├── relevant runtime information
└── time/context
```

Snapshots enable:

```text
reset
pause
resume
restore
branch
compare
replay
```

---

# 76. Branching

A simulation can branch:

```text
             State S
                │
          ┌─────┴─────┐
          ↓           ↓
       Branch A     Branch B
```

Branches may differ in:

```text
parameters
interventions
random seed
user actions
```

This is especially valuable for interactive experiments.

---

# 77. INVALID STATES AND ERRORS

The system must distinguish at least:

```text
semantic invalidity
computational failure
runtime failure
presentation failure
```

Examples:

```text
semantic:
    impossible constraint

computational:
    solver diverged

runtime:
    resource unavailable

presentation:
    unsupported renderer
```

This separation prevents every failure from becoming a generic error.

---

# 78. Core Extension Mechanism

The core must allow new domain concepts to be defined using existing primitives.

For example:

```text
Projectile
```

can be constructed from:

```text
Entity
+
Position
+
Velocity
+
Mass
+
Gravity
+
Equations
+
Numerical Evolution
```

Similarly:

```text
Molecule
```

can be constructed from:

```text
Entities
+
Relations
+
Properties
+
Geometry
```

And:

```text
Graph
```

from:

```text
Entities
+
Relations
```

---

# 79. Domain Libraries

Domain libraries should therefore define concepts such as:

```text
Physics
├── Newtonian mechanics
├── electromagnetism
├── thermodynamics
└── waves

Chemistry
├── atoms
├── molecules
├── reactions
└── equilibrium

Biology
├── cells
├── populations
└── ecosystems

Computer Science
├── graphs
├── trees
├── algorithms
└── networks
```

These libraries should compile down to or compose over the core.

---

# 80. What Does NOT Belong in the Core

The following should not be fundamental semantic primitives merely because they are useful:

```text
Projectile
Pendulum
Atom
Molecule
Reaction
Planet
Circuit
Graph
Tree
SortingAlgorithm
Predator
Prey
CellularAutomaton
NewtonLaw
OhmLaw
IdealGas
```

They are domain abstractions.

---

# 81. What Also Does NOT Belong in the Semantic Core

Execution mechanisms should remain separate:

```text
EulerSolver
RK4Solver
PDESolver
CollisionEngine
MonteCarloEngine
GPUBackend
CPUBackend
SpatialIndex
Renderer
VelloBackend
WebRenderer
VideoEncoder
```

These implement capabilities rather than define scientific meaning.

---

# 82. Refined Core Graph

The complete semantic structure can now be summarized as:

```text
                         SYSTEM
                            │
          ┌─────────────────┼─────────────────┐
          ↓                 ↓                 ↓
       CONTEXT            STATE            BEHAVIOR
          │                 │                 │
      ┌───┼───┐      ┌──────┼──────┐    ┌────┼─────┐
      ↓   ↓   ↓      ↓      ↓      ↓    ↓    ↓     ↓
   DOMAIN SPACE TIME OBJECTS VALUES RELATIONS FUNC EQUATION RULE
                         │       │       │       │
                         │       │       │       └── PROCESS
                         │       │       └────────── EVENT
                         │       └────────────────── CONSTRAINT
                         │
                     COLLECTIONS
                         │
                       FIELDS
                         │
                         ↓
                      EVOLUTION
                         │
              ┌──────────┴──────────┐
              ↓                     ↓
          OBSERVATION          INTERVENTION
              │                     │
              ↓                     ↓
             DATA                STATE'
              │
           ANALYSIS
              │
              ↓
        DERIVED DATA
              │
              ↓
         PROJECTION
              │
              ↓
       REPRESENTATION
              │
              ↓
             VIEW
              │
              ↓
        INTERACTION
              │
              └──────────────→ MODEL
```

---

# 83. Refined Primitive Candidates

After all validation and gap analysis, the strongest candidates for genuine semantic primitives are:

```text
Value
Type
Quantity
Unit
Dimension
Domain

Object
Identity
Property
Variable
Collection
Relation
Field

State

Expression
Function
Predicate
Equation
Rule
Process
Event
Constraint

Space
Time
Parameter
Randomness

Observation
Measurement
Data
Analysis

Projection
Representation
View

Subsystem
Interface
Dependency
```

This is a candidate set, not yet a final language specification.

---

# 84. Concepts That May Collapse Into More General Ones

The next stage should investigate whether some of these are actually special cases.

For example:

```text
Property
Variable
DerivedValue
```

might be unified under a broader value-binding abstraction.

Likewise:

```text
Rule
Equation
Constraint
Function
```

may share a common expression/relationship foundation.

And:

```text
Entity
Object
Subsystem
```

may share an identity/composition foundation.

This should be resolved during runtime semantics and DSL design rather than prematurely.

---

# 85. Core Semantic Layers

The refined model can be viewed as five major layers.

## Layer 1 Mathematics

```text
values
types
quantities
units
dimensions
domains
expressions
functions
equations
predicates
distributions
```

## Layer 2 Model

```text
objects
identity
properties
variables
collections
relations
fields
state
constraints
```

## Layer 3 Dynamics

```text
rules
processes
events
transitions
time
randomness
dependencies
```

## Layer 4 Observation

```text
measurements
data
analysis
experiments
```

## Layer 5 Presentation

```text
projections
representations
views
interaction
animation
```

Execution sits across these layers:

```text
computation
runtime
scheduling
solvers
storage
parallelism
```

rather than being part of the semantic hierarchy.

---

# 86. The Core Invariants

The following principles should now be treated as architectural invariants.

### Invariant 1

```text
Model ≠ Scene
```

### Invariant 2

```text
Model ≠ Simulation
```

### Invariant 3

```text
Simulation ≠ Animation
```

### Invariant 4

```text
Model Space ≠ Render Space
```

### Invariant 5

```text
Semantic State ≠ Computational State
```

### Invariant 6

```text
Semantic State ≠ Presentation State
```

### Invariant 7

```text
Equation ≠ Solver
```

### Invariant 8

```text
Domain Concept ≠ Core Primitive
```

### Invariant 9

```text
Observation ≠ Intervention
```

### Invariant 10

```text
Representation ≠ What It Represents
```

These invariants should guide the rest of the architecture.

---

# 87. What the Refined Model Can Now Express

The core can describe:

```text
static mathematics
dynamic mathematics
particle systems
field systems
equation-based systems
rule-based systems
graph systems
agent systems
population systems
hybrid systems
stochastic systems
continuous systems
discrete systems
event-driven systems
experimental systems
data-driven systems
educational visualizations
pure animations
```

without requiring those domains to be built into the semantic core.

---

# 88. The Core Is Not Yet an Execution Model

This is deliberate.

We have defined:

```text
what exists
what values mean
what relationships exist
what behavior means
what state is
what observation means
what projection means
```

We have not yet fully defined:

```text
when things execute
how continuous evolution works
how events are scheduled
how solvers interact
how dependencies are evaluated
how state commits
how conflicts are resolved
how randomness is consumed
how snapshots work
how parallelism works
```

Those belong to the next stage.

---

# 89. Boundary Between Core and Runtime

The boundary can now be stated as:

```text
CORE
"What does this model mean?"

RUNTIME
"How do we execute this model?"
```

For example:

```text
Core:
    dN/dt = -λN

Runtime:
    solve using RK4 with timestep 0.01
```

Or:

```text
Core:
    particle A collides with particle B

Runtime:
    detect collision using spatial index
    calculate collision time
    apply solver
```

---

# 90. Boundary Between Core and Renderer

Similarly:

```text
CORE:
    particle.position = (x,y)

PROJECTION:
    position → visual coordinate

REPRESENTATION:
    circle

RENDERER:
    draw circle
```

The renderer should not need to know what a "particle" scientifically means.

---

# 91. Boundary Between Core and Domain Library

Example:

```text
Physics Library
    ↓
defines:
    gravity
    force
    momentum
    collision
```

using:

```text
Core
    Entity
    Quantity
    Vector
    Function
    Equation
    Constraint
    Process
```

This gives the core its desired domain independence.

---

# 92. Boundary Between Core and Education

An educational activity can wrap the model:

```text
Lesson
 ↓
Experiment
 ↓
Model
 ↓
Observation
 ↓
Question
 ↓
Student response
 ↓
Feedback
```

Educational semantics therefore remain above the scientific core.

---

# 93. Refined Architecture

The entire system can now be viewed as:

```text
                  ┌─────────────────────────┐
                  │       DOMAIN LIBRARIES  │
                  │ physics / chemistry /   │
                  │ biology / CS / math     │
                  └────────────┬────────────┘
                               ↓
                  ┌─────────────────────────┐
                  │          CORE           │
                  │                         │
                  │ values / domains        │
                  │ objects / state         │
                  │ relations / fields      │
                  │ behavior / constraints  │
                  │ observation / data      │
                  │ projection              │
                  └────────────┬────────────┘
                               ↓
                  ┌─────────────────────────┐
                  │         RUNTIME         │
                  │                         │
                  │ scheduling / solvers    │
                  │ evolution / events      │
                  │ numerical computation   │
                  │ randomness / replay     │
                  │ caching / parallelism   │
                  └────────────┬────────────┘
                               ↓
                  ┌─────────────────────────┐
                  │      PRESENTATION       │
                  │                         │
                  │ representations         │
                  │ views / interaction     │
                  │ animation / rendering   │
                  └────────────┬────────────┘
                               ↓
                  ┌─────────────────────────┐
                  │       EDUCATION         │
                  │                         │
                  │ lessons / activities    │
                  │ assessment / narration  │
                  └─────────────────────────┘
```

The layers communicate through explicit semantic boundaries.

---

# 94. Final Assessment

The generalized model has now survived:

```text
24 initial validation cases
+
extended gap analysis
```

The result is not that every implementation problem has been solved.

Rather, we now have a much stronger answer to:

> **What is this system actually modeling?**

It is not primarily:

```text
a scene graph
```

and not primarily:

```text
a physics engine
```

and not primarily:

```text
an animation library
```

It is a:

> **general state-and-meaning system with mathematical structure, behavior, evolution, observation, projection, and interaction.**

---

# 95. Next Stage

The next artifact is now:

```text
06 Runtime Semantics
```

This is where the architecture must answer the difficult operational questions.

Among them:

```text
1. What exactly is a simulation step?

2. What happens when multiple processes modify the same state?

3. How are continuous processes integrated?

4. How are discrete events scheduled?

5. How do stochastic events interact with deterministic evolution?

6. How are hybrid systems executed?

7. How are dependencies evaluated?

8. When does a state become committed?

9. How are observations synchronized with evolution?

10. How are projections updated?

11. How do interactions enter the simulation?

12. How do snapshots and branching work?

13. How does deterministic replay work?

14. How do multiple solvers coexist?

15. How are numerical errors handled?

16. How are computational and semantic states synchronized?

17. How is simulation execution decoupled from rendering?

18. How does parallel/GPU execution affect semantics?
```

Only after those questions are resolved should the DSL syntax be designed.

The intended progression is therefore now:

```text
CASE SPACE
    ↓
CAPABILITIES
    ↓
TAXONOMY
    ↓
GENERALIZED MODEL
    ↓
VALIDATION
    ↓
GAP ANALYSIS
    ↓
REFINED CORE MODEL
    ↓
★ RUNTIME SEMANTICS
    ↓
DSL DESIGN
    ↓
IMPLEMENTATION ARCHITECTURE
```

The core model is now sufficiently mature to make the next step meaningful: **define exactly how this abstract system behaves when it runs.**
