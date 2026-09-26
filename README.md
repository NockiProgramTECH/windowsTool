# Windows Maintenance Tool

## État : module 1 uniquement — validation Windows en attente

Socle Tauri 2 + React + TypeScript. L'interface affiche uniquement :

> Windows Maintenance Tool — Setup OK

Aucune commande métier, collecte de données, suppression, gestion de processus
ou demande d'élévation n'est implémentée. Les modules 2 à 4 restent bloqués tant
que le module 1 n'a pas été testé et confirmé sur une vraie machine Windows.

## Prérequis Windows

- Windows 10/11 avec Microsoft Edge WebView2 Runtime.
- Visual Studio Build Tools 2022 : charge de travail **Développement Desktop en C++**
  et SDK Windows.
- Node.js 22.12+ avec npm.
- Rust stable **1.95 ou supérieur**, installé via rustup, cible MSVC
  (`stable-x86_64-pc-windows-msvc` pour un PC x64).
- CLI Cargo Tauri 2 : `cargo install tauri-cli --version "^2" --locked`.

Voir les [prérequis officiels Tauri](https://v2.tauri.app/start/prerequisites/).

## Démarrer (PowerShell, à la racine du dépôt)

```powershell
npm ci
cargo tauri dev
```

Tauri démarre Vite automatiquement sur le port 1420 puis ouvre la fenêtre native.
Ne lancez pas un second Vite sur ce port. Quittez avec Ctrl+C.
La CLI npm fournie permet aussi `npm run tauri -- dev`.

## Compiler et vérifier l'absence de warnings

```powershell
npm ci
$env:RUSTFLAGS = "-D warnings"
cargo fmt --manifest-path src-tauri/Cargo.toml --all -- --check
cargo tauri build --no-bundle
cargo test --manifest-path src-tauri/Cargo.toml --all-targets
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings
Remove-Item Env:RUSTFLAGS
```

Le build Tauri lance automatiquement `npm run build` (vérification TypeScript
stricte puis bundle Vite). L'exécutable est produit dans
`src-tauri/target/release/windows-maintenance-tool.exe`.
La création d'installateurs est volontairement désactivée pour ce module.

Le workflow `.github/workflows/windows.yml` automatise ces contrôles sous Windows
après un push/une PR. Il ne remplace pas le test manuel de la fenêtre native.
Il a été ajouté, mais n'a pas été exécuté dans cette session.

## Structure

```text
src/                         # React TSX minimal
src-tauri/
├── Cargo.toml               # chemins lib/bin explicites
├── build.rs                 # génération Tauri
├── tauri.conf.json
├── icons/
├── lib.rs                   # exposition des modules backend
├── main.rs                  # initialisation Tauri + registre IPC vide
├── commands/
│   ├── mod.rs
│   └── system.rs            # réservé, sans commande métier
├── services/mod.rs          # réservé
├── models/mod.rs            # réservé
└── errors/mod.rs            # AppError + tests
```

Les chemins Cargo explicites respectent l'arborescence demandée, sans dossier
`src-tauri/src` intermédiaire. Les modules publics de la bibliothèque constituent
le socle du backend, sans masquer les warnings avec `allow(dead_code)`.

`AppError` implémente `Serialize`, `Display` et `std::error::Error`.
Ses trois variantes sont `SystemError`, `PermissionDenied` et `NotSupported`.
Le format IPC sera un objet typé, par exemple :

```json
{"type":"PermissionDenied","message":"Accès refusé"}
```

Deux tests vérifient les traits et la sérialisation des trois variantes.
`sysinfo = "0.39.6"` est déclaré pour les prochains modules (version stable
identifiée dans le [registre](https://crates.io/crates/sysinfo)). Il n'est pas
encore utilisé pour collecter des données.

## Vérifications réalisées dans le sandbox Linux

- `npm install` : réussi, audit sans vulnérabilité signalée.
- `npm run build` : réussi, aucun warning émis.
- `git diff --check` : réussi.
- `npx tauri info` : configuration détectée ; Rust/Cargo et les bibliothèques
  natives Linux nécessaires sont absents.
- Compilation Rust, tests Rust et lancement Windows : **non exécutés**.
  L'accès direct aux serveurs Rust a également échoué ; aucune confirmation
  de compilation native sans warning n'est donc revendiquée.

`package-lock.json` est inclus. `Cargo.lock` devra être généré lors du premier
build Rust réussi et ajouté au dépôt pour figer les dépendances de l'application ;
il n'a pas été fabriqué manuellement.

## Validation obligatoire avant le module 2

Sur une vraie machine Windows :

- [ ] Les commandes de build, tests et Clippy ci-dessus passent sans warning.
- [ ] `cargo tauri dev` ouvre l'application sans erreur.
- [ ] La fenêtre affiche uniquement le texte « Windows Maintenance Tool — Setup OK ».
- [ ] La structure est conforme et `main.rs` ne contient aucune logique métier.
- [ ] Les tests d'`AppError` passent.
- [ ] L'utilisateur confirme explicitement le fonctionnement du module 1.

Pour vérifier seulement le frontend : `npm run dev`. Cet aperçu navigateur
ne valide ni Rust, ni IPC, ni la fenêtre native Windows.
