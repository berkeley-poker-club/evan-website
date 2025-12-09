# Poker at Berkeley Website

### Install deps

```bash
./setup.sh
```

### Starting local dev server (preview website locally with hot reloading)

```bash
cargo tauri dev
```
or:

```bash
npm run dev
```

### Building

```bash
cargo tauri build
```

or:

```bash
npm run build
```

The built site is in `dist/`

## Deploying

```bash
npm run deploy # deploys to OCF only (poker.berkeley.edu) ; will prompt you for OCF password

npm run deploy:all # deploys to pokeratberkeley.com and poker.berkeley.edu ; will prompt you for OCF password
```

## Documentation on Libraries

[Tauri](https://v2.tauri.app)

[Leptos](https://book.leptos.dev)
