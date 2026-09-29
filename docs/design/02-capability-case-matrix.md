# Prismal
## Capability / Case Matrix

> **Purpose:** Identify the capabilities required across the test-case catalogue before designing the generalized model, runtime, renderer, or DSL.
>
> This document answers:
>
> **"What does each case require the system to represent, simulate, visualize, interact with, compute, and export?"**
>
> It does **not** prescribe the final DSL syntax or architecture.

---

# 1. Matrix Legend

| Value | Meaning |
|---|---|
| `✓` | Required |
| `○` | Optional / useful |
| `-` | Not fundamentally required |
| `M` | May require multiple forms simultaneously |
| `C` | Context-dependent |
| `?` | Requires further investigation |
| `N/A` | Not applicable |

---

# 2. Core Capability Dimensions

The matrix is organized around the following dimensions.

## 2.1 Mathematical Nature

| Capability | Description |
|---|---|
| Symbolic | Expressions, equations, identities, symbolic manipulation |
| Numeric | Numerical values and calculations |
| Symbolic + Numeric | Symbolic expressions combined with numerical evaluation |
| Exact | Exact quantities/results |
| Approximate | Approximation permitted or required |
| Scalar | Scalar quantities |
| Vector | Vector quantities |
| Tensor | Tensor/matrix/general multidimensional quantities |
| Algebraic | Algebraic relationships |
| Geometric | Geometric relationships |
| Statistical | Probability distributions, statistics, sampling |
| Logical | Boolean/propositional/logical relationships |

## 2.2 Dynamics

| Capability | Description |
|---|---|
| Static | No simulated evolution |
| Dynamic | State changes over time |
| Continuous-time | State evolves continuously |
| Discrete-time | State advances in discrete steps |
| Event-driven | Changes occur in response to events |
| Deterministic | Same initial state produces same result |
| Stochastic | Randomness is intrinsic |
| Reversible | Evolution can conceptually be reversed |
| Irreversible | Evolution has directionality |
| Equilibrium | System settles into or represents equilibrium |
| Transient | System behavior before equilibrium |
| Feedback | Output influences future system state |
| Emergent | Higher-level behavior emerges from lower-level rules |

## 2.3 Model Structure

| Capability | Description |
|---|---|
| Entity-based | Distinct modeled entities |
| Particle-based | Individual particles/objects |
| Agent-based | Autonomous entities following rules |
| Field-based | Values distributed over space |
| Equation-based | Governing mathematical equations |
| Rule-based | Behavior defined through rules |
| Graph-based | Nodes and edges |
| Network-based | Interconnected entities |
| State-machine | Explicit states and transitions |
| Population-based | Aggregate groups/populations |
| Hybrid | Multiple modeling paradigms combined |
| Hierarchical | Entities/models contain other entities/models |
| Dynamic topology | Entities/relations can appear/disappear/change |

## 2.4 Spatial Structure

| Capability | Description |
|---|---|
| Non-spatial | No spatial coordinates required |
| 1D | One-dimensional spatial model |
| 2D | Two-dimensional spatial model |
| 3D model | Three-dimensional conceptual/model space |
| 3D projection | 3D information represented in the 2D renderer |
| Arbitrary topology | Structure isn't necessarily Euclidean space |
| Coordinate system | Explicit coordinates |
| Multiple coordinate systems | Several coordinate spaces |
| Relative geometry | Relationships rather than absolute coordinates |
| Spatial interaction | Entities interact based on spatial relationships |
| Collision/contact | Physical contact/collision |

## 2.5 Representation

| Capability | Description |
|---|---|
| Geometry | Shapes and geometric objects |
| Graph | Plots, graphs, networks |
| Diagram | Explanatory diagrams |
| Symbolic | Equations, formulas, notation |
| Particle visualization | Individual objects/particles |
| Field visualization | Vectors, contours, gradients, etc. |
| Timeline | Events/evolution over time |
| Table | Tabular data |
| Statistical visualization | Histograms, distributions, etc. |
| Text/annotation | Labels and explanations |
| Camera/view | Viewport into a larger model |
| Multiple views | Several simultaneous representations |
| Synchronized views | Multiple views reflecting the same model state |
| User-defined representation | Author can define a custom projection |

## 2.6 Interaction

| Capability | Description |
|---|---|
| Static viewing | Observe without interaction |
| Play/pause | Control simulation |
| Step | Advance one or more simulation steps |
| Reset | Return to initial state |
| Parameter control | Modify parameters |
| Slider control | Continuous/discrete UI controls |
| Object manipulation | Move/rotate/modify entities |
| Construction | Build a system interactively |
| Measurement | Obtain values from the simulation |
| Probing | Inspect a location/entity/property |
| Experimentation | Run experiments with varying conditions |
| Repetition | Repeat an experiment |
| Random seed control | Reproduce stochastic experiments |
| State save/restore | Store simulation states |
| Branching | Explore alternative states |
| Real-time intervention | Modify system during execution |

## 2.7 Computation

| Capability | Description |
|---|---|
| Closed-form calculation | Direct analytical calculation |
| Numerical evaluation | Evaluate equations numerically |
| Numerical integration | Integrate differential equations |
| Iterative solving | Repeated numerical solution |
| Root finding | Solve equations numerically |
| Linear algebra | Matrix/vector operations |
| Random sampling | Generate random variables |
| Statistical aggregation | Collect/aggregate simulation data |
| Optimization | Search for optimal parameters |
| Constraint solving | Maintain/solve constraints |
| Symbolic manipulation | Transform symbolic expressions |
| Parallel computation | Independent work executed concurrently |
| GPU computation | Large-scale parallel computation |
| Deterministic replay | Reproduce a computation exactly |
| Numerical error handling | Detect/manage numerical instability |

---

# 3. Test Case Matrix: Mathematics

| Test Case | Symbolic | Numeric | Exact | Approx. | Scalar | Vector | Tensor | Geometric | Statistical | Dynamic | Spatial | Interactive |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Arithmetic / numbers | ✓ | ✓ | ✓ | ○ | ✓ | - | - | - | - | - | - | ○ |
| Algebra | ✓ | ✓ | ✓ | ○ | ✓ | ○ | - | ○ | - | - | - | ○ |
| Functions | ✓ | ✓ | ✓ | ✓ | ✓ | ○ | - | ○ | - | ○ | ○ | ○ |
| Coordinate geometry | ✓ | ✓ | ✓ | ○ | ✓ | ✓ | - | ✓ | - | ○ | ✓ | ✓ |
| Euclidean geometry | ✓ | ✓ | ✓ | ○ | ✓ | ✓ | - | ✓ | - | ○ | ✓ | ✓ |
| Vectors | ✓ | ✓ | ✓ | ○ | - | ✓ | ○ | ✓ | - | ○ | ✓ | ✓ |
| Trigonometry | ✓ | ✓ | ✓ | ○ | ✓ | ✓ | - | ✓ | - | ○ | ✓ | ○ |
| Limits | ✓ | ✓ | ✓ | ✓ | ✓ | ○ | - | ○ | - | ○ | ○ | ○ |
| Differentiation | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ○ | ✓ | - | ○ | ✓ | ○ |
| Integration | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ○ | ✓ | - | ○ | ✓ | ○ |
| Differential equations | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ○ | - | ✓ | ✓ | ✓ |
| Linear algebra | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ○ | - | ○ | ○ | ○ |
| Probability | ✓ | ✓ | ✓ | ○ | ✓ | - | - | - | ✓ | ✓ | - | ✓ |
| Statistics | ○ | ✓ | ○ | ✓ | ✓ | ○ | ○ | - | ✓ | ✓ | - | ✓ |

---

# 4. Test Case Matrix: Classical Physics

| Test Case | Spatial | Dynamic | Continuous | Discrete | Deterministic | Stochastic | Equation | Particle/Agent | Field | Numeric | Interactive |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Kinematics | ✓ | ✓ | ✓ | ○ | ✓ | - | ✓ | ○ | - | ✓ | ✓ |
| Projectile motion | ✓ | ✓ | ✓ | ○ | ✓ | - | ✓ | ✓ | - | ✓ | ✓ |
| Newtonian mechanics | ✓ | ✓ | ✓ | ○ | ✓ | ○ | ✓ | ✓ | - | ✓ | ✓ |
| Forces | ✓ | ✓ | ✓ | ○ | ✓ | ○ | ✓ | ✓ | ○ | ✓ | ✓ |
| Collisions | ✓ | ✓ | ✓ | ○ | ✓ | ○ | ✓ | ✓ | - | ✓ | ✓ |
| Conservation of momentum | ✓ | ✓ | ✓ | ○ | ✓ | ○ | ✓ | ✓ | - | ✓ | ✓ |
| Work / energy | ✓ | ✓ | ✓ | ○ | ✓ | ○ | ✓ | ✓ | - | ✓ | ✓ |
| Rigid-body motion | ✓ | ✓ | ✓ | ○ | ✓ | ○ | ✓ | ✓ | - | ✓ | ✓ |
| Rotation / torque | ✓ | ✓ | ✓ | ○ | ✓ | ○ | ✓ | ✓ | - | ✓ | ✓ |
| Pendulum | ✓ | ✓ | ✓ | ○ | ✓ | ○ | ✓ | ✓ | - | ✓ | ✓ |
| Harmonic oscillator | ✓ | ✓ | ✓ | ○ | ✓ | ○ | ✓ | ○ | - | ✓ | ✓ |
| Gravitation | ✓ | ✓ | ✓ | ○ | ✓ | ○ | ✓ | ✓ | ✓ | ✓ | ✓ |

---

# 5. Fields and Electromagnetism

| Test Case | Spatial | Dynamic | Continuous | Discrete | Deterministic | Stochastic | Equation | Particle | Field | Vector | Interactive |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Gravitational field | ✓ | ○ | ✓ | ○ | ✓ | - | ✓ | ○ | ✓ | ✓ | ✓ |
| Electric field | ✓ | ○ | ✓ | ○ | ✓ | - | ✓ | ○ | ✓ | ✓ | ✓ |
| Electric potential | ✓ | ○ | ✓ | ○ | ✓ | - | ✓ | ○ | ✓ | - | ○ |
| Magnetic field | ✓ | ○ | ✓ | ○ | ✓ | - | ✓ | ○ | ✓ | ✓ | ✓ |
| Electric circuits | ✓ | ✓ | ○ | ✓ | ✓ | ○ | ✓ | ✓ | - | ○ | ✓ |
| RC/RL circuits | ✓ | ✓ | ✓ | ○ | ✓ | ○ | ✓ | - | - | ○ | ✓ |
| Electromagnetic induction | ✓ | ✓ | ✓ | ○ | ✓ | ○ | ✓ | ○ | ✓ | ✓ | ✓ |
| Electromagnetic waves | ✓ | ✓ | ✓ | ○ | ✓ | ○ | ✓ | - | ✓ | ✓ | ✓ |

---

# 6. Waves and Optics

| Test Case | Spatial | Dynamic | Continuous | Discrete | Deterministic | Stochastic | Equation | Field | Particle | Symbolic | Interactive |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Mechanical wave | ✓ | ✓ | ✓ | ○ | ✓ | ○ | ✓ | ✓ | ○ | ✓ | ✓ |
| Sound wave | ✓ | ✓ | ✓ | ○ | ✓ | ○ | ✓ | ✓ | ○ | ✓ | ✓ |
| Superposition | ✓ | ✓ | ✓ | ○ | ✓ | - | ✓ | ✓ | ○ | ✓ | ✓ |
| Standing waves | ✓ | ✓ | ✓ | ○ | ✓ | - | ✓ | ✓ | - | ✓ | ✓ |
| Reflection | ✓ | ✓ | ✓ | ○ | ✓ | - | ✓ | ✓ | ○ | ✓ | ✓ |
| Refraction | ✓ | ✓ | ✓ | ○ | ✓ | - | ✓ | ✓ | ○ | ✓ | ✓ |
| Lenses | ✓ | ○ | - | - | ✓ | - | ✓ | - | - | ✓ | ✓ |
| Mirrors | ✓ | ○ | - | - | ✓ | - | ✓ | - | - | ✓ | ✓ |
| Interference | ✓ | ✓ | ✓ | ○ | ✓ | ○ | ✓ | ✓ | ○ | ✓ | ✓ |
| Diffraction | ✓ | ✓ | ✓ | ○ | ✓ | ○ | ✓ | ✓ | ○ | ✓ | ✓ |

---

# 7. Thermodynamics and Statistical Physics

| Test Case | Spatial | Dynamic | Continuous | Discrete | Deterministic | Stochastic | Population | Particle | Statistical | Numeric | Interactive |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Temperature | ○ | ○ | ✓ | ✓ | ✓ | ○ | ✓ | ✓ | ✓ | ✓ | ○ |
| Heat transfer | ✓ | ✓ | ✓ | ○ | ✓ | ○ | ✓ | ○ | ✓ | ✓ | ✓ |
| Gas particles | ✓ | ✓ | ✓ | ○ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Ideal gas | ○ | ○ | ✓ | ○ | ✓ | - | ✓ | ○ | ✓ | ✓ | ✓ |
| Diffusion | ✓ | ✓ | ✓ | ○ | ○ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Brownian motion | ✓ | ✓ | ✓ | ○ | - | ✓ | ○ | ✓ | ✓ | ✓ | ✓ |
| Entropy | ○ | ✓ | ✓ | ✓ | ○ | ✓ | ✓ | ✓ | ✓ | ✓ | ○ |
| Statistical distributions | - | ○ | - | ✓ | - | ✓ | ✓ | - | ✓ | ✓ | ✓ |

---

# 8. Atomic Physics

| Test Case | Spatial | Dynamic | Symbolic | Numeric | Stochastic | Particle | Field | Quantum / Probabilistic | Diagrammatic | Interactive |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Atomic structure | ○ | ○ | ✓ | ✓ | ○ | ✓ | ○ | ✓ | ✓ | ✓ |
| Electron configurations | - | ○ | ✓ | ✓ | - | ✓ | - | ✓ | ✓ | ✓ |
| Energy levels | - | ○ | ✓ | ✓ | ○ | ✓ | - | ✓ | ✓ | ✓ |
| Atomic spectra | ○ | ✓ | ✓ | ✓ | ○ | ○ | - | ✓ | ✓ | ✓ |
| Quantum probability | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Wave-particle concepts | ✓ | ✓ | ✓ | ✓ | ○ | ✓ | ✓ | ✓ | ✓ | ✓ |

---

# 9. Nuclear and Particle Physics

| Test Case | Spatial | Dynamic | Continuous | Discrete | Deterministic | Stochastic | Particle | Population | Event-driven | Statistical | Interactive |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Radioactive decay, microscopic | ✓ | ✓ | - | ✓ | - | ✓ | ✓ | ○ | ✓ | ✓ | ✓ |
| Radioactive decay, macroscopic | ○ | ✓ | ✓ | ○ | ✓ | - | - | ✓ | ○ | ✓ | ✓ |
| Half-life | ○ | ✓ | ✓ | ✓ | ✓ | ✓ | ○ | ✓ | ○ | ✓ | ✓ |
| Alpha decay | ✓ | ✓ | - | ✓ | - | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Beta decay | ✓ | ✓ | - | ✓ | - | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Gamma decay | ✓ | ✓ | - | ✓ | - | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Decay chains | ✓ | ✓ | - | ✓ | - | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Nuclear reactions | ✓ | ✓ | - | ✓ | ○ | ○ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Particle interactions | ✓ | ✓ | - | ✓ | ○ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |

> **Important:** The microscopic and macroscopic radioactive-decay cases are deliberately separate. A single scientific concept can require different computational models at different scales.

---

# 10. Chemistry

| Test Case | Spatial | Dynamic | Continuous | Discrete | Deterministic | Stochastic | Entity-based | Population | Equation | Numeric | Interactive |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Atomic structure | ○ | ○ | - | ✓ | ○ | ○ | ✓ | - | ✓ | ✓ | ✓ |
| Molecular structure | ✓ | ○ | - | ✓ | ✓ | ○ | ✓ | - | ✓ | ✓ | ✓ |
| Molecular geometry | ✓ | ○ | - | ✓ | ✓ | - | ✓ | - | ✓ | ✓ | ✓ |
| Chemical bonding | ✓ | ○ | - | ✓ | ✓ | ○ | ✓ | - | ✓ | ✓ | ✓ |
| Chemical reaction | ✓ | ✓ | ○ | ✓ | ✓ | ○ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Reaction kinetics | ○ | ✓ | ✓ | ✓ | ✓ | ○ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Chemical equilibrium | ○ | ✓ | ✓ | ✓ | ✓ | ○ | - | ✓ | ✓ | ✓ | ✓ |
| Acids and bases | ○ | ✓ | ✓ | ✓ | ✓ | ○ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Electrochemistry | ✓ | ✓ | ✓ | ✓ | ✓ | ○ | ✓ | ✓ | ✓ | ✓ | ✓ |
| States of matter | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Organic chemistry | ✓ | ✓ | ○ | ✓ | ✓ | ○ | ✓ | ✓ | ✓ | ✓ | ✓ |

---

# 11. General Simulation Models

These cases are particularly important because they expose capabilities that are not tied to a scientific domain.

| Case | Entity | Spatial | Continuous | Discrete | Deterministic | Stochastic | Agent | Field | Event-driven | Emergent | Hybrid |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Particle system | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ○ | ✓ | ○ | ✓ | ✓ |
| Agent simulation | ✓ | ✓ | ○ | ✓ | ✓ | ✓ | ✓ | ○ | ✓ | ✓ | ✓ |
| Continuous field | - | ✓ | ✓ | ○ | ✓ | ○ | - | ✓ | ○ | ✓ | ○ |
| Discrete-event system | ✓ | ○ | - | ✓ | ✓ | ✓ | ✓ | - | ✓ | ✓ | ○ |
| Cellular automaton | ✓ | ✓ | - | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ○ |
| Population model | ✓ | ○ | ✓ | ✓ | ✓ | ✓ | ○ | - | ○ | ✓ | ✓ |
| Network model | ✓ | ○ | ○ | ✓ | ✓ | ✓ | ✓ | - | ✓ | ✓ | ✓ |
| Hybrid physical model | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |

---

# 12. Biology / Life Sciences

| Test Case | Spatial | Dynamic | Continuous | Discrete | Deterministic | Stochastic | Entity | Population | Field | Network | Interactive |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Cell structure | ✓ | ○ | - | ✓ | ✓ | ○ | ✓ | - | - | ○ | ✓ |
| Cell processes | ✓ | ✓ | ✓ | ✓ | ○ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Population growth | ○ | ✓ | ✓ | ✓ | ✓ | ✓ | ○ | ✓ | - | - | ✓ |
| Predator-prey | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ○ | - | ✓ |
| Ecosystem model | ✓ | ✓ | ✓ | ✓ | ○ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Genetics / inheritance | ○ | ✓ | - | ✓ | ✓ | ✓ | ✓ | ✓ | - | ✓ | ✓ |

---

# 13. Computer Science

| Test Case | Spatial | Dynamic | Discrete | Deterministic | Stochastic | Graph | Agent | State-machine | Symbolic | Numeric | Interactive |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Sorting algorithms | ○ | ✓ | ✓ | ✓ | - | - | ○ | ✓ | ✓ | ✓ | ✓ |
| Searching | ○ | ✓ | ✓ | ✓ | - | - | ○ | ✓ | ✓ | ✓ | ✓ |
| Trees | ○ | ✓ | ✓ | ✓ | - | ✓ | ○ | ✓ | ✓ | ○ | ✓ |
| Graph algorithms | ○ | ✓ | ✓ | ✓ | ○ | ✓ | ○ | ✓ | ✓ | ✓ | ✓ |
| Path finding | ✓ | ✓ | ✓ | ✓ | ○ | ✓ | ✓ | ○ | ✓ | ✓ | ✓ |
| Automata | ○ | ✓ | ✓ | ✓ | ○ | ✓ | ○ | ✓ | ✓ | - | ✓ |
| Networking | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Operating-system concepts | ○ | ✓ | ✓ | ✓ | ○ | ○ | ✓ | ✓ | ✓ | ✓ | ✓ |

---

# 14. Abstract Mathematics / Conceptual Models

| Test Case | Spatial | Symbolic | Numeric | Geometric | Graph | Logical | Dynamic | Interactive | Multiple Views |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Sets | ○ | ✓ | ○ | ○ | ✓ | ✓ | ○ | ✓ | ✓ |
| Relations | ○ | ✓ | ○ | ○ | ✓ | ✓ | ○ | ✓ | ✓ |
| Functions as mappings | ○ | ✓ | ✓ | ✓ | ✓ | ✓ | ○ | ✓ | ✓ |
| Transformations | ✓ | ✓ | ✓ | ✓ | ○ | - | ✓ | ✓ | ✓ |
| Proof visualization | ○ | ✓ | ○ | ✓ | ○ | ✓ | ○ | ✓ | ✓ |
| Recursion | ○ | ✓ | ✓ | ○ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Fractals | ✓ | ✓ | ✓ | ✓ | ○ | - | ✓ | ✓ | ✓ |
| Dynamical systems | ✓ | ✓ | ✓ | ✓ | ✓ | ○ | ✓ | ✓ | ✓ |

---

# 15. Representation Matrix

The previous sections describe what the model requires. This section describes **how the same model may need to be projected**.

| Representation | Geometry | Graph | Symbolic | Particles | Field | Timeline | Table | Statistics | Diagram | Animation | Interaction |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Spatial scene | ✓ | ○ | ○ | ✓ | ✓ | ○ | ○ | ○ | ✓ | ✓ | ✓ |
| Function graph | ○ | ✓ | ✓ | - | - | ○ | ○ | ○ | ○ | ✓ | ✓ |
| Equation view | - | ○ | ✓ | - | - | ○ | ○ | - | ○ | ○ | ○ |
| Particle view | ✓ | ○ | ○ | ✓ | ○ | ✓ | ○ | ✓ | ○ | ✓ | ✓ |
| Field visualization | ✓ | ○ | ○ | ○ | ✓ | ✓ | ○ | ✓ | ○ | ✓ | ✓ |
| Timeline | - | ○ | ○ | ○ | - | ✓ | ✓ | ✓ | ○ | ✓ | ✓ |
| Statistical view | - | ✓ | ○ | ○ | ○ | ✓ | ✓ | ✓ | ○ | ✓ | ✓ |
| Network graph | ○ | ✓ | ○ | ○ | - | ✓ | ○ | ✓ | ✓ | ✓ | ✓ |
| Schematic diagram | ✓ | ○ | ✓ | ○ | ○ | ○ | ○ | - | ✓ | ○ | ✓ |
| Hybrid dashboard | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |

> A representation is therefore treated as a **projection/view of model state**, rather than as the model itself.

---

# 16. Interaction Capability Matrix

| Interaction | Static Model | Dynamic Model | Stochastic Model | Spatial Model | Graph Model | Symbolic Model | Educational Use |
|---|---:|---:|---:|---:|---:|---:|---:|
| Play / pause | ○ | ✓ | ✓ | ✓ | ✓ | ○ | ✓ |
| Step simulation | ○ | ✓ | ✓ | ✓ | ✓ | ○ | ✓ |
| Reset | ○ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Change parameter | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Slider | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Drag entity | ✓ | ✓ | ✓ | ✓ | ○ | ○ | ✓ |
| Add entity | ✓ | ✓ | ✓ | ✓ | ✓ | ○ | ✓ |
| Remove entity | ✓ | ✓ | ✓ | ✓ | ✓ | ○ | ✓ |
| Measure | ○ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Probe | ○ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Construct experiment | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Repeat experiment | - | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Random seed | - | ○ | ✓ | ○ | ○ | - | ✓ |
| Save state | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Restore state | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Branch state | ○ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Real-time intervention | - | ✓ | ✓ | ✓ | ✓ | ○ | ✓ |

---

# 17. Time / Simulation Matrix

| Case Type | Continuous Time | Discrete Time | Event Driven | Real Time | Accelerated | Slow Motion | Deterministic Replay | Random Seed |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| Static visualization | - | - | - | - | - | - | - | - |
| Kinematics | ✓ | ✓ | ○ | ✓ | ✓ | ✓ | ✓ | - |
| Physics simulation | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ○ |
| Chemical kinetics | ✓ | ✓ | ○ | ✓ | ✓ | ✓ | ✓ | ○ |
| Radioactive decay | ○ | ✓ | ✓ | ✓ | ✓ | ✓ | ○ | ✓ |
| Agent simulation | ○ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Cellular automaton | - | ✓ | ○ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Discrete-event simulation | - | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Statistical simulation | ○ | ✓ | ✓ | ✓ | ✓ | ✓ | ○ | ✓ |

---

# 18. Computation Matrix

| Case | Analytical | Numerical Evaluation | Integration | Iteration | Sampling | Statistics | Linear Algebra | Constraint Solving | Optimization |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Arithmetic | ✓ | ✓ | - | - | - | - | - | - | - |
| Algebra | ✓ | ✓ | - | ○ | - | - | ○ | ✓ | ○ |
| Function plotting | ✓ | ✓ | - | ○ | - | - | - | - | - |
| Kinematics | ✓ | ✓ | ○ | ○ | - | ○ | ✓ | - | ○ |
| Newtonian mechanics | ○ | ✓ | ✓ | ✓ | ○ | ○ | ✓ | ○ | ○ |
| Differential equations | ○ | ✓ | ✓ | ✓ | ○ | ○ | ✓ | ✓ | ○ |
| Probability | ○ | ✓ | - | ✓ | ✓ | ✓ | - | - | ○ |
| Statistics | ○ | ✓ | - | ✓ | ✓ | ✓ | ○ | - | ✓ |
| Particle simulation | - | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ○ | - |
| Field simulation | ○ | ✓ | ✓ | ✓ | ○ | ✓ | ✓ | ✓ | ○ |
| Chemical kinetics | ○ | ✓ | ✓ | ✓ | ○ | ✓ | ✓ | ○ | ○ |
| Radioactive decay | ✓ | ✓ | - | ✓ | ✓ | ✓ | - | - | - |
| Network simulation | ○ | ✓ | - | ✓ | ✓ | ✓ | ✓ | ○ | ○ |

---

# 19. Educational Capability Matrix

| Capability | Mathematics | Physics | Chemistry | Biology | CS | General |
|---|---:|---:|---:|---:|---:|---:|
| Explanatory text | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Labels | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Highlighting | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Guided construction | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Step-by-step progression | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Formula display | ✓ | ✓ | ✓ | ○ | ✓ | ✓ |
| Live value display | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Measurement feedback | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Experiment comparison | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Prediction before simulation | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| User hypothesis | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Automated assessment | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Replay | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Multiple synchronized representations | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |

---

# 20. Output Capability Matrix

| Output | Static | Interactive | Dynamic | Stochastic | Multiple Views | Educational |
|---|---:|---:|---:|---:|---:|---:|
| Interactive desktop scene | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Web/WASM scene | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Image | ✓ | - | ○ | ○ | ✓ | ✓ |
| Animation | ✓ | - | ✓ | ✓ | ✓ | ✓ |
| Video | ✓ | - | ✓ | ✓ | ✓ | ✓ |
| GIF | ✓ | - | ✓ | ✓ | ✓ | ✓ |
| Frame sequence | ✓ | - | ✓ | ✓ | ✓ | ✓ |
| Slide / presentation | ✓ | - | ✓ | ○ | ✓ | ✓ |
| Graph/data output | ✓ | ○ | ✓ | ✓ | ○ | ✓ |
| Experiment dataset | ○ | ○ | ✓ | ✓ | - | ✓ |
| Narrated lesson | ✓ | - | ✓ | ○ | ✓ | ✓ |

---

# 21. Stress-Test Capability Matrix

These cases are intended to expose architectural limits rather than represent particular scientific subjects.

| Stress Case | Required Capability |
|---|---|
| Millions of particles | High-volume entity representation |
| Very large spatial scale | Coordinate abstraction / numerical range |
| Very small spatial scale | Precision / scale abstraction |
| Very long simulation | Efficient state evolution |
| Very fast simulation | Efficient stepping |
| Very slow simulation | Time scaling |
| Chaotic system | Numerical stability / sensitivity |
| Highly stochastic system | Randomness + statistical aggregation |
| Emergent behavior | Large numbers of interacting entities |
| Dynamic entity creation | Runtime entity lifecycle |
| Dynamic entity destruction | Runtime entity lifecycle |
| Dynamic graph topology | Runtime relationship mutation |
| Continuous + discrete hybrid | Multiple time/state semantics |
| Symbolic + numeric hybrid | Shared symbolic/numeric model |
| Exact + approximate hybrid | Precision semantics |
| User-defined equations | Extensible computation model |
| User-defined rules | Extensible behavior model |
| User-defined entities | Extensible semantic model |
| User-defined representation | Extensible projection system |
| User-defined interaction | Extensible interaction system |
| Numerical instability | Diagnostics / error handling |
| Reproducible randomness | Seeded execution |
| Parallel simulation | Parallel computation model |
| GPU-scale simulation | Data-oriented/high-throughput execution |
| Nested simulations | Hierarchical model composition |
| Coupled simulations | Model composition / synchronization |

---

# 22. Cross-Cutting Capability Matrix

The following capabilities appear repeatedly across otherwise unrelated domains.

| Cross-Cutting Capability | Mathematics | Physics | Chemistry | Biology | CS | Importance |
|---|---:|---:|---:|---:|---:|---|
| Entities | ✓ | ✓ | ✓ | ✓ | ✓ | High |
| Properties | ✓ | ✓ | ✓ | ✓ | ✓ | High |
| Relations | ✓ | ✓ | ✓ | ✓ | ✓ | High |
| State | ✓ | ✓ | ✓ | ✓ | ✓ | High |
| State transition | ✓ | ✓ | ✓ | ✓ | ✓ | High |
| Time | ✓ | ✓ | ✓ | ✓ | ✓ | High |
| Spatial position | ✓ | ✓ | ✓ | ✓ | ✓ | High |
| Equations | ✓ | ✓ | ✓ | ✓ | ✓ | High |
| Rules | ✓ | ✓ | ✓ | ✓ | ✓ | High |
| Events | ✓ | ✓ | ✓ | ✓ | ✓ | High |
| Randomness | ✓ | ✓ | ✓ | ✓ | ✓ | High |
| Measurements | ✓ | ✓ | ✓ | ✓ | ✓ | High |
| Parameters | ✓ | ✓ | ✓ | ✓ | ✓ | High |
| Views | ✓ | ✓ | ✓ | ✓ | ✓ | High |
| Animation | ✓ | ✓ | ✓ | ✓ | ✓ | High |
| Interaction | ✓ | ✓ | ✓ | ✓ | ✓ | High |
| Data collection | ✓ | ✓ | ✓ | ✓ | ✓ | High |
| Synchronization | ✓ | ✓ | ✓ | ✓ | ✓ | High |
| Composition | ✓ | ✓ | ✓ | ✓ | ✓ | High |
| Extensibility | ✓ | ✓ | ✓ | ✓ | ✓ | High |

---

# 23. Initial Capability Families Emerging From the Matrix

The matrix suggests that the eventual system will probably need capability families resembling:

```text
MODEL
├── entities
├── properties
├── relations
├── state
├── parameters
├── constraints
└── composition

MATHEMATICS
├── values
├── expressions
├── equations
├── functions
├── vectors
├── matrices/tensors
├── probability
└── statistics

DYNAMICS
├── continuous evolution
├── discrete evolution
├── events
├── rules
├── state transitions
├── feedback
└── stochastic processes

SPACE
├── coordinates
├── geometry
├── topology
├── transforms
├── collision/contact
└── spatial queries

COMPUTATION
├── evaluation
├── integration
├── numerical solving
├── symbolic operations
├── sampling
├── statistics
├── constraints
└── optimization

REPRESENTATION
├── geometry
├── graphs
├── diagrams
├── equations
├── particles
├── fields
├── timelines
├── tables
├── statistics
└── custom projections

INTERACTION
├── controls
├── manipulation
├── construction
├── measurement
├── experimentation
├── branching
└── state management

TIME
├── simulation clock
├── timestep
├── event queue
├── playback
├── time scaling
└── replay

OUTPUT
├── interactive runtime
├── image
├── animation
├── video
├── data
└── presentation

EDUCATION
├── explanation
├── annotation
├── guided activity
├── experiment
├── assessment
└── narration
```

---

# 24. Important Findings

This matrix should **not yet be converted directly into DSL keywords**.

Several important patterns have emerged.

### 24.1 One concept can have multiple models

For example:

```text
Radioactive decay
        │
        ├── microscopic model
        │     └── individual stochastic decay events
        │
        └── macroscopic model
              └── deterministic exponential decay
```

The authoring system should therefore avoid encoding "radioactive decay" as a special primitive.

---

### 24.2 One model can have multiple representations

For example, the same projectile simulation could simultaneously provide:

```text
             ┌── spatial trajectory
             │
Projectile ──┼── velocity vectors
             │
             ├── x(t) graph
             │
             ├── y(t) graph
             │
             ├── equation display
             │
             └── numerical measurements
```

These should ideally remain synchronized because they are views over the same underlying state.

---

### 24.3 Continuous and discrete are not mutually exclusive

A system may contain:

```text
continuous physics
        +
discrete collision event
        +
stochastic event
        +
user interaction
```

Therefore the model must be capable of combining different temporal and behavioral semantics.

---

### 24.4 Deterministic and stochastic are model properties

A stochastic simulation should not simply be considered "an animation with randomness."

Randomness can affect:

- entity creation
- entity destruction
- transitions
- measurements
- parameters
- initial conditions
- event timing
- microscopic behavior

Therefore randomness needs to exist at the model/runtime level.

---

### 24.5 Representation should remain independent of simulation

The same simulation state should potentially feed:

```text
Renderer A → spatial scene
Renderer B → graph
Renderer C → equations
Renderer D → table
Renderer E → diagram
```

This separation is one of the most important architectural implications of the matrix.

---

### 24.6 2D rendering does not imply 2D modeling

A model may represent:

- 3D conceptual space
- atomic structure
- molecular geometry
- a network
- a probability distribution
- an abstract mathematical space

while still being rendered through a 2D system.

Therefore:

```text
model dimensionality ≠ renderer dimensionality
```

---

# 25. What This Matrix Is For

The next step should **not** be immediately writing the DSL.

Instead:

```text
TEST CASE CATALOGUE
        ↓
CAPABILITY / CASE MATRIX       ← THIS DOCUMENT
        ↓
CAPABILITY TAXONOMY
        ↓
GENERALIZED MODEL
        ↓
RUNTIME SEMANTICS
        ↓
REPRESENTATION / PROJECTION MODEL
        ↓
INTERACTION MODEL
        ↓
DSL SURFACE
        ↓
IMPLEMENTATION
```

The **Capability Taxonomy** should now consolidate the recurring capabilities in this matrix into a non-domain-specific set of primitives.

Only after that should we ask:

> "What is the smallest/generalized model that can express all of these capabilities without making the DSL unnecessarily specialized to mathematics, physics, chemistry, etc.?"
