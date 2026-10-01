//! Relations across containers (MK-8.5c, D-065): endpoints in collections of contained
//! objects, named by a path (`a in left.atoms`); members, collections and aggregates named
//! the same way (`left.atoms[1]`, `for o in left.atoms`); a member destroyed inside its own
//! container disconnects the relations its container's container holds.

mod common;
use common::*;
use prismal_present::expect::run_case;
use prismal_present::interact::Interactive;
use prismal_runtime::Config;

/// Two cells of two atoms each; one bond joins the first atom of each cell. The bonded atoms
/// start 2 m apart on a bond of rest length 1 m, as the guide's spring pair: their gap is
/// `1 + cos(2 t)` m. Each cell lets go of its first atom at 1.5 s.
const CELLS: &str = "
space Plane = euclidean(2)

model Cells in Plane {
  object Atom {
    param { m: Mass = 1 kg }
    state {
      pos: Point            = origin
      vel: Vector<Velocity> = 0
    }
    flow { der(pos) = vel }
  }
  object Cell {
    param { x0: Length = 0 m }
    parts {
      atoms: Atom[2, max 2] { pos = origin + (x0, (index - 1) * 1 m) }
    }
    event release on at t0 + 1.5 s { destroy atoms[1] }
  }
  relation Bond(a in left.atoms, b in right.atoms) {
    param {
      k:    Quantity<M/T^2> = 2 N/m
      rest: Length          = 1 m
    }
    derived {
      stretch: Length        = |b.pos - a.pos| - rest
      pull:    Vector<Force> = k * stretch * (b.pos - a.pos) / |b.pos - a.pos|
    }
  }
  parts {
    left:  Cell { x0 = -1 m }
    right: Cell { x0 = 1 m }
    bonds: Bond[1, max 2] {
      a = left.atoms[1]
      b = right.atoms[1]
    }
  }
  flow {
    for o in left.atoms  { der(o.vel) = sum(s.pull for s in bonds if s.a == o) / o.m }
    for o in right.atoms { der(o.vel) = -sum(s.pull for s in bonds if s.b == o) / o.m }
  }
  event join on request { connect bonds(left.atoms[2], right.atoms[2]) { rest = 2 m } }
  derived {
    gap:    Length = right.atoms[1].pos.x - left.atoms[1].pos.x
    links:  Real   = count(bonds)
    inside: Real   = count(left.atoms) + count(right.atoms)
  }
}

presentation Lab for Cells {
  view scene: spatial(Plane, scale: 1 m -> 50 px, y: up) {
    for o in left.atoms { marker(o.pos) as l }
    for o in right.atoms { marker(o.pos) as r }
    for s in bonds { segment(s.a.pos, s.b.pos) as bond }
  }
  observe {
    gap    = gap at t0 + 1 s
    before = links at t0 + 1 s
    after  = links at t0 + 2 s
    atoms  = inside at t0 + 2 s
  }
}

run bonded of Cells with Lab {
  until t0 + 2 s
  expect {
    gap    == 0.583853 m within 1e-6 m
    before == 1 within 1e-12
    after  == 0 within 1e-12
    atoms  == 2 within 1e-12
  }
}
";

#[test]
fn bonds_between_atoms_of_two_cells() {
    let prog = program(CELLS);
    for case in &prog.doc.runs {
        let report = run_case(&prog, case).unwrap_or_else(|e| panic!("{}: {e}", case.name));
        for r in &report.results {
            assert!(r.pass, "{} {}: {}", case.name, r.id, r.message);
        }
    }
    // A second bond between the other two atoms, made during the run, and drawn.
    let mut i = Interactive::new(&prog, "Lab", Config::until(1.0)).unwrap_or_else(|d| panic!("{d:?}"));
    i.request("join", None).unwrap();
    let links = i.cm.index["Cells.links"];
    assert_eq!(i.session.current.state_at(0.5)[links].num(), 2.0);
    let bonds: Vec<String> = i.frame().reps().filter_map(|r| r.name.clone()).filter(|n| n.starts_with("bond[")).collect();
    assert_eq!(bonds, ["bond[1]", "bond[2]"]);
}

#[test]
fn paths_print_back() {
    let doc = prismal_syntax::compile(CELLS).unwrap().doc;
    let text = prismal_syntax::format(&doc);
    for line in ["relation Bond(a in left.atoms, b in right.atoms) {", "a = left.atoms[1]", "for o in left.atoms {", "connect bonds(left.atoms[2], right.atoms[2])"] {
        assert!(text.contains(line), "missing `{line}` in\n{text}");
    }
    assert_eq!(prismal_syntax::compile(&text).unwrap().doc.models, doc.models);
    // The IR keeps the path as part identities.
    let bond = doc.models[0].objects.iter().find(|o| o.name == "Bond").unwrap();
    assert_eq!(bond.ends[0].over, "Cells.part.left/Cells.Cell.part.atoms");
}

#[test]
fn paths_go_through_contained_objects_only() {
    let src = CELLS.replace("relation Bond(a in left.atoms, b in right.atoms)", "relation Bond(a in left.atoms.pos, b in right.atoms)");
    let errs = prismal_syntax::compile(&src).err().unwrap_or_default();
    assert!(errs.iter().any(|d| d.code == "SX-E08" && d.message.contains("contained objects only")), "{errs:?}");
}
