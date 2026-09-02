# Taker - Agents & Contribution Guidelines

## Welcome to Taker!

Thank you for your interest in contributing to Taker. This document provides guidelines and information for contributors.

## Code of Conduct

### Our Pledge
We pledge to make participation in our project a harassment-free experience for everyone, regardless of age, body size, disability, ethnicity, sex characteristics, gender identity and expression, level of experience, education, socio-economic status, nationality, personal appearance, race, religion, or sexual identity and orientation.

### Our Standards
Examples of behavior that contributes to creating a positive environment:
- Using welcoming and inclusive language
- Being respectful of differing viewpoints and experiences
- Gracefully accepting constructive criticism
- Focusing on what is best for the community
- Showing empathy towards other community members

Examples of unacceptable behavior:
- The use of sexualized language or imagery and unwelcome sexual attention or advances
- Trolling, insulting/derogatory comments, and personal or political attacks
- Public or private harassment
- Publishing others' private information without explicit permission
- Other conduct which could reasonably be considered inappropriate in a professional setting

## How to Contribute

### Reporting Bugs
Before creating bug reports, please check existing issues to avoid duplicates.

When creating a bug report, include:
- A clear and descriptive title
- Steps to reproduce the issue
- Expected behavior vs actual behavior
- Your environment (OS, Rust version, Node.js version)
- Any error messages or logs

### Suggesting Features
Feature suggestions are welcome! Please:
- Use a clear and descriptive title
- Provide a detailed description of the proposed feature
- Explain why this feature would be useful
- List any alternatives you've considered

### Pull Requests
1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Setup
```bash
# Clone the repository
git clone https://github.com/yourusername/taker.git
cd taker

# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Run tests
cargo test

# Run linter
cargo clippy
```

### Code Style
- Follow Rust formatting with `rustfmt`
- Follow JavaScript/TypeScript formatting with Prettier
- Write meaningful commit messages
- Add comments for complex logic
- Keep functions small and focused

### Testing
- Write unit tests for new functionality
- Ensure all tests pass before submitting PR
- Add integration tests for complex features
- Test on multiple platforms if possible

## Project Structure

```
taker/
├── src-tauri/           # Rust backend
│   ├── src/
│   │   ├── main.rs      # Tauri entry point
│   │   ├── db/          # SQLite operations
│   │   ├── ai/          # LLM inference
│   │   ├── audio/       # STT/TTS
│   │   ├── plugins/     # Plugin system
│   │   └── commands/    # Tauri commands
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
├── docs/                # Documentation
└── README.md
```

## Plugin Development

### Creating a Plugin
Plugins are the primary way to extend Taker. See [docs/plugin-development.md](docs/plugin-development.md) for details.

### Plugin Manifest
Every plugin must have a `manifest.toml`:
```toml
[plugin]
id = "com.example.my-plugin"
name = "My Plugin"
version = "1.0.0"
description = "A brief description"
author = "Your Name"
type = "output"  # model | output | theme | integration

[plugin.permissions]
requires = ["fs:read", "fs:write"]
optional = ["network:https"]
```

### Plugin Types
- **Model Plugins**: Provide LLM inference (local or cloud)
- **Output Plugins**: Generate new formats (flashcards, slides, audio)
- **Theme Plugins**: Custom UI themes and fonts
- **Integration Plugins**: Connect to external services

## Getting Help

- **GitHub Issues**: For bugs and feature requests
- **Discussions**: For questions and ideas
- **Discord**: For real-time chat (coming soon)

## License

By contributing to Taker, you agree that your contributions will be licensed under the MIT License.
