// Prismal web player: renders the frame descriptions of the presentation kernel (PK-12.1)
// as SVG, MathML and HTML controls, and forwards the learner's gestures to the kernel
// compiled to WebAssembly (crates/prismal-web). All model logic stays in the kernel; this
// file only draws and routes input.

import init, { WebPlayer, examples as loadExamples } from './pkg/prismal_web.js';

const SVGNS = 'http://www.w3.org/2000/svg';
const $ = (s) => document.querySelector(s);

const st = {
  examples: [],
  player: null,
  catalogue: null,
  layout: null,
  lesson: null,
  frame: null,
  p: 0,
  playing: false,
  speed: 1,
  lastTick: null,
  views: {},       // per view id: { kind, svg, panel, vb, base }
  drag: null,      // { rep, part, view, pointerId }
  focus: null,     // { rep } of the focused draggable, restored after each render
  items: new Map() // keyed HTML items (controls, formulas, labels) by rep id
};

// ------------------------------------------------------------------ utilities

function fmt(x) {
  if (!Number.isFinite(x)) return String(x);
  if (x === 0) return '0';
  const mag = Math.floor(Math.log10(Math.abs(x)));
  if (mag < -4 || mag >= 6) return x.toExponential(4).replace(/\.?0+e/, 'e');
  return x.toFixed(Math.max(0, 5 - mag)).replace(/(\.\d*?)0+$/, '$1').replace(/\.$/, '');
}

function svg(tag, attrs = {}, parent) {
  const e = document.createElementNS(SVGNS, tag);
  for (const [k, v] of Object.entries(attrs)) {
    // Inline style, so that style sheets cannot override sizes given in view units.
    if (k === 'font-size') e.style.fontSize = `${v}px`;
    else e.setAttribute(k, v);
  }
  if (parent) parent.appendChild(e);
  return e;
}

function el(tag, attrs = {}, ...children) {
  const e = document.createElement(tag);
  for (const [k, v] of Object.entries(attrs)) {
    if (k === 'class') e.className = v;
    else if (k === 'text') e.textContent = v;
    else e.setAttribute(k, v);
  }
  for (const c of children) e.append(c);
  return e;
}

function status(msg, bad = false) {
  const s = $('#status');
  s.textContent = msg || '';
  s.classList.toggle('bad', bad);
}

function announce(msg) {
  const a = $('#announcer');
  a.textContent = '';
  requestAnimationFrame(() => { a.textContent = msg; });
}

function parseErr(e) {
  try { return JSON.parse(String(e)); } catch { return [{ code: 'error', message: String(e) }]; }
}

/// A "nice" step (1, 2 or 5 times a power of ten) near `raw`.
function niceStep(raw) {
  const p = Math.pow(10, Math.floor(Math.log10(raw)));
  const m = raw / p;
  return (m < 1.5 ? 1 : m < 3.5 ? 2 : m < 7.5 ? 5 : 10) * p;
}

function allReps(frame) {
  const out = [];
  for (const v of frame.views) for (const r of v.reps) out.push([v.id, r]);
  for (const r of frame.overlay || []) out.push([null, r]);
  return out;
}

// ------------------------------------------------------------------ loading

async function main() {
  await init();
  st.examples = JSON.parse(loadExamples());
  const sel = $('#program');
  for (const ex of st.examples) sel.append(el('option', { value: ex.key, text: ex.title }));
  sel.append(el('option', { value: '', text: 'Edited source' }));
  st.want = parseAddress();
  sel.value = st.examples.some((e) => e.key === st.want.key) ? st.want.key : st.examples[0].key;
  sel.addEventListener('change', () => {
    if (!sel.value) return;
    location.hash = sel.value;
    loadExample(sel.value);
  });
  $('#presentation').addEventListener('change', () => openPresentation());
  $('#medium').addEventListener('change', () => openPresentation());
  $('#compile').addEventListener('click', compileSource);
  $('#format').addEventListener('click', () => {
    // Formatting prints from the IR: the program must compile. Comments that are not
    // notes before a declaration are not kept (D-036).
    try {
      const p = new WebPlayer($('#source').value);
      $('#source').value = p.formatted();
      p.free();
      $('#program').value = '';
      status('Formatted. Only comments directly before a declaration are kept.');
    } catch (e) {
      compileSource();
    }
  });
  $('#source').addEventListener('keydown', (e) => {
    if (e.key === 'Enter' && (e.ctrlKey || e.metaKey)) { e.preventDefault(); compileSource(); }
  });
  $('#source').addEventListener('input', () => { $('#program').value = ''; });
  setupTabs();
  setupTransport();
  $('#undo').addEventListener('click', () => { st.player.undo(); status(''); afterSessionAction(); });
  $('#redo').addEventListener('click', () => { st.player.redo(); status(''); afterSessionAction(); });
  $('#run-cases').addEventListener('click', runCases);
  window.addEventListener('resize', () => { if (st.frame) render(st.frame); });
  loadExample(sel.value);
}

/// The address `#key`, `#key/Presentation` or `#key/Presentation@seconds` opens that
/// program, presentation and instant (presentation time in a lesson, simulation time in a
/// session).
function parseAddress() {
  const m = decodeURIComponent(location.hash.slice(1)).match(/^([^/@]*)(?:\/([^@]*))?(?:@([0-9.]+))?$/);
  if (!m) return { key: '' };
  return { key: m[1], presentation: m[2] || null, time: m[3] != null ? parseFloat(m[3]) : null };
}

function loadExample(key) {
  const ex = st.examples.find((e) => e.key === key);
  $('#source').value = ex.source;
  compileSource();
}

function compileSource() {
  const src = $('#source').value;
  const list = $('#diagnostics');
  list.replaceChildren();
  stop();
  try {
    st.player = new WebPlayer(src);
  } catch (e) {
    st.player = null;
    const diags = parseErr(e);
    for (const d of diags) {
      const where = d.line ? `${d.line}:${d.col}` : '';
      const b = el('button', { type: 'button' }, el('code', { text: `${d.code} ${where}` }), ' ', d.message);
      b.addEventListener('click', () => selectSource(d.line, d.col, d.end - d.start));
      list.append(el('li', {}, b));
    }
    clearStage();
    status(`${diags.length} diagnostic${diags.length === 1 ? '' : 's'}: the program does not compile.`, true);
    selectTab('source');
    return;
  }
  st.catalogue = JSON.parse(st.player.catalogue());
  const psel = $('#presentation');
  psel.replaceChildren();
  for (const p of st.catalogue.presentations) {
    const note = p.kind === 'observations' ? ' (no views)' : p.kind === 'lesson' ? ' (lesson)' : '';
    psel.append(el('option', { value: p.name, text: p.name + note }));
  }
  const first = st.catalogue.presentations.find((p) => p.kind !== 'observations') || st.catalogue.presentations[0];
  if (first) psel.value = first.name;
  const asked = st.want && st.want.presentation;
  if (asked && st.catalogue.presentations.some((p) => p.name === asked)) psel.value = asked;
  $('#cases').replaceChildren();
  $('#cases-summary').textContent = `${st.catalogue.cases.length} case${st.catalogue.cases.length === 1 ? '' : 's'}`;
  openPresentation();
}

function selectSource(line, col, len) {
  const ta = $('#source');
  const lines = ta.value.split('\n');
  let idx = 0;
  for (let i = 0; i < line - 1 && i < lines.length; i++) idx += lines[i].length + 1;
  idx += Math.max(0, col - 1);
  selectTab('source');
  ta.focus();
  ta.setSelectionRange(idx, idx + Math.max(1, len || 1));
  const lh = parseFloat(getComputedStyle(ta).lineHeight) || 18;
  ta.scrollTop = Math.max(0, (line - 4) * lh);
}

function clearStage() {
  $('#views').replaceChildren();
  $('#overlay').replaceChildren();
  $('#overlay').hidden = true;
  $('#captions').replaceChildren();
  $('#transport').hidden = true;
  $('#interactive-bar').hidden = true;
  $('#medium-label').hidden = true;
  $('#observations').replaceChildren();
  $('#description').replaceChildren();
  st.items.clear();
  st.views = {};
  st.layout = null;
  st.frame = null;
}

function openPresentation() {
  stop();
  clearStage();
  if (!st.player) return;
  const name = $('#presentation').value;
  const info = st.catalogue.presentations.find((p) => p.name === name);
  if (!info) { status('The program declares no presentation.'); return; }
  $('#medium-label').hidden = info.kind !== 'lesson';
  if (info.kind === 'observations') {
    status(`${name} has no views: its observations and expectations run headless. See the Cases tab.`);
    selectTab('cases');
    return;
  }
  try {
    st.layout = JSON.parse(st.player.open(name, $('#medium').value === 'video'));
  } catch (e) {
    const diags = parseErr(e);
    status(diags.map((d) => `${d.code}: ${d.message}`).join('; '), true);
    return;
  }
  buildViews();
  st.p = 0;
  st.sess = null;
  $('#beats').hidden = false;
  $('#restart').textContent = 'Restart';
  $('#restart').title = 'Play the lesson again without your inputs';
  if (st.layout.mode === 'lesson') {
    st.lesson = st.layout.lesson;
    $('#transport').hidden = false;
    const controls = st.layout.permits.includes('timeline_controls');
    $('#scrub').disabled = !controls;
    $('#speed').disabled = !controls;
    buildBeats();
    status(lessonNotes());
  } else {
    st.lesson = null;
    st.sess = st.layout.session;
    st.p = st.sess.t;
    $('#interactive-bar').hidden = false;
    if (st.sess.dynamic) {
      // A running model: the transport plays the session's run (RC section 12).
      $('#transport').hidden = false;
      $('#scrub').disabled = false;
      $('#speed').disabled = false;
      $('#beats').hidden = true;
      $('#restart').textContent = 'Reset';
      $('#restart').title = 'A new run with no interventions, from the start';
    }
    status('');
  }
  if (st.want && st.want.time != null && clocked()) st.p = Math.max(clockStart(), Math.min(clockEnd(), st.want.time));
  st.want = null;
  refresh();
}

function lessonNotes() {
  const l = st.lesson;
  const notes = [];
  if (l.medium === 'video') notes.push('Video medium: explore beats play their fallbacks (PK-9.10).');
  for (const d of l.diagnostics) notes.push(d);
  for (const u of l.unsupported) notes.push(u);
  return notes.join(' ');
}

// ------------------------------------------------------------------ views

function buildViews() {
  const host = $('#views');
  for (const v of st.layout.views) {
    const fig = el('figure', { class: 'view', 'aria-label': `view ${v.name}` });
    const cap = el('figcaption', {}, el('span', { text: `${v.name} (${v.kind})` }));
    fig.append(cap);
    const entry = { ...v, fig, panel: el('div', { class: 'view-panel' }) };
    if (v.kind === 'spatial') {
      const [x0, y0, x1, y1] = v.extent;
      const pad = 40;
      let w = Math.max(x1 - x0 + 2 * pad, 320);
      let h = Math.max(y1 - y0 + 2 * pad, 220);
      const cx = (x0 + x1) / 2, cy = (y0 + y1) / 2;
      entry.base = [cx - w / 2, cy - h / 2, w, h];
      entry.vb = [...entry.base];
      entry.svg = svg('svg', { role: 'group', 'aria-label': `${v.name}: spatial view` });
      if (st.layout.permits.includes('zoom') || st.layout.permits.includes('pan')) {
        const reset = el('button', { type: 'button', class: 'hint', text: 'Reset view' });
        reset.addEventListener('click', () => { entry.vb = [...entry.base]; render(st.frame); });
        cap.append(reset);
        enableZoomPan(entry);
      }
      // A wide scene spans the row; a small one sits beside the other views.
      if (w > 480) fig.classList.add('wide');
    } else if (v.kind === 'plot') {
      entry.W = 560; entry.H = 360; entry.m = 44;
      entry.svg = svg('svg', { viewBox: `0 0 ${entry.W} ${entry.H}`, role: 'group', 'aria-label': `${v.name}: plot` });
    }
    if (entry.svg) {
      fig.append(entry.svg);
      setupPointer(entry);
    }
    fig.append(entry.panel);
    host.append(fig);
    st.views[v.id] = entry;
  }
}

/// View coordinates of a plot view to SVG coordinates, and back.
function plotMap(v) {
  const [x0, x1] = v.x, [y0, y1] = v.y;
  const sx = (v.W - 2 * v.m) / (x1 - x0), sy = (v.H - 2 * v.m) / (y1 - y0);
  return {
    to: ([x, y]) => [v.m + (x - x0) * sx, v.H - v.m - (y - y0) * sy],
    from: ([X, Y]) => [x0 + (X - v.m) / sx, y0 + (v.H - v.m - Y) / sy],
  };
}

function mapFor(v) {
  if (v.kind === 'plot') return plotMap(v);
  return { to: (p) => p, from: (p) => p };
}

/// SVG user units per screen pixel, to keep strokes and handles a constant size.
function unitsPerPx(v) {
  const w = v.svg.clientWidth || v.svg.getBoundingClientRect().width || 600;
  if (v.kind === 'spatial') return (v.box || v.vb)[2] / w;
  return v.W / w;
}

function drawSpatialChrome(v, g, reps, u) {
  const [x, y, w, h] = v.box || v.vb;
  const step = niceStep((60 * u) / v.px_per_m) * v.px_per_m; // at least ~60 screen px
  const metres = step / v.px_per_m;
  const hasGrid = reps.some((r) => r.kind === 'grid');
  const hasAxes = reps.some((r) => r.kind === 'axes');
  if (hasGrid) {
    for (let gx = Math.ceil(x / step) * step; gx <= x + w; gx += step) svg('line', { class: 'gridline', x1: gx, y1: y, x2: gx, y2: y + h, 'stroke-width': u }, g);
    for (let gy = Math.ceil(y / step) * step; gy <= y + h; gy += step) svg('line', { class: 'gridline', x1: x, y1: gy, x2: x + w, y2: gy, 'stroke-width': u }, g);
  }
  if (hasAxes) {
    svg('line', { class: 'axis', x1: x, y1: 0, x2: x + w, y2: 0, 'stroke-width': u }, g);
    svg('line', { class: 'axis', x1: 0, y1: y, x2: 0, y2: y + h, 'stroke-width': u }, g);
    const fs = 10 * u;
    for (let gx = Math.ceil(x / step) * step; gx <= x + w; gx += step) {
      if (Math.abs(gx) < 1e-9) continue;
      svg('line', { class: 'axis', x1: gx, y1: -3 * u, x2: gx, y2: 3 * u, 'stroke-width': u }, g);
      const t = svg('text', { class: 'tick', x: gx, y: 14 * u, 'text-anchor': 'middle', 'font-size': fs }, g);
      t.textContent = fmt(gx / v.px_per_m);
    }
    const up = v.y_up ? -1 : 1;
    for (let gy = Math.ceil(y / step) * step; gy <= y + h; gy += step) {
      if (Math.abs(gy) < 1e-9) continue;
      svg('line', { class: 'axis', x1: -3 * u, y1: gy, x2: 3 * u, y2: gy, 'stroke-width': u }, g);
      const t = svg('text', { class: 'tick', x: -6 * u, y: gy + 3 * u, 'text-anchor': 'end', 'font-size': fs }, g);
      t.textContent = fmt((up * gy) / v.px_per_m);
    }
    const axes = v.axes && v.axes.length >= 2 ? v.axes : ['x', 'y'];
    const tx = svg('text', { class: 'tick', x: x + w - 6 * u, y: -6 * u, 'text-anchor': 'end', 'font-size': fs }, g);
    tx.textContent = `${axes[0]} (m)`;
    const ty = svg('text', { class: 'tick', x: 6 * u, y: (v.y_up ? y : y + h) + 14 * u, 'font-size': fs }, g);
    ty.textContent = `${axes[1]} (m), grid ${fmt(metres)} m`;
  }
}

function drawPlotChrome(v, g) {
  const m = plotMap(v);
  const [x0, x1] = v.x, [y0, y1] = v.y;
  svg('rect', { class: 'frame', x: v.m, y: v.m, width: v.W - 2 * v.m, height: v.H - 2 * v.m }, g);
  // About one tick per 70 px across and 32 px down.
  const xs = niceStep((x1 - x0) / Math.max(2, (v.W - 2 * v.m) / 70));
  const ys = niceStep((y1 - y0) / Math.max(2, (v.H - 2 * v.m) / 32));
  for (let x = Math.ceil(x0 / xs) * xs; x <= x1 + 1e-9; x += xs) {
    const [X] = m.to([x, y0]);
    svg('line', { class: 'gridline', x1: X, y1: v.m, x2: X, y2: v.H - v.m }, g);
    const t = svg('text', { class: 'tick', x: X, y: v.H - v.m + 16, 'text-anchor': 'middle', 'font-size': 12 }, g);
    t.textContent = fmt(Math.abs(x) < xs * 1e-9 ? 0 : x);
  }
  for (let y = Math.ceil(y0 / ys) * ys; y <= y1 + 1e-9; y += ys) {
    const [, Y] = m.to([x0, y]);
    svg('line', { class: 'gridline', x1: v.m, y1: Y, x2: v.W - v.m, y2: Y }, g);
    const t = svg('text', { class: 'tick', x: v.m - 6, y: Y + 4, 'text-anchor': 'end', 'font-size': 12 }, g);
    t.textContent = fmt(Math.abs(y) < ys * 1e-9 ? 0 : y);
  }
  if (x0 < 0 && x1 > 0) { const [X] = m.to([0, 0]); svg('line', { class: 'axis', x1: X, y1: v.m, x2: X, y2: v.H - v.m }, g); }
  if (y0 < 0 && y1 > 0) { const [, Y] = m.to([0, 0]); svg('line', { class: 'axis', x1: v.m, y1: Y, x2: v.W - v.m, y2: Y }, g); }
  const [ux, uy] = v.units || ['', ''];
  if (ux) {
    const t = svg('text', { class: 'tick', x: v.W - v.m, y: v.H - 8, 'text-anchor': 'end', 'font-size': 12 }, g);
    t.textContent = ux === 's' ? 'elapsed time (s)' : `(${ux})`;
  }
  if (uy) {
    const t = svg('text', { class: 'tick', x: v.m, y: v.m - 10, 'font-size': 12 }, g);
    t.textContent = `(${uy})`;
  }
}

function arrowHead(from, to, size) {
  const dx = to[0] - from[0], dy = to[1] - from[1];
  const len = Math.hypot(dx, dy);
  if (len < 1e-9) return null;
  const ux = dx / len, uy = dy / len;
  const s = Math.min(size, len * 0.6);
  const bx = to[0] - ux * s, by = to[1] - uy * s;
  return `${to[0]},${to[1]} ${bx - uy * s * 0.45},${by + ux * s * 0.45} ${bx + uy * s * 0.45},${by - ux * s * 0.45}`;
}

function drawRep(v, g, r, u, colorIndex) {
  const m = mapFor(v);
  const cls = ['rep'];
  if (r.drag) cls.push('draggable');
  if (r.valid === false) cls.push('invalid');
  const grp = svg('g', { class: cls.join(' '), 'data-rep': r.id }, g);
  if (r.opacity != null) grp.setAttribute('opacity', r.opacity);
  const title = svg('title', {}, grp);
  title.textContent = r.text;
  let focusable = null;
  switch (r.shape) {
    case 'point': {
      const [X, Y] = m.to(r.at);
      if (r.highlighted) svg('circle', { class: 'ring', cx: X, cy: Y, r: 13 * u, 'stroke-width': 3 * u }, grp);
      const c = svg('circle', { class: 'marker', cx: X, cy: Y, r: (r.drag ? 8 : 6) * u, 'stroke-width': 1.5 * u }, grp);
      if (r.label) {
        const t = svg('text', { class: 'rep-label', x: X + 10 * u, y: Y - 10 * u, 'font-size': 12 * u }, grp);
        t.textContent = r.label;
      }
      if (r.drag) focusable = c;
      break;
    }
    case 'arrow': {
      const a = m.to(r.from), b = m.to(r.to);
      grp.classList.add('arrow', `c${colorIndex % 4}`);
      svg('line', { x1: a[0], y1: a[1], x2: b[0], y2: b[1], 'stroke-width': 2 * u }, grp);
      const head = arrowHead(a, b, 11 * u);
      if (head) svg('polygon', { points: head }, grp);
      if (r.highlighted) svg('circle', { class: 'ring', cx: b[0], cy: b[1], r: 13 * u, 'stroke-width': 3 * u }, grp);
      const len = Math.hypot(b[0] - a[0], b[1] - a[1]);
      if (r.label && len > 24 * u) {
        // At the middle, offset to the left of the direction of the arrow.
        const nx = -(b[1] - a[1]) / len, ny = (b[0] - a[0]) / len;
        const t = svg('text', { class: 'rep-label', x: (a[0] + b[0]) / 2 + nx * 12 * u, y: (a[1] + b[1]) / 2 + ny * 12 * u + 4 * u, 'text-anchor': 'middle', 'font-size': 12 * u }, grp);
        t.textContent = r.label;
      }
      if (r.drag === 'head') {
        focusable = svg('circle', { class: 'handle', cx: b[0], cy: b[1], r: 7 * u, 'stroke-width': 2 * u, 'data-part': 'head' }, grp);
      }
      break;
    }
    case 'segment': {
      const a = m.to(r.from), b = m.to(r.to);
      svg('line', { class: 'segment', x1: a[0], y1: a[1], x2: b[0], y2: b[1], 'stroke-width': 2 * u }, grp);
      break;
    }
    case 'polyline': {
      const pts = r.points.map((p) => m.to(p).join(',')).join(' ');
      const trace = r.kind === 'trace';
      svg('polyline', { class: trace ? 'trace' : 'graph', points: pts, 'stroke-width': (trace ? 1.5 : 2) * u }, grp);
      break;
    }
    case 'polygon': {
      const pts = r.points.map((p) => m.to(p).join(',')).join(' ');
      svg('polygon', { class: 'shape', points: pts, 'stroke-width': 2 * u }, grp);
      break;
    }
    case 'group': {
      // Members are placed by the kernel; the group carries opacity and highlight (D-043).
      if (r.highlighted) grp.classList.add('highlighted');
      let arrows = 0;
      for (const mem of r.members) drawRep(v, grp, mem, u, mem.shape === 'arrow' ? colorIndex + arrows++ : 0);
      break;
    }
    default:
      grp.remove();
      return;
  }
  if (r.drawn != null) {
    // `reveal draw`: every stroke is drawn up to the fraction reached (D-042).
    for (const e of grp.querySelectorAll('line, polyline, polygon')) {
      if (e.parentNode.classList.contains('arrow') && e.tagName === 'polygon') {
        e.style.opacity = r.drawn > 0.95 ? 1 : 0;
        continue;
      }
      e.setAttribute('pathLength', '1');
      e.style.strokeDasharray = '1';
      e.style.strokeDashoffset = String(1 - r.drawn);
    }
  }
  if (focusable) {
    focusable.setAttribute('tabindex', '0');
    focusable.setAttribute('role', 'button');
    focusable.setAttribute('aria-label', `${r.text}. Arrow keys move it.`);
    focusable.classList.add('rep');
    focusable.dataset.rep = r.id;
    focusable.addEventListener('keydown', (e) => onRepKey(e, r.id));
    focusable.addEventListener('focus', () => { st.focus = { rep: r.id }; });
    focusable.addEventListener('blur', () => {
      // Re-rendering replaces the element; its focus is restored after the render.
      if (!st.rendering && st.focus && st.focus.rep === r.id) st.focus = null;
    });
    grp.addEventListener('pointerdown', (e) => startDrag(e, v, r));
  }
}

// ------------------------------------------------------------------ HTML items (panel reps)

function item(id, make) {
  let it = st.items.get(id);
  if (!it) {
    it = make();
    st.items.set(id, it);
  }
  it.seen = true;
  return it;
}

function renderItem(host, r) {
  const it = item(r.id, () => {
    const root = el('div', { class: `item ${r.shape}` });
    return { root, kind: r.shape };
  });
  it.root.classList.toggle('highlighted', !!r.highlighted);
  it.root.style.opacity = r.opacity != null ? r.opacity : '';
  it.root.title = r.text;
  if (it.root.parentElement !== host) host.append(it.root);
  switch (r.shape) {
    case 'formula': {
      if (it.mathml !== r.mathml) {
        it.root.innerHTML = r.mathml || '';
        it.values = el('div', { class: 'values' });
        it.root.append(it.values);
        it.root.setAttribute('aria-label', r.text);
        it.mathml = r.mathml;
      }
      const vals = r.symbols.filter((s) => s.value != null).map((s) => `${s.symbol} = ${s.value}`).join(',  ');
      it.values.textContent = vals;
      break;
    }
    case 'equation': {
      if (it.mathml !== r.mathml) {
        it.root.innerHTML = r.mathml || '';
        it.values = el('div', { class: 'values' });
        it.root.prepend(el('span', { class: 'what', text: `equation ${r.name}` }));
        it.root.append(it.values);
        it.root.setAttribute('aria-label', r.text);
        it.mathml = r.mathml;
      }
      it.values.textContent = r.symbols.filter((s) => s.value != null).map((s) => `${s.symbol} = ${s.value}`).join(',  ');
      break;
    }
    case 'button': {
      if (!it.button) {
        it.button = el('button', { type: 'button', text: r.label });
        it.button.addEventListener('click', () => {
          if (st.lesson) { status('Buttons act in labs; in a lesson the timeline requests events.', true); return; }
          report(JSON.parse(st.player.press(r.id)));
          afterSessionAction();
        });
        it.root.replaceChildren(it.button);
      }
      it.button.setAttribute('aria-label', r.text);
      break;
    }
    case 'table': {
      const table = el('table', { class: 'data' });
      table.append(el('thead', {}, el('tr', {}, ...r.columns.map((c) => el('th', { text: c })))));
      const body = el('tbody');
      // The latest rows, most recent last.
      for (const row of r.rows.slice(-12)) body.append(el('tr', {}, ...row.map((c) => el('td', { text: c }))));
      table.append(body);
      it.root.replaceChildren(el('span', { class: 'what', text: `${r.rows.length} rows` }), table);
      break;
    }
    case 'control':
      renderControl(it, r);
      break;
    case 'text':
      it.root.replaceChildren(el('span', { class: 'label-value', text: r.text }));
      break;
    case 'status':
      it.root.replaceChildren(el('span', { class: 'status-value', text: r.text }));
      break;
    default:
      it.root.replaceChildren(el('span', { class: 'hint', text: r.text }));
  }
}

function controlDisplay(r, value) {
  if (r.control === 'toggle') return value ? 'true' : 'false';
  if (r.display_unit) return `${fmt(value / r.display_unit.scale)} ${r.display_unit.text}`;
  return r.unit ? `${fmt(value)} ${r.unit}` : fmt(value);
}

function renderControl(it, r) {
  if (!it.input) {
    const label = el('label', { class: 'control' });
    const name = el('span', { class: 'what', text: `${r.symbol}` });
    let input;
    if (r.control === 'slider') {
      input = el('input', { type: 'range' });
      input.min = r.min ?? -10;
      input.max = r.max ?? 10;
      // Without a declared step the slider is continuous; keyboard steps are the kernel's
      // (PK-11.2a).
      input.step = r.step ?? 'any';
      input.addEventListener('keydown', (e) => onControlKey(e, r, it));
    } else if (r.control === 'toggle') {
      input = el('input', { type: 'checkbox' });
    } else {
      input = el('input', { type: 'number', step: r.step ?? 'any' });
      if (r.min != null) input.min = r.min;
      if (r.max != null) input.max = r.max;
    }
    const out = el('output');
    label.append(name, input, out);
    it.root.replaceChildren(label);
    it.input = input;
    it.out = out;
    const read = () => (r.control === 'toggle' ? (input.checked ? 1 : 0) : parseFloat(input.value));
    input.addEventListener('input', () => { it.out.textContent = controlDisplay(r, read()); it.editing = true; });
    input.addEventListener('change', () => { it.editing = false; commitControl(r.id, read()); });
    input.addEventListener('blur', () => { it.editing = false; });
  }
  it.input.setAttribute('aria-label', r.text);
  it.input.disabled = !!r.disabled;
  if (!it.editing) {
    if (r.control === 'toggle') it.input.checked = !!r.value;
    else it.input.value = r.value;
    it.out.textContent = controlDisplay(r, r.value);
  }
}

function onControlKey(e, r, it) {
  const keys = { ArrowLeft: 'left', ArrowDown: 'left', ArrowRight: 'right', ArrowUp: 'right' };
  const k = keys[e.key];
  if (!k) return;
  e.preventDefault();
  it.editing = false;
  if (st.lesson) {
    // An explore control: one step of 1/100 of the range, or the declared step.
    const step = r.step ?? ((r.max - r.min) / 100);
    const v = Math.min(r.max, Math.max(r.min, parseFloat(it.input.value) + (k === 'right' ? step : -step)));
    commitControl(r.id, v);
  } else {
    report(JSON.parse(st.player.key(r.id, k)));
    afterSessionAction();
  }
}

function commitControl(id, value) {
  if (st.lesson) {
    try {
      st.lesson = JSON.parse(st.player.lesson_set_control(st.p, id, value));
      afterLessonInput();
    } catch (e) {
      status(String(e), true);
    }
  } else {
    const res = JSON.parse(st.player.set_control(id, value));
    report(res);
    afterSessionAction();
  }
}

/// The run changes after an intervention: its span and diagnostics are read again.
function afterSessionAction() {
  if (st.sess) {
    st.sess = JSON.parse(st.player.session());
    const d = st.sess.diagnostics;
    if (d.length && !$('#status').classList.contains('bad')) status(`Run: ${d[0]}${d.length > 1 ? ` (and ${d.length - 1} more)` : ''}`, true);
  }
  refresh();
}

function report(res) {
  if (res.ok) status('');
  else status(`${res.why === 'refused' ? 'Refused' : 'Rejected'}: ${res.message}`, true);
}

// ------------------------------------------------------------------ rendering

function render(frame) {
  st.rendering = true;
  try {
    renderFrame(frame);
  } finally {
    st.rendering = false;
  }
}

function renderFrame(frame) {
  st.frame = frame;
  for (const it of st.items.values()) it.seen = false;
  for (const vf of frame.views) {
    const v = st.views[vf.id];
    if (!v) continue;
    if (v.svg) {
      if (v.kind === 'spatial') {
        if (!st.lesson) growToFit(v, vf.reps);
        v.box = cameraBox(v, vf.camera);
        v.svg.setAttribute('viewBox', v.box.join(' '));
      } else {
        // A plot is drawn at its rendered size, so that text and margins stay readable.
        const w = Math.max(240, Math.round(v.svg.clientWidth || 560));
        if (w !== v.W) {
          v.W = w;
          v.H = Math.round(Math.min(w * 0.62, 420));
          v.svg.setAttribute('viewBox', `0 0 ${v.W} ${v.H}`);
        }
      }
      const u = unitsPerPx(v);
      v.svg.replaceChildren();
      const g = svg('g', {}, v.svg);
      if (v.kind === 'spatial') drawSpatialChrome(v, g, vf.reps, u);
      else drawPlotChrome(v, g);
      if (v.kind === 'plot') {
        const clip = svg('clipPath', { id: `clip-${cssId(v.id)}` }, v.svg);
        svg('rect', { x: v.m, y: v.m, width: v.W - 2 * v.m, height: v.H - 2 * v.m }, clip);
      }
      const reps = svg('g', v.kind === 'plot' ? { 'clip-path': `url(#clip-${cssId(v.id)})` } : {}, v.svg);
      const top = svg('g', {}, v.svg);
      let arrows = 0;
      for (const r of vf.reps) {
        if (['point', 'arrow', 'segment', 'polyline', 'polygon', 'group'].includes(r.shape)) {
          // Draggable representations are drawn last and outside the plot's clip.
          drawRep(v, r.drag ? top : reps, r, u, r.shape === 'arrow' ? arrows++ : 0);
        } else if (!['axes', 'grid'].includes(r.shape)) {
          renderItem(v.panel, r);
        }
      }
    } else {
      for (const r of vf.reps) renderItem(v.panel, r);
    }
  }
  const overlay = $('#overlay');
  const lessonExplore = st.lesson && exploreAt(st.p);
  for (const r of frame.overlay || []) {
    if (r.shape === 'control' && st.lesson && !lessonExplore) r.disabled = true;
    renderItem(overlay, r);
  }
  for (const [id, it] of st.items) {
    if (!it.seen) { it.root.remove(); st.items.delete(id); }
  }
  overlay.hidden = overlay.children.length === 0;
  const caps = $('#captions');
  caps.replaceChildren(...(frame.captions || []).map((c) => el('span', { text: c })));
  for (const a of frame.announcements || []) announce(`${a} at ${fmt(frame.t)} s`);
  if (st.focus) {
    const f = document.querySelector(`[tabindex][data-rep="${CSS.escape(st.focus.rep)}"]`);
    if (f && document.activeElement !== f) f.focus({ preventScroll: true });
  }
  renderDescription(frame);
}

/// The view box of a spatial view: the view's own framing, or the timeline's camera
/// (D-042) moving from it to its centre and zoom.
function cameraBox(v, cam) {
  if (!cam) return v.vb;
  const [x, y, w, h] = v.base;
  const c0 = [x + w / 2, y + h / 2];
  const c = cam.center ? [c0[0] + (cam.center[0] - c0[0]) * cam.blend, c0[1] + (cam.center[1] - c0[1]) * cam.blend] : c0;
  const [cw, ch] = [w / cam.zoom, h / cam.zoom];
  return [c[0] - cw / 2, c[1] - ch / 2, cw, ch];
}

function cssId(id) {
  return id.replace(/[^a-zA-Z0-9_-]/g, '_');
}

function growToFit(v, reps) {
  const pts = [];
  const collect = (rs) => {
    for (const r of rs) {
      if (r.shape === 'point') pts.push(r.at);
      else if (r.shape === 'arrow' || r.shape === 'segment') pts.push(r.from, r.to);
      else if (r.shape === 'group') collect(r.members);
    }
  };
  collect(reps);
  let [x, y, w, h] = v.vb;
  const pad = 30;
  for (const [px, py] of pts) {
    if (px - pad < x) { w += x - (px - pad); x = px - pad; }
    if (py - pad < y) { h += y - (py - pad); y = py - pad; }
    if (px + pad > x + w) w = px + pad - x;
    if (py + pad > y + h) h = py + pad - y;
  }
  v.vb = [x, y, w, h];
}

let lastDescription = 0;
function renderDescription(frame) {
  const now = performance.now();
  if (st.playing && now - lastDescription < 500) return;
  lastDescription = now;
  const list = $('#description');
  const lines = [`${frame.run} run, simulation time ${fmt(frame.t)} s`];
  for (const [, r] of allReps(frame)) if (r.shape !== 'axes' && r.shape !== 'grid') lines.push(r.text);
  list.replaceChildren(...lines.map((t) => el('li', { text: t })));
}

function refresh() {
  if (!st.player || !st.layout) return;
  if (st.sess) st.p = JSON.parse(st.player.seek(st.p)).t;
  const frame = JSON.parse(st.player.frame(st.p, 0));
  render(frame);
  renderObservations();
  if (clocked()) updateTransport();
}

/// Whether the open presentation has a clock: a lesson, or a session of a dynamic model.
function clocked() {
  return !!(st.lesson || (st.sess && st.sess.dynamic));
}

function clockStart() {
  return st.lesson ? 0 : st.sess.t0;
}

function clockEnd() {
  return st.lesson ? st.lesson.end : st.sess.end;
}

function renderObservations() {
  const obs = JSON.parse(st.player.observations());
  const dl = $('#observations');
  dl.replaceChildren();
  for (const [name, lines] of Object.entries(obs)) {
    const dd = el('dd');
    if (lines.length === 0) dd.append(el('span', { class: 'hint', text: 'none' }));
    for (const l of lines.slice(-50)) dd.append(el('div', { text: l }));
    dl.append(el('dt', { text: name }), dd);
  }
  if (!dl.children.length) dl.append(el('dt', { text: '-' }), el('dd', { class: 'hint', text: 'No observations for this mode.' }));
}

// ------------------------------------------------------------------ pointer and keyboard

function toView(v, e) {
  const pt = v.svg.createSVGPoint();
  pt.x = e.clientX; pt.y = e.clientY;
  const q = pt.matrixTransform(v.svg.getScreenCTM().inverse());
  return mapFor(v).from([q.x, q.y]);
}

function startDrag(e, v, r) {
  if (st.lesson) return;
  const part = r.drag === 'body' ? null : r.drag;
  if (part && e.target.dataset.part !== part) return;
  e.preventDefault();
  e.stopPropagation();
  const res = JSON.parse(st.player.pointer_down(r.id, part));
  if (!res.ok) { report(res); return; }
  // Drag mode `hold`: the run pauses while dragging and resumes after the commit (PK-10.9).
  const resume = st.playing;
  stop();
  st.drag = { rep: r.id, view: v, pointerId: e.pointerId, resume };
  v.svg.setPointerCapture(e.pointerId);
}

function setupPointer(v) {
  v.svg.addEventListener('pointermove', (e) => {
    if (!st.drag || st.drag.pointerId !== e.pointerId) return;
    const [x, y] = toView(v, e);
    const res = JSON.parse(st.player.pointer_move(x, y));
    status(res.ok ? '' : `Not valid here: ${res.message || 'rejected'}`, !res.ok);
    render(JSON.parse(st.player.frame(st.p, 0)));
  });
  const end = (e) => {
    if (!st.drag || st.drag.pointerId !== e.pointerId) return;
    const resume = st.drag.resume;
    st.drag = null;
    const res = JSON.parse(st.player.pointer_up());
    if (!res.ok) report(res);
    else if (!res.committed) status('Nothing committed: no valid position during the drag.', true);
    else status('');
    afterSessionAction();
    if (resume) togglePlay();
  };
  v.svg.addEventListener('pointerup', end);
  v.svg.addEventListener('pointercancel', (e) => {
    if (!st.drag) return;
    st.drag = null;
    st.player.cancel();
    refresh();
  });
}

window.addEventListener('keydown', (e) => {
  if (e.key === 'Escape' && st.drag) {
    st.drag = null;
    st.player.cancel();
    status('Drag cancelled.');
    refresh();
  }
  if (e.key === ' ' && clocked() && !['INPUT', 'TEXTAREA', 'SELECT', 'BUTTON'].includes(document.activeElement.tagName)) {
    e.preventDefault();
    togglePlay();
  }
});

function onRepKey(e, id) {
  const keys = { ArrowLeft: 'left', ArrowRight: 'right', ArrowUp: 'up', ArrowDown: 'down' };
  const k = keys[e.key];
  if (!k || st.lesson) return;
  e.preventDefault();
  const res = JSON.parse(st.player.key(id, k));
  report(res);
  st.focus = { rep: id };
  afterSessionAction();
}

function enableZoomPan(v) {
  const canZoom = st.layout.permits.includes('zoom');
  const canPan = st.layout.permits.includes('pan');
  if (canZoom) {
    v.svg.addEventListener('wheel', (e) => {
      e.preventDefault();
      const pt = v.svg.createSVGPoint();
      pt.x = e.clientX; pt.y = e.clientY;
      const q = pt.matrixTransform(v.svg.getScreenCTM().inverse());
      const k = Math.exp(e.deltaY * 0.0015);
      const [x, y, w, h] = v.vb;
      v.vb = [q.x - (q.x - x) * k, q.y - (q.y - y) * k, w * k, h * k];
      render(st.frame);
    }, { passive: false });
  }
  if (canPan) {
    let pan = null;
    v.svg.addEventListener('pointerdown', (e) => {
      if (st.drag || e.target.closest('.draggable')) return;
      pan = { x: e.clientX, y: e.clientY, vb: [...v.vb], id: e.pointerId };
      v.svg.setPointerCapture(e.pointerId);
    });
    v.svg.addEventListener('pointermove', (e) => {
      if (!pan || pan.id !== e.pointerId) return;
      const u = unitsPerPx(v);
      v.vb = [pan.vb[0] - (e.clientX - pan.x) * u, pan.vb[1] - (e.clientY - pan.y) * u, pan.vb[2], pan.vb[3]];
      render(st.frame);
    });
    const endPan = () => { pan = null; };
    v.svg.addEventListener('pointerup', endPan);
    v.svg.addEventListener('pointercancel', endPan);
  }
}

// ------------------------------------------------------------------ lesson transport

function setupTransport() {
  $('#play').addEventListener('click', togglePlay);
  $('#restart').addEventListener('click', () => {
    stop();
    if (st.lesson) {
      st.layout = JSON.parse(st.player.lesson_restart());
      st.lesson = st.layout.lesson;
      st.p = 0;
      buildBeats();
      status(lessonNotes());
    } else {
      st.layout = JSON.parse(st.player.reset());
      st.sess = st.layout.session;
      st.p = st.sess.t;
      status('');
    }
    refresh();
  });
  $('#continue').addEventListener('click', () => {
    try {
      st.lesson = JSON.parse(st.player.lesson_continue(st.p));
      afterLessonInput();
      if (!st.playing) togglePlay();
    } catch (e) {
      status(String(e), true);
    }
  });
  $('#scrub').addEventListener('input', () => {
    st.p = parseFloat($('#scrub').value);
    refresh();
  });
  $('#speed').addEventListener('change', () => { st.speed = parseFloat($('#speed').value); });
}

function afterLessonInput() {
  buildBeats();
  const refusals = st.lesson.refusals || [];
  const last = refusals[refusals.length - 1];
  if (last && Math.abs(last.at - st.p) < 1e-9) status(`Refused: ${last.reason}`, true);
  else status(lessonNotes());
  refresh();
}

function exploreAt(p) {
  return (st.lesson?.explore || []).find((x) => p >= x.start && p < x.end) || null;
}

function buildBeats() {
  const host = $('#beats');
  host.replaceChildren();
  const end = st.lesson.end || 1;
  for (const b of st.lesson.beats) {
    const w = Math.max(0, b.end - b.start);
    const btn = el('button', { type: 'button', role: 'listitem', title: `${b.scene} / ${b.beat}: ${fmt(b.start)} s to ${fmt(b.end)} s`, text: b.beat });
    btn.style.flex = `${Math.max(w, end * 0.012)} 1 0`;
    if (st.lesson.explore.some((x) => x.beat === b.beat)) btn.classList.add('explore');
    btn.addEventListener('click', () => { st.p = b.start; refresh(); });
    btn.dataset.beat = b.beat;
    host.append(btn);
  }
}

function updateTransport() {
  const [t0, end] = [clockStart(), clockEnd()];
  const scrub = $('#scrub');
  scrub.min = t0;
  scrub.max = end;
  scrub.value = st.p;
  const label = (x) => fmt(Math.round(x * 10) / 10);
  let where = '';
  if (st.lesson) {
    const l = st.lesson;
    const beat = l.beats.find((b) => st.p >= b.start && st.p < b.end) || l.beats[l.beats.length - 1];
    for (const b of $('#beats').children) b.classList.toggle('current', beat && b.dataset.beat === beat.beat);
    const ex = exploreAt(st.p);
    $('#continue').hidden = !ex;
    if (ex) $('#continue').textContent = `Continue (explore ends in ${Math.ceil(ex.end - st.p)} s)`;
    if (beat) where = `, beat ${beat.beat}`;
  } else {
    $('#continue').hidden = true;
    where = `, ${st.sess.interventions} intervention${st.sess.interventions === 1 ? '' : 's'}`;
  }
  $('#clock').textContent = `${st.lesson ? '' : 't = '}${label(st.p)} s / ${label(end)} s${where}`;
  $('#play').textContent = st.playing ? 'Pause' : st.p >= end ? 'Replay' : 'Play';
  $('#play').setAttribute('aria-label', $('#play').textContent);
}

function togglePlay() {
  if (!clocked()) return;
  if (st.playing) { stop(); updateTransport(); return; }
  if (st.p >= clockEnd()) st.p = clockStart();
  st.playing = true;
  st.lastTick = null;
  requestAnimationFrame(tick);
  updateTransport();
}

function stop() {
  st.playing = false;
}

function tick(now) {
  if (!st.playing || !clocked()) return;
  const dt = st.lastTick == null ? 0 : Math.min(0.1, (now - st.lastTick) / 1000) * st.speed;
  st.lastTick = now;
  const end = clockEnd();
  st.p = Math.min(end, st.p + dt);
  if (st.sess) st.p = JSON.parse(st.player.seek(st.p)).t;
  const frame = JSON.parse(st.player.frame(st.p, dt));
  render(frame);
  updateTransport();
  if (st.p >= end) {
    stop();
    updateTransport();
    renderObservations();
    return;
  }
  requestAnimationFrame(tick);
}

// ------------------------------------------------------------------ side panel

function setupTabs() {
  for (const b of document.querySelectorAll('.tabs button')) {
    b.addEventListener('click', () => selectTab(b.dataset.tab));
  }
}

function selectTab(name) {
  for (const b of document.querySelectorAll('.tabs button')) b.setAttribute('aria-selected', String(b.dataset.tab === name));
  for (const t of document.querySelectorAll('.tab')) t.hidden = t.id !== `tab-${name}`;
  if (name === 'observe' && st.player && st.layout) renderObservations();
}

function runCases() {
  if (!st.player) return;
  const host = $('#cases');
  host.replaceChildren(el('p', { class: 'hint', text: 'Running...' }));
  setTimeout(() => {
    const cases = JSON.parse(st.player.run_cases());
    host.replaceChildren();
    let pass = 0, fail = 0;
    for (const c of cases) {
      const box = el('div', { class: 'case' }, el('h3', { text: `run ${c.case}` }));
      const ul = el('ul');
      if (c.error) { ul.append(el('li', { class: 'fail', text: c.error })); fail++; }
      for (const b of c.beats || []) ul.append(el('li', { class: 'hint', text: `beat ${b.beat}: ${fmt(b.start)} s to ${fmt(b.end)} s` }));
      for (const r of c.results || []) {
        ul.append(el('li', { class: r.pass ? 'pass' : 'fail', text: r.pass ? `expect ${r.id}` : `expect ${r.id}: ${r.message}` }));
        if (r.pass) pass++; else fail++;
      }
      box.append(ul);
      host.append(box);
    }
    if (!cases.length) host.append(el('p', { class: 'hint', text: 'The program has no cases.' }));
    $('#cases-summary').textContent = `${pass} passed, ${fail} failed`;
  }, 10);
}

main().catch((e) => {
  status(`The player could not start: ${e}. Build the WebAssembly package first (web/README.md).`, true);
});
