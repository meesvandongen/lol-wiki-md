# Orianna

## Table of Contents
- [Basic Information](#basic-information)
- [Statistics](#statistics)
  - [Map-specific Stats](#map-specific-stats)
- [Pets](#pets)
- [Abilities](#abilities)
- [Patch History](#patch-history)
- [Trivia](#trivia)

## Basic Information

| Attribute | Value |
|-----------|------:|
| **Champion** | Orianna |
| **Title** | the Lady of Clockwork |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2011-06-01 |
| **Release Patch** | V1.0.0.119 |
| **Latest Changes** | V25.09 |
| **Roles** | Burst |
| **Riot Positions** | Middle |
| **External Positions** | Middle |
| **Blue Essence** | 2400 |
| **Riot Points** | 880 |
| **Difficulty** | 3 |
| **Hero Type** | Mage |
| **Alt Type** | Support |
| **Adaptive Type** | Magic |
| **Damage** | 2 |
| **Toughness** | 1 |
| **Control** | 2 |
| **Mobility** | 1 |
| **Utility** | 2 |
| **Style** | 90 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $585.0$ | $+110.0$ |
| **Mana** | $418.0$ | $+25.0$ |
| **Health Regen** | $7.0$ | $+0.55$ |
| **Mana Regen** | $8.0$ | $+0.8$ |
| **Armor** | $20.0$ | $+4.2$ |
| **Magic Resist** | $26.0$ | $+1.3$ |
| **Attack Damage** | $44.0$ | $+2.6$ |
| **Attack Speed** | $0.658$ | |
| **Movement Speed** | $325.0$ | $+0.0$ |
| **Attack Range** | $525.0$ | $+0.0$ |
| **Base Attack Speed** | $0.658$ | |
| **Attack Speed Ratio** | $0.658$ | |
| **Bonus AS per Level** | $3.5\%$ | |
| **Missile Speed** | $1500$ units/second | |
| **Acquisition Radius** | $525$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $120$ units | |
| **Selection Height** | $140$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### URF

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $90.0\%$ |
| **Damage Taken** | $110.0\%$ |

## Pets

### The Ball

| Attribute | Value |
|-----------|------:|
| **Sight** | 700 (While static and not attached) / 225 (Area reveal radius (can see into brush or terrain)) |
| **Control** | Can be commanded by **Orianna** to perform certain actions when she casts Command: Attack, Command: Dissonance, Command: Protect, or Command: Shockwave. |
| **Targeting** | Untargetable |
| **Spell Effects** | *The Ball*, when hitting enemies, applies spell effects as area damage. |

**Abilities:**

- **Escort:** *The Ball* is attached to **Orianna** at all times, but can be commanded to attach to other units through Command: Protect.
- **Clockwork Affinity:** *The Ball*, while not attached to a unit, can be picked up by **Orianna** when she moves near it. If **Orianna** moves too far away from *The Ball*, it snaps back and attaches to her instantly.
- **Collision Immunity:** *The Ball* ignores all terrain collision.

---

## Abilities

### Passive: Clockwork Windup

| Attribute | Value |
|-----------|------:|
| **Effect Radius** | 1290 (Standard leash range, estimated) / 1355 (Leash range on ally, estimated) / 135 (Ball pickup range, estimated) units |
| **Targeting** | Passive |
| **Affects** | Enemies, Self |
| **Damage Type** | Magic |
| **Spell Effects** | Proc |
| **Parry** | True |

**INNATE - THE BALL:** **Orianna** is accompanied by *The Ball*, which she commands with her abilities. *The Ball* incurs a (cd) $0.01$-second global cooldown on all of her abilities when she casts any ability. Additionally, *The Ball* attaches to **Orianna** if she is near it, and snaps back to her if she moves too far away from it, which incurs a (cd) $0.75$-second cooldown on *Command: Shockwave*.

**INNATE:** **Orianna**’s basic attacks generate a stack of *Clockwork Winding* for 4 seconds, refreshing on subsequent attacks and stacking up to 2 times. All stacks are lost when attacking a new enemy.


**CLOCKWORK WINDING:** **Orianna**’s basic attacks are empowered to deal 10 to 50 (+ 15% AP) **bonus** magic damage on-hit, increased by「 20% ⟷ 10×0.2 to 50×0.2 (+ 3% AP) 」per stack, up to 10*(1+0.2×2) to 50*(1+0.2×2) (+ 21% AP) **total bonus** magic damage.

*See [Pets](#Pets) for more details about The Ball.*

**Notes:**

- *Clockwork Winding*’s stack count can be seen in **Orianna**’s buff bar.
- *The Ball* will not incur its global cooldown from a *Command: Protect* cast that is used on a target already attached to it.
- The attacks do not deal the **bonus** damage against structures.
- Runaan's Hurricane extra bolts will cause *Clockwork Winding* to reset its stacks, as she hits targets other than her latest one on-hit.

---

### Q: Command: Attack

| Attribute | Value |
|-----------|------:|
| **Cast Time** | None |
| **Target Range** | 825 units |
| **Effect Radius** | 175 (Landing impact radius) units |
| **Width** | 160 (Flying ball missile width) units |
| **Speed** | 1400 (Flying ball missile speed) units/second |
| **Cost** | 35 Mana |
| **Cooldown** | 6 / 5.25 / 4.5 / 3.75 / 3 seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Spellaoe |
| **Projectile** | Special |
| **Out of Range Behavior** | walk to location |

**ACTIVE:** **Orianna** commands *The Ball* to fly to the target location and remain there, dealing magic damage to enemies it passes through and nearby enemies upon arrival, reduced to 70% against those hit beyond the first.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 60 / 90 / 120 / 150 / 180 (+ 55% AP) |
| **Reduced Damage** | 42 / 63 / 84 / 105 / 126 (+ 38.5% AP) |

**Notes:**

- If *The Ball* is currently on **Orianna** herself, *Command: Attack* will have it cast to fly at least 150 units if the player attempts to cast it closer.
- *The Ball* does not grant sight while in flight, but the sound effect of striking an enemy is audible through the Fog of War.
- **Orianna** cannot use Command: Protect while *The Ball* is in transit with *Command: Attack* but she may buffer Command: Dissonance and Command: Shockwave to cast once it has reached the location.
- *The Ball* will remain at the target location until **Orianna** retrieves it or if she goes beyond its leash range.
  - *The Ball* will also never exceed its leash range regardless of target direction.
- *The Ball* will drop to the ground upon colliding with Yasuo’s W but not with Braum’s E.
  - Samira’s Blade Whirl?

---

### W: Command: Dissonance

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 225 (Both damage and persisting field radius) units |
| **Cost** | 60 / 65 / 70 / 75 / 80 Mana |
| **Cooldown** | 7 seconds |
| **Targeting** | Auto |
| **Affects** | Enemies, Allies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |

**ACTIVE:** **Orianna** commands *The Ball* to emit an electric pulse that deals magic damage to nearby enemies.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 70 / 120 / 170 / 220 / 270 (+ 70% AP) |

The pulse leaves behind an electric field that last 3 seconds, granting (ms) **bonus** movement speed to **Orianna** and her allies when they move within. Enemies that move within the field are slowed by the same amount, decaying over 2 seconds after leaving.

| Attribute | Value |
|-----------|------:|
| **Movement Speed Modifier** | 20 / 25 / 30 / 35 / 40% |

**Notes:**

- No additional notes.

---

### E: Command: Protect

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 1120 units |
| **Width** | 160 (Flying ball missile width) units |
| **Speed** | 1850 (Flying ball missile speed) units/second |
| **Cost** | 60 Mana |
| **Cooldown** | 9 seconds |
| **Targeting** | Unit |
| **Affects** | Enemies, Allies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | aoe |
| **Projectile** | True |

**PASSIVE:** *The Ball* grants (armor) **bonus** armor and (mr) **bonus** magic resistance to the unit it is attached to.

| Attribute | Value |
|-----------|------:|
| **Bonus Resistances** | 6 / 12 / 18 / 24 / 30 |

**ACTIVE:** **Orianna** commands *The Ball* to fly to herself or the target allied champion and attach itself to the target, dealing magic damage to enemies it passes through and granting the target a shield for $2.5$ seconds upon arrival.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 60 / 90 / 120 / 150 / 180 (+ 30% AP) |

| Attribute | Value |
|-----------|------:|
| **Shield Strength** | 55 / 90 / 125 / 160 / 195 (+ 45% AP) |

**Notes:**

- *The Ball* does not grant sight while in flight, but the sound effect of striking an enemy is audible through the Fog of War.
- **Orianna** cannot cast her other abilities while the *The Ball* is moving to the target with *Command: Protect*.
- **Orianna** gains *Command: Protect*’s bonus resistances whenever *The Ball* is attached to her (even if she didn't self-cast).
- *The Ball* will remain where *Command: Protect*’s target ally died or, if they died or became untargetable before The Ball reaches them, it will instead snap back to **Orianna**.
  - *The Ball* will return to **Orianna** if *Command: Protect*’s target ally moves too far away from her (*The Ball* will always respect its leash range, which is increased by 100 when attached to allied champions).
- *The Ball* will join *Command: Protect*’s target ally in stealth if they are affected by it when **Orianna** targets them.
- *The Ball* snaps back to **Orianna** upon being intercepted by projectile-blocking effects.
- *Command: Protect* has a forgiveness radius of 175 units.

---

### R: Command: Shockwave

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.5$ seconds |
| **Effect Radius** | 415 units |
| **Cost** | 100 Mana |
| **Cooldown** | 110 / 102.5 / 95 / 87.5 / 80 seconds |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |

**ACTIVE:** **Orianna** commands *The Ball* to unleash a shockwave that deals magic damage to nearby enemies, stuns them for $0.75$ seconds, and pulls them over 325 units, though not through terrain.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 250 / 325 / 400 / 475 / 550 (+ 95% AP) |

**Notes:**

- *Command: Shockwave* tosses enemies a set distance (those on the edge of the flinging area will be tossed to the center while those close to the middle will go over *The Ball* towards the edge) and can send them through walls. Effect at cast time end
  - *Command: Shockwave* will cast from wherever the unit the *The Ball* is attached to is at the end of the cast time, even if the unit exceeds max tether range.
- Displacement immunity will not resist the application of the stun.

---

## Patch History

### V25.09
- Command: Attack
  - Mana cost changed to 35 at all ranks from 30 / 35 / 40 / 45 / 50.

### V14.20
- Command: Protect
  - **New Effect:** Cast now has a forgiveness radius of 175 units.

### V14.16
- Command: Attack
  - AP ratio increased to 55% AP from 50% AP.

### V14.13
- Stats
  - Base attack damage increased to 44 from 40.

### V14.3
- Command: Shockwave
  - **Bug Fixes:** Cast VFX is now properly and consistently tracked in Fog of War, such as when cast on units that are moving very fast before entering vision.

### V13.24#December 12th Hotfix|V13.24
- Stats
  - Base health reduced to 585 from 600.
- Command: Dissonance
  - Movement speed modifier reduced to 20 / 25 / 30 / 35 / 40% from 30 / 35 / 40 / 45 / 50%.

### V13.22
- Stats
  - Basic attack missile speed increased to 1500 from 1450.

### V13.19
- Command: Shockwave
  - **Bug Fixes:** No longer is castable beyond its intended range.

### V13.18
- General
  - Updated ability icons.
- Clockwork Windup
  - **Bug Fixes:** Basic attacks no longer improperly cancel when the ball returns to her location.

### V13.17
- Stats
  - Health growth increased to 110 from 105.
- Command: Dissonance
  - Base damage increased to 70 / 120 / 170 / 220 / 270 from 60 / 105 / 150 / 195 / 240.

## Trivia

- *Orianna* is a widespread Romance languages name, merged from two Latin roots with feminine suffix *-na*:
  - *Orior* "I rise"; cf. Orient, referring the rising sun.
  - *Aurum* "gold"; which she shares with Aurelion Sol.
- Orianna was the last champion priced lower than on release.
- Clockwork Winding is named after and references Clockwork.
- Orianna's icon during her 'Champion Spotlight' was a walfas-styled face.
- Multiple miniature copies of The Ball can be seen on Dr. Mundo desk during the game's Mac Version trailer.
- In the now-removed official League of Legends forums, the old icon of Command: Shockwave was used to represent the "Forum Games" section.

---
*This page was automatically generated from League of Legends Wiki data.*