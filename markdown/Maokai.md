# Maokai

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
| **Champion** | Maokai |
| **Title** | the Twisted Treant |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2011-02-16 |
| **Release Patch** | V1.0.0.111 |
| **Roles** | Vanguard |
| **Riot Positions** | Support |
| **External Positions** | Jungle, Support |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $665.0$ | $+109.0$ | $2518.0$ |
| **Mana** | $375.0$ | $+43.0$ | $1106.0$ |
| **Health Regen** | $5.0$ | $+0.75$ | $17.8$ |
| **Mana Regen** | $6.0$ | $+0.6$ | $16.2$ |
| **Armor** | $35.0$ | $+5.2$ | $123.4$ |
| **Magic Resist** | $32.0$ | $+2.05$ | $66.8$ |
| **Attack Damage** | $64.0$ | $+3.3$ | $120.1$ |
| **Attack Speed** | $0.800$ | $+2.1\%$ | $1.089$ |
| **Movement Speed** | $335.0$ | $+0.0$ | $335.0$ |
| **Attack Range** | $125.0$ | $+0.0$ | $125.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.8$ |
| **Attack Speed Ratio** | $0.695$ |
| **Bonus AS per Level** | $2.1\%$ |
| **Acquisition Radius** | $600 units$ |
| **Gameplay Radius** | $80 units$ |
| **Pathing Radius** | $50 units$ |
| **Selection Radius** | $125 units$ |
| **Selection Height** | $190 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Sap Magic

**Innate:** Periodically, **Maokai**’s next basic attack will heal him for a portion of his **maximum** health.

*Each time **Maokai** casts an ability or is hit by an enemy ability, *Sap Magic* cooldown is cdr by a few seconds.*

**Innate:** Periodically, **Maokai** empowers his next basic attack to have an uncancellable windup and heal him for 4–12.8 **maximum** health after a $0.25$-second delay. Each time **Maokai** casts an ability, hits at least one enemy champion or epic monster with **Sapling Toss**, or is struck by an enemy's ability, 'Sap Magic's *cooldown* is reduced by 4 seconds, modified to $1.5$ if he is hit by a large monster basic attack or ability. *Sap Magic basic attack reset *'Maokai's** basic attack timer, and will not trigger if he is above *95% **maximum'* health*.*

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Self |

**Notes:**

- *Sap Magic* does not occur if the attack is dodge and/or missed if **Maokai** is blind. It will occur if it is block.
- The striking ability has to trigger spell effects such as spell damage or area damage for the cooldown to be reduced.
- If **Maokai** possesses a spell shield and it consumes an enemy ability, he will still receive the cooldown reduction.
- The empowered attack will not trigger against structures nor wards.

---

### Q: Bramble Smash

**Active:** **Maokai** sends a shockwave in the target direction that deals magic damage and briefly slow enemies hit.

*Enemies near **Maokai** are also briefly stun and airborne.*

**Active:** **Maokai** sends a shockwave in the target direction that deals magic damage to enemies hit and slow them by 99% for $0.25$ seconds. *Bramble Smash* deals **bonus** magic damage to monsters. Enemies near **Maokai** are also stun for $0.5$ seconds and airborne up to 300 units based on their proximity to him.

| Attribute | Value |
|-----------|-------|
| **Range** | 600 units |
| **Cooldown** | $7-5$ seconds |
| **Cast Time** | 0.3 seconds |
| **Cost** | 40 Mana |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Speed** | 1600 units/second |
| **Effect Radius** | 325 units |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |
| **Projectile** | True |

**Scaling:**
- **Magic Damage:** $65-245$
- **maximum** health) (+ 40% AP) **Bonus Monster Damage:* $120-160$*maximum** health) (+ 40% AP)

**Notes:**

- 'Bramble Smash's damage based on the target's health ratio is capped at 9999 against non-champions.
- Displacement immunity will also resist the application of the stun.

---

### W: Twisted Advance

**Active:** **Maokai** dashes to the target enemy while being untargetable, dealing magic damage and briefly root them upon arrival.

**Active:** **Maokai** dashes to the target enemy while being untargetable. Upon arrival, he deals magic damage and root them for a duration.

| Attribute | Value |
|-----------|-------|
| **Range** | 525 units |
| **Cooldown** | $14-10$ seconds |
| **Cast Time** | none |
| **Cost** | 60 Mana |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Speed** | 1300 units/second |
| **Spell Shield** | True |
| **Spell Effects** | spell |

**Scaling:**
- **Magic Damage:** $60-160$ (+ 40% AP)
- **Root Duration:** $1-1.4$ seconds

**Notes:**

- **Maokai** will track the target if they change locations.
  - He will dash to the target's previous location without applying 'Twisted Advance's effects if the target is too far away or moves beyond 2000 units.
  - He will stop dashing if the target is too far away the moment the dash begins.
- *Twisted Advance* will fail to deal damage and apply the root if the target is untargetable upon arrival.

---

### E: Sapling Toss

**Active:** **Maokai** flings a *Sapling* to the target location, where it remains for some time. A *Sapling* will chase nearby enemies for a short time, expiring afterward or upon contact.

*When a *Sapling* expires, it deals magic damage to nearby enemies based on their **maximum** health and slow them for a short time.*

**Active:** **Maokai** projectile a *Sapling* to the target location, granting sight of the area. Once landed, a *Sapling* remains stationary for 30 seconds or until it reacts to the first nearby visible enemy, chasing them for up to $2.5$ seconds. The *Sapling* explodes upon colliding with an enemy or when it expires, dealing magic damage to nearby enemies, capped at 300 against non-champions, and slow them by 45% for 2 seconds. A *Sapling* placed in a brush becomes empowered: lasting 30 (+ $1.5$% **bonus** health) seconds and causing its explosion to deal $66.7$% damage to non-minion targets struck and attach two *Saplings* to them for $1.5$ seconds, as well as slow enemies hit by 45% (+ 1% per 100 **bonus** health) (+ 1% per 100 AP) for 2 seconds and standard sight them for 3 seconds. The attached *Saplings* explode on the afflicted target every $0.75$ seconds over the duration, dealing them the same damage each time. Targets can only be attached to two *Saplings* at a time; subsequent empowered *Sapling* explosions on a target will refresh the duration and only attach a *Sapling* if one has exploded on them already. Against minions, the explosion deals 100% damage. The **total** damage is capped at 600 against non-champions. *See [Pets](#Pets) for more details about Saplings.*

| Attribute | Value |
|-----------|-------|
| **Range** | 1100 units |
| **Cooldown** | $18-14$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $60-80$ Mana |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | 175 / 350 / 475 units |
| **Spell Shield** | True |
| **Spell Effects** | Special |
| **Projectile** | True |

**Scaling:**
- **Magic Damage:** $50-150$
- **bonus** health) (+ 25% AP)
- **Magic Damage per Instance:** $50×2/3-1502/32/3 (+ $25×2/3
- **Total Attached Sapling Damage:** $(50×2/3-150×2/3)22/3)*2 (+ $(25×2/3)*2
- **Total Magic Damage:** $50×2-150×2$ (+ $5×2$%
- **bonus** health) (+ $25×2$% AP)
- **Minion Damage:** $50-150$ (+ 5%
- **bonus** health) (+ 25% AP)

**Notes:**

- Applies area damage from a normal *Sapling* and deals persistent damage to enemies hit by an empowered *Sapling*.
- *Sapling Toss* grants sight of the target location even while the *Sapling* is in flight.
- The *Saplings* will continue to chase their target even if they lose sight of them (but they do not prioritize champions so they will chase the first target they encounter).
- *Saplings* cannot stack on top of each other (they will move a short distance away from one another if they are aimed at the same location).
- The *Sapling* will stop its movement upon colliding with *Unbreakable*.
- *Saplings* attack Baron Nashor always at the same two points, no matter where they were placed.

---

### R: Nature's Grasp

**Active:** **Maokai** summons a colossal wall of five thorny brambles that slowly advances in the target direction, each stopping when they collide with an enemy champion.

*Each bramble deals magic damage to enemies hit and root them for a short time based on the bramble's distance travelled.*

**Active:** **Maokai** summons a colossal wall of five thorny brambles that slowly advances in the target direction, accelerating over time, each stopping when they collide with an enemy champion. Hitting at least one enemy champion grants **Maokai** **bonus movement speed** decaying over 2 seconds. Each bramble deals magic damage to enemies hit and root them for type=distance traveled seconds.

| Attribute | Value |
|-----------|-------|
| **Range** | 3000 units |
| **Cooldown** | $130-90$ seconds |
| **Cast Time** | $0.5$ seconds |
| **Cost** | 100 Mana |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Speed** | 100–750@0–3 (@=seconds active) units/second |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |
| **Projectile** | True |

**Scaling:**
- **Bonus Movement Speed:** $40-60$%
- **Magic Damage:** $150-300$ (+ 75% AP)

**Notes:**

- The brambles are made up of 4 projectile each which trail 100 units behind one another. The front missile carries the visual effect. When colliding with a champion, all missiles of one bramble are destroyed simultaneously.
  - Missiles destroyed by other means such as Blade Whirl may not destroy their fellows from the same branch, resulting in one or more invisible missiles that may still collide. Effect at cast time start

---

## Patch History

### V14.22
- Stats
  - Base health increased to 665 from 635.
  - Movement speed increased to 335 from 330.
- *Sapling Toss*
  - Sapling base movement speed increased to 400 to 460 from 400 at all levels.
  - **Removed:*** Sapling movement speed is no longer increased by the movement speed gained via ''Maokai's' innate Boots movement speed (60 maximum bonus possible).
    - *Boots*: 425
    - *Berserker's Greaves*, *Ionian Boots of Lucidity*, *Mercury's Treads*, *Plated Steelcaps* and *Sorcerer's Shoes*: 445
    - *Boots of Swiftness*: 460

### V14.18
- *Bramble Smash*
  - Base damage reduced to $65-245$ from $65-265$.
- *Sapling Toss*
  - Cooldown increased to $18-14$ seconds from $16-12$.
  - Bush duration health ratio decreased to $1.5$% **bonus** health from $2.5$%.

### V14.5
- Stats
  - Base movement speed reduced to 330 from 335.
- *Bramble Smash*
  - Bonus monster damage increased to $120-160$ from $80-160$.
- *Twisted Advance*
  - Cooldown increased to $14-10$ seconds from $13-9$.

### V14.4
- Stats
  - Base armor reduced to 35 from 39.
- *Bramble Smash*
  - Base damage reduced to $65-265$ from $70-270$.
- *Nature's Grasp*
  - Root duration reduced to type=distance traveled seconds from type=distance traveled.

### V14.3
- Stats
  - Base mana regeneration reduced to 6 from $7.2$.
- *Sap Magic*
  - **Removed:*** Heal no longer has a base value of 4–9 to 34 for 6@1–17.
  - Heal health ratio increased to 4–12.8 from 4@1; 5.33@6; 6.66@9; 8@11; 9.33@13; 10.66@15; 12@17 (@=%).
- *Bramble Smash*
  - Cooldown reduced to $7-5$ seconds from $8-5$.
  - Mana cost reduced to 40 from 60.
- *Sapling Toss*
  - Cooldown changed to $16-12$ seconds from 14 at all ranks.
  - Mana cost changed to $60-80$ from $45-85$.
- *Nature's Grasp*
  - Cooldown changed to $130-90 3$ seconds from $120-100 3$.

### V13.15
- *Bramble Smash*
  - Bonus monster damage reduced to $80-160$ from $100-180$.

### V13.4
- *Bramble Smash*
  - Health ratio increased to $2-4$% of target's **maximum** health from $2-3$%.
- *Sapling Toss*
  - Cooldown increased to 14 seconds from 10.
  - Base damage reduced to $50-150$ from $55-155$.
  - Empowered base damage reduced to $100-300$ from $110-310$.
  - Damage AP ratio reduced to 25% AP from 35% AP.
  - Empowered AP ratio reduced to 50% AP from 70% AP.
  - Slow AP ratio reduced to 1% per 100 AP from 4% per 100 AP.
  - Slow health ratio increased to 1% per 100 **bonus** health from $0.9$% per **bonus** health.
- *Bramble Smash*
  - Bonus monster damage reduced to $100-180$ from $120-200$.
- *Sapling Toss*
  - Damage AP ratio reduced to 35% AP from 40% AP.
  - Damage health ratio reduced to 5% **bonus** health from 6% **bonus** health.
  - Empowered AP ratio reduced to 70% AP from 80% AP.
  - Empowered health ratio reduced to 10% **bonus** health from 12% **bonus** health.

### V12.23
- *Sap Magic*
  - Heal health ratio increased to 4@1; 5.33@6; 6.66@9; 8@11; 9.33@13; 10.66@15; 12@17 (@=%) **maximum** health from 4@1; 5@6; 6@9; 7@11; 8@13; 9@15; 10@17 (@=%).
- *Bramble Smash*
  - Base damage increased to $70-270$ from $65-245$.
  - Bonus monster damage increased to $120-200$ from $80-160$.
- *Sapling Toss*
  - Empowered slow health ratio increased to $0.9$% per 100 **bonus** health from $0.6$% per 100 **bonus** health.
  - Empowered slow AP ratio increased to 4% per 100 AP from 2% per 100 AP.

### V12.22
- *Nature's Grasp*
  - **Bug Fixes:** Smaller sized champions can no longer sometimes fit between the gaps of the missiles.

### V12.20
- *Sap Magic*
  - Heal health ratio reduced to 4@1; 5@6; 6@9; 7@11; 8@13; 9@15; 10@17 (@=%) **maximum** health from 4.5@1; 6@6; 7.5@9; 9@11; 10@13; 11@15; 12@17 (@=%).
- *Nature's Grasp*
  - Cooldown increased to $120-100 3$ seconds from $120-80 3$.

## Trivia

- Maokai was named after his concept artist, Maokai Xiao.
- In Chinese, Maokai is rendered as 茂凯 / Màokǎi.
- Maokai's dance references the Harlem Globetrotters.
- Tabs for **Jarvan IV** and **Lee Sin** can be seen during Maokai's Art Spotlight.

---
*This page was automatically generated from League of Legends Wiki data.*