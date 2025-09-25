# Lux

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
| **Champion** | Lux |
| **Title** | the Lady of Luminosity |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2010-10-19 |
| **Release Patch** | V1.0.0.103 |
| **Latest Changes** | V14.10 |
| **Roles** | Burst, Artillery |
| **Riot Positions** | Middle, Support |
| **External Positions** | Middle, Support |
| **Blue Essence** | 225 |
| **Riot Points** | 260 |
| **Difficulty** | 1 |
| **Hero Type** | Mage |
| **Alt Type** | Support |
| **Adaptive Type** | Magic |
| **Damage** | 3 |
| **Toughness** | 1 |
| **Control** | 2 |
| **Mobility** | 1 |
| **Utility** | 2 |
| **Style** | 90 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $580.0$ | $+99.0$ |
| **Mana** | $480.0$ | $+23.5$ |
| **Health Regen** | $5.5$ | $+0.55$ |
| **Mana Regen** | $7.0$ | $+0.8$ |
| **Armor** | $21.0$ | $+5.2$ |
| **Magic Resist** | $30.0$ | $+1.3$ |
| **Attack Damage** | $54.0$ | $+3.3$ |
| **Attack Speed** | $0.669$ | |
| **Movement Speed** | $330.0$ | $+0.0$ |
| **Attack Range** | $550.0$ | $+0.0$ |
| **Base Attack Speed** | $0.669$ | |
| **Attack Speed Ratio** | $0.625$ | |
| **Bonus AS per Level** | $3.0\%$ | |
| **Missile Speed** | $1600$ units/second | |
| **Acquisition Radius** | $625$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $100$ units | |
| **Selection Height** | $85$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $90.0\%$ |
| **Damage Taken** | $110.0\%$ |

#### URF

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $95.0\%$ |
| **Damage Taken** | $100.0\%$ |

## Abilities

### Passive: Illumination

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | Special |
| **Spell Effects** | Proc |
| **Parry** | unknown |

**INNATE:** **Lux**’s abilities apply a mark to enemies hit for 6 seconds, refreshing on subsequent hits. 

**Lux**’s basic attacks and *Final Spark* consume the mark to deal 30 to 200 (+ 30% AP) **bonus** magic damage.

**Notes:**

- Starting an attack windup against a target with a mark that is about to expire refreshes its duration to $0.25$ seconds. Casting *Final Spark* refreshes the marks of all marked enemies to $1.25$ seconds if they are within 5000 (Estimated) units of the ability's casting position and have a mark with a remaining duration of less than 1 second.
- Spell shield prevents Final Spark from consuming the mark.
  - The on-hit damage from *Illumination* via basic attack cannot be blocked by *spell shield*.
- *Illumination*’s trigger from a basic attack can be blocked (bonus damage is negated and the mark will still be consumed).
- : Parry interactions (dodge, blind).

---

### Q: Light Binding

| Attribute | Value |
|-----------|------:|
| **Range** | 1300 / er 1240 units |
| **Cast Time** | $0.25$ seconds |
| **Width** | 140 units |
| **Speed** | 1200 units/second |
| **Cost** | 50 Mana |
| **Cooldown** | 11 / 10.5 / 10 / 9.5 / 9 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |
| **Projectile** | True |

**ACTIVE:** **Lux** shoots a sphere of light in the target direction that deals magic damage to the first two enemies hit and roots them for 2 seconds.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 80 / 120 / 160 / 200 / 240 (+ 65% AP) |

**Notes:**

- Targets immune to the root still count towards *Light Binding*’s two-target limit.
- This ability will cast from wherever the caster is at the end of the cast time.
- *Light Binding* spawns an additional missile at the same location and vector as the main missile. This additional "Dummy" missile has the same visual effects as the main missile and gets destroyed when it hits a single enemy, thereby making the surviving VFX of the single missile weaker in intensity.
  - This hacky solution also causes Lux Q to count as two skillshots for things such as "skillshots dodged" Eternals.

---

### W: Prismatic Barrier

| Attribute | Value |
|-----------|------:|
| **Range** | 1175 units |
| **Cast Time** | $0.25$ (Effect starts at beginning of cast time) seconds |
| **Effect Radius** | 110 (Circle check upon commencing of direction reversal) units |
| **Width** | 220 (Both outgoing and returning, homing missile) units |
| **Speed** | 2400 (Original speed; decelerates/accelerates depending on travel time, see notes) |
| **Cost** | 60 / 65 / 70 / 75 / 80 Mana |
| **Cooldown** | 14 / 13 / 12 / 11 / 10 seconds |
| **Targeting** | Direction |
| **Affects** | Allies |
| **Projectile** | Special |

**ACTIVE:** **Lux** throws her wand in the target direction that homes back to her after reaching maximum range. Allied champions hit by the wand gain a shield for $2.5$ seconds, which can stack up to 2 times, stacking with the previous shield and refreshing its duration.

| Attribute | Value |
|-----------|------:|
| **Shield Strength** | 40 / 55 / 70 / 85 / 100 (+ 40% AP) |
| **Maximum Shield** | 80 / 110 / 140 / 170 / 200 (+ 80% AP) |

**Lux** gains the shield upon throwing and upon retrieving the wand.

**Notes:**

- Each pass of the wand grants a shield to each target once.
- *Prismatic Barrier* may only stack up to 2 times. Subsequent instances of passing through the wand have no effect on the shield's strength nor duration.
- The shield will also be granted to allies near the wand's return location.
- Both passes of *Prismatic Barrier* cannot hit units whose center is beyond the missile range or behind its origin, even if their hitbox radius overlaps.
  - At the return location, this behaviour is overridden by an additional center check with the same diameter as the missile width.
- If **Lux** dies before her wand returns it will fizzle upon reaching maximum range.
- There is an additional circle check at the end of the missile's length that allows it to hit the edge of an ally's radius. This is intended.
- **Lux**’s wand decelerates on the way out and accelerates on the way back.
- This ability will cast from wherever the caster is at the start of the cast time.
- *Prismatic Barrier* is destroyed by Wind Wall and Blade Whirl but not Unbreakable.

---

### E: Lucent Singularity

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 1100 units |
| **Effect Radius** | 310 (Area of effect radius) / 650 (Sight radius) units |
| **Speed** | 1200 units/second |
| **Cost** | 70 / 80 / 90 / 100 / 110 Mana |
| **Cooldown** | 10 / 9.5 / 9 / 8.5 / 8 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Location / Auto |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |
| **Projectile** | True |
| **Out of Range Behavior** | Walk in range of the target location to cast (first cast) |

**ACTIVE:** **Lux** sends a lucent singularity to the target location, remaining there for 5 seconds to grant sight of the area and slow nearby enemies.

| Attribute | Value |
|-----------|------:|
| **Slow** | 25 / 30 / 35 / 40 / 45% |

*Lucent Singularity* can be recast at any time while it is in flight or within the duration, and does so automatically after the duration.

**RECAST:** **Lux** detonates the singularity, dealing magic damage to enemies within. If *Lucent Singularity* was recast while in flight, it will detonate upon arrival.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 65 / 115 / 165 / 215 / 265 (+ 80% AP) |

The slow lingers for 1 second after leaving the area. Enemies hit by *Lucent Singularity*’s detonation are also slowed by the same amount for 1 second.

**Notes:**

- The initial cast counts as an ability activation for the purposes of on-cast effects such as Spellblade and triggering Force Pulse’s passive.
  - The manual recast does not.
- *Lucent Singularity* grants sight while also in flight.
- The ability will not preserve the caster's facing direction when using Flash and similar effects.

---

### R: Final Spark

| Attribute | Value |
|-----------|------:|
| **Range** | 3400 units |
| **Cast Time** | 1 seconds |
| **Width** | 200 units |
| **Cost** | 100 Mana |
| **Cooldown** | 60 / 55 / 50 / 45 / 40 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |
| **Projectile** | False |

**ACTIVE:** **Lux** fires a massive light beam in a line in the target direction that deals magic damage to enemies hit and reveals them for $1.5$ seconds, as well as grants sight of the surrounding area.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 300 / 350 / 400 / 450 / 500 (+ 120% AP) |

**Notes:**

- *Final Spark* grants sight of its surroundings during the cast time and for $0.5$ seconds afterwards.
- This ability will cast from wherever the caster is at the start of the cast time.
- The ability will not preserve the caster's facing direction when using Flash and similar effects.

#### Channel Behavior (cast)

| Aspect | State / Notes |
|--------|---------------|
| **Abilities** | Lucent Singularity can only be recast if it is present. Light Binding and Prismatic Barrier are disabled. |

---

## Patch History

### V14.10
- General
  - **Bug Fixes:** Restored Recall winddown animation for all skins.
  - **Bug Fixes:** Restored Respawn animation for all skins.

### V14.9
- Lucent Singularity
  - **Bug Fixes:** Vision bubble is no longer inconsistent inside terrain, causing some targets to remain undetected in its radius.

### V14.7
- Illumination
  - AP ratio increased to 30% AP from 25% AP.
- Light Binding
  - AP ratio increased to 65% AP from 60% AP.

### V14.2
- Light Binding
  - **Bug Fixes:** No longer has a very rare chance to break a spell shield that was added right after it applied its full effects already.
    - Width of the extra dummy VFX missile is now identical to the functionally relevant one.

### V14.1#January 12th Hotfix|V14.1
- Stats
  - Base health increased to 580 from 560.
  - Base armor increased to 21 from 19.
  - Attack speed growth increased to 3% from 2%.
- Prismatic Barrier
  - AP ratio per hit increased to 40% AP from 35% AP.

### V13.17
- Stats
  - Base mana regeneration reduced to 7 from 8.
- Illumination
  - Base damage increased to 30 to 200 from 20 to 190.
  - AP ratio increased to 25% AP from 20% AP.
- Light Binding
  - Cooldown reduced to 11 / 10.5 / 10 / 9.5 / 9 seconds from 11 at all ranks.
- Lucent Singularity
  - Base damage reduced to 65 / 115 / 165 / 215 / 265 from 70 / 120 / 170 / 220 / 270.

### V13.14
- Lux and Lux
  - **Bug Fixes:** Staff no longer resets position while using the *Taunt* emote in a loop transition.
- Lux
  - Light Binding
    - **Bug Fixes:** Orb no longer renders behind a turret.
  - Prismatic Barrier
    - **Bug Fixes:** VFX trail no longer renders behind environment objects.

### V12.23#December 14th Hotfix|V12.23
- Lucent Singularity
  - AP ratio increased to 80% AP from 70% AP.
- Final Spark
  - AP ratio increased to 120% AP from 100% AP.

### V12.14
- Light Binding
  - Root buff renamed to *Light Binding* from *Prismatic Seal*.
- Prismatic Barrier
  - Shield buff renamed to *Prismatic Barrier* from *Stardust Barrier*.

### V12.10
- Stats
  - Base health increased to 560 from 490.
  - Health growth increased to 99 from 85.
  - Armor growth increased to $5.2$ from 4.
  - Magic resistance growth increased to $1.3$ from $0.5$.
- Prismatic Barrier
  - Base shield per hit reduced to 40 / 55 / 70 / 85 / 100 from 45 / 65 / 85 / 105 / 125.

## Trivia

- *Luxanna* comes from Latin word for light, lux.
- Lux's ultimate has had three names, all referencing from Touhou Project.
  1. Infinite Light (V1.0.0.144 - V1.0.0.145)
  1. Final Spark (V1.0.0.145 - present, references Marisa Kirisame's Last Spell in Imperishable Night)
    - Final Spark also references by from Dragon Ball Z.
- Lux's Series 2 Eternals make the following references:
  - *Double Rainbow* is a reference to Lux's joke interaction based on Yosemitebear62's viral video.
  - *Ultraviolent* is a name puns on "Ultra Violet" and "Violence".

---
*This page was automatically generated from League of Legends Wiki data.*