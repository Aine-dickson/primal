# 2. Quantities and Units

Physical values in Prismal carry their dimension. A length cannot be added to a time, a velocity must come out as length per time, and every value is stored in coherent SI units whatever unit it was written in. This chapter covers how to write quantities and how dimension errors are reported.

## Writing quantities

A number followed by a unit is a quantity: `120 km`, `1.5 h`, `9.81 m/s^2`, `4 N/m`, `0.01 /m`, `45 deg`.

- Units are written with unit symbols, `*`, `/` and whole powers `^2`. A unit has no spaces inside it: `9.81 m/s^2` is one unit, while `4 / m` divides 4 by a binding named `m`.
- Unit symbols known to the prototype: `m cm mm km`, `s ms min h`, `kg g`, `N J W Pa Hz`, `A K mol cd`, and the angle units `rad deg rev`.
- Unit symbols are recognized only directly after a number. Elsewhere `m`, `s` and `g` are ordinary names, so a model may have a parameter `g` or `m`.
- The value is converted to coherent SI when the program is read: `120 km` is stored as `120000` metres and `1.5 h` as `5400` seconds. Displays and test expectations compare SI values; a test may write its expected value in any unit of the right dimension.

## Types

| Type | Values |
|---|---|
| `Real` | plain numbers (dimensionless) |
| `Length`, `Mass`, `Time`, `Velocity`, `Acceleration`, `Force`, `Energy`, `Power`, `Pressure`, `Frequency`, `Momentum`, `Area`, `Volume`, `Current`, `Amount` | named dimensions |
| `Quantity<L/T^2>` | any dimension, built from the base symbols `L M T I Θ N J` |
| `Angle` | angles; dimensionless, so `rad` is `1`, `deg` is `π/180`, `rev` is `2π` (D-021) |
| `Boolean` | `true`, `false` |

`Quantity<1/L>` is the type of a coefficient per metre, `Quantity<M/T^2>` that of a spring stiffness (N/m).

## Rules

- `+`, `-` and comparisons need operands of the same dimension.
- `*` and `/` combine dimensions: a length divided by a time is a velocity.
- `^` takes a whole number (`speed^2`); `sqrt` takes a square root and halves the dimension (`sqrt(L / g)` is a time).
- `sin`, `cos`, `tan`, `exp`, `log` take dimensionless arguments (an angle is dimensionless).
- A declared type is checked: `speed: Velocity = distance / duration` compiles only if the right side is a length per time.
- A bare number where a quantity is required is an error, with one exception: `0` stands for zero of any dimension (and for the zero vector, chapter 3). Write `5 m`, not `5`.

## A program

```text
model Trip {
  const {
    g: Acceleration = 9.81 m/s^2
  }
  param {
    distance: Length = 120 km   where distance > 0 m
    duration: Time   = 1.5 h    where duration > 0 s
    mass:     Mass   = 1200 kg  where mass > 0 kg
    slope:    Angle  = 3 deg    in [0 deg, 30 deg]   unit deg
  }
  derived {
    speed:  Velocity = distance / duration
    energy: Energy   = 0.5 * mass * speed^2
    climb:  Length   = distance * sin(slope)
    work:   Energy   = mass * g * climb
    power:  Power    = work / duration
  }
}
```

- `const { ... }` declares **constants**. Unlike parameters, a constant can never be changed by a case, a control or a lesson.
- `where distance > 0 m` is a range written as a condition. `in [0 deg, 30 deg]` and `where` both reject a run that starts outside, and both reject a control that tries to leave the range. Note that the bound `0 m` carries its unit: `distance > 0` would also be accepted, since `0` has any dimension.
- `unit deg` is a **display unit** (MK-3.12): `slope` is stored in radians like every angle, but controls, labels and live formulas show it in degrees.

```text
presentation TripPanel for Trip {
  panel inputs {
    slider(distance, range: [10 km, 300 km])
    slider(duration, range: [0.5 h, 5 h])
    slider(slope, range: [0 deg, 30 deg])
  }
  panel results {
    label(speed)
    label(energy)
    label(power)
  }
  observe {
    v = speed  live
    e = energy live
    p = power  live
  }
}
```

Labels and sliders show values in coherent SI units (`speed` as `22.2222 m/s`), except bindings with a display unit (`slope` in degrees).

```cases
run default_trip of Trip with TripPanel {
  expect {
    v == 22.2222222222222 m/s within 1e-9 m/s
    v == 80 km/h within 1e-9 m/s              // the same value, written in km/h
    e == 296296.296296296 J within 1e-6 J
    p == 13691.0861531541 W within 1e-6 W
  }
}

run long_trip of Trip with TripPanel {
  param { distance = 300 km; duration = 2.5 h }
  expect { v == 120 km/h within 1e-9 m/s }
}
```

`v == 80 km/h` shows that the expected value may be written in any unit of the right dimension; the tolerance is also a quantity, `1e-9 m/s`.

## Unit mistakes

Adding quantities of different dimensions:

```error
// error: MK-E01
model Wrong {
  param { d: Length = 1 m; t: Time = 1 s }
  derived { w: Length = d + t }
}
```

The compiler reports `cannot add Quantity<L> and Quantity<T>` at the expression.

A declared type that the definition does not have:

```error
// error: MK-E01
model Wrong {
  param { d: Length = 1 m; t: Time = 1 s }
  derived { v: Velocity = d * t }
}
```

Reported as `found Quantity<L T>, expected Quantity<L T^-1>`: the product is a length times a time.

A bare number where a quantity is required:

```error
// error: MK-E03
model Wrong {
  param { h: Length = 5 }
}
```

A dimensioned argument to a function that needs a pure number:

```error
// error: MK-E02
model Wrong {
  param { d: Length = 1 m }
  derived { a: Real = sin(d) }
}
```

A word that is not a unit symbol ends the number, and what follows is unexpected:

```error
// error: SX-E02
model Wrong {
  param { h: Length = 5 miles }
}
```

## Exercises

1. Add `fuel_rate: Quantity<M/L> = 0.06 kg/km` to `Trip` and a derived `fuel: Mass = fuel_rate * distance`. Check it with a case (`7.2 kg` for the default trip).
2. Write `kinetic: Energy = 0.5 * mass * speed` and read the error. Which dimension does the compiler find?
3. A pendulum's period is `T = 2π sqrt(L / g)`. Write a model with `L: Length` and `g: Acceleration` and a derived `T: Time`. Check that `L = 1 m`, `g = 9.81 m/s^2` gives `2.00607 s` (to `1e-5 s`).

<details>
<summary>A solution to exercise 3</summary>

```text
model Period {
  param {
    L: Length       = 1 m        where L > 0 m
    g: Acceleration = 9.81 m/s^2 where g > 0 m/s^2
  }
  derived {
    T: Time = 2π * sqrt(L / g)
  }
}

presentation PeriodChecks for Period {
  observe { period = T live }
}
```

```cases
run one_metre of Period with PeriodChecks {
  expect { period == 2.00607 s within 1e-5 s }
}
```

`2π` is a product written without `*`: a number directly followed by `π` (or `pi`). Any other name directly after a number is an error.

</details>
