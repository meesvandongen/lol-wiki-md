# Ekko

## Table of Contents
- [Basic Information](#basic-information)
- [Statistics](#statistics)
  - [Advanced Stats](#advanced-stats)
- [Abilities](#abilities)
- [Patch History](#patch-history)
- [Trivia](#trivia)

## Basic Information

| Attribute | Value |
|-----------|-------|
| **Champion** | Ekko |
| **Title** | the Boy Who Shattered Time |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2015-05-29 |
| **Release Patch** | V5.10 |
| **Roles** | Assassin |
| **Riot Positions** | Jungle, Middle |
| **External Positions** | Jungle, Middle |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $655.0$ | $+99.0$ | $2338.0$ |
| **Mana** | $280.0$ | $+70.0$ | $1470.0$ |
| **Health Regen** | $9.0$ | $+0.9$ | $24.3$ |
| **Mana Regen** | $7.0$ | $+0.8$ | $20.6$ |
| **Armor** | $32.0$ | $+4.2$ | $103.4$ |
| **Magic Resist** | $32.0$ | $+2.05$ | $66.8$ |
| **Attack Damage** | $58.0$ | $+3.0$ | $109.0$ |
| **Attack Speed** | $0.688$ | $+3.3\%$ | $1.074$ |
| **Movement Speed** | $340.0$ | $+0.0$ | $340.0$ |
| **Attack Range** | $125.0$ | $+0.0$ | $125.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.688$ |
| **Attack Speed Ratio** | $0.625$ |
| **Bonus AS per Level** | $3.3\%$ |
| **Attack Windup** | $16.2\%$ |
| **Acquisition Radius** | $600 units$ |
| **Pathing Radius** | $35 units$ |
| **Selection Radius** | $120 units$ |
| **Selection Height** | $155 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Z-Drive Resonance

**Innate:** **Ekko** basic attacks on-hit and damaging abilities apply stacks, the third stack deals **bonus** magic damage. If the target was a champion, this grants *ms **bonus** movement speed* to **Ekko**.

**Innate - Resonance:** ''Ekko's* basic attacks on-hit and damaging abilities apply a stack of *Resonance* to enemies hit for 4 seconds, refreshing on subsequent hits and stacking up to 3 times. The third stack consumes them all to deal 30 to 80 by 10–80 to 140 (+ 90% AP) **bonus** magic damage. *Z-Drive Resonance' deals 300% damage against monsters. *Resonance* cannot affect the same target more than once every few seconds. **Innate - Stolen Time:** Triggering *Resonance* against a champion grants **Ekko** key=% *ms **bonus** movement speed* for 2–3@1–11 seconds.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Enemies / Self |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Single target |

**Notes:**

- ''Ekko's' basic attacks have different animations based on how many stacks his target has. His attack pattern is as follows:
  - Downwards → Upwards → Sideways *** If the target has already been affected by *Resonance*, ''Ekko's' basic attack animations will kick or attack the target sideways.
- *Resonance* stacks will not be applied nor consumed if the basic attack is dodged or blocked or if the attack blind.

---

### Q: Timewinder

**Active:** **Ekko** throws a temporal grenade that deals magic damage to enemies hit. It expands at max range or upon hitting an enemy champion, slow nearby enemies.

*After a delay, the grenade homes back to **Ekko**, dealing magic damage to enemies hit.*

**Active:** **Ekko** throws a temporal grenade in the target direction that deals magic damage to enemies hit. At 700 units or upon hitting an enemy champion, the grenade slows down for $1.75$ seconds to gradually expand into a *Temporal Sickness* field that slows nearby enemies, travelling for another 190 units. Afterwards, the grenade contracts and homes back to **Ekko** at an increased speed, dealing magic damage to enemies hit. *Enemies can be hit only once per pass.*

| Attribute | Value |
|-----------|-------|
| **Range** | 1100 units |
| **Cooldown** | $9-7$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $50-90$ Mana |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Speed** | 1650 / 200 / 2300 units/second |
| **Effect Radius** | 160 – 210 units |
| **Spell Shield** | Special |
| **Spell Effects** | spellaoe |
| **Projectile** | True |

**Scaling:**
- **Magic Damage:** $80-140$ (+ 30% AP)
- **Slow:** $40-60$%
- **Magic Damage:** $40-140$ (+ 60% AP)

**Notes:**

- Spell shield will block only a single instance of damage. Effect at cast time end

---

### W: Parallel Convergence

**Passive:** **Ekko**’s basic attacks deal **bonus** magic damage against a low-health target, increased based on their **missing** health.

**Active:** **Ekko** indicates the target location where a sphere will be created, which slow enemies within.

**Passive:** ''Ekko's** basic attacks deal **bonus'' magic damage equal to 3% as of the target's **missing* healthhealth 30% of their **maximum** health*. The damage has a minimum threshold of 15 and is capped at 150 against minions and monsters. **Active:** **Ekko** creates an afterimage of himself that, after 2 seconds, bats a device to the target location and grants sight of the area for $2.5$ seconds. After travelling over $1.25$ seconds, the device expands into a chronosphere that is visible for $1.5$ seconds and which slows enemies within by 40%. If **Ekko** enters the sphere within 2 seconds of its creation, it detonates to grant him a shield for 2 seconds and stun enemies within for $2.25$ seconds. *Enemies can see the indicator for Parallel Convergence 2 seconds after casting.*

| Attribute | Value |
|-----------|-------|
| **Range** | 1600 units |
| **Cooldown** | $22-14$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $30-50$ Mana |
| **Targeting** | Location |
| **Effect Radius** | 375 units |
| **Spell Shield** | True |
| **Projectile** | False |

**Scaling:**
- **Shield Strength:** $100-180$ (+ 150% AP)

**Notes:**

- Applies proc damage for the passive.
- The passive will proc with the attack that triggers the final stack of *Resonance* if it reduces a target's health below the 30% threshold.
- **Ekko** can detonate the expansion even while untargetable (i.e. *Chronobreak*’s dash), but not if he is resurrection.
- *Parallel Convergence* will continue to slow enemies even if its expansion is detonated.

---

### E: Phase Dive

**Active:** **Ekko** dash in the target direction, gaining *range **bonus** attack range*.

*His next basic attack blink him to the target and deals **bonus** magic damage.*

**Active:** **Ekko** dashes in the target direction, then empowers his next basic attack within 3 seconds to have a $0.25$-second cast time, gain range*bonus** range*, cause him to blink within 125 range of the target, and deal **bonus** magic damage. *Phase Dive basic attack reset *'Ekko's** basic attack timer. **Ekko'* can cast any of his abilities during the dash.*

| Attribute | Value |
|-----------|-------|
| **Range** | 325 / 550 units |
| **Cooldown** | $9-7$ seconds |
| **Cast Time** | none |
| **Cost** | $40-60$ Mana |
| **Targeting** | Direction |
| **Affects** | Self |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Spell |

**Scaling:**
- **Bonus Magic Damage:** $50-150$ (+ 40% AP)

**Notes:**

- The dash distance can be extended to up-to 550 units when targeting across terrain. It will only extend for the distance needed to cross the wall.
- While ground or root, **Ekko** loses the **bonus attack range** from *Phase Dive*.
- 'Phase Drive's attack can blind and be blocked and dodged, consuming the basic attack without dealing any damage.
- If the target becomes untargetable, death, or is too far away during the empowered attack's cast time, it is cancelled but not consumed.

---

### R: Chronobreak

**Passive:** **Ekko** reveals a time-delayed *afterimage* of himself that tracks where he was 4 seconds ago.

**Active:** **Ekko** enters stasis and dash to the *afterimage*, based on the damage received in the last 4 seconds. He then explodes, dealing magic damage to nearby enemies.

**Passive:** Upon learning *Chronobreak* or if its **current** cooldown is lower than 4 seconds, **Ekko** reveals a time-delayed *afterimage* of himself that constantly tracks where he was 4 seconds ago. **Active:** **Ekko** enters stasis (buff) at the start of the cast time, and afterwards heals himself and dash to his 'afterimage's location at the time of cast over $0.5$ seconds. Upon arrival, the stasis ends and he creates an explosion that deals magic damage to nearby enemies. **Ekko is immune to all airborne during Chronobreak**.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $110-50$ seconds |
| **Cast Time** | $0.5$ seconds |
| **Cost** | 100 Mana |
| **Targeting** | Auto |
| **Affects** | Enemies / Self |
| **Damage Type** | Magic |
| **Effect Radius** | 375 / Global |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |

**Scaling:**
- **Magic Damage:** $200-500$ (+ 175% AP)
- **Heal:** $100-200$ (+ 60% AP) (+ 3% *per 1% of health lost in the past 4 seconds*)

**Notes:**

- **Ekko** will attempt to basic attack the closest target after appearing at the cast location, but can do so quicker if he manually attack commands after the dash, except if *Phase Dive* is primed.
- The 'afterimage's* location will explode even if *'Ekko's' dash is interrupted.
- A *link* can also be seen between the *afterimage* and **Ekko** that traces along his path.
  - The *link* follows the same visibility rules as the *afterimage*.
  - Upon activation, **Ekko** travels full distance through the *link*, and thus will detonate *Parallel Convergence* that the *link* passes through.
- *Quicksilver Sash* Quicksilver incurs a 1-second *cooldown* upon casting *Chronobreak*.

---

## Patch History

### V25.04
- *Timewinder*
  - Base outgoing damage increased to $80-140$ from $70-130$.
- Ekko
  - **Bug Fixes:** Restored VFX trail and SFX during the Homeguard animation.
  - *Chronobreak*
    - **Bug Fixes:** The Paragon Chroma no longer causes the afterimage to duplicate at very regular intervals while moving, instead of only one afterimage existing.

### V14.24
- Ekko
  - Renamed to *Arcane Firelight* from *Firelight*.

### V14.22
- *Timewinder*
  - **Bug Fixes:** No longer sometimes fails to apply the slow.
- *Phase Dive*
  - **Bug Fixes:** Attack now properly consumes *Hail of Blades*.
- *Chronobreak*
  - **New Effect:** Entering stasis now destroys homing projectiles.

### V13.16
- *Timewinder*
  - Initial base damage increased to $70-130$ from $60-120$.
  - Slow increased to $40-60$% from $32-60$%.
- *Parallel Convergence*
  - Base shield increased to $100-180$ from $70-150$.

### V13.1
- *Parallel Convergence*
  - **Bug Fixes:** No longer detonates in some instances when casting *Chronobreak* without actually travelling through the area with it.

### V13.3
- General
  - Adjusted splash artwork for Ekko.

### V12.19
- *Z-Drive Resonance*
  - AP ratio increased to 90% AP from 80% AP.
- *Chronobreak*
  - Base damage increased to $200-500 3$ from $150-450 3$.
  - Damage AP ratio increased to 175% AP from 150% AP.

### V12.10
- Stats
  - Base health increased to 655 from 585.
  - Health growth increased to 99 from 85.
  - Armor growth increased to $4.2$ from 3.
  - Magic resistance growth increased to $2.05$ from $1.25$.
- *Parallel Convergence*
  - Base shield reduced to $70-150$ from $80-160$.

### V12.9
- *Timewinder*
  - **Bug Fixes:** Tooltip now shows and calculates with the proper AP ratio for the outgoing damage.

### V11.18
- *Z-Drive Resonance*
  - **Bug Fixes:** Updated the tooltip to include the additional damage dealt against jungle monsters.

## Trivia

- The name *Ekko* is a play on the English word Echo.
  - Besides the proverbial-and-literal 'Ekkos of the past' his name, having two Ks (KK) references the Rewind symbol ⏪.
  - His name could also be a reference to Mikky Ekko and his first album 'Time' although this has never been confirmed.
- *Chronobreak* gives its namesake to the proprietary time manipulation tool developed by Riot Games for use in professional matches.

---
*This page was automatically generated from League of Legends Wiki data.*