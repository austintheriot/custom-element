# PR #7 live example run

- Date: 2026-10-10
- Browser: Chromium, driven by Playwright 1.64.0
- Servers: `trunk serve` for `hello_world`, `counter`, `cell_state` and
  `refcell_state`. Vite 5.2.10 for `leptos_in_react`.

| Example | Steps | Result | Console |
|---|---|---|---|
| `hello_world` | Load the page. Create a second element with `document.createElement`. | Both elements show "Hello, world!" in a shadow root. The created element is not an `HTMLUnknownElement`. | 0 errors |
| `counter` | Click `+` once and the increment button twice. Move the element with `moveBefore`, then click the increment button once. | Count 0, then 4 with step 2, then 6 after the move. | 0 errors |
| `cell_state` | Click Increment three times. Move the element with `moveBefore`, then click once. | Count 3, then 4 after the move. | 0 errors |
| `refcell_state` | Click Increment three times. Move the element with `moveBefore`, then click once. | "Count: 3; updates: 3", then "Count: 4; updates: 4". | 0 errors |
| `leptos_in_react` | Show the app, increment by 100 from React, decrement from Leptos, hide the app. | The count follows both buttons. An interval in the example adds 1 each tick. Hiding removes the app. | 0 errors |

Each trunk page logs two warnings from the trunk-generated loader: the
preload `integrity` attribute and the deprecated wasm-bindgen
initialization parameters. The crate does not emit either warning.
