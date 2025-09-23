# Alistar

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
| **Champion** | Alistar |
| **Title** | the Minotaur |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2009-02-21 |
| **Release Patch** | Alpha Week 2 |
| **Latest Changes** | V25.15 |
| **Roles** | Vanguard |
| **Riot Positions** | Support |
| **External Positions** | Support |
| **Blue Essence** | 675 |
| **Riot Points** | 585 |
| **Difficulty** | 1 |
| **Hero Type** | Tank |
| **Alt Type** | Support |
| **Adaptive Type** | Magic |
| **Damage** | 1 |
| **Toughness** | 3 |
| **Control** | 3 |
| **Mobility** | 1 |
| **Utility** | 2 |
| **Style** | 65 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $685.0$ | $+120.0$ |
| **Mana** | $350.0$ | $+40.0$ |
| **Health Regen** | $8.5$ | $+0.85$ |
| **Mana Regen** | $8.5$ | $+0.8$ |
| **Armor** | $40.0$ | $+4.7$ |
| **Magic Resist** | $32.0$ | $+2.05$ |
| **Attack Damage** | $62.0$ | $+3.75$ |
| **Attack Speed** | $0.625$ | |
| **Movement Speed** | $330.0$ | $+0.0$ |
| **Attack Range** | $125.0$ | $+0.0$ |
| **Base Attack Speed** | $0.625$ | |
| **Attack Speed Ratio** | $0.625$ | |
| **Bonus AS per Level** | $2.1\%$ | |
| **Acquisition Radius** | $600$ units | |
| **Gameplay Radius** | $80$ units | |
| **Pathing Radius** | $50$ units | |
| **Selection Radius** | $140$ units | |
| **Selection Height** | $170$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $95.0\%$ |
| **Damage Taken** | $105.0\%$ |
| **Healing** | $80.0\%$ |

#### URF

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $100.0\%$ |
| **Damage Taken** | $95.0\%$ |

#### Ultimate Spellbook

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $95.0\%$ |
| **Damage Taken** | $100.0\%$ |

## Abilities

### Passive: Triumphant Roar

| Attribute | Value |
|-----------|------:|
| **Effect Radius** | 1000 (Range for dying enemies) / 950 (Heal radius) units |
| **Static Cooldown** | 3 |
| **Targeting** | Passive |
| **Affects** | Self, Allies |
| **Projectile** | Special |

**INNATE:** **Alistar** generates a stack of *Triumph* for each enemy champion he stuns or displaces with his abilities, and each time a nearby enemy minion or non-epic monster dies. He generates 7 stacks if a nearby enemy champion or epic monster dies.

At 7 stacks, **Alistar** consumes them all to heal himself for 5% of his **maximum** health and nearby allied champions for 7% of his **maximum** health.

*Triumphant Roar* can occur only once every few seconds, though **Alistar** may still generate stacks before then.

**Notes:**

- If **Alistar** is at 7 stacks when *Triumphant Roar* comes off cooldown, he will roar as soon as it elapses.
- **Alistar** does not gain stacks for unsuccessfully trying to apply crowd control, for instance against a crowd control immune target.
  - **Alistar** will still gain a stack for unsuccessfully trying to knock back a champion protected by a spell shield with *Headbutt*.
- Healing occurs simultaneously for **Alistar** and his allies.
  - There is a VFX projectile (700 missile speed) that cannot be destroyed by *Wind Wall* and is unrelated to the actual heal.
- **Alistar** loses all *Triumph* stacks upon dying.

---

### Q: Pulverize

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ (Standard cast) / $0.15$ (In WQ combo) seconds |
| **Effect Radius** | 375 units |
| **Cost** | 50 / 55 / 60 / 65 / 70 mana |
| **Cooldown** | 14 / 13 / 12 / 11 / 10 seconds |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |

**ACTIVE:** **Alistar** smashes the ground beneath him, dealing magic damage to nearby enemies and stunning and knocking them up simultaneously for 1 second.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 60 / 100 / 140 / 180 / 220 (+ 80% AP) |

**Notes:**

- *Pulverize* can be buffered during Headbutt to cast with reduced cast time when the dash ends.
- Displacement immunity will also resist the application of the stun. Effect at cast time end

---

### W: Headbutt

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 650 (Target range) units |
| **Speed** | 1200 / 1544 / units/second |
| **Cost** | 50 / 55 / 60 / 65 / 70 mana |
| **Cooldown** | 14 / 13 / 12 / 11 / 10 seconds |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spell |
| **Call For Help** | True |
| **Grounded** | True |
| **Knockdown** | True |

**ACTIVE:** **Alistar** dashes to the target enemy's location. If they are within 400 units upon arrival, he deals them magic damage and knocks them back 700 units (200 units when combined with Pulverize) over $0.5 seconds$ while also stunning them for $0.75$ seconds.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 55 / 110 / 165 / 220 / 275 (+ 100% AP) |

**Notes:**

- The dash speed depends on the inverse of the distance to the target:
  - If **Alistar**’s radius overlaps with his target's, he will dash with 1200 speed.
  - If **Alistar** is further from his target than the previous condition, he will dash with (1200 × dash distance) / (dash distance - sum of radiuses).
    - At maximum target range, against an enemy with a 65 gameplay radius (median of all champions), **Alistar** will dash with 1544 speed (reaching his target within $0.429$ seconds (rounded to next game tick)).
    - Both the lower the dash distance and the larger **Alistar** and/or his target, the faster the dash becomes; this scales hyperbolically.
- When Pulverize is buffered during the dash, the knockback distance is reduced to 200 units.
- The CC duration of the ability can be reduced by up to $33.33$% Tenacity, due to airborne not being reducible while the stun is.
- The knockback direction is in a straight line from **Alistar**’s original location at start of cast and the enemy when **Alistar**’s dash ends.
- *Headbutt* can knock enemies through terrain. The knockback stops short on walls that cannot be surpassed but the disable duration is not changed.
- The spell indicator for this ability also displays the direction for the knock back relative to **Alistar**’s position.
- **Alistar**’s attack range is reducedduring *Headbutt*, to prevent him from being able to attack the target when he completes the dash.
- Displacement immunity will not resist the application of the stun.

---

### E: Trample

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 350 units |
| **Cost** | 50 / 55 / 60 / 65 / 70 mana |
| **Cooldown** | 12 / 11.5 / 11 / 10.5 / 10 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | Special |
| **Spell Effects** | Special |
| **Parry** | Special |

**ACTIVE:** **Alistar** tramples the ground around him every $0.5$ seconds over 5 seconds, becoming ghosted and dealing magic damage to nearby enemies. Each time this damages at least one enemy champion, he generates a stack of *Trample* that lasts for the remaining duration, stacking up to 5 times.

| Attribute | Value |
|-----------|------:|
| **Magic Damage Per Tick** | 8 / 11 / 14 / 17 / 20 (+ 7% AP) |
| **Total Magic Damage** | 80 / 110 / 140 / 170 / 200 (+ 70% AP) |

At 5 stacks, **Alistar**’s next basic attack on-hit against a champion within 6 seconds ends *Trample*’s effects to deal 20 to 275 **bonus** magic damage and stun the target for 1 second.

**Notes:**

- *Trample* deals persistent area damage around **Alistar**, while the bonus damage on the resulting attack is proc damage.
- The stun and **bonus** damage are blocked by spell shields but the base attack damage is not.
- The on-hit effect is consumed if the attack blocked, but not if it is dodged or missed while **Alistar** is blinded. In all cases, the attack's effects are negated as normal.
- The bonus damage and stun can be applied by Guinsoo's Rageblade Phantom Hit if the triggering attack did not.
  - If the triggering attack does however apply the bonus damage and stun already, then Phantom Hit cannot apply them a subsequent time by triggering on-hit effects, despite *Trample*’s empowered attack being an on-hit effect.

---

### R: Unbreakable Will

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Cost** | 100 mana |
| **Cooldown** | 120 / 110 / 100 / 90 / 80 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Auto |
| **Affects** | Self |

**ACTIVE:** **Alistar** cleanses himself of all crowd control. For the next 7 seconds, he reduces incoming damage taken.

| Attribute | Value |
|-----------|------:|
| **Damage Reduction** | 55 / 60 / 65 / 70 / 75% |

**Notes:**

- True damage cannot be reduced by any means and will deal full damage to **Alistar** during *Unbreakable Will*.
- *Unbreakable Will*’s modifier to incoming damage stacks additively with Hemoplague for a total reduction of 45/55/65%.

---

## Patch History

### V25.15
- Stats
  - Base armor reduced to 40 from 47.

### V14.16
- Headbutt
  - **Bug Fixes:** No longer grants passive stacks from immobilizing effects that fail to apply to the target including when resisted by crowd control immunity.

### V14.10
- Headbutt
  - **Bug Fixes:** Hitbox is no longer extended when impacting an enemy at close range.

### V14.9
- Stats
  - Selection radius reduced to 140 units from 145.

### V13.7
- Stats
  - Base armor increased to 47 from 44.
  - Base health increased 685 from 670.

### V13.4
- Triumphant Roar
  - Ally heal increased to 7% of his **maximum** health from 6%.
- Pulverize
  - AP ratio increased to 80% AP from 70% AP.
- Headbutt
  - AP ratio increased to 100% AP from 90% AP.

### V13.3
- Triumphant Roar
  - Self heal changed to 5% **maximum** health from 23 to 142.
  - Ally heal changed to 6% of **Alistar's maximum** health from 46 to 284.
- Pulverize
  - Mana cost reduced to 50 / 55 / 60 / 65 / 70 from 55 / 60 / 65 / 70 / 75.
  - AP ratio increased to 70% AP from 50% AP.
- Headbutt
  - Mana cost reduced to 50 / 55 / 60 / 65 / 70 from 65 / 70 / 75 / 80 / 85.
  - AP ratio increased to 90% AP from 70% AP.
- Trample
  - Mana cost reduced to 50 / 55 / 60 / 65 / 70 from 50 / 60 / 70 / 80 / 90.
  - AP ratio per tick increased to 7% AP from 4% AP.
    - Maximum AP ratio increased to 70% AP from 40% AP.

### V12.14
- Pulverize
  - Cooldown reduced to 14 / 13 / 12 / 11 / 10 seconds from 15 / 14 / 13 / 12 / 11.
  - Mana cost reduced to 55 / 60 / 65 / 70 / 75 from 65 / 70 / 75 / 80 / 85.

### V12.10
- Stats
  - Base health increased to 670 from 600.
  - Health growth increased to 120 from 106.
  - Armor growth increased to $4.7$ from $3.5$.
  - Magic resistance growth increased to $2.05$ from $1.25$.
- Triumphant Roar
  - Base heal reduced to 23 to 142 from 25 to 161.
    - Ally heal reduced to 46 to 284 from 50 to 362.

### V11.11
- Pulverize
  - **Bug Fixes:** No longer grants a stack of Triumphant Roar when used on a spell-shielded target.
    - The same issue is not yet fixed for Headbutt.

## Trivia

- Alistar is voiced.md) by Harlan Hogan.
- *Alistar* is an alternative form of Alistair, the Anglicized form of Scottish Gaelic *Alasdair*, ultimately from Ancient Greek Ἀλέξανδρος, meaning "Defender": from Proto-Indo_European language roots **h₂lek-* "to ward off, to defend" and **h₂nḗr* "man"
- During Alpha Test, he was simply called 'Minotaur'.
- Alistar was deemed overpowered/obnoxious in the Ultra Rapid Fire game mode (available in April 2014) and was ultimately disabled in non-custom games.
- Alistar's Series 1 Eternals make the following references:
  - *By the Horns* references the phrase take the bull by the horns.
  - *Indomita-Bull* is a pun on the words "indomitable" and "bull".
  - *Matador* is another term for bullfighter.
- Alistar's Series 2 Eternals make the following references:
  - *The Combo* is a reference to the League of Legends community that each champion has its own combo depending on its difficulty, but nevertheless, Alistar is one of the champions with an easy to use combo.
  - *Unstoppa-Bull* is a pun on the words "unstoppable" and "bull".

---
*This page was automatically generated from League of Legends Wiki data.*