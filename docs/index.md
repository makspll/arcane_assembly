# Design Document


## Gameplay

`Arcane Assembly` (Working title) is a 2d, roguelike, side-view, platformer inspired mainly by noita and other games like primordialis or balatro.

The main premise is the player progresses through a series of levels with different environments and characteristics, to try and beat the game and final boss. 

The difficulty arises from needing to learn:
- Enemy movement patterns
- Spell building mechanisms
- Traversal strategies

And only those skills persist with the player in-between playthroughs on top of some minor collectibles and/or unlockables.

The environment is highly randomised within reason, and many possible combinations of enemy behaviors as well as environments prevent easy traversal through the levels.


### Spell & Ability building Mechanics
The player has access to a set of **spell** slots (limited? unlimited?), which provide the player with the ability to combine **Spell Components** and affect the environment. Whether that be:
- Manipulating parts of the map
- Inflicting damage on enemies
- Moving the player or enemies around

**Spells** are composed of **Spell Components**, These could be:
- The payload (determines the projectile movement and sprite)
- Trail (of other spells?, particles?)
- Area effects (i.e. sphere around the projectile inflicts damage, which also provides a visual component)
- Stat improvements (i.e. increased projectile speed, increased area of effect radius if present)
- Utility effects (i.e. teleport shooter to location on impact, Heal impacted creature etc.)

**Spell Components** are combined into spells at a high level, for example a spell could combine a component that fires a basic projectile, with a component that creates a persistent area of effect damaging field at the location of impact, which also teleports nearby creatures to this same location.

**Spell Components** compose via **Children** slots, for example a "Sequence" spell might have 2 slots available, and any other **Spell Components** can be slotted into those slots.
**Spell Component** nodes can also be **Modified** for example by adding follow up nodes on hit, or other events, this allows for complex behaviours to emerge.

When "Sequence" is cast, its logic is run first, if there are any present, and then the childrens'. If the children have any of their own children, execution follows a depth-first-search pattern.

An example spell tree which casts 3 fireballs in sequence, 3 times, with each last fireball, spawning an explosion at the hit location: 
```json
      "Loop x 3"
          |
      "Sequence"
    /           \
"Fireball"  "Sequence"
            /          \
        "Fireball"    "Fireball" 
                          |
                          | On Hit Modifier
                          |
                      "Explode"
```

Scripts that run these will live in the same context, meaning they can modify the current cast's properties.
For example the execution might initialize `cast_location` to the wand tip, and a spell later might modify this. Notably this sort of context resets between after modifiers execute. So after a hit for example `cast_location` would reset to the hit location by the modifier, as this represents a new execution context. The way this context propagates itself can be a way to modify spell behaviour as needed.

**Modifier** slots may be unlockable as the player progresses through the game. For example he may be given a choice to unlock a specific modifier for a subset of spells available. This forces the player to make interesting choices.

### Map Generation

**Levels** are randomly and procedurally (deterministically) generated based on a random seed.

There are a set number of **Levels** in a given **Playthrough** (5 ? 10 ?), each with different characteristics. This could mean:
- Different enemy spawns
- Different art style
- Different environmental hazards (traps, or dangerous objects, pits)

Each **Level** must provide at least one traversable path from the start to the end, where the player can progress to the next level. There can be multiple exits as long as they lead to the next level.

**Levels** will either be generated at "pixel" granuality like Noita, or at "feature" granuality, where pre-existing sprites are combined to form a **level**.

### Enemey Generation
**Enemies** are either pre-set combinations of:
- Movement patterns
- Sprites
- Spells & Attacks

Or procedurally generated, where the **Leg** component decides the movement patterns, while the **Torso** component modifies basic stats, and the **Weapon**/**Hands** component, generates a set of abilities / spells.

All of which are driven by generic scripted logic that is flexible enough to allow any part to work with any compatibile (compatibility as defined by sizes or visual characteristics) part


### Physics & Environment


## Engine & Tech

### Moddable / Scripted
The main game logic and UI should be driven by scripts via exposed Rust bindings, so that players can install mods which modify game logic and add other features. The core scripts are essentially treated as "Mods" by the engine in every sense apart from being loaded slightly earlier.

Core logic should be overridable too, by either (or both):
- Allowing mods to load "Override" scripts that load additional methods and override existing functions in the core script contexts (i.e. override `on_collision` callback in some core script which decides interactions between creatures)
- Allowing mods the same power as core mods and hooking into all the same callbacks so they can modify the same parts of the game (i.e. `on_level_start` callback which spawns some new map features) .

### Singleplayer / Multiplayer ?

Multiplayer would be considerably more time intensive to build, but we should consider if we want to support it down the line. Ideally everything should be:
- Deterministic
- Work in an peer to peer server context (same inputs = same outputs) with a little correction from the authorative peer every now and then.

### Physics

We need to consider how physics based the game will be. Noita for example leans heavily on a pixel-level "everything is destructible" mechanic, whereas we can focus on different gameplay aspects. Doing that level of simulation will probably be a considerable time investment.

## Licensing & Rights

I propose that all internal contributors reserve equal copyright granted by a license we choose to the project. Should the project generate any income, income should be distributed equally amongst all contributors. Where internal contributors are those who agreed to participate and have been added to the license. Any other external contributions (for example unrelated people who decided to submit a PR to add a feature) do not claim any copyright or claim to generated funds. 

The license should be compatibile with a "source available" mindset but not allow external contributors to generate income from re-distributions of the software.
