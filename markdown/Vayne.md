# Vayne

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
| **Champion** | Vayne |
| **Title** | the Night Hunter |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2011-05-10 |
| **Release Patch** | V1.0.0.118 |
| **Roles** | Marksman |
| **Riot Positions** | Bottom |
| **External Positions** | Top, Bottom |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $550.0$ | $+103.0$ | $2301.0$ |
| **Mana** | $232.0$ | $+35.0$ | $827.0$ |
| **Health Regen** | $3.5$ | $+0.55$ | $12.9$ |
| **Mana Regen** | $7.0$ | $+0.4$ | $13.8$ |
| **Armor** | $23.0$ | $+4.6$ | $101.2$ |
| **Magic Resist** | $30.0$ | $+1.3$ | $52.1$ |
| **Attack Damage** | $60.0$ | $+-0.4$ | $53.2$ |
| **Attack Speed** | $0.658$ | $+-0.3\%$ | $0.624$ |
| **Movement Speed** | $330.0$ | $+0.0$ | $330.0$ |
| **Attack Range** | $550.0$ | $+0.0$ | $550.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.658$ |
| **Attack Speed Ratio** | $0.658$ |
| **Bonus AS per Level** | $-0.3\%$ |
| **Missile Speed** | $2000 units/second$ |
| **Acquisition Radius** | $575 units$ |
| **Pathing Radius** | $35 units$ |
| **Selection Radius** | $120 units$ |
| **Selection Height** | $155 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Night Hunter

**Innate:** **Vayne** gains *ms **bonus** movement speed* while facing a nearby enemy champion, significantly increased during **Final Hour**.

**Innate:** **Vayne** gains ms*bonus** movement speed* while facing a nearby sight enemy champion, increased to 90 while under the effects of **Final Hour**.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Self |
| **Effect Radius** | 2000 units |

**Notes:**

- The **bonus** movement speed persists for 2 seconds after **Vayne** loses sight of an enemy champion.

---

### Q: Tumble

**Active:** **Vayne** dash in the target direction. Her next basic attack within a few seconds will deal **bonus** physical damage.

**Active:** **Vayne** dash a fixed distance in the target direction, though not through terrain, and empowers her next basic attack within 3 seconds to have an uncancelable windup and deal **bonus** physical damage. *Tumble basic attack reset *'Vayne's* basic attack timer.*

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $6-2$ seconds |
| **Cast Time** | none |
| **Cost** | 30 Mana |
| **Targeting** | Direction |
| **Affects** | Self |
| **Damage Type** | physical |
| **Speed** | 500 + Dash speed units/second |
| **Spell Effects** | spell |

**Scaling:**
- **Bonus Physical Damage:** $75-115$% AD (+ 50% AP)

**Notes:**

- The bonus damage applies .
- Although *Tumble* will reset the basic attack timer, **Vayne** cannot attack until the animation is complete.
- **Vayne** can cast *Final Hour* during *Tumble*. She does not become invisible during the dash.

---

### W: Silver Bolts

**Passive:** **Vayne**’s basic attacks on-hit and **Condemn** will apply a stack of *Silver Bolts*, which expires upon attacking a new enemy.

*The third stack will consume them all to deal **bonus** true damage based on the target's **maximum** health.*

**Passive:** ''Vayne's* basic attacks on-hit and **Condemn** apply a stack of *Silver Bolts' for $3.5$ seconds, refreshing on subsequent applications, expiring upon attacking a new enemy, and stacking up to 3 times. The third stack consumes them all to deal **bonus true damage**, with a minimum threshold. *Silver Bolts* deals modified damage against monsters.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Enemies |
| **Damage Type** | True |
| **Spell Shield** | False |
| **Spell Effects** | Proc |

**Scaling:**
- **Bonus True Damage:** $6-10$% of target's
- **maximum** health
- **Minimum Bonus Damage:** $50-110$
- **Monster Damage:** $140-200$

**Notes:**

- Stacks are not removed if **Vayne** switches to attacking a target immune to *Silver Bolts* (structures, wards).
- *Silver Bolts* do not apply to additional targets with *Runaan's Hurricane*.
- The stacks can be consumed even if the target is in stasis.

---

### E: Condemn

**Active:** **Vayne** fires a heavy bolt at the target enemy that deals physical damage and airborne.

*If the target hits terrain, they are briefly stun and dealt **bonus** physical damage.*

**Active:** **Vayne** fires a heavy bolt at the target enemy that deals physical damage and airborne 475 units, though not through terrain. If the target collides with terrain, they take **bonus** physical damage and become stun for $1.5$ seconds.

| Attribute | Value |
|-----------|-------|
| **Range** | 550 units |
| **Cooldown** | $20-12$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | 90 Mana |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | physical |
| **Speed** | 2200 / 2000 units/second |
| **Spell Shield** | True |
| **Spell Effects** | spell |
| **Projectile** | True |

**Scaling:**
- **Physical Damage:** $50-190$ bonus AD)
- **Bonus Physical Damage:** $50×1.5-190×1.5$ (+ 75% bonus AD)2.5-190×2.5$ (+ 125% bonus AD)

**Notes:**

- 'Condemn's airborne direction is determined at the end of the cast time.
  - Because of this, **Vayne** may use Flash during the cast time to create a better angle.
- Cleanse the airborne will also end the displacement early.
- The spell indicator for this ability also displays the direction for the knock back relative to ''Vayne's' position.
- 'Condemn's* stun duration starts when *'Vayne's' target collides with a wall (they can be immobilized for up to 2 seconds depending on airborne duration based on distance traveled).
- 'Condemn's missile will fail to fire if **Vayne** is suppression during the cast time.
- *Condemn* can interact with player-generated terrain.

---

### R: Final Hour

**Active:** **Vayne** equips her crossbow for a long duration, gaining *attack damage*.

*During *Final Hour*, **Night Hunter** grants increased *movement speed*, and *Tumble* is empowered to have a cdr cooldown and grant **Vayne** invisibility.*

**Active:** **Vayne** equips her crossbow, gaining **bonus attack damage** for a duration. While active, *Final Hour* empowers **Night Hunter** to have tripled **bonus** movement speed and **Tumble** to have a reduced *cooldown*. Additionally, **Tumble*’s* cast grants **Vayne** invisibility for 1 second. Attacking or casting abilities ends the stealth immediately. Scoring an enemy champion takedown within 3 seconds of damaging them will extend 'Final Hour's duration by 4 seconds, up to its original duration.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $100-70$ seconds |
| **Cast Time** | none |
| **Cost** | 80 Mana |
| **Targeting** | Auto |
| **Affects** | Self |

**Scaling:**
- **Effect Duration:** $8-12$ seconds
- **Bonus Attack Damage:** $35-65 3$
- **

**Notes:**

- ''Vayne's* animations and basic attack projectiles change slightly during *Final Hour'.
- 'Tumble's visual effects can be seen by enemies upon **Vayne** becoming invisible.
- Using a basic attack breaks the stealth at the start of the attack windup.

---

## Patch History

### V25.17
- *Silver Bolts*
  - **New Effect:** Damage is now modified to $140-200$ against monsters.
    - **Old Effect:** Damage is capped at 200 against monsters.

### V25.15
- *Condemn*
  - **Bug Fixes:** Casting Flash during 'Condemn's cast time no longer causes her to face the wrong direction.

### V14.22
- *Final Hour*
  - Attack damage increased to $35-65 3$ from $25-55 3$.

### V14.9
- General
  - Adjusted splash artwork for Vayne and Vayne..

### V14.6
- *Tumble*
  - **Bug Fixes:** Animation no longer sometimes stutters upon issuing movement commands at a very used during *Final Hour*.

### V14.5
- *Night Hunter*
  - Bonus movement speed reduced to 30 from 45.
- *Tumble*
  - Cooldown increased to $6-2$ seconds from $4-2$.

### V14.2
- *Tumble*
  - **New Effect:** Empowered attack's windup is now uncancelable windup.

### V13.20
- *Night Hunter*
  - **Bug Fixes:** Movement speed buff is now properly displayed in the buff bar when it is active.

### V13.18
- General
  - Updated ability icons.

### V13.15
- *Final Hour*
  - **Bug Fixes:** Casting *Tumble* now properly triggers the invisibility.

## Trivia

- Vayne is voiced.md) by Nika Futterman.
- Vayne's dance references gun fu, also known as the Gun Kata.
  - A side-by-side comparison can be seen here.
- *Silver Bolts* references werewolf fiction, possibly inspired by silver's & other heavy metals' Oligodynamic effect.
- Night Hunter, The Darkin Blade, Grandmaster-at-Arms, and The Relentless Storm are the only abilities in *League of Legends* that have the same name as their champion's title.
- As per her retconned lore, Vayne hunts and kills monsters, hybrids, shape-shifters, etc. not due to any ethical guidelines, but to satisfy her bloodlust, making her resemble , a fictional serial killer who mostly targets other morally deplorable murderers.
  - Similarly, Vayne & Dexter both developed *killing urge* after witnessing their respective parents being murdered (Vayne's father & mother; Dexter's ).
- **Vayne** ,**Warwick** and **Trundle** are the only champions without damaging area-of-effect abilities.
- Vayne was nearly cancelled due to her concept artist unable to properly concept her with a big hat.

---
*This page was automatically generated from League of Legends Wiki data.*