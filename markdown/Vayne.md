# Vayne

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
| **Champion** | Vayne |
| **Title** | the Night Hunter |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2011-05-10 |
| **Release Patch** | V1.0.0.118 |
| **Latest Changes** | V25.17 |
| **Roles** | Marksman |
| **Riot Positions** | Bottom |
| **External Positions** | Top, Bottom |
| **Blue Essence** | 1575 |
| **Riot Points** | 790 |
| **Difficulty** | 2 |
| **Hero Type** | Marksman |
| **Alt Type** | Assassin |
| **Adaptive Type** | Physical |
| **Damage** | 3 |
| **Toughness** | 1 |
| **Control** | 2 |
| **Mobility** | 2 |
| **Utility** | 1 |
| **Style** | 0 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $550.0$ | $+103.0$ |
| **Mana** | $232.0$ | $+35.0$ |
| **Health Regen** | $3.5$ | $+0.55$ |
| **Mana Regen** | $7.0$ | $+0.4$ |
| **Armor** | $23.0$ | $+4.6$ |
| **Magic Resist** | $30.0$ | $+1.3$ |
| **Attack Damage** | $60.0$ | $+2.35$ |
| **Attack Speed** | $0.658$ | |
| **Movement Speed** | $330.0$ | $+0.0$ |
| **Attack Range** | $550.0$ | $+0.0$ |
| **Base Attack Speed** | $0.658$ | |
| **Attack Speed Ratio** | $0.658$ | |
| **Bonus AS per Level** | $3.3\%$ | |
| **Missile Speed** | $2000$ units/second | |
| **Acquisition Radius** | $575$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $120$ units | |
| **Selection Height** | $155$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $100.0\%$ |
| **Damage Taken** | $95.0\%$ |

#### URF

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $95.0\%$ |
| **Damage Taken** | $105.0\%$ |

## Abilities

### Passive: Night Hunter

| Attribute | Value |
|-----------|------:|
| **Effect Radius** | 2000 (Enemy champion check) units |
| **Targeting** | Passive |
| **Affects** | Self |

**INNATE:** **Vayne** gains ms while facing a nearby visible enemy champion, increased to 90 while under the effects of *Final Hour*.

**Notes:**

- The **bonus** movement speed persists for 2 seconds after **Vayne** loses sight of an enemy champion.

---

### Q: Tumble

| Attribute | Value |
|-----------|------:|
| **Range** | 300 (Dash distance) units |
| **Cast Time** | none |
| **Speed** | 500 + units/second |
| **Cost** | 30 Mana |
| **Cooldown** | 6 / 5 / 4 / 3 / 2 seconds |
| **Cooldown Start** | post-effect |
| **Targeting** | Direction |
| **Affects** | Self |
| **Damage Type** | physical |
| **Spell Effects** | spell |
| **Parry** | True |
| **Grounded** | True |
| **Knockdown** | True |

**ACTIVE:** **Vayne** dashes a fixed distance in the target direction, though not through terrain, and empowers her next basic attack within 3 seconds to have an uncancelable windup and deal **bonus** physical damage.

| Attribute | Value |
|-----------|------:|
| **Bonus Physical Damage** | 75 / 85 / 95 / 105 / 115% AD (+ 50% AP) |

*Tumble resets **Vayne**’s basic attack timer.*

**Notes:**

- The bonus damage applies life steal.
- Although *Tumble* will reset the basic attack timer, **Vayne** cannot attack until the animation is complete.
- **Vayne** can cast Final Hour during *Tumble*. She does not become invisible during the dash.

---

### W: Silver Bolts

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Enemies |
| **Damage Type** | True |
| **Spell Shield** | False |
| **Spell Effects** | Proc |
| **Parry** | True |

**PASSIVE:** **Vayne**’s basic attacks on-hit and *Condemn* apply a stack of *Silver Bolts* for $3.5$ seconds, refreshing on subsequent applications, expiring upon attacking a new enemy, and stacking up to 3 times.

The third stack consumes them all to deal **bonus** true damage, with a minimum threshold.

| Attribute | Value |
|-----------|------:|
| **Bonus True Damage** | 6 / 7 / 8 / 9 / 10% of target's **maximum** health |

| Attribute | Value |
|-----------|------:|
| **Minimum Bonus Damage** | 50 / 65 / 80 / 95 / 110 |

*Silver Bolts* deals modified damage against monsters.

| Attribute | Value |
|-----------|------:|
| **Monster Damage** | 140 / 155 / 170 / 185 / 200 |

**Notes:**

- Stacks are not removed if **Vayne** switches to attacking a target immune to *Silver Bolts* (structures, wards).
- *Silver Bolts* do not apply to additional targets with Runaan's Hurricane.
- The stacks can be consumed even if the target is in stasis.

---

### E: Condemn

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 550 units |
| **Speed** | 2200 (Missile speed) / 2000 (Knockback speed) units/second |
| **Cost** | 90 Mana |
| **Cooldown** | 20 / 18 / 16 / 14 / 12 seconds |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | physical |
| **Spell Shield** | True |
| **Spell Effects** | spell |
| **Projectile** | True |
| **Call For Help** | True |

**ACTIVE:** **Vayne** fires a heavy bolt at the target enemy that deals physical damage and knocks them back 475 units, though not through terrain.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 50 / 85 / 120 / 155 / 190 (+ 50% **bonus** AD) |

If the target collides with terrain, they take **bonus** physical damage and become stunned for 1.5 seconds.

| Attribute | Value |
|-----------|------:|
| **Bonus Physical Damage** | 75 / 127.5 / 180 / 232.5 / 285 (+ 75% **bonus** AD) |
| **Total Physical Damage** | 125 / 212.5 / 300 / 387.5 / 475 (+ 125% **bonus** AD) |

**Notes:**

- *Condemn*’s displacement direction is determined at the end of the cast time.
  - Because of this, **Vayne** may use Flash during the cast time to create a better angle.
- Cleansing the knock back will also end the displacement early.
- The spell indicator for this ability also displays the direction for the knock back relative to **Vayne**’s position.
- *Condemn*’s stun duration starts when **Vayne**’s target collides with a wall (they can be immobilized for up to 2 seconds depending on displacement duration based on distance traveled).
- *Condemn*’s missile will fail to fire if **Vayne** is suppressed during the cast time.
- *Condemn* can interact with player-generated terrain.

---

### R: Final Hour

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Cost** | 80 Mana |
| **Cooldown** | 100 / 92.5 / 85 / 77.5 / 70 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Auto |
| **Affects** | Self |

**ACTIVE:** **Vayne** equips her crossbow, gaining **bonus** attack damage for a duration.

| Attribute | Value |
|-----------|------:|
| **Effect Duration** | 8 / 9 / 10 / 11 / 12 seconds |

| Attribute | Value |
|-----------|------:|
| **Bonus Attack Damage** | 35 / 50 / 65 |

While active, *Final Hour* empowers *Night Hunter* to have tripled **bonus** movement speed and *Tumble* to have a reduced cooldown. Additionally, *Tumble’s* cast grants **Vayne** invisibility for 1 second. Attacking or casting abilities ends the stealth immediately.

| Attribute | Value |
|-----------|------:|
| **Tumble Cooldown Reduction** | 30 / 35 / 40 / 45 / 50% |

Scoring an enemy champion takedown within 3 seconds of damaging them will extend *Final Hour*’s duration by 4 seconds, up to its original duration.

**Notes:**

- **Vayne**’s animations and basic attack projectiles change slightly during *Final Hour*.
- *Tumble*’s visual effects can be seen by enemies upon **Vayne** becoming invisible.
- Using a basic attack breaks the stealth at the start of the attack windup.

---

## Patch History

### V25.17
- Silver Bolts
  - **New Effect:** Damage is now modified to 140 / 155 / 170 / 185 / 200 against monsters.
    - **OLD EFFECT:** Damage is capped at 200 against monsters.

### V25.15
- Condemn
  - **Bug Fixes:** Casting Flash during *Condemn*’s cast time no longer causes her to face the wrong direction.

### V14.22
- Final Hour
  - Attack damage increased to 35 / 50 / 65 from 25 / 40 / 55.

### V14.9
- General
  - Adjusted splash artwork for Vayne and Vayne..

### V14.6
- Tumble
  - **Bug Fixes:** Animation no longer sometimes stutters upon issuing movement commands at a very used during Final Hour.

### V14.5
- Night Hunter
  - Bonus movement speed reduced to 30 from 45.
- Tumble
  - Cooldown increased to 6 / 5 / 4 / 3 / 2 seconds from 4 / 3.5 / 3 / 2.5 / 2.

### V14.2
- Tumble
  - **New Effect:** Empowered attack's windup is now uncancelable.

### V13.20
- Night Hunter
  - **Bug Fixes:** Movement speed buff is now properly displayed in the buff bar when it is active.

### V13.18
- General
  - Updated ability icons.

### V13.15
- Final Hour
  - **Bug Fixes:** Casting Tumble now properly triggers the invisibility.

## Trivia

- Vayne is voiced.md) by Nika Futterman.
- Vayne's dance references gun fu, also known as the Gun Kata.
  - A side-by-side comparison can be seen here.
- Silver Bolts references werewolf fiction, possibly inspired by silver's & other heavy metals' Oligodynamic effect.
- Night Hunter, The Darkin Blade, Grandmaster-at-Arms, and The Relentless Storm are the only abilities in *League of Legends* that have the same name as their champion's title.
- As per her retconned lore, Vayne hunts and kills monsters, hybrids, shape-shifters, etc. not due to any ethical guidelines, but to satisfy her bloodlust, making her resemble , a fictional serial killer who mostly targets other morally deplorable murderers.
  - Similarly, Vayne & Dexter both developed killing urge after witnessing their respective parents being murdered (Vayne's father & mother; Dexter's ).
- Vayne ,Warwick and Trundle are the only champions without damaging area-of-effect abilities.
- Vayne was nearly cancelled due to her concept artist unable to properly concept her with a big hat.

---
*This page was automatically generated from League of Legends Wiki data.*