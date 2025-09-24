# Trundle

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
| **Champion** | Trundle |
| **Title** | the Troll King |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2010-12-01 |
| **Release Patch** | V1.0.0.106 |
| **Latest Changes** | V25.16 |
| **Roles** | Juggernaut |
| **Riot Positions** | Top |
| **External Positions** | Top, Jungle |
| **Blue Essence** | 1575 |
| **Riot Points** | 790 |
| **Difficulty** | 1 |
| **Hero Type** | Fighter |
| **Alt Type** | Tank |
| **Adaptive Type** | Physical |
| **Damage** | 2 |
| **Toughness** | 3 |
| **Control** | 1 |
| **Mobility** | 1 |
| **Utility** | 2 |
| **Style** | 20 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $650.0$ | $+110.0$ |
| **Mana** | $340.0$ | $+45.0$ |
| **Health Regen** | $6.0$ | $+0.75$ |
| **Mana Regen** | $7.5$ | $+0.6$ |
| **Armor** | $37.0$ | $+3.9$ |
| **Magic Resist** | $32.0$ | $+2.05$ |
| **Attack Damage** | $68.0$ | $+4.0$ |
| **Attack Speed** | $0.670$ | |
| **Movement Speed** | $350.0$ | $+0.0$ |
| **Attack Range** | $175.0$ | $+0.0$ |
| **Base Attack Speed** | $0.67$ | |
| **Attack Speed Ratio** | $0.67$ | |
| **Bonus AS per Level** | $2.9\%$ | |
| **Acquisition Radius** | $400$ units | |
| **Pathing Radius** | $25.767$ units | |
| **Selection Radius** | $135$ units | |
| **Selection Height** | $130$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Taken** | $105.0\%$ |
| **Healing** | $100.0\%$ |

#### URF

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $95.0\%$ |
| **Damage Taken** | $95.0\%$ |
| **Healing** | $80.0\%$ |

#### Ultimate Spellbook

| Metric | Value |
|--------|------:|
| **Damage Taken** | $105.0\%$ |

## Abilities

### Passive: King's Tribute

| Attribute | Value |
|-----------|------:|
| **Effect Radius** | 1400 units |
| **Targeting** | Passive |
| **Affects** | Self |

**INNATE:** Whenever a nearby enemy dies, **Trundle** heals himself for 1.8 to 5.5 of the target's **maximum** health.

**Notes:**

- *King's Tribute* does not trigger when an enemy structure is destroyed.

---

### Q: Chomp

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Cost** | 20 Mana |
| **Cooldown** | $3.5$ seconds |
| **Cooldown Start** | post-effect |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | Special |
| **Spell Effects** | spell |
| **Parry** | Special |

**ACTIVE:** **Trundle** empowers his next basic attack within 7 seconds to have an uncancellable windup, gain 25 **bonus** range, deal **bonus** physical damage and slow the target by 75% for $0.1$ seconds.

| Attribute | Value |
|-----------|------:|
| **Bonus Physical Damage** | 10 / 30 / 50 / 70 / 90 (+ 15 / 25 / 35 / 45 / 55% AD) |

After using the empowered attack, **Trundle** gains **bonus** attack damage for 5 seconds and reduces the target's **bonus** attack damage by half that amount for the same duration.

| Attribute | Value |
|-----------|------:|
| **Bonus Attack Damage** | 20 / 25 / 30 / 35 / 40 |

| Attribute | Value |
|-----------|------:|
| **Attack Damage Reduction** | 10 / 12.5 / 15 / 17.5 / 20 |

*Chomp resets **Trundle**’s basic attack timer.*

**Notes:**

- *Chomp* will still incur the attack damage increase for **Trundle** even if the attack is dodged, blocked, missed while he is blinded, or it is blocked by spell shield.
- Spell shield will only block the attack damage reduction and slow.
- If the attack damage reduction would reduce the target's **bonus** attack damage below 0, the target's **base** attack damage value is unaffected, but their **total** attack damage will still be reduced. The target will however retain a **bonus** attack damage value of 0.
  - Effects that scale with **total** attack damage, such as basic attacks, will take the attack damage reduction into account for their calculations.
    - Those that have a ratio for **base** attack damage or **bonus** attack damage do not take the reduction into account.

---

### W: Frozen Domain

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 750 units |
| **Effect Radius** | 775 units |
| **Cost** | 40 Mana |
| **Cooldown** | 18 / 17 / 16 / 15 / 14 seconds |
| **Targeting** | Location |
| **Affects** | Self |
| **Out of Range Behavior** | cast at max |

**ACTIVE:** **Trundle** coats the target location in ice for 8 seconds. While he is within the area, he gains **bonus** attack speed, (ms) **bonus** movement speed, and 25% increased healing from all sources.

| Attribute | Value |
|-----------|------:|
| **Bonus Attack Speed** | 30 / 45 / 60 / 75 / 90% |

| Attribute | Value |
|-----------|------:|
| **Bonus Movement Speed** | 20 / 28 / 36 / 44 / 52% |

**Notes:**

- No additional details.

---

### E: Pillar of Ice

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 1000 units |
| **Effect Radius** | cr 360 (Slow zone radius) / 225 (Knockback radius and destination) |
| **Cost** | 75 Mana |
| **Cooldown** | 21 / 19.5 / 18 / 16.5 / 15 seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Spell Shield** | False |
| **Out of Range Behavior** | walk to location |
| **Pillar radius** | pathing radiusgameplay radius 150 (Both pathing and hitbox radius for certain spells that can collide with player-generated terrain) |

**ACTIVE:** **Trundle** erects a pillar of ice at the target location for 6 seconds, which knocks back units hit to 225 units from its center. The pillar acts as terrain and slows nearby enemies.

| Attribute | Value |
|-----------|------:|
| **Slow** | 34 / 38 / 42 / 46 / 50% |

**Notes:**

- *Pillar of Ice* displaces allied units away from the area but does not render them airborne.
  - Allied channels (e.g. Recall, Teleport) will be interrupted however.
- *Pillar of Ice* cannot be placed inside impassable terrain. Attempting to do so will cause the pillar to spawn in the nearest available spot of accessible terrain.
- *Pillar Of Ice* triggers effects such as drawing turret aggro, Sudden Impact and applying Elixir of Sorcery by dealing 0 proc true damage.

---

### R: Subjugate

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 650 units |
| **Cost** | 100 Mana |
| **Cooldown** | 120 / 110 / 100 / 90 / 80 (Starts on cast) seconds |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | magic |
| **Spell Shield** | True |
| **Spell Effects** | dot |
| **Call For Help** | True |

**ACTIVE:** **Trundle** drains the life force out of the target enemy champion, dealing magic damage and healing himself for the same amount. He also steals 40% of their **current** (armor penetration) armor and (magic penetration) magic resistance, and increases in by 18% while reducing the target's by $9.9$%.

| Attribute | Value |
|-----------|------:|
| **Total Magic Damage** | 20 / 22.5 / 25 / 27.5 / 30% (+ 2% per 100 AP) of the target's **maximum** health |

Half of the **total** damage and stealing are applied instantly, and the other half, as well as the modifiers, are applied every second over the next 4 seconds, even if the target has died.

| Attribute | Value |
|-----------|------:|
| **Initial Magic Damage** | 10 / 11.25 / 12.5 / 13.75 / 15% (+ 1% per 100 AP) of the target's **maximum** health |
| **Magic Damage Per Second** | 2.5 / 2.8125 / 3.125 / 3.4375 / 3.75% (+ $0.25$% per 100 AP) of the target's **maximum** health |

The armor and magic resistance will remain stolen for 4 seconds after the drain has ended.

**Notes:**

- The **total** value of armor and magic resist stolen and damage dealt is determined at the time of cast. The stolen stats do not update dynamically relative to the target's stats at any point during the effect, as the steal is applied to the **current** value of armor and magic resistance.
- **Trundle** gains $11.7$ extra range on his basic attacks (relative to his and enemy center) as a consequence of his increased size.
  - So do his enemies.
  - He gains $3.78$ to $6.255$ range against the target he *subjugated*, depending on their base size, and so does the target against him.
- The debuff on the target also *persists through death*.
- **Trundle** will lose the buff if the target loses the debuff, for example if it enters resurrection.

---

## Patch History

### V25.16
- Frozen Domain
  - Bonus attack speed reduced to 30 / 45 / 60 / 75 / 90% from 30 / 50 / 70 / 90 / 110%.
  - Cooldown increased to 18 / 17 / 16 / 15 / 14 seconds from 16 / 15 / 14 / 13 / 12.

### V25.07
- Pillar of Ice
  - Cooldown reduced to 21 / 19.5 / 18 / 16.5 / 15 seconds from 24 / 22 / 20 / 18 / 16.
  - Slow increased to 34 / 38 / 42 / 46 / 50% from 30 / 34 / 38 / 42 / 46%.

### V14.21
- General
  - **Bug Fixes:** No longer sometimes accidentally plays the default (en-us) version of his VO in other localizations.

### V14.5
- Pillar of Ice
  - **Bug Fixes:** Pillar no longer grants vision around itself.

### V14.3
- Stats
  - Base health reduced to 650 from 686.
- Chomp
  - Base damage reduced to 10 / 30 / 50 / 70 / 90 from 20 / 40 / 60 / 80 / 100.

### V14.2
- Chomp
  - **New Effect:** Empowered attack now gains 25 bonus attack range.

### V13.23
- Stats
  - Attack damage growth increased to 4 from 3.
- Chomp
  - Mana cost reduced to 20 from 30.

### V13.9
- Stats
  - Base mana increased to 340 from 281.
  - Base attack speed increased to $0.67$ from $0.6$.

### V13.4
- Subjugate
  - **UNDOCUMENTED/BUG FIX:** **Trundle** no longer loses his own buff if the target manages to dispel its debuff (e.g. by having Guardian Angel triggered).

### V13.3
- General
  - **New Effect:** Dance animation speed now scales with movement speed.
- Frozen Domain
  - Cooldown reduced to 16 / 15 / 14 / 13 / 12 seconds from 18 / 17 / 16 / 15 / 14.

## Trivia

- The word "trundle" means to "move or cause to move slowly and heavily, typically in a noisy or uneven way."
- Trundle was the first champion to cost .
- Trundle's hair resembles the Troll doll hair.
- Trundle's Recall animation is the famous "Winter is Coming" pose from the television series *Game of Thrones*.
- The area for Frozen Domain is currently, not counting global abilities, the largest AoE in the game.
- Trundle, Karma, Lee Sin, and Sejuani are the only champions to feature a 'traditional' skin representing them before their visual updates (his was conceived while producing a 'Frost Troll' skin)
- Trundle's dance references Dancing Trollface.
  - A side-by-side comparison can be seen here.
  - He will dance faster the more movement speed he has.
- Trundle, Vayne, and Warwick are the only champions without damaging area-of-effect abilities.

---
*This page was automatically generated from League of Legends Wiki data.*