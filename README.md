# hanni
Unofficial rust desktop wrapper for TETR.IO

> [!IMPORTANT]
> This project is incomplete, majority of the TETR.IO Desktop settings don't work even if they seem to be available.

## Development
Follow the [Tauri setup guide](https://tauri.app/start/prerequisites/), ignoring the iOS and Android setups (for now?...)

### Proxy
This project uses a [SOCKS5 proxy](socks5-proxy/src/main.rs) to modify web requests.<br/>
<sub>*Can't possibly imagine why you would want that, right?*</sub>

macOS / Linux
```bash
./scripts/build-proxy.sh
```

Windows
```bash
./scripts/build-proxy.ps1
```

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
