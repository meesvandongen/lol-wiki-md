# Rell

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
| **Champion** | Rell |
| **Title** | the Iron Maiden |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2020-12-10 |
| **Release Patch** | V10.25 |
| **Latest Changes** | V25.17 |
| **Roles** | Vanguard |
| **Riot Positions** | Support |
| **External Positions** | Support |
| **Blue Essence** | 2400 |
| **Riot Points** | 880 |
| **Difficulty** | 2 |
| **Hero Type** | Tank |
| **Alt Type** | Support |
| **Adaptive Type** | Magic |
| **Damage** | 1 |
| **Toughness** | 3 |
| **Control** | 3 |
| **Mobility** | 2 |
| **Utility** | 2 |
| **Style** | 85 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $620.0$ | $+104.0$ |
| **Mana** | $320.0$ | $+40.0$ |
| **Health Regen** | $7.5$ | $+0.85$ |
| **Mana Regen** | $7.0$ | $+0.7$ |
| **Armor** | $30.0$ | $+4.0$ |
| **Magic Resist** | $28.0$ | $+1.8$ |
| **Attack Damage** | $55.0$ | $+3.0$ |
| **Attack Speed** | $0.625$ | |
| **Movement Speed** | $315.0$ | $+0.0$ |
| **Attack Range** | $175.0$ | $+0.0$ |
| **Base Attack Speed** | $0.625$ | |
| **Attack Speed Ratio** | $0.625$ | |
| **Bonus AS per Level** | $2.0\%$ | |
| **Attack Windup** | $21.0\%$ | |
| **Acquisition Radius** | $400$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $120$ units | |
| **Selection Height** | $150$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $95.0\%$ |
| **Damage Taken** | $100.0\%$ |

#### Ultimate Spellbook

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $90.0\%$ |
| **Damage Taken** | $105.0\%$ |

## Abilities

### Passive: Break the Mold

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Self, Enemies |
| **Damage Type** | magic |
| **Spell Shield** | False |
| **Spell Effects** | proc |
| **Parry** | True |

**INNATE:** **Rell**’s basic attacks deal **bonus** magic damage on-hit equal to the sum of 5% of her **total** armor and 5% of her **total** magic resistance.

**INNATE:** **Rell**’s basic attacks and abilities against non-minions apply a stack of *Break The Mold* for 5 seconds, refreshing on subsequent hits and stacking up to 5 times. Each stack reduces the target's (ar) armor and (mr) magic resistance by 3% for a maximum of 15% reduction.

**Rell** gains **bonus** armor and **bonus** magic resistance equal to the sum resistances reduced from all afflicted enemies.

*Break the Mold* will reduce the target's armor and magic resistance by a minimum of 1.5 to 3.

**Notes:**

- The attacks do not affect structures.
- The first stack of *Break The Mold* that is applied to a target will snapshot the targets current (ar) armor and (mr) magic resistance. All subsequent stacks will reduce the targets resistances based on the snapshotted amount, even if the target's total resistances change. This snapshot lasts until the debuff expires.
  - For example, applying a stack of *Break The Mold* to a champion with 100 (ar) armor and (mr) magic resistance will reduce their resistance by 3 and grant **Rell** the lost stats. If their resistances are then boosted to 500 before another stack is applied, the subsequent stack will only reduce their resistance by a further 3 despite 3% of 500 being 15.

---

### Q: Shattering Strike

| Attribute | Value |
|-----------|------:|
| **Range** | 520 (Forward Range) / -220 (Backward Range) units |
| **Cast Time** | $0.4$ seconds |
| **Width** | 150 (Rectangle width) units |
| **Cost** | 50 mana |
| **Cooldown** | 11 / 10.5 / 10 / 9.5 / 9 seconds |
| **Queue Time** | $0.25$ seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | False |

**ACTIVE:** **Rell** thrusts her lance in the target direction, lunging forward 100 units and destroying the damage-mitigating shields of all enemies hit (excluding the shields of monsters) before dealing them magic damage and stunning them for $0.65$ seconds.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 60 / 100 / 140 / 180 / 220 (+ 60% AP) |

**Notes:**

- **Rell** is locked out of moving, attacking, and casting any other ability for $0.35$ seconds after *Shattering Strike* is cast.

---

### W: Ferromancy: Crash Down

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.625$ seconds |
| **Target Range** | 400 (Initial dash range) / 100 (Minimum landing range) units |
| **Collision Radius** | 200 (Sliding dash collision radius) units |
| **Effect Radius** | 180 units |
| **Cost** | 40 Mana |
| **Cooldown** | 10 seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Out of Range Behavior** | cast at max |
| **Grounded** | True |
| **Knockdown** | True |

**PASSIVE - MOUNTED ALACRITY:** While **Rell** is **MOUNTED**, she gains **bonus** movement speed.

| Attribute | Value |
|-----------|------:|
| **Bonus Movement Speed** | 20 / 25 / 30 / 35 / 40 |

**ACTIVE:** **Rell** becomes **DISMOUNTED** and leaps to the target location over the cast time, granting herself a shield that lasts until destroyed or casting *Ferromancy: Mount Up*. Upon arrival, she deals magic damage to nearby enemies, stuns them for $0.8$ seconds, and knocks them up for $0.4$ seconds. She will continue sliding forward another 320 units over $0.5$ seconds (Estimated), though not through terrain, affecting further enemies along her path.

| Attribute | Value |
|-----------|------:|
| **Shield Strength** | 20 / 40 / 60 / 80 / 100 (+ 11% **maximum** health) |

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 60 / 90 / 120 / 150 / 180 (+ 60% AP) |

While **Rell** is **DISMOUNTED**, she can cast *Ferromancy: Mount Up*. Upon completing a Recall channel or respawning, **Rell** will automatically revert to **MOUNTED** form without casting the ability and reset *Ferromancy: Mount Up’s* cooldown.

*This ability can be cast only while Rell is **MOUNTED**. **Rell** can cast Magnet Storm during the dash, and is not considered to be dismounted until after the leap ends.*

**Notes:**

- **Rell** slides between 250 and 350 units in testing, depending on how far the spell was targeted. It is not known where this is intended.
- If **Rell** dashes before a wall on the map, the slide will cover a shorter distance to the terrain over the same time, moving slower.
  - Interaction with player-made walls.
- The following table refers for interactions while **Rell** is dashing/in cast time:

---

### W: Ferromancy: Mount Up

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Cooldown** | 10 seconds |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spell |
| **Parry** | True |
| **Grounded** | False |
| **Knockdown** | Special |

**PASSIVE:** While **Rell** is **DISMOUNTED**, she gains 15% **bonus** armor, 15% **bonus** magic resistance, 20% **bonus** attack speed, and 75 **bonus** attack range.

**ACTIVE:** **Rell** becomes **MOUNTED**, gaining (ms) 30% **bonus** movement speed decaying over 2 seconds and empowering her next basic attack within $3.5$ seconds to have a $0.2$-second cast time, gain 100 **bonus** attack range and cause her to charge at the target's location, during which she also gains 40% **bonus** attack speed. Upon arrival or collision, she deals **bonus** magic damage, stuns the target for $0.6$ seconds, and flings them 150 units over herself, though not through terrain, over $0.4$ seconds.

| Attribute | Value |
|-----------|------:|
| **Bonus Magic Damage** | 10 / 25 / 40 / 55 / 70 (+ 40% AP) |

While **Rell** is **MOUNTED**, she can cast *Ferromancy: Crash Down*.

*Ferromancy: Mount Up resets **Rell**’s basic attack timer. This ability can be cast only while **Rell** is **DISMOUNTED**.*

**Notes:**

- **Rell** will always respawn and start the game as mounted.
- The basic attack reset is not considered one for Hail of Blades.
- **Rell** can use the empowered attack even while grounded or rooted.
- The dash can be knocked down but the attack's effects will still be applied.
- The movement speed reduction is a negative bonus, not a slow, and is thus not reduced by slow resist.
- Displacement immunity will not resist the application of the stun.
- If the target becomes untargetable, dies, or is too far away or no longer in sight during the empowered attack's cast time, it is cancelled but not consumed.

---

### E: Full Tilt

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 1200 units |
| **Effect Radius** | 300 (Explosion radius) / 2200 (Empowered Ally detection range, estimated) / 1600 (Enemy Champion detection range, estimated) units |
| **Cost** | 40 Mana |
| **Cooldown** | 14 / 13 / 12 / 11 / 10 seconds |
| **Queue Time** | $0.5$ seconds |
| **Targeting** | Unit / Auto |
| **Affects** | Self, Allies, Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Out of Range Behavior** | If targeting an ally, walk in range of the target unit to cast |

**ACTIVE:** **Rell** powers up herself and the target allied champion for 3 seconds, both gaining 10% **bonus** movement speed, increased to 25% while facing the empowered ally or a visible enemy champion.

Additionally, **Rell**’s next basic attack or *Shattering Strike* within 5 seconds creates an explosion around the target (See Notes) that deals **bonus** magic damage. The damage based on the target's health is capped at 150 to 300 against monsters and structures.

| Attribute | Value |
|-----------|------:|
| **Bonus Magic Damage** | 5 / 5.5 / 6 / 6.5 / 7% (+ 3% per 100 AP) of target's **maximum** health |

If cast without a valid target, or self-cast, *Full Tilt* will automatically target the closest allied champion in range.

**Notes:**

- If *Shattering Strike* hits more than one enemy, the *Full Tilt* explosion will be centered around the unit with the lowest Spawn ID (among enemy champions, this would also be the enemy first pick in Blind Pick mode).
  - "Spawn ID" is an unofficial abbreviation to describe the spawn order for all units at the beginning of games.

---

### R: Magnet Storm

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Effect Radius** | 450 (Initial knockup radius) / er 375 (Continous effect radius, center to edge) units |
| **Inner Radius** | 225 (Pulls target into at least this radius, continuous kinetics does not affect targets already in this radius) units |
| **Speed** | 300 (Kinematic attraction speed, in units per second) units/second |
| **Cost** | 100 Mana |
| **Cooldown** | 120 / 110 / 100 / 90 / 80 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | Special |
| **Spell Effects** | aoedot |

**ACTIVE:** **Rell** erupts with magnetic fury, pulling nearby enemies inward (to a 225 units around Rell, but minimum 100 pull distance) and creating a gravitational field around her for the next 2 seconds that deals magic damage every $0.25$ seconds to nearby enemies and drags them towards her.

| Attribute | Value |
|-----------|------:|
| **Magic Damage Per Tick** | 15 / 20 / 25 / 30 / 35 (+ $13.75$% AP) |
| **Total Magic Damage** | 120 / 160 / 200 / 240 / 280 (+ 110% AP) |

**Notes:**

- *Magnet Storm* will **not** drag units that are:
  - Dashing.
  - Being displaced (e.g. by airborne effects)
  - Attached.
  - Displacement immune.
  - Immune to crowd control.
- Spell shield will block the initial pull but not the dragging effect.
- *Magnet Storm* cannot be cast again while it is active.

---

## Patch History

### V25.17
- Ferromancy: Crash Down
  - **Bug Fixes:** Tooltip now notes the proper shield health ratio in accordance with the

### V25.16
- Ferromancy: Crash Down
  - Shield health ratio reduced to 11% **maximum** health from 13%.

### V25.15
- Ferromancy: Crash Down
  - Base shield reduced to 20 / 40 / 60 / 80 / 100 from 25 / 50 / 75 / 100 / 125.

### V25.S1.3
- Stats
  - Base health reduced to 620 from 640.
  - Armor growth reduced to 4 from $4.3$.

### V14.24
- Stats
  - Base health increased to 640 from 610.
- Break the Mold
  - Resistances reduction per stack increased to 3% from 2%.
  - Minimum resistances reduction increased to 1.5 to 3 from 1 to 2.
- Ferromancy: Crash Down
  - Base shield increased to 25 / 50 / 75 / 100 / 125 from 15 / 40 / 65 / 90 / 115.
  - Shield health ratio increased to 13% **maximum** health from 12%.

### V14.23
- Stats
  - Base mana reduced to 320 from 350.
  - Mana growth reduced to 40 from 45.
  - Base mana regeneration increased to 7 from 6.
  - Mana regeneration growth increased to $0.7$ from $0.35$.
  - Base armor reduced to 30 from 36.
  - Armor growth increased to $4.3$ from $4.2$.
  - Base magic resistance reduced to 28 from 30.
  - Magic resistance growth reduced to $1.8$ from $2.05$.
  - Attack speed growth increased to 2% from $1.5$%.
  - Attack windup reduced to 18.75% from 21%.
  - Attack windup time is no longer reduced by 100-(0.42 to 0.39)/0.42×100.
  - Attack windup modifier increased to 1 (default) from $0.4$.
  - Base movement speed reduced to 315 from 330.
- Break the Mold
  - Minimum resistance steal increased to 1 to 2 from 0.8 to 2.
  - **New Effect:** Now deals on-hit magic damage equal to (armor) 5% **total** armor and (mr) 5% **total** magic resistance.
- Shattering Strike
  - Stun duration reduced to $0.65$ seconds from $0.75$.
- Ferromancy: Crash Down
  - Cooldown reduced to 10 seconds from 11.
  - Knock up duration reduced to $0.4$ seconds from 1.
  - **New Effect:** Now also stuns for $0.8$ seconds at the start of the displacement.
  - ***NEW EFFECT - MOUNTED ALACRITY:*** While **Rell** is mounted, she passively gains 20 / 25 / 30 / 35 / 40 **bonus** movement speed.
  - Base shield increased to 15 / 40 / 65 / 90 / 115 from 15 / 40 / 65 / 90 / 110.
- Ferromancy: Mount Up
  - Cooldown reduced to 10 seconds from 11.
  - Stun duration reduced to $0.6$ seconds from 1.
  - **Removed:*** **Rell**’s movement speed is no longer reduced by 10% while dismounted.
  - Bonus resistances increased to 15% from 12%.
  - Bonus attack speed reduced to 20% from 30%.
- Full Tilt
  - Cooldown reduced to 14 / 13 / 12 / 11 / 10 seconds from 15 at all ranks.
  - ***REMOVED - MOUNTED ALACRITY:*** **Rell** no longer passively gains 5 to 20 for 6 / 24 to 40 for 5 / 45 / 50 **bonus** movement speed while **MOUNTED**, reduced by 50% while in combat.
    - *Effect name moved to Ferromancy: Crash Down.*
  - **Removed:*** Active movement speed no longer ramps up linearly from 75% of its maximum value over the first 2 seconds.
  - Base movement speed reduced to 10% from 12 / 13 / 14 / 15 / 16%.
  - Rally movement speed modifier increased to 250% of base from 200%.
    - Rally movement speed changed to 25% from 24 / 26 / 28 / 30 / 32%.
  - **Removed:*** No longer has a base damage of 25 / 35 / 45 / 55 / 65.
  - **Removed:*** Damage no longer scales with 50% AP.
  - Health ratio increased to 5 / 5.5 / 6 / 6.5 / 7% of target's **maximum** health from 3% at all ranks.
  - **New Effect:** Health ratio now scales with 3% per 100 AP.
  - Monster damage cap increased to 150 to 300 from 150 at all levels.
  - **New Effect:** Damage based on the target's health now applies against structures.
  - **New Effect:** Damage based on the target's health is now capped at 150 to 300 against structures.

### V14.18
- Full Tilt
  - Bonus movement speed reduced to 12 / 13 / 14 / 15 / 16% from 12 / 14 / 16 / 18 / 20%.
    - Increased bonus movement speed reduced to 24 / 26 / 28 / 30 / 32% from 24 / 28 / 32 / 36 / 40%.

### V14.16
- Full Tilt
  - **Bug Fixes:** Bonus damage no longer improperly counts towards applying the resistances reduction from Break the Mold if applied by Shattering Strike.

### V14.14
- Full Tilt
  - **Bug Fixes:** Can no longer be used on allies without having vision of them.

### V14.7
- Stats
  - Magic resistance growth increased to 2.05 from 1.85.
- Shattering Strike
  - **Removed:*** No longer deals 170 / 245 / 320 / 395 / 470 **bonus** damage against monsters.
- Ferromancy: Crash Down
  - **Removed:*** No longer deals 125 / 150 / 175 / 200 / 225 **bonus** damage against monsters.
- Ferromancy: Mount Up
  - **Removed:*** No longer deals 55 / 110 / 165 / 220 / 275 **bonus** damage against monsters.
- Full Tilt
  - **Removed:*** No longer deals 120 / 165 / 210 / 255 / 300 **bonus** damage against monsters.

## Trivia

- Rell's lance is featured in the promotional art for Preseason 2021 Mythic Forge as a reveal teaser.

---
*This page was automatically generated from League of Legends Wiki data.*