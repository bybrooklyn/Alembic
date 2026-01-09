/** @type {import('tailwindcss').Config} */
export default {
    content: ['./src/**/*.{astro,html,js,jsx,md,mdx,svelte,ts,tsx,vue}'],
    theme: {
        extend: {
            fontFamily: {
                sans: ['Space Grotesk', 'sans-serif'],
                mono: ['IBM Plex Mono', 'monospace'],
                display: ['Space Grotesk', 'sans-serif'],
            },
            colors: {
                helios: {
                    main: 'rgb(var(--bg-main) / <alpha-value>)',
                    panel: 'rgb(var(--bg-panel) / <alpha-value>)',
                    elevated: 'rgb(var(--bg-elevated) / <alpha-value>)',
                    solar: 'rgb(var(--accent-primary) / <alpha-value>)',
                    'solar-dim': 'rgb(var(--accent-secondary) / <alpha-value>)',
                    ink: 'rgb(var(--text-primary) / <alpha-value>)',
                    'ink-muted': 'rgb(var(--text-muted) / <alpha-value>)',
                    border: 'rgb(var(--border-subtle) / <alpha-value>)',
                },
                status: {
                    success: 'rgb(var(--status-success) / <alpha-value>)',
                    warning: 'rgb(var(--status-warning) / <alpha-value>)',
                    error: 'rgb(var(--status-error) / <alpha-value>)',
                }
            },
        },
    },
    plugins: [],
}
