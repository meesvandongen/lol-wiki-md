# Seraphine

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
| **Champion** | Seraphine |
| **Title** | the Starry-Eyed Songstress |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2020-10-29 |
| **Release Patch** | V10.22 |
| **Latest Changes** | V25.16 |
| **Roles** | Burst, Enchanter |
| **Riot Positions** | Bottom, Support |
| **External Positions** | Bottom, Support |
| **Blue Essence** | 1575 |
| **Riot Points** | 790 |
| **Difficulty** | 1 |
| **Hero Type** | Support |
| **Alt Type** | Mage |
| **Adaptive Type** | Magic |
| **Damage** | 3 |
| **Toughness** | 1 |
| **Control** | 3 |
| **Mobility** | 1 |
| **Utility** | 2 |
| **Style** | 100 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $570.0$ | $+90.0$ |
| **Mana** | $360.0$ | $+25.0$ |
| **Health Regen** | $6.5$ | $+0.6$ |
| **Mana Regen** | $11.5$ | $+0.95$ |
| **Armor** | $26.0$ | $+4.2$ |
| **Magic Resist** | $30.0$ | $+1.3$ |
| **Attack Damage** | $50.0$ | $+3.0$ |
| **Attack Speed** | $0.669$ | |
| **Movement Speed** | $330.0$ | $+0.0$ |
| **Attack Range** | $525.0$ | $+0.0$ |
| **Base Attack Speed** | $0.669$ | |
| **Attack Speed Ratio** | $0.625$ | |
| **Bonus AS per Level** | $2.0\%$ | |
| **Missile Speed** | $1800$ units/second | |
| **Acquisition Radius** | $525$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $120$ units | |
| **Selection Height** | $100$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $100.0\%$ |
| **Damage Taken** | $120.0\%$ |
| **Healing** | $80.0\%$ |

#### URF

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $92.0\%$ |
| **Damage Taken** | $100.0\%$ |
| **Healing** | $80.0\%$ |

## Abilities

### Passive: Stage Presence

| Attribute | Value |
|-----------|------:|
| **Effect Radius** | 800 (Note granting radius) units |
| **Speed** | 3000 (Note missiles speed) units/second |
| **Targeting** | Passive |
| **Affects** | Self, Allies, Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | False |
| **Spell Effects** | proc |
| **Projectile** | True |
| **Parry** | False |

**INNATE - ECHO:** **Seraphine**’s basic ability casts each generate a stack of *Echo*, stacking up to 2 times. At 2 stacks, **Seraphine**’s next basic ability casts an additional time for 1 mana after a delay, consuming all *Echo* stacks after the cast time of the second cast.
**Seraphine** gains maximum stacks of *Echo* when the game starts and upon respawning.

**INNATE - HARMONY:** **Seraphine**’s ability casts grant a *Note* to herself and nearby allied champions that lasts 6 seconds, refreshes on subsequent *Notes* and stacks up to 4 times on each unit.

While any amount of *Notes* are active, **Seraphine**’s next basic attack is empowered to have an uncancellable windup, gain 25 **bonus** attack range per *Note*, and fire all *Notes* at the target, with each one dealing 4 to 25 (+ 4% AP) magic damage, reduced by 75% for *Notes* from allies.

**Notes:**

**Echo Details:**
- *Echo*’s current stacks are represented by a counter under **Seraphine**’s health bar, visible to all players. The last stack is highlighted to indicate that an *Echo* cast is ready.
- **Seraphine**’s basic abilities alternate their icons between 3 different ones each, depending on the current amount of stacks she has.
- The additional cast must complete for *Echo* stacks to be consumed; if the cast time does not finish or does not begin at all, or if the ability fails to cast when there is not enough mana, **Seraphine** will keep her stacks.
- The additional cast does not affect the cooldown of the mimicked ability.
- The additional cast counts as a separate cast instance for the purpose of e.g. Electrocute or Conqueror. **Harmony Details:**
- *Notes* orbit at a radius of 100 from the center of their holder and they fire from this location.
- The empowered attack can critically strike.
- The empowered attack will not trigger against wards.
- *Notes* are fired one after another with a short delay.
- **Seraphine** can grant *Notes* to more than 4 allied champions.
- *Harmony* does **not** grant *Notes* to clones.
- *Harmony* grants *Notes* even if the ally is untargetable.
- The *Notes* will fire from allies regardless of how far they are away from **Seraphine**.
- **Seraphine** gains a faint blue attack range indicator when there are *Notes* present.
- *Notes* do not discharge when attacking wards, allowing **Seraphine** to attack them from increased range multiple times. **Other Details:**
- **Seraphine**, while alive, will play music if there is at least one ally champion nearby.
  - This is audible to **Seraphine** and all allied champions that are in range of *Harmony*, indicative of if they can receive a *Note* or not.

---

### Q: High Note

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 900 units |
| **Effect Radius** | 350 units |
| **Speed** | 1300 (Missile travel speed) units/second |
| **Cost** | 60 / 70 / 80 / 90 / 100 Mana |
| **Cooldown** | 8 / 7.5 / 7 / 6.5 / 6 seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | True |
| **Out of Range Behavior** | cast at max |

**ACTIVE:** **Seraphine** hurls a soundwave to the target location that quickly expands in a radius upon arrival, dealing magic damage to enemies within the area.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 60 / 85 / 110 / 135 / 160 (+ 50% AP) |

Against champions, the damage is increased by 0%–75%@0–75 (@=target's **missing** health).

| Attribute | Value |
|-----------|------:|
| **Maximum Champion Damage** | 105 / 148.75 / 192.5 / 236.25 / 280 (+ 87.5% AP) |

**Notes:**

- *High Note* uses a modified icon for the second and third stack of Stage Presence.
- The maximum damage of *High Note* uses a cosmetic critical strike.
- *High Note* will fire from wherever *Seraphine* is at the end of the cast time.
- The impact delay depends solely on the missile speed. It is 0 seconds to $0.75$ seconds (after end of cast time) within the standard cast radius, but can be increased further by **Seraphine** being moved away from the cast location during the cast time.
- The area of effect is covered via expansion in several rings (similar to R’s area of effect) and targets can only be hit once.
- The range indicator for the target range has a radius of 950 units, but the center of *High Note* can only be cast up to 900 units. The area indicator shows the proper 350 area of effect radius and cast location.

---

### W: Surround Sound

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Effect Radius** | 800 (Note granting radius) units |
| **Cost** | 70 / 75 / 80 / 85 / 90 Mana |
| **Cooldown** | 22 seconds |
| **Targeting** | Auto |
| **Affects** | Self, Allies |
| **Projectile** | Special |

**ACTIVE:** **Seraphine** grants a shield to herself and nearby allied champions for $2.5$ seconds. For the same duration, she also gains (ms) 20% (+ 2% per 100 AP) decaying **bonus** movement speed and grants allies 8% (+ 0.8% per 100 AP) **bonus** movement speed.

| Attribute | Value |
|-----------|------:|
| **Shield Strength** | 60 / 80 / 100 / 120 / 140 (+ 20% AP) |

If **Seraphine** already had a shield at the time of cast, *Surround Sound* will pulse after the duration, healing herself and nearby allied champions, increased for each ally (including Seraphine herself).

| Attribute | Value |
|-----------|------:|
| **Heal Per Ally** | 3 / 3.5 / 4 / 4.5 / 5% of target's **missing** health |

*Surround Sound's shield and **bonus** movement speed can stack up to 2 times.*

**Notes:**

- *Surround Sound* uses a modified icon for the second and third stack of Stage Presence.
- *Surround Sound*’s effects are gained at the start of the cast time.
- **Seraphine** can move during the cast time.
- Casting *Surround Sound* again during the pulse's delay refreshes the delay.
- The heal scaling counts clones as additional allies.
- The heal will be canceled and the indicator will disappear upon **Seraphine** dying.
- Once the pulse delay ends, the heal occurs instantly on **Seraphine** and all nearby allied champions.
  - There is a missile that flows through all allied champions which can be blocked by Wind Wall, without affecting the heal however.
- *Surround Sound* will affect untargetable allies.

---

### E: Beat Drop

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 1300 units |
| **Width** | 140 units |
| **Speed** | 1200 units/second |
| **Cost** | 60 Mana |
| **Cooldown** | 11 / 10.5 / 10 / 9.5 / 9 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | True |

**ACTIVE:** **Seraphine** fires a heavy soundwave in the target direction that deals magic damage to enemies hit, reduced to 70% against minions, and slows them by 99% for a few seconds.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 70 / 100 / 130 / 160 / 190 (+ 50% AP) |
| **Minion Damage** | 49 / 70 / 91 / 112 / 133 (+ 35% AP) |

| Attribute | Value |
|-----------|------:|
| **Disable Duration** | 1.1 / 1.2 / 1.3 / 1.4 / 1.5 seconds |

Enemies that are already slowed are also rooted for the same duration.

Enemies that are immobilized or grounded are also stunned for the same duration.

**Notes:**

- *Beat Drop* will only be empowered from slows, immobilizes, and grounding effects applied by herself or her allies.
- *Beat Drop* uses a modified icon for the second and third stack of Stage Presence.
- *Beat Drop* may still be empowered even if the disable ends shortly after the ability hits.
- The root and the stun cannot apply at the same time.
  - The root will only apply if the target is slowed and not immobilized or grounded.
    - The stun will apply as normal; it does not consider if the target is slowed.
  - The root will be overridden by the stun if the target is hit by *Beat Drop* again while they were rooted by it.
- A quarter note will appear next to an enemy champion while they are slowed and two quarter notes tied will appear if they are immobilized or grounded.
  - This indicates if *Beat Drop* will apply an additional crowd control effect if it strikes the target.

- This ability will cast from wherever the caster is at the end of the cast time.

---

### R: Encore

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.5$ seconds |
| **Target Range** | 1200 units |
| **Width** | 320 units |
| **Speed** | 1600 units/second |
| **Cost** | 100 Mana |
| **Cooldown** | 160 / 150 / 140 / 130 / 120 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | True |

**ACTIVE:** **Seraphine** projects a captivating force in the target direction that deals magic damage to enemies hit, charms them, during which they are revealed, and slows them by 40% for a duration, increasing by 15% every $0.25$ seconds over the duration up to 99%.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 150 / 175 / 200 / 225 / 250 (+ 40% AP) |

| Attribute | Value |
|-----------|------:|
| **Disable Duration** | 1.25 / 1.375 / 1.5 / 1.625 / 1.75 seconds |

*Encore*’s projectile resets its remaining travel distance whenever it hits an allied or enemy champion, excluding **Seraphine**. Allied champions hit gain 4 *Notes*.

**Notes:**

- *Encore* has two cast times. Both last $0.5$ seconds. The wave will always project after the first cast time, and the second cast time only serves as a lock out for **Seraphine**.
  - **Seraphine** cannot buffer casts during the first cast time, but she may do so during the second cast time.
- Clones also count for *Encore*’s extension.
- *Encore* will not extend on enemy champions that are untargetable, but will do so for allied champions.
- If *Encore*’s projectile is negated via spell shield, this will also prevent it from resetting its travel distance.
- This ability will cast from wherever the caster is at the end of the cast time.

  - *Encore* fires from **Seraphine**’s location at the end of the initial cast time.
- There is no limit as to how many times the wave can extend.
- Each *Encore* cast cannot extend from the same champion more than once.
  - : Champions that died, clearing marker buffs

---

## Patch History

### V25.16
- Surround Sound
  - **Bug Fixes:** After ranking the ability 3 times, an Echo cast no longer illegally nullifies all ability damage that **Seraphine** and her affected allies would receive if the shield would absorb an amount of it that does not exceed half of its total amount.

### V25.15
- General
  - **Bug Fixes:** Music now properly plays for allies and no longer plays double for **Seraphine**.
- Surround Sound
  - Heal now calculates at once rather than consecutively (diminishing in effect) for each registered ally.

### V25.05
- High Note
  - Bonus champion damage increased to 0%–75%@0–75 (@=target's **missing** health) from 0%–60%@0–75 (@=target's **missing** health).

### V25.S1.1
- Seraphine
  - Removed special missions for progressively unlocking each form. Rewards are now automatically distributed after purchase.

### V14.22
- High Note
  - AP ratio reduced to 50% AP from 60% AP.
    - Maximum AP ratio against champions reduced to 80% AP from 96% AP.

### V14.17
- Surround Sound
  - Base shield reduced to 60 / 80 / 100 / 120 / 140 from 60 / 85 / 110 / 135 / 160.
  - Cooldown increased to 22 seconds at all ranks from 22 / 21 / 20 / 19 / 18.

### V14.15
- General
  - **Bug Fixes:** Abilities SFX are no longer audible through the Fog of War.

### V14.9
- High Note
  - AP ratio increased to 60% AP from 50% AP.
    - Maximum champion AP ratio increased to 96% AP from 80% AP.

### V14.5
- Stats
  - Base attack damage reduced to 50 from 55.
  - Mana growth reduced to 25 from 50.
  - Mana regeneration growth increased to $0.95$ from $0.4$.
  - Base movement speed increased to 330 from 325.
- Stage Presence
  - Base damage per note changed to 4 to 25 from 5@1; 10@6; 18@11; 30@16.
  - AP ratio per note reduced to 4% AP from 5% AP.
  - Minion damage reduced to 100% from 300%.
- High Note
  - Missile speed increased to 1300 from 1200.
  - Cooldown changed to 8 / 7.5 / 7 / 6.5 / 6 seconds from 10 / 8.75 / 7.5 / 6.25 / 5.
  - Mana cost changed to 60 / 70 / 80 / 90 / 100 from 65 / 70 / 75 / 80 / 85.
  - Base damage increased to 60 / 85 / 110 / 135 / 160 from 55 / 80 / 105 / 130 / 155.
  - Bonus damage increased to 0%–60%@0–75 (@=target's **missing** health) from 0%–50%@0–75 (@=target's **missing** health).
  - **Removed:*** Bonus damage no longer affects non-champions.
  - **Removed:*** No longer prevents minions from dying to other minions while in flight.
- Surround Sound
  - Cooldown changed to 22 / 21 / 20 / 19 / 18 seconds from 28 / 25 / 22 / 19 / 16.
  - Mana cost reduced to 70 / 75 / 80 / 85 / 90 from 80 / 85 / 90 / 95 / 100.
  - Base shield increased to 60 / 85 / 110 / 135 / 160 from 50 / 75 / 100 / 125 / 150.
  - Self bonus movement speed AP ratio reduced to 2% per 100 AP from 4% per 100 AP.
    - Ally movement speed unchanged at 40% of the self value.
      - Ally movement speed AP ratio reduced to 0.8 per 100 AP from 1.6 per 100 AP.
- Beat Drop
  - Cooldown changed to 11 / 10.5 / 10 / 9.5 / 9 seconds from 10 at all ranks.
  - Mana cost reduced to 60 at all ranks from 60 / 65 / 70 / 75 / 80.
  - Base damage changed to 70 / 100 / 130 / 160 / 190 from 60 / 95 / 130 / 165 / 200.
  - AP ratio increased to 50% AP from 35% AP.
  - Disable duration changed to 1.1 / 1.2 / 1.3 / 1.4 / 1.5 seconds from $1.25$ at all ranks.
  - Minion damage reduced to 70% from 100%.
- Encore
  - Cooldown increased to 160 / 140 / 120 seconds from 160 / 130 / 100.
  - AP ratio reduced to 40% AP from 60% AP.

### V13.22
- Stage Presence
  - Base damage per note increased to 5@1; 10@6; 18@11; 30@16 from 4@1; 8@6; 14@11; 24@16.
  - AP ratio per note reduced to 5% AP from 7% AP.
- High Note
  - Cooldown increased to 10 / 8.75 / 7.5 / 6.25 / 5 seconds from 9 / 8 / 7 / 6 / 5.
- Surround Sound
  - Shield AP ratio reduced to 20% AP from 25% AP.

## Trivia

- Her joke animation is a reference to *Dance Dance Revolution*.
- Seraphine's Champion Spotlight on the *League of Legends* YouTube channel garnered the most dislikes for any Champion, with 138k (68%) dislikes before the platform made these numbers private (circa Nov 2021). However, it remained the third most liked spotlight behind Sett and Yasuo.
  - Her spotlight received the most dislikes in Korea, totaling around 93%.
- Seraphine is the first champion to be simultaneously released in League of Legends and Wild Rift.

---
*This page was automatically generated from League of Legends Wiki data.*