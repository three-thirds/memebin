export type Shortcut = { mod: boolean; shift: boolean; alt: boolean; code: string};

const STORAGE_KEY = "shortcuts:v1";

export const DEFAULTS = {
    settings: { mod: true, shift: false, alt: false, code: "KeyB" },
    popup: { mod: true, shift: true, alt: false, code: "KeyM" },
} satisfies Record<string, Shortcut>;

export type ActionId = keyof typeof DEFAULTS;

export const LABELS: Record<ActionId, string> = {
    settings: "Open Settings",
    popup: "Open Popup"
}

function load() : Record<ActionId, Shortcut> {
    try {
        const raw = localStorage.getItem(STORAGE_KEY);
        if (raw) return { ...structuredClone(DEFAULTS), ...JSON.parse(raw) };
    } catch{}
    return structuredClone(DEFAULTS);
}

function save() {
    try {
        localStorage.setItem(STORAGE_KEY, JSON.stringify(shortcuts.bindings));

    } catch{}
}

export const shortcuts = $state({
    bindings: load(),
    recording: null as ActionId | null,
    error: ""
})

const isMac = typeof navigator !== "undefined" && /Mac|iPod|iPhone|iPad/.test(navigator.platform);

export function matches (e: KeyboardEvent, shortcut: Shortcut) {
    return (
        (e.ctrlKey || e.metaKey) === shortcut.mod &&
        e.shiftKey === shortcut.shift &&
        e.altKey === shortcut.alt &&
        e.code === shortcut.code
    );
}

export function formatShortcut(shortcut: Shortcut) {
    const parts = [];
    if (shortcut.mod) parts.push(isMac ? "⌘" : "Ctrl");
    if (shortcut.alt) parts.push("Alt");
    if (shortcut.shift ) parts.push("Shift");
    parts.push(shortcut.code.replace(/^Key|^Digit/, ""));
    return parts
}

export function capture(e: KeyboardEvent) {

    const id = shortcuts.recording;
    if(!id) return;

    e.preventDefault();
    e.stopPropagation();

    if(e.key === "Escape") {
        shortcuts.recording = null;
        return;
    }

    if(["Control", "Shift", "Alt", "Meta"].includes(e.key)) return;

    if(!(e.ctrlKey || e.metaKey || e.altKey )) return;

        const clashing = (Object.keys(shortcuts.bindings) as ActionId[]).find((other)=> other !== id && matches(e, shortcuts.bindings[other]));
    if (clashing) {
        shortcuts.error = `Shortcut clashes with ${LABELS[clashing]}`;
        return;
    }

    shortcuts.bindings[id] = {
        mod: e.ctrlKey || e.metaKey, 
        shift: e.shiftKey,
        alt: e.altKey,
        code: e.code
    };
    shortcuts.error = "";
    shortcuts.recording = null;
    save();
}

export function reset(id: ActionId) {
    shortcuts.bindings[id] = { ...DEFAULTS[id] };
    shortcuts.error = "";
    save();
}

export function toAccelerator(shortcut: Shortcut) {
    const parts: string[] = [];
    if (shortcut.mod) parts.push(isMac ? "Command" : "Ctrl");
    if (shortcut.alt) parts.push("Alt");
    if (shortcut.shift) parts.push("Shift");
    parts.push(shortcut.code.replace(/^Key|^Digit/, ""));
    return parts.join("+");
}