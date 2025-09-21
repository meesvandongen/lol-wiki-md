# Lissandra

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
| **Champion** | Lissandra |
| **Title** | the Ice Witch |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2013-04-30 |
| **Release Patch** | V3.6 |
| **Roles** | Burst |
| **Riot Positions** | Middle |
| **External Positions** | Middle |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $620.0$ | $+110.0$ | $2490.0$ |
| **Mana** | $475.0$ | $+30.0$ | $985.0$ |
| **Health Regen** | $7.0$ | $+0.55$ | $16.4$ |
| **Mana Regen** | $8.0$ | $+0.4$ | $14.8$ |
| **Armor** | $22.0$ | $+4.9$ | $105.3$ |
| **Magic Resist** | $30.0$ | $+1.3$ | $52.1$ |
| **Attack Damage** | $55.0$ | $+2.7$ | $100.9$ |
| **Attack Speed** | $0.656$ | $+1.5\%$ | $0.823$ |
| **Movement Speed** | $325.0$ | $+0.0$ | $325.0$ |
| **Attack Range** | $550.0$ | $+0.0$ | $550.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.656$ |
| **Attack Speed Ratio** | $0.625$ |
| **Bonus AS per Level** | $1.5\%$ |
| **Missile Speed** | $2200 units/second$ |
| **Acquisition Radius** | $600 units$ |
| **Pathing Radius** | $35 units$ |
| **Selection Radius** | $120 units$ |
| **Selection Height** | $250 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Iceborn Subjugation

**Innate:** Whenever an enemy champion dies while **Lissandra** is nearby, she spawns a *Frozen Thrall* from their corpse.

*Frozen Thralls* are invulnerable and untargetable ice spirits that slow nearby enemies and gain increasing ms over their lifespan. They will chase nearby enemies for a few seconds before exploding, dealing magic damage in an area.

**Innate:** Whenever a nearby enemy champion dies, **Lissandra** spawns a *Frozen Thrall* from their corpse. *Frozen Thralls* are ice spirits that have 325*.103–325@0–4 (@=seconds alive) movement speed and slow nearby enemies by 25%. They will chase nearby sight enemies for 4 seconds, prioritizing champion, after which they shatter to deal 120 to 340–520 (+ 50% AP) magic damage to nearby enemies.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | 1350 / cr 450 units |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |

**Notes:**

- *Thralls* are units that are considered champions (they are clone) and they take the form of the champion they arise from.
  - Additionally, near the end of their life span, they will cue that champion's basic attack animation.
  - They cannot be teleported by an allied *Realm Warp*.
- *Thralls* are untargetable and invulnerable.
- *Thralls* are revealed to enemies through the fog of war.
- *Thralls* will continue to chase their target even if they enter a brush.
- *Iceborn Subjugation* will not summon a *Thrall* against enemy champions that enter a zombie state, and will instead summon one after they've left the state.

---

### Q: Ice Shard

**Active:** **Lissandra** throws a spear of ice in the target direction that slows the first target hit and deals magic damage to all enemies it passes through.

*If *Ice Shard* hits an enemy, it will shatter, increasing its width and maximum range.*

**Active:** **Lissandra** launches a shard of ice in the target direction that deals magic damage and slow enemies hit for $1.5$ seconds. If *Ice Shard* hits an enemy, it will shatter, increasing its width and maximum range.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $8-4$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $55-75$ Mana |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | magic |
| **Speed** | 2200 units/second |
| **Spell Shield** | Special |
| **Spell Effects** | aoe |
| **Projectile** | True |

**Scaling:**
- **Magic Damage:** $80-220$ (+ 75% AP)
- **Slow:** $20-36$%

**Notes:**

- *Ice Shard* picks a location 950 units away in the direction of the cast for the shattered missile to end up at.
- The initial *Ice Shard* missile has a range of 700 units which it arrives at after . If it hasn't collided with an enemy in this path when it does, it checks for enemies in a cr 100 radius around the point 25 units in front of it.
  - Colliding or hitting an enemy in either fashion creates a new "shattered" missile with the same speed but greater width that continues to travel along the same line to the designated point 950 units from the cast's original position, originating at the location at which the initial missile collided at.
- Spell shield will block the damage and the slow but will not stop the projectile from shattering.

---

### W: Ring of Frost

**Active:** **Lissandra** freezes nearby enemies, dealing magic damage and briefly root them.

**Active:** **Lissandra** freezes nearby enemies, dealing magic damage and root them for a duration.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $10-8$ seconds |
| **Cast Time** | none |
| **Cost** | 40 Mana |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | magic |
| **Effect Radius** | 275 units |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |

**Scaling:**
- **Magic Damage:** $70-210$ (+ 70% AP)
- **Root Duration:** $1.25-1.65$ seconds

**Notes:**

- No additional information.

---

### E: Glacial Path

**Active:** **Lissandra** sends a claw of ice in the target direction that deals magic damage to enemies it passes through. *Glacial Path* can be recast while the claw is active.

**Recast:** **Lissandra** blinks to the claw.

**Active:** **Lissandra** sends a claw of ice in the target direction that deals magic damage to enemies it passes through, decelerating over $1.25$ seconds. *Glacial Path* can be recast after $0.5$ seconds while the claw is active. **Recast:** **Lissandra** consumes the claw and blinks to its current location.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $24-12$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $80-100$ Mana |
| **Targeting** | Direction / Auto |
| **Affects** | Enemies, Self |
| **Damage Type** | magic |
| **Speed** | 1200 / 640 units/second |
| **Spell Shield** | True |
| **Spell Effects** | aoe |
| **Projectile** | True |

**Scaling:**
- **Magic Damage:** $70-210$ (+ 60% AP)

**Notes:**

- **Lissandra** can reactivate *Glacial Path* for the duration (plus another $0.2$-$0.3$ seconds after the claw 'sinks' into the ground).
  - The Claw can also hit enemies near the end point at this time, shortly after the missile has reached its maximum range.
- *Glacial Path* allows **Lissandra** to surpass through every single wall in all maps, so long as the claw is at least halfway through them.
- *Glacial Path* cannot be recast while ground or root.
- 'Glacial Path's endpoint shows through terrain, fog of war and brush to enemies within 600 range of it.

---

### R: Frozen Tomb

**Active:** **Lissandra** can cast *Frozen Tomb* on herself or an enemy champion.

**Enemy Cast:** **Lissandra** briefly freezes the target enemy champion, knockdown and stun them.

**Active:** **Lissandra** can cast *Frozen Tomb* on herself or an enemy champion. **Enemy Cast:** **Lissandra** freezes the target enemy champion, knockdown and stun them for $1.5$ seconds. **Self Cast:** **Lissandra** instantly entombs herself in ice, entering stasis (buff) for $2.5$ seconds and healing herself every $0.25$ seconds over the duration. The healing is increased by type=**missing** health at the time of cast. *Frozen Tomb* creates a field of ice that spreads out from the target over $1.5$ seconds and covers the surrounding area for 3 seconds, dealing magic damage to enemies and slow them for $0.5$ seconds, refreshing every $0.25$ seconds while they remain.

| Attribute | Value |
|-----------|-------|
| **Range** | 550 units |
| **Cooldown** | $120-80$ seconds |
| **Cast Time** | $0.375$ / None |
| **Cost** | 100 Mana |
| **Targeting** | Unit |
| **Affects** | Enemies, Self |
| **Damage Type** | magic |
| **Effect Radius** | 550 units |
| **Spell Shield** | Special |
| **Spell Effects** | spellaoe |

**Scaling:**
- **Minimum Heal per Tick:* $100/10-200/10$ (+ $55/10$% AP)2/10-200×2/10$ (+ $55×2/10$% AP) **Minimum Total Heal:* $100-200$ (+ 55% AP)2-200×2$ (+ $55×2$% AP)

**Notes:**

- Enemies can only be damaged by the field of ice once every 4 seconds.
  - Under normal circumstances, this means once per cast of *Frozen Tomb*.
- Enemies who come in contact with 'Frozen Tomb's slow field will be damaged and a small visual and sound effect will play.
- Spell shield will block the single-targeted portion and the radiating ice damage, but will not stop it from spreading.
- If the target becomes untargetable, death, or is too far away during the cast time, this ability will cancel but does not go on *cooldown* nor pay its cost.
  - This only applies to the enemy cast.

---

## Patch History

### V25.06
- *Iceborn Subjugation*
  - **Bug Fixes:** Frozen Thralls can no longer trigger *Lane Swap Detector* penalties as if they were a valid champion.

### V14.18
- *Ice Shard*
  - AP ratio reduced to 75% AP from 85% AP.

### V14.17
- *Ice Shard*
  - Base damage increased to $80-220$ from $80-200$.
- *Frozen Tomb*
  - Slow increased to $45-75 3$% from $30/45/75$%.

### V14.14
- *Frozen Tomb*
  - **Bug Fixes:** No longer triggers *Malignance* Hatefog when being blocked by a spell shield.

### V14.13
- *Ice Shard*
  - AP ratio increased to 85% AP from 80% AP.
- *Ring of Frost*
  - Cooldown reduced to $10-8$ seconds from $12-8$.

### V14.9
- Stats
  - Selection radius increased to 120 units from $88.8889007568359$.

### V13.22
- Stats
  - Attack speed growth increased to $1.5$% from $1.36$%.

### V13.19
- *Ring of Frost*
  - Root duration increased to $1.25-1.65$ seconds from $1.1-1.5$.

### V13.17
- *Ice Shard*
  - **Bug Fixes:** Cast of the missile is no longer incorrectly cancelled by crowd control.

### V13.14
- *Iceborn Subjugation*
  - **Removed:*** Frozen Thralls no longer stop moving to detonate.
  - **New Effect:** Frozen Thralls are now revealed.
  - **New Effect:** Frozen Thralls will now follow targets into brush.
- *Ice Shard*
  - **New Effect:** Now slows all targets in a radius instead of just the first target hit.
  - **New Effect:** Missile now casts from the position of post-cast instead of pre-cast.
- *Frozen Tomb*
  - Tick interval reduced to 0.25 seconds per tick from 0.5 seconds per tick.
  - Bonus heal increased to type=**missing** health at the time of cast from type=**missing** health at the time of cast. *Formula change.*
  - Minimum base heal increased to $100-200 3$ from $90-190 3$.
  - Minimum heal AP ratio increased to 55% AP from 25% AP.
  - Updated logic slightly around stopping dashes.

## Trivia

- Lissandra first appeared in the Journal of Justice, Issue 2, several years before her announcement as a champion. Like now, she was portrayed as the leader of one of the three tribes of the Freljord, but her portrayal was originally vastly different, as a mortal princess allied with **Ashe**'s tribe.
  - This was later revealed to be her alter ego, and most people are not aware of her being the Ice Witch herself.
- If Ashe, Lissandra, and/or **Sejuani** are played on opposing teams, Battle for Freljord will trigger. This in-game quest consists in one killing the other to earn the title of 'Queen of Freljord' (complete with a floating ice crown floating above their heads) referencing the civil war taking place there, between the three tribes each lead.
- Lissandra is the only champion to possess a single-targeted ability (*Frozen Tomb*) that can be used on herself but not on allies.
- Lissandra, **Elise**, and **Lucian** are the only champions to feature a monologue on their login screens.
- *Lissandra* may be the feminine form of Greek name Λύσανδρος (Lysander or "liberator").
- Her champion theme is the same as the pick music of ARAM and is titled "Freljord".
- Lissandra, **Blitzcrank**, **Caitlyn**, **Rumble**, **Sion**, **Varus**, **Vi**, **Viego**, **Xerath**, and **Ziggs** are the only champions who can apply crowd control on themselves.
- Lissandra's Series 1 Eternals make the following references:
  - *En-Thrall-ing* is a name puns between "Enchanted" with "Frozen Thralls" from the *Iceborn Subjugation* passive.

---
*This page was automatically generated from League of Legends Wiki data.*