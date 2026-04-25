# Current Mod Structure
Right now I am thinking this is how mods look like:

```json
assets
    <mod_name>
        <mod_name>.mod.json // describes the mod, and its contents, including spells weapons etc
        <mod_name>.lua      // the main entrypoint of the script
        assets/             // where assets which mods can access live 
        spells/             // spell scripts refered to by the .mod.json file
```

When starting up, the game looks for .mod.json files and all assets become the dependency of that file, meaning changes to any file will re-load the entire mod.