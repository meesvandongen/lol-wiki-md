# Vladimir

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
| **Champion** | Vladimir |
| **Title** | the Crimson Reaper |
| **Resource** | Crimson Rush |
| **Range Type** | Ranged |
| **Release Date** | 2010-07-27 |
| **Release Patch** | V1.0.0.97 |
| **Latest Changes** | V25.16 |
| **Roles** | Battlemage |
| **Riot Positions** | Middle |
| **External Positions** | Top, Middle |
| **Blue Essence** | 1575 |
| **Riot Points** | 790 |
| **Difficulty** | 3 |
| **Hero Type** | Mage |
| **Alt Type** | Fighter |
| **Adaptive Type** | Magic |
| **Damage** | 3 |
| **Toughness** | 2 |
| **Control** | 1 |
| **Mobility** | 1 |
| **Utility** | 1 |
| **Style** | 100 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $607.0$ | $+110.0$ |
| **Mana** | $2.0$ | $+0.0$ |
| **Health Regen** | $7.0$ | $+0.6$ |
| **Armor** | $27.0$ | $+4.5$ |
| **Magic Resist** | $30.0$ | $+1.3$ |
| **Attack Damage** | $55.0$ | $+3.0$ |
| **Attack Speed** | $0.658$ | |
| **Movement Speed** | $330.0$ | $+0.0$ |
| **Attack Range** | $450.0$ | $+0.0$ |
| **Base Attack Speed** | $0.658$ | |
| **Attack Speed Ratio** | $0.658$ | |
| **Bonus AS per Level** | $2.0\%$ | |
| **Attack Windup** | $19.7\%$ | |
| **Missile Speed** | $1600$ units/second | |
| **Acquisition Radius** | $600$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $120$ units | |
| **Selection Height** | $183.33$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $100.0\%$ |
| **Damage Taken** | $105.0\%$ |
| **Healing** | $90.0\%$ |

#### URF

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $92.0\%$ |
| **Damage Taken** | $105.0\%$ |
| **Healing** | $120.0\%$ |

## Abilities

### Passive: Crimson Pact

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Self |

**INNATE:** **Vladimir** gains ($3.3$% **bonus** health) as (ap) ability power and (160% AP) as (health) **bonus** health. These two bonuses do not stack with each other.

**Notes:**

- *Crimson Pact* only affects **Vladimir**’s **maximum** health and will not increase **Vladimir**’s **current** health to prevent **Vladimir** from restoring health whenever his ability power fluctuates.
  - The only exception is purchasing applicable items while inside the fountain, which will restore health equal to the bonus gained.
  - **Vladimir**’s **current** health can decrease to match his **maximum** health if *Crimson Pact*’s **bonus** health is lost.
- *Crimson Pact*’s **bonus** ability power stacks multiplicatively with other sources of % ability power but its **bonus** health stacks additively with other sources of **bonus** health. With Rabadon's Deathcap Magical Opus, the total is ( + ) **bonus** ability power and ( + ) **bonus** health. <!-- ** Case 2: With Vigilant Wardstone you get a total of ( + ) **bonus** ability power and ( + ) **bonus** health.
  - Case 3: With both Rabadon's Deathcap and Vigilant Wardstone you get a total of ( + ) **bonus** ability power and ( + ) **bonus** health.
  - With Cinderhulk you get a total of **bonus** ability power and ( + ) **bonus** health. <!--** With Rabadon's Deathcap + Cinderhulk you get a total of ( + ) **bonus** ability power and ( + ) **bonus** health. -->

---

### Q: Transfusion

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 600 units |
| **Cooldown** | 9 / 7.9 / 6.8 / 5.7 / 4.6 seconds |
| **Targeting** | Unit |
| **Affects** | Self, Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Single target |
| **Projectile** | False |
| **Call For Help** | True |

**ACTIVE:** **Vladimir** drains blood from the target enemy, dealing magic damage and healing himself. He then generates 1 point of *Crimson Rush* over the cooldown.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 80 / 100 / 120 / 140 / 160 (+ 60% AP) |

| Attribute | Value |
|-----------|------:|
| **Heal** | 20 / 25 / 30 / 35 / 40 (+ 35% AP) |

At 2 points of *Crimson Rush*, **Vladimir** surges, gaining ms decaying over $0.5$ seconds while the *Crimson Rush* depletes over $2.5$ seconds. *Crimson Rush* depletes 75% slower during *Sanguine Pool*, *Tides of Blood*, or stasis.

Casting *Transfusion* during the surge consumes all *Crimson Rush* to deal 85% increased damage and heal **Vladimir** for an additional 30 to 200 (+ 5% (+ 4% per 100 AP) of his **missing** health). The bonus healing is reduced to「 35% ⟷ 30×0.35 to 200×0.35 (+ 1.75% (+ 1.4% per 100 AP) of his **missing** health) 」against minions.

| Attribute | Value |
|-----------|------:|
| **Increased Damage** | 148 / 185 / 222 / 259 / 296 (+ 111% AP) |

**Notes:**

- Even though the healing effect is visualized as a projectile, the healing itself triggers instantly.
- *Vladimir*’s resource bar indicates his current *Crimson Rush* and changes colors depending on the charge-up stage of his surge.
  1. White while generating the first stack and while at 1 stack (there is no time-out period).
  1. Orange while generating the second stack.
  1. Red while he is surging (*Crimson Rush* will deplete over $2.5$ seconds once triggered).
  - Each stack generates over-time (*Transfusion*’s cooldown).
- **Vladimir** can cast Sanguine Pool and Hemoplague during *Transfusion*’s cast time.
- The Crimson Rush depletes normally while under resurrection effects.

---

### W: Sanguine Pool

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 350 units |
| **Cost** | 15% **current** health |
| **Cooldown** | 28 / 25 / 22 / 19 / 16 seconds |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | Magic |
| **Spell Effects** | aoedot |

**ACTIVE:** **Vladimir** sinks into a pool of blood, becoming untargetable and ghosted for 2 seconds. He also gains (ms) $37.5$% **bonus** movement speed that decays exponentially over 1 second.

Enemies within the pool are dealt magic damage every $0.5$ seconds over the duration and are slowed by 40%. **Vladimir** heals himself for 30% of the pre-mitigation damage (Damage calculated before modifiers) dealt, reduced to 18% against minions.

| Attribute | Value |
|-----------|------:|
| **Magic Damage Per Tick** | 20 / 33.75 / 47.5 / 61.25 / 75 (+ $3.75$% **bonus** health) |
| **Total Magic Damage** | 80 / 135 / 190 / 245 / 300 (+ 15% **bonus** health) |

***Vladimir** cannot use basic attacks nor abilities during Sanguine Pool, but he can still move. If Tides of Blood is charging at the time of Sanguine Pool's activation, that ability may still be recast.*

**Notes:**

- The first tick damages immediately so the final one occurs $0.5$ seconds before **Vladimir** becomes targetable again.
- There is an extra damage tick for 0 damage when **Vladimir** becomes targetable again, triggering the same spell effects as the normal ticks (except those which require damage greater than 0 being dealt).
  - This will trigger turret aggro onto **Vladimir** if an enemy champion is still within the effect radius when *Sanguine Pool* ends.
- The slow ends immediately once affected enemies get out of range.
- **Vladimir** can still use summoner spells and item actives during *Sanguine Pool*.
- While unable to attack, **Vladimir** can still input attack commands - causing him to follow his attack target.
  - **Vladimir**’s attack range is reduced「 to 0 ⟷ by 450 」while pooled - causing him to attempt to move right up to his attack target.

---

### E: Tides of Blood

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 600 (range of each blood missile) units |
| **Width** | 120 (width of each blood missile) units |
| **Speed** | 4000 (speed of every blood missile) units/second |
| **Cost** | 2 to 8 **maximum** health |
| **Cooldown** | 13 / 11 / 9 / 7 / 5 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | True |
| **Silence** | True |

**ACTIVE:** **Vladimir** charges for up to $1.5$ seconds, during which he increases *Tides of Blood*’s damage over the first second of the channel, and becomes slowed by 20% afterwards for the remaining duration. *Tides of Blood* can be recast within the duration, and does so automatically afterwards or if it is interrupted.

**RECAST:** **Vladimir** unleashes a nova of 15 blood bolts around himself that each deal magic damage to the first enemy hit, increased based on charge time up to the first second.

| Attribute | Value |
|-----------|------:|
| **Minimum Magic Damage** | 30 / 45 / 60 / 75 / 90 (+ $1.5$% **maximum** health) (+ 35% AP) |
| **Maximum Magic Damage** | 60 / 90 / 120 / 150 / 180 (+ 6% **maximum** health) (+ 80% AP) |

If *Tides of Blood* was charged for at least 1 second, enemies hit are also slowed for $0.5$ seconds.

| Attribute | Value |
|-----------|------:|
| **Slow** | 40 / 45 / 50 / 55 / 60% |

*Enemies can intercept multiple bolts, but can be damaged only once.*

*If **Vladimir** is below 12% of his **maximum** health, Tides of Blood will not cost any health.*

**Notes:**

- The spell indicator shows 11 equally spaced missile indicators when hovering the ability in the HUD, however the spell actually casts 15 equally spaced missiles.
- The health cost may still drop **Vladimir** below the specified amount if he is above it. This is verified for every tick of health cost, i.e if the first tick drops him below it, the next ones will stop affecting him if he remains below it.
- The following table refers for interactions while **Vladimir** is channeling:

#### Channel Behavior (charge)

| Aspect | State / Notes |
|--------|---------------|
| **Abilities** | Sanguine Pool is usable. Transfusion and Hemoplague both interrupt after $0.25$ seconds. |
| **Items** | Interrupts / Allowed / Disabled |
| **Summoner Spells** | Allowed / Interrupts |

---

### R: Hemoplague

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 625 units |
| **Effect Radius** | 375 units |
| **Cooldown** | 120 seconds |
| **Targeting** | Location |
| **Affects** | Self, Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | Special |
| **Spell Effects** | Area of effect |
| **Projectile** | False |
| **Out of Range Behavior** | walk to location |

**ACTIVE:** **Vladimir** spreads a virulent plague at the target location that infects enemies hit for 4 seconds, increasing the damage they take from all sources by 10%.

After the duration, the infection bursts to deal magic damage to all affected targets and, after a $0.4$-second delay, heal **Vladimir** for each infected champion, reduced by 40% for champions beyond the first.

| Attribute | Value |
|-----------|------:|
| **Magic damage** | 150 / 200 / 250 / 300 / 350 (+ 70% AP) |

| Attribute | Value |
|-----------|------:|
| **Heal** | 150 / 200 / 250 / 300 / 350 (+ 70% AP) |
| **Reduced Heal** | 60 / 80 / 100 / 120 / 140 (+ 28% AP) |
| **Maximum Total Heal** | 390 / 520 / 650 / 780 / 910 (+ 182% AP) |

**Notes:**

- *Hemoplague* will amplify almost all sources and types of damage, even including damage from neutral monsters.
  - True damage will however not be amplified.
  - *Hemoplague*’s effect stacks multiplicatively with other damage modifiers.
  - *Hemoplague* amplifies itself for an actual damage of 165 / 220 / 275 / 330 / 385 (+ $77$% AP).
- *Hemoplague*’s modifier to incoming damage stacks additively with Alistar’s Unbreakable Will for a total damage reduction of 45 / 55 / 65%.
- Spell shield does not negate the detonation.

---

## Patch History

### V25.16
- General
  - **Bug Fixes:** Recall SFX no longer unintentionally plays for all players in the lobby that have vision of **Vladimir** regardless of the location of their view.
- Crimson Pact
  - **New Effect:** While inside the fountain, now restores health equal to the bonus gained from purchasing items that increment the effect.

### V25.11
- General
  - New default Recall animation.
- Vladimir
  - Adjusted splash art.
  - Updated models and VFX.

### V14.24
- Hemoplague
  - **Bug Fixes:** No longer reveals the user if cast while the cursor is near an enemy champion.

### V14.19
- Sanguine Pool
  - **New Effect:** Healing is now 60% effective against minions.
  - In-game tooltip now clarifies that the healing is based on pre-mitigation damage dealt.

### V14.18
- Sanguine Pool
  - **Bug Fixes:** No longer improperly takes damage from certain effects despite being untargetable to them.

### V14.13
- Sanguine Pool
  - **Bug Fixes:** The ability's health scaling is now properly applied to the heal.

### V14.12
- Sanguine Pool
  - Health cost reduced to 15% **current** health from 20%.
  - Health ratio increased to 15% **bonus** health from 10%.
  - Healing increased to 30% of damage dealt from 15%.

### V13.13
- Hemoplague
  - **Bug Fixes:** Range indicator is no longer different before and after leveling up the ability.

### V13.9
- Transfusion
  - **Bug Fixes:** No longer damages through untargetability.

### V13.7
- Transfusion
  - **New Effect:** Now completes the cast even if the target dies during the cast time.

## Trivia

- Vladimir was the second champion released in Season One (the first being Xin Zhao).
  - He was also the only champion labeled as Mage/Tank for his legacy champion classes.
- Vladimir is one of champions who use health as a resource for abilities, the other five being Briar, Dr. Mundo, Olaf, Soraka, and Zac.
- The old icon for Tides of Blood old.png is similar to the old one for Ravenous Flock old.png.
- His name (Russian: Влади́мир) comes from Old Church Slavonic Владимѣръ **, meaning "great ruler".
  - The name is either native Slavonic or loaned from Germanic *Waldemar*.
- Vladimir's Series 2 Eternals make the following references:
  - *What Is A Man?* is a reference to the Die Monster / What Is A Man? meme.

---
*This page was automatically generated from League of Legends Wiki data.*