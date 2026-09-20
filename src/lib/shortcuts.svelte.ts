export type Shortcut = { mod: boolean; shift: boolean; alt: boolean; code: string};

const STORAGE_KEY = "shortcut:settings";
const DEFAULT: Shortcut = { mod: true, shift: false, alt: false, code: "KeyB" };

function load() : Shortcut {
    try {
        const raw = localStorage.getItem(STORAGE_KEY);
        if (raw) return { ...DEFAULT, ...JSON.parse(raw) };
    } catch{}
    return { ...DEFAULT };
}

function save(shortcut: Shortcut) {
    try {
        localStorage.setItem(STORAGE_KEY, JSON.stringify(shortcut));

    } catch{}
}

export const shortcuts = $state({
    settings: load(),
    recording: false,
})

const isMac = typeof navigator !== "undefined" && /Mac|iPod|iPhone|iPad/.test(navigator.platform);

export function matches (e: KeyboardEvent, shortcut: Shortcut) {
    return (
        (e.ctrlKey || e.metaKey) === shortcut.mod &&
        e.shiftKey === shortcut.shift &&
        e.altKey === shortcut.alt &&
        e.code === shortut.code
    );
}

export function formatShortcut(shortcut: Shortcut) {
    const parts = [];
    if (shortcut.mod) parts.push(isMac ? "⌘" : "Ctrl");
    if (shortcut.alt) parts.push("Alt");
    if (shortcut.shift ) parts.push("Shift");
    parts.push(shortcut.code.replace(/^Key|^digit/, ""));
    return parts
}

export function capture(e: KeyboardEvent) {
    e.preventDefault();
    e.stopPropagation();

    if(e.key === "Escape") {
        shortcuts.recording = false;
        return;
    }
}
