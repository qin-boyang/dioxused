# Development

Your new bare-bones project includes minimal organization with a single `main.rs` file and a few assets.

```
project/
├─ assets/ # Any assets that are used by the app should be placed here
├─ src/
│  ├─ main.rs # main.rs is the entry point to your application and currently contains all components for the app
├─ Cargo.toml # The Cargo.toml file defines the dependencies and feature flags for your project
```

### Automatic Tailwind (Dioxus 0.7+)

As of Dioxus 0.7, there no longer is a need to manually install tailwind. Simply `dx serve` and you're good to go!

Automatic tailwind is supported by checking for a file called `tailwind.css` in your app's manifest directory (next to Cargo.toml). To customize the file, use the dioxus.toml:

```toml
[application]
tailwind_input = "my.css"
tailwind_output = "assets/out.css" # also customize the location of the out file!
```

### Tailwind Manual Install

To use tailwind plugins or manually customize tailwind, you can can install the Tailwind CLI and use it directly.

### Tailwind
1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the Tailwind CSS CLI: https://tailwindcss.com/docs/installation/tailwind-cli
3. Run the following command in the root of the project to start the Tailwind CSS compiler:

```bash
npx @tailwindcss/cli -i ./input.css -o ./assets/tailwind.css --watch
```

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve
```

To run for a different platform, use the `--platform platform` flag. E.g.
```bash
dx serve --platform desktop
```

To create bundle for release, use the `--release` flag.
```bash
dx bundle --platform web --release
```
Bundle for web can be found in `target/dx/dioxused/releae/web/public/`

### Emoji 
Trading/Financial:
💰 Money bag
💵 Dollar
💶 Euro
💷 Pound
💴 Yen
📈 Chart increasing
📉 Chart decreasing
💹 Chart with upward trend
🏦 Bank
💳 Credit card
🧾 Receipt
📊 Bar chart
⚖️ Balance scale
🎯 Target
📌 Pin
💎 Gem/Diamond
🪙 Coin
💱 Currency exchange
💲 Heavy dollar sign

Status/Results:
✅ Check mark
❌ Cross mark
⚠️ Warning
ℹ️ Information
🔄 Refresh/Loading
🔁 Repeat
▶️ Start
⏸️ Pause
⏹️ Stop
⏭️ Next
⏮️ Previous
✨ Sparkles (success)
🎉 Party popper (success)
💥 Collision (error)
🔔 Bell (notification)
🔕 Bell with slash (muted)
