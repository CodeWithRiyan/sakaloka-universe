import js from "@eslint/js"
import reactHooks from "eslint-plugin-react-hooks"
import reactRefresh from "eslint-plugin-react-refresh"
import { globalIgnores } from "eslint/config"
import globals from "globals"
import tseslint from "typescript-eslint"

export default tseslint.config([
  globalIgnores([
    "dist",
    "src/pages/landing/**",
    "src/components/layouts/public/**",
    "src/components/custom/accordion.tsx",
    "src/components/custom/orbiting-circles.tsx",
    "src/components/custom/pricing.tsx",
    "src/components/custom/star-rating.tsx",
    "src/components/custom/word-rotate.tsx",
  ]),
  {
    files: ["**/*.{ts,tsx}"],
    extends: [
      js.configs.recommended,
      tseslint.configs.recommended,
      reactHooks.configs["recommended-latest"],
      reactRefresh.configs.vite,
    ],
    rules: {
      "no-restricted-imports": [
        "error",
        {
          patterns: [
            {
              group: [
                "@/pages/landing",
                "@/pages/landing/*",
                "**/pages/landing",
                "**/pages/landing/*",
                "@/components/layouts/public",
                "@/components/layouts/public/*",
                "**/components/layouts/public",
                "**/components/layouts/public/*",
                "@/components/custom/accordion",
                "@/components/custom/orbiting-circles",
                "@/components/custom/pricing",
                "@/components/custom/star-rating",
                "@/components/custom/word-rotate",
              ],
              message:
                "Marketing-only landing code is quarantined in Venus. Keep it out of the app bundle and migrate it to the centralized Strapi-managed landing site instead.",
            },
          ],
        },
      ],
    },
    languageOptions: {
      ecmaVersion: 2020,
      globals: globals.browser,
    },
  },
])
