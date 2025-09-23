# Ezreal

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
| **Champion** | Ezreal |
| **Title** | the Prodigal Explorer |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2010-03-16 |
| **Release Patch** | V1.0.0.79 |
| **Latest Changes** | V25.06 |
| **Roles** | Marksman |
| **Riot Positions** | Bottom |
| **External Positions** | Bottom |
| **Blue Essence** | 675 |
| **Riot Points** | 585 |
| **Difficulty** | 2 |
| **Hero Type** | Marksman |
| **Alt Type** | Mage |
| **Adaptive Type** | Physical |
| **Damage** | 3 |
| **Toughness** | 1 |
| **Control** | 1 |
| **Mobility** | 3 |
| **Utility** | 1 |
| **Style** | 45 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $600.0$ | $+102.0$ |
| **Mana** | $375.0$ | $+70.0$ |
| **Health Regen** | $4.0$ | $+0.65$ |
| **Mana Regen** | $8.5$ | $+1.0$ |
| **Armor** | $24.0$ | $+4.2$ |
| **Magic Resist** | $30.0$ | $+1.3$ |
| **Attack Damage** | $60.0$ | $+2.75$ |
| **Attack Speed** | $0.625$ | |
| **Movement Speed** | $325.0$ | $+0.0$ |
| **Attack Range** | $550.0$ | $+0.0$ |
| **Base Attack Speed** | $0.625$ | |
| **Attack Speed Ratio** | $0.625$ | |
| **Bonus AS per Level** | $2.5\%$ | |
| **Missile Speed** | $2000$ units/second | |
| **Acquisition Radius** | $550$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $115$ units | |
| **Selection Height** | $170$ units | |
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
| **Damage Dealt** | $95.0\%$ |

## Abilities

### Passive: Rising Spell Force

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Self |

**INNATE:** **Ezreal** generates a stack of *Rising Spell Force* for each enemy hit by his abilities, lasting for 6 seconds, refreshing on subsequent hits, and stacking up to 5 times.

**RISING SPELL FORCE:** For each stack, **Ezreal** gains as, up to a maximum of 50%.

**Notes:**

- *Rising Spell Force* will stack even if the abilities hit were blocked by spell shields.

---

### Q: Mystic Shot

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 1200 units |
| **Width** | 120 units |
| **Speed** | 2000 units/second |
| **Cost** | 28 / 31 / 34 / 37 / 40 Mana |
| **Cooldown** | 5.5 / 5.25 / 5 / 4.75 / 4.5 seconds |
| **Queue Time** | $0.05$ seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | Special |
| **Spell Effects** | Special |
| **Projectile** | True |
| **Parry** | True |
| **Call For Help** | True |

**ACTIVE:** **Ezreal** fires a bolt of energy in the target direction that deals physical damage to the first enemy hit, applying on-hit effects and triggering on-attack effects.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 20 / 45 / 70 / 95 / 120 (+ 130% AD) (+ 15% AP) |

If *Mystic Shot* successfully hits an enemy, the **current** cooldowns of **Ezreal**’s abilities, including *Mystic Shot*’s, are reduced by $1.5$ seconds.

**Notes:**

- *Mystic Shot* deals basic damage, but also triggers spell effects by also being tagged as spell damage.
- *Mystic Shot* benefits from life steal, omnivamp, and physical vamp, but not spell vamp.
- Even if the ability is blocked by spell shield it will still trigger the cooldown reduction. Effect at cast time end
- *Mystic Shot* will be buffered and cast when the cooldown ends if the player attempts to cast it within $0.05$ seconds of the cooldown ending.

---

### W: Essence Flux

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 1200 units |
| **Width** | 160 units |
| **Speed** | 1700 units/second |
| **Cost** | 50 Mana |
| **Cooldown** | 8 seconds |
| **Queue Time** | $0.5$ seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | Special |
| **Spell Effects** | spell |
| **Projectile** | True |
| **Parry** | Special |

**ACTIVE:** **Ezreal** fires an orb in the target direction that marks the first enemy champion, epic monster, or structure hit for 4 seconds.

His next basic attack or ability against the target detonates the mark to deal them **bonus** magic damage.

| Attribute | Value |
|-----------|------:|
| **Bonus Magic Damage** | 80 / 135 / 190 / 245 / 300 (+ 100% **bonus** AD) (+ 70 / 75 / 80 / 85 / 90% AP) |

If the mark was detonated with an ability, **Ezreal** restores mana plus the mana cost of that ability.

**Notes:**

- The application of *Essence Flux* deals 0 proc damage.
  - This triggers in-combat effects such as drawing turret aggro and drawing monster aggression.
  - It also triggers Sudden Impact and applies Elixir of Sorcery.
  - It does not trigger Cheap Shot, however, as proc damage doesn't trigger Cheap Shot.
- Block and Dodge prevents the mark from being triggered by a basic attack.
- Spell shield prevents the mark from being triggered by an ability. If the mark is triggered by a basic attack, spell shield will prevent *Essence Flux*’s damage. Effect at cast time end

---

### E: Arcane Shift

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 475 (Maximum blink range) units |
| **Effect Radius** | 750 (Homing missile range) units |
| **Speed** | 2000 (Homing missile speed) units/second |
| **Cost** | 70 Mana |
| **Cooldown** | 26 / 23 / 20 / 17 / 14 seconds |
| **Queue Time** | $0.5$ seconds |
| **Targeting** | Location |
| **Affects** | Enemies, Self |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spell |
| **Projectile** | True |
| **Out of Range Behavior** | cast at max |
| **Call For Help** | True |
| **Grounded** | True |

**ACTIVE:** **Ezreal** blinks from his current location to up to 475 units towards the target location, then fires a homing bolt towards the nearest enemy that deals magic damage and reveals them for 1 second.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 80 / 130 / 180 / 230 / 280 (+ 50% **bonus** AD) (+ 75% AP) |

*Arcane Shift* prioritizes firing at the nearest enemy marked by *Essence Flux*.

*The target does not have to be visible to be hit by this ability.*

**Notes:**

- If **Ezreal** is moved during the cast time, his range to blink will update accordingly. This does not exceed the maximum target range.
- The target is revealed as soon as **Ezreal** fires the missile at them.

---

### R: Trueshot Barrage

| Attribute | Value |
|-----------|------:|
| **Cast Time** | 1 seconds |
| **Target Range** | Global |
| **Width** | 320 (missile width) units |
| **Speed** | 2000 units/second |
| **Cost** | 100 Mana |
| **Cooldown** | 120 / 112.5 / 105 / 97.5 / 90 seconds |
| **Queue Time** | $0.5$ seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | True |

**ACTIVE:** **Ezreal** fires a powerful arc of energy in the target direction that briefly grants sight of its surroundings and deals magic damage to enemies hit.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 350 / 450 / 550 / 650 / 750 (+ 100% **bonus** AD) (+ 90% AP) |

Minions and non-epic monsters take 50% reduced damage.

| Attribute | Value |
|-----------|------:|
| **Reduced Damage** | 175 / 225 / 275 / 325 / 375 (+ 50% **bonus** AD) (+ 45% AP) |

**Notes:**

- *Trueshot Barrage*’s projectile has an icon on the mini-map while it is in flight. It can be seen by only **Ezreal** and his allies. Effect at cast time start

---

## Patch History

### V25.06
- Stats
  - Armor growth reduced to $4.2$ from $4.7$.

### V14.17
- Stats
  - Base attack damage reduced to 60 from 62.

### V14.15
- Mystic Shot
  - AD ratio reduced to 130% AD from 140% AD.

### V14.12
- Stats
  - Attack damage growth increased to $2.75$ from $2.5$.
- Mystic Shot
  - AD ratio increased to 140% AD from 130% AD.
- Trueshot Barrage
  - Base damage increased to 350 / 550 / 750 from 325 / 500 / 675.

### V14.3
- Mystic Shot
  - AD ratio reduced to 130% AD from 135% AD.
- Trueshot Barrage
  - Base damage reduced to 325 / 500 / 675 from 350 / 525 / 700.
  - Bonus AD ratio reduced to 100% **bonus** AD from 120%.

### V14.2
- Mystic Shot
  - AD ratio increased to 135% AD from 130% AD.
- Essence Flux
  - Bonus AD ratio increased to 100% **bonus** AD from 60%.
- Trueshot Barrage
  - Base damage increased to 350 / 525 / 700 from 350 / 500 / 650.
  - Bonus AD ratio increased to 120% **bonus** AD from 100%.

### V14.1#January 12th Hotfix|V14.1
- Essence Flux
  - Cooldown reduced to 8 seconds from 12.
- Arcane Shift
  - Mana cost reduced to 70 from 90.

### V13.21
- General
  - **Bug Fixes:** Voice lines for when he scores *First Blood* or attacks epic monsters have been restored.

### V13.18
- Essence Flux
  - **Bug Fixes:** Mark is now properly triggered when hitting a target with an ability that doesn't apply on-hit effects even if they are under the effect of a dodging ability.

### V13.8
- Stats
  - Base attack damage increased to 62 from 60.

## Trivia

- Ezreal was the first to have a 'Champion Spotlight'.
  - He is one of few to feature in multiple ones due to significant gameplay changes (the others being Lee Sin, Karma, Katarina, and Sivir).
    - He was also the first champion released after the game's official launch on 27-Oct-2009.
- He was named after Colt 'Ezreal' Hallam.
- His dance both before and after his visual update references the dance from the 'Hare Hare Yukai' ED of the anime.
  - According to a forum post by Colt 'Ezreal' Hallam, the idea for him to have this dance came from Miyuki 'Shurelia' Mitsuhashi.
  - A side-by-side comparison from before his update can be seen here.
  - A side-by-side comparison from after his update can be seen here.
- Much like Akali’s Visual Gameplay Update, Ezreal's previous iteration is still considered canon.
- Ezreal is the first champion to have 13 skins and one of the first to have 12 skins.
  - He and Miss Fortune attained their twelfth skin at the same time with the Pajama Guardian skin set.
- Ezreal is the first champion to have a skillshot ability that can give buffs to allies with his Essence Flux, but it was reworked from V8.20.
  - Ezreal is also the first champion to have a skillshot ability that can heal to allies with his Essence Flux, but its heal component was removed from V1.0.0.94.
- There is a NPC based on and named after Ezreal.

---
*This page was automatically generated from League of Legends Wiki data.*