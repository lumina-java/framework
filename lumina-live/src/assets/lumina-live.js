/**
 * ⚡ Lumina Live Micro-Runtime (v0.4.0)
 * Server-Driven Reactive UI for Lumina Framework (Rust)
 * Supports Tri-Dialect Directives:
 *   - 🇮🇩 Indonesia: kilat:* (klik, kirim, model, tunggu, yakin, pantau, etc.)
 *   - 🌐 Universal:  live:*  (click, submit, model, loading, confirm, poll, etc.)
 *   - 🅰️ Legacy:     wire:*  (click, submit, model, loading, confirm, poll, etc.)
 * Zero external dependencies (~6.5 KB) with Idiomorph DOM Morphing
 */
(() => {
    'use strict';

    // Directives normalization table
    const DIRECTIVES = {
        'klik': 'click', 'click': 'click',
        'kirim': 'submit', 'submit': 'submit',
        'model': 'model',
        'ikat': 'bind', 'bind': 'bind',
        'tunggu': 'loading', 'loading': 'loading',
        'target': 'target',
        'navigasi': 'navigate', 'navigate': 'navigate',
        'aktif': 'current', 'current': 'current',
        'selubung': 'cloak', 'cloak': 'cloak',
        'ubah': 'dirty', 'dirty': 'dirty',
        'yakin': 'confirm', 'confirm': 'confirm',
        'transisi': 'transition', 'transition': 'transition',
        'awal': 'init', 'init': 'init',
        'tampak': 'intersect', 'intersect': 'intersect',
        'pantau': 'poll', 'poll': 'poll',
        'luring': 'offline', 'offline': 'offline',
        'abaikan': 'ignore', 'ignore': 'ignore',
        'rujukan': 'ref', 'ref': 'ref',
        'ganti': 'replace', 'replace': 'replace',
        'tampil': 'show', 'show': 'show',
        'urut': 'sort', 'sort': 'sort',
        'alir': 'stream', 'stream': 'stream'
    };

    const PREFIXES = ['kilat:', 'live:', 'wire:'];

    // Helper: Find directive on element matching any dialect prefix
    function getDirective(el, canonicalName) {
        for (const [key, val] of Object.entries(DIRECTIVES)) {
            if (val === canonicalName) {
                for (const p of PREFIXES) {
                    const attr = p + key;
                    if (el.hasAttribute(attr)) {
                        return { name: attr, value: el.getAttribute(attr) };
                    }
                    // check modifiers like kilat:pantau.5d or live:poll.5s
                    for (const a of el.attributes) {
                        if (a.name.startsWith(attr + '.') || a.name === attr) {
                            return { name: a.name, value: a.value };
                        }
                    }
                }
            }
        }
        return null;
    }

    // Helper: Find closest live component root
    function getComponentRoot(el) {
        return el.closest('[data-kilat-id], [data-live-id], [data-wire-id]');
    }

    // Fast Idiomorph DOM Morpher (Preserves focus, caret selection, and input states)
    function morph(target, source) {
        if (!target || !source) return;

        // Save active element focus and selection
        const active = document.activeElement;
        const activeId = active ? active.id : null;
        const activeName = active ? active.name : null;
        let selStart = null, selEnd = null;
        if (active && (active.tagName === 'INPUT' || active.tagName === 'TEXTAREA')) {
            try {
                selStart = active.selectionStart;
                selEnd = active.selectionEnd;
            } catch (e) {}
        }

        morphElement(target, source);

        // Restore focus and selection
        if (activeId || activeName) {
            const restored = (activeId && document.getElementById(activeId)) ||
                             (activeName && document.querySelector(`[name="${activeName}"]`));
            if (restored && typeof restored.focus === 'function') {
                restored.focus();
                if (selStart !== null && selEnd !== null) {
                    try { restored.setSelectionRange(selStart, selEnd); } catch (e) {}
                }
            }
        }
    }

    function morphElement(oldNode, newNode) {
        if (oldNode.nodeType !== newNode.nodeType || oldNode.nodeName !== newNode.nodeName) {
            oldNode.replaceWith(newNode.cloneNode(true));
            return;
        }

        // Handle text nodes
        if (oldNode.nodeType === Node.TEXT_NODE) {
            if (oldNode.textContent !== newNode.textContent) {
                oldNode.textContent = newNode.textContent;
            }
            return;
        }

        if (oldNode.nodeType !== Node.ELEMENT_NODE) return;

        // Skip ignored nodes (kilat:abaikan / live:ignore / wire:ignore)
        if (getDirective(oldNode, 'ignore')) return;

        // Sync attributes
        const oldAttrs = oldNode.attributes;
        const newAttrs = newNode.attributes;

        for (let i = newAttrs.length - 1; i >= 0; i--) {
            const attr = newAttrs[i];
            if (oldNode.getAttribute(attr.name) !== attr.value) {
                oldNode.setAttribute(attr.name, attr.value);
            }
        }

        for (let i = oldAttrs.length - 1; i >= 0; i--) {
            const attr = oldAttrs[i];
            if (!newNode.hasAttribute(attr.name)) {
                oldNode.removeAttribute(attr.name);
            }
        }

        // Sync input values
        if (oldNode.tagName === 'INPUT' || oldNode.tagName === 'TEXTAREA') {
            if (oldNode.type === 'checkbox' || oldNode.type === 'radio') {
                oldNode.checked = newNode.checked;
            } else if (oldNode.value !== newNode.value) {
                if (oldNode !== document.activeElement) {
                    oldNode.value = newNode.value;
                } else if (newNode.value === '') {
                    oldNode.value = '';
                }
            }
        } else if (oldNode.tagName === 'SELECT') {
            if (oldNode.value !== newNode.value) {
                oldNode.value = newNode.value;
            }
        }

        // Sync children
        const oldChildren = Array.from(oldNode.childNodes);
        const newChildren = Array.from(newNode.childNodes);

        const max = Math.max(oldChildren.length, newChildren.length);
        for (let i = 0; i < max; i++) {
            const oldChild = oldChildren[i];
            const newChild = newChildren[i];

            if (!oldChild && newChild) {
                oldNode.appendChild(newChild.cloneNode(true));
            } else if (oldChild && !newChild) {
                oldChild.remove();
            } else if (oldChild && newChild) {
                morphElement(oldChild, newChild);
            }
        }
    }

    // Core Lumina Live Client (Dual Transport: WebSocket Streaming + HTTP Micro-RPC)
    class LuminaLive {
        constructor() {
            this.endpoint = '/_lumina/live/call';
            this.ws = null;
            this.pendingCalls = new Map();
        }

        init() {
            // Remove cloaks
            document.querySelectorAll('[kilat\\:selubung], [live\\:cloak], [wire\\:cloak]').forEach(el => {
                for (const p of PREFIXES) {
                    el.removeAttribute(p + 'selubung');
                    el.removeAttribute(p + 'cloak');
                }
            });

            this.initWebSocket();
            this.setupEventListeners();
            this.setupPolling();
            this.setupOffline();
            this.setupInitHooks();
            this.setupNavigation();
        }

        initWebSocket() {
            if (typeof window === 'undefined' || !window.WebSocket) return;
            try {
                const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
                const wsUrl = `${protocol}//${window.location.host}/_lumina/live/ws`;
                const socket = new WebSocket(wsUrl);

                socket.onopen = () => {
                    this.ws = socket;
                };

                socket.onmessage = (event) => {
                    try {
                        const data = JSON.parse(event.data);
                        if (data.id && data.html) {
                            this.applyUpdate(data);
                        }
                    } catch (e) {
                        console.error('[Lumina Live WS Parse Error]', e);
                    }
                };

                socket.onclose = () => {
                    this.ws = null;
                };

                socket.onerror = () => {
                    this.ws = null;
                };
            } catch (e) {
                this.ws = null;
            }
        }

        applyUpdate(data) {
            const componentEl = document.querySelector(`[data-kilat-id="${data.id}"], [data-live-id="${data.id}"], [data-wire-id="${data.id}"]`);
            if (!componentEl) return;

            const temp = document.createElement('div');
            temp.innerHTML = data.html.trim();
            const newRoot = temp.firstElementChild || temp;

            morph(componentEl, newRoot);

            if (data.snapshot) {
                componentEl.setAttribute('data-kilat-snapshot', data.snapshot);
                if (componentEl.hasAttribute('data-live-snapshot')) componentEl.setAttribute('data-live-snapshot', data.snapshot);
                if (componentEl.hasAttribute('data-wire-snapshot')) componentEl.setAttribute('data-wire-snapshot', data.snapshot);
            }

            this.setLoading(componentEl, false);
        }

        async callAction(componentEl, action, params = [], explicitUpdates = null) {
            const id = componentEl.getAttribute('data-kilat-id') || componentEl.getAttribute('data-live-id') || componentEl.getAttribute('data-wire-id');
            const name = componentEl.getAttribute('data-kilat-name') || componentEl.getAttribute('data-live-name') || componentEl.getAttribute('data-wire-name');
            const snapshot = componentEl.getAttribute('data-kilat-snapshot') || componentEl.getAttribute('data-live-snapshot') || componentEl.getAttribute('data-wire-snapshot');

            if (!id || !name || !snapshot) {
                console.error('[Lumina Live] Root component attributes missing.');
                return;
            }

            // Collect all current model values from component inputs (deferred by default ala Livewire v3)
            const updates = explicitUpdates || {};
            if (!explicitUpdates) {
                componentEl.querySelectorAll('[kilat\\:model], [live\\:model], [wire\\:model]').forEach(el => {
                    const dir = getDirective(el, 'model');
                    if (!dir) return;
                    const prop = dir.value;
                    const val = el.type === 'checkbox' ? el.checked : el.value;
                    updates[prop] = val;
                });
            }

            // Set loading states
            this.setLoading(componentEl, true, action);

            const payload = { id, name, snapshot, action, params, updates };

            // 1. Jalur Utama: WebSocket Streaming (< 1ms Latency)
            if (this.ws && this.ws.readyState === WebSocket.OPEN) {
                try {
                    this.ws.send(JSON.stringify(payload));
                    return;
                } catch (e) {
                    console.warn('[Lumina Live WS fallback to HTTP]', e);
                }
            }

            // 2. Jalur Cadangan: HTTP Micro-RPC (Stateless fallback)
            try {
                const res = await fetch(this.endpoint, {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify(payload)
                });

                if (!res.ok) {
                    const err = await res.text();
                    console.error('[Lumina Live Action Error]', err);
                    return;
                }

                const data = await res.json();
                if (data.html) {
                    this.applyUpdate(data);
                }
            } catch (err) {
                console.error('[Lumina Live Network Error]', err);
            } finally {
                this.setLoading(componentEl, false, action);
            }
        }

        setLoading(componentEl, isLoading, actionName) {
            const loadings = componentEl.querySelectorAll('[kilat\\:tunggu], [live\\:loading], [wire\\:loading]');
            loadings.forEach(el => {
                const target = getDirective(el, 'target');
                if (target && target.value !== actionName) return;

                if (isLoading) {
                    el.style.display = el.dataset.origDisplay || '';
                    el.classList.add('lumina-loading');
                } else {
                    if (!el.dataset.origDisplay) el.dataset.origDisplay = el.style.display;
                    el.style.display = 'none';
                    el.classList.remove('lumina-loading');
                }
            });
        }

        setupEventListeners() {
            // Click Handler
            document.addEventListener('click', (e) => {
                const target = e.target.closest('[kilat\\:klik], [live\\:click], [wire\\:click]');
                if (!target) return;

                const dir = getDirective(target, 'click');
                if (!dir) return;

                e.preventDefault();

                // Check confirm dialog
                const confirmDir = getDirective(target, 'confirm');
                if (confirmDir && !window.confirm(confirmDir.value)) {
                    return;
                }

                const root = getComponentRoot(target);
                if (!root) return;

                const { action, params } = this.parseAction(dir.value);
                this.callAction(root, action, params);
            });

            // Submit Handler
            document.addEventListener('submit', (e) => {
                const form = e.target;
                const dir = getDirective(form, 'submit');
                if (!dir) return;

                e.preventDefault();

                const root = getComponentRoot(form);
                if (!root) return;

                const { action, params } = this.parseAction(dir.value);
                this.callAction(root, action, params);
            });

            // Model / Two-Way Binding Handler (Deferred by default ala Livewire v3)
            const inputDebounces = new Map();
            const handleModel = (e) => {
                const target = e.target;
                const dir = getDirective(target, 'model');
                if (!dir) return;

                const root = getComponentRoot(target);
                if (!root) return;

                const propName = dir.value;
                let val = target.type === 'checkbox' ? target.checked : target.value;

                const isLive = dir.name.includes('.live') || dir.name.includes('.langsung');
                const isBlur = dir.name.includes('.blur') || dir.name.includes('.ubah');

                // DEFAULT: DEFERRED (Livewire v3 standard)
                // When typing into standard kilat:model, DO NOT send network request on every key!
                // It is automatically bundled and synced when any action or form submit is executed!
                if (e.type === 'input') {
                    if (!isLive) return;
                }

                if (e.type === 'change' && isLive) {
                    return;
                }

                // Extract debounce duration (e.g. kilat:tunda.300ms or live:model.debounce.500ms)
                let delay = isLive ? 250 : 0;
                for (const attr of target.attributes) {
                    const match = attr.name.match(/\.(?:debounce|tunda)\.(\d+)(ms|s)?/);
                    if (match) {
                        delay = match[2] === 's' ? parseInt(match[1]) * 1000 : parseInt(match[1]);
                        break;
                    }
                }

                if (inputDebounces.has(target)) clearTimeout(inputDebounces.get(target));
                const sendUpdate = () => {
                    const updates = {};
                    updates[propName] = val;
                    this.callAction(root, '$set', [propName, val], updates);
                };

                if (delay > 0) {
                    inputDebounces.set(target, setTimeout(sendUpdate, delay));
                } else {
                    sendUpdate();
                }
            };

            document.addEventListener('input', handleModel);
            document.addEventListener('change', handleModel);
        }

        setupPolling() {
            document.querySelectorAll('[kilat\\:pantau], [live\\:poll], [wire\\:poll]').forEach(el => {
                const root = getComponentRoot(el);
                if (!root) return;

                let dir = null;
                for (const a of el.attributes) {
                    if (a.name.includes('pantau') || a.name.includes('poll')) {
                        dir = a; break;
                    }
                }
                if (!dir) return;

                let interval = 5000;
                const match = dir.name.match(/\.(\d+)(d|s|ms)?/);
                if (match) {
                    const unit = match[2];
                    const num = parseInt(match[1]);
                    interval = (unit === 'd' || unit === 's') ? num * 1000 : num;
                }

                const action = dir.value || '$refresh';
                setInterval(() => {
                    if (document.contains(root)) {
                        this.callAction(root, action, []);
                    }
                }, interval);
            });
        }

        setupOffline() {
            const check = () => {
                const off = !navigator.onLine;
                document.querySelectorAll('[kilat\\:luring], [live\\:offline], [wire\\:offline]').forEach(el => {
                    el.style.display = off ? '' : 'none';
                });
            };
            window.addEventListener('online', check);
            window.addEventListener('offline', check);
            check();
        }

        setupInitHooks() {
            document.querySelectorAll('[kilat\\:awal], [live\\:init], [wire\\:init]').forEach(el => {
                const root = getComponentRoot(el);
                const dir = getDirective(el, 'init');
                if (root && dir) {
                    const { action, params } = this.parseAction(dir.value);
                    this.callAction(root, action, params);
                }
            });
        }

        setupNavigation() {
            // SPA Navigation (kilat:navigasi / live:navigate / wire:navigate)
            document.addEventListener('click', (e) => {
                const a = e.target.closest('a[kilat\\:navigasi], a[live\\:navigate], a[wire\\:navigate]');
                if (!a || !a.href || a.target === '_blank' || a.origin !== window.location.origin) return;

                e.preventDefault();
                window.history.pushState({}, '', a.href);
                fetch(a.href)
                    .then(res => res.text())
                    .then(html => {
                        const doc = new DOMParser().parseFromString(html, 'text/html');
                        morph(document.body, doc.body);
                        document.title = doc.title;
                        this.init();
                    })
                    .catch(() => { window.location.href = a.href; });
            });

            window.addEventListener('popstate', () => {
                fetch(window.location.href)
                    .then(res => res.text())
                    .then(html => {
                        const doc = new DOMParser().parseFromString(html, 'text/html');
                        morph(document.body, doc.body);
                        document.title = doc.title;
                        this.init();
                    });
            });
        }

        parseAction(raw) {
            const match = raw.match(/^([a-zA-Z0-9_]+)(?:\((.*)\))?$/);
            if (!match) return { action: raw, params: [] };

            const action = match[1];
            let params = [];
            if (match[2]) {
                try {
                    params = JSON.parse(`[${match[2]}]`);
                } catch (e) {
                    params = match[2].split(',').map(s => s.trim().replace(/^['"]|['"]$/g, ''));
                }
            }
            return { action, params };
        }
    }

    // Auto-initialize when DOM is ready
    window.LuminaLive = new LuminaLive();
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', () => window.LuminaLive.init());
    } else {
        window.LuminaLive.init();
    }
})();
