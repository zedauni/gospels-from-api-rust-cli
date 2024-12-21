# Petit CLI pour obtenir des données sur les évangiles à partir d'une API

Application en ligne de commande écrite en Rust qui extrait des données d'évangiles à partir d'une API pour une date spécifique, un mois entier ou une plage de mois. Les données extraites sont sauvegardées sous forme de fichiers JSON locaux épurés.

## Caractéristiques

- Extraction de données d'évangiles pour une date spécifique.
- Extraction de données d'évangiles pour un mois entier.
- Extraction de données d'évangiles pour une plage de mois.
- Sauvegarde des résultats au format JSON.

## Utilisation

### Télécharger la dernière version



Ensuite, exécutez le programme en utilisant les options suivantes :

### Obtenir des données sur les évangiles pour une date spécifique
```bash
./gospels --day YYYY-MM-DD

Ex : ./gospels 2024-12-25
```

### Obtenir les données des évangiles pour un mois entier
```bash
./gospels -mois AAAA-MM

Ex : ./gospels 2024-12
```

### Obtenir les données de gospels pour une plage de mois
```bash
./gospels --range YYYY-MM YYYY-MM

Ex : ./gospels 2024-12 2025-02
```

Les fichiers JSON seront générés dans le répertoire courant.

### Sortie actuelle de l'Aide (gospels -h)

```
gospels 0.1.0
Gospels via API

USAGE:
    gospels [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --day <day>           Date spécifique à traiter (YYYY-MM-DD)
    -m, --month <month>       Mois à traiter (YYYY-MM)
    -r, --range <range>...    Plage de mois à traiter (YYYY-MM YYYY-MM)
```

## Exemple de contenu généré

Pour une date spécifique, un fichier JSON sera créé avec la structure suivante :

```json
{
  "2024-03-10": [
    {
      "content": "...Car Dieu a tellement aimé le monde<br />\nqu’il a donné son Fils unique,<br />\nafin que quiconque croit en lui ne se perde pas,<br />\nmais obtienne la vie éternelle...",
      "titre": "« Dieu a envoyé son Fils pour que, par lui, le monde soit sauvé »",
      "ref": "Jn 3, 14-21"
    }
  ]
}
```

## Développement

1. Prérequis pour le développement
  - [Rust](https://www.rust-lang.org/tools/install) installé sur votre système.
  - Connexion Internet pour accéder à l'API.

2. Clonez le dépôt :
   ``bash
   git clone https://github.com/zedauni/gospels-from-api-rust-cli.git
   cd gospels-from-api-rust-cli
   ```

3. Exécutez le projet :
   ``bash
   cargo run
   ```

4. Ou construisez le projet :
   ``bash
   cargo build --release
   ```

## Contributions

Les contributions sont les bienvenues ! N'hésitez pas à ouvrir une issue ou une pull request pour améliorer le projet.

## Licence

Ce projet est sous licence MIT. Voir le fichier [LICENSE](LICENSE) pour plus de détails.

## Remerciements

Dans la version actuelle, l'API utilisée est celle fournie par [AELF](https://api.aelf.org). Attention, [AELF consent aux internautes le droit de reproduire tout ou partie du contenu du site pour stockage aux fins de représentation sur écran monoposte et de reproduction, en un exemplaire, pour copie de sauvegarde ou tirage sur papier](https://www.aelf.org/page/conditions-generales-dutilisation). Ce droit est limité à un usage strictement personnel, privé et non collectif, et toute mise en réseau, toute rediffusion, toute commercialisation totale ou partielle auprès de tiers est strictement interdite, sous quelque forme que ce soit.


Il est de votre responsabilité de vous conformer aux Conditions Générales d'Utilisation du site et des données de l'AELF. Dans le cas contraire, vous devez modifier le code source du projet pour utiliser l'API de votre choix. 

Merci à [API AELF] (https://www.aelf.org/page/mentions-legales) pour la mise à disposition des données liturgiques.