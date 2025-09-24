# Rammus

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
| **Champion** | Rammus |
| **Title** | the Armordillo |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2009-07-10 |
| **Release Patch** | July 10, 2009 Patch |
| **Latest Changes** | V25.13 |
| **Roles** | Vanguard |
| **Riot Positions** | Jungle |
| **External Positions** | Jungle |
| **Blue Essence** | 675 |
| **Riot Points** | 585 |
| **Difficulty** | 1 |
| **Hero Type** | Tank |
| **Alt Type** | Fighter |
| **Adaptive Type** | Physical |
| **Damage** | 2 |
| **Toughness** | 3 |
| **Control** | 3 |
| **Mobility** | 2 |
| **Utility** | 1 |
| **Style** | 65 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $675.0$ | $+100.0$ |
| **Mana** | $310.0$ | $+33.0$ |
| **Health Regen** | $8.0$ | $+0.55$ |
| **Mana Regen** | $7.85$ | $+0.5$ |
| **Armor** | $35.0$ | $+4.5$ |
| **Magic Resist** | $32.0$ | $+2.05$ |
| **Attack Damage** | $65.0$ | $+2.75$ |
| **Attack Speed** | $0.700$ | |
| **Movement Speed** | $335.0$ | $+0.0$ |
| **Attack Range** | $125.0$ | $+0.0$ |
| **Base Attack Speed** | $0.7$ | |
| **Attack Speed Ratio** | $0.625$ | |
| **Bonus AS per Level** | $2.2\%$ | |
| **Acquisition Radius** | $600$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $120$ units | |
| **Selection Height** | $130$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $100.0\%$ |
| **Damage Taken** | $100.0\%$ |

#### URF

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $110.0\%$ |
| **Damage Taken** | $85.0\%$ |

#### Ultimate Spellbook

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $105.0\%$ |
| **Damage Taken** | $95.0\%$ |

## Abilities

### Passive: Spiked Shell

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Self |

**INNATE:** **Rammus** gains **bonus** attack damage equal to the sum of 15% **total** armor and 15% **total** magic resistance.

**Notes:**

- No additional details.

---

### Q: Powerball

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Collision Radius** | 75 (center-to-edge, against champions) units |
| **Effect Radius** | 250 (effect radius upon collision) units |
| **Cost** | 60 Mana |
| **Cooldown** | 12 / 10.5 / 9 / 7.5 / 6 seconds |
| **Cooldown Start** | post-effect |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |
| **Grounded** | False |
| **Silence** | True |

**ACTIVE:** **Rammus** channels for up to 6 seconds to roll into a ball, gaining (ms) 150/6 to 235/6 **bonus total** movement speed per second over the duration, up to a maximum of 150 to 235. 

*Powerball* can be recast after 1 second within the duration, and does so automatically afterwards or upon casting *Defensive Ball Curl*.

**Rammus** stops rolling upon colliding with an enemy, dealing magic damage to all nearby enemies and knocking them back 125 units, though not through terrain. Enemies hit are then stunned and revealed for $0.4$ (Estimated) seconds, as well as slowed for 1 second.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 80 / 120 / 160 / 200 / 240 (+ 100% AP) |

| Attribute | Value |
|-----------|------:|
| **Slow** | 40 / 50 / 60 / 70 / 80% |

**RECAST:** **Rammus** ends *Powerball*.

**Notes:**

- The initial cast and the manual recast count as ability activations for the purposes of on-cast effects such as Spellblade and triggering Force Pulse’s passive.
  - If the ability is not manually recasted, the secondary effect will trigger without being considered as an ability activation.
- The **bonus** movement speed stacks multiplicatively with other sources of movement speed boosts.
- *Powerball* will still collide with enemies if **Rammus** is ghosted.
- For the purpose of moving closer to an enemy when right clicking them, *Powerball* also reduces **Rammus**' attack range「 to 0 ⟷ by 125 」for the duration.
  - Regardless of this, during *Powerball* **Rammus** will also always attempt to path towards his target's center instead.
- **Rammus** will ignore unit collision for the purpose of pathfinding during *Powerball*.
  - Rammus acts as if he were ghosted for the duration of *Powerball*.
- *Powerball* is not a movement channel, and so will not be interrupted by root and ground.
- Displacement immunity will also resist the application of the stun.
- The following table refers for interactions while **Rammus** is channeling:

---

### W: Defensive Ball Curl

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Cost** | 40 Mana |
| **Cooldown** | 7 seconds |
| **Cooldown Start** | post-effect |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | False |
| **Spell Effects** | aoe |

**ACTIVE:** **Rammus** enters a defensive stance for 7 seconds, gaining (armor) **bonus** armor and (mr) **bonus** magic resistance. While active, enemies that use a basic attack on-hit against **Rammus** are dealt 15 (+ 10% **total** armor) (+ 10% **total** magic resistance) magic damage.

| Attribute | Value |
|-----------|------:|
| **Bonus Armor** | 27 / 32 / 37 / 42 / 47 (+ 30 / 37.5 / 45 / 52.5 / 60% **total** armor) |

| Attribute | Value |
|-----------|------:|
| **Bonus Magic Resistance** | 27 / 32 / 37 / 42 / 47 (+ 30 / 37.5 / 45 / 52.5 / 60% **total** magic resistance) |

*Defensive Ball Curl* can be recast after 1 second within the duration, and does so automatically upon casting *Powerball* or *Soaring Slam*.

**RECAST:** **Rammus** ends *Defensive Ball Curl*.

**Notes:**

- *Defensive Ball Curl* ends immediately if Powerball is cast.
- *Defensive Ball Curl* will return damage from attacks and/or abilities that apply on-hit effects (Mystic Shot) as well as from Runaan's Hurricane Wind's Fury (will not return damage from turrets, H-28G Evolution Turret, or Jack in the Box).
- The resistance scaling will factor from all sources, including *Defensive Ball Curl*’s flat bonus. This will recalculate over the duration.

---

### E: Frenzying Taunt

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 325 units |
| **Cost** | 50 Mana |
| **Cooldown** | 12 seconds |
| **Targeting** | Unit |
| **Affects** | Enemies, Self |
| **Spell Shield** | True |

**ACTIVE:** **Rammus** taunts the target enemy champion or monster for a duration. Monsters are additionally dealt magic damage upon being affected.

| Attribute | Value |
|-----------|------:|
| **Taunt Duration** | 1.2 / 1.4 / 1.6 / 1.8 / 2 seconds |

| Attribute | Value |
|-----------|------:|
| **Monster Magic Damage** | 80 / 100 / 120 / 140 / 160 (+ 70% AP) |

**Notes:**

- *Frenzying Taunt*, despite being usable on monsters, does not force them to attack **Rammus**. He does not draw aggro since the ability does not deal damage.

---

### R: Soaring Slam

| Attribute | Value |
|-----------|------:|
| **Cast Time** | None |
| **Target Range** | 800 (Minimum cast range) OR units |
| **Effect Radius** | 400 (Slam radius) / 200 (Inner radius) / 400 (Tremors radius) units |
| **Speed** | 900 (Minimum dash speed) / 2000 (Maximum dash speed) units/second |
| **Cost** | 100 Mana |
| **Cooldown** | 120 / 112.5 / 105 / 97.5 / 90 seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | Special |
| **Spell Effects** | AoE |
| **Out of Range Behavior** | cast at max |
| **Grounded** | True |
| **Knockdown** | False |

**ACTIVE:** **Rammus** leaps to the target location with immunity to crowd control. Upon arrival, he creates an impact that deals magic damage to nearby enemies and slows them for $1.5$ seconds.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 150 / 200 / 250 / 300 / 350 (+ 60% AP) |

| Attribute | Value |
|-----------|------:|
| **Slow** | 30 / 35 / 40 / 45 / 50% |

If *Soaring Slam* was cast during *Powerball*, enemies within the epicenter are also knocked up for $0.75$ seconds and are dealt *Powerball’s* damage. *Powerball* will not collide with enemies during the dash, and its channel will be maintained for the dash and end afterwards.

The impact causes 3 aftershocks to burst from the area over $3.5$ seconds, each one applying and refreshing the duration of the initial slow.

*Soaring Slam*’s damage affects structures and is doubled against turrets.

**Notes:**

- Spell shield will block the impact (including with Powerball) but not each aftershock.
- **Rammus** is required to have had at least $461.5$ movement speed *average* over the last second to be able to cast *Soaring Slam* further than its minimum cast range and thereby deal the increased damage at the epicenter.
  - The maximum increased center damage is reachable once **Rammus** has had at least $1307.69$ movement speed *average* for the past second.
- *Soaring Slam* will not impact nor generate aftershocks if the dash is interrupted.
- The following table refers for interactions while **Rammus** is dashing:

---

## Patch History

### V25.13
- Stats
  - Health growth increased to 100 from 94.
- Defensive Ball Curl
  - Base bonus armor increased to 27 / 32 / 37 / 42 / 47 from 27 at all ranks.
  - Base bonus magic resistance increased to 27 / 32 / 37 / 42 / 47 from 27 at all ranks.

### V25.12
- Stats
  - Base attack damage increased to 65 from 55.
  - Base armor reduced to 35 from 40.
  - Armor growth reduced to $4.5$ from $5.5$.
  - Base attack speed increased to $0.7$ from $0.656$.
- Spiked Shell
  - **New Effect:** Now grants **bonus** AD equal to 15% armor and 15% magic resistance.
    - **OLD EFFECT:** Basic attacks deal 10 (+ 10% armor) **bonus** magic damage on-hit, increased by 50% during *Defensive Ball Curl.
- Powerball
  - Base damage increased to 80 / 120 / 160 / 200 / 240 from 80 / 110 / 140 / 170 / 200.
  - **Removed:*** No longer prevents Frenzying Taunt from being cast shortly after it ends.
- Defensive Ball Curl
  - **New Effect:** While active, enemies that use a basic attack on-hit against **Rammus** are dealt 15 (+ 10% **total** armor) (+ 10% **total** magic resistance) magic damage.
    - **OLD EFFECT:** While active, enemies that use a basic attack on-hit against **Rammus** are dealt magic damage equal to *Spiked Shell’s* **bonus** damage.
  - Base bonus armor reduced to 27 from 40.
  - Armor ratio for bonus armor reduced to 30 / 37.5 / 45 / 52.5 / 60% **total** armor from 35 / 45 / 55 / 65 / 75%.
  - Base bonus magic resistance increased to 27 from 10.
  - Magic resistance ratio for bonus magic resistance increased to 30 / 37.5 / 45 / 52.5 / 60% **total** magic resistance from 30 / 35 / 40 / 45 / 50%.
  - Duration increased to 7 seconds from 6.
  - **Removed:*** Basic attacks no longer extend its duration by $0.4$ seconds, up to a maximum of 4 additional seconds.
  - **Bug Fixes:** Now properly reflects damage from Dragon basic attacks.
  - Tooltip now more clearly states how much armor and magic resistance will be gained from the ability.
  - Tooltip is no longer blank when the ability is active and will now show the amount of damage returned.
- Frenzying Taunt
  - **Removed:*** No longer grants 20 / 25 / 30 / 35 / 40% **bonus** attack speed for 1.2 / 1.4 / 1.6 / 1.8 / 2 seconds.
    - **Removed:*** The duration of the **bonus** attack speed no longer continuously refreshes during *Powerball*, *Defensive Ball Curl* and *Soaring Slam*.
  - **New Effect:** Now deals 80 / 100 / 120 / 140 / 160 (+ 70% AP) magic damage to monsters.
- Soaring Slam
  - **Removed:*** Aftershocks no longer deal 20 / 30 / 40 (+ 10% AP) magic damage.
  - **Removed:*** Aftershocks no longer stack the initial slow up to 4 times. *Now only applies and refreshes the duration of the initial slow.*
  - **Removed:*** Enemies in the epicenter no longer take 0 to 50 increased damage.
    - *[This modifier did not affect the damage applied by Powerball.]*
  - Base damage increased to 150 / 250 / 350 from 100 / 175 / 250.
  - Slow increased to 30 / 40 / 50% from 15 / 17.5 / 20%.
  - Cooldown increased to 120 / 105 / 90 seconds from 90 at all ranks.
  - **Bug Fixes:** Tooltip now correctly notes that Powerball’s damage is dealt to enemies in the epicenter.
  - Re-timed animation to play the full animation over the dash.
  - **Bug Fixes:** No longer breaks its animation and fails to deliver Powerball if they are cast too quickly together.
  - **Bug Fixes:** Fixed a bug where the animation bugged out at short ranges.

### V14.23
- Powerball
  - Cooldown reduced to 12 / 10.5 / 9 / 7.5 / 6 seconds from 16 / 13.5 / 11 / 8.5 / 6.

### V14.9
- General
  - Adjusted splash artwork for Rammus.

### V14.7
- General
  - Adjusted splash artwork for Rammus.

### V14.2
- Defensive Ball Curl
  - **Bug Fixes:** No longer interrupts attack windups when the effect ends.

### V13.22
- Defensive Ball Curl
  - Base bonus armor increased to 40 from 30.

### V13.21
- Powerball
  - Base damage reduced to 80 / 110 / 140 / 170 / 200 from 100 / 125 / 150 / 175 / 200.
- Defensive Ball Curl
  - Base bonus armor reduced to 30 from 35.
  - Armor ratio reduced to 35 / 45 / 55 / 65 / 75% **total** armor from 40 / 50 / 60 / 70 / 80%.

### V13.7
- Defensive Ball Curl
  - Increased armor reduced to 40 / 50 / 60 / 70 / 80% from 40 / 55 / 70 / 85 / 100%.

### V13.5
- Stats
  - Attack damage growth increased to $2.75$ from $3.5$.
- Powerball
  - Base damage reduced to 100 / 125 / 150 / 175 / 200 from 100 / 130 / 160 / 190 / 220.

## Trivia

- *Rammus* possibly plays on English verb ram & Latin masculine suffix *-us*.
- His dance references the back spin, a breakdance move.
  - A side-by-side comparison can be seen here.
- The ward skin Armordillo Ward.png references him.
- It is possible for Rammus to reach approximately 3104 armor in an "ideal" scenario. Details can be seen here.
- In the V1.0.0.115 April Fools' Day patch, he was jokingly listed to receive a skin called **"Rammurf"**, referencing Urf.
- Out of all the champions in the game, Rammus has gone the longest without an update to his base splash art up until 2016.

---
*This page was automatically generated from League of Legends Wiki data.*