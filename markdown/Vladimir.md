# Vladimir

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
| **Champion** | Vladimir |
| **Title** | the Crimson Reaper |
| **Resource** | Crimson Rush |
| **Range Type** | Ranged |
| **Release Date** | 2010-07-27 |
| **Release Patch** | V1.0.0.97 |
| **Roles** | Battlemage |
| **Riot Positions** | Middle |
| **External Positions** | Top, Middle |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $607.0$ | $+110.0$ | $2477.0$ |
| **Mana** | $2.0$ | $+0.0$ | $2.0$ |
| **Health Regen** | $7.0$ | $+0.6$ | $17.2$ |
| **Armor** | $27.0$ | $+4.5$ | $103.5$ |
| **Magic Resist** | $30.0$ | $+1.3$ | $52.1$ |
| **Attack Damage** | $55.0$ | $+3.0$ | $106.0$ |
| **Attack Speed** | $0.658$ | $+2.0\%$ | $0.882$ |
| **Movement Speed** | $330.0$ | $+0.0$ | $330.0$ |
| **Attack Range** | $450.0$ | $+0.0$ | $450.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.658$ |
| **Attack Speed Ratio** | $0.658$ |
| **Bonus AS per Level** | $2.0\%$ |
| **Attack Windup** | $19.7\%$ |
| **Missile Speed** | $1600 units/second$ |
| **Acquisition Radius** | $600 units$ |
| **Pathing Radius** | $35 units$ |
| **Selection Radius** | $120 units$ |
| **Selection Height** | $183.33 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Crimson Pact

**Innate:** **Vladimir** gains ap based on his **bonus** health, and *health **bonus** health* based on his ability power.

**Innate:** **Vladimir** gains ($ as ap and ( as *health **bonus** health*. These two bonuses do not stack with each other.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Self |

**Notes:**

- *Crimson Pact* only affects ''Vladimir's** **maximum** health and will not increase **Vladimir's** **current** health to prevent **Vladimir'' from restoring health whenever his ability power fluctuates.
  - The only exception is purchasing applicable items while inside the fountain, which will restore health equal to the bonus gained.
  - ''Vladimir's** **current** health can decrease to match his **maximum'* health if *Crimson Pact's **bonus** health is lost.
- 'Crimson Pact's **bonus** ability power stacks multiplicatively with other sources of % ability power but its **bonus** health stacks additively with other sources of **bonus** health. With *Rabadon's Deathcap Magical Opus, the total is (*bonus* health*bonus* ability power and (% AP/10000% **bonus* health*bonus** health. <!-- ** Case 2: With *Vigilant Wardstone you get a total of ((1+/100)% **bonus* health*bonus* ability power and (% AP*(1+/100)/10000% bonus** health*bonus** health.
  - Case 3: With both *Rabadon's Deathcap* and *Vigilant Wardstone you get a total of (% AP(1+/100)% **bonus* health*bonus* ability power and (% AP(+)**(1+/100)/10000% **bonus* health*bonus** health.
  - With *Cinderhulk* you get a total of % **bonus* health)*bonus* ability power and (*bonus* health*bonus** health. <!--** With *Rabadon's Deathcap* + *Cinderhulk you get a total of ((1+/100)% **bonus* health*bonus** ability power and (% AP*(1+/100)/10000% bonus** health*bonus** health. -->

---

### Q: Transfusion

**Active:** **Vladimir** drains blood from the target enemy, dealing magic damage and heal himself. He then generates 1 point of *Fury* over the *cooldown*.

*At 2 points of *Fury*, **Vladimir** surges, briefly gaining *ms **bonus** movement speed* and during which his *Fury* depletes over a short time.*

**Active:** **Vladimir** drains blood from the target enemy, dealing magic damage and heal himself. He then generates 1 point of *Fury* over the *cooldown*. At 2 points of *Fury*, **Vladimir** surges, gaining ms*bonus* movement speed decaying over $0.5$ seconds while the *Fury* depletes over $2.5$ seconds. *Fury* depletes 75% slower during **Sanguine Pool**, **Tides of Blood**, or stasis (buff). Casting *Transfusion* during the surge consumes all *Fury* to deal 85% increased damage and heal **Vladimir** for an additional 30 to 200 (+ 5% . The bonus healing is reduced toagainst minions.

| Attribute | Value |
|-----------|-------|
| **Range** | 600 units |
| **Cooldown** | $9-4.6$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Targeting** | Unit |
| **Affects** | Self, Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Single target |
| **Projectile** | False |

**Scaling:**
- **Magic Damage:** $80-160$ (+ 60% AP)
- **Heal:** $20-40$ (+ 35% AP)
- **Increased Damage:** $80×1.85-160×1.85$ (+ 111% AP)

**Notes:**

- Even though the healing effect is visualized as a projectile, the healing itself triggers instantly.
- 'Vladimir's* resource bar indicates his current *Fury' and changes colors depending on the charge-up stage of his surge. *# White while generating the first stack and while at 1 stack (there is no time-out period). *# Orange while generating the second stack. *# Red while he is surging (*Crimson Rush* will deplete over $2.5$ seconds once triggered).
  - Each stack generates over-time ('Transfusion's cooldown).
- **Vladimir** can cast *Sanguine Pool* and *Hemoplague* during 'Transfusion's cast time.
- The Fury depletes normally while under resurrection effects.

---

### W: Sanguine Pool

**Active:** **Vladimir** sinks into a pool of blood for a short time, becoming untargetable, ghosted, and gaining that decays over a shorter duration. He cannot use basic attacks and abilities, but can still move.

*Enemies within the pool are continually slow, and dealt magic damage based on his bonus health. **Vladimir** heals for a portion of the damage dealt.*

**Active:** **Vladimir** sinks into a pool of blood, becoming untargetable and ghosted for 2 seconds. He also gains ms*bonus** movement speed* that decays exponentially over 1 second. Enemies within the pool are dealt magic damage every $0.5$ seconds over the duration and are slow by 40%. **Vladimir** heals himself for 30% of the pre-mitigation damage dealt, reduced to $30×0.6$% against minions. '**Vladimir** cannot use basic attacks nor abilities during Sanguine Pool, but he can still move. If *Tides of Blood* is charging at the time of Sanguine Pool's activation, that ability may still be recast.'

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $28-16$ seconds |
| **Cast Time** | none |
| **Cost** | 15% **current** health |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | 350 units |
| **Spell Effects** | aoedot |

**Scaling:**
- **Magic Damage Per Tick:** $20-75$ (+ $3.75$% **bonus* health)4-75×4$ (+ 15%
- **bonus** health)

**Notes:**

- The first tick damages immediately so the final one occurs $0.5$ seconds before **Vladimir** becomes targetable again.
- There is an extra damage tick for 0 damage when **Vladimir** becomes targetable again, triggering the same spell effects as the normal ticks (except those which require damage greater than 0 being dealt).
  - This will trigger turret aggro onto **Vladimir** if an enemy champion is still within the effect radius when *Sanguine Pool* ends.
- The slow ends immediately once affected enemies get out of range.
- **Vladimir** can still use summoner spells and item actives during *Sanguine Pool*.
- While unable to attack, **Vladimir** can still input attack commands - causing him to follow his attack target.
  - ''Vladimir's' is reducedwhile pooled - causing him to attempt to move right up to his attack target.

---

### E: Tides of Blood

**Active:** **Vladimir** channel up to a brief moment to ramp up a health cost. *Tides of Blood* can be recast within the duration, and does so automatically afterwards or if interrupt.

**Recast:** **Vladimir** unleashes a nova of blood bolts that each deal magic damage to the first enemy hit based on his **maximum** health, which is further increased based on charge time. If he charged long enough, enemies hit are also briefly slow.

**Active:** **Vladimir** channel for up to $1.5$ seconds, during which he increases 'Tides of Blood's* damage over the first second of the channel, and becomes slow by 20% afterwards for the remaining duration. *Tides of Blood' can be recast within the duration, and does so automatically afterwards or if it is interrupt. **Recast:** **Vladimir** unleashes a nova of 15 blood bolts around himself that each deal magic damage to the first enemy hit, increased based on charge time up to the first second. If *Tides of Blood* was charged for at least 1 second, enemies hit are also slow for $0.5$ seconds. *Enemies can intercept multiple bolts, but can be damaged only once.* *If **Vladimir** is below 12% of his **maximum** health, Tides of Blood will not cost any health.*

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $13-5$ seconds |
| **Cast Time** | none |
| **Cost** | 2 to 8 **maximum** health |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | Magic |
| **Speed** | 4000 units/second |
| **Effect Radius** | 600 units |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | True |

**Scaling:**
- **Minimum Magic Damage:** $30-90$ (+ $1.5$% **maximum* health) (+ 35% AP)*maximum** health) (+ 80% AP)
- **Slow:** $40-60$%

**Notes:**

- The spell indicator shows 11 equally spaced missile indicators when hovering the ability in the HUD, however the spell actually casts 15 equally spaced missiles.
- The health cost may still drop **Vladimir** below the specified amount if he is above it. This is verified for every tick of health cost, i.e if the first tick drops him below it, the next ones will stop affecting him if he remains below it.
- The following table refers for interactions while **Vladimir** is channel:

---

### R: Hemoplague

**Active:** **Vladimir** unleashes a plague at the target location that infects nearby enemies hit for a few seconds, increasing the damage they take from all sources.

*After the duration, infected targets are dealt magic damage, which heals **Vladimir** for each champion hit.*

**Active:** **Vladimir** spreads a virulent plague at the target location that infects enemies hit for 4 seconds, increasing the damage they take from all sources by 10%. After the duration, the infection bursts to deal magic damage to all affected targets and, after a $0.4$-second delay, heal **Vladimir** for each infected champion, reduced by 40% for champions beyond the first.

| Attribute | Value |
|-----------|-------|
| **Range** | 625 units |
| **Cooldown** | 120 seconds |
| **Cast Time** | none |
| **Targeting** | Location |
| **Affects** | Self, Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | 375 units |
| **Spell Shield** | Special |
| **Spell Effects** | Area of effect |
| **Projectile** | False |

**Scaling:**
- **Magic damage:** $150-350$ (+ 70% AP) **Heal:* $150-350$ (+ 70% AP)0.4-350×0.4$ (+ $700.4$% AP)(1+0.4×4)-350*(1+0.4×4)$ (+ $70*(1+0.4×4)$% AP)

**Notes:**

- *Hemoplague* will amplify almost all sources and types of damage, even including damage from neutral monsters.
  - will however not be amplified.
  - 'Hemoplague's effect stacks multiplicatively with other damage modifiers.
  - *Hemoplague* amplifies itself for an actual damage of $150×1.1-350×1.1$ (+ $77$% AP).
- 'Hemoplague's modifier to incoming damage stacks additively with Unbreakable Will for a total damage reduction of $45-65 3$%.
- Spell shield does not negate the detonation.

---

## Patch History

### V25.16
- General
  - **Bug Fixes:** Recall SFX no longer unintentionally plays for all players in the lobby that have vision of **Vladimir** regardless of the location of their view.
- *Crimson Pact*
  - **New Effect:** While inside the fountain, now restores health equal to the bonus gained from purchasing items that increment the effect.

### V25.11
- General
  - New default Recall animation.
- Vladimir
  - Adjusted splash art.
  - Updated models and VFX.

### V14.24
- *Hemoplague*
  - **Bug Fixes:** No longer reveals the user if cast while the cursor is near an enemy champion.

### V14.19
- *Sanguine Pool*
  - **New Effect:** Healing is now 60% effective against minions.
  - In-game tooltip now clarifies that the healing is based on pre-mitigation damage dealt.

### V14.18
- *Sanguine Pool*
  - **Bug Fixes:** No longer improperly takes damage from certain effects despite being untargetable to them.

### V14.13
- *Sanguine Pool*
  - **Bug Fixes:** The ability's health scaling is now properly applied to the heal.

### V14.12
- *Sanguine Pool*
  - Health cost reduced to 15% **current** health from 20%.
  - Health ratio increased to 15% **bonus** health from 10%.
  - Healing increased to 30% of damage dealt from 15%.

### V13.13
- *Hemoplague*
  - **Bug Fixes:** Range indicator is no longer different before and after leveling up the ability.

### V13.9
- *Transfusion*
  - **Bug Fixes:** No longer damages through untargetability.

### V13.7
- *Transfusion*
  - **New Effect:** Now completes the cast even if the target dies during the cast time.

## Trivia

- Vladimir was the second champion released in Season One (the first being **Xin Zhao**).
  - He was also the only champion labeled as Mage/Tank for his legacy champion classes.
- Vladimir is one of champions who use health as a resource for abilities, the other five being **Briar**, **Dr. Mundo**, **Olaf**, **Soraka**, and **Zac**.
- The old icon for Tides of Blood old.png is similar to the old one for Ravenous Flock old.png.
- His name (Russian: Влади́мир) comes from Old Church Slavonic Владимѣръ *, meaning "great ruler".
  - The name is either native Slavonic or loaned from Germanic *Waldemar*.
- Vladimir's Series 2 Eternals make the following references:
  - *What Is A Man?* is a reference to the Die Monster / What Is A Man? meme.

---
*This page was automatically generated from League of Legends Wiki data.*