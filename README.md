# Poker at Berkeley - Website

A modern website for Poker at Berkeley

### install deps (MacOS / Linux)

```bash
./setup.sh
```

### starting local dev server

```bash
npm run dev
```

### building

```bash
npm run build
```

The built site is in `dist/`

## deploying

```bash
npm run deploy # deploys to OCF only (poker.berkeley.edu) ; will prompt you for OCF password

npm run deploy:all # deploys to pokeratberkeley.com and poker.berkeley.edu ; will prompt you for OCF password
```
