# Karthus

## Table of Contents
- [Basic Information](#basic-information)
- [Statistics](#statistics)
  - [Map-specific Stats](#map-specific-stats)
- [Abilities](#abilities)
- [Patch History](#patch-history)
- [Trivia](#trivia)

## Basic Information

| Attribute | Value |
|-----------|------:|
| **Champion** | Karthus |
| **Title** | the Deathsinger |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2009-06-12 |
| **Release Patch** | June 12, 2009 Patch |
| **Latest Changes** | V25.18 |
| **Roles** | Battlemage |
| **Riot Positions** | Jungle |
| **External Positions** | Jungle |
| **Blue Essence** | 1575 |
| **Riot Points** | 790 |
| **Difficulty** | 2 |
| **Hero Type** | Mage |
| **Adaptive Type** | Magic |
| **Damage** | 3 |
| **Toughness** | 1 |
| **Control** | 1 |
| **Mobility** | 1 |
| **Utility** | 2 |
| **Style** | 100 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $620.0$ | $+110.0$ |
| **Mana** | $467.0$ | $+31.0$ |
| **Health Regen** | $6.5$ | $+0.55$ |
| **Mana Regen** | $8.0$ | $+0.8$ |
| **Armor** | $21.0$ | $+4.7$ |
| **Magic Resist** | $30.0$ | $+1.3$ |
| **Attack Damage** | $46.0$ | $+3.25$ |
| **Attack Speed** | $0.625$ | |
| **Movement Speed** | $335.0$ | $+0.0$ |
| **Attack Range** | $450.0$ | $+0.0$ |
| **Base Attack Speed** | $0.625$ | |
| **Attack Speed Ratio** | $0.625$ | |
| **Bonus AS per Level** | $2.1\%$ | |
| **Missile Speed** | $1500$ units/second | |
| **Acquisition Radius** | $450$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $100$ units | |
| **Selection Height** | $150$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $93.0\%$ |
| **Damage Taken** | $105.0\%$ |

#### URF

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $90.0\%$ |
| **Damage Taken** | $100.0\%$ |

## Abilities

### Passive: Death Defied

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Self |

**INNATE:** Upon taking fatal damage, **Karthus** enters a **zombie state** for 7 seconds, during which he can cast his abilities at no cost. If *Defile* has been learned, it will remain toggled on for *Death Defied*’s entire duration. *Requiem* becomes disabled after 4 seconds of the duration have elapsed.

While under this state, **Karthus** becomes untargetable and immune to crowd control as well as prevents all incoming damage, but is also rendered unable to move, declare basic attacks, use summoner spells, and activate items.

**Notes:**

- *Death Defied*’s untargetability does not destroy in-flight projectiles.
- **Karthus** cannot be executed by the Aspect of the Dragon during *Death Defied*.
- At the start of *Death Defied*, **Karthus** is set to (health) 1 health.
  - **Karthus** can still regenerate his health over the duration, but he will always die at the end of it.
- After *Death Defied* ends, the corpse of **Karthus** will retain unit collision despite being dead on the ground.
- **Karthus' ** mana bar drains over the duration of *Death Defied* as an indicator of his time remaining in this state.

---

### Q: Lay Waste

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 875 units |
| **Effect Radius** | 160 units |
| **Cost** | 20 / 25 / 30 / 35 / 40 Mana |
| **Cooldown** | 1 seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Special |
| **Out of Range Behavior** | Walk in range of the target location to cast

During *Death Defied*, target at maximum range (clamped) |

**ACTIVE:** **Karthus** conjures a blast at the target location that detonates after |See notes, granting sight of the area and dealing magic damage to all enemies within, doubled when only one target is struck.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 40 / 59 / 78 / 97 / 116 (+ 35% AP) |
| **Enhanced Damage** | 80 / 118 / 156 / 194 / 232 (+ 70% AP) |

**Notes:**

- The delay between the cast and the detonation is inconsistent, but always matches up with the VFX.
  - Defile’s ticks beyond the first has a similar issue.
- Applies spell damage if it hits a single target and area damage if it hits multiple targets.
  - *Lay Waste* also plays a different sound effect when it only hits one target.

---

### W: Wall of Pain

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 1000 units |
| **Cost** | 70 Mana |
| **Cooldown** | 15 seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Spell Shield** | False |
| **Out of Range Behavior** | walk to location |

**ACTIVE:** **Karthus** erects a wall of pain at the target location perpendicular to his facing that lasts 5 seconds, granting sight around its pillars and center.

| Attribute | Value |
|-----------|------:|
| **Wall Length** | 800 / 900 / 1000 / 1100 / 1200 |

Enemies that touch the wall are inflicted with (magic penetration) 25% magic resistance reduction and become slowed for 5 seconds, decaying over the duration. This can affect enemies only once per cast.

| Attribute | Value |
|-----------|------:|
| **Slow** | 40 / 50 / 60 / 70 / 80% |
| **Reduced Slow** | 20 / 25 / 30 / 35 / 40% |

**Notes:**

No additional notes.

---

### E: Defile

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 550 units |
| **Cost** | 30 / 42 / 54 / 66 / 78 Mana per second |
| **Static Cooldown** | $0.5$ |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | False |
| **Spell Effects** | aoedot |

**PASSIVE:** **Karthus** restores mana whenever he kills an enemy.

| Attribute | Value |
|-----------|------:|
| **Mana Restored** | 10 / 20 / 30 / 40 / 50 |

**TOGGLE:** **Karthus** surrounds himself in a necrotic aura that deals magic damage every $0.25$ seconds to all nearby enemies. Toggling *Defile* off triggers a final tick of damage.

| Attribute | Value |
|-----------|------:|
| **Magic Damage Per Tick** | 7.5 / 12.5 / 17.5 / 22.5 / 27.5 (+ 5% AP) |
| **Damage Per Second** | 30 / 50 / 70 / 90 / 110 (+ 20% AP) |

*Defile cannot be toggled off during Death Defied.*

**Notes:**

*Toggle abilities do not count as ability activations for the purposes of on-cast effects such as Spellblade and triggering Force Pulse’s passive.
- *Defile* will restore mana if **Karthus** destroys an enemy structure.
- *Defile* re-calculates its damage with each tick based on changes in **Karthus**’s ability power or an increase in the rank of *Defile* without the need to toggle it off and on again.
- *Defile* will toggle off automatically if **Karthus** enters resurrection.

---

### R: Requiem

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Effect Radius** | Global |
| **Cost** | 100 Mana |
| **Cooldown** | 200 / 190 / 180 / 170 / 160 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Aoe |
| **Out of Range Behavior** | False |
| **Target Warning** | Auto warning |
| **Silence** | True |

**ACTIVE:** **Karthus** channels for 3 seconds, then deals magic damage to all targetable enemy champions upon completion.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 200 / 275 / 350 / 425 / 500 (+ 70% AP) |

**Notes:**

- *Requiem*’s channel is telegraphed to all enemy champions by a beam of light that descends on all targets, including if they are untargetable or even dead.
- *Requiem* will affect clones.
- The damage will apply spell effects to targets in order of being spawned. Single target spell effects (such as Luden's Companion) are thus triggered on the leftmost champion on the loading screen.
- *Requiem* will not affect Neeko if she's disguised as a non-champion if **Karthus** is affected by Death Defied.
- The following table refers for interactions while **Karthus** is channeling:

#### Channel Behavior (channel)

| Aspect | State / Notes |
|--------|---------------|
| **Attacking** | Disabled |
| **Movement** | Disabled |
| **Abilities** | Disabled |
| **Items** | Disabled / Allowed |
| **Summoner Spells** | Allowed / Interrupts / Disabled |

---

## Patch History

### V25.18
- Wall of Pain
  - **Bug Fixes:** Wall VFX now properly layers over terrain that is not at default elevation. Previously, the VFX could be completely hidden if the VFX coincided entirely with those sections of the map.

### V14.24
- Lay Waste
  - **Bug Fixes:** No longer reveals the user if cast while the cursor is near an enemy champion.

### V14.21
- Requiem
  - AP ratio reduced to 70% AP from 75% AP.

### V14.15
- Death Defied
  - **Bug Fixes:** Restored death voice lines after *Death Defied* expires.

### V14.14
- Death Defied
  - **Bug Fixes:** Is no longer sometimes unable to issue movement commands after the zombie state has expired.

### V14.13
- Death Defied
  - **Bug Fixes:** Corpse is no longer sometimes a valid movement collision target.
- Lay Waste
  - Base damage reduced to 40 / 59 / 78 / 97 / 116 from 43 / 62 / 81 / 100 / 119.
    - Single-target damage reduced to 80 / 118 / 156 / 194 / 232 from 86 / 124 / 162 / 200 / 238.

### V14.11
- Lay Waste
  - Base damage reduced to 43 / 62 / 81 / 100 / 119 from 45 / 65 / 85 / 105 / 125.
    - Single-target damage reduced to 86 / 124 / 162 / 200 / 238 from 90 / 130 / 170 / 210 / 250.

### V14.2#January 24th Hotfix|V14.2
- Wall of Pain
  - **Bug Fixes:** Magic resistance reduction while in zombie form no longer multiplies the magic resistance of enemies hit by 25 instead of $0.75$.
- Requiem
  - **Bug Fixes:** Now properly benefits from ability haste as soon as he respawns.

### V14.2
- Lay Waste
  - Base damage increased to 45 / 65 / 85 / 105 / 125 from 43 / 61 / 79 / 97 / 115.
    - Single-target damage increased to 90 / 130 / 170 / 210 / 250 from 86 / 122 / 158 / 194 / 230.
- Wall of Pain
  - Magic resistance reduction increased to 25% from 15%.

### V13.22
- Stats
  - Basic attack missile speed increased to 1500 from 1200.

## Trivia

- Karthus is voiced.md) by Adam_Harrington_(voice_actor), who also voices Kassadin, Mordekaiser, Ryze, and Shaco.
- During development he was called *Lich*.
- Requiem old.png was likely named after Requiem Mass referencing Karthus' singing for/about the dead.
- His Karthus made a cameo in Fright Night (2011 film) along with Fiddlesticks.
- A glass case reading **PRESS R TO WIN*’* can be seen in the game's Mac Version launch trailer.
- Before 2016, Karthus had been disabled for every single Ultra Rapid Fire edition (most likely to prevent 'Spam R to Win' abuse cases).
- For a brief time in Season 2015, Karthus had a bug where he could remain in Death Defied indefinitely while alive and held an 87% win rate.

---
*This page was automatically generated from League of Legends Wiki data.*