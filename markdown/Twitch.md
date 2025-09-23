# Twitch

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
| **Champion** | Twitch |
| **Title** | the Plague Rat |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2009-05-01 |
| **Release Patch** | May 1, 2009 Patch |
| **Latest Changes** | V25.13 |
| **Roles** | Marksman |
| **Riot Positions** | Bottom |
| **External Positions** | Bottom, Support |
| **Blue Essence** | 1575 |
| **Riot Points** | 790 |
| **Difficulty** | 2 |
| **Hero Type** | Marksman |
| **Alt Type** | Assassin |
| **Adaptive Type** | Physical |
| **Damage** | 3 |
| **Toughness** | 1 |
| **Control** | 1 |
| **Mobility** | 2 |
| **Utility** | 1 |
| **Style** | 0 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $630.0$ | $+104.0$ |
| **Mana** | $300.0$ | $+40.0$ |
| **Health Regen** | $3.75$ | $+0.6$ |
| **Mana Regen** | $7.25$ | $+0.7$ |
| **Armor** | $27.0$ | $+4.2$ |
| **Magic Resist** | $30.0$ | $+1.3$ |
| **Attack Damage** | $59.0$ | $+3.1$ |
| **Attack Speed** | $0.679$ | |
| **Movement Speed** | $330.0$ | $+0.0$ |
| **Attack Range** | $550.0$ | $+0.0$ |
| **Base Attack Speed** | $0.679$ | |
| **Attack Speed Ratio** | $0.679$ | |
| **Bonus AS per Level** | $3.4\%$ | |
| **Missile Speed** | $2500$ units/second | |
| **Acquisition Radius** | $575$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $135$ units | |
| **Selection Height** | $120$ units | |
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
| **Damage Dealt** | $85.0\%$ |
| **Damage Taken** | $110.0\%$ |

## Abilities

### Passive: Deadly Venom

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Enemies |
| **Damage Type** | True |
| **Spell Shield** | False |
| **Spell Effects** | default |
| **Parry** | Special |

**INNATE:** **Twitch**’s basic attacks on-hit apply a stack of *Deadly Venom* for 6 seconds, refreshing on subsequent applications and stacking up to 6 times.

**DEADLY VENOM:** For each stack, the target is dealtfor a maximum ofThis effect is considered a poison.

**Notes:**

- The first 5 stacks on a target are indicated each by a small mark around them, while a target affected by the maximum stacks of 6 is indicated by a single large mark above them instead.
- *Deadly Venom* does not affect structures.
- Being applied on-hit, *Deadly Venom* stacks will still be applied if the attack was parried or blocked, but not if dodged and/or missed if **Twitch** is blinded.

---

### Q: Ambush

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Cost** | 40 Mana |
| **Cooldown** | 16 seconds |
| **Cooldown Start** | post-effect |
| **Targeting** | Auto |
| **Affects** | Self |
| **Detection Radius** | 500 |

**ACTIVE:** After a 1-second delay, **Twitch** becomes camouflaged for a duration. Attacking or casting *Venom Cask* or *Contaminate* ends *Ambush* immediately.

| Attribute | Value |
|-----------|------:|
| **Stealth Duration** | 10 / 11 / 12 / 13 / 14 seconds |

During this time, **Twitch** gains ms, increased to 30% while facing enemy champions within a 1000-unit radius who cannot see him.

Upon breaking stealth, **Twitch** gains **bonus** attack speed for 6 seconds.

| Attribute | Value |
|-----------|------:|
| **Bonus Attack Speed** | 45 / 50 / 55 / 60 / 65% |

When an enemy champion dies while afflicted with *Deadly Venom*, *Ambush*’s cooldown is reset.

**Notes:**

- Entering stealth cancels **Twitch**’s current basic attack.
- *Ambush* follows the same rules as stealth but he can still perform actions normally before entering camouflage. Activating Recall during the 1-second delay allows him to channel it while stealthed.
- If **Twitch** enters stasis during the delay, he will gain the camouflage after the stasis ends.
- Using a basic attack breaks the stealth at the start of the attack windup.

---

### W: Venom Cask

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 950 units |
| **Effect Radius** | 300 units |
| **Speed** | 1400 units/second |
| **Cost** | 70 Mana |
| **Cooldown** | 13 / 12 / 11 / 10 / 9 seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Spell Shield** | True |
| **Projectile** | True |
| **Out of Range Behavior** | walk to location |

**ACTIVE:** **Twitch** hurls a cask of venom that explodes at the target location, applying *Deadly Venom* to enemies hit and granting sight of the area.

The area then becomes contaminated for 3 seconds, applying a *Deadly Venom* stack each second to enemies within and slowing them.

| Attribute | Value |
|-----------|------:|
| **Slow** | 30 / 35 / 40 / 45 / 50% (+ 6% per 100 AP) |

**Notes:**

- *Venom Cask* can apply a maximum of 4 Deadly Venom stacks per enemy per cast.
- *Venom Cask*’s missile will fail to fire if **Twitch** is suppressed during the cast time.

---

### E: Contaminate

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Effect Radius** | 1200 units |
| **Cost** | 50 / 60 / 70 / 80 / 90 Mana |
| **Cooldown** | 12 / 11 / 10 / 9 / 8 seconds |
| **Targeting** | Proximity |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | Aoe |
| **Projectile** | False |

**ACTIVE:** **Twitch** sends out a lethal toxin to each nearby enemy afflicted by *Deadly Venom*, dealing them physical damage.

| Attribute | Value |
|-----------|------:|
| **Base Physical Damage** | 20 / 30 / 40 / 50 / 60 |

*Contaminate* deals additional physical damage and 35% AP magic damage for each stack of *Deadly Venom* on the target.

| Attribute | Value |
|-----------|------:|
| **Physical Damage Per Stack** | 15 / 20 / 25 / 30 / 35 (+ 35% **bonus** AD) |
| **Minimum Mixed Damage** | 35 / 50 / 65 / 80 / 95 (+ 35% **bonus** AD) (+ 35% AP) |
| **Maximum Mixed Damage** | 110 / 150 / 190 / 230 / 270 (+ 210% **bonus** AD) (+ 210% AP) |

*A nearby enemy with Deadly Venom is required to cast this ability. The target does not have to be visible to be targeted by this ability.*

**Notes:**

- *Contaminate* will deal the additional damage to targets based on the number of Deadly Venom stacks they had at the start of the cast time.
- **Twitch** is given a range indicator for *Contaminate*’s radius upon infecting an enemy champion with Deadly Venom (actual range is slightly larger than shown by the indicator).
- *Contaminate* will not deal damage to enemies that are not within range of the ability before the cast time completes.
  - If the target moves out of range after the cast time, they are still dealt the damage.

---

### R: Spray and Pray

| Attribute | Value |
|-----------|------:|
| **Range** | 1100 (bolt travel distance, further increased by Rapid Firecannon) units |
| **Cast Time** | none |
| **Target Range** | er Twitch's range |
| **Width** | 120 (bolt width) units |
| **Speed** | 5000 units/second |
| **Cost** | 100 Mana |
| **Cooldown** | 90 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Auto |
| **Affects** | Self |
| **Damage Type** | Physical |
| **Spell Effects** | basic |
| **Projectile** | True |
| **Parry** | True |
| **Call For Help** | True |

**ACTIVE:** **Twitch** gains **bonus** attack damage and range for 6 seconds, during which his basic attacks are replaced by *bolts* that travel slightly further than his attack range in a straight line, dealing damage to every enemy unit (non-champions and champions, wards and plants, turrets and structures) hit.

| Attribute | Value |
|-----------|------:|
| **Bonus Attack Damage** | 30 / 37.5 / 45 / 52.5 / 60 |

The *bolts* deal key=% of the triggering attack's damage, apply on-hit effects, and can critically strike for damage.

**Notes:**

- The extra distance that the *bolts* travel scales with **bonus** attack range.
- *Spray and Pray* allows **Twitch** to outrange turrets by 50 units, allowing him to hit them without them returning fire.
- **Twitch** targets his line attack at his target's location at the *start* of his attack windup.
  - The *bolts* reach targets at a delay, composed of **Twitch**’s windup time and distance the *bolt* has to travel at finite speed. Even the primary target can dodge the attack if they are moving quickly enough.
- The *bolt* projectiles will naturally pass through terrain and enemy structures (the latter will be damaged by *bolts* even if they are not directly targeted).
- If **Twitch** is blinded before winding up the attack, the hits will *miss* against **all** targets.
- Runaan's Hurricane Wind's Fury interacts with *Spray and Pray*’s **bonus** attack range but not with the modified missile effect (the secondary bolts will not have pass-through effects).
- Whenever the *bolts* penetrate a target, a small elongated cloud appears at the location, which's VFX and SFX **can** be seen and heard inside the Fog of War.
- The *bolts** travel distance scales with the full value of **Twitch**’s range increases such as Rapid Firecannon, but not with increases (which only increase his effective attack range, thereby the target range).
- (Outdated as of V10.13, now can hit everything with edge range, except for turrets) The cr center of a unit must be within the maximum travel distance of the *bolt* missile, and in front of the spawn location (**Twitch**’s cr center) for the *bolt* to be able to hit them.
  - Other than this condition, the *bolt* missile has to only touch (pass within its half width of 60 units) the er edge of the unit's radius.
  - This is standard behaviour for *linear skillshots*.
- Malignance Hatefog is special cased to work with *Spray and Pray*.
- Axiom Arcanist amplifies *bolt* damage as area of effect.

---

## Patch History

### V25.13
- Ambush
  - **Bug Fixes:** Starting the charge of

### Rift Herald#Summoned form|Rift Herald Rodeo
    - *This bug fix had been noted before on patch

### V25.11
- Contaminate
  - AP ratio per stack increased to 35% AP from 30% AP.
    - Maximum AP ratio increased to 210% AP from 180% AP.

### V25.06
- Contaminate
  - **Bug Fixes:** If there are multiple targets affected by Deadly Venom and the ability is cast shortly before any target's Deadly Venom would expire, no longer always applies the would-be damage from the highest number of Deadly Venom stacks on targets affected by the lowest number of Deadly Venom stacks and vice versa.

### V25.04
- Spray and Pray
  - **Bug Fixes:** Now counts as an area of effect spell for Axiom Arcanist instead of single-target.

### V25.S1.3
- Deadly Venom
  - **Bug Fixes:** Fixed an issue that prevented it from triggering Runaan's Hurricane Wind's Fury bolts.

### V25.S1.1
- Venom Cask
  - Slow AP ratio increased to 6% per 100 AP from 5% per 100 AP.

### V14.24
- Ambush
  - Bonus attack speed increased to 45 / 50 / 55 / 60 / 65% from 40 / 45 / 50 / 55 / 60%.
  - Attack speed duration increased to 6 seconds from 5.

### V14.12
- Spray and Pray
  - **Bug Fixes:** Fixed a bug that caused attacks that tag multiple targets to not receive the damage amp from Press the Attack.

### V14.9
- Stats
  - Base health reduced to 630 from 682.
  - Health growth increased to 104 from 100.
- Venom Cask
  - Slow AP ratio reduced to 5% per 100 AP from 6% per 100 AP.
- Spray and Pray
  - Bonus attack damage reduced to 30 / 45 / 60 from 40 / 55 / 70.

## Trivia

- Twitch was voiced.md) by the late Doug Boyd.
  - Twitch is voiced by an unknown voice actor in the Wild Rift Chat shorts.
- Deadly Venom displaying an X when fully stacked on a target might be referencing the Black Death (the doors of those afflicted were marked as such).

---
*This page was automatically generated from League of Legends Wiki data.*