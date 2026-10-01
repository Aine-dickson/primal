// Drives the web player in a headless browser through the DevTools protocol with real mouse,
// wheel and key events, which the player forwards to the engine as raw input (D-047,
// HI-4.5): hover, drags in a spatial view and in a plot, focus after a drag, a keyboard step,
// clicks on members of a collection and their keyboard activation (D-059), wheel zoom, pan
// and reset, and an author's page layout (D-063).
//
// Needs Node 22 or later, the player built (`./web/build.sh`) and Microsoft Edge or Chrome
// (path in the BROWSER environment variable, or the default Edge path on Windows; extra flags
// in BROWSER_ARGS).
//
//   node web/check-input.mjs

import { spawn } from 'node:child_process';
import { createServer } from 'node:http';
import { readFileSync, statSync, mkdtempSync } from 'node:fs';
import { join, extname, dirname } from 'node:path';
import { tmpdir } from 'node:os';
import { fileURLToPath } from 'node:url';

const web = dirname(fileURLToPath(import.meta.url));
const browser = process.env.BROWSER || 'C:\\Program Files (x86)\\Microsoft\\Edge\\Application\\msedge.exe';
const types = { '.html': 'text/html', '.js': 'text/javascript', '.css': 'text/css', '.wasm': 'application/wasm' };
const server = createServer((req, res) => {
  const path = join(web, decodeURIComponent(new URL(req.url, 'http://x').pathname));
  try {
    const file = statSync(path).isDirectory() ? join(path, 'index.html') : path;
    res.writeHead(200, { 'content-type': types[extname(file)] || 'application/octet-stream' });
    res.end(readFileSync(file));
  } catch {
    res.writeHead(404);
    res.end();
  }
});
await new Promise((ok) => server.listen(0, '127.0.0.1', ok));
const port = server.address().port;
const devtools = 9337;
// BROWSER_ARGS adds flags, such as `--no-sandbox` where the system forbids Chrome's sandbox.
const edge = spawn(browser, [...(process.env.BROWSER_ARGS || '').split(' ').filter(Boolean), '--headless=new', '--disable-gpu', `--remote-debugging-port=${devtools}`, `--user-data-dir=${mkdtempSync(join(tmpdir(), 'prismal-input-'))}`, '--window-size=1300,900', 'about:blank']);
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));

let target;
for (let i = 0; i < 50 && !target; i++) {
  await sleep(200);
  try {
    target = (await (await fetch(`http://127.0.0.1:${devtools}/json`)).json()).find((t) => t.type === 'page');
  } catch {}
}
if (!target) {
  console.error(`the browser at ${browser} did not open its DevTools port; on Linux CI, BROWSER_ARGS=--no-sandbox may be needed`);
  edge.kill();
  process.exit(1);
}
const ws = new WebSocket(target.webSocketDebuggerUrl);
await new Promise((r) => ws.addEventListener('open', r));
let id = 0;
const pending = new Map();
ws.addEventListener('message', (m) => {
  const d = JSON.parse(m.data);
  if (pending.has(d.id)) {
    pending.get(d.id)(d);
    pending.delete(d.id);
  }
});
const cdp = (method, params = {}) => new Promise((r) => { const i = ++id; pending.set(i, r); ws.send(JSON.stringify({ id: i, method, params })); });
// The page's size is set here: some headless browsers ignore `--window-size`.
await cdp('Emulation.setDeviceMetricsOverride', { width: 1300, height: 900, deviceScaleFactor: 1, mobile: false });
const ev = async (expr) => (await cdp('Runtime.evaluate', { expression: expr, returnByValue: true })).result.result.value;
const open = async (hash) => {
  await cdp('Page.navigate', { url: `http://127.0.0.1:${port}/?${Math.random()}#${hash}` });
  for (let i = 0; i < 50; i++) {
    await sleep(200);
    if (await ev(`!!document.querySelector('#views svg g')`)) break;
  }
  await sleep(300);
};
const center = (sel) => ev(`(() => { const r = document.querySelector(${JSON.stringify(sel)}).getBoundingClientRect(); return [r.x + r.width / 2, r.y + r.height / 2]; })()`);
const mouse = (type, x, y, extra = {}) => cdp('Input.dispatchMouseEvent', { type, x, y, button: 'left', buttons: type === 'mouseReleased' ? 0 : 1, clickCount: 1, ...extra });
const key = (k, vk) => cdp('Input.dispatchKeyEvent', { type: 'rawKeyDown', key: k, code: k, windowsVirtualKeyCode: vk }).then(() => cdp('Input.dispatchKeyEvent', { type: 'keyUp', key: k, code: k, windowsVirtualKeyCode: vk }));
const title = (rep) => ev(`document.querySelector('[data-rep="${rep}"] title').textContent`);
let failed = 0;
const check = (name, cond, detail) => {
  console.log(`${cond ? 'ok  ' : 'FAIL'} ${name}${detail ? `: ${detail}` : ''}`);
  if (!cond) failed++;
};

// RP-07: the head of u dragged one metre (40 view pixels) to the right, then a key step.
await open('rp07/VectorPlot');
const head = 'VectorPlot.view.scene.arrow.1';
const [hx, hy] = await center(`[data-rep="${head}"] circle.handle`);
// Screen pixels per view pixel: the view's box is fitted into the element.
const scale = await ev(`(() => { const s = document.querySelector('#views svg'); const r = s.getBoundingClientRect(); return Math.min(r.width / s.viewBox.baseVal.width, r.height / s.viewBox.baseVal.height); })()`);
await mouse('mouseMoved', hx + 2, hy, { buttons: 0 });
check('hover over a draggable part shows a grab cursor', (await ev(`document.querySelector('#views svg').style.cursor`)) === 'grab');
await mouse('mousePressed', hx + 2, hy);
await mouse('mouseMoved', hx + 20 * scale, hy);
await mouse('mouseMoved', hx + 40 * scale, hy);
await mouse('mouseReleased', hx + 40 * scale, hy);
await sleep(200);
const u = await title(head);
check('drag of u to (4 m, 0 m)', u.startsWith('arrow u = (4 m, 0 m)'), u);
check('the grabbed arrow has focus', (await ev('document.activeElement.dataset.rep')) === head);
await key('ArrowRight', 39);
await sleep(200);
const stepped = await title(head);
check('an arrow key steps the focused head', stepped.startsWith('arrow u = (4.25 m, 0 m)'), stepped);

// RP-06: the plot's handle dragged up by a few pixels.
await open('rp06/QuadraticPlot');
const [px, py] = await center('[data-rep="QuadraticPlot.view.plot.handle"] circle');
await mouse('mousePressed', px, py);
await mouse('mouseMoved', px, py - 10);
await mouse('mouseReleased', px, py - 10);
await sleep(200);
const handle = await title('QuadraticPlot.view.plot.handle');
check('a drag in a plot moves the handle up', !/y = 1$/.test(handle), handle);

// The guide's orbits lab (D-059): a click on a planet removes it; Enter on a focused planet
// clicks it.
await open('g9-orbits/Sky');
const planet = (k) => `Sky.view.sky.planet[${k}]`;
const shown = (k) => ev(`!!document.querySelector('[data-rep="${planet(k)}"]')`);
check('planets are drawn', (await shown(1)) && (await shown(2)));
const [cx, cy] = await center(`[data-rep="${planet(2)}"] circle.marker`);
await mouse('mouseMoved', cx, cy, { buttons: 0 });
check('hover over a planet shows a grab cursor', (await ev(`document.querySelector('#views svg').style.cursor`)) === 'grab');
await mouse('mousePressed', cx, cy);
await mouse('mouseReleased', cx + 1, cy);
await sleep(200);
check('a click on planet 2 removes it', !(await shown(2)) && (await shown(1)));
check('the text alternative says what activating does', (await title(planet(1))).endsWith('activate: remove'), await title(planet(1)));
await ev(`document.querySelector('[tabindex][data-rep="${planet(1)}"]').focus()`);
await key('Enter', 13);
await sleep(200);
check('Enter on a focused planet removes it', !(await shown(1)));
// D-060: an empty point of the sky shows a crosshair, and a click there places a planet.
await mouse('mouseMoved', cx, cy, { buttons: 0 });
check('hover over an empty point shows a crosshair', (await ev(`document.querySelector('#views svg').style.cursor`)) === 'crosshair');
await mouse('mousePressed', cx, cy);
await mouse('mouseReleased', cx, cy);
await sleep(200);
check('a click on an empty point places planet 3', await shown(3));

// D-061: the author's color reaches the browser, from the theme's palette.
await open('g7-wheel/WheelView');
const valveFill = await ev(`getComputedStyle(document.querySelector('[data-rep$="valve"] circle.marker')).fill`);
check('the valve is drawn in the red of the theme', ['rgb(198, 40, 40)', 'rgb(255, 138, 128)'].includes(valveFill), valveFill);

// D-062: a member of a turned group is dragged in the group's frame.
await open('g7-dial/Knob');
const tipSel = '[data-rep$="tip"] circle.marker';
check('the dial tip can be grabbed', !!(await ev(`document.querySelector('${tipSel}')`)));
const tipBefore = await title('Knob.view.scene.dial.tip');
const [tx, ty] = await center(tipSel);
await mouse('mousePressed', tx, ty);
await mouse('mouseMoved', tx, ty - 20);
await mouse('mouseReleased', tx, ty - 20);
await sleep(200);
const tipAfter = await title('Knob.view.scene.dial.tip');
check('dragging the tip up lengthens the hand', tipAfter !== tipBefore, `${tipBefore} to ${tipAfter}`);

// RP-08 permits zoom and pan.
await open('rp08/ProjectileLesson@1');
const vb = () => ev(`document.querySelector('#views svg').getAttribute('viewBox')`);
const before = await vb();
const [sx, sy] = await center('#views svg');
await cdp('Input.dispatchMouseEvent', { type: 'mouseWheel', x: sx, y: sy, deltaX: 0, deltaY: -300 });
await sleep(200);
const zoomed = await vb();
check('the wheel zooms', +zoomed.split(' ')[2] < +before.split(' ')[2], `${before} to ${zoomed}`);
await mouse('mousePressed', sx + 100, sy + 60);
await mouse('mouseMoved', sx + 160, sy + 60);
await mouse('mouseReleased', sx + 160, sy + 60);
await sleep(200);
const panned = await vb();
check('a press and move on empty space pans', +panned.split(' ')[0] < +zoomed.split(' ')[0], `${zoomed} to ${panned}`);
await ev(`[...document.querySelectorAll('figcaption button')].find((b) => b.textContent === 'Reset view').click()`);
await sleep(200);
check('Reset view returns to the framing', (await vb()) === before);
// Two touches spreading apart pinch the view in (HI-4.5).
const pts = (d) => [{ x: sx - d, y: sy, id: 1 }, { x: sx + d, y: sy, id: 2 }];
await cdp('Input.dispatchTouchEvent', { type: 'touchStart', touchPoints: pts(20) });
await cdp('Input.dispatchTouchEvent', { type: 'touchMove', touchPoints: pts(40) });
await cdp('Input.dispatchTouchEvent', { type: 'touchMove', touchPoints: pts(60) });
await cdp('Input.dispatchTouchEvent', { type: 'touchEnd', touchPoints: [] });
await sleep(200);
const pinched = await vb();
check('two touches spreading apart zoom in', +pinched.split(' ')[2] < +before.split(' ')[2] * 0.6, `${before} to ${pinched}`);

// The guide's cannon page (D-063): the plot on the left, the controls above the readout on
// its right.
await open('g7-cannon/CannonPage');
const box = (name) => ev(`(() => { const r = document.querySelector('figure[aria-label="view ${name}"]').getBoundingClientRect(); return [r.left, r.top, r.right, r.bottom]; })()`);
const [fl, ctl, rd] = [await box('flight'), await box('controls'), await box('readout')];
check('a layout row puts the plot left of the panels', fl[2] <= ctl[0] && fl[2] <= rd[0], `${fl} | ${ctl} | ${rd}`);
check('a layout column puts the controls above the readout', ctl[3] <= rd[1] && Math.abs(ctl[0] - rd[0]) < 1, `${ctl} / ${rd}`);

ws.close();
edge.kill();
server.close();
console.log(failed ? `${failed} checks failed` : 'all checks pass');
process.exit(failed ? 1 : 0);
