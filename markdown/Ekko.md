# Ekko

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
| **Champion** | Ekko |
| **Title** | the Boy Who Shattered Time |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2015-05-29 |
| **Release Patch** | V5.10 |
| **Latest Changes** | V25.04 |
| **Roles** | Assassin |
| **Riot Positions** | Jungle, Middle |
| **External Positions** | Jungle, Middle |
| **Blue Essence** | 1575 |
| **Riot Points** | 790 |
| **Difficulty** | 2 |
| **Hero Type** | Assassin |
| **Alt Type** | Mage |
| **Adaptive Type** | Magic |
| **Damage** | 3 |
| **Toughness** | 2 |
| **Control** | 2 |
| **Mobility** | 3 |
| **Utility** | 1 |
| **Style** | 75 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $655.0$ | $+99.0$ |
| **Mana** | $280.0$ | $+70.0$ |
| **Health Regen** | $9.0$ | $+0.9$ |
| **Mana Regen** | $7.0$ | $+0.8$ |
| **Armor** | $32.0$ | $+4.2$ |
| **Magic Resist** | $32.0$ | $+2.05$ |
| **Attack Damage** | $58.0$ | $+3.0$ |
| **Attack Speed** | $0.688$ | |
| **Movement Speed** | $340.0$ | $+0.0$ |
| **Attack Range** | $125.0$ | $+0.0$ |
| **Base Attack Speed** | $0.688$ | |
| **Attack Speed Ratio** | $0.625$ | |
| **Bonus AS per Level** | $3.3\%$ | |
| **Attack Windup** | $16.2\%$ | |
| **Acquisition Radius** | $600$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $120$ units | |
| **Selection Height** | $155$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $110.0\%$ |
| **Damage Taken** | $100.0\%$ |

## Abilities

### Passive: Z-Drive Resonance

| Attribute | Value |
|-----------|------:|
| **On-target CD Static** | 5 (Begins after the stacks are consumed) |
| **Targeting** | Passive |
| **Affects** | Enemies / Self |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Single target |
| **Parry** | unknown |

**INNATE - RESONANCE:** **Ekko**’s basic attacks on-hit and damaging abilities apply a stack of *Resonance* to enemies hit for 4 seconds, refreshing on subsequent hits and stacking up to 3 times. The third stack consumes them all to deal 30 to 80 by 10 / 80 to 140 (+ 90% AP) **bonus** magic damage. *Z-Drive Resonance* deals 300% damage against monsters.

*Resonance* cannot affect the same target more than once every few seconds.

**INNATE - STOLEN TIME:** Triggering *Resonance* against a champion grants **Ekko** 50%–80%@1–16 (ms) **bonus** movement speed for 2–3@1–11 seconds.

**Notes:**

- **Ekko**’s basic attacks have different animations based on how many stacks his target has. His attack pattern is as follows:
  - Downwards (first stack) → Upwards (second stack) → Sideways (third stack to consume all stacks)
    - If the target has already been affected by *Resonance*, **Ekko**’s basic attack animations will kick or attack the target sideways.
- *Resonance* stacks will not be applied nor consumed if the basic attack is dodged or blocked or if the attack misses.

---

### Q: Timewinder

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 1100 units |
| **Effect Radius** | 160 – 210 (Temporal Sickness expansion) units |
| **Width** | 120 (Outward) / 200 (Returning) units |
| **Speed** | 1650 (Outward) / 200 (Temporal Sickness (Estimated)) / 2300 (Returning) units/second |
| **Cost** | 50 / 60 / 70 / 80 / 90 Mana |
| **Cooldown** | 9 / 8.5 / 8 / 7.5 / 7 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | Special |
| **Spell Effects** | spellaoe |
| **Projectile** | True |

**ACTIVE:** **Ekko** throws a temporal grenade in the target direction that deals magic damage to enemies hit.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 80 / 95 / 110 / 125 / 140 (+ 30% AP) |

At 700 units or upon hitting an enemy champion, the grenade slows down for $1.75$ seconds to gradually expand into a *Temporal Sickness* field that slows nearby enemies, travelling for another 190 units.

| Attribute | Value |
|-----------|------:|
| **Slow** | 40 / 45 / 50 / 55 / 60% |

Afterwards, the grenade contracts and homes back to **Ekko** at an increased speed, dealing magic damage to enemies hit.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 40 / 65 / 90 / 115 / 140 (+ 60% AP) |
| **Total Magic Damage** | 120 / 160 / 200 / 240 / 280 (+ 90% AP) |

*Enemies can be hit only once per pass.*

**Notes:**

- Spell shield will block only a single instance of damage.
- This ability will cast from wherever the caster is at the end of the cast time.

---

### W: Parallel Convergence

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 1600 units |
| **Effect Radius** | 375 units |
| **Cost** | 30 / 35 / 40 / 45 / 50 Mana |
| **Cooldown** | 22 / 20 / 18 / 16 / 14 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Location |
| **Spell Shield** | True |
| **Projectile** | False |
| **Out of Range Behavior** | walk to location |

**PASSIVE:** **Ekko**’s basic attacks deal **bonus** magic damage equal to 3% (+ 3% per 100 AP) of the target's **missing** health against enemies below (health) 30% of their **maximum** health. The damage has a minimum threshold of 15 and is capped at 150 against minions and monsters.

**ACTIVE:** **Ekko** creates an afterimage of himself that, after 2 seconds, bats a device to the target location and grants sight of the area for $2.5$ seconds. After travelling over $1.25$ seconds, the device expands into a chronosphere that is visible for $1.5$ seconds and which slows enemies within by 40%.

If **Ekko** enters the sphere within 2 seconds of its creation, it detonates to grant him a shield for 2 seconds and stun enemies within for $2.25$ seconds.

| Attribute | Value |
|-----------|------:|
| **Shield Strength** | 100 / 120 / 140 / 160 / 180 (+ 150% AP) |

*Enemies can see the indicator for Parallel Convergence 2 seconds after casting.*

**Notes:**

- Applies proc damage for the passive.
- The passive will proc with the attack that triggers the final stack of Resonance if it reduces a target's health below the 30% threshold.
- **Ekko** can detonate the expansion even while untargetable (i.e. Chronobreak’s dash), but not if he is resurrecting.
- *Parallel Convergence* will continue to slow enemies even if its expansion is detonated.

---

### E: Phase Dive

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 325 (Dash distance) / 550 (Maximum increased dash distance across terrain) units |
| **Cost** | 40 / 45 / 50 / 55 / 60 Mana |
| **Cooldown** | 9 / 8.5 / 8 / 7.5 / 7 seconds |
| **Cooldown Start** | post-effect |
| **Targeting** | Direction |
| **Affects** | Self |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Spell |
| **Terrain Grace** | True |
| **Parry** | True |
| **Grounded** | True |
| **Knockdown** | True |

**ACTIVE:** **Ekko** dashes in the target direction, then empowers his next basic attack within 3 seconds to have a $0.25$-second cast time, gain (range) 300 **bonus** range, cause him to blink within 125 (Estimated) range of the target, and deal **bonus** magic damage.

| Attribute | Value |
|-----------|------:|
| **Bonus Magic Damage** | 50 / 75 / 100 / 125 / 150 (+ 40% AP) |

*Phase Dive resets **Ekko**’s basic attack timer. **Ekko** can cast any of his abilities during the dash.*

**Notes:**

- The dash distance can be extended to up-to 550 units when targeting across terrain. It will only extend for the distance needed to cross the wall.
- While grounded or rooted, **Ekko** loses the **bonus** attack range from *Phase Dive*.
- *Phase Drive*’s attack can miss and be blocked and dodged, consuming the basic attack without dealing any damage.
- If the target becomes untargetable, dies, or is too far away during the empowered attack's cast time, it is cancelled but not consumed.

---

### R: Chronobreak

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.5$ seconds |
| **Effect Radius** | 375 (Damage radius) / Global (Activation radius) |
| **Cost** | 100 Mana |
| **Cooldown** | 110 / 95 / 80 / 65 / 50 seconds |
| **Targeting** | Auto |
| **Affects** | Enemies / Self |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Grounded** | True |
| **Knockdown** | False |

**PASSIVE:** Upon learning *Chronobreak* or if its **current** cooldown is lower than 4 seconds, **Ekko** reveals a time-delayed *afterimage* of himself that constantly tracks where he was 4 seconds ago.

**ACTIVE:** **Ekko** enters stasis at the start of the cast time, and afterwards heals himself and dashes to his *afterimage*’s location at the time of cast over $0.5$ seconds. Upon arrival, the stasis ends and he creates an explosion that deals magic damage to nearby enemies.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 200 / 275 / 350 / 425 / 500 (+ 175% AP) || Attribute | Value |
|-----------|------:|
| **Heal** | 100 / 125 / 150 / 175 / 200 (+ 60% AP) (+ 3% *per 1% of health lost in the past 4 seconds*) |

***Ekko** is immune to all displacements during Chronobreak*.

**Notes:**

- **Ekko** will attempt to basic attack the closest target after appearing at the cast location, but can do so quicker if he manually attack commands after the dash, except if Phase Dive is primed.
- The *afterimage*’s location will explode even if **Ekko**’s dash is interrupted.
- A *link* can also be seen between the *afterimage* and **Ekko** that traces along his path.
  - The *link* follows the same visibility rules as the *afterimage*.
  - Upon activation, **Ekko** travels full distance through the *link*, and thus will detonate Parallel Convergence that the *link* passes through.
- Quicksilver Sash Quicksilver incurs a 1-second cooldown upon casting *Chronobreak*.

---

## Patch History

### V25.04
- Timewinder
  - Base outgoing damage increased to 80 / 95 / 110 / 125 / 140 from 70 / 85 / 100 / 115 / 130.

### V25.S1.1
- Ekko
  - **Bug Fixes:** Restored VFX trail and SFX during the Homeguard animation.
  - Chronobreak
    - **Bug Fixes:** The Paragon Chroma no longer causes the afterimage to duplicate at very regular intervals while moving, instead of only one afterimage existing.

### V14.24
- Ekko
  - Renamed to *Arcane Firelight* from *Firelight*.

### V14.22
- Timewinder
  - **Bug Fixes:** No longer sometimes fails to apply the slow.
- Phase Dive
  - **Bug Fixes:** Attack now properly consumes Hail of Blades.
- Chronobreak
  - **New Effect:** Entering stasis now destroys homing projectiles.

### V13.16
- Timewinder
  - Initial base damage increased to 70 / 85 / 100 / 115 / 130 from 60 / 75 / 90 / 105 / 120.
  - Slow increased to 40 / 45 / 50 / 55 / 60% from 32 / 39 / 46 / 53 / 60%.
- Parallel Convergence
  - Base shield increased to 100 / 120 / 140 / 160 / 180 from 70 / 90 / 110 / 130 / 150.

### V13.1
- Parallel Convergence
  - **Bug Fixes:** No longer detonates in some instances when casting Chronobreak without actually travelling through the area with it.

### V13.3
- General
  - Adjusted splash artwork for Ekko.

### V12.19
- Z-Drive Resonance
  - AP ratio increased to 90% AP from 80% AP.
- Chronobreak
  - Base damage increased to 200 / 350 / 500 from 150 / 300 / 450.
  - Damage AP ratio increased to 175% AP from 150% AP.

### V12.10
- Stats
  - Base health increased to 655 from 585.
  - Health growth increased to 99 from 85.
  - Armor growth increased to $4.2$ from 3.
  - Magic resistance growth increased to $2.05$ from $1.25$.
- Parallel Convergence
  - Base shield reduced to 70 / 90 / 110 / 130 / 150 from 80 / 100 / 120 / 140 / 160.

### V12.9
- Timewinder
  - **Bug Fixes:** Tooltip now shows and calculates with the proper AP ratio for the outgoing damage.

## Trivia

- The name *Ekko* is a play on the English word Echo.
  - Besides the proverbial-and-literal 'Ekkos of the past' his name, having two Ks (KK) references the Rewind symbol ⏪.
  - His name could also be a reference to Mikky Ekko and his first album 'Time' although this has never been confirmed.
- *Chronobreak* gives its namesake to the proprietary time manipulation tool developed by Riot Games for use in professional matches.

---
*This page was automatically generated from League of Legends Wiki data.*