# Nasus

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
| **Champion** | Nasus |
| **Title** | the Curator of the Sands |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2009-10-01 |
| **Release Patch** | V0.9.25.24 |
| **Latest Changes** | V25.03 |
| **Roles** | Juggernaut |
| **Riot Positions** | Top |
| **External Positions** | Top |
| **Blue Essence** | 675 |
| **Riot Points** | 585 |
| **Difficulty** | 1 |
| **Hero Type** | Fighter |
| **Alt Type** | Tank |
| **Adaptive Type** | Physical |
| **Damage** | 2 |
| **Toughness** | 3 |
| **Control** | 2 |
| **Mobility** | 1 |
| **Utility** | 1 |
| **Style** | 20 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $631.0$ | $+104.0$ |
| **Mana** | $326.0$ | $+62.0$ |
| **Health Regen** | $9.0$ | $+0.9$ |
| **Mana Regen** | $7.45$ | $+0.5$ |
| **Armor** | $34.0$ | $+4.7$ |
| **Magic Resist** | $32.0$ | $+2.05$ |
| **Attack Damage** | $67.0$ | $+4.0$ |
| **Attack Speed** | $0.638$ | |
| **Movement Speed** | $350.0$ | $+0.0$ |
| **Attack Range** | $125.0$ | $+0.0$ |
| **Base Attack Speed** | $0.638$ | |
| **Attack Speed Ratio** | $0.638$ | |
| **Bonus AS per Level** | $3.5\%$ | |
| **Acquisition Radius** | $350$ units | |
| **Gameplay Radius** | $80$ units | |
| **Pathing Radius** | $50$ units | |
| **Selection Radius** | $135$ units | |
| **Selection Height** | $170$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $90.0\%$ |
| **Damage Taken** | $105.0\%$ |

#### Ultimate Spellbook

| Metric | Value |
|--------|------:|
| **Damage Taken** | $95.0\%$ |

## Abilities

### Passive: Soul Eater

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Self |

**INNATE:** **Nasus** gains life steal.

**Notes:**

- No additional details.

---

### Q: Siphoning Strike

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Cost** | 20 Mana |
| **Cooldown** | 7.5 / 6.5 / 5.5 / 4.5 / 3.5 seconds |
| **Cooldown Start** | post-effect |
| **Targeting** | Auto |
| **Affects** | Self |
| **Damage Type** | Physical |
| **Spell Shield** | False |
| **Spell Effects** | spell |
| **Parry** | True |

**ACTIVE:** **Nasus** empowers his next basic attack within 10 seconds to have an uncancellable windup, gain range, and deal **bonus** physical damage. *Siphoning Strike*’s **base** damage is affected by critical strike modifiers.

| Attribute | Value |
|-----------|------:|
| **Bonus Physical Damage** | 35 / 55 / 75 / 95 / 115 (+ 100% of *Siphoning Strike* stacks) |

If *Siphoning Strike* kills the target, **Nasus** permanently gains 3 stacks, increased to 12 if the target is a champion, large minion, or large monster.

*Siphoning Strike resets **Nasus**' basic attack timer.*

**Notes:**

- **Nasus** can gain *Siphoning Strike* stacks by killing any enemy unit, this includes champions, minions, monsters, wards, turrets, and pets.
  - Jungle plants and structures that are not turrets (e.g. inhibitors) will not grant stacks.
- *Siphoning Strike* will not generate stacks from secondary units killed by other effects (e.g. Tiamat).
- Life steal applies to the entire damage of *Siphoning Strike*
- *Siphoning Strike* will trigger Tear of the Goddess Manaflow.

---

### W: Wither

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 700 units |
| **Cost** | 80 Mana |
| **Cooldown** | 15 / 14 / 13 / 12 / 11 seconds |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Spell Shield** | True |
| **Call For Help** | True |

**ACTIVE:** **Nasus** ages the target enemy champion for 5 seconds, slowing them by 35% and crippling them byboth increasing every second over the duration.

| Attribute | Value |
|-----------|------:|
| **Maximum Slow** | 47 / 59 / 71 / 83 / 95% |
| **Additional Slow Per Second** | 3 / 6 / 9 / 12 / 15% |

| Attribute | Value |
|-----------|------:|
| **Maximum Cripple** | 35.25 / 44.25 / 53.25 / 62.25 / 71.25% |
| **Additional Cripple Per Second** | 2.25 / 4.5 / 6.75 / 9 / 11.25% |

**Notes:**

- *Wither* interrupts the target's attack windup when it is first applied.
- If *Wither*’s duration is affected by Tenacity the effects will apply slower (negative tenacity percentage) or faster (positive tenacity percentage) so the maximum values are still reached when the modified duration ends.
- *Wither*’s cripple effectiveness calculates from its slow's base values, thus slow resist will not interact with the attack speed modifier indirectly.
- Both slow and cripple from *Wither* are considered to be a single debuff.
  - Therefore, slow immunity will prevent both, even without technical cripple immunity.

---

### E: Spirit Fire

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 650 units |
| **Effect Radius** | 400 / sight 200 units |
| **Cost** | 60 / 70 / 80 / 90 / 100 Mana |
| **Cooldown** | 12 seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | False |
| **Spell Effects** | AoEDoT |
| **Out of Range Behavior** | walk to location |

**ACTIVE:** **Nasus** unleashes a spirit fire at the target location, granting sight of the area (centered at the location) for $2.5$ seconds (Estimated) and, after a delay, dealing magic damage to enemies within.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 50 / 80 / 110 / 140 / 170 (+ 60% AP) |

The fire then remains for 5 seconds, dealing magic damage each second to enemies within and inflicting them with armor penetration, lingering for 1 second.

| Attribute | Value |
|-----------|------:|
| **Magic Damage Per Tick** | 10 / 16 / 22 / 28 / 34 (+ 12% AP) |
| **Total Magic Damage** | 100 / 160 / 220 / 280 / 340 (+ 120% AP) |

| Attribute | Value |
|-----------|------:|
| **Armor Reduction** | 30 / 35 / 40 / 45 / 50% of target's armor |

**Notes:**

- No additional details.

---

### R: Fury of the Sands

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.2$ seconds |
| **Effect Radius** | cr 400 (Increased by 0.8 per 1% bonus beyond Nasus' base radius of 80 units) |
| **Cost** | 100 Mana |
| **Cooldown** | 120 / 110 / 100 / 90 / 80 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Auto |
| **Affects** | Self |
| **Damage Type** | Magic |
| **Spell Effects** | aoedot |

**ACTIVE:** **Nasus** empowers himself for 15 seconds, gaining health, armor, mr, increased size, and range for the duration.

| Attribute | Value |
|-----------|------:|
| **Bonus Health** | 300 / 375 / 450 / 525 / 600 |

| Attribute | Value |
|-----------|------:|
| **Bonus Resistances** | 40 / 47.5 / 55 / 62.5 / 70 |

| Attribute | Value |
|-----------|------:|
| **Increased Size** | 30 / 32.5 / 35 / 37.5 / 40% |

While **Nasus** is empowered, he deals magic damage every $0.5$ seconds to nearby enemies, capped at 240 per second, and *Siphoning Strike’s* cooldown is halved.

| Attribute | Value |
|-----------|------:|
| **Magic Damage Per Tick** | 1.5 / 1.75 / 2 / 2.25 / 2.5% (+ $0.5$% per 100 AP) of target's **maximum** health |
| **Total Magic Damage** | 45 / 52.5 / 60 / 67.5 / 75% (+ 15% per 100 AP) of target's **maximum** health |

**Notes:**

- *Fury of the Sands' *bonus health is not affected by Grievous Wounds and **Nasus** retains it once the duration ends.

---

## Patch History

### V25.S1.3
- Soul Eater
  - Life steal increased to key=% from key=%.

### V14.24
- Spirit Fire
  - **Bug Fixes:** No longer reveals the user if cast while the cursor is near an enemy champion.

### V14.21
- Soul Eater
  - Life steal reduced to key=% from key=%.
- Siphoning Strike
  - Base damage reduced to 35 / 55 / 75 / 95 / 115 from 40 / 60 / 80 / 100 / 120.

### V14.18
- Spirit Fire
  - Initial base damage reduced to 50 / 80 / 110 / 140 / 170 from 55 / 95 / 135 / 175 / 215.
    - Base damage per second reduced to 10 / 16 / 22 / 28 / 34 from 11 / 19 / 27 / 35 / 43.

### V14.13
- General
  - **Bug Fixes:** "Bad Blood" nemesis quest (against Renekton) no longer incorrectly triggers its win condition on **any** unit kill.
  - **Bug Fixes:** "Bad Blood" nemesis quest (against Renekton) no longer incorrectly activates when both nemeses are on the same team.

### V14.11
- Stats
  - Attack damage growth increased to 4 from $3.5$.
- Spirit Fire
  - Armor reduction increased to 30 / 35 / 40 / 45 / 50% from 25 / 30 / 35 / 40 / 45%.
  - Mana cost reduced to 60 / 70 / 80 / 90 / 100 from 70 / 85 / 100 / 115 / 130.

### V14.7
- Soul Eater
  - Life steal increased to key=% from key=%.
- Siphoning Strike
  - Base damage increased to 40 / 60 / 80 / 100 / 120 from 30 / 50 / 70 / 90 / 110.

### V13.12
- Soul Eater
  - Life steal increased to key=% from key=%.
- Fury of the Sands
  - Cooldown reduced to 120 / 100 / 80 seconds from 120 at all ranks.

### V13.4
- Fury of the Sands
  - Restored VFX overlay appearing for the moment while casting the ability for Nasus.

### V12.22
- Stats
  - Base mana regeneration increased to $7.45$ from $7.44$.

## Trivia

- *Nasus* literally means "nose" in Latin - referencing his African golden wolf head's long nose.
  - During development he was simply called *Anubis Head* or 'Amun' "hidden, unseen".
- He was voiced by Gene McDaniels.
- In the now-removed official League of Legends forums, the original icon of Fury of the Sands was used to represent the "Player Concepts" and "Champion Feedback" sections.
- Following the death of his original voice actor, he is voiced by Erik Todd Dellums in both League of Legends and Legends of Runeterra.
  - Dellums also voices Nasus in Shurima: Descent into the Tomb and Rise of the Ascended.
- 
  - In Nasus' case, Siphoning Strike infinitely stacks the bonus physical damage of itself.
- Nasus - Renekton is one of seven pairs of sibling champions (the others being Cassiopeia - Katarina, Kayle - Morgana, Garen - Lux, Yasuo - Yone, Darius - Draven, and Vi - Jinx).
  - Though not a pair, Anivia, Ornn, and Volibear are also siblings.
- Nasus is one of a few champions to have multiple textures in one skin. When he uses Fury of the Sands.
  - Four other champions with this feature are LeBlanc (via Mirror Image), Shaco’s (via Hallucinate), Wukong (via Warrior Trickster) and Malphite (via Granite Shield and Thunderclap).

---
*This page was automatically generated from League of Legends Wiki data.*