# Prismal
## 05b Model Gap Analysis and Extended Validation

### Purpose

The initial validation demonstrated that the generalized model can express a wide range of mathematical, scientific, computational, and educational cases.

This document goes one level deeper.

The question is no longer simply:

> Can we describe the example?

Instead:

> **Does the generalized model have the right concepts and relationships to remain expressive when the examples become more difficult?**

The analysis therefore focuses on gaps, ambiguities, hidden assumptions, and boundaries between the core model and specialized runtime capabilities.

---

# 1. Gap Analysis Method

Each suspected gap is classified as:

| Classification | Meaning |
|---|---|
| **G0: No gap** | Existing concepts are sufficient |
| **G1: Composition gap** | Existing primitives work, but composition needs clearer semantics |
| **G2: Missing core concept** | A genuinely fundamental concept is absent |
| **G3: Computation gap** | The model is sufficient; execution capability is missing |
| **G4: Representation gap** | The model is sufficient; visualization capability is missing |
| **G5: Runtime gap** | Execution/lifecycle semantics need to be defined |
| **G6: Library abstraction** | Useful concept belongs above the core |
| **G7: Unresolved** | More validation is required |

The goal is to avoid turning every difficult case into a new primitive.

---

# 2. Gap A: Collections and Populations

Many cases involve large or variable numbers of similar entities:

```text
10,000 particles
1,000,000 molecules
cells in a tissue
nodes in a graph
students in a dataset
radioactive nuclei
```

The current model has:

```text
Entity
```

but an entity-by-entity representation becomes problematic.

## Required operations

A collection may need:

```text
create
destroy
filter
select
map
reduce
aggregate
sample
iterate
group
sort
count
```

It may also have population-level properties:

```text
mean velocity
total energy
population size
distribution
```

## Deeper issue

There are actually multiple meanings of collection.

### Static collection

```text
set of graph nodes
```

### Dynamic collection

```text
particles created/destroyed during simulation
```

### Population

```text
N nuclei
```

### Data collection

```text
observations over time
```

### Mathematical set

```text
x ∈ A
```

These should not necessarily become one implementation type.

## Finding

**G1: Composition gap**

Collections are fundamental enough to require explicit semantics, but their exact abstraction remains to be determined.

---

# 3. Gap B: Units and Dimensions

Scientific quantities are not merely numbers.

```text
5 m
5 s
5 kg
```

cannot be freely combined.

For example:

\[
v=\frac{d}{t}
\]

produces:

```text
Length / Time
```

while:

\[
F=ma
\]

produces:

```text
Mass × Length / Time²
```

## Requirements

The model should be able to distinguish:

```text
value
unit
dimension
```

and perform dimensional reasoning.

Example:

```text
distance : Length
time     : Time

distance / time
    → Velocity
```

## Important distinction

Unit and dimension are not identical.

```text
1 m
100 cm
```

have different units but the same dimension.

Therefore:

```text
Dimension
Unit
Quantity
```

should probably be distinct concepts.

## Finding

**G2: Missing core concept**

Dimensionality is sufficiently fundamental to scientific computation that it should be represented by the core mathematical model.

Units may be a closely related core capability.

---

# 4. Gap C: Domains and Index Spaces

Consider:

\[
f(x),\quad x\in[-1,1]
\]

versus:

```text
grid[i,j]
```

versus:

```text
node ∈ Graph
```

versus:

```text
particle ∈ Population
```

These are all different kinds of domains.

The model currently has values and collections, but the concept of **where a quantity is defined** needs clarification.

## Candidate abstraction

```text
Domain
```

A domain may be:

```text
continuous
discrete
finite
infinite
spatial
temporal
parameter
categorical
topological
```

Examples:

```text
x ∈ ℝ
(x,y) ∈ ℝ²
i ∈ {0...,99}
node ∈ Graph
t ∈ [0,10]
```

## Finding

**G2: Missing core concept**

A generalized scientific model needs an explicit concept for domains/index spaces.

---

# 5. Gap D: Initial Conditions

An equation does not necessarily define a unique simulation.

For:

\[
\frac{dx}{dt}=v
\]

we also need:

\[
x(0)=x_0
\]

and possibly:

\[
v(0)=v_0
\]

Therefore the model needs to distinguish:

```text
law
```

from:

```text
initial state
```

The existing `State` concept already handles the latter, but the relationship needs explicit semantics.

## Finding

**G1: Composition gap**

No new primitive may be necessary.

Initial conditions can likely be ordinary state plus experiment configuration.

---

# 6. Gap E: Boundary Conditions

Field problems introduce another issue.

For example:

\[
\frac{\partial T}{\partial t}
=
\alpha\nabla^2T
\]

does not fully specify the problem without boundary conditions.

Examples:

```text
T = 100°C at boundary
```

or:

```text
∂T/∂n = 0
```

or:

```text
periodic boundary
```

## Question

Are boundary conditions:

```text
Constraint?
```

or:

```text
Relation?
```

or:

```text
Domain property?
```

The generalized model currently has enough ingredients to represent them, but the semantics need to be made explicit.

## Finding

**G1: Composition gap**

Likely a specialized form of constraints applied to domain boundaries.

---

# 7. Gap F: Constraints

Constraints occur everywhere:

### Physics

\[
|\mathbf v|=c
\]

### Geometry

```text
point lies on circle
```

### Chemistry

```text
mass conserved
```

### Graphs

```text
edge connects two nodes
```

### Optimization

```text
x ≥ 0
```

### UI/layout

```text
object stays inside region
```

The generalized model already includes constraints.

But constraints can have very different semantics.

## Types

```text
hard
soft
equality
inequality
geometric
physical
logical
optimization
boundary
```

They may:

```text
reject state
correct state
penalize state
trigger event
report violation
```

## Finding

**G1: Semantic gap**

`Constraint` is a core concept, but its execution semantics need explicit definition.

---

# 8. Gap G: Symbolic Mathematics

The initial tests mostly used numerical evaluation.

But the system must also support things such as:

\[
(x+1)^2
\]

expanding to:

\[
x^2+2x+1
\]

or:

\[
\frac{d}{dx}\sin x=\cos x
\]

or:

\[
\int 2x\,dx=x^2+C
\]

## Requirements

Potential capabilities include:

```text
expression tree
symbolic substitution
simplification
differentiation
integration
factorization
equation solving
symbolic comparison
```

## Critical distinction

Symbolic mathematics is not the same as numerical computation.

Therefore:

```text
Expression
```

should exist independently of:

```text
Numeric evaluation
```

## Finding

**G2: Missing/under-specified core capability**

The model has expressions and equations, but symbolic transformation semantics need to be explicitly incorporated.

---

# 9. Gap H: Functions as First-Class Objects

A function can be:

```text
f(x)=x²
```

but also:

```text
f : Vector → Vector
```

or:

```text
field : Position → Vector
```

or:

```text
rule : State → State
```

or:

```text
observation : State → Data
```

This suggests a deeper abstraction:

```text
Function
    input domain
    output domain
    mapping
```

Functions should not be restricted to scalar mathematics.

## Finding

**G0: Existing concept sufficient**

The generalized model already contains functions, but the eventual type/domain system must make their generality explicit.

---

# 10. Gap I: Distributions and Uncertainty

Stochastic systems require more than a random number generator.

Examples:

```text
Normal(μ,σ)
Poisson(λ)
Binomial(n,p)
```

A model may contain:

```text
random variable
probability distribution
uncertain parameter
sample
expectation
variance
```

## Example

Instead of:

```text
mass = 5 kg
```

we may have:

```text
mass ~ Normal(5 kg, 0.1 kg)
```

The model may then propagate uncertainty.

## Finding

**G2: Missing core mathematical capability**

Probability distributions and uncertainty should be investigated as first-class mathematical objects.

---

# 11. Gap J: Randomness vs Stochastic State

There is a subtle distinction between:

```text
randomness
```

and:

```text
stochastic state
```

For example:

```text
random() → value
```

is merely a computation.

But radioactive decay is a stochastic process:

```text
state
+
probability law
+
time
→
random event
```

Similarly, a Monte Carlo experiment samples a distribution repeatedly.

Therefore the system should distinguish:

```text
RNG
Random Variable
Distribution
Stochastic Process
Sample
```

## Finding

**G1: Semantic refinement required**

Randomness is not one concept.

---

# 12. Gap K: Continuous vs Discrete State

Consider:

```text
x(t)
```

versus:

```text
x[n]
```

The generalized model supports both, but the distinction affects:

- evolution
- observation
- integration
- indexing
- interpolation
- rendering

A hybrid system may contain both:

```text
continuous position
+
discrete mode
```

Example:

```text
traffic light:
    discrete state

vehicle:
    continuous position
```

## Finding

**G1: Existing concepts need explicit typing/semantics**

No new fundamental primitive is required.

---

# 13. Gap L: Event Time

An event-driven system does not necessarily have fixed timesteps.

Example:

```text
t = 0
collision at 0.372
decay at 1.824
collision at 2.041
```

The runtime may jump directly between events.

Therefore:

```text
time
```

must support:

```text
continuous clock
discrete clock
scheduled events
condition-triggered events
event queues
```

## Finding

**G3: Runtime capability**

The model can describe event-driven evolution, but execution semantics need to support it.

---

# 14. Gap M: Dynamic Creation and Destruction

Some systems change their own topology.

Examples:

```text
particle created
particle destroyed
chemical species generated
cell divides
graph node added
graph edge removed
```

Therefore state is not necessarily:

```text
fixed entities + changing properties
```

It may be:

```text
changing entities + changing relations
```

## Finding

**G1: Composition/runtime semantics gap**

The model needs explicit lifecycle semantics.

Candidate operations:

```text
create
destroy
attach
detach
add relation
remove relation
```

These need not be domain-specific.

---

# 15. Gap N: Hierarchical State

Consider a molecule:

```text
molecule
├── atom
├── atom
└── atom
```

or a simulation:

```text
system
├── subsystem A
│   ├── ...
│   └── ...
└── subsystem B
```

Or a biological system:

```text
organism
├── organ
│   ├── tissue
│   │   └── cell
```

The current model supports composition, but hierarchy deserves explicit semantics.

## Finding

**G1: Composition gap**

Hierarchy can probably be constructed from entities, collections, relations, and subsystems.

---

# 16. Gap O: References and Identity

If an entity is observed:

```text
probe reads particle A
```

the system needs stable identity.

Identity is especially important when:

```text
entities are created
entities are destroyed
collections reorder
relations change
```

A visual object and a model entity must not be confused.

## Finding

**G2: Missing core semantic detail**

Entities require stable identity semantics.

---

# 17. Gap P: Dependency Graph

Expressions and derived properties naturally create dependencies.

Example:

```text
position
   ↓
velocity
   ↓
kinetic_energy
   ↓
total_energy
```

Another example:

```text
charge distribution
        ↓
electric field
        ↓
force
        ↓
acceleration
        ↓
velocity
        ↓
position
```

The runtime therefore needs to know:

```text
what depends on what?
```

This is relevant for:

- evaluation order
- caching
- incremental updates
- reactive views
- invalidation

## Finding

**G2: Core semantic/runtime requirement**

Dependency relationships are fundamental to a usable reactive scientific model.

---

# 18. Gap Q: Cyclic Dependencies

The previous example becomes more difficult when feedback exists:

```text
particle
   ↓
field
   ↓
force
   ↓
particle
```

This is not necessarily an invalid dependency.

It can represent coupled equations.

Therefore the runtime cannot simply require an acyclic dependency graph.

It may need to distinguish:

```text
acyclic evaluation
```

from:

```text
iterative/coupled evolution
```

## Finding

**G3: Computation/runtime gap**

The semantic model can express cycles, but the runtime needs strategies for solving coupled dependencies.

---

# 19. Gap R: Algebraic Constraints vs Evolution Equations

Consider:

\[
F=ma
\]

versus:

\[
\frac{dx}{dt}=v
\]

versus:

\[
x+y=1
\]

These all look like equations but have different roles.

An equation may:

```text
define a quantity
constrain variables
describe evolution
define a relation
```

Therefore `Equation` should not imply a single execution strategy.

## Finding

**G1: Semantic refinement**

Equations need explicit roles or interpretation.

---

# 20. Gap S: Differential Equations of Different Order

The model must handle:

```text
dx/dt = f(x,t)
```

and:

```text
d²x/dt² = f(x,v,t)
```

and systems:

```text
dx/dt = f(x,y)
dy/dt = g(x,y)
```

and PDEs:

```text
∂u/∂t = F(u, ∇u...)
```

This should not result in separate primitives:

```text
ODE
PDE
SecondOrderODE
```

unless later validation proves that necessary.

## Finding

**G0/G3**

The mathematical model can represent these as equations.

The computational layer requires specialized solvers.

---

# 21. Gap T: Discretization

A continuous model may be computed through:

```text
finite difference
finite element
finite volume
particle approximation
spectral method
```

The same equation can therefore have multiple computational realizations.

Example:

```text
semantic field
      ↓
discretization
      ↓
numerical representation
      ↓
solver
```

## Finding

**G3: Computation architecture**

Discretization should be a computation-level abstraction, not part of the semantic domain model unless the author explicitly chooses to model the discretization itself.

---

# 22. Gap U: Numerical Error and Tolerance

Numerical computation introduces:

```text
absolute tolerance
relative tolerance
step size
precision
convergence
stability
error estimate
```

These affect execution but generally do not change the underlying scientific meaning.

## Finding

**G3: Runtime/computation capability**

---

# 23. Gap V: Approximate vs Exact Values

Mathematics may contain:

```text
π
√2
1/3
```

while numerical execution may use:

```text
3.14159...
1.41421...
0.33333...
```

Therefore the model may need to preserve exact expressions even when evaluating numerically.

## Finding

**G2: Core mathematical capability**

Values may need at least:

```text
exact symbolic form
numeric approximation
```

with explicit conversion/evaluation.

---

# 24. Gap W: Geometry vs Topology

Geometry answers:

```text
where?
how large?
what shape?
```

Topology answers things such as:

```text
connected?
inside?
adjacent?
boundary?
```

Graphs and meshes also introduce topological structure.

The existing model has geometry and relations, but these concepts should not be conflated.

## Finding

**G1: Semantic refinement**

Geometry and topology should be related but distinct.

---

# 25. Gap X: Coordinate Systems and Transformations

A single model may involve:

```text
world coordinates
local coordinates
object coordinates
screen coordinates
graph coordinates
polar coordinates
```

Example:

```text
model position
      ↓
world transform
      ↓
camera transform
      ↓
screen position
```

The renderer needs transformations without changing the underlying model.

## Finding

**G0: Existing space/projection concepts are sufficient**

But coordinate-transform semantics must be formally specified later.

---

# 26. Gap Y: Model Geometry vs Render Geometry

A physical object may have:

```text
semantic geometry:
    radius = 0.1 m
```

while its visual representation may use:

```text
circle radius = 20 px
```

These are not the same property.

Likewise, an atom might be visually enlarged for educational purposes.

Therefore:

```text
model geometry
```

must remain separate from:

```text
presentation geometry
```

## Finding

**G0: Existing model/view separation handles this**

This distinction must be preserved rigorously.

---

# 27. Gap Z: Measurement Changes the Model

Most observations are passive:

```text
read position
```

But some experiments have interventions:

```text
apply force
remove particle
change temperature
insert probe
```

Some domains also have observation processes that affect the system.

Therefore the system needs to distinguish:

```text
Observation
```

from:

```text
Intervention
```

and:

```text
Interaction
```

## Finding

**G2: Semantic distinction required**

Observation and intervention should not be treated as identical operations.

---

# 28. Gap AA: Measurement Has Semantics

A measurement is not merely:

```text
read(property)
```

It may involve:

```text
instrument
sampling interval
resolution
noise
uncertainty
calibration
measurement transformation
```

For example, the true position may be:

```text
x = 1.234567 m
```

while an instrument reports:

```text
1.23 m ± 0.01 m
```

## Finding

**G2: Observation/measurement model needs expansion**

This becomes especially important for educational experiments and realistic scientific simulation.

---

# 29. Gap AB: Data Is More Than a List

A dataset may have:

```text
dimensions
axes
units
timestamps
metadata
uncertainty
provenance
sampling rate
```

Examples:

```text
temperature(t)
position(t)
pressure(x,y,t)
```

Therefore data should probably have explicit structure.

## Finding

**G2: Core data semantics need strengthening**

---

# 30. Gap AC: Derived Quantities

Many scientific quantities are not independently stored.

Example:

\[
v=\frac{dx}{dt}
\]

\[
K=\frac12mv^2
\]

\[
E=mc^2
\]

These can be derived from existing state.

Therefore the model needs a clear distinction between:

```text
stored state
```

and:

```text
derived state
```

## Finding

**G1: Semantic refinement**

---

# 31. Gap AD: Caching Derived Values

Once derived values exist:

```text
kinetic_energy
electric_field
total_energy
```

recomputing everything after every change may be expensive.

The runtime may need:

```text
dependency tracking
memoization
incremental evaluation
invalidations
```

## Finding

**G3: Runtime capability**

---

# 32. Gap AE: Multiple Scales

Scientific systems often operate at several scales.

Example:

```text
microscopic molecules
        ↓
mesoscopic structure
        ↓
macroscopic temperature
```

A single simulation may couple levels.

## Requirements

Potentially:

```text
multi-scale model
scale mapping
aggregation
coarse-graining
coupling
```

## Finding

**G1/G3**

The core can probably represent multiple scales, but explicit multi-scale computational semantics may be needed.

---

# 33. Gap AF: Model Fidelity

The same phenomenon may have different models.

For radioactive decay:

```text
microscopic stochastic model
```

and:

```text
macroscopic deterministic model
```

Both are valid.

Therefore a model needs some notion of:

```text
assumptions
approximation
scope
validity range
```

For example:

```text
air resistance ignored
```

or:

```text
ideal gas assumption
```

## Finding

**G2: Model metadata/semantics**

Assumptions should be representable, though they may not affect computation directly.

---

# 34. Gap AG: Model Composition

Suppose:

```text
thermal system
```

is composed with:

```text
mechanical system
```

which is composed with:

```text
electrical system
```

They may share:

```text
temperature
force
energy
time
```

Composition therefore needs explicit interfaces.

## Candidate structure

```text
Subsystem
├── inputs
├── outputs
├── internal state
├── behavior
└── interface
```

## Finding

**G2: Core composition semantics**

Subsystem composition is too important to leave entirely implicit.

---

# 35. Gap AH: Coupling

Composition is not enough.

Subsystems can influence each other:

```text
A → B
B → A
```

or:

```text
A ↔ B
```

Examples:

```text
particle ↔ field
temperature ↔ pressure
predator ↔ prey
motor ↔ electrical circuit
```

## Finding

**G1/G3**

Coupling can likely be represented through dependencies, shared state, relations, and equations.

Runtime semantics must handle coupled evolution.

---

# 36. Gap AI: State Snapshots and Branching

Interactive experiments require:

```text
save state
restore state
branch
compare runs
```

Example:

```text
             initial state
                  │
            ┌─────┴─────┐
            ↓           ↓
        experiment A experiment B
```

This is especially important for education.

## Finding

**G5: Runtime capability**

---

# 37. Gap AJ: Reproducibility

A stochastic experiment should be reproducible from something like:

```text
model version
initial state
parameters
random seed
computation configuration
```

Potentially also:

```text
solver version
precision
```

## Finding

**G5: Runtime/provenance capability**

Reproducibility should be an explicit design goal.

---

# 38. Gap AK: Deterministic Replay

Interactive simulations may need to replay an exact run.

This becomes difficult if:

```text
parallel execution
floating-point differences
nondeterministic scheduling
GPU computation
```

are involved.

Therefore deterministic replay cannot simply mean:

```text
same seed
```

## Finding

**G5: Runtime requirement**

Replay semantics need to be defined separately from ordinary randomness.

---

# 39. Gap AL: Real-Time Data

The model may receive external data:

```text
sensor → model
camera → model
network → model
user input → model
```

This creates:

```text
external input
```

rather than purely internally generated state.

## Finding

**G2: Input/source abstraction**

The model should distinguish:

```text
internal state
external input
observation
intervention
```

---

# 40. Gap AM: Streaming

Data may arrive continuously:

```text
sensor
 ↓
stream
 ↓
observation
 ↓
model
```

The system should not assume every dataset exists completely before execution.

## Finding

**G3/G5: Runtime capability**

Streaming can probably be built over existing data/input concepts.

---

# 41. Gap AN: Agent-Based Models

Agents have:

```text
state
behavior
goals
perception
decision
action
```

Example:

```text
agent
 ↓
observe environment
 ↓
decide
 ↓
act
 ↓
environment changes
```

This introduces a potentially important distinction:

```text
Process
```

versus:

```text
Agent behavior
```

But agent behavior may simply be a composition of:

```text
observation
+
rule
+
state transition
+
interaction
```

## Finding

**G6: Library abstraction**

`Agent` should initially remain a higher-level abstraction.

---

# 42. Gap AO: Field vs Distributed Entity State

Some systems can be represented either way.

Example:

```text
temperature field
```

or:

```text
many particles carrying temperature
```

These are not always equivalent computationally.

The model therefore must not silently collapse:

```text
field
```

into:

```text
collection of entities
```

## Finding

**G0**

The generalized model correctly keeps both as first-class alternatives.

---

# 43. Gap AP: Topology Changes

Some systems alter their structure:

```text
graph edge added
bond broken
mesh refined
cell divides
network connection lost
```

This differs from merely changing property values.

## Finding

**G2: Lifecycle/topology semantics**

Dynamic topology is sufficiently common to deserve explicit core semantics.

---

# 44. Gap AQ: Hierarchical Time

Some systems operate on multiple timescales:

```text
fast collision
slow chemical reaction
very slow population change
```

A single global timestep may be inefficient or impossible.

Potential mechanisms:

```text
substeps
multiple clocks
event scheduling
adaptive timesteps
multi-rate integration
```

## Finding

**G3: Computation/runtime capability**

The semantic model can remain independent of the execution strategy.

---

# 45. Gap AR: Multiple Simultaneous Clocks

The distinction between:

```text
simulation time
presentation time
wall-clock time
```

was already established.

Extended validation adds:

```text
subsystem time
event time
measurement time
```

However, these should not automatically become independent clocks.

The core should establish a general notion of:

```text
time context
```

with specialized runtime scheduling mechanisms.

## Finding

**G1/G5**

---

# 46. Gap AS: Animation of Things That Are Not Model State

A label can animate:

```text
opacity 0 → 1
```

without changing scientific state.

A camera can move:

```text
camera.x += ...
```

without affecting the simulation.

Therefore animation needs its own state space:

```text
presentation state
```

## Finding

**G0**

The model/view separation already supports this.

---

# 47. Gap AT: Authorial Intent

Educational visualization frequently requires instructions such as:

```text
show this
then hide this
highlight this
ask a question
wait for answer
continue
```

These are not scientific model semantics.

They belong to:

```text
presentation
education
animation
```

## Finding

**G6**

Authorial sequencing should remain above the scientific model.

---

# 48. Gap AU: User-Defined Representations

The user may invent a representation that the core library does not know.

Example:

```text
represent pressure as moving arrows
represent probability as particle density
represent an algorithm as a race
```

Therefore representation should be extensible.

## Finding

**G2: Extension mechanism required**

The core representation model must allow user-defined mappings from model/data to visual structures.

---

# 49. Gap AV: Representation Depends on Data

A visualization may depend not just on state but on derived data.

Example:

```text
state
 ↓
observation
 ↓
histogram
```

or:

```text
trajectory
 ↓
Fourier transform
 ↓
frequency spectrum
```

Therefore:

```text
Projection
```

should accept:

```text
state
data
derived data
```

## Finding

**G0**

The generalized model already allows this, but the projection semantics should explicitly include data pipelines.

---

# 50. Gap AW: Analysis as a First-Class Operation

Scientific visualization often includes analysis:

```text
mean
variance
FFT
regression
derivative
integral
correlation
fit
```

These may operate on observations rather than model state.

Therefore:

```text
Data
 ↓
Analysis
 ↓
Derived Data
```

should be recognized as a valid pipeline.

## Finding

**G2/G6**

Basic analysis is likely core computational infrastructure; specialized analyses can be libraries.

---

# 51. Gap AX: Optimization

Optimization appears in:

- parameter fitting
- engineering design
- machine learning
- physics
- geometry
- control

Generic structure:

\[
\min_x f(x)
\]

subject to constraints.

The existing model has:

```text
Function
Constraint
```

but needs an optimization computation capability.

## Finding

**G3: Computation capability**

No new semantic primitive is necessarily required.

---

# 52. Gap AY: Inverse Problems

Forward:

```text
parameters
 ↓
model
 ↓
observations
```

Inverse:

```text
observations
 ↓
infer parameters
 ↓
model
```

Example:

```text
observed decay curve
        ↓
estimate λ
```

This is an important scientific workflow.

## Finding

**G3/G6**

The model is sufficient; inference/optimization belongs primarily in computation/analysis libraries.

---

# 53. Gap AZ: Uncertainty Propagation

If:

```text
x ± σx
```

is used to compute:

\[
y=f(x)
\]

then uncertainty may propagate to \(y\).

This requires:

```text
uncertain value
distribution
sampling
propagation
analysis
```

## Finding

**G2/G3**

The mathematical model needs uncertainty semantics; numerical methods for propagation belong to computation.

---

# 54. Gap BA: Scale of Representation

A model may contain:

```text
10^9 molecules
```

but rendering all of them is impossible.

The system may need:

```text
aggregation
sampling
level of detail
statistical representation
instancing
```

This is particularly important for a 2D visualization runtime.

## Finding

**G3/G4**

The semantic model should remain independent of rendering scale.

---

# 55. Gap BB: Rendering Is Not Simulation

A simulation may run at:

```text
1,000,000 steps/sec
```

while the renderer displays:

```text
60 frames/sec
```

Therefore:

```text
simulation execution
```

and:

```text
render execution
```

must be decoupled.

## Finding

**G5: Runtime architecture**

This is now strongly validated.

---

# 56. Gap BC: Simulation Without Rendering

A model may run purely to produce:

```text
dataset
```

with no visual representation.

Conversely, a visualization may run without simulation.

Therefore both must be valid:

```text
Model → Data
```

and:

```text
Model → View
```

## Finding

**G0**

This should be a foundational architectural rule.

---

# 57. Gap BD: Rendering Without a Scientific Model

Pure animations can exist:

```text
shape
 ↓
transform
 ↓
fade
 ↓
text
```

with no scientific model.

Therefore the presentation subsystem should be usable independently.

## Finding

**G0**

---

# 58. Gap BE: Educational Semantics

An educational activity may contain:

```text
objective
instructions
prediction
interaction
experiment
measurement
feedback
assessment
```

These should not pollute the scientific model.

## Finding

**G6**

Education is a layer built over the model/runtime/presentation system.

---

# 59. Gap BF: Accessibility

A scientific visualization may need:

```text
text alternatives
keyboard interaction
screen-reader descriptions
high-contrast modes
audio narration
non-visual data representation
```

These are not merely rendering details.

They affect how representations are exposed.

## Finding

**G4/G6**

Accessibility belongs primarily to the presentation/output layer, with hooks into the semantic model.

---

# 60. Gap BG: Provenance

Scientific results may need to record:

```text
model
parameters
initial state
seed
solver
version
observations
transformations
```

This matters for reproducibility and educational experiment comparison.

## Finding

**G5**

Provenance should be a runtime/data capability.

---

# 61. Gap BH: Model Versioning

If the model changes:

```text
v1
→
v2
```

old experiment results may no longer be reproducible.

Therefore saved experiments may need to identify the model version.

## Finding

**G5**

---

# 62. Gap BI: Failure and Invalid State

What happens when:

```text
division by zero
negative concentration
invalid geometry
numerical instability
constraint violation
solver divergence
```

occurs?

The model cannot simply assume every state is valid.

Possible outcomes:

```text
reject
clamp
repair
warn
error
continue
```

## Finding

**G2/G3**

The system needs explicit validity/error semantics.

---

# 63. Gap BJ: Partial State

Large systems may not have all values available at all times.

Examples:

```text
streaming sensor
lazy computation
remote data
partially initialized model
```

This introduces:

```text
unknown
unavailable
pending
invalid
```

as states distinct from ordinary values.

## Finding

**G2: Semantic state refinement**

---

# 64. Gap BK: Identity of Mathematical Objects

An equation can itself be:

```text
displayed
referenced
modified
observed
used computationally
```

Likewise a function can be:

```text
passed as input
plotted
differentiated
sampled
composed
```

Therefore mathematical constructs need identity/references just like entities.

## Finding

**G1**

The generalized model should not restrict identity to physical entities.

---

# 65. Gap BL: Composition of Functions and Processes

We may have:

```text
f(x)
```

then:

```text
g(f(x))
```

or:

```text
observation
 ↓
analysis
 ↓
projection
```

or:

```text
process A
 ↓
process B
```

This suggests a general composition mechanism.

## Finding

**G2: Core composition semantics**

Composition is one of the most important abstractions to formalize.

---

# 66. Gap BM: Reactive Updates

Interactive views need to respond when state changes.

Example:

```text
parameter changes
      ↓
equation changes
      ↓
trajectory changes
      ↓
graph changes
      ↓
measurement changes
```

This is effectively a dependency/reactivity system.

## Finding

**G3/G5**

The model already contains dependencies; the runtime needs efficient propagation.

---

# 67. Gap BN: Scheduling

Multiple behaviors may be active:

```text
gravity
collision
decay
user input
measurement
animation
```

The runtime needs to determine:

```text
what runs?
when?
in what order?
at what time?
```

This is particularly difficult when behaviors interact.

## Finding

**G5: Major runtime requirement**

Scheduling should not be confused with model semantics.

---

# 68. Gap BO: Conflicting Behaviors

Suppose two rules both modify:

```text
position
```

or:

```text
temperature
```

The runtime needs conflict semantics.

Possible approaches include:

```text
composition
priority
constraints
simultaneous equations
ordered execution
solver-based resolution
```

## Finding

**G5/G7**

This needs deeper runtime validation.

---

# 69. Gap BP: Causality

Scientific models often contain directional relationships:

```text
force → acceleration
temperature → pressure
reaction → concentration change
```

But equations may describe relationships without establishing causal interpretation.

Therefore the model should avoid assuming:

```text
equation = causal graph
```

## Finding

**G1**

Causality may be metadata or a higher-level modeling concept rather than a universal property of equations.

---

# 70. Gap BQ: Conservation Laws

Physics and chemistry frequently rely on:

```text
energy conservation
momentum conservation
mass conservation
charge conservation
```

These can often be expressed as equations/constraints.

Therefore no `ConservationLaw` primitive is necessarily required.

However, the model may benefit from a semantic classification:

```text
Constraint
    └── conservation relationship
```

## Finding

**G6**

Conservation laws can initially be library-level abstractions built over equations and constraints.

---

# 71. Gap BR: Physical Laws

Likewise:

```text
Newton's laws
Coulomb's law
Ohm's law
ideal gas law
```

do not need to be core primitives.

They can be domain-library definitions.

For example:

```text
law
→ equation(s)
→ constraints
→ applicable domain
```

## Finding

**G6**

---

# 72. Gap BS: Semantic Types

The validation has revealed many kinds of values:

```text
number
quantity
vector
matrix
tensor
point
geometry
function
expression
distribution
set
entity reference
dataset
```

The generalized model therefore needs a richer type system than simply:

```text
Number
```

## Finding

**G2: Core mathematical/type capability**

The type system becomes a major foundation for both the model and eventual DSL.

---

# 73. Gap BT: Genericity

The core must avoid assumptions such as:

```text
Entity.position is always Vector2
```

because:

- some systems are non-spatial
- some are 1D
- some are 3D
- some use abstract spaces

Similarly:

```text
time is always continuous
```

must not be assumed.

## Finding

**G0**

The existing generalized model already explicitly avoids these assumptions.

---

# 74. Gap BU: Model Space vs Render Space

A 3D model may be projected into 2D.

A graph may be laid out in 2D despite having no physical spatial coordinates.

A function domain may be mathematical rather than physical.

Therefore:

```text
model space
```

and:

```text
render space
```

must remain different abstractions.

## Finding

**G0**

This is a foundational invariant.

---

# 75. Gap BV: Representation Semantics

A visual object may represent:

```text
entity
property
relation
field
data
equation
event
state
```

Therefore a representation should have an explicit relationship to what it represents.

For example:

```text
circle
represents particle.position
```

or:

```text
arrow
represents electric_field(x,y)
```

## Finding

**G2: Core projection semantics**

This relationship is central to synchronization and interaction.

---

# 76. Gap BW: Bidirectional Mapping

If a user drags a visual object:

```text
visual position
      ↓
model position
```

while ordinary rendering is:

```text
model position
      ↓
visual position
```

Therefore some projections are bidirectional.

## Finding

**G2**

Projection must support explicitly declared:

```text
model → view
view → model
```

relationships.

Not every representation should be editable.

---

# 77. Gap BX: Selection and Reference

Interactive visualizations require:

```text
select particle
select node
select equation
select region
```

Selection belongs to presentation state, but it needs a reference to model objects.

## Finding

**G1**

Can be handled by identity/reference semantics.

---

# 78. Gap BY: Spatial Queries

Interactive and physical systems need:

```text
nearest object
objects inside region
intersection
distance
collision
neighbors
```

These are computational queries over model state.

## Finding

**G3**

They are runtime/computation capabilities rather than fundamental semantic primitives.

---

# 79. Gap BZ: Querying the Model

More generally:

```text
find entities satisfying condition
```

or:

```text
measure all particles with velocity > v
```

requires query semantics.

## Finding

**G2/G3**

A generic query/filter mechanism is likely fundamental infrastructure.

---

# 80. Gap CA: Event Conditions

Events may be triggered by:

```text
time
state condition
spatial relation
random process
external input
user action
```

The generalized event model supports these conceptually.

However, conditions need a uniform representation.

## Finding

**G1**

Conditions should likely be expressions/predicates over model state.

---

# 81. Gap CB: Predicates and Logic

A condition such as:

```text
velocity > 10
```

or:

```text
distance(A,B) < r
```

or:

```text
node.visited == false
```

is fundamentally a Boolean expression.

Therefore logical expressions need first-class support.

## Finding

**G2**

This extends the mathematical expression system.

---

# 82. Gap CC: State Machines

A traffic light:

```text
RED → GREEN → YELLOW → RED
```

is naturally represented as a state machine.

This can be constructed from:

```text
state
event
condition
transition
```

## Finding

**G6**

A state-machine library can sit on top of core primitives.

---

# 83. Gap CD: Control Systems

A control system contains:

```text
plant
sensor
controller
feedback
actuator
```

Example:

```text
target
 ↓
controller
 ↓
motor
 ↓
position
 ↓
sensor
 └────────→ controller
```

The generalized model already contains:

```text
state
observation
process
interaction
feedback
```

## Finding

**G0/G6**

Control-system abstractions can be built compositionally.

---

# 84. Gap CE: Simulation Semantics

The generalized model currently permits:

```text
continuous
discrete
event-driven
stochastic
hybrid
```

But a runtime needs to determine exactly how these coexist.

For example:

```text
continuous dynamics
+
collision event
+
user input
+
random decay
```

requires a coherent execution semantics.

## Finding

**G5: Major unresolved runtime issue**

This is one of the primary reasons runtime semantics must come before DSL syntax.

---

# 85. Gap CF: Multiple Solvers

A single model may admit:

```text
analytical solution
Euler
Runge-Kutta
Verlet
finite difference
Monte Carlo
custom solver
```

The model should not encode one solver.

Instead:

```text
semantic model
       ↓
execution strategy
```

## Finding

**G3**

---

# 86. Gap CG: Solver Selection

Who chooses the solver?

Possibilities:

```text
author
runtime
automatic selection
library abstraction
```

This is an execution-policy question.

## Finding

**G5/G7**

Needs later runtime design.

---

# 87. Gap CH: Model Validation vs Numerical Validation

There are actually two kinds of correctness.

### Semantic validation

Is the model logically/scientifically well-defined?

### Numerical validation

Does the computation accurately approximate the intended model?

These must not be conflated.

## Finding

**G0**

The architecture should explicitly recognize both.

---

# 88. Gap CI: Visualization Validation

There is also:

```text
model correctness
computation correctness
representation correctness
```

A simulation can be mathematically correct but visually misleading.

Therefore the system should conceptually separate:

```text
model
solver
projection
renderer
```

## Finding

**G0**

---

# 89. Gap CJ: Scale-Free Semantics

The model should not inherently assume:

```text
meters
pixels
seconds
frames
```

These are context-dependent.

A representation can choose:

```text
1 m = 100 px
```

without modifying the model.

## Finding

**G0**

---

# 90. Gap CK: Time as a Dimension

Time appears in:

```text
simulation
data
equations
animation
measurement
events
```

But not every model has time.

Therefore time should remain an optional context.

## Finding

**G0**

---

# 91. Gap CL: Static Relationships

Not every relation evolves.

Example:

```text
triangle has three sides
molecule has atoms
graph has nodes
```

Relations may be:

```text
static
dynamic
derived
conditional
```

## Finding

**G1**

Relation lifecycle semantics need refinement.

---

# 92. Gap CM: Derived Relations

Some relations are inferred.

Example:

```text
A is connected to B
```

may be derived from:

```text
distance(A,B) < threshold
```

This is different from explicitly storing an edge.

## Finding

**G1**

The model needs to distinguish:

```text
stored relation
derived relation
```

---

# 93. Gap CN: Materialized vs Virtual State

Likewise:

```text
electric field
```

may be:

```text
computed on demand
cached
stored as a grid
```

while semantically remaining:

```text
E(x,y)
```

## Finding

**G3**

This is a computational representation issue.

---

# 94. Gap CO: Observational Equivalence

Two different internal models may produce the same observations.

Example:

```text
microscopic decay model
```

and:

```text
macroscopic exponential model
```

may produce nearly identical aggregate curves.

Therefore the system should not assume:

```text
same visualization = same model
```

## Finding

**G0**

This reinforces the model/observation separation.

---

# 95. Gap CP: Model Transformation

One model may be transformed into another:

```text
microscopic
 ↓ aggregation
macroscopic
```

or:

```text
continuous
 ↓ discretization
discrete computational model
```

This suggests a general concept:

```text
Model Transformation
```

But this may be a runtime/library mechanism rather than a core semantic primitive.

## Finding

**G6/G3**

---

# 96. Gap CQ: Model Metadata

Models may need metadata:

```text
name
description
author
domain
units
assumptions
references
version
validity
parameters
```

## Finding

**G6**

Metadata is important but does not need to complicate the semantic core.

---

# 97. Gap CR: Extensibility

The entire project depends on the ability to add:

```text
new domains
new computations
new representations
new interactions
new educational constructs
```

without modifying the core.

Therefore extension boundaries are fundamental.

## Finding

**G2: Architectural requirement**

The core must be extensible by composition and registration rather than hard-coded domain branching.

---

# 98. Consolidated Gap Table

| Gap | Area | Classification | Core? |
|---|---|---|---|
| Collections | State | G1 | Likely |
| Units | Mathematics | G2 | **Yes** |
| Dimensions | Mathematics | G2 | **Yes** |
| Domains | Mathematics | G2 | **Yes** |
| Initial conditions | State | G1 | Existing |
| Boundary conditions | Constraints | G1 | Existing |
| Constraints semantics | Model | G1 | **Yes** |
| Symbolic math | Mathematics | G2 | **Yes** |
| Distributions | Probability | G2 | **Yes** |
| Random processes | Dynamics | G1 | **Yes** |
| Dynamic topology | State | G2 | **Yes** |
| Identity | Model | G2 | **Yes** |
| Dependencies | Runtime/model | G2 | **Yes** |
| Coupled systems | Runtime | G3/G5 | Runtime |
| Discretization | Computation | G3 | Runtime |
| Numerical error | Computation | G3 | Runtime |
| Exact values | Mathematics | G2 | **Yes** |
| Measurement semantics | Observation | G2 | **Yes** |
| Data model | Observation | G2 | **Yes** |
| Multi-scale | Model/runtime | G1/G3 | Mixed |
| Assumptions | Model | G2 | Metadata/semantic |
| Composition | Model | G2 | **Yes** |
| Intervention | Interaction | G2 | **Yes** |
| Provenance | Runtime | G5 | Runtime |
| Replay | Runtime | G5 | Runtime |
| Streaming | Runtime | G3/G5 | Runtime |
| User representations | Presentation | G2 | **Yes** |
| Bidirectional projection | Presentation | G2 | **Yes** |
| Queries | Model/runtime | G2/G3 | **Yes** |
| Predicates | Mathematics | G2 | **Yes** |
| Scheduling | Runtime | G5 | Runtime |
| Conflicting behaviors | Runtime | G5/G7 | Unresolved |
| Solver selection | Runtime | G5/G7 | Unresolved |
| Model transformations | Runtime/library | G3/G6 | Not necessarily |
| Extensibility | Architecture | G2 | **Yes** |

---

# 99. The Core Has Become More Precise

After gap analysis, the generalized system can now be described more precisely.

At its center is not simply:

```text
Entity + Property + Behavior
```

but something closer to:

```text
                         SYSTEM
                            │
             ┌──────────────┼──────────────┐
             ↓              ↓              ↓
           STATE         CONTEXT       SEMANTICS
             │          space/time     mathematics
             │
      ┌──────┼────────┐
      ↓      ↓        ↓
   entities values relations
      │
      └──────── fields / collections
                            │
                            ↓
                        BEHAVIOR
                            │
          ┌─────────┬───────┼─────────┐
          ↓         ↓       ↓         ↓
       function   equation rule     process
                            │
                            ↓
                         EVENTS
                            │
                            ↓
                       EVOLUTION
                            │
                            ↓
                         OBSERVE
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
                            └──────────→ STATE
```

---

# 100. New Candidate Core Layer: Mathematical Type System

The gap analysis strongly suggests that the generalized model needs a formal mathematical/value layer.

Candidate hierarchy:

```text
VALUE
├── Boolean
├── Integer
├── Real
├── Complex
├── Quantity
│   ├── Unit
│   └── Dimension
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
├── EntityReference
└── Dataset
```

This is not yet a final type hierarchy.

It is a validation target.

---

# 101. New Candidate Core Layer: Domain

Candidate:

```text
DOMAIN
├── Continuous
├── Discrete
├── Spatial
├── Temporal
├── Finite
├── Infinite
├── Categorical
└── Topological
```

The same mathematical object can then be interpreted over different domains.

Example:

```text
f : ℝ → ℝ
```

versus:

```text
f : GraphNode → ℝ
```

This may become one of the most important foundations of the DSL.

---

# 102. New Candidate Core Layer: Dependency

A generalized dependency relation may look like:

```text
A
 ↓
B
```

meaning:

```text
B depends on A
```

But dependencies can represent:

```text
value dependency
state dependency
temporal dependency
event dependency
data dependency
representation dependency
```

The runtime can then use the dependency graph for:

- evaluation
- invalidation
- scheduling
- synchronization
- incremental updates

---

# 103. New Candidate Core Layer: Observation/Data

Observation should be treated as a pipeline:

```text
MODEL STATE
     ↓
OBSERVATION
     ↓
MEASUREMENT
     ↓
DATA
     ↓
ANALYSIS
     ↓
DERIVED DATA
     ↓
PROJECTION
```

This allows:

```text
simulation → graph
experiment → histogram
sensor → dashboard
trajectory → Fourier spectrum
```

without requiring separate domain-specific visualization mechanisms.

---

# 104. New Candidate Core Layer: Projection

Projection is becoming one of the most important abstractions.

General form:

```text
SOURCE
   ↓
PROJECTION
   ↓
REPRESENTATION
```

Source may be:

```text
model state
entity
property
field
relation
equation
dataset
derived data
event
```

Projection may define:

```text
what is represented
where
how
with what mapping
whether editable
```

This gives the system a principled foundation for Manim-like visualization without making the scene graph the underlying model.

---

# 105. Core vs Runtime vs Library

The gap analysis now suggests three major boundaries.

## CORE

```text
values
types
dimensions
units
domains
entities
identity
properties
relations
collections
fields
state
functions
expressions
equations
predicates
rules
processes
events
constraints
composition
dependencies
observations
data
projections
```

## RUNTIME

```text
solvers
scheduling
timesteps
event queues
numerical integration
discretization
random execution
parallelism
GPU
caching
incremental updates
replay
snapshots
streaming
resource management
```

## LIBRARIES

```text
physics
chemistry
biology
mathematics
graph algorithms
control systems
statistics
education
animation presets
domain laws
domain entities
specialized solvers
```

This separation is becoming increasingly strong.

---

# 106. The Most Important Unresolved Questions

Before moving to runtime semantics, several questions should receive focused validation.

### Question 1

Is:

```text
Value
```

sufficient as the universal mathematical foundation?

Or do we need a richer algebra/type system?

### Question 2

Are:

```text
Domain
Space
Time
```

three separate concepts?

Or are Space and Time specialized domains?

### Question 3

Can:

```text
Field
Collection
Relation
```

be unified under a broader abstraction?

Or are they fundamentally different?

### Question 4

Can:

```text
Equation
Constraint
Rule
Function
Process
Event
```

be expressed through a smaller semantic foundation?

### Question 5

What exactly constitutes:

```text
State
```

when values are derived, lazy, stochastic, or computationally represented?

### Question 6

What exactly is:

```text
Projection
```

and how does bidirectional interaction work?

### Question 7

How should coupled continuous/discrete/stochastic systems evolve?

### Question 8

How are conflicting behaviors resolved?

### Question 9

How does the runtime choose or combine computational strategies?

### Question 10

What is the minimum core required to support user-defined extensions?

---

# 107. Extended Validation Verdict

The generalized model remains viable.

However, the gap analysis shows that the original model was slightly too coarse in several places.

The biggest newly exposed foundations are:

```text
MATHEMATICAL TYPE SYSTEM
DOMAIN / INDEX SPACE
UNITS / DIMENSIONS
IDENTITY
COLLECTIONS
DEPENDENCIES
COMPOSITION
UNCERTAINTY / DISTRIBUTIONS
OBSERVATION / MEASUREMENT
DATA
PROJECTION
```

Meanwhile, many seemingly complicated capabilities remain safely outside the semantic core:

```text
ODE solvers
PDE solvers
physics engines
Monte Carlo engines
optimization algorithms
GPU execution
rendering optimization
```

This is a healthy result.

It means the abstraction is becoming **more expressive without becoming more domain-specific**.

---

# 108. Revised Architectural Principle

The system should ultimately follow this separation:

```text
WHAT EXISTS?
        ↓
      MODEL
        ↓
WHAT IS TRUE?
        ↓
    SEMANTICS
        ↓
HOW DOES IT CHANGE?
        ↓
    EVOLUTION
        ↓
HOW IS IT CALCULATED?
        ↓
   COMPUTATION
        ↓
WHAT CAN WE OBSERVE?
        ↓
   OBSERVATION
        ↓
HOW DO WE SHOW IT?
        ↓
   PROJECTION
        ↓
 REPRESENTATION
        ↓
HOW DOES THE USER ACT?
        ↓
   INTERACTION
```

And critically:

```text
DOMAIN KNOWLEDGE
```

should plug into this architecture rather than define it.

---

# 109. Decision

The gap analysis does **not** invalidate the generalized model.

Instead, it refines it.

The next artifact should therefore be:

```text
05c Refined Core Model
```

which consolidates the validated findings into a more rigorous core before runtime semantics are designed.

The sequence now becomes:

```text
01 Test Case Catalogue
        ↓
02 Capability / Case Matrix
        ↓
03 Capability Taxonomy
        ↓
04 Generalized Model
        ↓
05 Model Validation
        ↓
05b Gap Analysis
        ↓
05c Refined Core Model
        ↓
06 Runtime Semantics
        ↓
07 DSL Design
```

The key principle remains:

> **Do not design syntax until we know exactly what the language means.**
