---
description: Initialize and implement إملاء صوتي عربي from specification
---

Read the following files in order:
1. CLAUDE.md - Contains MANDATORY instructions you MUST follow
2. project-spec.md - Full project specification

## Before Starting
- Use the MCP servers and plugins already available in your Claude Code environment for documentation lookups
- Confirm you understand version requirements in CLAUDE.md
- Create fix.md file for logging issues

## Implementation Steps
1. **FIRST**: Run /nuxt-ui-remote:setup_project_with_template and select the **Dashboard** template
2. Copy CLAUDE.md, fix.md, .env.example to the new project root
3. Implement features phase by phase as defined in project-spec.md
4. After implementation, create README.md with complete setup and run instructions
5. Log any issues encountered in fix.md

## Critical Rules
- **MCP FIRST**: Use ALL available MCP servers and plugins in your environment — query them before creating ANY file, do NOT rely on training data for framework APIs
- NEVER use versions different from specification
- ALWAYS create README.md with setup and run instructions after project is ready
- ALWAYS log issues in fix.md with date, problem, and solution
- NO COMMENTS in code files
- DO NOT manually create nuxt.config.ts or install Nuxt UI - use /nuxt-ui-remote:setup_project_with_template

## Version Requirements
- Frontend: Nuxt (Latest)
- UI Library: Nuxt UI (Latest)
- Backend: Tauri (Latest)
- Database: SQLite (Latest)

Reply "Ready to implement إملاء صوتي عربي" when you've read all files.