# Lissandra

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
| **Champion** | Lissandra |
| **Title** | the Ice Witch |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2013-04-30 |
| **Release Patch** | V3.6 |
| **Latest Changes** | V25.06 |
| **Roles** | Burst |
| **Riot Positions** | Middle |
| **External Positions** | Middle |
| **Blue Essence** | 2400 |
| **Riot Points** | 880 |
| **Difficulty** | 2 |
| **Hero Type** | Mage |
| **Adaptive Type** | Magic |
| **Damage** | 3 |
| **Toughness** | 1 |
| **Control** | 3 |
| **Mobility** | 2 |
| **Utility** | 1 |
| **Style** | 100 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $620.0$ | $+110.0$ |
| **Mana** | $475.0$ | $+30.0$ |
| **Health Regen** | $7.0$ | $+0.55$ |
| **Mana Regen** | $8.0$ | $+0.4$ |
| **Armor** | $22.0$ | $+4.9$ |
| **Magic Resist** | $30.0$ | $+1.3$ |
| **Attack Damage** | $55.0$ | $+2.7$ |
| **Attack Speed** | $0.656$ | |
| **Movement Speed** | $325.0$ | $+0.0$ |
| **Attack Range** | $550.0$ | $+0.0$ |
| **Base Attack Speed** | $0.656$ | |
| **Attack Speed Ratio** | $0.625$ | |
| **Bonus AS per Level** | $1.5\%$ | |
| **Missile Speed** | $2200$ units/second | |
| **Acquisition Radius** | $600$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $120$ units | |
| **Selection Height** | $250$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $95.0\%$ |
| **Damage Taken** | $105.0\%$ |

## Abilities

### Passive: Iceborn Subjugation

| Attribute | Value |
|-----------|------:|
| **Effect Radius** | 1350 (Aura radius around Lissandra, center-to-edge) / cr 450 (Thrall slow field and explosion radius) units |
| **Targeting** | Passive |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |

**INNATE:** Whenever a nearby enemy champion dies, **Lissandra** spawns a *Frozen Thrall* from their corpse.

*Frozen Thralls* are ice spirits that have 325*.103–325@0–4 (@=seconds alive) movement speed and slow nearby enemies by 25%. They will chase nearby visible enemies for 4 seconds, prioritizing champions, after which they shatter to deal 120 to 340 / 370 / 400 / 430 / 460 / 490 / 520 (+ 50% AP) magic damage to nearby enemies.

**Notes:**

- *Thralls* are units that are considered champions (they are clones) and they take the form of the champion they arise from.
  - Additionally, near the end of their life span, they will cue that champion's basic attack animation.
  - They cannot be teleported by an allied Realm Warp.
- *Thralls* are untargetable and invulnerable.
- *Thralls* are revealed to enemies through the fog of war.
- *Thralls* will continue to chase their target even if they enter a brush.
- *Iceborn Subjugation* will not summon a *Thrall* against enemy champions that enter a zombie state, and will instead summon one after they've left the state.

---

### Q: Ice Shard

| Attribute | Value |
|-----------|------:|
| **Range** | 825 (Standard range including area check after missile end) / 950 (Enhanced range from cast origin) units |
| **Cast Time** | $0.25$ seconds |
| **Width** | 150 (standard width) / 180 (Width after shattering) units |
| **Speed** | 2200 (Both before and after shattering) units/second |
| **Cost** | 55 / 60 / 65 / 70 / 75 Mana |
| **Cooldown** | 8 / 7 / 6 / 5 / 4 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | magic |
| **Spell Shield** | Special |
| **Spell Effects** | aoe |
| **Projectile** | True |

**ACTIVE:** **Lissandra** launches a shard of ice in the target direction that deals magic damage and slows enemies hit for $1.5$ seconds.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 80 / 115 / 150 / 185 / 220 (+ 75% AP) |

| Attribute | Value |
|-----------|------:|
| **Slow** | 20 / 24 / 28 / 32 / 36% |

If *Ice Shard* hits an enemy, it will shatter, increasing its width and maximum range.

**Notes:**

- *Ice Shard* picks a location 950 units away in the direction of the cast for the shattered missile to end up at.
- The initial *Ice Shard* missile has a range of 700 units which it arrives at after . If it hasn't collided with an enemy in this path when it does, it checks for enemies in a cr 100 radius around the point 25 units in front of it.
  - Colliding or hitting an enemy in either fashion creates a new "shattered" missile with the same speed but greater width that continues to travel along the same line to the designated point 950 units from the cast's original position, originating at the location at which the initial missile collided at.
- Spell shield will block the damage and the slow but will not stop the projectile from shattering.

---

### W: Ring of Frost

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 275 units |
| **Cost** | 40 Mana |
| **Cooldown** | 10 / 9.5 / 9 / 8.5 / 8 seconds |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |

**ACTIVE:** **Lissandra** freezes nearby enemies, dealing magic damage and rooting them for a duration.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 70 / 105 / 140 / 175 / 210 (+ 70% AP) |

| Attribute | Value |
|-----------|------:|
| **Root Duration** | 1.25 / 1.35 / 1.45 / 1.55 / 1.65 seconds |

**Notes:**

- No additional information.

---

### E: Glacial Path

| Attribute | Value |
|-----------|------:|
| **Range** | 1025 units |
| **Cast Time** | $0.25$ seconds |
| **Width** | 250 units |
| **Speed** | 1200 (starting speed) / 640 (minimum speed) units/second |
| **Cost** | 80 / 85 / 90 / 95 / 100 Mana |
| **Cooldown** | 24 / 21 / 18 / 15 / 12 (Starts on cast) seconds |
| **Targeting** | Direction / Auto |
| **Affects** | Enemies, Self |
| **Damage Type** | magic |
| **Spell Shield** | True |
| **Spell Effects** | aoe |
| **Projectile** | True |
| **Grounded** | Special |

**ACTIVE:** **Lissandra** sends a claw of ice in the target direction that deals magic damage to enemies it passes through, decelerating over $1.25$ seconds. *Glacial Path* can be recast after $0.5$ seconds while the claw is active.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 70 / 105 / 140 / 175 / 210 (+ 60% AP) |

**RECAST:** **Lissandra** consumes the claw and blinks to its current location.

**Notes:**

- **Lissandra** can reactivate *Glacial Path* for the duration (plus another $0.2$-$0.3$ seconds after the claw 'sinks' into the ground).
  - The Claw can also hit enemies near the end point at this time, shortly after the missile has reached its maximum range.
- *Glacial Path* allows **Lissandra** to surpass through every single wall in all maps, so long as the claw is at least halfway through them.
- *Glacial Path* cannot be recast while grounded or rooted.
- *Glacial Path*’s endpoint shows through terrain, fog of war and brush to enemies within 600 range of it.

---

### R: Frozen Tomb

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.375$ (Enemy cast) / None (Self cast) |
| **Target Range** | 550 units |
| **Effect Radius** | 550 units |
| **Cost** | 100 Mana |
| **Cooldown** | 120 / 110 / 100 / 90 / 80 seconds |
| **Targeting** | Unit |
| **Affects** | Enemies, Self |
| **Damage Type** | magic |
| **Spell Shield** | Special |
| **Spell Effects** | spellaoe |
| **Call For Help** | True |

**ACTIVE:** **Lissandra** can cast *Frozen Tomb* on herself or an enemy champion.

**ENEMY CAST:** **Lissandra** freezes the target enemy champion, knocking them down and stunning them for $1.5$ seconds.

**SELF CAST:** **Lissandra** instantly entombs herself in ice, entering stasis for $2.5$ seconds and healing herself every $0.25$ seconds over the duration. The healing is increased by 0%–100%@0–100 (@=**missing** health at the time of cast).

| Attribute | Value |
|-----------|------:|
| **Minimum Heal per Tick** | 10 / 12.5 / 15 / 17.5 / 20 (+ 5.5% AP) |
| **Maximum Heal per Tick** | 20 / 25 / 30 / 35 / 40 (+ 11% AP) |

| Attribute | Value |
|-----------|------:|
| **Minimum Total Heal** | 100 / 125 / 150 / 175 / 200 (+ 55% AP) |
| **Maximum Total Heal** | 200 / 250 / 300 / 350 / 400 (+ 110% AP) |

*Frozen Tomb* creates a field of ice that spreads out from the target over $1.5$ seconds and covers the surrounding area for 3 seconds, dealing magic damage to enemies and slowing them for $0.5$ seconds, refreshing every $0.25$ seconds while they remain.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 150 / 200 / 250 / 300 / 350 (+ 75% AP) |

| Attribute | Value |
|-----------|------:|
| **Slow** | 45 / 52.5 / 60 / 67.5 / 75% |

**Notes:**

- Enemies can only be damaged by the field of ice once every 4 seconds.
  - Under normal circumstances, this means once per cast of *Frozen Tomb*.
- Enemies who come in contact with *Frozen Tomb*’s slow field will be damaged and a small visual and sound effect will play.
- Spell shield will block the single-targeted portion and the radiating ice damage, but will not stop it from spreading.
- If the target becomes untargetable, dies, or is too far away during the cast time, this ability will cancel but does not go on cooldown nor pay its cost.
  - This only applies to the enemy cast.

---

## Patch History

### V25.06
- Iceborn Subjugation
  - **Bug Fixes:** Frozen Thralls can no longer trigger Lane Swap Detector penalties as if they were a valid champion.

### V14.18
- Ice Shard
  - AP ratio reduced to 75% AP from 85% AP.

### V14.17
- Ice Shard
  - Base damage increased to 80 / 115 / 150 / 185 / 220 from 80 / 110 / 140 / 170 / 200.
- Frozen Tomb
  - Slow increased to 45 / 60 / 75% from 30 / 45 / 75%.

### V14.14
- Frozen Tomb
  - **Bug Fixes:** No longer triggers Malignance Hatefog when being blocked by a spell shield.

### V14.13
- Ice Shard
  - AP ratio increased to 85% AP from 80% AP.
- Ring of Frost
  - Cooldown reduced to 10 / 9.5 / 9 / 8.5 / 8 seconds from 12 / 11 / 10 / 9 / 8.

### V14.9
- Stats
  - Selection radius increased to 120 units from $88.8889007568359$.

### V13.22
- Stats
  - Attack speed growth increased to $1.5$% from $1.36$%.

### V13.19
- Ring of Frost
  - Root duration increased to 1.25 / 1.35 / 1.45 / 1.55 / 1.65 seconds from 1.1 / 1.2 / 1.3 / 1.4 / 1.5.

### V13.17
- Ice Shard
  - **Bug Fixes:** Cast of the missile is no longer incorrectly cancelled by crowd control.

### V13.14
- Iceborn Subjugation
  - **Removed:*** Frozen Thralls no longer stop moving to detonate.
  - **New Effect:** Frozen Thralls are now revealed.
  - **New Effect:** Frozen Thralls will now follow targets into brush.
- Ice Shard
  - **New Effect:** Now slows all targets in a radius instead of just the first target hit.
  - **New Effect:** Missile now casts from the position of post-cast instead of pre-cast.
- Frozen Tomb
  - Tick interval reduced to 0.25 seconds per tick from 0.5 seconds per tick.
  - Bonus heal increased to 0%–100%@0–70 (@=**missing** health at the time of cast) from 0%–100%@0–100 (@=**missing** health at the time of cast). *Formula change.*
  - Minimum base heal increased to 100 / 150 / 200 from 90 / 140 / 190.
  - Minimum heal AP ratio increased to 55% AP from 25% AP.
  - Updated logic slightly around stopping dashes.

## Trivia

- Lissandra first appeared in the Journal of Justice, Issue 2, several years before her announcement as a champion. Like now, she was portrayed as the leader of one of the three tribes of the Freljord, but her portrayal was originally vastly different, as a mortal princess allied with Ashe's tribe.
  - This was later revealed to be her alter ego, and most people are not aware of her being the Ice Witch herself.
- If Ashe, Lissandra, and/or Sejuani are played on opposing teams, Battle for Freljord will trigger. This in-game quest consists in one killing the other to earn the title of 'Queen of Freljord' (complete with a floating ice crown floating above their heads) referencing the civil war taking place there, between the three tribes each lead.
- Lissandra is the only champion to possess a single-targeted ability (Frozen Tomb) that can be used on herself but not on allies.
- Lissandra, Elise, and Lucian are the only champions to feature a monologue on their login screens.
- *Lissandra* may be the feminine form of Greek name Λύσανδρος (Lysander or "liberator").
- Her champion theme is the same as the pick music of ARAM and is titled "Freljord".
- Lissandra, Blitzcrank, Caitlyn, Rumble, Sion, Varus, Vi, Viego, Xerath, and Ziggs are the only champions who can apply crowd control on themselves.
- Lissandra's Series 1 Eternals make the following references:
  - *En-Thrall-ing* is a name puns between "Enchanted" with "Frozen Thralls" from the Iceborn Subjugation passive.

---
*This page was automatically generated from League of Legends Wiki data.*