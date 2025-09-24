# Cassiopeia

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
| **Champion** | Cassiopeia |
| **Title** | the Serpent's Embrace |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2010-12-14 |
| **Release Patch** | V1.0.0.107 |
| **Latest Changes** | V25.18 |
| **Roles** | Battlemage |
| **Riot Positions** | Middle |
| **External Positions** | Middle |
| **Blue Essence** | 2400 |
| **Riot Points** | 880 |
| **Difficulty** | 3 |
| **Hero Type** | Mage |
| **Adaptive Type** | Magic |
| **Damage** | 3 |
| **Toughness** | 1 |
| **Control** | 3 |
| **Mobility** | 1 |
| **Utility** | 1 |
| **Style** | 100 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $630.0$ | $+104.0$ |
| **Mana** | $450.0$ | $+40.0$ |
| **Health Regen** | $5.5$ | $+0.5$ |
| **Mana Regen** | $8.0$ | $+0.8$ |
| **Armor** | $18.0$ | $+4.7$ |
| **Magic Resist** | $32.0$ | $+1.3$ |
| **Attack Damage** | $53.0$ | $+3.0$ |
| **Attack Speed** | $0.647$ | |
| **Movement Speed** | $328.0$ | $+0.0$ |
| **Attack Range** | $550.0$ | $+0.0$ |
| **Base Attack Speed** | $0.647$ | |
| **Attack Speed Ratio** | $0.647$ | |
| **Bonus AS per Level** | $1.5\%$ | |
| **Missile Speed** | $1500$ units/second | |
| **Acquisition Radius** | $600$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $120$ units | |
| **Selection Height** | $120$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### URF

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $95.0\%$ |
| **Damage Taken** | $105.0\%$ |

## Abilities

### Passive: Serpentine Grace

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Self |

**INNATE:** **Cassiopeia** gains 4 to 72 (ms) **bonus** movement speed, but she cannot purchase Boots items.

**BLESSING OF NOXUS BONUS:** *Serpentine Grace*’s **bonus** movement speed is increased by 1 to 18, for a total of 4+1 to 72+18.

**Notes:**

- The movement speed from *Serpentine Grace* is worth 48 gold per level, up to a maximum of 864 gold at level 18.
- Without other movement speed modifiers taken into account, *Serpentine Grace* grants **Cassiopeia** a total of 4+Cassiopeia|color=ms (ms) movement speed.
- As Magical Footwear grants boots, the rune will be replaced with Cash Back.

---

### Q: Noxious Blast

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 850 units |
| **Effect Radius** | 200 units |
| **Cost** | 50 / 55 / 60 / 65 / 70 mana |
| **Cooldown** | $3.5$ seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | aoedot |
| **Out of Range Behavior** | walk to location |

**ACTIVE:** **Cassiopeia** creates a blast at the target location that explodes after a $0.4$-second delay. Enemies within the blast are poisoned for 3 seconds, taking magic damage every 0.429 seconds over the duration.

| Attribute | Value |
|-----------|------:|
| **Total Magic Damage** | 75 / 110 / 145 / 180 / 215 (+ 65% AP) |
| **Magic Damage Per Tick** | 10.71 / 15.71 / 20.71 / 25.71 / 30.71 (+ 9.29% AP) |

If *Noxious Blast* hits an enemy champion, **Cassiopeia** gains (ms) **bonus** movement speed that decays over 3 seconds.

| Attribute | Value |
|-----------|------:|
| **Bonus Movement Speed** | 30 / 35 / 40 / 45 / 50% |

**Notes:**

- *Noxious Blast* always uses quick cast, regardless of player settings.
- The damage inflicted by *Noxious Blast* is calculated as follows: damage / 7 ticks rounded down to the nearest integer.
  - For example: with 0 **bonus** AP *Noxious Blast* inflicts 75 damages over 7 ticks of damage, so 75/7 (≈10.71) deals first of all 10 damage, then (2×75/7) 10 (≈11.43) deals in a second time 11 damages, then (3×75/7) 10 11 (≈11.14) deals in a third time 11 damages, and so on.
- The damage displayed for *Noxious Blast* DOT (Damage_over_time) always remains constant and corresponds to the previously mentioned formula: damage / 7 ticks rounded down to the nearest integer. It's important not to confuse the consistent displayed damage with the actual damage, which is updated with each tick, see tick and updates.
  - With the same example as above, the damage displayed is always 10.
- Occasionally, *Noxious Blast* may not deal the displayed damage. This discrepancy occurs when the calculated damage falls between 0.5 and 1. In such cases, rounding to the nearest integer, specifically rounding up, creates a 1-point difference. It's important to note that the damage is rounded down to the nearest integer.
  - For example: with a Blasting Wand, *Noxious Blast* shows 75 + so rounded to the nearest integer 116 magic damage, but really deals using the formula above 115 magic damage.
- When **Cassiopeia** has precisely 20 **bonus** AP or 40 **bonus** AP. In this scenario, *Noxious Blast* may inflict 1 less damage than intended. Importantly, this bug occurs independently of rounding considerations.
- Enemies within the blast of *Noxious Blast* are poisoned for something between $3.2$ and $3.3$ seconds.

---

### W: Miasma

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 700 units |
| **Effect Radius** | 200 (Individual poison clouds) units |
| **Speed** | 3000 units/second |
| **Cost** | 70 / 75 / 80 / 85 / 90 Mana |
| **Cooldown** | 24 / 22 / 20 / 18 / 16 seconds |
| **Queue Time** | $0.35$ seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | False |
| **Spell Effects** | aoedot |
| **Projectile** | True |
| **Out of Range Behavior** | cast at max |

**ACTIVE:** **Cassiopeia** spews forth 7 bolts of venom in an arc at the target location, creating toxic clouds at the area for 5 seconds.

Enemies within the clouds are poisoned to take magic damage every 0.263 seconds and become grounded and slowed by an amount that decays over the area's duration.

| Attribute | Value |
|-----------|------:|
| **Magic Damage Per Second** | 20 / 25 / 30 / 35 / 40 (+ 10% AP) |
| **Total Magic Damage** | 100 / 125 / 150 / 175 / 200 (+ 50% AP) |

| Attribute | Value |
|-----------|------:|
| **Slow** | 40 / 50 / 60 / 70 / 80% |

**Notes:**

- *Miasma*’s slow and ground debuffs are each marked as non-dispellable, so they are not removed by most cleanses. Each is however allowed to be removed by cleanses that **also** grant immunity to the debuff type, such as Ragnarok.
- If a target becomes untargetable while affected by the ground, the debuff will refresh to $0.25$ seconds.
- Wind Wall will block the portion of *Miasma* it destroys.
- *Miasma* inflicts 5 / 6.25 / 7.5 / 8.75 / 10 (+ $2.5$% AP) magic damage per tick, and in fact because there is 19 ticks and not 20 ticks, the **total** magic damage is currently not 100 / 125 / 150 / 175 / 200 (+ 50% AP) but 95 / 118.75 / 142.5 / 166.25 / 190 (+ $47.5$% AP).
  - Sometimes *Miasma* randomly inflicts only 18 ticks of damage.

---

### E: Twin Fang

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.125$ seconds |
| **Target Range** | 700 (BUG: Displayed range, 765, is slightly larger than the real range) units |
| **Speed** | 2500 units/second |
| **Cost** | 40 Mana |
| **Cooldown** | $0.75$ seconds |
| **Queue Time** | $0.35$ seconds |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spell |
| **Projectile** | True |
| **Call For Help** | True |

**ACTIVE:** **Cassiopeia** launches her fangs at the target enemy that deal 52 to 120 (+ 10% AP) magic damage. If this kills the target, *Twin Fang*’s (mana) mana cost is refunded.

Against a poisoned target, *Twin Fang* deals **bonus** magic damage and heals **Cassiopeia**. The heal is reduced by 75% against minions and small and medium monsters.

| Attribute | Value |
|-----------|------:|
| **Bonus Magic Damage** | 20 / 43 / 66 / 89 / 112 (+ 55% AP) |
| **Total Enhanced Damage** | 52 to 120 (+ 20 / 43 / 66 / 89 / 112) (+ 65% AP) |

| Attribute | Value |
|-----------|------:|
| **Heal** | 10 / 11.5 / 13 / 14.5 / 16% AP |
| **Reduced Heal** | 2.5 / 2.875 / 3.25 / 3.625 / 4% AP |

**Notes:**

- Upon reaching the target, *Twin Fang* will wait for an ongoing Noxious Blast to explode before dealing its damage.
- *Twin Fang*’s damage against poisoned targets:
  - When maxed first: 52+20 / 56+20 / 60+40 / 64+40 / 68+60 / 72+60 / 76+80 / 80+80 / 84+100 to 120+100 (+ 65% AP).
  - When maxed second: 52+20 / 56+20 / 60+20 / 64+20 / 68+20 / 72+20 / 76+20 / 80+40 / 84+40 / 88+60 / 92+60 / 96+80 / 100+100 to 120+100 (+ 65% AP).
  - When maxed last: 52+20 to 100+20 / 104+40 / 108+60 / 112+60 / 116+80 / 120+100 (+ 65% AP).
- *Twin Fang*’s cast indicator is incorrectly adding her own radius to the range like an edge range ability, and is therefore slightly larger than the actual cast range *Twin Fang* can cast at.
- Using *Twin Fang* on an out-of-range target after completing a movement order causes **Cassiopeia** to move back to her original location after casting, if no other order is issued during the entire process.
  - This may be a consequence of *Twing Fang* being a "Walk in range of the target" ability that does not interrupt movement commands.

---

### R: Petrifying Gaze

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.5$ seconds |
| **Effect Radius** | cr 850 (950 but starts 100 units behind Cassiopeia. Because of this, range is slightly greater laterally) |
| **Angle** | cr 80° |
| **Cost** | 100 mana |
| **Cooldown** | 120 / 110 / 100 / 90 / 80 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | False |

**ACTIVE:** **Cassiopeia** blasts enemies in a cone in the target direction, dealing magic damage to enemies struck within and slowing them by 40% for 2 seconds. Enemies with their facing direction towards her are instead stunned for the same duration.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 150 / 200 / 250 / 300 / 350 (+ 50% AP) |

**Notes:**

- **Cassiopeia** will turn to face the target direction over the cast time.
  - *Petrifying Gaze*’s target direction will change if **Cassiopeia**’s facing direction changes during the cast time (i.e. through knockbacks).
    - The visual effect will not follow changes in **Cassiopeia**’s facing direction and may not portray *Petrifying Gaze*’s target direction accurately.
- The facing direction of champions whose abilities/animations cause them to lock their facing or spin is always the direction they are moving in. For abilities that cause the champion to spin in place (e.g. Death Lotus, Judgment) it will factor the direction they were facing on cast.
  - R is an exception and his facing direction is actually considered to be the direction he is facing.

---

## Patch History

### V25.18
- Stats
  - Base mana increased to 450 from 400.
- Twin Fang
  - Poisoned bonus damage increased to 20 / 43 / 66 / 89 / 112 from 20 / 40 / 60 / 80 / 100.

### V25.S1.3
- Noxious Blast
  - Total AP ratio reduced to 65% AP from 70% AP.
- Miasma
  - AP ratio per second reduced to 10% AP from 15% AP.

### V25.S1.2
- Serpentine Grace
  - Blessing of Noxus bonus movement speed reduced to 1 to 18 from 2 to 36.

### V25.S1.1
- Serpentine Grace
  - **New Effect:** Now grants an additional 2 to 36 **bonus** movement speed if her team gets *Blessing of Noxus*.

### V14.16
- Twin Fang
  - **Bug Fixes:** Heal is now properly reduced against monsters.

### V14.15
- Stats
  - Base mana increased to 400 from 350.
  - Mana growth reduced to 40 from 60.
- Noxious Blast
  - Mana cost reduced to 50 / 55 / 60 / 65 / 70 from 50 / 60 / 70 / 80 / 90.
  - AP ratio per tick reduced to 10% AP from 12.8571% AP.
    - Total AP ratio reduced to 70% AP from 90% AP.
- Miasma
  - Mana cost reduced to 70 / 75 / 80 / 85 / 90 from 70 / 80 / 90 / 100 / 110.
- Twin Fang
  - Mana cost reduced to 40 at all ranks from 50 / 48 / 46 / 44 / 42.
  - Poison bonus AP ratio reduced to 55% AP from 60% AP.

### V14.10
- General
  - Magical Footwear is now replaced by Cash Back instead of Triple Tonic.

### V13.22
- Stats
  - Basic attack missile speed increased to 1500 from 1200.

### V12.23
- General
  - **Bug Fixes:** Basic attack hit VFX are now properly visible.

### V12.10
- Stats
  - Base health increased to 630 from 560.
  - Health growth increased to 104 from 90.
  - Armor growth increased to $4.7$ from $3.5$.
  - Magic resistance growth increased to $1.3$ from $0.5$.
- Twin Fang
  - Heal AP ratio reduced to 10 / 11.5 / 13 / 14.5 / 16% AP from 12 / 14 / 16 / 18 / 20% AP.

## Trivia

- Cassiopeia was the last champion released in 2010 as well as the third one to feature an 'Art Spotlight' (hers in particular shows how her taunt was animated).
  - Cassiopeia's basic attack particles were snakes at release, as seen in her Champion Spotlight.
- Cassiopeia - Katarina is one of seven pairs of sibling champions (the others being Kayle - Morgana, Garen - Lux, Nasus - Renekton, Yasuo - Yone, Darius - Draven, and Vi - Jinx).
- Twin Fang directly benefits from allied poison abilities in the same fashion Last Breath does with displacement ones.

---
*This page was automatically generated from League of Legends Wiki data.*