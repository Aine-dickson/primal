//! Page layouts (PK-7.4a, D-063): `layout row(...)` and `column(...)` reach the host's
//! layout as a page tree, views left out follow it, the SVG renderer places views side by
//! side or one below the other, mistakes are diagnostics, and printing keeps the layout.

use prismal_host::{Content, Document, Instance};
use prismal_ir::present::Layout;
use prismal_svg::{render, Options};
use serde_json::{json, Value as Json};

const MODEL: &str = "
model Cannon {
  param {
    speed: Velocity = 20 m/s
    angle: Angle    = 45 deg
  }
  derived { reach: Length = speed^2 * sin(2 * angle) / (9.81 m/s^2) }
}
";

fn src(layout: &str) -> String {
    format!(
        "{MODEL}
presentation Page for Cannon {{
  view flight: plot(x: [0, 60], y: [0, 25]) {{ marker(at: (reach / (1 m), 0)) }}
  panel controls {{ slider(angle, range: [5 deg, 85 deg]) }}
  panel readout {{ label(reach) }}
  {layout}
}}
"
    )
}

fn open(src: &str) -> (Json, Json) {
    let doc = Document::load(Content::Text(src.into())).unwrap_or_else(|d| panic!("{d:?}"));
    let mut inst = Instance::new(doc.program().unwrap_or_else(|| panic!("{:?}", doc.diagnostics())));
    let layout = inst.open("Page", false).unwrap_or_else(|d| panic!("{d}"));
    let frame = inst.frame(0.0, 0.0);
    (layout, frame)
}

fn size(svg: &str) -> (f64, f64) {
    let attr = |name: &str| svg.split_once(&format!(" {name}=\"")).and_then(|(_, r)| r.split_once('"')).unwrap().0.parse::<f64>().unwrap();
    (attr("width"), attr("height"))
}

#[test]
fn layouts_reach_the_host_as_a_page() {
    let (layout, _) = open(&src("layout row(flight, column(controls, readout))"));
    assert_eq!(
        layout["page"],
        json!({ "row": [{ "view": "Page.view.flight" }, { "column": [{ "view": "Page.view.controls" }, { "view": "Page.view.readout" }] }] })
    );
    // A view the layout leaves out follows it.
    let (layout, _) = open(&src("layout row(flight, controls)"));
    assert_eq!(layout["page"], json!({ "column": [{ "row": [{ "view": "Page.view.flight" }, { "view": "Page.view.controls" }] }, { "view": "Page.view.readout" }] }));
    // Without a layout, the medium decides.
    let (layout, _) = open(&src(""));
    assert!(layout.get("page").is_none());
}

#[test]
fn rows_place_views_side_by_side() {
    let opts = Options { header: false, ..Default::default() };
    let draw = |l: &str| {
        let (layout, frame) = open(&src(l));
        render(&layout, &frame, &opts)
    };
    let (stacked, row, column) = (draw(""), draw("layout row(flight, column(controls, readout))"), draw("layout column(readout, controls, flight)"));
    let ((sw, sh), (rw, rh), (cw, ch)) = (size(&stacked), size(&row), size(&column));
    assert!(rw > sw && rh < sh, "a row is wider and lower than a stack: {rw}x{rh} against {sw}x{sh}");
    assert_eq!((cw, ch), (sw, sh), "a column is a stack, in its own order");
    // The column's order is the author's: the readout's label comes before the slider.
    // Searched in the drawing, after the text alternatives.
    let at = |svg: &str, what: &str| {
        let body = &svg[svg.find("</desc>").unwrap()..];
        body.find(what).unwrap_or_else(|| panic!("{what} not drawn"))
    };
    assert!(at(&column, "reach") < at(&column, "angle"), "readout first");
}

#[test]
fn layout_mistakes_are_reported() {
    for (layout, code) in [("layout row(flight, nowhere)", "SX-E03"), ("layout row(flight, column(controls, flight))", "SX-E09")] {
        let codes: Vec<String> = match Document::load(Content::Text(src(layout))) {
            Ok(d) => d.diagnostics().iter().map(|x| x.code.clone()).collect(),
            Err(ds) => ds.iter().map(|x| x.code.clone()).collect(),
        };
        assert!(codes.iter().any(|c| c == code), "{layout}: expected {code}, got {codes:?}");
    }
    // A layout written in the IR is checked by the presentation checks (PK-E01).
    let doc = Document::load(Content::Text(src(""))).unwrap();
    let mut ir = doc.ir().clone();
    ir.presentations[0].layout = Some(Layout::Row(vec![Layout::View("Page.view.flight".into()), Layout::View("Page.view.gone".into())]));
    let codes: Vec<String> = match Document::load(Content::Ir(ir)) {
        Ok(d) => d.diagnostics().iter().map(|x| x.code.clone()).collect(),
        Err(ds) => ds.iter().map(|x| x.code.clone()).collect(),
    };
    assert!(codes.iter().any(|c| c == "PK-E01"), "got {codes:?}");
}

#[test]
fn printing_keeps_the_layout() {
    let doc = Document::load(Content::Text(src("layout row(flight, column(controls, readout))"))).unwrap();
    let text = doc.text();
    assert!(text.contains("  layout row(flight, column(controls, readout))\n"), "{text}");
    let again = Document::load(Content::Text(text)).unwrap();
    assert_eq!(again.ir().presentations[0].layout, doc.ir().presentations[0].layout);
}
