# Jax

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
| **Champion** | Jax |
| **Title** | Grandmaster at Arms |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2009-02-21 |
| **Release Patch** | Alpha Week 2 |
| **Latest Changes** | V25.12 |
| **Roles** | Skirmisher |
| **Riot Positions** | Top, Jungle |
| **External Positions** | Top, Jungle |
| **Blue Essence** | 675 |
| **Riot Points** | 585 |
| **Difficulty** | 1 |
| **Hero Type** | Fighter |
| **Alt Type** | Assassin |
| **Adaptive Type** | Physical |
| **Damage** | 3 |
| **Toughness** | 2 |
| **Control** | 2 |
| **Mobility** | 2 |
| **Utility** | 1 |
| **Style** | 10 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $665.0$ | $+103.0$ |
| **Mana** | $339.0$ | $+52.0$ |
| **Health Regen** | $8.5$ | $+0.55$ |
| **Mana Regen** | $8.2$ | $+0.7$ |
| **Armor** | $36.0$ | $+4.2$ |
| **Magic Resist** | $32.0$ | $+2.05$ |
| **Attack Damage** | $68.0$ | $+4.25$ |
| **Attack Speed** | $0.638$ | |
| **Movement Speed** | $350.0$ | $+0.0$ |
| **Attack Range** | $125.0$ | $+0.0$ |
| **Base Attack Speed** | $0.638$ | |
| **Attack Speed Ratio** | $0.638$ | |
| **Bonus AS per Level** | $3.4\%$ | |
| **Acquisition Radius** | $400$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $120$ units | |
| **Selection Height** | $130$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $105.0\%$ |
| **Damage Taken** | $97.0\%$ |

#### URF

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $90.0\%$ |
| **Damage Taken** | $115.0\%$ |

#### Ultimate Spellbook

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $95.0\%$ |
| **Damage Taken** | $105.0\%$ |

## Abilities

### Passive: Relentless Assault

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Enemies, Self |

**INNATE:** **Jax**’s basic attacks generate a stack of *Relentless Assault* on-attack for $2.5$ seconds, refreshing on subsequent attacks and stacking up to 8 times. Stacks expire by one every $0.25$ seconds when the duration ends.

**RELENTLESS ASSAULT:** For each stack, **Jax** gains , up to a maximum of key=%.

**GRANDMASTER AT ANGLING:** While out-of-combat with champions and idle in the river for 10 seconds, **Jax** will occasionally catch a fish, granting him 1 gold and 1 ability power for 5 seconds. He catches a fish at an average rate of one every 15 seconds. He also has a 5% chance to catch a rare fish that grants 10 gold and 10 ability power for 5 seconds.

**Notes:**

- **Jax** will start fishing while idle in the river if he is in-combat with only monsters (e.g. any of the three Epic monsters).
- **Jax** catching a fish is a random event and will play a special animation.
- The total number of fish caught during the game is displayed when fishing as Grandmaster at Angling passive.

---

### Q: Leap Strike

| Attribute | Value |
|-----------|------:|
| **Cast Time** | False |
| **Target Range** | 700 units |
| **Cost** | 65 Mana |
| **Cooldown** | 8 / 7.5 / 7 / 6.5 / 6 seconds |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | physical |
| **Spell Shield** | True |
| **Spell Effects** | single |
| **Call For Help** | True |
| **Grounded** | True |
| **Knockdown** | True |

**ACTIVE:** **Jax** dashes to the target unit's location.

If the target is an enemy and they are in range upon arrival, **Jax** deals physical damage to them.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 65 / 105 / 145 / 185 / 225 (+ 100% **bonus** AD) |

***Jax** can cast any of his abilities during the dash.*

**Notes:**

- *Leap Strike* cannot be cast on structures.
- If the target is an enemy champion, **Jax** will be ordered to basic attack them after the dash ends.
- Spell shield will block the damage, including when empowered.

---

### W: Empower

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Cost** | 30 Mana |
| **Cooldown** | 7 / 6 / 5 / 4 / 3 seconds |
| **Cooldown Start** | post-effect |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | magic |
| **Spell Shield** | True |
| **Spell Effects** | spell |
| **Parry** | True |

**ACTIVE:** **Jax** empowers his next basic attack or *Leap Strike* against an enemy within 10 seconds to deal **bonus** magic damage, reduced to 50% against structures. If *Empower* is used on a basic attack, it will gain range and have an uncancellable windup.

| Attribute | Value |
|-----------|------:|
| **Bonus Magic Damage** | 50 / 85 / 120 / 155 / 190 (+ 60% AP) |

*Empower resets **Jax**’s basic attack timer.*

**Notes:**

- *Empower* is applied in a separate damage instance from **Jax**’s basic attacks.
  - This causes effects like Bone Plating to be applied twice.

---

### E: Counter Strike

| Attribute | Value |
|-----------|------:|
| **Cast Time** | None |
| **Effect Radius** | 375 units |
| **Cost** | 50 / 60 / 70 / 80 / 90 Mana |
| **Cooldown** | 17 / 15 / 13 / 11 / 9 seconds |
| **Cooldown Start** | post-effect |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | magic |
| **Spell Shield** | True |
| **Spell Effects** | aoe |

**ACTIVE:** **Jax** enters *Evasion* for 2 seconds: a defensive stance that causes him to dodge all incoming non-turret basic attacks and take 25% reduced damage from all area of effect abilities sourced from champions. 

 *Counter Strike* can be recast after 1 second, and does so automatically after the duration.

**RECAST:** **Jax** deals magic damage to nearby enemies, with the **total** damage increased by 20% for each attack dodged, up to a 100% increase, and stuns them for 1 second.

| Attribute | Value |
|-----------|------:|
| **Minimum Magic Damage** | 40 / 70 / 100 / 130 / 160 (+ 70% AP) (+ $3.5$% of target's **maximum** health) |
| **Maximum Magic Damage** | 80 / 140 / 200 / 260 / 320 (+ 140% AP) (+ 7% of target's **maximum** health) |

**Notes:**

- The initial cast and the manual recast count as ability activations for the purposes of on-cast effects such as Spellblade and triggering Force Pulse’s passive.
- *Counter Strike* will also dodge abilities that can trigger on-hit effects (Parrrley, Mystic Shot) which will count towards *Counter Strike*’s increased damage. There are exceptions of abilities that *Counter Strike* will not dodge but will dodge the damage from on-hit effects that they trigger (Alpha Strike, Piercing Darkness).

---

### R: Grandmaster-at-Arms

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Effect Radius** | 375 units |
| **Cost** | 100 mana |
| **Cooldown** | 110 / 105 / 100 / 95 / 90 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Auto |
| **Affects** | Enemies, Self |
| **Damage Type** | Magic |
| **Spell Shield** | Special |
| **Spell Effects** | Special |
| **Parry** | Special |

**PASSIVE:** **Jax**’s basic attacks generate a stack of *Grandmaster-at-Arms* on-hit for $2.5$ seconds, refreshing on subsequent hits and stacking up to 2 times. At 2 stacks, his next basic attack on-hit is empowered to have an uncancellable windup and consume all stacks to deal **bonus** magic damage, reduced to 50% against structures. While *Grandmaster-at-Arms* is active, the empowered attack triggers at 1 stack instead.

| Attribute | Value |
|-----------|------:|
| **Bonus Magic Damage** | 75 / 102.5 / 130 / 157.5 / 185 (+ 60% AP) |

**ACTIVE:** **Jax** swings his lantern around, dealing magic damage to nearby enemies. If this hits a champion, he gains **bonus** armor, increased for each champion hit beyond the first, and **bonus** magic resistance equal to 60% of that amount as well as 10% increased for 8 seconds.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 100 / 137.5 / 175 / 212.5 / 250 (+ 100% AP) |

| Attribute | Value |
|-----------|------:|
| **Bonus Armor** | 25 / 37.5 / 50 / 62.5 / 75 (+ 40% **bonus** AD) |
| **Bonus Magic Resistance** | 15 / 22.5 / 30 / 37.5 / 45 (+ 24% **bonus** AD) |

| Attribute | Value |
|-----------|------:|
| **Bonus Armor per Champion Hit** | 20 / 22.5 / 25 / 27.5 / 30 (+ 10% **bonus** AD) |
| **Bonus Magic Resistance per Champion Hit** | 12 / 13.5 / 15 / 16.5 / 18 (+ 6% **bonus** AD) |

***Jax** can move during Grandmaster-at-Arms' cast time.*

**Notes:**

- The attack bonus deals proc damage. The swing deals area damage.
- The bonus resistances are represented by a glowing aura around **Jax** that grows with each enemy champion hit beyond the first.
- The **bonus** magic damage can be dodged and blocked but it cannot miss.
- Spell shield will only block the bonus damage from the third attack if it is empowered.
- The empowered attack will trigger but not be consumed nor apply its effects against wards.
  - Stacks will still be generated when attacking them.
- Effect at cast time end

---

## Patch History

### V25.12
- Relentless Assault
  - Bonus attack speed per stack increased to key=% from key=%.
    - Maximum bonus attack speed increased to key=% from key=%.

### V25.11
- Grandmaster-at-Arms
  - **UNDOCUMENTED / NEW EFFECT:** Empowered basic attack now has an uncancellable windup.

### V25.08
- Empower
  - **Bug Fixes:** No longer sometimes fails to apply its damage against structures.

### V25.07
- Grandmaster-at-Arms
  - **Bug Fixes:** After spending a second or further skill point in the ability, no longer causes the attack animations associated with the ability to stop playing.

### V25.S1.3
- Stats
  - Health growth increased to 103 from 100.
  - Base mana regeneration increased to $8.2$ from $7.6$.
- Grandmaster-at-Arms
  - On-hit base damage increased to 75 / 130 / 185 from 70 / 120 / 170.

### V14.22
- Grandmaster-at-Arms
  - Cooldown increased to 110 / 100 / 90 seconds from 100 / 90 / 80.
  - Base damage reduced to 100 / 175 / 250 from 150 / 250 / 350.

### V14.18
- Grandmaster-at-Arms
  - On-hit base damage increased to 70 / 120 / 170 from 60 / 110 / 160.
  - Bonus armor increased to 25 / 50 / 75 from 15 / 40 / 65.
    - Bonus magic resistance increased to 15 / 30 / 45 from 9 / 24 / 39.
  - Bonus armor per subsequent target increased to 20 / 25 / 30 from 15 / 20 / 25.
    - Bonus magic resistance per subsequent target increased to 12 / 15 / 18 from 9 / 12 / 15.

### V14.14
- Empower
  - **Bug Fixes:** Empowered attack no longer sometimes fails to apply against towers.
- Counter Strike
  - Cooldown increased to 17 / 15 / 13 / 11 / 9 seconds from 15 / 13.5 / 12 / 10.5 / 9.
- Grandmaster-at-Arms
  - **Bug Fixes:** Restored empowered attack SFX when attacking structures.

### V14.10
- Grandmaster-at-Arms
  - **Bug Fixes:** Cast indicator now matches with the correct range of 375.
  - **Bug Fixes:** Ability power and attack damage from conversions and adaptive force is now properly included in the ratio calculations.

### V14.9
- General
  - Adjusted splash artwork for Jax.

## Trivia

- Jax's eyes are blue.
- Jax was likely inspired by , "The Weapons Master" of the *Sword of Shannara Trilogy*. He was adept with any weapon and was undefeated in combat, though he often wielded a sword and cudgel.
- In the V1.0.0.115 (April Fools' Day) patch, Wriggle's Lantern was given the following joke passive, referencing Jax’s usage of a lamp post (which is functionally similar to a lantern) as a weapon:
  - New **UNIQUE PASSIVE**: Taunts nearby Jax (both enemy and allied).
- In the now-removed official League of Legends forums, the original icon of Relentless Assault was used to represent the "Off Topic Discussion" section.
- The old icon art of Grandmaster's Might features a stunned soldier who was directly mirrored from the original icon for Time Bomb.
- Grandmaster-at-Arms, The Darkin Blade, Night Hunter, and The Relentless Storm are the only abilities in *League of Legends* that have the same name as their champion's title.
- Jax's Series 2 Eternals make the following references:
  - *	Active: Jax enters Evasion...* is a reference to his ability description for Counter Strike.

---
*This page was automatically generated from League of Legends Wiki data.*