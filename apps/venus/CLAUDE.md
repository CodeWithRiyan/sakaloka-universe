# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

- **Development server**: `pnpm dev` (or `npm run dev`) - Starts Vite development server with HMR
- **Build**: `pnpm build` (or `npm run build`) - TypeScript compilation followed by production build
- **Lint**: `pnpm lint` (or `npm run lint`) - Run ESLint on the codebase
- **Format**: `pnpm format` (or `npm run format`) - Format code using Prettier
- **Preview**: `pnpm preview` (or `npm run preview`) - Preview production build locally

## Architecture Overview

This is a modern React application built with:

### Tech Stack

- **Framework**: React 19 with TypeScript
- **Build Tool**: Vite 7
- **Styling**: Tailwind CSS 4 (using @tailwindcss/vite)
- **State Management**: Redux Toolkit with RTK Query
- **Routing**: React Router 7
- **UI Components**: Radix UI primitives with shadcn/ui design system
- **Animations**: Framer Motion
- **Forms**: React Hook Form with Zod validation

### Project Structure

```
src/
├── components/
│   ├── auth/           # Authentication components (ProtectedRoute)
│   ├── layouts/        # Layout components (MainLayout, Navbar)
│   └── ui/             # shadcn/ui components and design system
├── hooks/              # Custom React hooks (breakpoint, mobile detection)
├── lib/                # Utilities (cn function, reveal animations)
├── pages/              # Page components (lazy-loaded)
├── routes/             # Routing configuration
├── store/              # Redux store setup and slices
└── types/              # TypeScript type definitions
```

### Key Architecture Patterns

**Routing Structure**: Uses a nested routing system with public and private routes. Routes are configured in `src/routes/routes.tsx` with lazy loading for code splitting. Protected routes use `ProtectedRoute` wrapper for authentication.

**State Management**: Redux Toolkit store configured in `src/store/index.ts` with middleware and root reducer/middleware patterns. API calls handled through RTK Query.

**Component Organization**:

- UI components follow shadcn/ui conventions in `components/ui/`
- Layout components provide consistent page structure
- Lazy loading implemented for route-level code splitting

**Styling**: Uses Tailwind CSS 4 with the new Vite plugin. Custom utilities in `lib/utils.ts` include the `cn()` function for conditional classes. Theme system supports dark/light mode.

### Import Aliases

- `@/` maps to `./src/` directory
- All major folders have dedicated aliases in both Vite and TypeScript configs

### Design System

Uses shadcn/ui (New York variant) with:

- Radix UI primitives for accessibility
- Lucide React for icons
- CSS variables for theming
- Gray as base color

### Development Notes

- Package manager: Uses `pnpm` (lock file present)
- TypeScript: Strict configuration with project references
- ESLint: Configured with React hooks and refresh plugins
- Prettier: Configured with import sorting and Tailwind class sorting
