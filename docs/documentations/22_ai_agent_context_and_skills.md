# 🤖 Modul 22: AI-Ready Architecture, Universal Agent Context & Multi-Model Skills

> *"Lumina adalah framework Rust pertama di dunia yang AI-First: Zero-Hallucination Agent Collaboration, Universal AI Context, dan Multi-Model Tool Calling Stubs (Gemini, Claude, OpenAI)."*

---

## 🏛️ 1. Filosofi AI-Ready Lumina

Saat developer membuat proyek baru (`lumina new my_app`) atau melakukan `git clone` proyek Lumina, AI Assistant yang digunakan (**Cursor, GitHub Copilot, Google Antigravity, Claude Code, Windsurf, atau ChatGPT**) langsung mengenali ekosistem Lumina tanpa berhalusinasi atau menulis boilerplate Axum mentah.

### File Konteks yang Otomatis Dibuat:
1. **`AGENTS.md`**: Panduan master universal untuk AI Agent modern.
2. **`.cursorrules`**: Konfigurasi instruksi prioritas tinggi untuk Cursor AI Composer.
3. **`.github/copilot-instructions.md`**: Instruksi resmi untuk GitHub Copilot Workspace.
4. **`CLAUDE.md`**: Standar konvensi kode untuk Anthropic Claude Code CLI.

---

## ⚡ 2. Sinkronisasi Konteks AI (`lumina make:ai-context`)

Jika Anda melakukan clone repositori lama atau ingin memperbarui pedoman AI dengan versi terbaru:
```bash
# Sinkronkan seluruh file instruksi AI
lumina make:ai-context

# Atau gunakan alias
lumina ai:init

# Timpa file yang sudah ada
lumina make:ai-context --force

# Target spesifik
lumina make:ai-context --target=cursor
```

---

## 🧠 3. Multi-Model AI Skill Generator (`lumina make:ai-skill`)

Anda dapat membuat Tool / Function Calling definitions yang otomatis kompatibel dengan **OpenAI, Google Gemini, Anthropic Claude, dan Antigravity/Cursor**:

```bash
# Membuat skill umum dengan parameter tipe kuat
lumina make:ai-skill SearchProducts keyword:string limit:i64 category_id:i64
```

Perintah di atas secara otomatis menghasilkan 3 file terstandarisasi:
1. `app/ai/tools/search_products.json`: Schema Universal (OpenAI tool, Gemini declaration, Claude tool).
2. `app/ai/skills/search_products/SKILL.md`: Spesifikasi skill untuk Antigravity/Cursor agent.
3. `src/app/services/ai/search_products_tool.rs`: Handler Rust ber-tipe kuat untuk eksekusi logika bisnis secara aman.

---

## 🔍 4. Specialized AI Reverse-Engineer Skill

Jika Anda ingin merekayasa balik (*reverse-engineer*) website, tampilan HTML, atau komponen aplikasi target:
```bash
lumina make:ai-skill ReverseEngineer
```
AI Agent akan:
- Menganalisis DOM/CSS dan mengubahnya menjadi template Blade dengan **Tailwind CSS**.
- Mendeteksi elemen interaktif dan mengubahnya menjadi komponen **Lumina Live** (`kilat:*`, `wire:model`, `wire:click`).
- Menginferensi struktur data ke dalam Model ORM `#[derive(LuminaModel)]` dan migrasi database.

---

## 💬 5. Lumina Native AI Facade (`lumina::ai`)

Panggil model AI populer secara ekspresif ala Laravel:
```rust
use lumina::prelude::*;

// 1. Google Gemini (Gemini 1.5 Flash / Pro)
let reply = Ai::gemini()
    .model("gemini-1.5-flash")
    .with_system("Kamu adalah konsultan toko ritel.")
    .prompt("Analisis tren penjualan hari ini: ...")
    .generate()
    .await?;

// 2. OpenAI / ChatGPT (GPT-4o)
let summary = Ai::openai()
    .model("gpt-4o")
    .temperature(0.7)
    .chat("Buatkan ringkasan invoice berikut: ...")
    .generate()
    .await?;

// 3. Anthropic Claude (Claude 3.7 Sonnet)
let code_review = Ai::claude()
    .model("claude-3-7-sonnet-20250219")
    .max_tokens(4096)
    .complete("Refactor handler controller ini: ...")
    .generate()
    .await?;
```
