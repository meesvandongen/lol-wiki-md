# Nasus

## Table of Contents
- [Basic Information](#basic-information)
- [Statistics](#statistics)
  - [Advanced Stats](#advanced-stats)
- [Abilities](#abilities)
- [Patch History](#patch-history)
- [Trivia](#trivia)

## Basic Information

| Attribute | Value |
|-----------|-------|
| **Champion** | Nasus |
| **Title** | the Curator of the Sands |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2009-10-01 |
| **Release Patch** | V0.9.25.24 |
| **Roles** | Juggernaut |
| **Riot Positions** | Top |
| **External Positions** | Top |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $631.0$ | $+104.0$ | $2399.0$ |
| **Mana** | $326.0$ | $+62.0$ | $1380.0$ |
| **Health Regen** | $9.0$ | $+0.9$ | $24.3$ |
| **Mana Regen** | $7.45$ | $+0.5$ | $15.9$ |
| **Armor** | $34.0$ | $+4.7$ | $113.9$ |
| **Magic Resist** | $32.0$ | $+2.05$ | $66.8$ |
| **Attack Damage** | $67.0$ | $+4.0$ | $135.0$ |
| **Attack Speed** | $0.638$ | $+3.5\%$ | $1.015$ |
| **Movement Speed** | $350.0$ | $+0.0$ | $350.0$ |
| **Attack Range** | $125.0$ | $+0.0$ | $125.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.638$ |
| **Attack Speed Ratio** | $0.638$ |
| **Bonus AS per Level** | $3.5\%$ |
| **Acquisition Radius** | $350 units$ |
| **Gameplay Radius** | $80 units$ |
| **Pathing Radius** | $50 units$ |
| **Selection Radius** | $135 units$ |
| **Selection Height** | $170 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Soul Eater

**Innate:** **Nasus** gains based on level.

**Innate:** **Nasus** gains .

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Self |

**Notes:**

- No additional details.

---

### Q: Siphoning Strike

**Active:** **Nasus**’s next basic attack within a period will gain *range **bonus** range* and deal **bonus** physical damage based on the amount of Siphoning Strike stacks.

*If this attack kills the target, **Nasus** generates permanent stacks of Siphoning Strike, increased against champions, large minions, and large monsters.*

**Active:** **Nasus** empowers his next basic attack within 10 seconds to have an uncancellable windup, gain range*bonus** range*, and deal **bonus** physical damage. 'Siphoning Strike's **base** damage is affected by critical strike modifiers. If *Siphoning Strike* kills the target, **Nasus** permanently gains 3 stacks, increased to 12 if the target is a champion, large minion, or large monster. 'Siphoning Strike basic attack reset **Nasus**' basic attack timer.'

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $7.5-3.5$ seconds |
| **Cast Time** | none |
| **Cost** | 20 Mana |
| **Targeting** | Auto |
| **Affects** | Self |
| **Damage Type** | Physical |
| **Spell Shield** | False |
| **Spell Effects** | spell |

**Scaling:**
- **Bonus Physical Damage:** $35-115$ (+ 100% of *Siphoning Strike* stacks)

**Notes:**

- **Nasus** can gain *Siphoning Strike* stacks by killing any enemy unit, this includes champions, minions, monsters, wards, turrets, and pets.
  - Jungle plants and structures that are not turrets (e.g. inhibitors) will not grant stacks.
- *Siphoning Strike* will not generate stacks from secondary units killed by other effects (e.g. *Tiamat*).
- applies to the entire damage of *Siphoning Strike*
- *Siphoning Strike* will trigger *Tear of the Goddess* Manaflow.

---

### W: Wither

**Active:** **Nasus** ages the target enemy champion for a few seconds, gradually slow and cripple them over the duration.

**Active:** **Nasus* ages the target enemy champion for 5 seconds, slow them by 35% and cripple them byboth increasing every second over the duration.

| Attribute | Value |
|-----------|-------|
| **Range** | 700 units |
| **Cooldown** | $15-11$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | 80 Mana |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Spell Shield** | True |

**Scaling:**
- **Maximum Slow:** $35+3×4-35+154$%*Maximum Cripple:** $(35+3×4)*0.75-(35+15×4)0.75$%0.75-15×0.75$%

**Notes:**

- *Wither* interrupts the target's attack windup when it is first applied.
- If 'Wither's duration is affected by Tenacity the effects will apply slower (negative tenacity percentage) or faster (positive tenacity percentage) so the maximum values are still reached when the modified duration ends.
- 'Wither's cripple effectiveness calculates from its slow's base values, thus slow resist will not interact with the attack speed modifier indirectly.
- Both slow and cripple from *Wither* are considered to be a single debuff.
  - Therefore, slow immunity will prevent both, even without technical cripple immunity.

---

### E: Spirit Fire

**Active:** **Nasus** unleashes a spirit fire at the target location that deals magic damage to enemies within.

*The fire remains for a few seconds, armor penetration and continually dealing magic damage.*

**Active:** **Nasus** unleashes a spirit fire at the target location, granting sight of the area for $2.5$ seconds and, after a delay, dealing magic damage to enemies within. The fire then remains for 5 seconds, dealing magic damage each second to enemies within and inflicting them with armor penetration, lingering for 1 second.

| Attribute | Value |
|-----------|-------|
| **Range** | 650 units |
| **Cooldown** | 12 seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $60-100$ Mana |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | 400 / sight 200 units |
| **Spell Shield** | False |
| **Spell Effects** | AoEDoT |

**Scaling:**
- **Magic Damage:** $50-170$ (+ 60% AP)
- **Magic Damage Per Tick:* $10-34$ (+ 12% AP)10-34×10$ (+ 120% AP) **Armor Reduction:** $30-50$% of target's armor

**Notes:**

- No additional details.

---

### R: Fury of the Sands

**Active:** **Nasus** empowers himself for some time, gaining bonus size, health, armor, mr, and range.

*While empowered, **Siphoning Strike** has reduced *cooldown*, and **Nasus** creates an aura that continually deals magic damage to nearby enemies based on their **maximum** health.*

**Active:** **Nasus** empowers himself for 15 seconds, gaining *health **bonus** health*, *armor *bonus armor*, *mr **bonus** magic resistance*, increased size, and range*bonus** attack range* for the duration. While **Nasus** is empowered, he deals magic damage every $0.5$ seconds to nearby enemies, capped at 240 per second, and **Siphoning Strike*’s* *cooldown* is halved.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $120-80$ seconds |
| **Cast Time** | $0.2$ seconds |
| **Cost** | 100 Mana |
| **Targeting** | Auto |
| **Affects** | Self |
| **Damage Type** | Magic |
| **Effect Radius** | 400 units |
| **Spell Effects** | aoedot |

**Scaling:**
- **Bonus Health:** $300-600$
- **Bonus Resistances:** $40-70$
- **Increased Size:** $30-40$%
- **Magic Damage Per Tick:** $1.5-2.5$%

**Notes:**

- 'Fury of the Sands'bonus health is not affected by Grievous Wounds and **Nasus** retains it once the duration ends.

---

## Patch History

### V14.24
- *Spirit Fire*
  - **Bug Fixes:** No longer reveals the user if cast while the cursor is near an enemy champion.

### V14.21
- *Soul Eater*
  - Life steal reduced to key=% from key=%.
- *Siphoning Strike*
  - Base damage reduced to $35-115$ from $40-120$.

### V14.18
- *Spirit Fire*
  - Initial base damage reduced to $50-170$ from $55-215$.
    - Base damage per second reduced to $50×0.2-170×0.2$ from $55×0.2-215×0.2$.

### V14.13
- General
  - **Bug Fixes:** "Bad Blood" nemesis quest (against **Renekton**) no longer incorrectly triggers its win condition on **any** unit kill.
  - **Bug Fixes:** "Bad Blood" nemesis quest (against **Renekton**) no longer incorrectly activates when both nemeses are on the same team.

### V14.11
- Stats
  - Attack damage growth increased to 4 from $3.5$.
- *Spirit Fire*
  - Armor reduction increased to $30-50$% from $25-45$%.
  - Mana cost reduced to $60-100$ from $70-130$.

### V14.7
- *Soul Eater*
  - Life steal increased to key=% from key=%.
- *Siphoning Strike*
  - Base damage increased to $40-120$ from $30-110$.

### V13.12
- *Soul Eater*
  - Life steal increased to key=% from key=%.
- *Fury of the Sands*
  - Cooldown reduced to $120-80 3$ seconds from 120 at all ranks.

### V13.4
- *Fury of the Sands*
  - Restored VFX overlay appearing for the moment while casting the ability for Nasus.

### V12.22
- Stats
  - Base mana regeneration increased to $7.45$ from $7.44$.

### V12.19
- *Siphoning Strike*
  - Bonus attack range increased to 50 units from 25.
- *Wither*
  - Cripple increased to 75% of slow from 50%.
    - Base cripple increased to $35×0.75$% from $35×0.5$%.
    - Additional cripple per second increased to $3×0.75-15×0.75$% from $3×0.5-15×0.5$%.
    - Maximum cripple increased to $(35+3×4)*0.75-(35+15×4)*0.75$% from $(35+3×4)*0.5-(35+15×4)*0.5$%.
- *Fury of the Sands*
  - Bonus increased to $30-40 3$% from 30% at all ranks.
  - **New Effect:** Damage radius now scales with bonus size.
    - Minimum radius of cr 400 units, which cannot be reduced below this amount. Scales with $0.8$ bonus range per 1% bonus beyond **Nasus**' base radius of 80 units.
  - Damage tick interval reduced to $0.5$ seconds per tick from 1 second per tick.
    - Total damage unchanged.
  - **Bug Fixes:** Damage now displays as the stacking display type.

## Trivia

- *Nasus* literally means "nose" in Latin - referencing his African golden wolf head's long nose.
  - During development he was simply called *Anubis Head* or 'Amun' "hidden, unseen".
- He was voiced by Gene McDaniels.
- In the now-removed official League of Legends forums, the original icon of *Fury of the Sands* was used to represent the "Player Concepts" and "Champion Feedback" sections.
- Following the death of his original voice actor, he is voiced by Erik Todd Dellums in both League of Legends and Legends of Runeterra.
  - Dellums also voices Nasus in Shurima: Descent into the Tomb and Rise of the Ascended.
  - Nasus is voiced by the same voice actor.
- 
  - In Nasus' case, *Siphoning Strike* infinitely stacks the bonus physical damage of itself.
- Nasus - **Renekton** is one of seven pairs of sibling champions (the others being **Cassiopeia** - **Katarina**, **Kayle** - **Morgana**, **Garen** - **Lux**, **Yasuo** - **Yone**, **Darius** - **Draven**, and **Vi** - **Jinx**).
  - Though not a pair, **Anivia**, **Ornn**, and **Volibear** are also siblings.
- Nasus is one of a few champions to have multiple textures in one skin. When he uses *Fury of the Sands*.
  - Four other champions with this feature are **LeBlanc** (via *Mirror Image*), **Shaco**’s (via *Hallucinate*), **Wukong** (via *Warrior Trickster*) and **Malphite** (via *Granite Shield* and *Thunderclap*).

---
*This page was automatically generated from League of Legends Wiki data.*