// ── Hiking Path Animation ──────────────────────────────────────────────────
// Animated Go-stone fish logo swimming along a winding path in the hero section.

(function () {
  const PATH_D =
    'M-20 520 C120 480 200 400 340 380 S520 420 640 360 S820 280 960 300 S1120 340 1250 280';

  // Speed curve — pace multiplier keyed to 0→1 progress
  const PACE = [
    [0.0, 0.4],
    [0.1, 1.0],
    [0.22, 0.5],
    [0.3, 0.3],
    [0.38, 0.9],
    [0.45, 1.2],
    [0.52, 0.6],
    [0.6, 0.35],
    [0.68, 1.0],
    [0.75, 1.3],
    [0.83, 0.7],
    [0.92, 0.5],
    [1.0, 0.4],
  ];

  function getPaceAt(t) {
    for (let i = 0; i < PACE.length - 1; i++) {
      const [t0, v0] = PACE[i];
      const [t1, v1] = PACE[i + 1];
      if (t >= t0 && t <= t1) {
        const f = (t - t0) / (t1 - t0);
        const s = f * f * (3 - 2 * f); // smoothstep
        return v0 + s * (v1 - v0);
      }
    }
    return 0.5;
  }

  const EMOTIONS = {
    climbing: ['😤', '🥵', '💪', '😰', '🫠', '😮‍💨'],
    descending: ['😄', '🏃', '🤸', '😜', '🥳'],
    flat: ['😌', '🎶', '🌿', '😊', '🤔', '👀'],
    summit: ['🎉', '🙌', '😭', '🥹', '🏔️', '🤩'],
    trailhead: ['😴', '🧃', '🥾', '😅', '💤'],
  };

  function pickEmotion(paceMultiplier, atEnd, atStart) {
    if (atEnd)
      return EMOTIONS.summit[
        Math.floor(Math.random() * EMOTIONS.summit.length)
      ];
    if (atStart)
      return EMOTIONS.trailhead[
        Math.floor(Math.random() * EMOTIONS.trailhead.length)
      ];
    const pool =
      paceMultiplier < 0.6
        ? EMOTIONS.climbing
        : paceMultiplier > 0.9
        ? EMOTIONS.descending
        : EMOTIONS.flat;
    return pool[Math.floor(Math.random() * pool.length)];
  }

  // Fish SVG markup — scaled-down Go-stone logo
  const FISH_SVG = '\
    <g transform="scale(-.0556,.0556) translate(-180,-180)">\
      <g class="hiking-fish">\
        <g fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" class="fish-lines">\
          <path d="M206.6 97.8 L258.5 150.7"/>\
          <path d="M109.7 0.8 L180.5 71.6"/>\
          <path d="M14.2 96.2 L85.0 167.1"/>\
          <path d="M111.2 193.2 L180.6 262.5"/>\
          <path d="M180.9 98.6 L112.0 167.4"/>\
          <path d="M85.8 193.5 L15.5 263.8"/>\
          <path d="M260.4 211.0 L206.9 263.3"/>\
          <path d="M180.9 289.5 L110.9 359.3"/>\
        </g>\
        <g fill="none" stroke="currentColor" stroke-width="2" class="fish-stones">\
          <circle cx="98.4" cy="179.9" r="16" transform="rotate(-45 98.4 179.9)"/>\
          <circle cx="193.4" cy="275.9" r="16" transform="rotate(-45 193.4 275.9)"/>\
          <circle cx="193.4" cy="84.9" r="16" transform="rotate(-45 193.4 84.9)"/>\
        </g>\
        <circle cx="289.4" cy="179.9" r="40" transform="rotate(-45 289.4 179.9)" fill="currentColor" class="fish-stone-filled"/>\
      </g>\
    </g>';

  function init() {
    const container = document.getElementById('hiking-path-container');
    if (!container) return;

    // Build SVG
    const svgNS = 'http://www.w3.org/2000/svg';
    const svg = document.createElementNS(svgNS, 'svg');
    svg.setAttribute('viewBox', '0 0 1200 600');
    svg.setAttribute('fill', 'none');
    svg.setAttribute('aria-hidden', 'true');
    svg.setAttribute('preserveAspectRatio', 'xMidYMid slice');
    svg.style.width = '100%';
    svg.style.height = '100%';

    // Ghost path
    const ghostPath = document.createElementNS(svgNS, 'path');
    ghostPath.setAttribute('d', PATH_D);
    ghostPath.setAttribute('stroke', 'currentColor');
    ghostPath.setAttribute('stroke-width', '1');
    ghostPath.setAttribute('stroke-linecap', 'round');
    ghostPath.setAttribute('stroke-dasharray', '5 11');
    ghostPath.setAttribute('opacity', '0.07');
    svg.appendChild(ghostPath);

    // Trail mask defs
    const defs = document.createElementNS(svgNS, 'defs');
    const mask = document.createElementNS(svgNS, 'mask');
    mask.setAttribute('id', 'trail-mask-' + Math.random().toString(36).slice(2, 8));
    const maskPath = document.createElementNS(svgNS, 'path');
    maskPath.setAttribute('d', PATH_D);
    maskPath.setAttribute('stroke', 'white');
    maskPath.setAttribute('stroke-width', '6');
    maskPath.setAttribute('stroke-linecap', 'round');
    maskPath.setAttribute('stroke-dasharray', '0 9999');
    mask.appendChild(maskPath);
    defs.appendChild(mask);
    svg.appendChild(defs);

    // Trail
    const trailPath = document.createElementNS(svgNS, 'path');
    trailPath.setAttribute('d', PATH_D);
    trailPath.setAttribute('stroke', 'currentColor');
    trailPath.setAttribute('stroke-width', '1.5');
    trailPath.setAttribute('stroke-linecap', 'round');
    trailPath.setAttribute('stroke-dasharray', '5 11');
    trailPath.setAttribute('opacity', '0.18');
    trailPath.setAttribute('mask', 'url(#' + mask.getAttribute('id') + ')');
    svg.appendChild(trailPath);

    // Glow
    const glow = document.createElementNS(svgNS, 'circle');
    glow.setAttribute('cx', '-20');
    glow.setAttribute('cy', '520');
    glow.setAttribute('r', '2.5');
    glow.setAttribute('fill', '#a09d98');
    glow.setAttribute('opacity', '0.35');
    svg.appendChild(glow);

    // Fish group
    const dotGroup = document.createElementNS(svgNS, 'g');
    dotGroup.setAttribute('transform', 'translate(-20,520)');
    dotGroup.innerHTML = FISH_SVG;
    svg.appendChild(dotGroup);

    // Emotion label
    const label = document.createElementNS(svgNS, 'text');
    label.setAttribute('style', 'display:none');
    svg.appendChild(label);

    container.appendChild(svg);

    // ── Animation loop ──────────────────────────────────────────────
    const total = ghostPath.getTotalLength();
    let progress = 0;
    let dir = 1; // 1 = forward, -1 = return
    let lastTime = null;
    let pause = 0;
    let emotionTimer = 0;
    let currentEmotion = '😊';
    let rafId;

    function step(ts) {
      if (lastTime === null) lastTime = ts;
      const dt = Math.min(ts - lastTime, 64) / 1000;
      lastTime = ts;

      if (pause > 0) {
        pause -= dt;
      } else {
        const paceT = dir === 1 ? progress : 1 - progress;
        const pace = getPaceAt(paceT);

        progress += dir * (dt / 18) * pace;
        progress += (Math.random() - 0.5) * 0.0008;

        let atEnd = false;
        let atStart = false;
        if (progress >= 1) {
          progress = 1;
          dir = -1;
          lastTime = null;
          pause = 1.8;
          atEnd = true;
        } else if (progress <= 0) {
          progress = 0;
          dir = 1;
          lastTime = null;
          pause = 1.2;
          atStart = true;
        }

        if (atEnd || atStart) {
          currentEmotion = pickEmotion(pace, atEnd, atStart);
          emotionTimer = 0;
        } else {
          emotionTimer -= dt;
          if (emotionTimer <= 0) {
            currentEmotion = pickEmotion(pace, false, false);
            emotionTimer = 3 + Math.random() * 4;
          }
        }

        const t = Math.max(0, Math.min(1, progress));
        const dist = t * total;
        const pt = ghostPath.getPointAtLength(dist);
        const bob = Math.sin(ts / 220) * 1.8;

        const cx = String(pt.x);
        const cy = String(pt.y + bob);
        const flipX = dir === 1 ? -1 : 1;
        dotGroup.setAttribute(
          'transform',
          `translate(${cx},${cy}) scale(${flipX},1)`
        );
        glow.setAttribute('cx', cx);
        glow.setAttribute('cy', cy);
        label.setAttribute('x', cx);
        label.setAttribute(
          'y',
          String(pt.y + bob - 10)
        );
        label.textContent = currentEmotion;

        maskPath.style.strokeDasharray = `${dist} ${total}`;
      }

      rafId = requestAnimationFrame(step);
    }

    rafId = requestAnimationFrame(step);
  }

  // Run on page load and after instant navigation
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', init);
  } else {
    init();
  }

  // Re-init after Zensical instant navigation
  document$.subscribe(init);
})();
