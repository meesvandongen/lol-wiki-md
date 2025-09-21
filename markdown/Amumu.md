# Amumu

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
| **Champion** | Amumu |
| **Title** | the Sad Mummy |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2009-06-26 |
| **Release Patch** | June 26, 2009 Patch |
| **Roles** | Vanguard |
| **Riot Positions** | Jungle |
| **External Positions** | Jungle, Support |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $685.0$ | $+94.0$ | $2283.0$ |
| **Mana** | $285.0$ | $+40.0$ | $965.0$ |
| **Health Regen** | $9.0$ | $+0.85$ | $23.4$ |
| **Mana Regen** | $7.4$ | $+0.55$ | $16.8$ |
| **Armor** | $33.0$ | $+4.0$ | $101.0$ |
| **Magic Resist** | $32.0$ | $+2.05$ | $66.8$ |
| **Attack Damage** | $57.0$ | $+3.8$ | $121.6$ |
| **Attack Speed** | $0.736$ | $+2.2\%$ | $1.009$ |
| **Movement Speed** | $335.0$ | $+0.0$ | $335.0$ |
| **Attack Range** | $125.0$ | $+0.0$ | $125.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.736$ |
| **Attack Speed Ratio** | $0.638$ |
| **Bonus AS per Level** | $2.2\%$ |
| **Acquisition Radius** | $600 units$ |
| **Gameplay Radius** | $55 units$ |
| **Pathing Radius** | $30 units$ |
| **Selection Radius** | $100 units$ |
| **Selection Height** | $130 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Cursed Touch

**Innate:** **Amumu**’s basic attacks and **Curse of the Sad Mummy** mark enemies with *Curse*. **Despair** will refresh ongoing *Curse* marks on enemies hit.

*Curse* targets take **bonus** true damage from incoming magic damage.

**Innate:** ''Amumu's* basic attacks and **Curse of the Sad Mummy** are empowered to mark enemies with *Curse* for 3 seconds, refreshing on subsequent applications and **Despair*’s' per-tick damage. *Cursed* targets receive *10% **bonus** true damage* from all incoming pre-mitigation magic damage.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Enemies |
| **Damage Type** | True |
| **Spell Shield** | Special |
| **Spell Effects** | default |

**Notes:**

- If the triggering basic attack from **Amumu** applies magic damage on-hit, *Curse* will be applied before the magic damage.
- The *Curse* application from **Curse of the Sad Mummy** happens after the magic damage is dealt by it, therefore amplifying the damage only if the target was also already marked before the hit.
- Neutral units / enemies affected by the *Curse* can receive bonus damage from the opposing team of **Amumu** / neutral units, but the damage will be credited to **Amumu**.
- The application of *Curse* on **Curse of the Sad Mummy** is negated if the ability is blocked by a spell shield.

---

### Q: Bandage Toss

**Active:** **Amumu** throws a bandage that deals magic damage and stun the first enemy hit while he dash to them.

**Amumu** periodically stocks a charge of *Bandage Toss*, up to 2.

**Active:** **Amumu** throws a bandage in the target direction that deals magic damage to the first enemy hit, stun them for 1 second and dash him to them, during which they are standard sight. **Amumu** periodically stocks a *Bandage Toss* charge, up to a maximum of 2. **Despair* can be cast during the dash.*

| Attribute | Value |
|-----------|-------|
| **Recharge** | $16-12$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | 50 mana |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Speed** | 2000 / 1800 units/second |
| **Spell Shield** | Special |
| **Spell Effects** | spell |
| **Projectile** | True |

**Scaling:**
- **Magic Damage:** $70-170$ (+ 85% AP)

**Notes:**

- **Amumu** will track the target if they change locations.
  - He will dash to the target's previous location if the target is 2000 or more units away or moves beyond 2000 units.
- *Bandage Toss* will always be directed towards the target's location even if **Amumu** moves while it is in flight.
- Immobilize effects will not prevent **Amumu** from commencing the dash.
- *Bandage Toss* does not fizzle on ''Amumu's' death, and can still stun and damage enemies.
- **Amumu** is still able to dash even if *Bandage Toss* is blocked by spell shield.
- **Amumu** will be ordered to basic attack the target upon completion of the dash. Effect at cast time end

---

### W: Despair

**Toggle:** **Amumu** begins crying, continually dealing magic damage to nearby enemies based on their **maximum** health.

**Toggle:** **Amumu** cries a continuous pool of tears, dealing magic damage every $0.5$ seconds to nearby enemies.

| Attribute | Value |
|-----------|-------|
| **Cast Time** | none |
| **Cost** | 8 mana per second |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | 350 units |
| **Spell Shield** | False |
| **Spell Effects** | aoedot |

**Scaling:**
- **Magic Damage Per Tick:** 5 (+ $0.5-1$%

**Notes:**

- Toggle abilities do not count as ability activations for the purposes of on-cast effects such as Spellblade and triggering *Force Pulse*’s passive.
- Despite the rounding in 'Despair's tooltip, it will deal increased damage for every point of ability power.

---

### E: Tantrum

**Passive:** **Amumu** gains damage reduction against physical damage. cd cooldown is reduced whenever he is hit by a basic attack.

**Active:** **Amumu** releases his anger, dealing magic damage to nearby enemies.

**Passive:** **Amumu** reduces every instance of pre-mitigation physical damage taken, capped at 50% of the damage instance. **Active:** **Amumu** releases his anger, dealing magic damage to nearby enemies. 'Tantrum's **current cooldown** is reduced by $0.75$ seconds whenever **Amumu** is hit by a basic attack on-hit.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $9-5$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | 35 mana |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | 350 units |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |

**Scaling:**
- **Physical Damage Reduction:** $5-13$ (+ 3%
- *bonus armor) (+ 3%
- **bonus** magic resistance)
- **Magic Damage:** $65-185$ (+ 50% AP)

**Notes:**

- A basic attack's basic damage and each on-hit effect are all separate damage instances, and may be reduced each if they deal physical damage.
- 'Tantrum's cooldown reduction will trigger multiple times for effects such as *Double Strike* and *Guinsoo's Rageblade* *Phantom Hit*.
  - The cooldown will be reduced, even if the incoming attack is blocked.
  - The cooldown will not be reduced, if the incoming attack misses due to the enemy being blind. Effect at cast time end

---

### R: Curse of the Sad Mummy

**Active:** **Amumu** entangles nearby enemies in bandages, dealing magic damage as well as briefly knockdown and stun them.

**Active:** **Amumu** entangles nearby enemies in bandages, dealing magic damage as well as knockdown and stun them for $1.5$ seconds.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $150-100$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $100-200$ mana |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | 550 units |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |

**Scaling:**
- **Magic Damage:** $200-400$ (+ 80% AP)

**Notes:**

No additional notes.

---

## Patch History

### V25.18
- *Bandage Toss*
  - Recharge timer reduced to $16-12$ seconds from $16-14$.
  - Mana cost changed to 50 at all ranks from $45-65$.
- *Despair*
  - Base damage per tick reduced to 5 from from 7.
- *Tantrum*
  - Base damage reduced to $65-185$ from $65-205$.

### V14.9
- Stats
  - Attack damage increased to 57 from 53.
  - Base armor increased to 33 from 30.

### V13.11
- *Despair*
  - Base damage per tick reduced to 7 from 10.

### V13.9
- *Despair*
  - Base damage per tick changed to 10 at all ranks from $6-14$.
  - Health ratio per tick increased to $0.5-1$% of target's **maximum** health from $0.5-0.8$%.

### V13.4
- Stats
  - Health growth reduced to 94 from 100.
  - Armor growth reduced to 4 from $4.2$.
- *Despair*
  - Health ratio per tick reduced to $0.5-0.8$% of target's **maximum** health from $0.5-1$%.

### V13.3
- *Bandage Toss*
  - Mana cost increased to $45-65$ from $40-60$.
- *Tantrum*
  - Base damage changed to $65-205$ from $80-200$.

### V13.1
- *Tantrum*
  - **Bug Fixes:** Now properly displays in the death recap the amount the cooldown is reduced by from enemy basic attacks.

### V12.23
- Stats
  - Health growth increased to 100 from 89.
- *Bandage Toss*
  - Mana cost increased to $40-60$ from $30-50$.
- *Despair*
  - Health ratio per tick increased to $0.5-1$% **maximum** health from $0.5-0.8$%.
- *Tantrum*
  - Base damage increased to $80-200$ from $85-185$.
- *Tantrum*
  - Base damage increased to $85-185$ from $75-175$.
  - Cooldown reduction from attacks increased to $0.75$ seconds from $0.5$.

### V12.22
- Stats
  - Base mana regeneration increased to $7.4$ from $7.38$.
  - Mana regeneration growth increased to $0.55$ from $0.53$.

### V12.16
- General
  - Adjusted splash artwork for Amumu.

## Trivia

- Amumu's original icon for *image=Bandage Toss old.png* shows him in a pose that references Spider-Man's web shooting from Marvel Comics.
- Amumu is seen on a caution in the game's Mac Version trailer.
- Amumu was the third champion to attain 7 skins.
- Amumu's dance references the Goth kids' dance from *South Park* adult animated series.
  - A side-by-side comparison can be seen here.
  - Both are in turn parodying classic 'Peanut's Dance', a bizarre shuffling dance performed by the characters from the Peanuts comic and animated films.
- When Amumu dies, his pose is that of an Egyptian mummy with his arms crossed right over left.
  - This is a nod to him being a Shuriman boy prince in life.
- Amumu's price range was lowered from to in accordance with the release of his The Curse of the Sad Mummy music video.
- Amumu and **Blitzcrank** were planned to have Amumu Blitzcrank skins, but both were cancelled for failing to meet Riot's quality standards.
  - They may return in the future.
- The ward skin Sad Mummy Ward.png references him.
- He has a base of 50 which is considered medium.

---
*This page was automatically generated from League of Legends Wiki data.*