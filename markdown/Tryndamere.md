# Tryndamere

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
| **Champion** | Tryndamere |
| **Title** | the Barbarian King |
| **Resource** | Fury |
| **Range Type** | Melee |
| **Release Date** | 2009-05-01 |
| **Release Patch** | May 1, 2009 Patch |
| **Latest Changes** | V14.20 |
| **Roles** | Skirmisher |
| **Riot Positions** | Top |
| **External Positions** | Top |
| **Blue Essence** | 675 |
| **Riot Points** | 585 |
| **Difficulty** | 1 |
| **Hero Type** | Fighter |
| **Alt Type** | Assassin |
| **Adaptive Type** | Physical |
| **Damage** | 3 |
| **Toughness** | 2 |
| **Control** | 1 |
| **Mobility** | 2 |
| **Utility** | 1 |
| **Style** | 10 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $696.0$ | $+108.0$ |
| **Mana** | $100.0$ | $+0.0$ |
| **Health Regen** | $8.5$ | $+0.9$ |
| **Armor** | $33.0$ | $+4.8$ |
| **Magic Resist** | $32.0$ | $+2.05$ |
| **Attack Damage** | $66.0$ | $+4.0$ |
| **Attack Speed** | $0.670$ | |
| **Movement Speed** | $345.0$ | $+0.0$ |
| **Attack Range** | $175.0$ | $+0.0$ |
| **Base Attack Speed** | $0.67$ | |
| **Attack Speed Ratio** | $0.694$ | |
| **Bonus AS per Level** | $3.4\%$ | |
| **Acquisition Radius** | $400$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $135$ units | |
| **Selection Height** | $180$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $110.0\%$ |
| **Damage Taken** | $90.0\%$ |
| **Healing** | $120.0\%$ |

## Abilities

### Passive: Battle Fury

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Self |

**INNATE:** **Tryndamere** generates 5 Fury on his basic attacks, and 10 Fury on critical strikes and each time he kills an enemy. After 8 seconds without dealing or taking damage, **Tryndamere** loses 5 Fury per second.

**Tryndamere** gains (critical strike chance) 0%–50%@0–100 (@=Fury) critical strike chance.

**Notes:**

- Fury generation stacks additively: if the given action triggers multiple ways of generating Fury, all of them apply.
  - Killing an enemy with a basic attack basic attack generates 15 Fury (5 Fury from the basic attack + 10 Fury from the kill).
  - Killing an enemy with a critical strike generates 20 Fury (10 Fury from the critical strike + 10 Fury from the kill).
  - The Fury gain for killing an enemy also stacks with Spinning Slash's Fury generation per enemy hit.
- Attacks against structures will not grant bonus Fury, but will still reset the timer on Fury decay.
- Attacks against wards behave like attacks on normal enemies, generating 5 Fury and resetting the decay timer. Attacks against wards can critically strike, generating 10 Fury.
- Attacks against jungle plants will generate 5 Fury and reset the decay timer. Attacks can also critically strike, generating 10 Fury. Additionally, Honeyfruit drops restore 5 Fury for each pod collected and reset the decay timer.
  - Attacking an Honeyfruit and collecting the pods generates 30 Fury, or 35 Fury if the plant was hit by a critical strike.
- Fury is only granted if the basic attack hits and will not be granted if **Tryndamere** cancels his basic attack windup.

---

### Q: Bloodlust

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Cost** | 100% **Current** Fury |
| **Cooldown** | 12 seconds |
| **Targeting** | Auto |
| **Affects** | Self |

**PASSIVE:** **Tryndamere** gains **bonus** attack damage, plus an additional amount based on his **missing** health.

| Attribute | Value |
|-----------|------:|
| **Bonus Attack Damage** | 5 / 10 / 15 / 20 / 25 |

| Attribute | Value |
|-----------|------:|
| **Additional Bonus AD** | 0.15 / 0.25 / 0.35 / 0.45 / 0.55 per 1% **missing** health |
| **Maximum Total Bonus AD** | 20 / 35 / 50 / 65 / 80 |

**ACTIVE:** **Tryndamere** consumes all of his Fury to heal himself, increased for every point of Fury consumed.

| Attribute | Value |
|-----------|------:|
| **Minimum Heal** | 30 / 40 / 50 / 60 / 70 (+ 30% AP) |
| **Heal Per 1 Fury** | 0.5 / 0.95 / 1.4 / 1.85 / 2.3 (+ $1.2$% AP) |
| **Maximum Heal** | 80 / 135 / 190 / 245 / 300 (+ 150% AP) |

**Notes:**

- *Bloodlust* can still be activated even if **Tryndamere** does not have any **Fury**.
- *Bloodlust* can be activated at any point of **Tryndamere**’s basic attack.
  - Note that if an basic attack will critically strike is determined when the basic attack starts.
    - This means that if Bloodlust is cast during Tryndamere's basic attack windup, the attack will use the critical strike chance given by the Fury **before** the Bloodlust cast.
    - Furthermore, the Fury for that basic attack will be given once the windup completes. This lets **Tryndamere** start building Fury immediately.

---

### W: Mocking Shout

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.3$ seconds |
| **Effect Radius** | 850 units |
| **Cooldown** | 14 seconds |
| **Targeting** | Proximity |
| **Affects** | Enemies |
| **Spell Shield** | True |

**ACTIVE:** **Tryndamere** reduces the **bonus** attack damage of nearby enemy champions for 4 seconds.

| Attribute | Value |
|-----------|------:|
| **Attack Damage Reduction** | 20 / 35 / 50 / 65 / 80 |

Targets facing in the opposite direction of **Tryndamere** are also slowed for the duration.

| Attribute | Value |
|-----------|------:|
| **Slow** | 30 / 37.5 / 45 / 52.5 / 60% |

*A nearby enemy champion is required to cast this ability. The target does not have to be visible to be targeted by this ability.*

**Notes:**

- If the attack damage reduction would reduce the target's **bonus** attack damage below 0, the target's **base** attack damage value is unaffected, but their **total** attack damage will still be reduced. The target will however retain a **bonus** attack damage value of 0.
  - Effects that scale with **total** attack damage, such as basic attacks, will take the attack damage reduction into account for their calculations.
    - Those that have a ratio for **base** attack damage or **bonus** attack damage do not take the reduction into account.

---

### E: Spinning Slash

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 660 units |
| **Effect Radius** | 225 units |
| **Cooldown** | 12 / 11 / 10 / 9 / 8 seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | physical |
| **Spell Shield** | True |
| **Spell Effects** | aoe |
| **Out of Range Behavior** | cast at max |
| **Parry** | unknown |
| **Grounded** | True |
| **Knockdown** | True |

**ACTIVE:** **Tryndamere** dashes to the target location, dealing physical damage to enemies hit and generating 2 Fury per enemy hit, increased to 5 Fury against champions.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 75 / 105 / 135 / 165 / 195 (+ 130% **bonus** AD) (+ 80% AP) |

Critical strikes on-hit reduce *Spinning Slash*’s **current** cooldown by $0.75$ seconds, doubled to $1.5$ seconds against champions.

*Bloodlust and Undying Rage can be cast during the dash.*

**Notes:**

- *Spinning Slash*’s Fury generation stacks additively with Battle Fury's Fury generation on kill.
  - Killing an minion with *Spinning Slash* grants 12 Fury (2 Fury from enemy hit + 10 Fury from the kill).
  - Killing an champion with *Spinning Slash* grants 15 Fury (5 Fury from champion hit + 10 Fury from the kill).
- There is no cooldown on Fury generation on kill with *Spinning Slash*.
- *Spinning Slash* has no minimum dash range.
- Flash will interrupt the dash but *Spinning Slash* will deal damage to enemies at the new location instantly.
  - Enemies already hit by *Spinning Slash* cannot be damaged more than once.
- To ensure *Spinning Slash* deals damage when dashing away from an enemy champion, **Tryndamere** needs to move *r-35* units towards the enemy beyond the distance he can basic attack at, where *r* represents the enemy's radius and *35* is the distance in units beyond **Tryndamere**’s basic attack range that *Spinning Slash* can reach.
- The cooldown reduction upon critically striking triggers on wards and jungle plants.
- The cooldown reduction upon critically striking triggers even if the attack is blocked.
- : Cooldown reduction on critical strike interaction with *parrying* effects (dodge, blind).

---

### R: Undying Rage

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Cooldown** | 120 / 110 / 100 / 90 / 80 seconds |
| **Targeting** | Auto |
| **Affects** | Self |

**ACTIVE:** **Tryndamere** becomes enraged, instantly gaining Fury and a minimum health threshold for 5 seconds.

| Attribute | Value |
|-----------|------:|
| **Fury Gained** | 50 / 75 / 100 |

| Attribute | Value |
|-----------|------:|
| **Minimum Health Threshold** | 30 / 50 / 70 |

*Undying Rage can be used while affected by cast-inhibiting crowd control.*

**Notes:**

- *Undying Rage* will prevent Overwhelm’s stacks from being consumed.

---

## Patch History

### V14.20
- Stats
  - Attack speed ratio increased to $0.694$ from $0.67$.
  - Armor growth increased to $4.8$ from $4.3$.

### V14.12
- Bloodlust
  - Base bonus attack damage reduced to 5 / 10 / 15 / 20 / 25 from 10 / 15 / 20 / 25 / 30.
- Spinning Slash
  - Base damage reduced to 75 / 105 / 135 / 165 / 195 from 80 / 110 / 140 / 170 / 200.

### V14.10
- Battle Fury
  - Critical strike chance increased to 0%–50%@0–100 (@=Fury) from 0%–40%@0–100 (@=Fury).

### V14.6
- Stats
  - Attack speed growth increased to $3.4$% from $2.9$%.

### V13.21
- Spinning Slash
  - **Bug Fixes:** Hit radius is no longer smaller than intended.
    - *Note: This was hotfixed during the previous patch.*

### V13.20#October 11th Hotfix|V13.20
- Spinning Slash
  - **Bug Fixes:** Now has the correct hit radius of 225 on every rank.

### V13.20
- Spinning Slash
  - **UNDOCUMENTED/BUG FIX:** Now lists the proper Fury gain values on hitting champions and non-champions in the tooltip.
  - Rescripted to use DataValues.
    - Hitbox radiuses reduced to cr 80 / 110 / 140 / 170 / 200 from cr 225.

### V13.18
- Stats
  - Base attack damage reduced to 66 from 68.
  - Health growth reduced to 108 from 115.

### V13.17
- Stats
  - Attack range increased to 175 units from 125.
  - Base attack damage reduced to 68 from 72.

### V13.5
- Stats
  - Health growth increased to 115 from 112.
  - Attack damage growth increased to 4 from $3.7$.

## Trivia

- Tryndamere holds his sword in a similar fashion from holds his knife.
- Tryndamere's dance references the Hopak, the traditional Cossack dance.
  - His pre-V1.0.0.122 dance references Bye Bye Bye by NSYNC.
    - A side-by-side comparison can be seen here.
    - He used to share this dance with Garen before they both got reworked.
- Tryndamere was the first manaless champion released.
  - He was also:
    - The first champion to utilize a form of the Fury resource.
    - The first with 100 points in Attack Rating.
- With Spinning Slash, Tryndamere is one of the only four champions in the game to have a single damaging ability, the others being Zilean with Time Bomb, reworked Taric with Dazzle and Bard with Cosmic Binding.
  - Master Yi and reworked Shen may also count with Alpha Strike and Shadow Dash respectively, as Wuju Style and Twilight Assault are on-hit damaging abilities and not normal damaging abilities.
- He is often seen wearing his horned helmet, but in real life, such horns are considered impractical for combat use.

---
*This page was automatically generated from League of Legends Wiki data.*