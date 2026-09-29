# Prismal
## 08 Interactive Model

### 1. Purpose

The previous artifacts established:

```text
MODEL
  ↓
RUNTIME
  ↓
OBSERVATION / DATA
  ↓
PROJECTION
  ↓
REPRESENTATION
  ↓
VIEW
```

This describes how the system produces something observable.

But an interactive system introduces another direction:

```text
USER
  ↓
INPUT
  ↓
INTERACTION
  ↓
ACTION
  ↓
MODEL / RUNTIME / VIEW
```

The purpose of this artifact is to define the semantics of that interaction.

The central question is:

> How does an external actor observe, select, manipulate, construct, measure, control, and experiment with a running system without confusing user interaction with model behavior, rendering, or animation?

---

# 2. Foundational Principle

Interaction is not simply:

```text
mouse event → callback
```

It is a semantic process:

```text
INPUT
  ↓
INTERPRETATION
  ↓
INTENT
  ↓
TARGET
  ↓
ACTION
  ↓
EFFECT
```

For example, a pointer movement by itself has no scientific meaning.

```text
mouse moved
```

may become:

```text
user intends to drag particle A
```

which may become:

```text
change particle.position
```

or:

```text
move a measurement probe
```

or:

```text
pan the camera
```

depending on the active interaction mode and target.

Therefore:

```text
INPUT ≠ INTERACTION ≠ ACTION
```

---

# 3. Interaction Architecture

The generalized interaction path is:

```text
             EXTERNAL ACTOR
                    │
                    ↓
                  INPUT
                    │
                    ↓
              INTERPRETATION
                    │
                    ↓
                 INTENT
                    │
                    ↓
                 TARGET
                    │
                    ↓
                OPERATION
                    │
          ┌─────────┼──────────┐
          ↓         ↓          ↓
       MODEL      RUNTIME     VIEW
          │         │          │
          └─────────┼──────────┘
                    ↓
                  EFFECT
                    ↓
              OBSERVATION
                    ↓
             REPRESENTATION
                    ↓
                  VIEWER
```

This closes the interactive loop.

---

# 4. The Interactive Loop

The complete system becomes:

```text
                 ┌──────────────────────────┐
                 │                          │
                 ↓                          │
              MODEL                        │
                 ↓                          │
              RUNTIME                      │
                 ↓                          │
            OBSERVATION                    │
                 ↓                          │
             PROJECTION                    │
                 ↓                          │
          REPRESENTATION                   │
                 ↓                          │
                VIEW                       │
                 ↓                          │
              USER                         │
                 ↓                          │
               INPUT                       │
                 ↓                          │
           INTERACTION                     │
                 ↓                          │
              ACTION ──────────────────────┘
```

This does not mean every interaction modifies the model.

Some interactions modify only:

```text
view
runtime execution
presentation state
```

while others modify semantic state.

---

# 5. Three Interaction Destinations

An interaction may affect one of three major domains.

```text
INTERACTION
├── MODEL ACTION
├── RUNTIME ACTION
└── VIEW ACTION
```

### Model action

Changes semantic state.

```text
drag particle
change mass
add node
delete molecule
```

### Runtime action

Changes execution.

```text
play
pause
step
change simulation speed
reset
branch
change solver configuration
```

### View action

Changes presentation.

```text
zoom
pan
rotate camera
select
hide layer
change layout
```

This distinction is fundamental.

---

# 6. Input

Input is raw external information.

Possible sources include:

```text
INPUT
├── Mouse
├── Keyboard
├── Touch
├── Pen / Stylus
├── Controller
├── Gamepad
├── Microphone
├── Camera
├── Sensor
├── Network
├── File
├── MIDI / external device
└── Programmatic input
```

The core model should not need to know the hardware-specific details.

---

# 7. Input Event

An input event contains information about an external occurrence.

Conceptually:

```text
InputEvent
├── source
├── type
├── timestamp
├── payload
└── device context
```

Examples:

```text
pointer_down
pointer_move
pointer_up
key_down
key_up
touch_start
touch_move
touch_end
```

The event itself does not yet have domain meaning.

---

# 8. Input Coordinates

Pointer input may initially exist in:

```text
DEVICE SPACE
```

while the interaction target exists in:

```text
SCREEN SPACE
VIEW SPACE
PROJECTION SPACE
MODEL SPACE
```

Therefore interaction may require the inverse coordinate chain:

```text
device
  ↓
screen
  ↓
view
  ↓
projection
  ↓
model
```

This connects directly to the inverse projection capability established in Artifact 07.

---

# 9. Interaction Intent

An interaction represents interpreted user intent.

Examples:

```text
select
drag
pan
zoom
rotate
measure
construct
delete
edit
probe
annotate
play
pause
step
reset
change_parameter
start_experiment
```

The same physical input can represent different intents.

For example:

```text
pointer drag
```

may mean:

```text
drag object
pan camera
draw geometry
select region
move measurement probe
```

depending on context.

---

# 10. Interaction Modes

A view or tool may have an active interaction mode.

Examples:

```text
MODE
├── Select
├── Pan
├── Zoom
├── Drag
├── Measure
├── Construct
├── Annotate
├── Probe
├── Inspect
└── Experiment
```

Mode determines how raw input is interpreted.

For example:

```text
SELECT + pointer_down
→ select target
```

while:

```text
MEASURE + pointer_down
→ begin measurement
```

---

# 11. Tools

An interaction mode may be exposed through a tool.

Examples:

```text
SelectionTool
MeasurementTool
ConstructionTool
ProbeTool
DragTool
AnnotationTool
CameraTool
```

Tools are not necessarily core semantic primitives.

They can be composed from:

```text
input
+
interaction state
+
targeting
+
operations
```

---

# 12. Interaction State

Some interactions span multiple input events.

For example:

```text
pointer_down
    ↓
dragging
    ↓
pointer_move
    ↓
pointer_move
    ↓
pointer_up
```

Therefore an interaction can have its own state.

Conceptually:

```text
InteractionState
├── idle
├── active
├── target
├── captured input
├── temporary values
└── completion state
```

---

# 13. Interaction as a State Machine

For example, measurement:

```text
IDLE
 ↓
select first point
 ↓
FIRST_POINT_SELECTED
 ↓
select second point
 ↓
MEASUREMENT_COMPLETE
 ↓
IDLE
```

Construction may be:

```text
IDLE
 ↓
select point A
 ↓
select point B
 ↓
select point C
 ↓
CONSTRUCT OBJECT
 ↓
IDLE
```

Thus interaction state machines are first-class concepts in the generalized model.

---

# 14. Targeting

An interaction normally needs a target.

Possible targets:

```text
TARGET
├── Model Object
├── Property
├── Relation
├── Collection
├── Field
├── Representation
├── View
├── Control
├── Parameter
├── Dataset
└── Runtime
```

Targeting is therefore distinct from input.

---

# 15. Representation Target vs Model Target

Suppose the user clicks a circle.

The immediate target is:

```text
circle representation
```

but the semantic target may be:

```text
particle A
```

The system needs the binding:

```text
circle_A
    ↕
particle_A
```

This allows interaction to cross the representation boundary.

---

# 16. Hit Testing

For graphical interaction, the system may need to determine what was targeted.

Conceptually:

```text
pointer position
      ↓
hit testing
      ↓
candidate representations
      ↓
target selection
```

Hit testing can consider:

```text
geometry
visibility
z-order
interaction priority
selection state
interaction mode
```

The result is a representation target, not automatically a model mutation.

---

# 17. Target Resolution

The complete process can be:

```text
Input
 ↓
Hit Test
 ↓
Representation
 ↓
Semantic Binding
 ↓
Model Object / Property
```

For example:

```text
screen coordinate
       ↓
circle_A
       ↓
binding
       ↓
particle_A.position
```

---

# 18. Target Ambiguity

Multiple representations may occupy the same location.

For example:

```text
particle
+
velocity arrow
+
label
+
measurement marker
```

A click may hit all of them.

The interaction system therefore needs target resolution rules.

Possible mechanisms:

```text
priority
layer
z-order
semantic role
active tool
explicit selection
cycling
```

The exact policy belongs to the interaction system rather than the renderer.

---

# 19. Selection

Selection is a fundamental interaction result.

```text
Input
 ↓
Target
 ↓
Selection
```

Selection can be:

```text
single
multiple
toggle
range
region
query-based
```

Selection usually belongs to presentation/interaction state rather than semantic model state.

---

# 20. Selection vs Model Membership

Important distinction:

```text
selected_objects = [A,B]
```

does not imply:

```text
model contains only A,B
```

and:

```text
A is selected
```

does not necessarily mean:

```text
A.selected = true
```

unless the author explicitly defines selection as model state.

---

# 21. Hover and Focus

Interaction state may include:

```text
hovered_target
focused_target
active_target
selected_targets
```

These are generally transient interaction state.

They can drive representation changes:

```text
hover
 ↓
highlight
```

without modifying semantic state.

---

# 22. Direct Manipulation

Direct manipulation means changing semantic values through a representation.

Examples:

```text
drag point
rotate vector
resize region
move node
adjust control point
```

The general path is:

```text
User Input
 ↓
Representation
 ↓
Inverse Projection
 ↓
Model Operation
 ↓
State Change
```

---

# 23. Dragging

Dragging is not simply:

```text
position = mouse_position
```

because the representation and model may use different spaces.

Instead:

```text
pointer position
 ↓
view → projection inverse
 ↓
model-space position
 ↓
constraints
 ↓
model update
```

---

# 24. Dragging with Constraints

Suppose a point must remain on a circle.

The raw pointer position may be:

```text
p_raw
```

but the valid model position is:

```text
p_valid = project_to_constraint(p_raw)
```

Therefore:

```text
input
 ↓
inverse projection
 ↓
constraint handling
 ↓
model action
```

Constraints can belong to the model while the interaction system supplies the proposed change.

---

# 25. Proposed State vs Committed State

Interactive manipulation introduces an important distinction.

During dragging:

```text
pointer moves
```

may produce a proposed state:

```text
S_proposed
```

before the runtime commits:

```text
S_current → S_new
```

This is useful when:

```text
constraints
validation
preview
expensive computation
```

are involved.

---

# 26. Preview Interaction

Some interactions should preview an operation before committing it.

Examples:

```text
drawing a circle
moving an object
constructing a line
selecting a region
```

Conceptually:

```text
committed state
      ↓
temporary interaction state
      ↓
preview
      ↓
commit / cancel
```

This avoids corrupting the model with incomplete operations.

---

# 27. Transactional Interaction

A multi-step interaction may be treated as a transaction.

Example:

```text
create triangle
```

requires:

```text
point A
point B
point C
```

Only after completion:

```text
CREATE triangle
```

is committed.

Therefore:

```text
interaction transaction
├── begin
├── temporary operations
├── preview
├── commit
└── cancel
```

---

# 28. Undo and Redo

Model-changing interactions naturally connect to history.

For example:

```text
drag A
 ↓
change parameter
 ↓
delete B
```

can produce:

```text
S₀
 ↓
S₁
 ↓
S₂
 ↓
S₃
```

Undo moves backward:

```text
S₃ → S₂
```

Redo moves forward:

```text
S₂ → S₃
```

The runtime history mechanism should record semantic operations or sufficient state information.

---

# 29. Interaction History

An interaction record may contain:

```text
InteractionRecord
├── actor
├── input
├── intent
├── target
├── operation
├── previous state
├── resulting state
├── timestamp
└── metadata
```

This supports:

```text
undo
redo
replay
audit
experiment provenance
```

---

# 30. Parameter Interaction

Parameters are especially important for educational simulations.

Example:

```text
gravity = 9.81
```

may be controlled by:

```text
slider
numeric field
keyboard
programmatic input
```

The interaction path is:

```text
UI control
 ↓
parameter operation
 ↓
model/runtime
 ↓
new state
 ↓
projection update
```

---

# 31. Parameter vs State

A parameter may configure the model without being an ordinary evolving state variable.

For example:

```text
gravity
decay_constant
initial_velocity
temperature
```

may be exposed to the user.

Changing them may require:

```text
recomputation
restart
reset
branch
```

depending on the model's semantics.

---

# 32. Live Parameter Changes

Some parameters can change while simulation runs.

Example:

```text
drag coefficient
```

may be modified during execution.

Then:

```text
S(t)
 +
 parameter change
 →
 S'(t)
```

The runtime semantics must determine when the change takes effect.

Possible boundaries:

```text
immediately
next simulation step
next event
next committed state
restart
```

The interaction system declares the requested operation; runtime semantics determines execution.

---

# 33. Runtime Controls

Some interactions affect execution rather than model state.

Examples:

```text
play
pause
step
stop
reset
seek
change speed
change simulation time
```

These target:

```text
runtime
```

rather than:

```text
semantic model
```

---

# 34. Play and Pause

Pause does not modify scientific state.

It modifies:

```text
execution state
```

For example:

```text
RUNNING
   ↓
pause
   ↓
PAUSED
```

The semantic state remains at the last committed simulation state.

---

# 35. Step

A step requests advancement of execution.

Conceptually:

```text
current state
      ↓
step request
      ↓
runtime evolution
      ↓
next committed state
```

The exact size of the step belongs to runtime semantics.

A user-level "step" therefore does not necessarily mean:

```text
Δt = 1
```

It means:

> Advance according to the runtime's defined stepping semantics.

---

# 36. Seeking

Interactive playback may allow:

```text
seek(t = 10s)
```

This is not necessarily equivalent to running forward from the current state.

The runtime may use:

```text
snapshot
+
replay
+
checkpoint
```

to reconstruct the requested state.

Interaction requests the seek; runtime executes it.

---

# 37. Reset

Reset may mean:

```text
return to initial state
```

but this must be distinguished from:

```text
recreate model
```

A reset may restore:

```text
semantic state
parameters
random state
runtime clock
experiment state
```

according to explicit semantics.

---

# 38. Branching

An interactive experiment may branch:

```text
initial state
       │
       ├── experiment A
       │
       └── experiment B
```

For example:

```text
change gravity → branch A
change mass → branch B
```

Branching should preserve the parent state and produce independent descendants.

---

# 39. Intervention

An intervention is an interaction that intentionally changes the model.

Examples:

```text
add force
remove particle
change temperature
move object
inject molecule
toggle switch
```

Conceptually:

```text
State
 ↓
Intervention
 ↓
State'
```

This differs from observation:

```text
State
 ↓
Observation
 ↓
Data
```

---

# 40. Observation Tools

An interactive system may provide tools that inspect without changing state.

Examples:

```text
probe
measurement
distance tool
angle tool
coordinate readout
field sampler
velocity probe
```

For example:

```text
click particle
 ↓
read position
 ↓
display measurement
```

No semantic mutation is required.

---

# 41. Measurement Interaction

A measurement interaction may itself be multi-stage.

Example:

```text
distance tool
```

becomes:

```text
select A
 ↓
select B
 ↓
calculate distance(A,B)
 ↓
Measurement
 ↓
display result
```

The measurement is observation.

The selections are interaction state.

---

# 42. Instrument Semantics

Some simulations may model measurement instruments explicitly.

For example:

```text
thermometer
```

may have:

```text
range
resolution
uncertainty
sampling rate
```

Then:

```text
model
 ↓
instrument model
 ↓
measurement
 ↓
data
```

This allows educational simulations to demonstrate measurement limitations.

---

# 43. Constructive Interaction

Interaction may create new semantic structures.

Examples:

```text
construct triangle
create graph edge
draw vector
add charge
add particle
create equation
define region
```

The interaction becomes:

```text
User
 ↓
Construction
 ↓
Proposed semantic structure
 ↓
Validation
 ↓
Commit
```

---

# 44. Construction Tools

Construction may be driven by:

```text
points
dragging
clicking
typed values
existing objects
constraints
templates
```

For example:

```text
geometry construction
```

could create:

```text
Point A
Point B
Line(A,B)
```

The resulting objects belong to the model.

---

# 45. Deletion

Deletion is a semantic operation.

For example:

```text
select molecule
 ↓
delete
 ↓
model object removed
```

Deletion may cascade through:

```text
relations
dependent observations
representations
```

The runtime must maintain consistency.

---

# 46. Dynamic Topology

Interactive systems frequently change topology.

Examples:

```text
add node
remove node
create edge
break bond
merge objects
split object
spawn particle
destroy particle
```

Interaction therefore needs operations beyond property assignment.

```text
create
destroy
connect
disconnect
merge
split
```

---

# 47. Interaction with Relations

A user may interact with a relation rather than an object.

Example:

```text
graph edge
```

may be:

```text
selected
weighted
deleted
rerouted
edited
```

Likewise:

```text
chemical bond
```

may be manipulated.

The target model can therefore be:

```text
relation
```

not only:

```text
entity
```

---

# 48. Interaction with Fields

Fields can be interacted with indirectly or directly.

Examples:

```text
move probe through electric field
inject heat
modify boundary
draw source distribution
```

The interaction may target:

```text
field parameters
boundary conditions
source terms
sample location
```

rather than a field value directly.

---

# 49. Region Interaction

A region may be used as:

```text
selection region
measurement region
boundary
construction area
filter
interaction target
```

For example:

```text
drag region
 ↓
select particles inside region
```

The region may itself be model state or presentation state depending on author intent.

---

# 50. User-Defined Interaction

The system must allow authors to define interactions beyond built-in tools.

Conceptually:

```text
interaction
    when condition
    target target
    operation operation
```

The DSL should eventually be able to express custom interactions without requiring every interaction to be implemented in the runtime.

---

# 51. Interaction Guards

An interaction may have conditions.

Example:

```text
allow dragging only if object is movable
```

or:

```text
allow bond creation only if distance < threshold
```

Conceptually:

```text
interaction
    guard predicate
    ↓
    action
```

This reuses the generalized predicate and constraint system.

---

# 52. Permissions

Not every actor must be allowed to perform every operation.

Conceptually:

```text
actor
 ↓
capability check
 ↓
operation
```

This can support:

```text
student mode
teacher mode
author mode
viewer mode
```

without making those roles core domain concepts.

---

# 53. Interaction Context

An interaction may depend on:

```text
active view
active tool
selection
current simulation state
current time
focused object
parameters
experiment branch
```

Thus:

```text
Interaction
    + Context
    → Operation
```

---

# 54. Input Capture

During some interactions, input should remain associated with the active interaction.

Example:

```text
pointer_down on object A
```

then:

```text
pointer_move leaves A
```

should generally continue dragging A.

This requires:

```text
input capture
```

until:

```text
pointer_up
cancel
interaction failure
```

---

# 55. Cancellation

Interactive operations need cancellation semantics.

Examples:

```text
Escape
right click
pointer cancellation
focus loss
invalid state
```

A cancelled interaction should not necessarily commit its temporary changes.

```text
begin
 ↓
temporary state
 ↓
cancel
 ↓
restore previous state
```

---

# 56. Interaction Failure

Failures should be distinguishable.

Examples:

```text
invalid target
invalid operation
constraint violation
unsupported operation
runtime unavailable
computation failure
input failure
```

The system should not silently turn all failures into no-ops.

---

# 57. Constraint Violation During Interaction

Suppose:

```text
x ≥ 0
```

and the user drags an object toward:

```text
x < 0
```

Possible semantics include:

```text
reject
clamp
project onto valid region
allow temporary violation
report violation
```

The core model defines the constraint.

The interaction/runtime configuration determines how the violation is handled.

---

# 58. Continuous Interaction

Some interactions produce a continuous stream of changes.

Example:

```text
slider movement
```

may produce:

```text
parameter:
1.0
1.1
1.2
1.3
...
```

or:

```text
drag:
p₀ → p₁ → p₂ → ...
```

The system must define whether each intermediate value becomes a committed model state.

Possible semantics:

```text
every update
batched updates
preview-only until release
sampled updates
```

---

# 59. Interaction Frequency vs Simulation Frequency

These frequencies may differ.

```text
USER INPUT:       120 Hz
SIMULATION:            60 Hz
RENDERING:             144 Hz
```

Therefore the system cannot assume:

```text
one input = one simulation step = one frame
```

This is a critical separation.

---

# 60. Interaction Scheduling

The runtime may receive:

```text
input events
simulation events
scheduled events
external events
```

These may need ordering.

For example:

```text
simulation reaches t = 2.0
user changes gravity
collision occurs
```

The runtime must establish deterministic semantics for their relative ordering.

---

# 61. Interaction Timestamp

Input may carry:

```text
wall-clock timestamp
```

while the model uses:

```text
simulation time
```

These are not equivalent.

A runtime may need to map:

```text
wall-clock input
        ↓
simulation-time intervention
```

depending on execution mode.

---

# 62. Real-Time Interaction

In real-time simulation:

```text
wall clock ≈ simulation clock
```

A user intervention can therefore be applied approximately at the current simulation time.

In accelerated simulation:

```text
simulation time ≫ wall-clock time
```

input scheduling becomes more complicated.

---

# 63. Deterministic Interaction

For reproducibility, an interaction sequence can become part of experiment history:

```text
initial state
+
parameters
+
random seed
+
interaction sequence
```

can reproduce an experiment.

This makes interaction itself part of provenance.

---

# 64. Interaction Replay

A recorded interaction stream might be:

```text
t=1.0 select A
t=2.2 change gravity=5
t=4.0 drag A
t=8.0 pause
```

Replay can reconstruct the same experiment if:

```text
model
runtime
randomness
inputs
```

are sufficiently controlled.

---

# 65. Interaction vs Animation

These must remain distinct.

Animation:

```text
presentation state
 ↓
temporal transformation
```

Interaction:

```text
external input
 ↓
operation
```

An animation may visually move an object without changing the model.

A drag may change the model and cause the representation to move as a consequence.

---

# 66. Interaction-Driven Animation

The two can be connected.

Example:

```text
user selects particle
 ↓
highlight animation
```

or:

```text
user releases object
 ↓
snap animation
```

The animation is a presentation consequence of the interaction.

---

# 67. Interaction vs Simulation

Likewise:

```text
user changes gravity
 ↓
model parameter changes
 ↓
simulation evolves
```

The interaction initiates a model change.

The resulting trajectory is simulation behavior.

They should not be merged.

---

# 68. Interaction vs Projection

Projection maps:

```text
model → representation
```

Interaction can use:

```text
representation → target
```

and, where supported:

```text
representation → inverse projection → model operation
```

Therefore interaction depends on projection metadata but does not become projection itself.

---

# 69. Bidirectional Binding

A binding can optionally support:

```text
model → representation
```

and:

```text
representation input → model operation
```

Example:

```text
particle.position
      ↕
circle.center
```

But the reverse mapping must be explicit.

Not every binding is writable.

---

# 70. Read-Only vs Writable Bindings

Bindings may be:

```text
read-only
write-only
read-write
```

Examples:

```text
temperature → color
```

is usually:

```text
read-only
```

while:

```text
point.position ↔ draggable point
```

may be:

```text
read-write
```

---

# 71. Writable Projection

A writable representation must define:

```text
input
 ↓
inverse mapping
 ↓
proposed model value
 ↓
validation
 ↓
operation
```

This avoids treating screen-space values as semantic values.

---

# 72. Non-Invertible Encodings

Consider:

```text
temperature → color
```

Many temperatures may produce similar colors.

Therefore:

```text
color → temperature
```

is ambiguous.

The representation may still be interactive if it defines a separate interaction mapping.

For example:

```text
click color scale
 ↓
choose temperature
```

This is not necessarily mathematical inversion.

---

# 73. Interaction Mapping

We therefore generalize:

```text
Input
 ↓
Interaction Mapping
 ↓
Operation
```

rather than requiring every interaction to be:

```text
P⁻¹
```

Inverse projection is one possible mechanism.

---

# 74. Interaction Operations

The core operation vocabulary can include:

```text
SET
MODIFY
CREATE
DESTROY
CONNECT
DISCONNECT
SELECT
MEASURE
OBSERVE
PARAMETER_CHANGE
START
PAUSE
STEP
RESET
SEEK
BRANCH
UNDO
REDO
```

These are semantic categories, not necessarily DSL keywords.

---

# 75. Atomic Operations

Some operations should be atomic.

Example:

```text
create bond(A,B)
```

should not leave the model half-connected if validation fails.

Atomicity becomes especially important for:

```text
topology changes
multi-object creation
constraint-sensitive operations
transactions
```

---

# 76. Compound Operations

A user action may correspond to several operations.

Example:

```text
delete molecule
```

may imply:

```text
destroy molecule
destroy bonds
update collections
invalidate observations
invalidate projections
```

These form one semantic operation from the user's perspective.

---

# 77. Operation Dependencies

Operations may depend on one another.

For example:

```text
create node
 ↓
create edge(node, existing_node)
```

The system must ensure ordering.

This can reuse runtime dependency semantics.

---

# 78. Concurrent Interaction

Multiple actors or input sources may exist.

For example:

```text
user
+
sensor
+
network
```

may all attempt to modify the model.

The system therefore needs a general operation ordering mechanism.

This is not necessarily required for the first implementation, but the semantic model should not make it impossible.

---

# 79. External Inputs

External inputs may be continuously supplied:

```text
sensor temperature
GPS position
microcontroller data
network message
camera tracking
```

These are conceptually similar to user interaction:

```text
external source
 ↓
input
 ↓
operation / observation
```

but they may not represent human intent.

Thus:

```text
interaction ⊂ external influence
```

is useful conceptually, but human interaction deserves its own semantic layer.

---

# 80. Educational Interaction

Educational simulations introduce specialized interaction patterns.

Examples:

```text
predict
experiment
change parameter
measure
construct
classify
match
sequence
answer
explore
```

The underlying mechanisms still reduce to:

```text
input
intent
target
operation
observation
feedback
```

Education-specific semantics can therefore be layered above the interaction core.

---

# 81. Prediction

A prediction interaction may ask:

```text
Where will the projectile land?
```

The user's response can be stored as:

```text
Prediction
├── target
├── predicted value
└── timestamp
```

The simulation can then evolve and compare:

```text
prediction
vs
observed result
```

---

# 82. Experiment Interaction

An educational experiment may expose:

```text
parameters
initial conditions
controls
measurements
```

The learner changes conditions and observes results.

Conceptually:

```text
Experiment
├── Model
├── Controls
├── Interventions
├── Observations
└── Results
```

This extends the Experiment concept from the generalized model.

---

# 83. Guided Interaction

A lesson may constrain available actions:

```text
step 1 → select object
step 2 → change parameter
step 3 → run simulation
step 4 → measure result
```

Guidance should be layered over the general interaction system rather than embedded inside it.

---

# 84. Feedback

Interaction may generate feedback:

```text
action
 ↓
evaluation
 ↓
feedback
```

Examples:

```text
correct
incorrect
valid
invalid
constraint violation
measurement result
```

Feedback itself may be represented visually, textually, or audibly.

---

# 85. Interaction Provenance

A user-driven experiment can record:

```text
actor
action
target
time
parameters
result
```

This enables:

```text
experiment replay
assessment
analysis
debugging
reproducibility
```

---

# 86. Interaction Security / Safety Boundary

The semantic model should distinguish:

```text
requested operation
```

from:

```text
authorized operation
```

and:

```text
valid operation
```

An operation may therefore fail because:

```text
permission denied
constraint violated
invalid target
unsupported
runtime unavailable
```

This is useful even outside educational contexts.

---

# 87. Interaction and State Commit

The runtime ultimately determines when an interaction becomes part of semantic state.

The generalized flow is:

```text
Input
 ↓
Interaction
 ↓
Operation
 ↓
Validation
 ↓
Runtime scheduling
 ↓
State transition
 ↓
Commit
```

This deliberately connects Artifact 08 back to Artifact 06.

---

# 88. Interaction and Observation

After an interaction commits:

```text
Sₙ → Sₙ₊₁
```

observations and projections can react:

```text
Sₙ₊₁
 ├──→ Observation
 └──→ Projection
          ↓
       View update
```

Thus the user sees the consequences of their action through the same observation/projection infrastructure as ordinary simulation evolution.

---

# 89. Complete Interactive Cycle

The complete architecture is now:

```text
                     MODEL
                       ↑
                       │
                  MODEL ACTION
                       ↑
                       │
USER → INPUT → INTENT → TARGET
                       │
                       ↓
                    OPERATION
                       │
               ┌───────┼────────┐
               ↓       ↓        ↓
             MODEL   RUNTIME    VIEW
               │       │        │
               └───────┼────────┘
                       ↓
                     STATE
                       ↓
                 OBSERVATION
                       ↓
                  PROJECTION
                       ↓
                REPRESENTATION
                       ↓
                      VIEW
                       ↓
                     USER
```

This is the complete closed loop.

---

# 90. Core Interactive Model

The refined interactive subsystem is:

```text
INTERACTION SYSTEM
├── INPUT
│   ├── source
│   ├── event
│   ├── timestamp
│   └── payload
│
├── INTERPRETATION
│   ├── intent
│   ├── mode
│   └── context
│
├── TARGETING
│   ├── hit testing
│   ├── target resolution
│   └── semantic binding
│
├── OPERATION
│   ├── model
│   ├── runtime
│   ├── view
│   └── observation
│
├── TRANSACTION
│   ├── begin
│   ├── preview
│   ├── commit
│   └── cancel
│
├── VALIDATION
│   ├── permissions
│   ├── constraints
│   └── operation validity
│
├── HISTORY
│   ├── undo
│   ├── redo
│   ├── replay
│   └── branching
│
└── FEEDBACK
    ├── visual
    ├── textual
    ├── auditory
    └── data
```

---

# 91. Fundamental Relationships

The interactive model now gives us:

```text
Input
    → produces → Interaction Event

Interaction Event
    → interpreted as → Intent

Intent
    → resolves → Target

Target + Intent
    → produces → Operation

Operation
    → validated by → Constraints / Permissions

Operation
    → affects → Model / Runtime / View

Model Operation
    → produces → State Transition

State Transition
    → produces → Observation / Projection Updates

Projection
    → updates → Representation

Representation
    → provides → User Feedback
```

---

# 92. Core Invariants

The interactive model should preserve:

```text
1. Input ≠ Interaction.

2. Interaction ≠ Operation.

3. Representation ≠ Model.

4. Selecting a representation does not automatically mutate the model.

5. A model-changing interaction must explicitly identify its semantic target.

6. Not every representation is writable.

7. Not every projection is invertible.

8. View manipulation does not automatically modify model state.

9. Runtime control does not automatically modify semantic state.

10. Interaction frequency ≠ simulation frequency.

11. Input time ≠ simulation time.

12. Temporary interaction state ≠ committed model state.

13. Cancelled interactions do not necessarily commit state changes.

14. Model constraints remain model semantics.

15. Interaction determines proposed operations; runtime determines their execution/commit semantics.

16. User interaction can be recorded as experiment provenance.

17. Interaction can initiate simulation but is not itself simulation.

18. Interaction can trigger animation but is not animation.

19. External input need not represent human interaction.

20. Domain-specific tools should be composable from the generalized interaction system.
```

---

# 93. Relationship to the Previous Artifacts

The three semantic systems now fit together:

```text
              MODEL
                │
                ↓
          RUNTIME SEMANTICS
                │
                ↓
             STATE
                │
          ┌─────┴─────┐
          ↓           ↓
    OBSERVATION    PROJECTION
          ↓           ↓
         DATA    REPRESENTATION
          │           ↓
          └──────→   VIEW
                       │
                       ↓
                      USER
                       │
                       ↓
                  INTERACTION
                       │
                       ↓
                    ACTION
                       │
                       └────→ MODEL / RUNTIME / VIEW
```

This gives the system a genuine closed semantic loop.

---

# 94. The Emerging Complete Architecture

Combining Artifacts 04-08:

```text
                         DOMAIN LIBRARIES
                               │
                               ↓
                         ┌─────────────┐
                         │    MODEL    │
                         │             │
                         │ values      │
                         │ objects     │
                         │ relations   │
                         │ fields      │
                         │ state       │
                         │ behavior    │
                         │ constraints │
                         └──────┬──────┘
                                │
                                ↓
                         ┌─────────────┐
                         │   RUNTIME   │
                         │             │
                         │ evolution   │
                         │ events      │
                         │ solvers     │
                         │ scheduling  │
                         │ randomness  │
                         │ history     │
                         └──────┬──────┘
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
                             USER
                                │
                                ↓
                           INTERACTION
                                │
                    ┌───────────┼───────────┐
                    ↓           ↓           ↓
                  MODEL       RUNTIME      VIEW
```

---

# 95. What We Have Now

Before DSL design, we have established four major semantic questions.

### 1. What exists?

```text
Model
```

### 2. How does it evolve?

```text
Runtime
```

### 3. How is it represented?

```text
Projection / Representation / View
```

### 4. How can an external actor participate?

```text
Interaction
```

These form the semantic foundation:

```text
MODEL
  ↕
RUNTIME
  ↕
OBSERVATION
  ↕
PROJECTION
  ↕
REPRESENTATION
  ↕
VIEW
  ↕
INTERACTION
  ↕
MODEL
```

---

# 96. Important Remaining Abstraction Boundary

There is now one final major question before DSL design:

> What should the author actually have to express, and what should the system infer?

For example, should an author explicitly define:

```text
projection
binding
hit testing
inverse mapping
interaction state machine
```

or should some of these be inferred?

Consider:

```text
point.position
```

and:

```text
draggable point
```

Should the system automatically infer:

```text
pointer
 ↓
inverse projection
 ↓
position
```

Or should the author explicitly specify the mapping?

Likewise:

```text
particles → circles
```

Should creation/destruction of circles automatically follow collection membership?

These are **DSL design questions**, but we should not answer them by intuition.

The semantic artifacts have now given us the space in which those decisions can be made.

---

# 97. Final Semantic Boundary Before DSL

The system can now be understood as five cooperating semantic layers:

```text
┌─────────────────────────────────────────────┐
│                 EDUCATION                   │
│ lessons / activities / assessment          │
├─────────────────────────────────────────────┤
│                INTERACTION                  │
│ input / intent / target / operations        │
├─────────────────────────────────────────────┤
│              PRESENTATION                   │
│ projection / representation / view          │
├─────────────────────────────────────────────┤
│                 OBSERVATION                 │
│ measurement / data / analysis               │
├─────────────────────────────────────────────┤
│                  RUNTIME                    │
│ evolution / events / scheduling / compute  │
├─────────────────────────────────────────────┤
│                   MODEL                     │
│ state / objects / relations / behavior     │
└─────────────────────────────────────────────┘
```

The DSL will not necessarily mirror these layers syntactically.

That is an important distinction.

The DSL's job is to give authors a **coherent language for expressing the semantics**, not to expose the internal architecture one-to-one.

---

# 98. Transition to DSL Design

We are now ready for:

```text
09 DSL Design
```

But DSL design should begin from a requirements question:

> Given everything established in Artifacts 01-08, what language constructs are actually necessary to express the full case catalogue?

The process should therefore be:

```text
CASE CATALOGUE
      ↓
SEMANTIC REQUIREMENTS
      ↓
LANGUAGE REQUIREMENTS
      ↓
CONSTRUCT INVENTORY
      ↓
SYNTAX OPTIONS
      ↓
SEMANTIC MAPPING
      ↓
TYPE SYSTEM
      ↓
SCOPING / BINDING
      ↓
COMPOSITION
      ↓
ERROR MODEL
      ↓
EXTENSION MODEL
      ↓
EXAMPLE PROGRAMS
      ↓
DSL VALIDATION
```

Only then should we settle the concrete syntax.

The objective is not to make the DSL look elegant first.

The objective is:

> **A language whose expressiveness is justified by the cases and semantics we have already derived.**
