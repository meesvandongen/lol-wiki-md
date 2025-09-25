# Amumu

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
| **Champion** | Amumu |
| **Title** | the Sad Mummy |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2009-06-26 |
| **Release Patch** | June 26, 2009 Patch |
| **Latest Changes** | V25.18 |
| **Roles** | Vanguard |
| **Riot Positions** | Jungle |
| **External Positions** | Jungle, Support |
| **Blue Essence** | 225 |
| **Riot Points** | 260 |
| **Difficulty** | 1 |
| **Hero Type** | Tank |
| **Alt Type** | Support |
| **Adaptive Type** | Magic |
| **Damage** | 2 |
| **Toughness** | 3 |
| **Control** | 3 |
| **Mobility** | 1 |
| **Utility** | 1 |
| **Style** | 90 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $685.0$ | $+94.0$ |
| **Mana** | $285.0$ | $+40.0$ |
| **Health Regen** | $9.0$ | $+0.85$ |
| **Mana Regen** | $7.4$ | $+0.55$ |
| **Armor** | $33.0$ | $+4.0$ |
| **Magic Resist** | $32.0$ | $+2.05$ |
| **Attack Damage** | $57.0$ | $+3.8$ |
| **Attack Speed** | $0.736$ | |
| **Movement Speed** | $335.0$ | $+0.0$ |
| **Attack Range** | $125.0$ | $+0.0$ |
| **Base Attack Speed** | $0.736$ | |
| **Attack Speed Ratio** | $0.638$ | |
| **Bonus AS per Level** | $2.2\%$ | |
| **Acquisition Radius** | $600$ units | |
| **Gameplay Radius** | $55$ units | |
| **Pathing Radius** | $30$ units | |
| **Selection Radius** | $100$ units | |
| **Selection Height** | $130$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Taken** | $100.0\%$ |

#### URF

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $110.0\%$ |
| **Damage Taken** | $90.0\%$ |

## Abilities

### Passive: Cursed Touch

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Enemies |
| **Damage Type** | True |
| **Spell Shield** | Special |
| **Spell Effects** | default |
| **Parry** | True |

**INNATE:** **Amumu**’s basic attacks and *Curse of the Sad Mummy* are empowered to mark enemies with *Curse* for 3 seconds, refreshing on subsequent applications and *Despair’s* per-tick damage.

*Cursed* targets receive 10% **bonus** true damage from all incoming pre-mitigation (Damage calculated before modifiers) magic damage.

**Notes:**

- If the triggering basic attack from **Amumu** applies magic damage on-hit, *Curse* will be applied before the magic damage.
- The *Curse* application from *Curse of the Sad Mummy* happens after the magic damage is dealt by it, therefore amplifying the damage only if the target was also already marked before the hit.
- Neutral units / enemies affected by the *Curse* can receive bonus damage from the opposing team of **Amumu** / neutral units, but the damage will be credited to **Amumu**.
- The application of *Curse* on *Curse of the Sad Mummy* is negated if the ability is blocked by a spell shield.

---

### Q: Bandage Toss

| Attribute | Value |
|-----------|------:|
| **Range** | 1100 units |
| **Cast Time** | $0.25$ seconds |
| **Width** | 160 units |
| **Speed** | 2000 (Bandage speed) / 1800 (Dash speed) units/second |
| **Cost** | 50 mana |
| **Recharge** | 16 / 15 / 14 / 13 / 12 seconds |
| **Static Cooldown** | 3 |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | Special |
| **Spell Effects** | spell |
| **Projectile** | True |
| **Grounded** | True |
| **Knockdown** | True |

**ACTIVE:** **Amumu** throws a bandage in the target direction that deals magic damage to the first enemy hit, stunning them for 1 second and pulling him to them, during which they are revealed.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 70 / 95 / 120 / 145 / 170 (+ 85% AP) |

**Amumu** periodically stocks a *Bandage Toss* charge, up to a maximum of 2.

*Despair can be cast during the dash.*

**Notes:**

- **Amumu** will track the target if they change locations.
  - He will dash to the target's previous location if the target is 2000 or more units away or moves beyond 2000 (Estimated) units.
- *Bandage Toss* will always be directed towards the target's location even if **Amumu** moves while it is in flight.
- Immobilizing effects will not prevent **Amumu** from commencing the dash.
- *Bandage Toss* does not fizzle on **Amumu**’s death, and can still stun and damage enemies.
- **Amumu** is still able to dash even if *Bandage Toss* is blocked by spell shield.
- **Amumu** will be ordered to basic attack the target upon completion of the dash. Effect at cast time end

---

### W: Despair

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 350 units |
| **Cost** | 8 mana per second |
| **Static Cooldown** | 1 |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | False |
| **Spell Effects** | aoedot |

**TOGGLE:** **Amumu** cries a continuous pool of tears, dealing magic damage every $0.5$ seconds to nearby enemies.

| Attribute | Value |
|-----------|------:|
| **Magic Damage Per Tick** | 5 (+ 0.5 / 0.625 / 0.75 / 0.875 / 1% (+ $0.25$% per 100 AP) of target's **maximum** health) |

**Notes:**

- Toggle abilities do not count as ability activations for the purposes of on-cast effects such as Spellblade and triggering Force Pulse’s passive.
- Despite the rounding in *Despair*’s tooltip, it will deal increased damage for every point of ability power (+0.0025% per 1 AP).

---

### E: Tantrum

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Effect Radius** | 350 units |
| **Cost** | 35 mana |
| **Cooldown** | 9 / 8 / 7 / 6 / 5 seconds |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |

**PASSIVE:** **Amumu** reduces every instance of pre-mitigation (Damage calculated before modifiers) physical damage taken, capped at 50% of the damage instance.

| Attribute | Value |
|-----------|------:|
| **Physical Damage Reduction** | 5 / 7 / 9 / 11 / 13 (+ 3% **bonus** armor) (+ 3% **bonus** magic resistance) |

**ACTIVE:** **Amumu** releases his anger, dealing magic damage to nearby enemies.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 65 / 95 / 125 / 155 / 185 (+ 50% AP) |

*Tantrum*’s **current** cooldown is reduced by $0.75$ seconds whenever **Amumu** is hit by a basic attack on-hit.

**Notes:**

- A basic attack's basic damage and each on-hit effect are all separate damage instances, and may be reduced each if they deal physical damage.
- *Tantrum*’s cooldown reduction will trigger multiple times for effects such as Double Strike and Guinsoo's Rageblade *Phantom Hit*.
  - The cooldown will be reduced, even if the incoming attack is blocked.
  - The cooldown will not be reduced, if the incoming attack misses due to the enemy being blinded. Effect at cast time end

---

### R: Curse of the Sad Mummy

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Effect Radius** | 550 units |
| **Cost** | 100 / 125 / 150 / 175 / 200 mana |
| **Cooldown** | 150 / 137.5 / 125 / 112.5 / 100 seconds |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |

**ACTIVE:** **Amumu** entangles nearby enemies in bandages, dealing magic damage as well as knocking them down and stunning them for $1.5$ seconds.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 200 / 250 / 300 / 350 / 400 (+ 80% AP) |

**Notes:**

Effect at cast time end

---

## Patch History

### V25.18
- Bandage Toss
  - Recharge timer reduced to 16 / 15 / 14 / 13 / 12 seconds from 16 / 15.5 / 15 / 14.5 / 14.
  - Mana cost changed to 50 at all ranks from 45 / 50 / 55 / 60 / 65.

### V14.19#September 26th Hotfix|V14.19
- Despair
  - Base damage per tick reduced to 5 from from 7.
- Tantrum
  - Base damage reduced to 65 / 95 / 125 / 155 / 185 from 65 / 100 / 135 / 170 / 205.

### V14.9
- Stats
  - Attack damage increased to 57 from 53.
  - Base armor increased to 33 from 30.

### V13.11
- Despair
  - Base damage per tick reduced to 7 from 10.

### V13.9
- Despair
  - Base damage per tick changed to 10 at all ranks from 6 / 8 / 10 / 12 / 14.
  - Health ratio per tick increased to 0.5 / 0.625 / 0.75 / 0.875 / 1% of target's **maximum** health from 0.5 / 0.575 / 0.65 / 0.725 / 0.8%.

### V13.4
- Stats
  - Health growth reduced to 94 from 100.
  - Armor growth reduced to 4 from $4.2$.
- Despair
  - Health ratio per tick reduced to 0.5 / 0.575 / 0.65 / 0.725 / 0.8% of target's **maximum** health from 0.5 / 0.625 / 0.75 / 0.875 / 1%.

### V13.3
- Bandage Toss
  - Mana cost increased to 45 / 50 / 55 / 60 / 65 from 40 / 45 / 50 / 55 / 60.
- Tantrum
  - Base damage changed to 65 / 100 / 135 / 170 / 205 from 80 / 110 / 140 / 170 / 200.

### V13.1
- Tantrum
  - **Bug Fixes:** Now properly displays in the death recap the amount the cooldown is reduced by from enemy basic attacks.

### V12.23
- Stats
  - Health growth increased to 100 from 89.
- Bandage Toss
  - Mana cost increased to 40 / 45 / 50 / 55 / 60 from 30 / 35 / 40 / 45 / 50.
- Despair
  - Health ratio per tick increased to 0.5 / 0.625 / 0.75 / 0.875 / 1% **maximum** health from 0.5 / 0.575 / 0.65 / 0.725 / 0.8%.
- Tantrum
  - Base damage increased to 80 / 110 / 140 / 170 / 200 from 85 / 110 / 135 / 160 / 185.

### V12.22#November 17th Hotfix|V12.22
- Tantrum
  - Base damage increased to 85 / 110 / 135 / 160 / 185 from 75 / 100 / 125 / 150 / 175.
  - Cooldown reduction from attacks increased to $0.75$ seconds from $0.5$.

## Trivia

- Amumu's original icon for Bandage Toss shows him in a pose that references Spider-Man's web shooting from Marvel Comics.
- Amumu is seen on a caution in the game's Mac Version trailer.
- Amumu was the third champion to attain 7 skins.
- Amumu's dance references the Goth kids' dance from *South Park* adult animated series.
  - A side-by-side comparison can be seen here.
  - Both are in turn parodying classic 'Peanut's Dance', a bizarre shuffling dance performed by the characters from the Peanuts comic and animated films.
- When Amumu dies, his pose is that of an Egyptian mummy with his arms crossed right over left.
  - This is a nod to him being a Shuriman boy prince in life.
- Amumu's price range was lowered from to in accordance with the release of his The Curse of the Sad Mummy music video.
- Amumu and Blitzcrank were planned to have Amumu Blitzcrank skins, but both were cancelled for failing to meet Riot's quality standards.
  - They may return in the future.
- The ward skin Sad Mummy Ward.png references him.
- He has a base of 50 which is considered medium.

---
*This page was automatically generated from League of Legends Wiki data.*