# Miss_Fortune

## Table of Contents
- [Basic Information](#basic-information)
- [Abilities](#abilities)
- [Patch History](#patch-history)
- [Trivia](#trivia)

## Basic Information

| Attribute | Value |
|-----------|------:|
| **Champion** | Miss Fortune |

## Abilities

### Passive: Love Tap

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Effects** | Proc |
| **Parry** | True |

**INNATE:** **Miss Fortune**’s basic attacks are empowered to apply a mark that expires upon attacking a new enemy. If the enemy was unmarked, this also deals 50%–100%@1–13 AD **bonus** physical damage, halved to 50/2 to 100/2 for 6 AD against minions.

**Notes:**

- With Runaan's Hurricane, *Love Tap* only applies to the primary target.
- The bonus damage applies life steal.
- The empowered attack will not trigger against buildings.

---

### Q: Double Up

| Attribute | Value |
|-----------|------:|
| **Cast Time** | Basic Attack Timer |
| **Target Range** | er Miss Fortune's attack range |
| **Effect Radius** | 500 units |
| **Angle** | 160° |
| **Speed** | 1400 units/second |
| **Cost** | 40 Mana |
| **Cooldown** | 7 / 6 / 5 / 4 / 3 seconds |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | Special |
| **Spell Effects** | Special |
| **Projectile** | True |
| **Parry** | True |
| **Call For Help** | True |

**ACTIVE:** **Miss Fortune** fires a shot at the target enemy that deals physical damage, triggers on-attack effects, and bounces to hit another enemy behind them, applying on-hit effects to both enemies hit.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 20 / 45 / 70 / 95 / 120 (+ 100% AD) (+ 35% AP) |

*Double Up*’s bounce is affected by critical strike modifiers. If *Double Up* kills the primary target, the bounce will critically strike.

*The bounce prioritizes units directly behind the primary target. A target does not have to be visible to be hit by the bounce.*

**Notes:**

- *Double Up* deals basic damage to both targets, but also triggers spell effects by also being tagged as spell damage.
- Spell shields can be used by either target. If it is on the primary target, the spell shield will not prevent the shot from bouncing.
- Neutral units count as valid targets to bounce.
- The bounce follows a priority order on targets behind in a certain angle:
  1. 500 units in 20º.
  1. 500 units in 40º.
  1. 500 units in 110º.
  1. 150 units (Estimated) in 160º.
- *Double Up*’s cast range adjusts based on **Miss Fortune**’s attack range, which can be increased by items such as Rapid Firecannon.
- The damage of a critically striking shot is 35 / 70 / 105 / 140 / 175 (+ 175% AD) (+ 61.25% AP) physical damage, increased to 42 / 84 / 126 / 168 / 210 (+ 210% AD) (+ 73.5% AP) physical damage by Infinity Edge bonus critical damage.
- If there is no secondary target, the shot will not bounce and the dud will instead fall to the ground.
  - The dud lands on the ground 176 units behind the primary target and has a missile speed of 400 on its way there.

---

### W: Strut

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Cost** | 45 Mana |
| **Cooldown** | 12 seconds |
| **Targeting** | Auto |
| **Affects** | Self |

**PASSIVE:** **Miss Fortune** gains (ms) **bonus** movement speed after 4 seconds without taking non-persistent damage. This bonus is increased after another 3 seconds, and is granted instantly whenever *Strut* is cast or upon respawning.

| Attribute | Value |
|-----------|------:|
| **Bonus Movement Speed** | 30 / 35 / 40 / 45 / 50 |

| Attribute | Value |
|-----------|------:|
| **Increased Bonus Movement Speed** | 60 / 70 / 80 / 90 / 100 |

**ACTIVE:** **Miss Fortune** gains (as) **bonus** attack speed for 4 seconds.

| Attribute | Value |
|-----------|------:|
| **Bonus Attack Speed** | 40 / 55 / 70 / 85 / 100% |

Marking a new target with *Love Tap* reduces *Strut*’s **current** cooldown by 2 seconds (Affected by ability haste).

**Notes:**

- No additional notes.

---

### E: Make It Rain

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 1000 units |
| **Effect Radius** | 200 units |
| **Cost** | 80 Mana |
| **Cooldown** | 18 / 17 / 16 / 15 / 14 seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | False |
| **Spell Effects** | aoedot |
| **Out of Range Behavior** | cast at max |

**ACTIVE:** **Miss Fortune** casts a storm of bullets at the target location for 2 seconds, granting sight of the area, dealing magic damage every $0.25$ seconds to enemies within, and slowing them by 40% (+ 6% per 100 AP).

| Attribute | Value |
|-----------|------:|
| **Magic Damage Per Tick** | 8.75 / 12.5 / 16.25 / 20 / 23.75 (+ 15% AP) |
| **Total Magic Damage** | 70 / 100 / 130 / 160 / 190 (+ 120% AP) |

**Notes:**

- *Make It Rain*’s slow cannot be cleansed.

---

### R: Bullet Time

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 1450 (Range of each bullet stream) units |
| **Angle** | 30° |
| **Width** | 40 (Width of each bullet stream) units |
| **Speed** | 2000 units/second |
| **Cost** | 100 Mana |
| **Cooldown** | 120 / 115 / 110 / 105 / 100 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | False |
| **Spell Effects** | aoedot |
| **Projectile** | True |
| **Silence** | True |

**ACTIVE:** **Miss Fortune** channels for up to 3 seconds, firing a number of waves of bullets in the target direction. Each wave is in a spread of 6 projectiles that deals 75% AD (+ 25% AP) physical damage to enemies hit.

| Attribute | Value |
|-----------|------:|
| **Total Waves** | 14 / 15 / 16 / 17 / 18 |
| **Maximum Total Physical Damage** | 1050 / 1125 / 1200 / 1275 / 1350% AD (+ 350 / 375 / 400 / 425 / 450% AP) | | Attribute | Value |
|-----------|------:|
| **Wave Interval Time** | 2.85/(14 to 18) seconds |

Each of the waves can critically strike for damage.

**Notes:**

- The damage dealt by each wave of *Bullet Time* is calculated when the wave reaches the target.
- **Miss Fortune** reveals herself for $4.5$ seconds if there is an enemy champion within the area.
- Targets cannot be damaged by more than one projectile per wave.
- The bullet streams that each fire 1 projectile per wave are spead by 6° between one another, aiming towards the angles +-3/9/15° from **Miss Fortune**’s facing direction.
- *Bullet time* picks 6 locations on the ground 500 units from **Miss Fortune** (with the aforementioned angles) and fires a bullet stream towards each.
  - If **Miss Fortune** is moved to a new location, these locations **will** update (since patch V13.3); the cone will fire towards her new facing direction.
- *Bullet time* fires the first wave at , and the last at . Times between waves are equally spread between these values.
  - **Miss Fortune** may cancel the last ~$0.1$ seconds of channel time at no loss of effect.
- The following table refers for interactions while **Miss Fortune** is channeling:

---

## Patch History

### V25.17
- Stats
  - Base attack damage increased to 55 from 53.

### V25.S1.3
- Stats
  - Base armor reduced to 25 from 28.
  - Armor growth reduced to 4 from $4.2$.

### V14.24
- Make It Rain
  - **Bug Fixes:** No longer reveals the user if cast while the cursor is near an enemy champion.

### V14.23
- Double Up
  - Mana cost reduced to 40 at all ranks from 43 / 46 / 49 / 52 / 55.

### V14.19
- Bullet Time
  - **Bug Fixes:** No longer causes a network error if 5 allies cast this ability at the same time.

### V14.18
- Stats
  - Base attack damage reduced to 53 from 55.

### V14.17
- General
  - Updated ability visual effects.
  - The following skins are affected: Miss Fortune, Miss Fortune, Miss Fortune, Miss Fortune, Miss Fortune, Miss Fortune, Miss Fortune, Miss Fortune, Miss Fortune.

### V14.13
- Stats
  - Base attack damage increased to 55 from 52.
- Strut
  - Bonus movement speed increased to 30 / 35 / 40 / 45 / 50 from 25 at all ranks.
  - Increased bonus movement speed increased to 60 / 70 / 80 / 90 / 100 from 55 / 65 / 75 / 85 / 95.
  - Out-of-combat timer reduced to 4 seconds from 5.
  - Increased movement speed delay reduced to 3 seconds from 5.

### V14.7
- Love Tap
  - **Bug Fixes:** Now properly grants a Conqueror stack when a basic attack applies the mark.

### V14.5
- Double Up
  - **Bug Fixes:** No longer improperly cancels her current attack windup upon hitting the primary target if there was no secondary target to bounce towards.
- Miss Fortune
  - **Bug Fixes:** Right arm can no longer twist unnaturally during her Joke (Ctrl+1).

## Trivia

- Miss Fortune's name is a play on the word "misfortune".
- Miss Fortune's twin pistols (Shock' and 'Awe') are named after Shock and Awe.
  - They can be seen in the game's Mac version launch trailer, together with a poster of Miss Fortune herself.
- Bullet Time is named after Bullet Time and references Max Payne (series).
- Miss Fortune is the third champion to have her price reduced twice (the others being Garen and Warwick) and the second to include a joke animation on release (the other being Kog'Maw).
- Miss Fortune is one of the first champions to have 12 skins.
  - She and Ezreal got their twelfth skin at the same time. The skins are in the Star Guardian Pajama skin set.

---
*This page was automatically generated from League of Legends Wiki data.*