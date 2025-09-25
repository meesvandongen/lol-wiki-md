# Kennen

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
| **Champion** | Kennen |
| **Title** | the Heart of the Tempest |
| **Resource** | Energy |
| **Range Type** | Ranged |
| **Release Date** | 2010-04-08 |
| **Release Patch** | V1.0.0.82 |
| **Latest Changes** | V25.14 |
| **Roles** | Specialist |
| **Riot Positions** | Top |
| **External Positions** | Top |
| **Blue Essence** | 2400 |
| **Riot Points** | 880 |
| **Difficulty** | 2 |
| **Hero Type** | Mage |
| **Alt Type** | Marksman |
| **Adaptive Type** | Magic |
| **Damage** | 3 |
| **Toughness** | 1 |
| **Control** | 3 |
| **Mobility** | 1 |
| **Utility** | 1 |
| **Style** | 65 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $580.0$ | $+98.0$ |
| **Energy** | $200.0$ | $+0.0$ |
| **Health Regen** | $5.5$ | $+0.65$ |
| **Energy Regen** | $50.0$ | $+0.0$ |
| **Armor** | $29.0$ | $+4.95$ |
| **Magic Resist** | $30.0$ | $+1.3$ |
| **Attack Damage** | $48.0$ | $+3.75$ |
| **Attack Speed** | $0.625$ | |
| **Movement Speed** | $335.0$ | $+0.0$ |
| **Attack Range** | $550.0$ | $+0.0$ |
| **Base Attack Speed** | $0.625$ | |
| **Attack Speed Ratio** | $0.69$ | |
| **Bonus AS per Level** | $3.4\%$ | |
| **Missile Speed** | $1700$ units/second | |
| **Acquisition Radius** | $550$ units | |
| **Gameplay Radius** | $55$ units | |
| **Pathing Radius** | $30$ units | |
| **Selection Radius** | $100$ units | |
| **Selection Height** | $177.778$ units | |
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
| **Damage Taken** | $95.0\%$ |

## Abilities

### Passive: Mark of the Storm

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Enemies |
| **Spell Shield** | True |

**INNATE:** **Kennen**’s abilities apply a stack of *Mark of the Storm* to enemies hit for 6 seconds, refreshing on subsequent applications and stacking up to 3 times.

The third stack against a target consumes them all to stun them for $1.25$ seconds and restore 25 energy. The stun duration is reduced to $0.5$ seconds if this occurs on the same target again within 6 seconds.

*Slicing Maelstrom can apply only up to 3 stacks on a target.*

**Notes:**

- **Kennen** will still restore energy if the target dies from the third stack of *Mark of the Storm*.
- After Slicing Maelstrom applies its third stack, the target gains a 6-second stack immunity against *Slicing Maelstrom* that refreshes on subsequent hits.

---

### Q: Thundering Shuriken

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.175$ seconds |
| **Target Range** | 1050 units |
| **Width** | 100 units |
| **Speed** | 1700 units/second |
| **Cost** | 60 / 55 / 50 / 45 / 40 Energy |
| **Cooldown** | 7 / 6.25 / 5.5 / 4.75 / 4 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Single target |
| **Projectile** | True |

**ACTIVE:** **Kennen** throws a shuriken in the target direction that deals magic damage to the first enemy hit.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 75 / 125 / 175 / 225 / 275 (+ 75% AP) |

**Notes:**

Effect at cast time start

---

### W: Electrical Surge

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Effect Radius** | 775 (Pending for test; targets acquired at cast start or end) units |
| **Cost** | 40 Energy |
| **Cooldown** | 13 / 11.25 / 9.5 / 7.75 / 6 seconds |
| **Targeting** | Proximity |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Special |
| **Projectile** | False |
| **Parry** | Special |

**PASSIVE:** **Kennen**’s basic attacks on-attack generate a stack of *Electrical Surge*, stacking up to 4 times. At 4 stacks, his next basic attack on-attack is empowered to consume all stacks on-hit to deal **bonus** magic damage and apply a stack of *Mark of the Storm*.
**Kennen** gains maximum stacks of *Electrical Surge* upon respawning.

| Attribute | Value |
|-----------|------:|
| **Bonus Magic Damage** | 35 / 45 / 55 / 65 / 75 (+ 80 / 90 / 100 / 110 / 120% **bonus** AD) (+ 35% AP) |

**ACTIVE:** **Kennen** sends out a surge of electricity that deals magic damage to all nearby enemies afflicted by *Mark of the Storm* or within *Slicing Maelstrom*.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 70 / 95 / 120 / 145 / 170 (+ 80% AP) |

*An enemy within cr 775 units with Mark of the Storm or inside Slicing Maelstrom is required to cast this ability. The target does not have to be visible to be targeted by this ability.*

**Notes:**

- The empowered basic attack applies bonus spell damage, while the electrical surge deals area damage.
- *Electrical Surge*’s enhanced attack will be consumed and the bonus damage will not be applied if **Kennen** is blinded.
  - *Electrical Surge*’s enhanced attack cannot be dodged or blocked.
- The empowered attack will trigger but not be consumed nor apply its effects against structures and wards.

---

### E: Lightning Rush

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 200 units |
| **Cost** | 80 Energy |
| **Cooldown** | 10 / 9 / 8 / 7 / 6 seconds |
| **Cooldown Start** | post-effect |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |

**ACTIVE:** **Kennen** turns into lightning for 2 seconds, becoming unable to declare basic attacks but gaining ghosting and (ms) 100% **bonus** movement speed. He deals magic damage to enemies he passes through, reduced to 65% against non-champions, and restores 40 energy upon damaging at least one enemy.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 80 / 120 / 160 / 200 / 240 (+ 80% AP) |
| **Non-Champion Damage** | 52 / 78 / 104 / 130 / 156 (+ 52% AP) |

*Lightning Rush* can be recast after $0.5$ seconds, and does so automatically after the duration.

**RECAST:** **Kennen** ends *Lightning Rush* to gain **bonus** attack speed and be allowed to exceed the attack speed cap (normally 3.003 attacks per second) by the amount gained for 4 seconds.

| Attribute | Value |
|-----------|------:|
| **Bonus Attack Speed** | 40 / 50 / 60 / 70 / 80% |

*Lightning Rush's recast can be used while affected by cast-inhibiting crowd control.*

**Notes:**

- *Lightning Rush* will only damage any particular enemy once per cast.
- For the purpose of aim-assist, **Kennen**’s attack range is reduced「 by 575 ⟷ to -25 」during *Lightning Rush*, further reduced to -$33.75$ with Rapid Firecannon.

---

### R: Slicing Maelstrom

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Effect Radius** | 550 units |
| **Cooldown** | 120 / 110 / 100 / 90 / 80 seconds |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | aoedot |

**ACTIVE:** **Kennen** summons a storm around himself for 3 seconds, gaining (armor) **bonus** armor and (mr) **bonus** magic resistance for the duration.

| Attribute | Value |
|-----------|------:|
| **Bonus Resistances** | 20 / 30 / 40 / 50 / 60 |

The storm strikes lightning bolts down on nearby enemies every $0.5$ seconds, each one dealing magic damage.

| Attribute | Value |
|-----------|------:|
| **Magic Damage Per Bolt** | 40 / 57.5 / 75 / 92.5 / 110 (+ $22.5$% AP) |

*Slicing Maelstrom* gains 10% additional damage for each subsequent strike a target has received, up to a combined total of 150% of the initial strike's damage.

| Attribute | Value |
|-----------|------:|
| **Total Single-Target Damage** | 300 / 431.25 / 562.5 / 693.75 / 825 (+ 168.75% AP) |

***Kennen** can move during Slicing Maelstrom's cast time.*

**Notes:**

No additional notes.

---

## Patch History

### V25.14
- Electrical Surge
  - **Bug Fixes:** Attack is no longer consumed on traps.

### V14.17
- Thundering Shuriken
  - AP ratio reduced to 75% AP from 85% AP.

### V14.13
- Stats
  - Base health reduced to 580 from 611.

### V14.9
- Lightning Rush
  - Indicator now includes a decal for the ability's collision radius.
- Slicing Maelstrom
  - Updated cast animation.
  - **New Effect:** Caster can now move during the cast time.

### V14.2
- Electrical Surge
  - **New Effect:** Now grants maximum passive stacks on respawn.

### V13.18
- Lightning Rush
  - Energy cost reduced to 80 at all ranks from 100 / 95 / 90 / 85 / 80.
- Slicing Maelstrom
  - Cooldown reduced to 120 / 100 / 80 seconds from 120 at all ranks.

### V13.16
- Kennen
  - **Bug Fixes:** VFX no longer conflicts with his character model to the extent that he becomes indistinguishable.

### V13.6
- Lightning Rush
  - **Bug Fixes:** Now properly deals 65% damage to jungle monsters.

### V13.5
- Thundering Shuriken
  - Base damage increased to 75 / 125 / 175 / 225 / 275 from 75 / 120 / 165 / 210 / 255.
  - AP ratio increased to 85% AP from 75% AP.
  - Cooldown reduced to 7 / 6.25 / 5.5 / 4.75 / 4 seconds from 8 / 7 / 6 / 5 / 4.
- Electrical Surge
  - **New Effect:** When an enemy champion becomes marked, now shows a range indicator to **Kennen**.
- Lightning Rush
  - Damage against non-champions increased to 65% from 50%.
    - Non-champion base damage increased to 52 / 78 / 104 / 130 / 156 from 40 / 60 / 80 / 100 / 120.
    - Non-champion AP ratio increased to 52% AP from 40% AP.

### V12.19
- Lightning Rush
  - **Undocumented:** Attack speed cap can now be exceeded during the attack speed buff.
    - *This change is documented on

## Trivia

- His dance references the Airflare, a breakdance move.
  - A side-by-side comparison can be seen here.
    - He shares this dance with Camille.
    - Her dance is very similar to Vayne, whose dance is a similar breakdance move.
- Kennen used to gain the 'Law of Inverse Ninja Strength' cosmetic Easter egg debuff (*"This unit is a flippin' ninja!"* - *"Ninjas are more effective when they work alone. For every Ninja on your team beyond yourself, you lose 1 health."*) when he, Akali, Shen, and/or Zed found themselves on the same team. It was removed in V3.14 for unknown reasons.

---
*This page was automatically generated from League of Legends Wiki data.*