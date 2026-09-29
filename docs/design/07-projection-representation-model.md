# Prismal
## 07 Projection & Representation Model

### 1. Purpose

The core model defines meaning.

Runtime semantics defines how that meaning evolves.

This artifact defines how that meaning becomes **observable, visual, spatial, symbolic, graphical, tabular, diagrammatic, or otherwise presentable**.

The central question is:

> Given model state or observed data, how can the system construct one or more representations that expose selected aspects of that meaning to a viewer?

The fundamental pipeline is:

```text
MODEL / STATE / DATA
          ↓
      PROJECTION
          ↓
   REPRESENTATION
          ↓
        VIEW
          ↓
      PRESENTATION
```

The representation system must remain independent of any particular renderer.

A circle rendered by Vello, SVG, Canvas, WebGPU, or another backend is still semantically the same kind of representation.

---

# 2. Foundational Principle

The system must distinguish:

```text
WHAT SOMETHING IS
```

from:

```text
HOW IT IS REPRESENTED
```

For example:

```text
Model:

ball
position = (3,2)
velocity = (5,1)
mass = 2 kg
```

could be represented as:

```text
spatial circle
+
velocity arrow
+
mass label
+
position coordinates
+
velocity graph
+
energy graph
```

None of these representations is the ball itself.

Therefore:

```text
MODEL ≠ REPRESENTATION
```

and:

```text
REPRESENTATION ≠ RENDERED PIXELS
```

---

# 3. The Representation Stack

The refined representation stack is:

```text
                    SEMANTIC WORLD
                         │
                         ↓
                       SOURCE
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
                  PRESENTATION
                         │
                         ↓
                     RENDERER
                         │
                         ↓
                    OUTPUT
```

Each layer answers a different question.

### Source

What model/data is being observed?

### Projection

Which meaning is selected and how is it mapped?

### Representation

What conceptual/display object expresses that meaning?

### View

How are one or more representations arranged and observed together?

### Presentation

What transient visual state is applied?

### Renderer

How is the presentation converted to pixels, vectors, GPU commands, video frames, etc.?

---

# 4. Source

A projection may consume many types of source.

```text
SOURCE
├── Model
├── State
├── Object
├── Property
├── Relation
├── Collection
├── Field
├── Function
├── Equation
├── Event
├── Observation
├── Dataset
└── Derived Data
```

Examples:

```text
particle.position
temperature(x,y)
graph.edges
f(x)
equation
measurement dataset
decay events
```

A projection does not need to consume the entire model.

It may select a specific aspect.

---

# 5. Selection

Before mapping something into a representation, the system may select a subset.

For example:

```text
all particles
```

may become:

```text
particles where velocity > 10
```

or:

```text
first 100 particles
```

or:

```text
particles inside region R
```

Conceptually:

```text
Source
  ↓
Selection
  ↓
Projection
```

Selection itself may be:

```text
static
dynamic
predicate-based
query-based
spatial
temporal
data-driven
user-controlled
```

---

# 6. Projection

A projection maps semantic information into representational information.

Conceptually:

```text
P : Source → Representation
```

But in practice:

```text
P : Source × Context → Representation
```

because projection may depend on:

```text
coordinate system
scale
viewport
time
parameters
selection
layout
visual encoding
```

Example:

```text
position = (3,2)
```

becomes:

```text
circle.center = screen_transform(3,2)
```

---

# 7. Projection Is Not Necessarily Geometric

This is important.

A projection does not necessarily mean:

```text
world coordinate → screen coordinate
```

It can mean:

```text
temperature → color
velocity → arrow
probability distribution → histogram
equation → mathematical typography
graph topology → nodes + edges
time → horizontal axis
population → bar height
```

Therefore projection is fundamentally:

> **A mapping from semantic information to representational meaning.**

Geometric projection is only one special case.

---

# 8. Representation

A representation is an object that expresses some source meaning.

Examples:

```text
Circle
Line
Arrow
Point
Curve
Polygon
Text
Equation
Axis
Graph
Node
Edge
Histogram
Heatmap
Timeline
Table
Legend
Annotation
```

Representations can themselves contain representations.

For example:

```text
Plot
├── axes
├── grid
├── curve
├── points
├── labels
└── legend
```

Therefore representation is compositional.

---

# 9. Representation Identity

Representations need their own identity.

Suppose:

```text
particle A
```

is represented by:

```text
circle_A
```

and later its position changes.

The runtime should update:

```text
circle_A
```

rather than destroy and recreate an unrelated representation unless the projection semantics require replacement.

Thus:

```text
semantic identity
```

and:

```text
representation identity
```

are distinct.

They may be associated:

```text
particle_A
      ↕
circle_A
```

but are not the same identity.

---

# 10. One Source → Many Representations

A single semantic value may drive multiple representations.

Example:

```text
velocity
   ├──→ arrow direction
   ├──→ arrow length
   ├──→ numeric label
   ├──→ graph point
   └──→ color
```

This means the representation system must allow:

```text
one-to-many projection
```

without duplicating semantic state.

---

# 11. Many Sources → One Representation

The reverse is equally important.

A single representation may depend on multiple sources.

Example:

```text
mass
position
velocity
```

may together determine:

```text
particle representation
```

Or:

```text
temperature(x,y)
boundary
scale
```

may determine:

```text
heatmap
```

Therefore:

```text
many-to-one projection
```

must be supported.

---

# 12. Many-to-Many Mapping

The general case is:

```text
Sources
   ↕
Projection
   ↕
Representations
```

A projection may therefore have:

```text
multiple inputs
multiple outputs
```

This is especially important for composite visualizations.

---

# 13. Direct and Derived Representation

A representation may be generated directly from source state.

Example:

```text
particle.position → circle.center
```

Or through derived computation:

```text
velocity
   ↓
magnitude(velocity)
   ↓
scale
   ↓
arrow.length
```

Or:

```text
temperature field
   ↓
sample grid
   ↓
normalize
   ↓
color mapping
   ↓
heatmap
```

Projection therefore may contain computational steps.

However, these computations are **presentation-oriented transformations**, not automatically changes to the scientific model.

---

# 14. Visual Encoding

A major capability is mapping semantic quantities to visual properties.

Examples:

```text
quantity → position
quantity → size
quantity → length
quantity → angle
quantity → opacity
quantity → color
quantity → stroke width
quantity → shape
quantity → visibility
quantity → label
```

For example:

```text
velocity.magnitude → arrow.length
temperature → heatmap.value
mass → circle.radius
charge → label
```

This is a **visual encoding**.

---

# 15. Visual Encoding Is a Function

Conceptually:

```text
encoding(value) → visual_property
```

Examples:

```text
radius = scale(mass)

opacity = normalize(temperature)

arrow_length = scale(speed)

bar_height = scale(population)
```

The mapping may be:

```text
linear
logarithmic
piecewise
categorical
thresholded
normalized
custom
```

The semantic quantity remains unchanged.

Only its representation changes.

---

# 16. Units and Visual Scaling

A physical quantity may have units:

```text
distance = 20 m
```

while the display may use:

```text
20 m → 200 px
```

This requires a distinction between:

```text
physical scale
```

and:

```text
presentation scale
```

For example:

```text
1 meter = 10 pixels
```

is not a change to the model's unit.

It is a view/projection mapping.

---

# 17. Coordinate Spaces

The system may involve several coordinate spaces:

```text
MODEL SPACE
     ↓
PROJECTION SPACE
     ↓
VIEW SPACE
     ↓
SCREEN SPACE
```

For example:

```text
model:
(3 m, 2 m)

projection:
(30,20)

view:
after camera transform

screen:
(740 px, 380 px)
```

These spaces must not be conflated.

---

# 18. Model Space

Model space belongs to the semantic model.

Example:

```text
position = (3 m, 2 m)
```

Its coordinates have domain-specific meaning.

---

# 19. Projection Space

Projection space is an intermediate representational space.

For example:

```text
Cartesian model
       ↓
logarithmic graph coordinates
```

or:

```text
geographic coordinates
       ↓
map projection
```

Projection space does not necessarily correspond directly to pixels.

---

# 20. View Space

View space incorporates the current view configuration.

Examples:

```text
camera
zoom
pan
rotation
viewport
clipping
```

A camera movement changes view space without changing model space.

---

# 21. Screen / Device Space

The renderer ultimately operates in a device/output space.

Examples:

```text
pixels
SVG coordinates
canvas coordinates
GPU normalized coordinates
video frame coordinates
```

This is renderer/output territory.

---

# 22. Dimensional Projection

A model can exist in dimensions different from its representation.

For example:

```text
3D model
   ↓
2D projection
   ↓
2D representation
```

or:

```text
4D mathematical object
   ↓
2D visualization
```

Therefore:

```text
model dimensionality ≠ representation dimensionality
```

A 2D renderer does not imply a 2D model.

---

# 23. Projection Types

The system should support multiple classes of projection.

```text
PROJECTION
├── Spatial
├── Geometric
├── Graph
├── Symbolic
├── Diagrammatic
├── Statistical
├── Tabular
├── Temporal
├── Field
├── Categorical
└── Composite
```

These are conceptual classes, not necessarily core primitive types.

---

# 24. Spatial Projection

Maps spatial model information to spatial representations.

Example:

```text
particle.position
      ↓
circle.center
```

or:

```text
velocity vector
      ↓
arrow
```

---

# 25. Geometric Projection

Represents mathematical geometry.

Example:

```text
circle equation
      ↓
circle geometry
```

or:

```text
function f(x)
      ↓
curve geometry
```

This is useful for pure mathematics where there may be no evolving model.

---

# 26. Graph Projection

Maps relations into graph representations.

Example:

```text
nodes
edges
weights
```

become:

```text
Node representations
Edge representations
Labels
```

A graph layout may then map abstract topology into spatial coordinates.

The layout itself is a projection/execution concern, not necessarily part of the graph model.

---

# 27. Symbolic Projection

Maps mathematical or logical meaning to symbolic representation.

Examples:

```text
equation
f(x) = x²
```

becomes a typeset mathematical representation.

Or:

```text
derivative(f)
```

becomes:

```text
df/dx
```

The equation exists semantically independently of its typography.

---

# 28. Diagrammatic Projection

A diagram can represent relationships that do not naturally have physical geometry.

Example:

```text
Input
   ↓
Process
   ↓
Output
```

or:

```text
electron
      ↓
energy transition
      ↓
photon
```

The diagram's layout is representational.

---

# 29. Statistical Projection

Data may be projected into:

```text
histogram
scatter plot
box plot
bar chart
density plot
distribution curve
```

For example:

```text
Dataset
   ↓
histogram projection
   ↓
bars
```

The bars are not the dataset itself.

---

# 30. Temporal Projection

Time can be represented through:

```text
timeline
axis
animation
event markers
trajectory
history plot
```

Example:

```text
event.time
      ↓
timeline position
```

Temporal projection can coexist with spatial projection.

---

# 31. Field Projection

Fields require special treatment because they contain values over a domain.

Example:

```text
E(x,y)
```

may become:

```text
vector arrows
contours
streamlines
heatmap
surface
glyphs
```

The same field can therefore have multiple projections.

```text
E(x,y)
 ├──→ arrows
 ├──→ contours
 └──→ heatmap
```

---

# 32. Composite Projection

Complex educational visualizations frequently combine representations.

Example:

```text
projectile model
 ├── spatial trajectory
 ├── velocity vector
 ├── equation
 ├── x-t graph
 ├── y-t graph
 └── numerical readout
```

A composite projection coordinates all of these.

The representations may share dependencies and synchronization.

---

# 33. View

A view is the context in which representations are observed together.

Conceptually:

```text
View
├── representations
├── coordinate systems
├── layout
├── camera
├── visibility
├── selection
└── presentation state
```

A view may contain:

```text
spatial scene
+
graph
+
equation
+
controls
+
measurement display
```

---

# 34. Multiple Views of One Model

The same model may be presented simultaneously through multiple views.

```text                    MODEL
                 ┌────────┼────────┐
                 ↓        ↓        ↓
              Spatial    Graph   Equation
                View      View      View
```

All three remain synchronized because they depend on the same semantic state.

---

# 35. Views Are Not Copies of the Model

A view should not duplicate model semantics.

For example:

```text
View A:
selected_particle = 4
```

does not mean:

```text
particle 4 has a property called selected
```

unless selection is explicitly part of the model.

This keeps presentation state separate.

---

# 36. View State

Typical view state includes:

```text
camera.position
camera.rotation
camera.zoom
viewport
layout
visibility
selection
hover
focus
active_tool
```

These belong to presentation/interaction state.

---

# 37. Representation State

A representation may also have its own state.

For example:

```text
Circle
├── geometry
├── style
├── visibility
├── label
└── interaction metadata
```

Some properties may be projected from the model:

```text
radius ← mass
```

while others are presentation-only:

```text
stroke_width = 2px
```

---

# 38. Static vs Dynamic Projection

A projection can be static:

```text
function f(x)
 ↓
curve
```

or dynamic:

```text
particle.position(t)
 ↓
circle.center(t)
```

A projection may therefore update whenever its source changes.

---

# 39. Projection Lifecycle

A dynamic projection has a lifecycle:

```text
CREATE
   ↓
BIND
   ↓
EVALUATE
   ↓
UPDATE
   ↓
INVALIDATE
   ↓
RECOMPUTE
   ↓
DESTROY
```

A source object being destroyed may therefore cause its associated representation to be removed.

Example:

```text
particle destroyed
      ↓
particle projection invalid
      ↓
circle removed
```

---

# 40. Identity-Preserving Updates

When possible, updates should preserve representation identity.

Example:

```text
particle A
```

continues to correspond to:

```text
circle A
```

even as:

```text
position
velocity
radius
```

change.

This matters for:

- animation continuity
- selection
- interaction
- accessibility
- efficient rendering
- stable references

---

# 41. Dynamic Collections

Suppose:

```text
particles = [A,B,C]
```

becomes:

```text
particles = [A,C,D]
```

The projection must determine:

```text
A → existing representation A
C → existing representation C
B → remove representation B
D → create representation D
```

This is a general **data-to-representation reconciliation** problem.

It should be semantic rather than renderer-specific.

---

# 42. Filtering and Conditional Representation

Representations may be conditional.

Example:

```text
if speed > threshold:
    show velocity arrow
```

or:

```text
show only particles inside region
```

Visibility can therefore depend on semantic state.

This creates:

```text
State
 ↓
predicate
 ↓
projection selection
 ↓
representation
```

---

# 43. Representation Groups

Representations can be grouped.

Example:

```text
ParticleRepresentation
├── body
├── velocity_arrow
├── force_arrow
├── label
└── trail
```

The group can then be:

```text
selected
hidden
animated
transformed
styled
```

as a unit.

---

# 44. Hierarchical Representation

Representations may form a tree:

```text
Scene
├── CoordinateSystem
├── Projectile
│   ├── Body
│   ├── VelocityArrow
│   └── Label
└── Graph
    ├── Axes
    └── Curve
```

This hierarchy is representational.

It does not imply that the scientific model has the same hierarchy.

---

# 45. Layout

A layout determines how representations are arranged.

Examples:

```text
grid
stack
flow
radial
force-directed
timeline
overlay
manual
constraint-based
```

Layouts can operate on:

```text
representation geometry
```

without changing semantic state.

---

# 46. Graph Layout

A graph is particularly important because it has abstract topology but may need spatial representation.

```text
Graph
nodes + edges
       ↓
Layout
       ↓
positions
       ↓
Node / Edge representations
```

The layout-generated position is not necessarily:

```text
node.position
```

in the semantic model.

It may simply be:

```text
representation.position
```

unless the author explicitly makes graph layout part of the model.

---

# 47. Visual Style

Style belongs primarily to presentation.

Examples:

```text
fill
stroke
stroke_width
opacity
font
font_size
shape
line_style
```

Style may be:

```text
static
data-driven
state-driven
interaction-driven
animated
```

For example:

```text
particle.charge
      ↓
color encoding
```

is a semantic-to-presentation mapping.

---

# 48. Color as Encoding

Color should be treated carefully because it can represent:

```text
scalar
category
direction
state
uncertainty
selection
```

Example:

```text
temperature
    ↓
normalized temperature
    ↓
color scale
```

The color scale is representational metadata.

---

# 49. Legends

When a visual encoding is not self-evident, the projection system may generate a legend.

For example:

```text
temperature → color
```

may produce:

```text
Legend
cold ───────── hot
```

The legend is a representation derived from the encoding definition.

---

# 50. Labels

Labels can be projected from:

```text
object identity
property
computed value
equation
metadata
```

Examples:

```text
particle.name
velocity.value
"F = ma"
```

Labels can update automatically as source state changes.

---

# 51. Measurement Representations

Observations may themselves be represented.

Example:

```text
Measurement
   ↓
numeric readout
```

or:

```text
measurement history
   ↓
graph
```

or:

```text
uncertainty
   ↓
error bars
```

Thus observation and representation remain separate:

```text
State
 ↓
Measurement
 ↓
Data
 ↓
Projection
 ↓
Error-bar representation
```

---

# 52. Uncertainty Representation

Scientific data may contain uncertainty.

Representations may encode it through:

```text
error bars
bands
opacity
distribution width
confidence regions
multiple samples
```

The representation should not discard uncertainty silently when the data contains it.

---

# 53. Exact vs Approximate Representation

A representation may display an approximation.

Example:

```text
exact:
π

display:
3.14159
```

or:

```text
exact equation
↓
numerical curve
```

The representation should not imply that the displayed approximation is the underlying exact semantic object.

---

# 54. Representation of Abstract Objects

Not everything has physical geometry.

For example:

```text
algorithm state
graph node
equation
logical proposition
set
function
chemical bond
```

can still have representations.

This is why the representation model cannot be reduced to:

```text
Shape + Position
```

---

# 55. Representation as a General Algebra

A useful abstraction is:

```text
Representation
├── structure
├── attributes
├── children
├── bindings
├── transformations
└── metadata
```

A representation can therefore be constructed, composed, transformed, queried, and updated without assuming a particular rendering backend.

---

# 56. Transformations

Representations can undergo transformations such as:

```text
translate
rotate
scale
reflect
shear
project
clip
warp
```

These are representational transformations unless explicitly bound to model state.

---

# 57. Transformation Hierarchy

A representation hierarchy can have transforms:

```text
Scene
  transform T₁
     ↓
 Group
  transform T₂
     ↓
 Circle
  transform T₃
```

The effective transformation becomes a composition:

```text
T = T₁ ∘ T₂ ∘ T₃
```

This allows hierarchical layouts and animation without changing the underlying model.

---

# 58. Bidirectional Projection

Most projections are:

```text
MODEL → REPRESENTATION
```

But interactive systems require some projections to support:

```text
REPRESENTATION → MODEL
```

For example:

```text
model.position
      ↓
screen position

user drags
      ↓
screen position
      ↓
inverse mapping
      ↓
model.position
```

This must be explicitly declared.

A representation should not automatically become authoritative merely because it can be manipulated.

---

# 59. Inverse Projection

An inverse projection can be conceptualized as:

```text
P⁻¹ : Representation/Input → Model Action
```

It may depend on:

```text
camera
coordinate transforms
constraints
selection
interaction mode
```

Not every projection has an inverse.

For example:

```text
temperature → color
```

is generally many-to-one and cannot uniquely recover temperature.

Therefore:

```text
projection ≠ automatically invertible
```

---

# 60. Lossy Projection

Many projections discard information.

Example:

```text
3D model
 ↓
2D projection
```

or:

```text
distribution
 ↓
histogram
```

or:

```text
vector
 ↓
arrow direction only
```

The projection system should recognize:

```text
information-preserving
```

versus:

```text
information-reducing
```

mappings.

This matters for interaction and interpretation.

---

# 61. Multiple Projections Can Restore Information

A single projection may be lossy, while several together expose more information.

Example:

```text
3D model
 ├── 2D top view
 ├── 2D side view
 └── numeric depth
```

or:

```text
particle
 ├── spatial view
 ├── velocity graph
 └── energy graph
```

The views collectively provide a richer observation of the same model.

---

# 62. Synchronization

Multiple projections of the same model must remain synchronized.

Example:

```text
particle.position changes
        ↓
 ┌──────┼────────┐
 ↓      ↓        ↓
scene  graph    label
```

All affected projections update from the new semantic state.

The runtime's dependency system therefore connects directly to the representation system.

---

# 63. Projection Dependency Graph

Conceptually:

```text
             particle.position
                /     |     \
               ↓      ↓      ↓
          circle    graph   label
             ↓        ↓       ↓
           scene     plot   readout
```

Only affected descendants need recomputation.

---

# 64. Projection Evaluation Order

Some projections depend on other projections.

For example:

```text
data
 ↓
normalization
 ↓
color encoding
 ↓
heatmap
 ↓
legend
```

The system must resolve dependencies before presentation.

This can use the same generalized dependency infrastructure used elsewhere in the runtime.

---

# 65. Projection vs Observation

Projection and observation are related but distinct.

Observation:

```text
State → Information
```

Projection:

```text
Information → Representation
```

For example:

```text
temperature(x,y)
      ↓
observation/sample
      ↓
dataset
      ↓
heatmap projection
```

But a projection can also operate directly on state:

```text
particle.position
      ↓
circle
```

Thus observation is not mandatory for every projection.

---

# 66. Projection vs Computation

Projection may perform computation, but should not silently redefine model semantics.

For example:

```text
velocity
 ↓
magnitude
 ↓
arrow length
```

is representational computation.

Whereas:

```text
velocity
 ↓
force calculation
 ↓
state evolution
```

is model/runtime computation.

The distinction is:

```text
model computation → determines meaning/state
projection computation → determines presentation
```

---

# 67. Projection Parameters

Projections may expose parameters.

Examples:

```text
scale
range
domain
color_map
sampling_resolution
line_width
label_format
projection_angle
```

Changing these should normally affect representation rather than semantic model state.

---

# 68. Representation Configuration

A representation may have configuration:

```text
Circle
├── geometry
├── style
├── bindings
└── metadata
```

For example:

```text
radius ← mass × 2
fill = charge_color
```

The configuration defines how semantic information controls the representation.

---

# 69. Representation Metadata

Representations may carry metadata useful to interaction and accessibility:

```text
source identity
semantic type
description
label
units
interaction capabilities
accessibility description
provenance
```

This allows a viewer or tool to know:

```text
"this circle represents particle A"
```

rather than treating it as an anonymous shape.

---

# 70. Accessibility

Because representations encode meaning, accessibility should be semantic rather than merely visual.

A representation may expose:

```text
textual description
numeric value
semantic role
relationships
state
```

For example:

```text
"Particle A at x=3 m, y=2 m, moving at 5 m/s."
```

This can be generated from the same projection metadata.

Thus:

```text
semantic representation
        ↓
visual rendering
        ↓
auditory/textual rendering
```

can share the same underlying representation model.

---

# 71. Non-Visual Representations

The model should not force every representation to be graphical.

Possible outputs include:

```text
visual
textual
symbolic
auditory
tabular
structured data
```

This makes the projection layer useful beyond video rendering.

---

# 72. Animation and Representation

Animation modifies presentation over presentation time.

For example:

```text
circle.position
```

may be driven by the model:

```text
model.position(t)
```

while:

```text
opacity
```

may be author-driven:

```text
opacity: 0 → 1
```

These remain distinct.

The representation model therefore provides the targets that animation operates upon.

---

# 73. Representation-Time State

A representation may have transient presentation state:

```text
animation_progress
interpolation_factor
hover
highlight
transition_state
```

This belongs to presentation, not semantic model state.

---

# 74. Representation Lifecycle and Animation

When a semantic object is created:

```text
object created
 ↓
projection detects new object
 ↓
representation created
 ↓
optional entrance animation
```

When it is destroyed:

```text
object destroyed
 ↓
representation invalidated
 ↓
optional exit animation
 ↓
representation removed
```

Thus lifecycle animation can be defined without changing semantic creation/destruction semantics.

---

# 75. Render Backend Boundary

The projection/representation model must end before renderer-specific details.

Conceptually:

```text
Representation
      ↓
Presentation
      ↓
Render Backend
```

Possible render backends might include:

```text
raster
vector
GPU
SVG
Canvas
WebGPU
video
image
```

The semantic representation should not depend on one of these.

---

# 76. Output Independence

The same representation may support multiple outputs:

```text
Representation
   ├──→ interactive window
   ├──→ image
   ├──→ animation
   ├──→ video
   ├──→ SVG
   └──→ accessibility representation
```

This is important for the project's original goal of supporting both interactive simulations and recorded educational content.

---

# 77. Model → Multiple Presentation Targets

A single model can therefore feed:

```text              MODEL
                /    |    \
               ↓     ↓     ↓
          Interactive Video  Data
             View           Export
```

The semantic model does not need to know which output is selected.

---

# 78. Representation Composition

Representations should compose recursively.

For example:

```text
Plot
├── Axis
│   ├── ticks
│   └── labels
├── Curve
├── Points
├── Legend
└── Annotation
```

And:

```text
Scene
├── Projectile
│   ├── Body
│   ├── VelocityArrow
│   └── Label
├── CoordinateSystem
└── Measurement
```

This gives the representation layer a tree/graph structure independent of the model graph.

---

# 79. Representation Graph vs Model Graph

These are different graphs.

```text
MODEL GRAPH
A ─── relation ─── B
```

might become:

```text
REPRESENTATION GRAPH
CircleA ─── Line ─── CircleB
```

but the representation graph may contain additional nodes:

```text
labels
arrows
axes
legends
annotations
```

and different relationships.

Therefore:

```text
model topology ≠ representation topology
```

---

# 80. Semantic Binding

A representation can declare bindings such as:

```text
circle.center ← particle.position
circle.radius ← particle.mass
arrow.direction ← particle.velocity
label.text ← particle.name
```

This is a key abstraction.

Bindings connect semantic values to representational properties.

---

# 81. Binding Categories

Bindings can be:

```text
direct
transformed
aggregated
conditional
formatted
sampled
animated
composed
```

Examples:

```text
direct:
x → position.x

transformed:
speed → radius

formatted:
mass → "2.5 kg"

aggregated:
particles → average_velocity

conditional:
energy > threshold → visible
```

---

# 82. Aggregated Representations

A representation may summarize many semantic objects.

Example:

```text
1,000 particles
      ↓
average velocity
      ↓
one arrow
```

or:

```text
population
      ↓
histogram
```

This is fundamentally different from one-to-one object mapping.

The projection model must support aggregation.

---

# 83. Expansion Representations

The reverse is also possible.

One semantic object may produce many representations.

Example:

```text
vector field
      ↓
sample grid
      ↓
1,000 arrows
```

The field itself is one semantic object, while its representation expands into many glyphs.

---

# 84. Projection Cardinality

Therefore projections may support:

```text
1 → 1
1 → many
many → 1
many → many
```

This is a core general capability.

---

# 85. Projection Context

A projection may depend on context:

```text
model
+
time
+
parameters
+
view
+
selection
+
resolution
```

For example, a field representation may use:

```text
sampling_resolution = 20
```

while another view uses:

```text
sampling_resolution = 100
```

Both represent the same field.

---

# 86. Resolution Independence

Semantic state should not depend on display resolution.

For example:

```text
field E(x,y)
```

can be represented at:

```text
10 × 10
50 × 50
500 × 500
```

without changing the underlying field.

The representation determines sampling resolution.

---

# 87. Level of Detail

Large models may require different representation detail.

For example:

```text
zoomed out:
population → one density field

zoomed in:
population → individual particles
```

The representation system may support:

```text
level of detail
```

without requiring the model to change.

---

# 88. Culling

A representation may not be generated/rendered if it cannot affect the current view.

Examples:

```text
outside viewport
below pixel resolution
hidden layer
filtered object
```

Culling is an execution/presentation optimization.

It must not imply that the semantic object no longer exists.

---

# 89. Representation Visibility

Visibility is presentation state.

```text
model object exists
```

can coexist with:

```text
representation.visible = false
```

Likewise:

```text
object not represented
```

does not mean:

```text
object does not exist
```

---

# 90. Selection and Highlighting

Selection belongs to the interactive/presentation boundary.

A selected object may receive:

```text
outline
highlight
label
expanded representation
measurement overlay
```

Selection does not automatically become model state.

This will be formalized further in the Interactive Model artifact.

---

# 91. Representation Semantics for Educational Content

Educational visualization adds another dimension.

A representation can intentionally emphasize:

```text
important variable
causal relationship
equation
step
event
measurement
prediction
```

For example:

```text
force vector
```

may be highlighted while:

```text
velocity vector
```

is faded.

This is presentation intent, not necessarily model semantics.

---

# 92. Explanatory Overlays

An educational view may contain representations that have no corresponding model entity:

```text
arrows
circles
callouts
text
brackets
highlight regions
step numbers
```

These are author-created representations.

Thus not every representation must originate from model state.

---

# 93. Author-Driven Representations

A representation can be explicitly authored:

```text
text("Newton's Second Law")
```

without requiring a model object named:

```text
Newton's Second Law
```

This supports pure explanatory animation and annotation.

---

# 94. Model-Driven and Author-Driven Representation

The system therefore supports:

```text
MODEL-DRIVEN
State → Projection → Representation
```

and:

```text
AUTHOR-DRIVEN
Author intent → Representation
```

and:

```text
HYBRID
Model → Projection → Representation
                  ↑
           Author configuration
```

This is essential for a Manim-like system.

---

# 95. Representation Synchronization with Runtime

After a semantic commit:

```text
Sₙ → Sₙ₊₁
```

the runtime identifies changed dependencies.

Then:

```text
changed state
      ↓
affected projections
      ↓
affected representations
      ↓
presentation update
      ↓
render
```

This is the normal synchronization path.

---

# 96. Projection Consistency

At a committed state:

```text
Sₙ
```

all projections claiming to represent that state should be internally consistent with it.

For example:

```text
ball.position = (3,2)
```

must not simultaneously produce:

```text
scene position = (3,2)
graph position = (8,5)
```

unless those views intentionally use different mappings.

The distinction is:

```text
different representation
≠
inconsistent semantic source
```

---

# 97. Temporal Projection Consistency

For dynamic systems, the projection must identify which simulation time it represents.

For example:

```text
View A → t = 10
View B → t = 20
```

may be intentional.

But if both claim to represent:

```text
t = 10
```

they must derive from the same relevant state.

This becomes important for:

- synchronized graphs
- trails
- playback
- recorded video
- multi-view experiments

---

# 98. Historical Representations

Some representations intentionally show past state.

Example:

```text
particle trail
```

does not represent only the current position.

It may represent:

```text
{position(t₀), position(t₁)..., position(tₙ)}
```

This means projections may consume:

```text
current state
historical state
dataset
experiment history
```

not just current state.

---

# 99. Temporal Windowing

A projection may request:

```text
last 5 seconds
```

or:

```text
entire trajectory
```

or:

```text
events between t=10 and t=20
```

This makes historical visualization a natural part of the projection system.

---

# 100. Representation of Events

Events themselves may be represented.

Example:

```text
collision event
      ↓
marker on trajectory
```

or:

```text
decay event
      ↓
flash + label + timeline marker
```

The event is semantic/runtime information.

The flash is representation/animation.

---

# 101. Representation of Processes

Processes may be represented through:

```text
motion
flow
growth
reaction arrows
progress indicators
animated transformations
```

Again:

```text
process semantics
≠
animation semantics
```

but they can be connected.

---

# 102. Representation of Equations

An equation can have several representations:

```text
symbolic equation
numeric values
graph
geometric interpretation
animated transformation
```

For example:

```text
y = x²
```

can produce:

```text
equation text
+
curve
+
tangent
+
area visualization
```

All can depend on the same mathematical source.

---

# 103. Representation of Functions

A function can be represented as:

```text
graph
table
equation
mapping diagram
3D surface
color field
audio mapping
```

Therefore function representation is not intrinsically "plotting."

---

# 104. Representation of Relations

Relations can be represented as:

```text
edge
arrow
bond
connector
line
table entry
text statement
```

The same semantic relation can therefore have different visual forms.

---

# 105. Representation of Collections

Collections can be represented as:

```text
list
table
scatter plot
particle cloud
histogram
aggregate glyph
graph
```

Again:

```text
Collection ≠ visual list
```

---

# 106. Projection and Model Scale

The same concept can have multiple projections at different scales.

Example:

```text
microscopic decay
particles + random events
```

and:

```text
macroscopic decay
N(t) curve
```

Both can represent related models.

The representation layer must not assume that every entity must appear individually.

---

# 107. Projection Composition

Projections can compose:

```text
Model
 ↓
select particles
 ↓
calculate velocity
 ↓
encode magnitude
 ↓
generate arrows
 ↓
layout
 ↓
group
 ↓
view
```

Thus projection is better understood as a **composable transformation pipeline** than a single function.

---

# 108. Projection Pipeline

General form:

```text
SOURCE
  ↓
SELECT
  ↓
TRANSFORM
  ↓
AGGREGATE / EXPAND
  ↓
ENCODE
  ↓
CONSTRUCT
  ↓
LAYOUT
  ↓
REPRESENTATION
```

Not every projection uses every stage.

---

# 109. Representation Pipeline vs Runtime Pipeline

These pipelines should remain separate.

Runtime:

```text
State
 ↓
Evolution
 ↓
Commit
```

Projection:

```text
Committed State
 ↓
Projection
 ↓
Representation
```

They interact at the state boundary.

---

# 110. Projection Does Not Own Semantic State

A projection may cache:

```text
sampled values
generated geometry
layout positions
formatted labels
```

but these are representational/computational state.

If the projection is destroyed:

```text
model remains intact
```

---

# 111. Representation Does Not Own Scientific Truth

Suppose:

```text
circle.radius = 20px
```

and:

```text
radius ← mass
```

The 20px is not the mass.

The binding explains the relationship.

This distinction prevents accidental semantic corruption.

---

# 112. Presentation Overrides

An author may intentionally override a projection.

Example:

```text
mass → radius
```

but for educational emphasis:

```text
selected particle → radius × 2
```

This should be understood as a presentation override.

It must not alter:

```text
particle.mass
```

---

# 113. Projection Conflicts

Two projections may target the same representation property.

Example:

```text
mass → radius
speed → radius
```

The system must not silently choose one.

Possible semantics:

```text
composition
priority
conflict
derived combination
```

This mirrors the runtime's general change-resolution problem.

---

# 114. Binding Conflicts

Bindings should therefore have explicit resolution semantics.

Possible forms:

```text
replace
compose
add
multiply
conditional
exclusive
```

For example:

```text
radius = base_radius × mass_scale × selection_scale
```

is composition rather than conflicting writes.

---

# 115. Representation Constraints

Representations can have constraints.

Examples:

```text
text must remain inside viewport
label must not overlap
arrow length ≥ minimum_visible_length
graph axis range contains data
```

These are representational constraints.

They should not automatically become scientific constraints.

---

# 116. Layout Constraints

Layout systems may impose:

```text
alignment
spacing
containment
non-overlap
aspect ratio
anchoring
relative position
```

These belong to representation/view semantics.

---

# 117. Camera Semantics

A camera is a view-level mapping.

Conceptually:

```text
View Space
     ↓
Camera Transform
     ↓
Screen Space
```

Camera changes:

```text
pan
zoom
rotate
projection
focus
```

must not change model coordinates.

---

# 118. Camera and Dimensionality

A camera can represent:

```text
2D → 2D
3D → 2D
N-dimensional → lower-dimensional representation
```

The semantic camera abstraction should therefore not be unnecessarily tied to a specific rendering API.

---

# 119. Clipping and Cropping

Views may restrict what is presented:

```text
viewport
clipping region
time range
value range
spatial region
```

This is a presentation operation.

The underlying model remains unchanged.

---

# 120. Representation Update Granularity

The system should support updates at multiple levels:

```text
whole view
whole representation tree
representation group
individual representation
individual property
```

This permits efficient incremental rendering.

---

# 121. Representation Invalidation

A representation becomes invalid when its source dependency changes in a relevant way.

Example:

```text
temperature field changed
      ↓
heatmap invalid
```

But:

```text
equation label
```

may remain valid.

This connects directly to runtime dependency tracking.

---

# 122. Partial Reprojection

The runtime should ideally reproject only affected portions.

For:

```text
1,000,000 particles
```

if:

```text
particle #52 changes
```

it should not semantically require rebuilding every unrelated representation.

The architecture should therefore support fine-grained dependencies.

---

# 123. Representation Caching

Generated representation data may be cached.

Examples:

```text
curve geometry
text layout
glyph geometry
heatmap tiles
graph layout
```

Caches are invalidated when their projection inputs change.

---

# 124. Representation Determinism

Given:

```text
source
projection configuration
view configuration
```

a deterministic projection should produce an equivalent representation.

This matters for:

```text
recording
testing
replay
rendering
cross-platform output
```

---

# 125. Representation and Reproducibility

A reproducible output may therefore depend on:

```text
model state
projection definition
projection configuration
view configuration
render configuration
```

This extends the runtime experiment provenance model.

---

# 126. Semantic vs Presentation Equality

Two representations may look identical but have different semantic bindings.

For example:

```text
two circles at the same position
```

could represent:

```text
particle A
```

and:

```text
particle B
```

Therefore visual equality does not imply semantic equality.

Conversely, one semantic object can have visually different representations.

---

# 127. Representation Equivalence

The system may eventually define equivalence such as:

```text
same semantic source
different representation
```

or:

```text
same visual output
different semantic source
```

These are distinct notions.

This may become useful for testing and accessibility.

---

# 128. Core Representation Model

The resulting conceptual structure is:

```text
REPRESENTATION SYSTEM
├── SOURCE
│   ├── state
│   ├── object
│   ├── property
│   ├── relation
│   ├── field
│   ├── function
│   ├── equation
│   ├── event
│   └── data
│
├── SELECTION
│
├── PROJECTION
│   ├── transformation
│   ├── aggregation
│   ├── expansion
│   ├── visual encoding
│   ├── formatting
│   └── construction
│
├── REPRESENTATION
│   ├── geometry
│   ├── symbolic
│   ├── textual
│   ├── graph
│   ├── statistical
│   ├── temporal
│   ├── diagrammatic
│   └── composite
│
├── VIEW
│   ├── layout
│   ├── camera
│   ├── coordinate systems
│   ├── visibility
│   ├── selection
│   └── presentation state
│
└── OUTPUT
    ├── interactive
    ├── image
    ├── animation
    ├── video
    ├── vector
    ├── data
    └── accessibility
```

---

# 129. Fundamental Relationships

The representation model can now express:

```text
Source
    ↓
Selection
    ↓
Projection
    ↓
Representation
    ↓
View
    ↓
Presentation
    ↓
Renderer
    ↓
Output
```

And:

```text
Source ──depends_on──→ Projection
Projection ──produces──→ Representation
Representation ──belongs_to──→ View
View ──uses──→ Coordinate System
View ──contains──→ Representation
Representation ──binds_to──→ Source
Representation ──contains──→ Representation
Presentation ──modifies──→ Representation
Renderer ──consumes──→ Presentation
```

---

# 130. Representation Invariants

The system should preserve these invariants:

```text
1. Model ≠ Representation

2. Representation ≠ Rendered pixels

3. View ≠ Model

4. Model coordinates ≠ screen coordinates

5. Semantic identity ≠ representation identity

6. One source may have many representations.

7. One representation may depend on many sources.

8. Projections may be lossy.

9. Projections are not automatically invertible.

10. Interaction must explicitly declare when inverse projection is possible.

11. Presentation changes do not automatically modify semantic state.

12. Representation topology need not equal model topology.

13. Model dimensionality need not equal representation dimensionality.

14. Renderer implementation must not define representation semantics.

15. Multiple views may observe the same state.

16. Representation resolution must not redefine model resolution.

17. Visibility does not imply semantic existence.

18. Destruction of a representation does not imply destruction of its source.

19. A representation may exist without a corresponding model object.

20. Projection configuration is distinct from scientific model parameters.
```

---

# 131. The Generalized Projection Equation

The earlier simple formulation:

```text
Projection : Source → Representation
```

is now too small.

A better abstraction is:

```text
Projection(
    source,
    context,
    configuration
)
→
representation
```

where context may include:

```text
time
view
coordinate system
selection
resolution
parameters
history
```

and configuration may define:

```text
selection
transformations
encodings
layout
formatting
style
```

---

# 132. The Complete Presentation Relationship

The system can now be expressed as:

```text
                 MODEL
                   │
                   ↓
                STATE
                   │
             ┌─────┴─────┐
             ↓           ↓
        OBSERVATION    DIRECT
             ↓           │
           DATA          │
             └─────┬─────┘
                   ↓
                SOURCE
                   ↓
               SELECTION
                   ↓
               PROJECTION
                   ↓
             REPRESENTATION
                   ↓
                  VIEW
                   ↓
             PRESENTATION
                   ↓
               RENDERER
                   ↓
                OUTPUT
```

And the interactive reverse path will later become:

```text
USER
 ↓
INPUT
 ↓
INTERACTION
 ↓
REPRESENTATION / VIEW
 ↓
INVERSE PROJECTION
 ↓
MODEL ACTION
```

That final reverse path is deliberately not fully specified here because it belongs to the next artifact.

---

# 133. What This Artifact Establishes

We can now make a stronger statement than before:

> **The visualization system is not a scene graph attached to a simulation.**

Instead:

```text
Semantic Model
      ↓
Runtime
      ↓
Observation / Data
      ↓
Projection System
      ↓
Representation System
      ↓
View System
      ↓
Presentation / Animation
      ↓
Rendering / Output
```

The same semantic model can therefore simultaneously support:

```text
interactive simulation
+
scientific graph
+
equation display
+
diagram
+
measurement panel
+
statistics
+
recorded animation
+
data export
```

without making any one representation the "real" one.

---

# 134. Remaining Question

One major subsystem is now deliberately left open:

> **What happens when a human interacts with these representations?**

For example:

```text
drag this particle
change this slider
click this node
draw a line
measure this distance
select this region
construct a triangle
change the equation parameter
pause the simulation
step one event
branch this experiment
```

The representation model tells us **what the user sees**.

The runtime tells us **how the system executes**.

The next artifact must define:

> **How user actions become semantic operations, runtime controls, observations, interventions, and view changes.**

That is:

# 08 Interactive Model
