//! Every program of the guide (docs/guide) compiles and every case in it passes; every
//! example of a mistake is reported with the diagnostic it names.

use prismal_web::examples::split_guide;
use prismal_web::Player;
use std::path::PathBuf;

fn chapters() -> Vec<(String, String)> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../docs/guide");
    let mut out: Vec<(String, String)> = std::fs::read_dir(&dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "md"))
        .map(|p| (p.file_name().unwrap().to_string_lossy().into_owned(), std::fs::read_to_string(&p).unwrap()))
        .collect();
    out.sort();
    out
}

#[test]
fn guide_programs_run() {
    let mut programs = 0;
    let mut expectations = 0;
    let mut failures = vec![];
    for (file, md) in chapters() {
        let (progs, errors) = split_guide(&md);
        for p in progs {
            programs += 1;
            let player = match Player::load(&p.source) {
                Ok(pl) => pl,
                Err(d) => {
                    failures.push(format!("{file} {}: does not compile: {d}", p.title));
                    continue;
                }
            };
            for case in player.run_cases().as_array().unwrap() {
                if let Some(e) = case.get("error") {
                    failures.push(format!("{file} {} {}: {e}", p.title, case["case"]));
                }
                for r in case["results"].as_array().into_iter().flatten() {
                    expectations += 1;
                    if r["pass"] != true {
                        failures.push(format!("{file} {} {} {}: {}", p.title, case["case"], r["id"], r["message"]));
                    }
                }
            }
            // Every presentation with views opens.
            let mut player = player;
            for pres in player.catalogue()["presentations"].as_array().unwrap().clone() {
                if pres["kind"] != "observations" {
                    if let Err(d) = player.open(pres["name"].as_str().unwrap(), false) {
                        failures.push(format!("{file} {}: presentation {} does not open: {d}", p.title, pres["name"]));
                    }
                }
            }
        }
        for e in errors {
            match Player::load(&e.source) {
                Ok(_) => failures.push(format!("{file}: error example expected {} but compiled:\n{}", e.code, e.source)),
                Err(d) => {
                    let codes: Vec<&str> = d.as_array().unwrap().iter().filter_map(|x| x["code"].as_str()).collect();
                    if !codes.contains(&e.code.as_str()) {
                        failures.push(format!("{file}: error example expected {}, got {codes:?}: {d}", e.code));
                    }
                }
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
    println!("{programs} programs, {expectations} expectations");
}

/// Chapter 7: dragging the cannon's landing marker to 30 m sets the speed whose reach is
/// 30 m, `sqrt(30 m * g / sin 90°)`.
#[test]
fn cannon_landing_drag() {
    let ex = prismal_web::examples::all().into_iter().find(|e| e.key == "g7-cannon").expect("the guide's cannon");
    let mut p = Player::load(&ex.source).unwrap();
    p.open("CannonLab", false).unwrap();
    assert_eq!(p.pointer_down("landing", None)["ok"], true);
    assert_eq!(p.pointer_move(30.0, 0.0)["ok"], true);
    assert_eq!(p.pointer_up()["committed"], true);
    let r: f64 = p.observations()["R"][0].as_str().unwrap().trim_end_matches(" m").parse().unwrap();
    assert!((r - 30.0).abs() < 1e-3, "{r}");
    let log = p.observations()["log"][0].as_str().unwrap().to_string();
    assert!(log.contains("set v = 17.1552 m/s"), "{log}");
}

/// A session runs as long as the presentation's longest time axis (the tank plots 400 s).
#[test]
fn session_horizon_follows_time_axes() {
    let ex = prismal_web::examples::all().into_iter().find(|e| e.key == "g5-tank").unwrap();
    let mut p = Player::load(&ex.source).unwrap();
    let layout = p.open("TankLab", false).unwrap();
    assert_eq!(layout["session"]["end"], 400.0);
}
