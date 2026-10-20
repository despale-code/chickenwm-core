//! Predetermined icon theme assets bundled with ChickenWM.
//!
//! Includes:
//! - `lucide`: Clean, modern vector icons (ISC License, matching macOS Finder aesthetic)
//! - `font-awesome`: Solid glyph icons (CC BY 4.0 / SIL OFL)
//! - `papirus`: Standard Linux XDG icon theme layout with `index.theme` (GPL 3.0)

pub struct IconAsset {
    pub rel_path: &'static str,
    pub svg: &'static str,
}

pub const LUCIDE_ICONS: &[IconAsset] = &[
    IconAsset {
        rel_path: "folder.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="#0ea5e9" stroke="#0284c7" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 8 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"/></svg>"##,
    },
    IconAsset {
        rel_path: "folder-home.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="#0ea5e9" stroke="#0284c7" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 8 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"/><path d="m9 13.5 3-2.5 3 2.5v3.5h-2v-2h-2v2H9Z" fill="#ffffff" stroke="none"/></svg>"##,
    },
    IconAsset {
        rel_path: "folder-desktop.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="#0ea5e9" stroke="#0284c7" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 8 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"/><rect x="8.5" y="11" width="7" height="4.5" rx="1" fill="none" stroke="#ffffff" stroke-width="1.5"/><path d="M10 17h4M12 15.5v1.5" stroke="#ffffff" stroke-width="1.5"/></svg>"##,
    },
    IconAsset {
        rel_path: "folder-downloads.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="#0ea5e9" stroke="#0284c7" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 8 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"/><path d="M12 10v6m0 0-2.5-2.5M12 16l2.5-2.5" stroke="#ffffff" stroke-width="2"/></svg>"##,
    },
    IconAsset {
        rel_path: "folder-documents.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="#0ea5e9" stroke="#0284c7" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 8 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"/><path d="M9 12h6M9 15h4" stroke="#ffffff" stroke-width="2"/></svg>"##,
    },
    IconAsset {
        rel_path: "folder-pictures.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="#0ea5e9" stroke="#0284c7" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 8 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"/><circle cx="10" cy="12" r="1.5" fill="#ffffff" stroke="none"/><path d="m8 16 3-3 5 5" stroke="#ffffff" stroke-width="1.8"/></svg>"##,
    },
    IconAsset {
        rel_path: "folder-music.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="#0ea5e9" stroke="#0284c7" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 8 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"/><path d="M11 16.5a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0Zm0 0V11l4-1v5.5a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0V10" stroke="#ffffff" stroke-width="1.6"/></svg>"##,
    },
    IconAsset {
        rel_path: "folder-videos.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="#0ea5e9" stroke="#0284c7" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 8 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"/><polygon points="10 11 15 13.5 10 16 10 11" fill="#ffffff" stroke="none"/></svg>"##,
    },
    IconAsset {
        rel_path: "drive.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="#475569" stroke="#334155" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><rect width="20" height="12" x="2" y="6" rx="2" fill="#64748b"/><path d="M6 14h.01M10 14h.01" stroke="#38bdf8" stroke-width="3"/><line x1="2" x2="22" y1="11" y2="11" stroke="#334155" stroke-width="1.5"/></svg>"##,
    },
    IconAsset {
        rel_path: "file.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="#1e293b" stroke="#64748b" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" fill="#1e293b"/><polyline points="14 2 14 8 20 8" fill="#334155"/></svg>"##,
    },
    IconAsset {
        rel_path: "code.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="#1e293b" stroke="#38bdf8" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8" fill="#38bdf8" fill-opacity="0.3"/><path d="m9 12-2 2 2 2M15 12l2 2-2 2M13 11l-2 6" stroke="#38bdf8" stroke-width="1.8"/></svg>"##,
    },
    IconAsset {
        rel_path: "pdf.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="#1e293b" stroke="#f43f5e" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8" fill="#f43f5e" fill-opacity="0.3"/><rect x="7" y="12" width="10" height="5" rx="1" fill="#f43f5e" stroke="none"/><text x="12" y="15.8" fill="#ffffff" font-size="3.5" font-weight="bold" font-family="sans-serif" text-anchor="middle">PDF</text></svg>"##,
    },
    IconAsset {
        rel_path: "image.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="#1e293b" stroke="#a855f7" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8" fill="#a855f7" fill-opacity="0.3"/><circle cx="9" cy="13" r="1.5" fill="#a855f7" stroke="none"/><path d="m7 18 4-4 3 3 2-2 2 3" stroke="#a855f7" stroke-width="1.5"/></svg>"##,
    },
    IconAsset {
        rel_path: "video.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="#1e293b" stroke="#f59e0b" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8" fill="#f59e0b" fill-opacity="0.3"/><polygon points="10 12 15 14.5 10 17 10 12" fill="#f59e0b" stroke="none"/></svg>"##,
    },
    IconAsset {
        rel_path: "audio.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="#1e293b" stroke="#ec4899" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8" fill="#ec4899" fill-opacity="0.3"/><path d="M10 16.5a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0Zm0 0V12l4-1v4.5a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0V11" stroke="#ec4899" stroke-width="1.5"/></svg>"##,
    },
    IconAsset {
        rel_path: "archive.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="#1e293b" stroke="#eab308" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8" fill="#eab308" fill-opacity="0.3"/><path d="M11 6h2M11 8h2M11 10h2M11 12h2M10 14h4v3h-4z" stroke="#eab308" stroke-width="1.5"/></svg>"##,
    },
    IconAsset {
        rel_path: "text.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="#1e293b" stroke="#94a3b8" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8" fill="#94a3b8" fill-opacity="0.3"/><path d="M8 12h8M8 15h6M8 18h4" stroke="#94a3b8" stroke-width="1.5"/></svg>"##,
    },
    IconAsset {
        rel_path: "document.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="#1e293b" stroke="#3b82f6" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8" fill="#3b82f6" fill-opacity="0.3"/><path d="M8 12h8M8 15h6M8 18h4" stroke="#3b82f6" stroke-width="1.5"/></svg>"##,
    },
    IconAsset {
        rel_path: "executable.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="#1e293b" stroke="#22c55e" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8" fill="#22c55e" fill-opacity="0.3"/><path d="m8 13 2.5 2.5L8 18M12 18h4" stroke="#22c55e" stroke-width="1.8"/></svg>"##,
    },
    IconAsset {
        rel_path: "font.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="#1e293b" stroke="#6366f1" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8" fill="#6366f1" fill-opacity="0.3"/><text x="12" y="17" fill="#6366f1" font-size="7" font-weight="bold" font-family="serif" text-anchor="middle">A</text></svg>"##,
    },
    IconAsset {
        rel_path: "disk.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="#1e293b" stroke="#06b6d4" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8" fill="#06b6d4" fill-opacity="0.3"/><circle cx="12" cy="14" r="3.5" fill="none" stroke="#06b6d4" stroke-width="1.5"/><circle cx="12" cy="14" r="1" fill="#06b6d4" stroke="none"/></svg>"##,
    },
];

pub const FONT_AWESOME_ICONS: &[IconAsset] = &[
    IconAsset {
        rel_path: "folder.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" fill="#eab308"><path d="M64 480h384c35.3 0 64-28.7 64-64V160c0-35.3-28.7-64-64-64H288l-64-64H64C28.7 32 0 60.7 0 96v320c0 35.3 28.7 64 64 64z"/></svg>"##,
    },
    IconAsset {
        rel_path: "folder-home.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" fill="#eab308"><path d="M64 480h384c35.3 0 64-28.7 64-64V160c0-35.3-28.7-64-64-64H288l-64-64H64C28.7 32 0 60.7 0 96v320c0 35.3 28.7 64 64 64z"/><path d="M256 190l-90 80h30v70h120v-70h30z" fill="#ffffff"/></svg>"##,
    },
    IconAsset {
        rel_path: "folder-desktop.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" fill="#eab308"><path d="M64 480h384c35.3 0 64-28.7 64-64V160c0-35.3-28.7-64-64-64H288l-64-64H64C28.7 32 0 60.7 0 96v320c0 35.3 28.7 64 64 64z"/><rect x="176" y="220" width="160" height="100" rx="16" fill="#ffffff"/><path d="M226 340h60M256 320v20" stroke="#ffffff" stroke-width="20" stroke-linecap="round"/></svg>"##,
    },
    IconAsset {
        rel_path: "folder-downloads.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" fill="#eab308"><path d="M64 480h384c35.3 0 64-28.7 64-64V160c0-35.3-28.7-64-64-64H288l-64-64H64C28.7 32 0 60.7 0 96v320c0 35.3 28.7 64 64 64z"/><path d="M256 200v110m-35-35l35 35 35-35M196 345h120" stroke="#ffffff" stroke-width="28" stroke-linecap="round" stroke-linejoin="round"/></svg>"##,
    },
    IconAsset {
        rel_path: "folder-documents.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" fill="#eab308"><path d="M64 480h384c35.3 0 64-28.7 64-64V160c0-35.3-28.7-64-64-64H288l-64-64H64C28.7 32 0 60.7 0 96v320c0 35.3 28.7 64 64 64z"/><path d="M206 240h100M206 280h100M206 320h60" stroke="#ffffff" stroke-width="24" stroke-linecap="round"/></svg>"##,
    },
    IconAsset {
        rel_path: "folder-pictures.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" fill="#eab308"><path d="M64 480h384c35.3 0 64-28.7 64-64V160c0-35.3-28.7-64-64-64H288l-64-64H64C28.7 32 0 60.7 0 96v320c0 35.3 28.7 64 64 64z"/><circle cx="216" cy="240" r="22" fill="#ffffff"/><path d="m186 330 45-50 40 30 35-40 40 60z" fill="#ffffff"/></svg>"##,
    },
    IconAsset {
        rel_path: "folder-music.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" fill="#eab308"><path d="M64 480h384c35.3 0 64-28.7 64-64V160c0-35.3-28.7-64-64-64H288l-64-64H64C28.7 32 0 60.7 0 96v320c0 35.3 28.7 64 64 64z"/><path d="M236 325a25 25 0 1 1-50 0 25 25 0 0 1 50 0zm0 0V220l75-20v95a25 25 0 1 1-50 0 25 25 0 0 1 50 0v-85" fill="#ffffff"/></svg>"##,
    },
    IconAsset {
        rel_path: "folder-videos.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" fill="#eab308"><path d="M64 480h384c35.3 0 64-28.7 64-64V160c0-35.3-28.7-64-64-64H288l-64-64H64C28.7 32 0 60.7 0 96v320c0 35.3 28.7 64 64 64z"/><polygon points="220 220 320 275 220 330 220 220" fill="#ffffff"/></svg>"##,
    },
    IconAsset {
        rel_path: "drive.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" fill="#475569"><rect x="32" y="128" width="448" height="256" rx="48" fill="#334155"/><circle cx="112" cy="256" r="24" fill="#38bdf8"/><path d="M380 256h40" stroke="#94a3b8" stroke-width="24" stroke-linecap="round"/></svg>"##,
    },
    IconAsset {
        rel_path: "file.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512" fill="#94a3b8"><path d="M0 64C0 28.7 28.7 0 64 0H224V128c0 17.7 14.3 32 32 32H384V448c0 35.3-28.7 64-64 64H64c-35.3 0-64-28.7-64-64V64zm384 64H256V0L384 128z"/></svg>"##,
    },
    IconAsset {
        rel_path: "code.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512" fill="#0284c7"><path d="M0 64C0 28.7 28.7 0 64 0H224V128c0 17.7 14.3 32 32 32H384V448c0 35.3-28.7 64-64 64H64c-35.3 0-64-28.7-64-64V64zm384 64H256V0L384 128z"/><path d="m140 260-45 45 45 45m104-90 45 45-45 45m-40-100-24 110" stroke="#ffffff" stroke-width="24" stroke-linecap="round" stroke-linejoin="round" fill="none"/></svg>"##,
    },
    IconAsset {
        rel_path: "pdf.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512" fill="#e11d48"><path d="M0 64C0 28.7 28.7 0 64 0H224V128c0 17.7 14.3 32 32 32H384V448c0 35.3-28.7 64-64 64H64c-35.3 0-64-28.7-64-64V64zm384 64H256V0L384 128z"/><text x="192" y="340" fill="#ffffff" font-size="75" font-weight="900" font-family="sans-serif" text-anchor="middle">PDF</text></svg>"##,
    },
    IconAsset {
        rel_path: "image.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512" fill="#9333ea"><path d="M0 64C0 28.7 28.7 0 64 0H224V128c0 17.7 14.3 32 32 32H384V448c0 35.3-28.7 64-64 64H64c-35.3 0-64-28.7-64-64V64zm384 64H256V0L384 128z"/><circle cx="140" cy="270" r="20" fill="#ffffff"/><path d="m110 370 50-50 40 30 40-40 45 60z" fill="#ffffff"/></svg>"##,
    },
    IconAsset {
        rel_path: "video.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512" fill="#d97706"><path d="M0 64C0 28.7 28.7 0 64 0H224V128c0 17.7 14.3 32 32 32H384V448c0 35.3-28.7 64-64 64H64c-35.3 0-64-28.7-64-64V64zm384 64H256V0L384 128z"/><polygon points="160 250 240 300 160 350 160 250" fill="#ffffff"/></svg>"##,
    },
    IconAsset {
        rel_path: "audio.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512" fill="#db2777"><path d="M0 64C0 28.7 28.7 0 64 0H224V128c0 17.7 14.3 32 32 32H384V448c0 35.3-28.7 64-64 64H64c-35.3 0-64-28.7-64-64V64zm384 64H256V0L384 128z"/><path d="M175 350a25 25 0 1 1-50 0 25 25 0 0 1 50 0zm0 0V250l65-15v85a25 25 0 1 1-50 0 25 25 0 0 1 50 0v-75" fill="#ffffff"/></svg>"##,
    },
    IconAsset {
        rel_path: "archive.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512" fill="#ca8a04"><path d="M0 64C0 28.7 28.7 0 64 0H224V128c0 17.7 14.3 32 32 32H384V448c0 35.3-28.7 64-64 64H64c-35.3 0-64-28.7-64-64V64zm384 64H256V0L384 128z"/><path d="M170 180h40M170 210h40M170 240h40M170 270h40M160 300h60v50h-60z" stroke="#ffffff" stroke-width="20" fill="none"/></svg>"##,
    },
    IconAsset {
        rel_path: "text.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512" fill="#64748b"><path d="M0 64C0 28.7 28.7 0 64 0H224V128c0 17.7 14.3 32 32 32H384V448c0 35.3-28.7 64-64 64H64c-35.3 0-64-28.7-64-64V64zm384 64H256V0L384 128z"/><path d="M110 260h160M110 310h160M110 360h100" stroke="#ffffff" stroke-width="24" stroke-linecap="round"/></svg>"##,
    },
    IconAsset {
        rel_path: "document.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512" fill="#2563eb"><path d="M0 64C0 28.7 28.7 0 64 0H224V128c0 17.7 14.3 32 32 32H384V448c0 35.3-28.7 64-64 64H64c-35.3 0-64-28.7-64-64V64zm384 64H256V0L384 128z"/><path d="M110 260h160M110 310h160M110 360h100" stroke="#ffffff" stroke-width="24" stroke-linecap="round"/></svg>"##,
    },
    IconAsset {
        rel_path: "executable.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512" fill="#16a34a"><path d="M0 64C0 28.7 28.7 0 64 0H224V128c0 17.7 14.3 32 32 32H384V448c0 35.3-28.7 64-64 64H64c-35.3 0-64-28.7-64-64V64zm384 64H256V0L384 128z"/><path d="m110 270 40 40-40 40m50 40h70" stroke="#ffffff" stroke-width="28" stroke-linecap="round" stroke-linejoin="round" fill="none"/></svg>"##,
    },
    IconAsset {
        rel_path: "font.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512" fill="#4f46e5"><path d="M0 64C0 28.7 28.7 0 64 0H224V128c0 17.7 14.3 32 32 32H384V448c0 35.3-28.7 64-64 64H64c-35.3 0-64-28.7-64-64V64zm384 64H256V0L384 128z"/><text x="192" y="360" fill="#ffffff" font-size="140" font-weight="bold" font-family="serif" text-anchor="middle">A</text></svg>"##,
    },
    IconAsset {
        rel_path: "disk.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512" fill="#0891b2"><path d="M0 64C0 28.7 28.7 0 64 0H224V128c0 17.7 14.3 32 32 32H384V448c0 35.3-28.7 64-64 64H64c-35.3 0-64-28.7-64-64V64zm384 64H256V0L384 128z"/><circle cx="192" cy="310" r="60" fill="none" stroke="#ffffff" stroke-width="25"/><circle cx="192" cy="310" r="16" fill="#ffffff"/></svg>"##,
    },
];

pub const PAPIRUS_THEME_INDEX: &str = r##"[Icon Theme]
Name=Papirus
Comment=Papirus icon theme for ChickenWM (XDG Freedesktop format)
Inherits=chickenwm
Directories=places,mimetypes,devices,apps
"##;

pub const PAPIRUS_ICONS: &[IconAsset] = &[
    IconAsset {
        rel_path: "places/folder.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48 48"><path d="M6 8a4 4 0 0 0-4 4v24a4 4 0 0 0 4 4h36a4 4 0 0 0 4-4V18a4 4 0 0 0-4-4H24l-4-6H6z" fill="#4182cf"/><path d="M6 14h36a4 4 0 0 1 4 4v18a4 4 0 0 1-4 4H6a4 4 0 0 1-4-4V18a4 4 0 0 1 4-4z" fill="#5294e2"/></svg>"##,
    },
    IconAsset {
        rel_path: "places/folder-download.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48 48"><path d="M6 8a4 4 0 0 0-4 4v24a4 4 0 0 0 4 4h36a4 4 0 0 0 4-4V18a4 4 0 0 0-4-4H24l-4-6H6z" fill="#4182cf"/><path d="M6 14h36a4 4 0 0 1 4 4v18a4 4 0 0 1-4 4H6a4 4 0 0 1-4-4V18a4 4 0 0 1 4-4z" fill="#5294e2"/><path d="M24 20v11m-4-4 4 4 4-4M18 34h12" stroke="#ffffff" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" fill="none"/></svg>"##,
    },
    IconAsset {
        rel_path: "places/folder-documents.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48 48"><path d="M6 8a4 4 0 0 0-4 4v24a4 4 0 0 0 4 4h36a4 4 0 0 0 4-4V18a4 4 0 0 0-4-4H24l-4-6H6z" fill="#4182cf"/><path d="M6 14h36a4 4 0 0 1 4 4v18a4 4 0 0 1-4 4H6a4 4 0 0 1-4-4V18a4 4 0 0 1 4-4z" fill="#5294e2"/><path d="M19 23h10M19 27h10M19 31h6" stroke="#ffffff" stroke-width="2" stroke-linecap="round"/></svg>"##,
    },
    IconAsset {
        rel_path: "places/folder-pictures.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48 48"><path d="M6 8a4 4 0 0 0-4 4v24a4 4 0 0 0 4 4h36a4 4 0 0 0 4-4V18a4 4 0 0 0-4-4H24l-4-6H6z" fill="#4182cf"/><path d="M6 14h36a4 4 0 0 1 4 4v18a4 4 0 0 1-4 4H6a4 4 0 0 1-4-4V18a4 4 0 0 1 4-4z" fill="#5294e2"/><circle cx="20" cy="23" r="2.5" fill="#ffffff"/><path d="m17 33 5-6 4 4 4-5 5 7z" fill="#ffffff"/></svg>"##,
    },
    IconAsset {
        rel_path: "places/folder-music.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48 48"><path d="M6 8a4 4 0 0 0-4 4v24a4 4 0 0 0 4 4h36a4 4 0 0 0 4-4V18a4 4 0 0 0-4-4H24l-4-6H6z" fill="#4182cf"/><path d="M6 14h36a4 4 0 0 1 4 4v18a4 4 0 0 1-4 4H6a4 4 0 0 1-4-4V18a4 4 0 0 1 4-4z" fill="#5294e2"/><path d="M22 31a2.5 2.5 0 1 1-5 0 2.5 2.5 0 0 1 5 0zm0 0V21l7-2v9a2.5 2.5 0 1 1-5 0 2.5 2.5 0 0 1 5 0v-8" fill="#ffffff"/></svg>"##,
    },
    IconAsset {
        rel_path: "places/folder-videos.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48 48"><path d="M6 8a4 4 0 0 0-4 4v24a4 4 0 0 0 4 4h36a4 4 0 0 0 4-4V18a4 4 0 0 0-4-4H24l-4-6H6z" fill="#4182cf"/><path d="M6 14h36a4 4 0 0 1 4 4v18a4 4 0 0 1-4 4H6a4 4 0 0 1-4-4V18a4 4 0 0 1 4-4z" fill="#5294e2"/><polygon points="21 22 29 27 21 32 21 22" fill="#ffffff"/></svg>"##,
    },
    IconAsset {
        rel_path: "places/user-home.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48 48"><path d="M6 8a4 4 0 0 0-4 4v24a4 4 0 0 0 4 4h36a4 4 0 0 0 4-4V18a4 4 0 0 0-4-4H24l-4-6H6z" fill="#4182cf"/><path d="M6 14h36a4 4 0 0 1 4 4v18a4 4 0 0 1-4 4H6a4 4 0 0 1-4-4V18a4 4 0 0 1 4-4z" fill="#5294e2"/><path d="M24 19l-9 8h3v7h12v-7h3z" fill="#ffffff"/></svg>"##,
    },
    IconAsset {
        rel_path: "places/user-desktop.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48 48"><path d="M6 8a4 4 0 0 0-4 4v24a4 4 0 0 0 4 4h36a4 4 0 0 0 4-4V18a4 4 0 0 0-4-4H24l-4-6H6z" fill="#4182cf"/><path d="M6 14h36a4 4 0 0 1 4 4v18a4 4 0 0 1-4 4H6a4 4 0 0 1-4-4V18a4 4 0 0 1 4-4z" fill="#5294e2"/><rect x="17" y="21" width="14" height="9" rx="1.5" fill="none" stroke="#ffffff" stroke-width="2"/><path d="M20 33h8M24 30v3" stroke="#ffffff" stroke-width="2" stroke-linecap="round"/></svg>"##,
    },
    IconAsset {
        rel_path: "devices/drive-harddisk.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48 48"><rect x="4" y="10" width="40" height="28" rx="4" fill="#3a4049"/><circle cx="12" cy="24" r="3" fill="#00d1b2"/><path d="M34 24h4" stroke="#8c97a4" stroke-width="3" stroke-linecap="round"/></svg>"##,
    },
    IconAsset {
        rel_path: "mimetypes/text-plain.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48 48"><path d="M10 4h20l10 10v26a4 4 0 0 1-4 4H10a4 4 0 0 1-4-4V8a4 4 0 0 1 4-4z" fill="#e4e6ea"/><path d="M30 4v10h10" fill="#c1c6ce"/><path d="M14 20h20M14 26h20M14 32h14" stroke="#717a87" stroke-width="2.5" stroke-linecap="round"/></svg>"##,
    },
    IconAsset {
        rel_path: "mimetypes/application-pdf.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48 48"><path d="M10 4h20l10 10v26a4 4 0 0 1-4 4H10a4 4 0 0 1-4-4V8a4 4 0 0 1 4-4z" fill="#e74c3c"/><path d="M30 4v10h10" fill="#c0392b"/><text x="24" y="32" fill="#ffffff" font-size="10" font-weight="bold" font-family="sans-serif" text-anchor="middle">PDF</text></svg>"##,
    },
    IconAsset {
        rel_path: "mimetypes/image-x-generic.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48 48"><path d="M10 4h20l10 10v26a4 4 0 0 1-4 4H10a4 4 0 0 1-4-4V8a4 4 0 0 1 4-4z" fill="#9b59b6"/><path d="M30 4v10h10" fill="#8e44ad"/><circle cx="18" cy="22" r="3" fill="#ffffff"/><path d="m14 36 7-8 5 5 5-6 7 9z" fill="#ffffff"/></svg>"##,
    },
    IconAsset {
        rel_path: "mimetypes/video-x-generic.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48 48"><path d="M10 4h20l10 10v26a4 4 0 0 1-4 4H10a4 4 0 0 1-4-4V8a4 4 0 0 1 4-4z" fill="#e67e22"/><path d="M30 4v10h10" fill="#d35400"/><polygon points="20 22 30 27 20 32 20 22" fill="#ffffff"/></svg>"##,
    },
    IconAsset {
        rel_path: "mimetypes/audio-x-generic.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48 48"><path d="M10 4h20l10 10v26a4 4 0 0 1-4 4H10a4 4 0 0 1-4-4V8a4 4 0 0 1 4-4z" fill="#1abc9c"/><path d="M30 4v10h10" fill="#16a085"/><path d="M22 33a2.5 2.5 0 1 1-5 0 2.5 2.5 0 0 1 5 0zm0 0V23l8-2v10a2.5 2.5 0 1 1-5 0 2.5 2.5 0 0 1 5 0v-9" fill="#ffffff"/></svg>"##,
    },
    IconAsset {
        rel_path: "mimetypes/package-x-generic.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48 48"><path d="M10 4h20l10 10v26a4 4 0 0 1-4 4H10a4 4 0 0 1-4-4V8a4 4 0 0 1 4-4z" fill="#f39c12"/><path d="M30 4v10h10" fill="#e67e22"/><path d="M22 14h4M22 18h4M22 22h4M22 26h4M21 29h6v6h-6z" stroke="#ffffff" stroke-width="2.5" fill="none"/></svg>"##,
    },
    IconAsset {
        rel_path: "mimetypes/text-x-script.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48 48"><path d="M10 4h20l10 10v26a4 4 0 0 1-4 4H10a4 4 0 0 1-4-4V8a4 4 0 0 1 4-4z" fill="#2980b9"/><path d="M30 4v10h10" fill="#2471a3"/><path d="m18 24-4 4 4 4m12-8 4 4-4 4m-4-9-4 10" stroke="#ffffff" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" fill="none"/></svg>"##,
    },
    IconAsset {
        rel_path: "mimetypes/application-x-executable.svg",
        svg: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48 48"><path d="M10 4h20l10 10v26a4 4 0 0 1-4 4H10a4 4 0 0 1-4-4V8a4 4 0 0 1 4-4z" fill="#27ae60"/><path d="M30 4v10h10" fill="#229954"/><path d="m16 23 4 4-4 4m6 4h8" stroke="#ffffff" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" fill="none"/></svg>"##,
    },
];
