---
name: project-roadmap
description: Use when user asks about current objectives, progress, next steps, or wants to sync TODO with ROADMAP. Reads TODO.md and ROADMAP.md to determine current phase and present relevant tasks.
---

# Project Roadmap Skill

## When to Use
- User asks "what's next?" or "what should I work on?"
- User asks about current progress or phase
- User wants to sync TODO items with the roadmap
- User mentions TODO.md or ROADMAP.md
- User says "proceed with roadmap" or "continue with todo"

## Instructions

1. **Read both files**: Load `TODO.md` and `ROADMAP.md` from the project root

2. **Parse ROADMAP.md**: Identify all phases and their descriptions:
   - Phase 1: Foundation (Weeks 1-4)
   - Phase 2: Model Management (Weeks 5-8)
   - Phase 3: Core Features (Weeks 9-12)
   - Phase 4: Audio Features (Weeks 13-16)
   - Phase 5: Plugin System (Weeks 17-20)
   - Phase 6: Polish & Release (Weeks 21-24)

3. **Parse TODO.md**: Extract all checklist items and their completion status:
   - Look for `- [ ]` (incomplete) and `- [x]` (complete) items
   - Group items by their parent section/phase

4. **Determine Current Phase**: 
   - Find the first phase that has incomplete items
   - Calculate completion percentage for each phase
   - Identify which phase the user should focus on next

5. **Present to User**: Output a structured summary:
   ```
   ## Current Progress
   - Phase X: Phase Name (XX% complete)
   
   ### Current Objectives
   - [ ] Task 1
   - [ ] Task 2
   - [ ] Task 3
   
   ### Next Up
   - [ ] Upcoming task 1
   - [ ] Upcoming task 2
   ```

6. **Update TODO Tab**: If the user confirms, help them add these items to their to-do list

## Output Format
- Use markdown formatting
- Include completion percentages
- Highlight the current focus area
- Show both completed and pending items

## Example Usage

**User**: "What's next in the project?"

**Assistant**: 
1. Reads TODO.md and ROADMAP.md
2. Determines Phase 1 is 60% complete
3. Presents remaining Phase 1 tasks
4. Suggests focusing on: "Configure ESLint, Prettier, Rustfmt" and "FTS5 for full-text search"

**User**: "Show me the roadmap progress"

**Assistant**:
1. Parses both files
2. Calculates completion for all phases
3. Shows:
   - Phase 1: 60% complete
   - Phase 2: 0% complete
   - Phase 3: 0% complete
   - etc.
4. Highlights current focus area
