# Taker - TODO

## Phase 1: Foundation (Weeks 1-4)

### Project Setup
- [ ] Initialize Tauri 2.0 project with Svelte 5 + Vite
- [ ] Configure Cargo.toml with dependencies
- [ ] Set up project structure
- [ ] Configure ESLint, Prettier, Rustfmt

### Database
- [ ] SQLite + SQLCipher setup
- [ ] Core schema (notes, vaults, flashcards, slideshows)
- [ ] FTS5 for full-text search
- [ ] Database migrations system

### Basic UI Shell
- [ ] Sidebar navigation (collapsible)
- [ ] Content-first chrome (minimal headers)
- [ ] Settings panel with left sidebar navigation
- [ ] Basic theme system (CSS variables)
- [ ] Command palette (Ctrl+K)

---

## Phase 2: Model Management (Weeks 5-8)

### Greetings Page
- [ ] Curated model recommendations
- [ ] HuggingFace search integration
- [ ] One-click download with progress
- [ ] "OpenAI Compatible Backend" button

### Model Download System
- [ ] `hf-hub` crate integration
- [ ] Download progress via Tauri Channels
- [ ] Atomic writes (.part files → rename)
- [ ] Cache management (scan, delete, disk usage)
- [ ] GGUF header validation

### Local Inference
- [ ] `llama-cpp-4` integration
- [ ] GPU auto-detection (CUDA, Metal, Vulkan)
- [ ] Model loading with progress
- [ ] Streaming token generation
- [ ] Context window management

### OpenAI Compatible Backend
- [ ] Simple configuration (URL + Key + Model)
- [ ] Store in OS keychain
- [ ] Test connection button
- [ ] Use as fallback or primary

---

## Phase 3: Core Features (Weeks 9-12)

### Note Management
- [ ] Markdown editor with live preview
- [ ] Vault system (create, switch, organize)
- [ ] Tags and folders
- [ ] Import/export (Markdown, JSON)

### AI Transformations
- [ ] Summarize: Notes → executive summary
- [ ] Flashcards: Notes → Q&A pairs (structured JSON)
- [ ] Slideshows: Notes → presentation slides
- [ ] Quiz: Notes → practice questions

### Selectable Output Formats
- [ ] Flashcard format selector (Anki, CSV, Custom)
- [ ] Presentation format selector (Reveal.js, Marp, PDF)
- [ ] Custom format support via plugins

---

## Phase 4: Audio Features (Weeks 13-16)

### Speech-to-Text
- [ ] whisper.cpp integration via Rust FFI
- [ ] Audio recording → transcription
- [ ] Language detection
- [ ] Real-time transcription option

### Text-to-Speech
- [ ] Piper TTS integration (CPU)
- [ ] Read notes aloud
- [ ] Flashcard pronunciation
- [ ] Speed control

### Podcast Generation
- [ ] Multi-note synthesis
- [ ] Single narrator (default)
- [ ] Multi-voice option (via plugins)

---

## Phase 5: Plugin System (Weeks 17-20)

### Hybrid Architecture
- [ ] Native Rust plugins (Tier 1): Core model, theme, outputs
- [ ] WASM plugins (Tier 2): Community extensions

### Plugin Runtime
- [ ] wasmtime integration
- [ ] Plugin discovery (~/.config/taker/plugins/)
- [ ] Permission validation
- [ ] Resource limits (memory, CPU)
- [ ] Hot-reload support

### Plugin Marketplace
- [ ] Plugin registry format
- [ ] Plugin discovery UI
- [ ] One-click install
- [ ] Version management
- [ ] Community ratings/reviews

### Example Plugins
- [ ] Duolingo Format: Flashcards styled like Duolingo
- [ ] PDF Export: Generate PDF flashcards/presentations
- [ ] Notion Sync: Export notes to Notion
- [ ] Minimal Theme: Clean, minimal UI theme
- [ ] High Contrast: Accessibility theme

---

## Phase 6: Polish & Release (Weeks 21-24)

### Cross-Platform
- [ ] Windows testing and optimization
- [ ] Linux testing and optimization
- [ ] (Android deferred to post-launch)

### Performance
- [ ] Startup time optimization (<2s)
- [ ] Memory usage optimization (<500MB baseline)
- [ ] Model loading optimization (<5s for 7B)

### Documentation
- [ ] User guide
- [ ] Plugin development SDK
- [ ] API documentation
- [ ] Contributing guide

### Release
- [ ] GitHub repository setup
- [ ] Community guidelines
- [ ] Issue templates
- [ ] CI/CD pipeline

---

## Icebox (Post-Launch)

### Android Support
- [ ] Tauri Mobile integration
- [ ] Touch-optimized UI
- [ ] Model optimization for mobile

### Advanced Features
- [ ] Spaced repetition algorithm (optional)
- [ ] Knowledge graph visualization
- [ ] Collaborative notes (optional)
- [ ] Voice cloning (via plugins)

### Plugin Ecosystem
- [ ] Plugin developer documentation
- [ ] Plugin SDK
- [ ] Community plugin marketplace
- [ ] Plugin certification program
