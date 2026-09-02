# Taker - Project Index

## Overview

Taker is a fully local, open-source AI-powered note-taking and study companion app. It transforms notes into podcasts, flashcards, slideshows, and summaries—all running offline with no trackers, no freemium aspects, and complete data ownership.

## Navigation

### Documentation
- [README.md](README.md) - Project overview and quick start
- [ROADMAP.md](ROADMAP.md) - Implementation phases and timeline
- [TODO.md](TODO.md) - Detailed task breakdown
- [AGENTS.md](AGENTS.md) - Contribution guidelines
- [LICENSE](LICENSE) - MIT License

### Source Code

#### Backend (Rust)
- [src-tauri/src/main.rs](src-tauri/src/main.rs) - Tauri entry point
- [src-tauri/src/lib.rs](src-tauri/src/lib.rs) - Shared types and utilities
- [src-tauri/src/commands/](src-tauri/src/commands/) - Tauri IPC commands
- [src-tauri/src/model/](src-tauri/src/model/) - LLM inference wrapper
- [src-tauri/src/plugins/](src-tauri/src/plugins/) - Plugin system
- [src-tauri/src/db/](src-tauri/src/db/) - SQLite operations

#### Frontend (Svelte)
- [src/lib/components/](src/lib/components/) - UI components
- [src/lib/stores/](src/lib/stores/) - State management
- [src/lib/utils/](src/lib/utils/) - Helper functions
- [src/lib/plugins/](src/lib/plugins/) - Plugin UI

### Plugins
- [plugins/built-in/](plugins/built-in/) - Official plugins
- [plugins/community/](plugins/community/) - Community plugins

### Documentation
- [docs/](docs/) - Detailed documentation

---

## Project Structure

```
taker/
├── src-tauri/           # Rust backend
│   ├── src/
│   │   ├── main.rs      # Tauri entry point
│   │   ├── lib.rs       # Shared types
│   │   ├── commands/    # Tauri IPC commands
│   │   ├── model/       # LLM inference
│   │   ├── plugins/     # Plugin system
│   │   └── db/          # SQLite operations
│   ├── Cargo.toml
│   └── tauri.conf.json
├── src/                 # Svelte frontend
│   ├── lib/
│   │   ├── components/  # UI components
│   │   ├── stores/      # State management
│   │   ├── utils/       # Helpers
│   │   └── plugins/     # Plugin UI
│   ├── routes/          # Page routes
│   └── App.svelte
├── plugins/             # Built-in plugins
│   ├── built-in/        # Official plugins
│   └── community/       # Community plugins
├── docs/                # Documentation
├── assets/              # Static assets
├── README.md            # Project overview
├── ROADMAP.md           # Implementation phases
├── TODO.md              # Task breakdown
├── AGENTS.md            # Contribution guidelines
├── LICENSE              # MIT License
└── INDEX.md             # This file
```

---

## Key Features

### Core Features
- 100% Local AI (no cloud dependency)
- Purpose-built study companion
- Plugin system for extensibility
- Cross-platform (Linux, Windows)
- Structured JSON output

### AI-Powered Transformations
- Summarize notes
- Generate flashcards
- Create slideshows
- Generate quizzes
- Podcast generation

### Audio Features
- Speech-to-text (whisper.cpp)
- Text-to-speech (Piper TTS)
- Podcast generation

### RAG & Search
- Hybrid search (vector + full-text)
- Semantic similarity
- Multilingual retrieval (200+ languages)

### Plugin System
- Model plugins (swap LLMs)
- Output plugins (new formats)
- Theme plugins (custom UI)
- Integration plugins (cloud APIs)

---

## Technology Stack

| Layer | Technology |
|-------|-----------|
| Desktop Framework | Tauri 2.0 |
| Frontend | Svelte 5 + Vite |
| LLM Inference | llama.cpp (direct FFI) |
| Model Source | HuggingFace Hub |
| Speech-to-Text | whisper.cpp |
| Text-to-Speech | Piper TTS |
| Database | SQLite + SQLCipher |
| Vector Search | sqlite-vec |
| Plugin System | Hybrid (Native + WASM) |

---

## Quick Start

```bash
# Clone the repository
git clone https://github.com/yourusername/taker.git
cd taker

# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

---

## Contributing

See [AGENTS.md](AGENTS.md) for contribution guidelines.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
