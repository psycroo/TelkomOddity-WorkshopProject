<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8"/>
<meta name="viewport" content="width=device-width, initial-scale=1.0"/>
<title>TelkomOddity — README</title>
<link href="https://fonts.googleapis.com/css2?family=Syne:wght@400;700;800&family=Space+Mono:wght@400;700&display=swap" rel="stylesheet"/>
<style>
  :root {
    --bg: #0a0a0f;
    --surface: #111118;
    --card: #16161f;
    --accent1: #ff3cac;
    --accent2: #00f5d4;
    --accent3: #ffe66d;
    --text: #eeeef5;
    --muted: #6b6b80;
    --border: rgba(255,255,255,0.07);
  }

  *, *::before, *::after { box-sizing: border-box; margin: 0; padding: 0; }

  html { scroll-behavior: smooth; }

  body {
    background: var(--bg);
    color: var(--text);
    font-family: 'Space Mono', monospace;
    min-height: 100vh;
    overflow-x: hidden;
  }

  /* ── Animated noise grain overlay ── */
  body::before {
    content: '';
    position: fixed;
    inset: 0;
    background-image: url("data:image/svg+xml,%3Csvg viewBox='0 0 200 200' xmlns='http://www.w3.org/2000/svg'%3E%3Cfilter id='n'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.9' numOctaves='4' stitchTiles='stitch'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23n)' opacity='0.04'/%3E%3C/svg%3E");
    pointer-events: none;
    z-index: 999;
    opacity: 0.35;
  }

  /* ── Grid background ── */
  .grid-bg {
    position: fixed;
    inset: 0;
    background-image:
      linear-gradient(rgba(255,60,172,0.04) 1px, transparent 1px),
      linear-gradient(90deg, rgba(255,60,172,0.04) 1px, transparent 1px);
    background-size: 40px 40px;
    pointer-events: none;
  }

  /* ── Hero ── */
  .hero {
    position: relative;
    min-height: 100vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    text-align: center;
    padding: 4rem 2rem;
    overflow: hidden;
  }

  .hero::after {
    content: '';
    position: absolute;
    width: 700px; height: 700px;
    background: radial-gradient(ellipse, rgba(255,60,172,0.15) 0%, transparent 70%);
    top: 50%; left: 50%;
    transform: translate(-50%,-50%);
    pointer-events: none;
    animation: pulse 4s ease-in-out infinite;
  }

  @keyframes pulse {
    0%,100% { transform: translate(-50%,-50%) scale(1); opacity: 0.7; }
    50%      { transform: translate(-50%,-50%) scale(1.15); opacity: 1; }
  }

  .badge {
    display: inline-block;
    background: rgba(255,60,172,0.15);
    border: 1px solid rgba(255,60,172,0.4);
    color: var(--accent1);
    font-size: 0.7rem;
    letter-spacing: 0.2em;
    text-transform: uppercase;
    padding: 0.4rem 1rem;
    border-radius: 999px;
    margin-bottom: 2rem;
    animation: fadeUp 0.6s ease both;
  }

  .hero-title {
    font-family: 'Syne', sans-serif;
    font-weight: 800;
    font-size: clamp(3.5rem, 10vw, 8rem);
    line-height: 0.9;
    letter-spacing: -0.03em;
    animation: fadeUp 0.7s ease 0.1s both;
    position: relative;
    z-index: 1;
  }

  .hero-title span.gradient {
    background: linear-gradient(135deg, var(--accent1), var(--accent2));
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }

  .hero-title span.yellow { color: var(--accent3); -webkit-text-fill-color: var(--accent3); }

  .hero-sub {
    margin-top: 1.5rem;
    font-size: clamp(0.85rem, 2vw, 1rem);
    color: var(--muted);
    max-width: 520px;
    line-height: 1.8;
    animation: fadeUp 0.7s ease 0.2s both;
    position: relative;
    z-index: 1;
  }

  .hero-chips {
    display: flex;
    flex-wrap: wrap;
    gap: 0.6rem;
    margin-top: 2rem;
    justify-content: center;
    animation: fadeUp 0.7s ease 0.3s both;
    position: relative;
    z-index: 1;
  }

  .chip {
    background: var(--card);
    border: 1px solid var(--border);
    color: var(--accent2);
    font-size: 0.72rem;
    letter-spacing: 0.1em;
    padding: 0.35rem 0.85rem;
    border-radius: 6px;
    transition: all 0.2s;
  }
  .chip:hover {
    border-color: var(--accent2);
    box-shadow: 0 0 12px rgba(0,245,212,0.2);
    transform: translateY(-2px);
  }

  .scroll-hint {
    position: absolute;
    bottom: 2.5rem;
    left: 50%;
    transform: translateX(-50%);
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.4rem;
    color: var(--muted);
    font-size: 0.65rem;
    letter-spacing: 0.15em;
    text-transform: uppercase;
    animation: fadeUp 1s ease 0.6s both;
  }
  .scroll-hint .arrow {
    width: 20px; height: 20px;
    border-right: 2px solid var(--muted);
    border-bottom: 2px solid var(--muted);
    transform: rotate(45deg);
    animation: bounce 1.5s ease infinite;
  }
  @keyframes bounce {
    0%,100% { transform: rotate(45deg) translateY(0); }
    50%      { transform: rotate(45deg) translateY(5px); }
  }

  @keyframes fadeUp {
    from { opacity: 0; transform: translateY(24px); }
    to   { opacity: 1; transform: translateY(0); }
  }

  /* ── Section ── */
  section {
    max-width: 900px;
    margin: 0 auto;
    padding: 5rem 2rem;
  }

  .section-label {
    font-size: 0.68rem;
    letter-spacing: 0.25em;
    text-transform: uppercase;
    color: var(--accent1);
    margin-bottom: 1rem;
  }

  .section-title {
    font-family: 'Syne', sans-serif;
    font-weight: 800;
    font-size: clamp(1.8rem, 4vw, 2.8rem);
    line-height: 1.1;
    margin-bottom: 1.5rem;
  }

  .section-body {
    color: var(--muted);
    line-height: 1.9;
    font-size: 0.88rem;
    max-width: 640px;
  }

  /* ── Divider ── */
  .divider {
    height: 1px;
    background: var(--border);
    max-width: 900px;
    margin: 0 auto;
  }

  /* ── Features ── */
  .features-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
    gap: 1.2rem;
    margin-top: 2.5rem;
  }

  .feature-card {
    background: var(--card);
    border: 1px solid var(--border);
    border-radius: 16px;
    padding: 1.8rem;
    position: relative;
    overflow: hidden;
    transition: transform 0.25s, border-color 0.25s, box-shadow 0.25s;
    cursor: default;
  }
  .feature-card::before {
    content: '';
    position: absolute;
    inset: 0;
    background: radial-gradient(circle at top left, var(--glow, rgba(255,60,172,0.08)), transparent 70%);
    pointer-events: none;
  }
  .feature-card:hover {
    transform: translateY(-4px);
    border-color: var(--accent-c, rgba(255,60,172,0.4));
    box-shadow: 0 12px 40px rgba(0,0,0,0.4);
  }

  .feature-card.c1 { --glow: rgba(255,60,172,0.1);  --accent-c: rgba(255,60,172,0.4); }
  .feature-card.c2 { --glow: rgba(0,245,212,0.08);  --accent-c: rgba(0,245,212,0.4); }
  .feature-card.c3 { --glow: rgba(255,230,109,0.08);--accent-c: rgba(255,230,109,0.4); }
  .feature-card.c4 { --glow: rgba(120,80,255,0.1);  --accent-c: rgba(120,80,255,0.4); }

  .feature-icon {
    font-size: 2rem;
    margin-bottom: 1rem;
    display: block;
  }
  .feature-name {
    font-family: 'Syne', sans-serif;
    font-weight: 700;
    font-size: 1rem;
    margin-bottom: 0.5rem;
    letter-spacing: -0.01em;
  }
  .feature-desc {
    font-size: 0.78rem;
    color: var(--muted);
    line-height: 1.8;
  }

  /* ── Data Structure ── */
  .struct-box {
    background: var(--card);
    border: 1px solid var(--border);
    border-radius: 16px;
    padding: 2rem;
    margin-top: 2rem;
    position: relative;
    overflow: hidden;
  }
  .struct-box::before {
    content: 'STRUCT';
    position: absolute;
    top: 1.2rem; right: 1.5rem;
    font-size: 0.6rem;
    letter-spacing: 0.2em;
    color: var(--muted);
    opacity: 0.5;
  }

  .struct-row {
    display: flex;
    align-items: baseline;
    gap: 1rem;
    padding: 0.55rem 0;
    border-bottom: 1px solid var(--border);
    font-size: 0.82rem;
  }
  .struct-row:last-child { border-bottom: none; }

  .struct-field {
    color: var(--accent2);
    min-width: 100px;
    font-weight: 700;
  }
  .struct-type {
    color: var(--accent3);
    min-width: 60px;
    font-size: 0.75rem;
  }
  .struct-desc { color: var(--muted); font-size: 0.78rem; }

  /* ── Functions ── */
  .fn-list { margin-top: 2rem; display: flex; flex-direction: column; gap: 1rem; }

  .fn-item {
    background: var(--card);
    border: 1px solid var(--border);
    border-left: 3px solid var(--accent-left, var(--accent1));
    border-radius: 0 12px 12px 0;
    padding: 1.2rem 1.5rem;
    display: flex;
    flex-wrap: wrap;
    gap: 0.8rem;
    align-items: flex-start;
    transition: background 0.2s;
  }
  .fn-item:hover { background: #1a1a25; }

  .fn-item.a1 { --accent-left: var(--accent1); }
  .fn-item.a2 { --accent-left: var(--accent2); }
  .fn-item.a3 { --accent-left: #f97316; }
  .fn-item.a4 { --accent-left: var(--accent3); }

  .fn-name {
    font-family: 'Space Mono', monospace;
    font-weight: 700;
    font-size: 0.85rem;
    color: var(--text);
    min-width: 200px;
  }
  .fn-name .kw { color: var(--accent1); margin-right: 4px; }
  .fn-badge {
    font-size: 0.65rem;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    padding: 0.2rem 0.6rem;
    border-radius: 999px;
    border: 1px solid;
    margin-left: 0.5rem;
    vertical-align: middle;
  }
  .fn-badge.read  { color: var(--accent2); border-color: rgba(0,245,212,0.4); }
  .fn-badge.write { color: var(--accent1); border-color: rgba(255,60,172,0.4); }

  .fn-desc { font-size: 0.78rem; color: var(--muted); line-height: 1.7; flex: 1; min-width: 200px; }

  /* ── Tech stack ── */
  .stack-list {
    display: flex;
    flex-wrap: wrap;
    gap: 0.8rem;
    margin-top: 1.8rem;
  }
  .stack-item {
    background: var(--card);
    border: 1px solid var(--border);
    border-radius: 10px;
    padding: 0.8rem 1.2rem;
    font-size: 0.78rem;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    transition: border-color 0.2s, transform 0.2s;
  }
  .stack-item:hover {
    border-color: rgba(255,60,172,0.3);
    transform: translateY(-2px);
  }
  .stack-item .dot {
    width: 8px; height: 8px;
    border-radius: 50%;
    background: var(--dot-c, var(--accent1));
    flex-shrink: 0;
  }

  /* ── Footer ── */
  footer {
    border-top: 1px solid var(--border);
    text-align: center;
    padding: 3rem 2rem;
    color: var(--muted);
    font-size: 0.72rem;
    letter-spacing: 0.05em;
    line-height: 2;
  }
  footer span { color: var(--accent1); }
</style>
</head>
<body>

<div class="grid-bg"></div>

<!-- ═══════════════ HERO ═══════════════ -->
<div class="hero">
  <div class="badge">⚡ Built on Stellar · Powered by Soroban</div>

  <h1 class="hero-title">
    <span class="gradient">Telkom</span><br>
    <span class="yellow">Oddity</span>
  </h1>

  <p class="hero-sub">
    A decentralized weird-event reporting system for Telkom University students.
    Nothing gets lost. Nothing gets covered up. Everything lives on-chain — forever.
  </p>

  <div class="hero-chips">
    <div class="chip">#SmartContract</div>
    <div class="chip">#Soroban</div>
    <div class="chip">#StellarBlockchain</div>
    <div class="chip">#Rust</div>
    <div class="chip">#Web3</div>
    <div class="chip">#TelkomUniversity</div>
  </div>

  <div class="scroll-hint">
    <span>scroll</span>
    <div class="arrow"></div>
  </div>
</div>

<!-- ═══════════════ ABOUT ═══════════════ -->
<div class="divider"></div>
<section>
  <p class="section-label">// 01 — About</p>
  <h2 class="section-title">What even is this?</h2>
  <p class="section-body">
    <strong style="color:var(--text)">TelkomOddity</strong> is a smart contract deployed on the
    <strong style="color:var(--accent2)">Stellar blockchain</strong> using the
    <strong style="color:var(--accent1)">Soroban</strong> framework.
    It lets Telkom University students permanently record bizarre, unique, or hilarious incidents
    that happen on campus — tied to their NIM and name.
    <br><br>
    No servers. No admins. No delete button (unless you own it). Every report is
    stored immutably on a decentralized ledger. Because weird things deserve to be remembered.
  </p>
</section>

<!-- ═══════════════ FEATURES ═══════════════ -->
<div class="divider"></div>
<section>
  <p class="section-label">// 02 — Features</p>
  <h2 class="section-title">What it can do</h2>

  <div class="features-grid">
    <div class="feature-card c1">
      <span class="feature-icon">📡</span>
      <div class="feature-name">Submit a Report</div>
      <div class="feature-desc">
        Drop your NIM, name, location, description, and category. Your incident gets written to the blockchain permanently.
      </div>
    </div>

    <div class="feature-card c2">
      <span class="feature-icon">📋</span>
      <div class="feature-name">View All Reports</div>
      <div class="feature-desc">
        Fetch the entire on-chain report list. Scroll through Telkom's wildest moments, preserved forever.
      </div>
    </div>

    <div class="feature-card c3">
      <span class="feature-icon">🗑️</span>
      <div class="feature-name">Delete by ID</div>
      <div class="feature-desc">
        Made a mistake? Use the report ID to remove it before it haunts you for eternity. Act fast.
      </div>
    </div>

    <div class="feature-card c4">
      <span class="feature-icon">📊</span>
      <div class="feature-name">Count Reports</div>
      <div class="feature-desc">
        Instantly check how many weird events have been submitted across the entire campus. The number goes up.
      </div>
    </div>
  </div>
</section>

<!-- ═══════════════ DATA STRUCTURE ═══════════════ -->
<div class="divider"></div>
<section>
  <p class="section-label">// 03 — Data Structure</p>
  <h2 class="section-title">What gets stored on-chain</h2>
  <p class="section-body">Each report is a <code style="color:var(--accent2)">Laporan</code> struct stored in Soroban instance storage.</p>

  <div class="struct-box">
    <div class="struct-row">
      <span class="struct-field">id</span>
      <span class="struct-type">u64</span>
      <span class="struct-desc">Auto-generated unique ID via Soroban PRNG</span>
    </div>
    <div class="struct-row">
      <span class="struct-field">nim</span>
      <span class="struct-type">String</span>
      <span class="struct-desc">Student ID number of the reporter</span>
    </div>
    <div class="struct-row">
      <span class="struct-field">nama</span>
      <span class="struct-type">String</span>
      <span class="struct-desc">Full name of the reporter</span>
    </div>
    <div class="struct-row">
      <span class="struct-field">lokasi</span>
      <span class="struct-type">String</span>
      <span class="struct-desc">Location where the incident happened on campus</span>
    </div>
    <div class="struct-row">
      <span class="struct-field">kejadian</span>
      <span class="struct-type">String</span>
      <span class="struct-desc">Description of the weird / unique / funny event</span>
    </div>
    <div class="struct-row">
      <span class="struct-field">kategori</span>
      <span class="struct-type">String</span>
      <span class="struct-desc">"ANEH" · "UNIK" · "LUCU" — pick your vibe</span>
    </div>
  </div>
</section>

<!-- ═══════════════ FUNCTIONS ═══════════════ -->
<div class="divider"></div>
<section>
  <p class="section-label">// 04 — Contract Functions</p>
  <h2 class="section-title">The actual on-chain API</h2>

  <div class="fn-list">
    <div class="fn-item a2">
      <div class="fn-name">
        <span class="kw">fn</span> get_all_laporan()
        <span class="fn-badge read">READ</span>
      </div>
      <div class="fn-desc">Returns the full list of all incident reports stored on the blockchain. No gas cost (read-only).</div>
    </div>

    <div class="fn-item a1">
      <div class="fn-name">
        <span class="kw">fn</span> buat_laporan(...)
        <span class="fn-badge write">WRITE</span>
      </div>
      <div class="fn-desc">Takes nim, nama, lokasi, kejadian, kategori — creates a new report and pushes it to on-chain storage. Costs a tiny XLM transaction fee.</div>
    </div>

    <div class="fn-item a3">
      <div class="fn-name">
        <span class="kw">fn</span> hapus_laporan(id)
        <span class="fn-badge write">WRITE</span>
      </div>
      <div class="fn-desc">Finds and removes a report by its unique ID. Returns a confirmation string or a not-found message.</div>
    </div>

    <div class="fn-item a4">
      <div class="fn-name">
        <span class="kw">fn</span> hitung_laporan()
        <span class="fn-badge read">READ</span>
      </div>
      <div class="fn-desc">Returns the total count of reports currently stored. Useful for dashboards or stats displays.</div>
    </div>
  </div>
</section>

<!-- ═══════════════ TECH STACK ═══════════════ -->
<div class="divider"></div>
<section>
  <p class="section-label">// 05 — Tech Stack</p>
  <h2 class="section-title">What's under the hood</h2>

  <div class="stack-list">
    <div class="stack-item">
      <div class="dot" style="--dot-c:#f97316"></div>
      Rust (no_std)
    </div>
    <div class="stack-item">
      <div class="dot" style="--dot-c:var(--accent1)"></div>
      Soroban SDK
    </div>
    <div class="stack-item">
      <div class="dot" style="--dot-c:var(--accent2)"></div>
      Stellar Blockchain
    </div>
    <div class="stack-item">
      <div class="dot" style="--dot-c:var(--accent3)"></div>
      WebAssembly (WASM)
    </div>
    <div class="stack-item">
      <div class="dot" style="--dot-c:#a78bfa"></div>
      Instance Storage
    </div>
    <div class="stack-item">
      <div class="dot" style="--dot-c:#34d399"></div>
      Soroban PRNG (ID gen)
    </div>
  </div>

  <p class="section-body" style="margin-top:2rem;">
    Every function call that <em>writes</em> to the blockchain costs a small
    <strong style="color:var(--accent3)">XLM transaction fee</strong> (gas fee) —
    typically less than <strong style="color:var(--accent3)">0.00001 XLM</strong>.
    Read-only calls are free. Data stored via Soroban's instance storage persists
    on-chain and cannot be altered by any third party.
  </p>
</section>

<!-- ═══════════════ FOOTER ═══════════════ -->
<div class="divider"></div>
<footer>
  <p>Built with <span>♥</span> on the Stellar Network · Soroban Smart Contract</p>
  <p style="margin-top:0.5rem;">Telkom University · <span>TelkomOddity</span> · On-chain since day one</p>
  <p style="margin-top:0.5rem; opacity:0.4;">The blockchain never forgets. Neither do we.</p>
</footer>

</body>
</html>