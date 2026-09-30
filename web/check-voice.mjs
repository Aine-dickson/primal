// Drives the web player's narration sound (D-053) in a headless browser: synthesized speech
// at each caption cue, recordings named by cue started at the right offset, notes on cues
// without a recording and recordings longer than their cue, and silence on pause. Speech
// and audio playback are replaced by recorders before the page loads, so nothing is heard.
//
// Needs Node 22 or later, the player built (`./web/build.sh`) and Microsoft Edge or Chrome
// (path in the BROWSER environment variable, or the default Edge path on Windows).
//
//   node web/check-voice.mjs

import { spawn } from 'node:child_process';
import { createServer } from 'node:http';
import { readFileSync, statSync, mkdtempSync, writeFileSync } from 'node:fs';
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
const devtools = 9338;
const temp = mkdtempSync(join(tmpdir(), 'prismal-voice-'));
const edge = spawn(browser, ['--headless=new', '--disable-gpu', '--autoplay-policy=no-user-gesture-required', `--remote-debugging-port=${devtools}`, `--user-data-dir=${join(temp, 'profile')}`, '--window-size=1300,900', 'about:blank']);
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));

let target;
for (let i = 0; i < 50 && !target; i++) {
  await sleep(200);
  try {
    target = (await (await fetch(`http://127.0.0.1:${devtools}/json`)).json()).find((t) => t.type === 'page');
  } catch {}
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
const ev = async (expr) => (await cdp('Runtime.evaluate', { expression: expr, returnByValue: true, awaitPromise: true })).result.result.value;
const playing = async (on) => {
  if ((await ev(`document.querySelector('#play').textContent`)) === 'Pause' ? !on : on) await ev(`document.querySelector('#play').click()`);
};
const scrub = (t) => ev(`(() => { const s = document.querySelector('#scrub'); s.value = '${t}'; s.dispatchEvent(new Event('input')); })()`);
let failed = 0;
const check = (name, cond, detail) => {
  console.log(`${cond ? 'ok  ' : 'FAIL'} ${name}${detail ? `: ${detail}` : ''}`);
  if (!cond) failed++;
};

// Recorders in place of speech and audio playback, installed before the player loads.
await cdp('Page.enable');
await cdp('Page.addScriptToEvaluateOnNewDocument', {
  source: `
    window.__spoken = []; window.__played = []; window.__cancels = 0;
    const synth = { speak: (u) => window.__spoken.push({ text: u.text, rate: u.rate, at: window.__clock() }), cancel: () => { window.__cancels++; }, pause() {}, resume() {} };
    Object.defineProperty(window, 'speechSynthesis', { value: synth });
    HTMLMediaElement.prototype.play = function () { window.__played.push({ src: this.src, offset: this.currentTime, rate: this.playbackRate, at: window.__clock() }); return Promise.resolve(); };
    window.__clock = () => parseFloat(document.querySelector('#scrub')?.value || 'NaN');
  `,
});

// A silent WAV of `seconds`, 8 kHz mono.
const wav = (seconds) => {
  const n = Math.round(8000 * seconds);
  const b = Buffer.alloc(44 + n);
  b.write('RIFF', 0); b.writeUInt32LE(36 + n, 4); b.write('WAVE', 8); b.write('fmt ', 12);
  b.writeUInt32LE(16, 16); b.writeUInt16LE(1, 20); b.writeUInt16LE(1, 22); b.writeUInt32LE(8000, 24);
  b.writeUInt32LE(8000, 28); b.writeUInt16LE(1, 32); b.writeUInt16LE(8, 34); b.write('data', 36); b.writeUInt32LE(n, 40);
  b.fill(128, 44);
  return b;
};

await cdp('Page.navigate', { url: `http://127.0.0.1:${port}/#rp08/ProjectileLesson` });
for (let i = 0; i < 50; i++) {
  await sleep(200);
  if (await ev(`!!document.querySelector('#views svg g')`)) break;
}
await sleep(300);
check('a lesson with captions offers a voice', await ev(`!document.querySelector('#voice-label').hidden`));

// Synthesized speech: every cue spoken once, in order, at its start, at the playback speed.
const cues = [
  ['A ball is launched at 45 degrees.', 0],
  ['It lands here. Why this distance?', 6.883],
  ['Watch the horizontal speed.', 9.883],
  ['Choose your own angle.', 18.65],
  ['Compare the distance with the first launch.', 28.181],
];
// In the video medium the explore beat plays its fallback, so the lesson runs to its end.
await ev(`(() => { const m = document.querySelector('#medium'); m.value = 'video'; m.dispatchEvent(new Event('change')); })()`);
await sleep(300);
await ev(`(() => { const s = document.querySelector('#voice'); s.value = 'speech'; s.dispatchEvent(new Event('change')); const v = document.querySelector('#speed'); v.value = '2'; v.dispatchEvent(new Event('change')); document.querySelector('#play').click(); })()`);
for (let i = 0; i < 100 && (await ev(`document.querySelector('#play').textContent`)) !== 'Replay'; i++) await sleep(250);
const spoken = await ev('window.__spoken');
check('every cue is spoken once, in order', JSON.stringify(spoken.map((s) => s.text)) === JSON.stringify(cues.map((c) => c[0])), JSON.stringify(spoken.map((s) => s.text)));
// The clock is read from the scrubber, in steps of 0.01 s.
check('each at its start', spoken.length === cues.length && spoken.every((s, k) => s.at >= cues[k][1] - 0.02 && s.at < cues[k][1] + 0.3), JSON.stringify(spoken.map((s) => s.at)));
check('at the playback speed', spoken.every((s) => s.rate === 2));

// Joining a cue part way: speech cannot start mid-sentence, so that cue stays silent.
await playing(false);
await scrub(7.5);
await ev('window.__spoken = []');
await playing(true);
await sleep(600);
await playing(false);
check('speech does not start part way through a cue', (await ev('window.__spoken')).length === 0);

// Recordings named by cue: b1 fits its 4 s cue, b3 lasts 5 s for a 3 s cue, the others are
// missing and synthesized.
writeFileSync(join(temp, 'b1.wav'), wav(1));
writeFileSync(join(temp, 'b3.wav'), wav(5));
const doc = await cdp('DOM.getDocument');
const input = await cdp('DOM.querySelector', { nodeId: doc.result.root.nodeId, selector: '#voice-files' });
await cdp('DOM.setFileInputFiles', { nodeId: input.result.nodeId, files: [join(temp, 'b1.wav'), join(temp, 'b3.wav')] });
await ev(`document.querySelector('#voice-files').dispatchEvent(new Event('change'))`);
await sleep(800);
const notes = await ev(`document.querySelector('#status').textContent`);
check('the voice switches to recordings', (await ev(`document.querySelector('#voice').value`)) === 'files');
check('cues without a recording are named', notes.includes('No recording for b4, b6, b9'), notes);
check('a recording longer than its cue is reported, once', notes.split('Recording b3 lasts 5 s, its cue 3 s').length === 2, notes);

// Starting inside b3 plays its recording from the offset into the cue.
await scrub(8);
await ev('window.__played = []; window.__spoken = []');
await playing(true);
await sleep(500);
const played = await ev('window.__played');
check('the recording of b3 plays', played.length === 1 && played[0].src.startsWith('blob:'), JSON.stringify(played));
check('from the offset into its cue', played.length === 1 && Math.abs(played[0].offset - (8 - 6.883)) < 0.1, JSON.stringify(played));
const cancels = await ev('window.__cancels');
await playing(false);
check('pausing silences the narration', (await ev('window.__cancels')) > cancels);

ws.close();
edge.kill();
server.close();
console.log(failed ? `${failed} checks failed` : 'all checks pass');
process.exit(failed ? 1 : 0);
