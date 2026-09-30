// Compares frames drawn by the SVG renderer (crates/prismal-svg) with the same frames drawn
// by the web player (web/) in a headless browser: for each case, the view boxes of spatial
// views and, in every view, the geometry of every representation (marker and handle
// centres, line ends, path and polygon points, in view coordinates; plots normalized to
// their frame), and the text alternatives of the representations drawn.
//
// Sizes that depend on the screen (stroke widths, radii, arrow heads, label positions) are
// not compared: the web player keeps them constant in screen pixels.
//
// Needs Node 18 or later, the player built (`./web/build.sh`) and Microsoft Edge or Chrome
// (path in the BROWSER environment variable, or the default Edge path on Windows).
//
//   node crates/prismal-svg/compare.mjs

import { execFile, execFileSync } from 'node:child_process';
import { promisify } from 'node:util';
import { createServer } from 'node:http';
import { mkdtempSync, readFileSync, statSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, extname, dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');

// Program, presentation, instant (presentation time in a lesson, simulation time in a
// session).
const CASES = [
  ['rp06', 'QuadraticPlot', 0],
  ['rp07', 'VectorPlot', 0],
  ['rp01', 'ProjectileLab', 1.2],
  ['rp03', 'BounceLab', 2.5],
  ['rp04', 'PendulumLab', 1.5],
  ['rp05', 'SpringLab', 0.7],
  ['rp08', 'ProjectileLesson', 9],
  ['rp08', 'ProjectileLesson', 20],
  ['g7-wheel', 'WheelView', 1],
  ['g8-circle', 'Unwrap', 7.5],
  ['g8-circle', 'Unwrap', 20],
  ['g8-freefall', 'DropMovie', 1.5],
  ['g8-freefall', 'DropMovie', 3.2],
  ['g8-freefall', 'DropMovie', 5.2],
  ['g8-freefall', 'DropMovie', 7.5],
];
const TOL = 0.01;

const browser = process.env.BROWSER || 'C:\\Program Files (x86)\\Microsoft\\Edge\\Application\\msedge.exe';

// ------------------------------------------------------------------ both renderers

function serve() {
  const types = { '.html': 'text/html', '.js': 'text/javascript', '.css': 'text/css', '.wasm': 'application/wasm' };
  const server = createServer((req, res) => {
    const path = join(root, 'web', decodeURIComponent(new URL(req.url, 'http://x').pathname));
    try {
      const file = statSync(path).isDirectory() ? join(path, 'index.html') : path;
      res.writeHead(200, { 'content-type': types[extname(file)] || 'application/octet-stream' });
      res.end(readFileSync(file));
    } catch {
      res.writeHead(404);
      res.end();
    }
  });
  return new Promise((ok) => server.listen(0, '127.0.0.1', () => ok(server)));
}

// The browser runs asynchronously, so that this process keeps serving the player to it. It
// uses its own profile, so that it never hands the page to a browser already open.
async function webDom(port, profile, [key, pres, t]) {
  const url = `http://127.0.0.1:${port}/#${key}/${pres}@${t}`;
  const args = ['--headless=new', '--disable-gpu', '--no-first-run', `--user-data-dir=${profile}`, '--window-size=1400,1000', '--virtual-time-budget=15000', '--dump-dom', url];
  const { stdout } = await promisify(execFile)(browser, args, { encoding: 'utf8', maxBuffer: 64 << 20 });
  return stdout;
}

function svgDoc(out, [key, pres, t]) {
  const exe = join(root, 'target', 'debug', 'examples', process.platform === 'win32' ? 'render.exe' : 'render');
  const path = execFileSync(exe, [key, pres, '--at', String(t), '--out', out], { cwd: root, encoding: 'utf8' }).trim();
  return readFileSync(resolve(root, path), 'utf8');
}

// ------------------------------------------------------------------ geometry

const attrs = (s) => Object.fromEntries([...s.matchAll(/([\w:-]+)="([^"]*)"/g)].map((m) => [m[1], m[2]]));
const nums = (s) => s.trim().split(/[\s,]+/).map(Number);
const unescape = (s) => s.replace(/&lt;/g, '<').replace(/&gt;/g, '>').replace(/&quot;/g, '"').replace(/&amp;/g, '&');

/// The views of a document by their accessible names: view box, plot frame, and per
/// representation its text alternative and the points of its marks.
function geometry(doc) {
  const views = {};
  let view = null;
  const reps = []; // stack of open groups: rep id or null
  const tags = /<(\/?)(svg|g|circle|line|polyline|polygon|path|rect|title)\b([^>]*?)(\/?)>/g;
  let m;
  let title = null;
  while ((m = tags.exec(doc))) {
    const [, close, tag, rest, self] = m;
    if (close) {
      if (tag === 'g') reps.pop();
      if (tag === 'svg') view = null;
      continue;
    }
    const a = attrs(rest);
    if (tag === 'svg') {
      if (a['aria-label'] && a.viewBox && /: (plot|spatial view)$/.test(a['aria-label'])) {
        view = views[a['aria-label']] = { viewBox: nums(a.viewBox), frame: null, reps: {} };
      }
      continue;
    }
    if (!view) continue;
    const rep = reps.filter(Boolean).at(-1);
    if (tag === 'g') {
      if (!self) reps.push(a['data-rep'] || null);
      if (a['data-rep']) {
        const r = (view.reps[a['data-rep']] ||= { text: null, marks: [], arrow: /\barrow\b/.test(a.class || '') });
        title = r;
      }
      continue;
    }
    if (tag === 'title') {
      if (title) {
        const end = doc.indexOf('</title>', tags.lastIndex);
        title.text = unescape(doc.slice(tags.lastIndex, end));
        title = null;
      }
      continue;
    }
    if (tag === 'rect' && /\bframe\b/.test(a.class || '')) view.frame = [+a.x, +a.y, +a.width, +a.height];
    if (!rep) continue;
    const r = view.reps[rep];
    if (tag === 'circle') r.marks.push([`circle.${(a.class || '').split(' ')[0]}`, [+a.cx, +a.cy]]);
    else if (tag === 'line') r.marks.push(['line', [+a.x1, +a.y1, +a.x2, +a.y2]]);
    else if (tag === 'polyline' || (tag === 'polygon' && !r.arrow)) r.marks.push([tag, nums(a.points)]);
    // Circles, ellipses and arcs (PK-6.3c): the numbers of the path, flags included.
    else if (tag === 'path') r.marks.push([tag, a.d.match(/-?\d+(\.\d+)?(e-?\d+)?/g).map(Number)]);
  }
  // Plot coordinates relative to the plot's frame, so that plots drawn at different
  // sizes compare.
  for (const v of Object.values(views)) {
    if (!v.frame) continue;
    const [fx, fy, fw, fh] = v.frame;
    for (const r of Object.values(v.reps)) r.marks = r.marks.map(([k, p]) => [k, p.map((x, i) => (i % 2 ? (x - fy) / fh : (x - fx) / fw) * 1000)]);
  }
  return views;
}

/// Texts agree when they agree outside their numbers and each pair of numbers agrees to
/// 1e-6, absolute or relative: the web player runs the kernel compiled to WebAssembly, and
/// replay is only tolerance-equivalent across platforms (D-015).
function sameText(a, b) {
  const num = /-?\d+(?:\.\d+)?(?:e[-+]?\d+)?/g;
  if (a === b) return true;
  if (a == null || b == null || a.replace(num, '#') !== b.replace(num, '#')) return false;
  const [x, y] = [a.match(num) || [], b.match(num) || []];
  return x.every((v, i) => Math.abs(v - y[i]) <= 1e-6 * Math.max(1, Math.abs(v)));
}

function compare(web, svg) {
  const errs = [];
  const names = new Set([...Object.keys(web), ...Object.keys(svg)]);
  let marks = 0;
  for (const name of names) {
    const [w, s] = [web[name], svg[name]];
    if (!w || !s) {
      errs.push(`view ${name}: drawn by ${w ? 'the web player' : 'the SVG renderer'} only`);
      continue;
    }
    if (!w.frame && w.viewBox.some((x, i) => Math.abs(x - s.viewBox[i]) > TOL)) errs.push(`view ${name}: view box ${w.viewBox} (web) against ${s.viewBox} (SVG)`);
    for (const id of new Set([...Object.keys(w.reps), ...Object.keys(s.reps)])) {
      const [a, b] = [w.reps[id], s.reps[id]];
      if (!a || !b) {
        errs.push(`${id}: drawn by ${a ? 'the web player' : 'the SVG renderer'} only`);
        continue;
      }
      if (!sameText(a.text, b.text)) errs.push(`${id}: text "${a.text}" (web) against "${b.text}" (SVG)`);
      if (a.marks.length !== b.marks.length) {
        errs.push(`${id}: ${a.marks.length} marks (web) against ${b.marks.length} (SVG)`);
        continue;
      }
      a.marks.forEach(([k, p], i) => {
        const [k2, q] = b.marks[i];
        marks++;
        const d = p.length === q.length ? Math.max(0, ...p.map((x, j) => Math.abs(x - q[j]))) : Infinity;
        if (k !== k2 || d > TOL) errs.push(`${id}: ${k} ${p.slice(0, 6)} (web) against ${k2} ${q.slice(0, 6)} (SVG)`);
      });
    }
  }
  return { errs, marks };
}

// ------------------------------------------------------------------ main

execFileSync('cargo', ['build', '-q', '-p', 'prismal-svg', '--example', 'render'], { cwd: root, stdio: 'inherit' });
const out = mkdtempSync(join(tmpdir(), 'prismal-svg-'));
const server = await serve();
const port = server.address().port;
let failed = 0;
for (const c of CASES) {
  const web = geometry(await webDom(port, join(out, 'profile'), c));
  const svg = geometry(svgDoc(out, c));
  const { errs, marks } = compare(web, svg);
  const reps = Object.values(svg).reduce((n, v) => n + Object.keys(v.reps).length, 0);
  const label = `${c[0]} ${c[1]} at ${c[2]} s`;
  if (Object.keys(web).length === 0) {
    failed++;
    console.log(`FAIL ${label}: the web player drew no view (is web/pkg built?)`);
  } else if (errs.length) {
    failed++;
    console.log(`FAIL ${label}`);
    for (const e of errs.slice(0, 12)) console.log(`  ${e}`);
  } else {
    console.log(`ok   ${label}: ${Object.keys(svg).length} views, ${reps} representations, ${marks} marks`);
  }
}
server.close();
console.log(failed ? `${failed} of ${CASES.length} cases differ` : `all ${CASES.length} cases agree`);
process.exit(failed ? 1 : 0);
