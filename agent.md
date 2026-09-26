Prompts pour agents IA — Windows Maintenance Tool (Phase 1)
Ces prompts sont conçus pour être donnés un par un à un agent IA (Claude Code, Cursor, etc.). Chaque module doit être validé sur une vraie machine Windows avant de passer au suivant. Ne donne jamais le module N+1 tant que le module N n'a pas été testé et confirmé fonctionnel.

Module 1 — Setup du projet Tauri + Rust
Prompt à donner à l'agent :

Crée un nouveau projet Tauri avec template React + TypeScript (TSX). 
Structure le backend Rust dès le départ selon cette arborescence :

src-tauri/
├── commands/
│   ├── mod.rs
│   └── system.rs
├── services/
├── models/
├── errors/
└── main.rs

Ajoute la crate `sysinfo` (dernière version stable) dans Cargo.toml.
Crée un module d'erreurs custom (errors/mod.rs) avec un enum AppError 
qui implémente Serialize (pour pouvoir le renvoyer au frontend), et qui 
distingue au minimum : SystemError, PermissionDenied, NotSupported.

Ne mets aucune logique métier dans main.rs — uniquement l'initialisation 
de l'app Tauri et l'enregistrement des commandes.

Le frontend doit juste afficher "Windows Maintenance Tool — Setup OK" 
pour l'instant, rien d'autre.

Livre : le projet complet, avec instructions de build (cargo tauri dev) 
et confirmation que ça compile sans warning.
Checklist de validation avant de continuer :

 cargo tauri dev lance l'app sans erreur sur Windows
 Structure de dossiers conforme (pas de logique dans main.rs)
 Le module errors compile et est bien typé
Module 2 — Dashboard système en lecture seule
Prompt à donner à l'agent :

Dans commands/system.rs, crée une commande Tauri `get_system_info` qui 
retourne une struct SystemInfo (Serialize) contenant :
- cpu_usage_percent: f32
- cpu_model: String
- ram_total_gb: f32
- ram_used_gb: f32
- disks: Vec<DiskInfo> (chaque DiskInfo : name, total_gb, free_gb)
- os_version: String

Utilise la crate sysinfo pour récupérer ces données réelles — aucune 
valeur ne doit être inventée ou simulée. Si une information n'est pas 
disponible sur la machine, retourne None plutôt qu'une fausse valeur.

Gère les erreurs proprement via AppError (module errors) et propage-les 
au frontend sous une forme structurée, pas juste une string.

Côté React : crée un composant Dashboard qui appelle get_system_info 
au montage et affiche les données dans des cartes simples (pas besoin 
de design poussé à ce stade). Ajoute un état de chargement et un état 
d'erreur visible si la commande échoue.

Ne mets aucun polling pour l'instant — un seul appel au chargement de 
la page suffit pour ce module.
Checklist de validation avant de continuer :

 Les chiffres affichés correspondent à ce que montre le Gestionnaire des tâches Windows
 Aucune valeur "en dur" ou placeholder dans le code
 L'état d'erreur s'affiche si on simule une panne (ex. désactiver temporairement l'appel)
Module 3 — Storage Scanner (scan uniquement, aucune suppression)
Prompt à donner à l'agent :

Crée une commande Tauri `scan_junk_files` qui analyse UNIQUEMENT ces 
emplacements (aucun autre chemin, pas de récursion en dehors de ces 
dossiers) :
- %TEMP% (fichiers temporaires utilisateur)
- C:\Windows\Temp (si accessible sans élévation, sinon retourner 
  NotSupported pour cette catégorie)
- Corbeille (via l'API Windows appropriée)

Pour chaque catégorie, retourne : nom de la catégorie, nombre de 
fichiers, taille totale en octets. 

IMPORTANT : cette commande ne doit RIEN supprimer. C'est un scan pur. 
La suppression sera un module séparé, ajouté seulement après validation 
explicite du scan.

Valide chaque chemin avant de le lister (vérifier qu'il existe, qu'il 
est bien dans la liste blanche autorisée — pas de chemin construit 
dynamiquement à partir d'une entrée utilisateur).

Côté React : affiche les résultats du scan dans un tableau avec 
checkbox par catégorie (les checkboxes ne servent à rien pour l'instant, 
elles préparent juste l'UI du futur module de suppression).
Checklist de validation avant de continuer :

 Les tailles annoncées correspondent à peu près à ce que montre l'Explorateur Windows
 Aucun appel de suppression n'existe dans le code à ce stade
 Les chemins scannés sont bien limités à la liste blanche (vérifier dans le code, pas juste à l'exécution)
Module 4 — Process Manager en lecture seule
Prompt à donner à l'agent :

Crée une commande Tauri `get_processes` qui retourne la liste des 
processus en cours via sysinfo : nom, PID, % CPU, RAM utilisée (Mo), 
chemin de l'exécutable si disponible.

Pas de kill_process dans ce module — uniquement la lecture.

Côté React : affiche la liste dans un tableau triable (par nom, CPU, 
ou RAM). Rafraîchis les données toutes les 2 secondes via un 
setInterval côté frontend pour l'instant (on optimisera avec les 
événements Tauri dans un module ultérieur si nécessaire).

Vérifie que l'affichage ne "saute" pas ou ne clignote pas de manière 
gênante à chaque rafraîchissement.
Checklist de validation avant de continuer :

 La liste correspond à peu près à celle du Gestionnaire des tâches
 Le rafraîchissement toutes les 2s ne bloque pas l'UI
 Aucune fonction de kill/suspend n'existe encore dans le code
Après ces 4 modules
Tu as un squelette Phase 1 fonctionnel, entièrement en lecture seule — aucun risque de casser quoi que ce soit chez un utilisateur final. C'est le bon moment pour :

Faire une revue de code générale (structure, gestion d'erreurs, cohérence)
Décider si tu passes à la suppression réelle (Safe Cleanup) ou si tu avances d'abord sur le reste de la Phase 1 (Network Diagnostics)
Vérifier qu'aucun module n'a introduit de fonctionnalité "fictive" non prévue par le cahier des charges