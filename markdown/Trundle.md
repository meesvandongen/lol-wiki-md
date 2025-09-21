# Trundle

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
| **Champion** | Trundle |
| **Title** | the Troll King |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2010-12-01 |
| **Release Patch** | V1.0.0.106 |
| **Roles** | Juggernaut |
| **Riot Positions** | Top |
| **External Positions** | Top, Jungle |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $650.0$ | $+110.0$ | $2520.0$ |
| **Mana** | $340.0$ | $+45.0$ | $1105.0$ |
| **Health Regen** | $6.0$ | $+0.75$ | $18.8$ |
| **Mana Regen** | $7.5$ | $+0.6$ | $17.7$ |
| **Armor** | $37.0$ | $+3.9$ | $103.3$ |
| **Magic Resist** | $32.0$ | $+2.05$ | $66.8$ |
| **Attack Damage** | $68.0$ | $+4.0$ | $136.0$ |
| **Attack Speed** | $0.670$ | $+2.9\%$ | $1.000$ |
| **Movement Speed** | $350.0$ | $+0.0$ | $350.0$ |
| **Attack Range** | $175.0$ | $+0.0$ | $175.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.67$ |
| **Attack Speed Ratio** | $0.67$ |
| **Bonus AS per Level** | $2.9\%$ |
| **Acquisition Radius** | $400 units$ |
| **Pathing Radius** | $25.767 units$ |
| **Selection Radius** | $135 units$ |
| **Selection Height** | $130 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: King's Tribute

**Innate:** Enemies that die near **Trundle** cause him to heal based on their **maximum** health.

**Innate:** Whenever a nearby enemy dies, **Trundle** heal himself for key=% of the target's **maximum** health.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Self |
| **Effect Radius** | 1400 units |

**Notes:**

- 'King's Tribute' does not trigger when an enemy structure is destroyed.

---

### Q: Chomp

**Active:** **Trundle**’s next basic attack within a few seconds will deal **bonus** physical damage and briefly slow the target.

*Afterwards, **Trundle** gains **bonus attack damage** for a few seconds, during which he reduces the target's attack damage.*

**Active:** **Trundle** empowers his next basic attack within 7 seconds to have an uncancellable windup, gain *25 **bonus** range*, deal **bonus** physical damage and slow the target by 75% for $0.1$ seconds. After using the empowered attack, **Trundle** gains **bonus attack damage** for 5 seconds and reduces the target's **bonus** attack damage by half that amount for the same duration. *Chomp basic attack reset *'Trundle's* basic attack timer.*

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $3.5$ seconds |
| **Cast Time** | none |
| **Cost** | 20 Mana |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | Special |
| **Spell Effects** | spell |

**Scaling:**
- **Bonus Physical Damage:** $10-90$ (+ $15-55$% AD)
- **Bonus Attack Damage:** $20-40$
- **Attack Damage Reduction:** $20/2-40/2$

**Notes:**

- *Chomp* will still incur the *attack damage* increase for **Trundle** even if the attack is dodged, blocked, missed while he is blind, or it is blocked by spell shield.
- Spell shield will only block the attack damage reduction and slow.
- If the attack damage reduction would reduce the target's **bonus** attack damage below 0, the target's **base** attack damage value is unaffected, but their **total** attack damage will still be reduced. The target will however retain a **bonus** attack damage value of 0.
  - Effects that scale with **total** attack damage, such as basic attacks, will take the attack damage reduction into account for their calculations. ** Those that have a ratio for base*** attack damage or **bonus** attack damage do not take the reduction into account.

---

### W: Frozen Domain

**Active:** **Trundle** coats the target location in ice for a period. While he is within the area, he gains **bonus attack speed**, *ms **bonus** movement speed*, and increased healing from all sources.

**Active:** **Trundle** coats the target location in ice for 8 seconds. While he is within the area, he gains **bonus attack speed**, *ms **bonus** movement speed*, and 25% increased healing from all sources.

| Attribute | Value |
|-----------|-------|
| **Range** | 750 units |
| **Cooldown** | $18-14$ seconds |
| **Cast Time** | none |
| **Cost** | 40 Mana |
| **Targeting** | Location |
| **Affects** | Self |
| **Effect Radius** | 775 units |

**Scaling:**
- **Bonus Attack Speed:** $30-90$%
- **Bonus Movement Speed:** $20-52$%

**Notes:**

- No additional details.

---

### E: Pillar of Ice

**Active:** **Trundle** erects an ice pillar at the target location that remains for a few seconds, which airborne units hit. The pillar acts as terrain and slows nearby enemies.

**Active:** **Trundle** erects a pillar of ice at the target location for 6 seconds, which airborne units hit to 225 units from its center. The pillar acts as terrain and slows nearby enemies.

| Attribute | Value |
|-----------|-------|
| **Range** | 1000 units |
| **Cooldown** | $21-15$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | 75 Mana |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Effect Radius** | 360 / 225 units |
| **Spell Shield** | False |

**Scaling:**
- **Slow:** $34-50$%

**Notes:**

- *Pillar of Ice* displaces allied units away from the area but does not render them airborne.
  - Allied channel (e.g. Recall, Teleport) will be interrupt however.
- *Pillar of Ice* cannot be placed inside impassable terrain. Attempting to do so will cause the pillar to spawn in the nearest available spot of accessible terrain.
- *Pillar Of Ice* triggers effects such as drawing turret aggro, *Sudden Impact* and applying *Elixir of Sorcery* by dealing 0 proc damage true damage.

---

### R: Subjugate

**Active:** **Trundle** drains the life force of the target enemy champion, dealing magic damage based on their **maximum** health and heal for the same amount. He also steals a portion of their armor penetration and magic penetration, and increases in while reducing their size.

*Half of the damage and stealing is applied instantly, and the other half is applied over a few seconds. The armor and magic resist will remain stolen for a few seconds afterwards.*

**Active:** **Trundle** drains the life force out of the target enemy champion, dealing magic damage and heal himself for the same amount. He also steals 40% of their **current** armor penetration and magic penetration, and increases in by 18% while reducing the target's by $9.9$%. Half of the **total** damage and stealing are applied instantly, and the other half, as well as the modifiers, are applied every second over the next 4 seconds, even if the target has death. The armor and magic resistance will remain stolen for 4 seconds after the drain has ended.

| Attribute | Value |
|-----------|-------|
| **Range** | 650 units |
| **Cooldown** | $120-80$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | 100 Mana |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | magic |
| **Spell Shield** | True |
| **Spell Effects** | dot |

**Scaling:**
- **Total Magic Damage:** $20-30$%
- **Initial Magic Damage:** $20/2-30/2$%

**Notes:**

- The **total** value of armor and magic resist stolen and damage dealt is determined at the time of cast. The stolen stats do not update dynamically relative to the target's stats at any point during the effect, as the steal is applied to the **current** value of armor and magic resistance.
- **Trundle** gains $11.7$ extra *range* on his basic attacks (relative to his and enemy center) as a consequence of his increased size.
  - So do his enemies.
  - He gains $3.78$ to $6.255$ range against the target he *subjugated*, depending on their base size, and so does the target against him.
- The debuff on the target also *persists through death*.
- **Trundle** will lose the buff if the target loses the debuff, for example if it enters resurrection.

---

## Patch History

### V25.16
- *Frozen Domain*
  - Bonus attack speed reduced to $30-90$% from $30-110$%.
  - Cooldown increased to $18-14$ seconds from $16-12$.

### V25.07
- *Pillar of Ice*
  - Cooldown reduced to $21-15$ seconds from $24-16$.
  - Slow increased to $34-50$% from $30-46$%.

### V14.21
- General
  - **Bug Fixes:** No longer sometimes accidentally plays the default (en-us) version of his VO in other localizations.

### V14.5
- *Pillar of Ice*
  - **Bug Fixes:** Pillar no longer grants vision around itself.

### V14.3
- Stats
  - Base health reduced to 650 from 686.
- *Chomp*
  - Base damage reduced to $10-90$ from $20-100$.

### V14.2
- *Chomp*
  - **New Effect:** Empowered attack now gains 25 bonus attack range.

### V13.23
- Stats
  - Attack damage growth increased to 4 from 3.
- *Chomp*
  - Mana cost reduced to 20 from 30.

### V13.9
- Stats
  - Base mana increased to 340 from 281.
  - Base attack speed increased to $0.67$ from $0.6$.

### V13.4
- *Subjugate*
  - **Undocumented/Bug Fix:** **Trundle** no longer loses his own buff if the target manages to dispel its debuff (e.g. by having *Guardian Angel* triggered).

### V13.3
- General
  - **New Effect:** Dance animation speed now scales with movement speed.
- *Frozen Domain*
  - Cooldown reduced to $16-12$ seconds from $18-14$.

## Trivia

- The word "trundle" means to "move or cause to move slowly and heavily, typically in a noisy or uneven way."
- Trundle was the first champion to cost .
- Trundle's hair resembles the Troll doll hair.
- Trundle's Recall animation is the famous "Winter is Coming" pose from the television series *Game of Thrones*.
- The area for *Frozen Domain* is currently, not counting global abilities, the largest AoE in the game.
- Trundle, **Karma**, **Lee Sin**, and **Sejuani** are the only champions to feature a 'traditional' skin representing them before their visual updates (his was conceived while producing a 'Frost Troll' skin)
- Trundle's dance references Dancing Trollface.
  - A side-by-side comparison can be seen here.
  - He will dance faster the more movement speed he has.
- **Trundle**, **Vayne**, and **Warwick** are the only champions without damaging area-of-effect abilities.

---
*This page was automatically generated from League of Legends Wiki data.*