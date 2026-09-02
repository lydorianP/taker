# Taker - Roadmap

## Vision

Taker aims to be the best local-first, AI-powered study companion that respects user privacy and provides complete data ownership. Unlike cloud-based competitors, Taker runs entirely on the user's device with no trackers, no freemium aspects, and no data leaving the machine.

## Development Phases

### Phase 1: Foundation (Weeks 1-4)
**Goal**: Establish the core project structure and basic UI shell.

#### Milestones
- Project initialization with Tauri 2.0 + Svelte
- SQLite database with SQLCipher encryption
- Basic note CRUD operations
- Sidebar navigation and content-first chrome
- Settings panel with theme support

#### Deliverables
- Working desktop app with basic note management
- Encrypted local storage
- Minimal, non-generic UI

---

### Phase 2: Model Management (Weeks 5-8)
**Goal**: Implement AI model discovery, download, and inference.

#### Milestones
- Greetings page with curated model recommendations
- HuggingFace Hub integration for model downloads
- Download progress tracking
- llama.cpp integration for local inference
- OpenAI-compatible backend configuration

#### Deliverables
- Model browser with search and download
- Local LLM inference without external dependencies
- Cloud API fallback option

---

### Phase 3: Core Features (Weeks 9-12)
**Goal**: Implement AI-powered note transformations.

#### Milestones
- Markdown editor with live preview
- Vault system for organizing notes
- Note summarization pipeline
- Flashcard generation (structured JSON)
- Slideshow generation

#### Deliverables
- Complete note management system
- AI-powered transformations
- Multiple output format support

---

### Phase 4: Audio Features (Weeks 13-16)
**Goal**: Add speech-to-text and text-to-speech capabilities.

#### Milestones
- whisper.cpp integration for STT
- Piper TTS integration for read-aloud
- Audio recording → transcription pipeline
- Podcast generation (multi-note synthesis)

#### Deliverables
- Voice note creation
- Audio playback with speed control
- Podcast-style audio generation

---

### Phase 5: Plugin System (Weeks 17-20)
**Goal**: Build extensible architecture for community contributions.

#### Milestones
- Hybrid plugin architecture (Native + WASM)
- Plugin discovery and loading
- Permission validation and sandboxing
- Plugin marketplace
- Example plugins (Duolingo format, themes)

#### Deliverables
- Extensible plugin system
- Plugin developer SDK
- Community plugin marketplace

---

### Phase 6: Polish & Release (Weeks 21-24)
**Goal**: Prepare for production release.

#### Milestones
- Cross-platform testing (Windows, Linux)
- Performance optimization
- Documentation completion
- CI/CD pipeline setup

#### Deliverables
- Stable release for Windows and Linux
- Comprehensive documentation
- Community contribution guidelines

---

## Post-Launch (Phase 7+)

### Android Support
- Tauri Mobile integration
- Touch-optimized UI
- Model optimization for mobile devices

### Advanced Features
- Spaced repetition algorithm (optional)
- Knowledge graph visualization
- Collaborative notes (optional)
- Voice cloning (via plugins)

### Plugin Ecosystem
- Plugin developer documentation
- Plugin SDK
- Community plugin marketplace
- Plugin certification program

---

## Technology Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| **LLM Backend** | llama.cpp (direct FFI) | No Ollama dependency, better performance, full control |
| **Model Source** | HuggingFace Hub | Official, well-maintained, huge model library |
| **Plugin System** | Hybrid (Native + WASM) | Core: performance; Community: sandboxing |
| **Cloud Backend** | Simple config (URL + Key + Model) | Minimal friction, works with any OpenAI-compatible API |
| **Output Formats** | Selectable + Plugin-extensible | User choice, community extensibility |
| **Theme System** | Built-in + Plugin themes | Consistency + customization |

---

## Success Metrics

1. **Startup Time**: < 2 seconds to usable state
2. **Memory Usage**: < 500 MB baseline (no model loaded)
3. **Model Load**: < 5 seconds for 7B model
4. **STT Latency**: < 2x real-time transcription
5. **TTS Latency**: < 100ms first byte
6. **Search Latency**: < 50ms for 10K notes
7. **App Size**: < 50 MB (excluding models)

---

## Competitive Landscape

### NotebookLM (The Feature Benchmark)
- Better: Best-in-class "notes to podcast" audio generation
- Bad: 100% cloud-locked, closed-source
- Different: Taker is fully offline, local-first

### Obsidian (The Local/Vault Benchmark)
- Better: Unbeatable local "vault" architecture
- Bad: Zero native AI (requires cloud plugins)
- Different: Taker is purpose-built, AI-native

### Anki (The Flashcard Benchmark)
- Better: Gold standard for spaced-repetition
- Bad: Dated UI, no native AI
- Different: Taker integrates flashcards into broader AI pipeline

### RemNote (The Study Workflow Benchmark)
- Better: Excellent note-to-flashcard integration
- Bad: AI features are cloud-based and paywalled
- Different: Taker is 100% local, free, and open-source

### Jan.ai / LM Studio (The Local AI Benchmark)
- Better: Beautiful UIs for managing local LLMs
- Bad: Just generic chat interfaces, no study features
- Different: Taker is a productivity and study tool

---

## Contributing

See [AGENTS.md](AGENTS.md) for contribution guidelines.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
