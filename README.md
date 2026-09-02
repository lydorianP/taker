# Taker

**Local AI Note Taker & Study Companion**

Taker is a fully local, open-source AI-powered note-taking and study companion app. It transforms notes into podcasts, flashcards, slideshows, and summaries—all running offline with no trackers, no freemium aspects, and complete data ownership.

## Features

### Core Features
- **100% Local AI**: No cloud dependency, no data leaving the device
- **Purpose-Built**: Not a generic chat wrapper; a study-focused productivity tool
- **Plugin System**: Mix and match AI models, outputs, themes
- **Cross-Platform**: Linux, Windows (Android post-launch)
- **Structured Output**: JSON-based data for RAG, flashcards, presentations

### AI-Powered Transformations
- **Summarize**: Notes → executive summary, key points
- **Flashcards**: Notes → Q&A pairs (Anki-compatible JSON)
- **Slideshows**: Notes → presentation slides (Reveal.js, Marp)
- **Podcast**: Notes → audio narration (TTS pipeline)
- **Quiz**: Notes → practice questions with answers

### Audio Features
- **Speech-to-Text**: Record audio → transcribe → create note
- **Text-to-Speech**: Read notes aloud, flashcard pronunciation
- **Podcast Generation**: Multi-note synthesis into audio episodes

### RAG & Search
- Hybrid search (vector + full-text)
- Semantic similarity across vaults
- Multilingual retrieval (200+ languages)
- Citation tracking (source note + chunk)

### Plugin System
- **Model Plugins**: Swap LLMs (Qwen, Llama, Phi, etc.)
- **Output Plugins**: New export formats (PDF, DOCX, etc.)
- **Theme Plugins**: Custom color schemes, fonts
- **Integration Plugins**: Cloud APIs via OpenAI-compatible endpoints

## Quick Start

### Prerequisites
- Rust (latest stable)
- Node.js 18+
- System dependencies for Tauri

### Installation

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

### First Launch
1. Taker will show the Greetings Page with recommended models
2. Click on a model to download it from HuggingFace
3. Or click "OpenAI Compatible Backend" to add a cloud API
4. Start creating notes and vaults!

## Technology Stack

| Layer | Technology | Rationale |
|-------|-----------|-----------|
| **Desktop Framework** | Tauri 2.0 | Small footprint (5-15 MB), Rust backend for native FFI |
| **Frontend** | Svelte 5 + Vite | Lightweight, fast, excellent Tauri integration |
| **LLM Inference** | llama.cpp (direct FFI) | No Ollama dependency, better performance |
| **Model Source** | HuggingFace Hub | Official, well-maintained, huge model library |
| **Speech-to-Text** | whisper.cpp | C/C++ library, Metal/CUDA acceleration |
| **Text-to-Speech** | Piper TTS | Fastest, CPU-only, 100+ voices |
| **Database** | SQLite + SQLCipher | Single encrypted file, zero config |
| **Vector Search** | sqlite-vec | Zero dependencies, same DB file |
| **Plugin System** | Hybrid (Native + WASM) | Core: performance; Community: sandboxing |

## Documentation

- [ROADMAP.md](ROADMAP.md) - Implementation phases and timeline
- [TODO.md](TODO.md) - Detailed task breakdown
- [INDEX.md](INDEX.md) - Project index and navigation
- [AGENTS.md](AGENTS.md) - Contribution guidelines

## Contributing

We welcome contributions! Please read [AGENTS.md](AGENTS.md) for details on our code of conduct and the process for submitting pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [llama.cpp](https://github.com/ggerganov/llama.cpp) - LLM inference
- [whisper.cpp](https://github.com/ggerganov/whisper.cpp) - Speech-to-text
- [Piper TTS](https://github.com/rhasspy/piper) - Text-to-speech
- [Tauri](https://tauri.app/) - Desktop framework
- [HuggingFace](https://huggingface.co/) - Model hub
