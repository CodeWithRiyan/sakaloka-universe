# UI Components

Venus uses **shadcn/ui** (Radix UI primitives + Tailwind CSS) as its component library,
with custom POS-specific components layered on top.

## Component Organization

```
components/
├── ui/                   # shadcn/ui base components (~40 files)
├── custom/               # Project-specific components
├── auth/                 # Authentication components
├── layouts/              # Layout shells
│   ├── public/           # MainLayout + Navbar (landing/auth pages)
│   └── private/          # DashboardLayout + Sidebar + Settings
└── loader.tsx            # Loading spinner
```

## shadcn/ui Components

All standard shadcn/ui components are available:

**Forms:** Input, Select, Checkbox, Radio Group, Slider, Switch, Textarea, DatePicker,
InputCurrency (custom currency input)

**Overlays:** Dialog, Sheet, Popover, Tooltip, Dropdown Menu, Context Menu, Command Palette,
Alert Dialog

**Layout:** Card, Tabs, Accordion, Separator, Collapsible, Resizable, Sidebar

**Data:** Table, Pagination, Skeleton (loading states)

**Feedback:** Sonner (toast notifications), Form errors

**Navigation:** Navigation Menu, Menubar

**Charts:** Recharts integration via Chart component

## Custom Components

| Component | File | Description |
|-----------|------|-------------|
| `TableData` | `custom/table-data/index.tsx` | TanStack Table wrapper with sorting, pagination |
| `BasicPagination` | `custom/basic-pagination.tsx` | Simple page navigator |
| `SortButton` | `custom/sort-button.tsx` | Column sort toggle |
| `InputSearch` | `custom/input-search.tsx` | Debounced search input |
| `Modal` | `custom/modal.tsx` | Generic modal wrapper |
| `ModalConfirm` | `custom/modal-confirm.tsx` | Confirmation dialog |
| `Upload` | `custom/upload.tsx` | File upload component |
| `StarRating` | `custom/star-rating.tsx` | Star rating display |
| `Pricing` | `custom/pricing.tsx` | Price display with IDR formatting |
| `FormError` | `custom/form-error.tsx` | Form validation error display |
| `FormSuccess` | `custom/form-success.tsx` | Form success message |

## Animation Components

| Component | Description |
|-----------|-------------|
| `ShiNeBorder` | Animated gradient border effect |
| `AuroraText` | Animated gradient text |
| `AnimatedGridPattern` | Background grid animation |
| `Meteors` | Falling meteor particle effect |
| `OrbitingCircles` | Orbiting circle animation |
| `WordRotate` | Rotating word animation |
| `SparklesText` | Text with sparkle effect |

## Theme

- **Dark/Light mode** via `ThemeProvider` at the app root
- CSS variables for color tokens
- Tailwind CSS 4.1 with `@tailwindcss/vite` plugin
- Utility function: `cn()` (clsx + tailwind-merge)

## Currency & Locale

| Utility | Purpose |
|---------|---------|
| `formatCurrency(num)` | Indonesian locale number formatting |
| `formatIDR(price)` | Format as IDR currency (Rp 25.000) |
| `parseNumber(str)` | Strip non-digits from formatted string |
| `InputCurrency` | Form input with live IDR formatting |

All dates use `dayjs` with Indonesian locale (`id`).
