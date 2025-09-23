# Nami

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
| **Champion** | Nami |
| **Title** | the Tidecaller |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2012-12-07 |
| **Release Patch** | V1.0.0.152 |
| **Latest Changes** | V14.24 |
| **Roles** | Enchanter |
| **Riot Positions** | Support |
| **External Positions** | Support |
| **Blue Essence** | 1575 |
| **Riot Points** | 790 |
| **Difficulty** | 2 |
| **Hero Type** | Support |
| **Alt Type** | Mage |
| **Adaptive Type** | Magic |
| **Damage** | 1 |
| **Toughness** | 1 |
| **Control** | 3 |
| **Mobility** | 1 |
| **Utility** | 2 |
| **Style** | 90 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $560.0$ | $+88.0$ |
| **Mana** | $365.0$ | $+43.0$ |
| **Health Regen** | $5.5$ | $+0.55$ |
| **Mana Regen** | $11.5$ | $+0.4$ |
| **Armor** | $29.0$ | $+5.2$ |
| **Magic Resist** | $30.0$ | $+1.3$ |
| **Attack Damage** | $54.0$ | $+3.1$ |
| **Attack Speed** | $0.644$ | |
| **Movement Speed** | $335.0$ | $+0.0$ |
| **Attack Range** | $550.0$ | $+0.0$ |
| **Base Attack Speed** | $0.644$ | |
| **Attack Speed Ratio** | $0.644$ | |
| **Bonus AS per Level** | $2.6\%$ | |
| **Missile Speed** | $1500$ units/second | |
| **Acquisition Radius** | $600$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $125$ units | |
| **Selection Height** | $180$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $95.0\%$ |
| **Damage Taken** | $105.0\%$ |
| **Healing** | $100.0\%$ |

## Abilities

### Passive: Surging Tides

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Allies |

**INNATE:** **Nami**’s abilities grant ms (+ 25% AP) **bonus** movement speed to allied champions hit, decaying over $1.5$ seconds. The bonus is「 doubled ⟷ increased to 200 (+ 50% AP) 」from *Tidal Wave*.

**Notes:**

- *Tidal Wave* uses a separate empowered buff that has priority over the normal buff, and it will be used even if the full/current strength of the empowered buff is weaker than the full-strength normal buff.
  - If the normal buff remains ongoing when the empowered buff expires, it will then begin with full strength and completely decay over its **remaining** duration.

---

### Q: Aqua Prison

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 850* (Indicator displays 875) units |
| **Effect Radius** | 200 (Effect radius for enemies) / 225 (Effect radius for Surging Tides on allies, could also be 160 + edge radius, this is pending for test) units |
| **Cost** | 60 mana |
| **Cooldown** | 12 / 11 / 10 / 9 / 8 seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |
| **Projectile** | True |
| **Out of Range Behavior** | walk to location |

**ACTIVE:** **Nami** launches a bubble at the target location that lands after , dealing magic damage to enemies hit and suspending them for $1.5$ seconds, during which they are revealed.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 90 / 145 / 200 / 255 / 310 (+ 50% AP) |

*Allied champions are granted Surging Tides in a slightly larger area.*

**Notes:**

- The combined time from the start of the cast time to the bubble landing is + = $0.99$ seconds.
- *Aqua Prison* provides the status effect that allows Last Breath to be cast.

---

### W: Ebb and Flow

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 725 units |
| **Effect Radius** | 800 (Bounce range) units |
| **Speed** | 2500 (Initial speed) / 1500 (Bounce speed) units/second |
| **Cost** | 70 / 75 / 80 / 85 / 90 mana |
| **Cooldown** | 10 seconds |
| **Targeting** | Unit |
| **Affects** | Enemies, Allies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | aoe |
| **Projectile** | True |
| **Call For Help** | True |

**ACTIVE:** **Nami** unleashes a stream of water onto the target champion or herself, which then bounces to nearby unaffected champions up to twice, alternating between enemies and allies and **Nami**.

*Ebb and Flow* heals allies and deals magic damage to enemies, with each bounce modifying the effectiveness of the next by -10% (+ 10% per 100 AP).

| Attribute | Value |
|-----------|------:|
| **Heal** | 55 / 80 / 105 / 130 / 155 (+ 40% AP) |
| **Minimum Heal** | (55 to 155)*(1-0.1×2) (+ 32% AP) |

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 60 / 95 / 130 / 165 / 200 (+ 50% AP) |
| **Minimum Damage** | (60 to 200)*(1-0.1×2) (+ 40% AP) |

**Notes:**

- If cast on an enemy with a spell shield they will not take damage but *Ebb and Flow* will still continue to bounce, and the target who blocked the ability may be targeted again by the final bounce.

---

### E: Tidecaller's Blessing

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 800 units |
| **Cost** | 55 / 60 / 65 / 70 / 75 mana |
| **Cooldown** | 11 seconds |
| **Targeting** | Unit |
| **Affects** | Self, Allies |
| **Damage Type** | Magic |
| **Spell Shield** | False |
| **Spell Effects** | proc |
| **Parry** | True |

**ACTIVE:** **Nami** blesses herself or an allied champion for 6 seconds, empowering their next 3 basic attacks or abilities to each deal **bonus** magic damage and slow enemies for 1 second.

| Attribute | Value |
|-----------|------:|
| **Bonus Magic Damage Per Hit** | 20 / 30 / 40 / 50 / 60 (+ 20% AP) |
| **Total Bonus Magic Damage** | 60 / 90 / 120 / 150 / 180 (+ 60% AP) |

| Attribute | Value |
|-----------|------:|
| **Slow** | 15 / 20 / 25 / 30 / 35% (+ 5% per 100 AP) |

Empowered abilities that apply area damage only deal key=% of the **bonus** damage to non-champions.

**Notes:**

- *Tidecaller's Blessing* has a forgiveness radius of 175 units.
- The effect will not trigger against structures nor wards.
- The empowerment given to allies respects enchantment redirection.
- Damage instances of *Tidecaller's Blessing* beyond the first do not count as separate applications for the purposes of Electrocute and Phase Rush.

---

### R: Tidal Wave

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.5$ seconds |
| **Target Range** | 2750 units |
| **Effect Radius** | 750 (vision radius) units |
| **Width** | 500 units |
| **Speed** | 850 units/second |
| **Cost** | 100 mana |
| **Cooldown** | 120 / 110 / 100 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | True |

**ACTIVE:** **Nami** surges a tidal wave in the target direction, granting sight around its trajectory as it travels, dealing magic damage to enemies hit, knocking them up for $0.5$ (Estimated) seconds, and slowing them by 70% for type=distance traveled seconds.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 150 / 250 / 350 (+ 60% AP) |

**Notes:**

- Surging Tides will trigger immediately on **Nami** on-cast.
- The wave travels over .

---

## Patch History

### V25.S1.3
- General
  - Adjusted splash artwork for Nami.

### V14.24
- Aqua Prison
  - **Bug Fixes:** No longer reveals the user if cast while the cursor is near an enemy champion.

### V14.20
- Tidecaller's Blessing
  - **New Effect:** Cast now has a forgiveness radius of 175 units.

### V14.11
- Stats
  - Base attack damage increased to 54 from 51.
- Ebb and Flow
  - Mana cost reduced to 70 / 75 / 80 / 85 / 90 from 70 / 80 / 90 / 100 / 110.
- Tidal Wave
  - Slow increased to 70% at all ranks from 50 / 60 / 70%.

### V14.7
- Ebb and Flow
  - Initial base damage reduced to 60 / 95 / 130 / 165 / 200 from 60 / 100 / 140 / 180 / 220.
  - Initial damage AP ratio reduced to 50% AP from 55% AP.
  - Initial base heal increased to 55 / 80 / 105 / 130 / 155 from 55 / 75 / 95 / 115 / 135.
  - Initial heal AP ratio increased to 40% AP from 25% AP.
  - Base bounce effectiveness reduced to -10% from -15%.
  - Bounce effectiveness AP ratio increased to 10% per 100 AP from $7.5$% per 100 AP.
- Tidecaller's Blessing
  - Base damage per hit reduced to 20 / 30 / 40 / 50 / 60 from 20 / 35 / 50 / 65 / 80.
    - Total base damage reduced to 60 / 90 / 120 / 150 / 180 from 60 / 105 / 150 / 195 / 240.

### V14.2
- Tidal Wave
  - **New Effect:** Missile now spawns from slightly behind her.

### V13.21
- Tidecaller's Blessing
  - Damage now triggers **Nami**’s own damage effects but still grants kill credit to the ally applying it.
- Nami
  - Tidal Wave
    - **Bug Fixes:** Ability SFX is now properly audible when it is launched from the fog of war.

### V13.20
- General
  - **Undocumented:** **Bug Fixes:** **Deep Sea Nami** voice lines and VFX are no longer global.

### V13.18
- General
  - Updated ability icons.

### V13.15
- Surging Tides
  - Base bonus movement speed increased to 100 from 90.
  - AP ratio increased 25% AP from 20% AP.
- Aqua Prison
  - Base damage increased to 90 / 145 / 200 / 255 / 310 from 75 / 130 / 185 / 240 / 295.

## Trivia

- Nami's backstory resembles Dinotopia.
- When playing Nami and typing 'hat' in the store search bar the result displayed will be Boots of Speed, referencing The Little Mermaid.

---
*This page was automatically generated from League of Legends Wiki data.*