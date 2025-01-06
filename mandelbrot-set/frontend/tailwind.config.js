const { fontFamily } = require("tailwindcss/defaultTheme");
const plugin = require("tailwindcss/plugin");

/**
 * Adapted from https://github.com/tailwindlabs/tailwindcss/blob/master/src/featureFlags.js
 *
 * @param {import("tailwindcss").Config} config
 * @param {string} flag
 * @returns {boolean}
 */
const futureFlagEnabled = (config, flag) => {
  return config.future === "all" || (config?.future?.[flag] ?? false);
};

/**
 * @param {string} modifier
 * @param {string} prefix
 * @returns {string}
 */
const addModifierIfNeeded = (modifier, prefix) => {
  return modifier ? `${prefix}\\/${modifier}` : prefix;
};

const hocusPlugin = plugin(({ addVariant, matchVariant, config }) => {
  const hoverOnlyWhenSupported = futureFlagEnabled(
    config(),
    "hoverOnlyWhenSupported"
  );
  const wrapSelectorForHoverIfNeeded = (/** @type {string} */ selector) =>
    hoverOnlyWhenSupported
      ? `@media (hover: hover) and (pointer: fine) { ${selector} }`
      : selector;

  const hoverSelector = wrapSelectorForHoverIfNeeded("&:hover");

  addVariant("hocus", [hoverSelector, "&:focus"]);
  addVariant("hocus-within", [hoverSelector, "&:focus-within"]);
  addVariant("hocus-visible", [hoverSelector, "&:focus-visible"]);

  if (matchVariant) {
    const variantFocusSelectors = {
      hocus: "focus",
      "hocus-within": "focus-within",
      "hocus-visible": "focus-visible",
    };

    const variantsEndParts = {
      group: " &",
      peer: " ~ &",
    };

    for (const [name, selectorEnd] of Object.entries(variantsEndParts)) {
      matchVariant(
        name,
        (value, { modifier }) => {
          const hoverSelector = wrapSelectorForHoverIfNeeded(
            `:merge(.${addModifierIfNeeded(
              modifier,
              name
            )}):hover${selectorEnd}`
          );
          const focusSelector = `:merge(.${addModifierIfNeeded(
            modifier,
            name
          )}):${value}${selectorEnd}`;

          return [hoverSelector, focusSelector];
        },
        { values: variantFocusSelectors }
      );
    }

    return;
  }

  const groupHoverSelector = wrapSelectorForHoverIfNeeded(
    ":merge(.group):hover &"
  );
  const peerHoverSelector = wrapSelectorForHoverIfNeeded(
    ":merge(.peer):hover ~ &"
  );

  addVariant("group-hocus", [groupHoverSelector, ":merge(.group):focus &"]);
  addVariant("group-hocus-within", [
    groupHoverSelector,
    ":merge(.group):focus-within &",
  ]);
  addVariant("group-hocus-visible", [
    groupHoverSelector,
    ":merge(.group):focus-visible &",
  ]);
  addVariant("peer-hocus", [peerHoverSelector, ":merge(.peer):focus ~ &"]);
  addVariant("peer-hocus-within", [
    peerHoverSelector,
    ":merge(.peer):focus-within ~ &",
  ]);
  addVariant("peer-hocus-visible", [
    peerHoverSelector,
    ":merge(.peer):focus-visible ~ &",
  ]);
});

/** @type {import("tailwindcss").Config} */
module.exports = {
  darkMode: ["class"],
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    screens: {
      xxs: "335px",
      xs: "390px",
      sm: "475px",
      md: "768px",
      lg: "1024px",
      xl: "1280px",
      "2xl": "1440px",
      "3xl": "1920px",
    },
    extend: {
      aria: {
        invalid: 'invalid="true"',
      },
      data: {
        checked: 'state="checked"',
        opened: 'state="open"',
        closed: 'state="closed"',
        "swipe-cancel": 'swipe="cancel"',
        "swipe-end": 'swipe="end"',
        "swipe-move": 'swipe="move"',
      },
      colors: {
        border: "hsl(var(--border))",
        input: "hsl(var(--input))",
        ring: "hsl(var(--ring))",
        background: "hsl(var(--background))",
        foreground: "hsl(var(--foreground))",
        primary: {
          DEFAULT: "hsl(var(--primary))",
          foreground: "hsl(var(--primary-foreground))",
        },
        secondary: {
          DEFAULT: "hsl(var(--secondary))",
          foreground: "hsl(var(--secondary-foreground))",
        },
        destructive: {
          DEFAULT: "hsl(var(--destructive))",
          foreground: "hsl(var(--destructive-foreground))",
        },
        muted: {
          DEFAULT: "hsl(var(--muted))",
          foreground: "hsl(var(--muted-foreground))",
        },
        accent: {
          DEFAULT: "hsl(var(--accent))",
          foreground: "hsl(var(--accent-foreground))",
        },
        popover: {
          DEFAULT: "hsl(var(--popover))",
          foreground: "hsl(var(--popover-foreground))",
        },
        card: {
          DEFAULT: "hsl(var(--card))",
          foreground: "hsl(var(--card-foreground))",
        },
      },
      borderRadius: {
        lg: `var(--radius)`,
        md: `calc(var(--radius) - 2px)`,
        sm: "calc(var(--radius) - 4px)",
      },
      fontFamily: {
        sans: ["var(--font-sans)", ...fontFamily.sans],
        mono: ["var(--font-mono)", ...fontFamily.mono],
      },
      spacing: {
        2.5: "0.625rem",
        4.5: "1.125rem",
        5.5: "1.375rem",
        6.5: "1.625rem",
        7.5: "1.875rem",
        8: "2rem",
        8.5: "2.125rem",
        9: "2.25rem",
        9.5: "2.375rem",
        11.5: "2.875rem",
        12: "3rem",
        12.5: "3.125rem",
        13: "3.25rem",
        14.5: "3.625rem",
        15: "3.75rem",
        17: "4.25rem",
        17.5: "4.375rem",
        18: "4.5rem",
        19: "4.75rem",
        22: "5.5rem",
        22.5: "5.625rem",
        25: "6.25rem",
        26: "6.5rem",
        27: "6.75rem",
        27.5: "6.875rem",
        30: "7.5rem",
        31: "7.75rem",
        32: "8rem",
        33: "8.25rem",
        34: "8.5rem",
        35: "8.75rem",
        36: "9rem",
        37: "9.25rem",
        37.5: "9.375rem",
        38: "9.5rem",
        42: "10.5rem",
        43: "10.75rem",
        44: "11rem",
        45: "11.25rem",
        46: "11.5rem",
        47: "11.75rem",
        48: "12rem",
        49: "12.25rem",
        50: "12.5rem",
        55: "13.75rem",
        56: "14rem",
        57: "14.25rem",
        58: "14.5rem",
        59: "14.75rem",
        60: "15rem",
        62: "15.5rem",
        62.5: "15.625rem",
        63: "15.75rem",
        64: "16rem",
      },
      keyframes: {
        "accordion-down": {
          from: { height: 0 },
          to: { height: "var(--radix-accordion-content-height)" },
        },
        "accordion-up": {
          from: { height: "var(--radix-accordion-content-height)" },
          to: { height: 0 },
        },
      },
      animation: {
        "accordion-down": "accordion-down 0.2s ease-out",
        "accordion-up": "accordion-up 0.2s ease-out",
      },
      borderWidth: {
        3: "3px",
        5: "5px",
        6: "6px",
        7: "7px",
        8: "8px",
      },
      zIndex: {
        1: "1",
        2: "2",
        60: "60",
      },
      fontSize: {
        xs: ["12px", "1.5"],
      },
      transitionDuration: {
        400: "400ms",
      },
      textUnderlineOffset: {
        3: "3px",
      },
    },
  },
  corePlugins: {
    container: false,
  },
  plugins: [
    hocusPlugin,
    require("tailwindcss-animate"),
    require("tailwindcss-radix"),
    require("@tailwindcss/container-queries"),
  ],
};
