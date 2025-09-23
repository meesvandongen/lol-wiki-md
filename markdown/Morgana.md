# Morgana

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
| **Champion** | Morgana |
| **Title** | the Fallen |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2009-02-21 |
| **Release Patch** | Alpha Week 2 |
| **Latest Changes** | V25.17 |
| **Roles** | Catcher |
| **Riot Positions** | Support |
| **External Positions** | Support |
| **Blue Essence** | 675 |
| **Riot Points** | 585 |
| **Difficulty** | 1 |
| **Hero Type** | Support |
| **Alt Type** | Mage |
| **Adaptive Type** | Magic |
| **Damage** | 2 |
| **Toughness** | 1 |
| **Control** | 3 |
| **Mobility** | 1 |
| **Utility** | 2 |
| **Style** | 100 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $630.0$ | $+104.0$ |
| **Mana** | $340.0$ | $+60.0$ |
| **Health Regen** | $5.5$ | $+0.4$ |
| **Mana Regen** | $11.0$ | $+0.4$ |
| **Armor** | $25.0$ | $+5.0$ |
| **Magic Resist** | $30.0$ | $+1.3$ |
| **Attack Damage** | $56.0$ | $+3.5$ |
| **Attack Speed** | $0.625$ | |
| **Movement Speed** | $335.0$ | $+0.0$ |
| **Attack Range** | $450.0$ | $+0.0$ |
| **Base Attack Speed** | $0.625$ | |
| **Attack Speed Ratio** | $0.625$ | |
| **Bonus AS per Level** | $1.5\%$ | |
| **Missile Speed** | $1600$ units/second | |
| **Acquisition Radius** | $600$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $120$ units | |
| **Selection Height** | $145$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $95.0\%$ |
| **Damage Taken** | $110.0\%$ |

#### URF

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $90.0\%$ |
| **Damage Taken** | $105.0\%$ |

## Abilities

### Passive: Soul Siphon

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Self |

**INNATE:** **Morgana** heals herself for 18% of the post-mitigation damage (Damage calculated after modifiers) dealt by her abilities against champions, large minions, and medium and large monsters.

**Notes:**

- No additional details.

---

### Q: Dark Binding

| Attribute | Value |
|-----------|------:|
| **Range** | 1300 units |
| **Cast Time** | $0.25$ seconds |
| **Width** | 140 units |
| **Speed** | 1200 units/second |
| **Cost** | 50 / 55 / 60 / 65 / 70 Mana |
| **Cooldown** | 10 seconds |
| **Queue Time** | $0.5$ seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Single target |
| **Projectile** | True |

**ACTIVE:** **Morgana** throws a sphere of dark magic in the target direction that deals magic damage to the first enemy hit and roots them for a duration.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 80 / 135 / 190 / 245 / 300 (+ 90% AP) |

| Attribute | Value |
|-----------|------:|
| **Root Duration** | 2 / 2.25 / 2.5 / 2.75 / 3 seconds |

**Notes:**

No additional notes.

---

### W: Tormented Shadow

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 900 units |
| **Effect Radius** | 280 units |
| **Cost** | 70 / 80 / 90 / 100 / 110 Mana |
| **Cooldown** | 12 seconds |
| **Queue Time** | $0.5$ seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | False |
| **Spell Effects** | aoedot |
| **Out of Range Behavior** | walk to location |

**PASSIVE:** *Tormented Shadow*’s **current** cooldown is reduced by 5% of its **total** cooldown whenever *Soul Siphon* triggers. Simultaneous triggers from multiple targets will stack the cooldown reduction.

**ACTIVE:** **Morgana** torments the soil at the target location, causing the area to become desecrated for 5 seconds. Enemies within take magic damage on-cast and every $0.5$ seconds thereafter, increased by type=target's **missing** health. *Tormented Shadow* deals 170% damage against monsters.

| Attribute | Value |
|-----------|------:|
| **Minimum Damage Per Tick** | 9 / 15.5 / 22 / 28.5 / 35 (+ 10% AP) |
| **Maximum Damage Per Tick** | 18 / 31 / 44 / 57 / 70 (+ 20% AP) |

| Attribute | Value |
|-----------|------:|
| **Minimum Total Damage** | 90 / 155 / 220 / 285 / 350 (+ 100% AP) |
| **Maximum Total Damage** | 180 / 310 / 440 / 570 / 700 (+ 200% AP) |

**Notes:**

- Damage from multiple *Tormented Shadows* does not stack.

---

### E: Black Shield

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 800 units |
| **Cost** | 80 mana |
| **Cooldown** | 26 / 23.5 / 21 / 18.5 / 16 seconds |
| **Queue Time** | $0.5$ seconds |
| **Targeting** | Unit |
| **Affects** | Self, Allies |

**ACTIVE:** **Morgana** grants a shield to the target allied champion or herself for 5 seconds, which absorbs incoming magic damage and grants crowd control immunity while it holds.

| Attribute | Value |
|-----------|------:|
| **Magic Shield Strength** | 100 / 155 / 210 / 265 / 320 (+ 70% AP) |

**Notes:**

- *Black Shield* will not resist self nor allied crowd control (including e.g. an allied Tempered Fate).
- *Black Shield* will not resist nearsight.
- *Black Shield* negates crowd control effects before any magic damage is absorbed; even if the shield is broken by an enemy dealing enough damage, its associated disables will not apply. Shield-destroying effects bypass this however, since they destroy the shield before applying their effects.
- *Black Shield* will not prevent effects other than crowd control from triggering (e.g. Nether Grasp will not suppress a target protected by *Black Shield* but the tether still applies).
  - Although not considered a *crowd control* effect, *Black Shield* is special-cased to block Test of Spirit’s spirit pull.
- *Black Shield* takes priority over other sources of crowd control immunity and those that grant immunity to specific types of crowd control (displacement immunity and slow immunity).
- Spell shield will take priority over *Black Shield*.
- *Black Shield* will always take priority over regular shields. If used in conjunction with Lifeline, the most recently-applied one will have priority however.
- *Black Shield* has a forgiveness radius of 175 units.

---

### R: Soul Shackles

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.35$ seconds |
| **Effect Radius** | 625 (Latch-on range) units |
| **Tether Radius** | 625 units |
| **Cost** | 100 Mana |
| **Cooldown** | 120 / 115 / 110 / 105 / 100 seconds |
| **Queue Time** | $0.5$ seconds |
| **Targeting** | Proximity |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | Special |
| **Spell Effects** | aoe |
| **Projectile** | False |

**ACTIVE:** **Morgana** latches chains of energy onto nearby enemy champions over the cast time, dealing magic damage and forming a tether between herself and each target for 3 seconds, during which she gains **bonus** movement speed and the targets are revealed and slowed by 20%.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 200 / 237.5 / 275 / 312.5 / 350 (+ 80% AP) |
| **Total Magic Damage** | 400 / 475 / 550 / 625 / 700 (+ 160% AP) |

| Attribute | Value |
|-----------|------:|
| **Bonus Movement Speed** | 20 / 30 / 40 / 50 / 60% |

If a target does not break their tether by the end of its duration, they are dealt the same magic damage again and become stunned for a duration, during which they are revealed.

| Attribute | Value |
|-----------|------:|
| **Stun Duration** | 1.5 / 1.625 / 1.75 / 1.875 / 2 seconds |

*An enemy champion within cr 575 units is required to cast this ability. The target does not have to be visible to be tethered by this ability.*

**Notes:**

- Spell shield will block the tether's application and initial damage but not the aftereffects of one already applied.
- **Morgana** will turn to face southeast upon casting *Soul Shackles*.
- A circle indicator for Soul Shackles' maximum tether range is visible to **Morgana** and her tethered targets only.

---

## Patch History

### V25.17
- Tormented Shadow
  - Minimum base damage per tick increased to 9 / 15.5 / 22 / 28.5 / 35 from 7 / 14 / 21 / 28 / 35.
    - Maximum base damage per tick increased to 18 / 31 / 44 / 57 / 70 from 14 / 28 / 42 / 56 / 70.
  - Mana cost reduced to 70 / 80 / 90 / 100 / 110 from 70 / 85 / 100 / 115 / 130.

### V25.16
- Tormented Shadow
  - Minimum base damage per tick increased to 7 / 14 / 21 / 28 / 35 from 6 / 11.5 / 17 / 22.5 / 28.
  - Minimum AP ratio per tick increased to 10% AP from 8.5% AP.
  - Bonus damage reduced to type=target's **missing** health from type=target's **missing** health.
    - Maximum base damage per tick reduced to 14 / 28 / 42 / 56 / 70 from 16.2 / 31.05 / 45.9 / 60.75 / 75.6.
    - Maximum AP ratio per tick reduced to 20% AP from 22.95% AP.

### V25.09
- Black Shield
  - Base shield increased to 100 / 155 / 210 / 265 / 320 from 80 / 135 / 190 / 245 / 300.
- Soul Shackles
  - Base damage per hit increased to 200 / 275 / 350 from 175 / 250 / 325.
  - Bonus movement speed increased to 20 / 40 / 60% from 10 / 35 / 60%.

### V14.24
- Tormented Shadow
  - **Bug Fixes:** No longer reveals the user if cast while the cursor is near an enemy champion.

### V14.20
- Black Shield
  - **New Effect:** Cast now has a forgiveness radius of 175 units.

### V14.18
- General
  - **Bug Fixes:** First encounter voice lines are no longer audible to enemies.

### V14.15
- General
  - **Bug Fixes:** Restored first encounter with Jarvan IV VO.

### V14.12
- General
  - **Bug Fixes:** "Loves me" and "Loves me not" voice-overs are no longer incorrectly swapped in her Joke.

### V14.4
- Soul Shackles
  - Tether now innately checks every $0.05$ seconds for valid range conditions, and no longer waits for every 8th server tick.
- Morgana
  - Black Shield
    - **Bug Fixes:** VFX now disappears smoothly when destroyed.

### V13.24
- Tormented Shadow
  - **Bug Fixes:** Pool VFX is now visible over terrain.

## Trivia

- Dark Binding's debuff reads: *“This unit is unable to move. Lasts for roughly 3 years.”*
- Morgana and Morgana are voiced.md) by Erica Lindbeck, who also voices Taliyah and Zoe.
- Her dance is a reference to Exid - Up & Down.
  - A side-by-side comparison can be seen here.
- Morgana was voiced.md) by Rebecca Schweitzer, who also voices Pre-rework Sivir.
- Her dance is a reference to the whirling practices of the Mevlevi Order.
  - A side-by-side comparison can be seen here.

---
*This page was automatically generated from League of Legends Wiki data.*