# Brand

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
| **Champion** | Brand |
| **Title** | the Burning Vengeance |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2011-04-12 |
| **Release Patch** | V1.0.0.115 |
| **Latest Changes** | V25.16 |
| **Roles** | Burst |
| **Riot Positions** | Jungle, Support |
| **External Positions** | Jungle, Middle, Support |
| **Blue Essence** | 225 |
| **Riot Points** | 260 |
| **Difficulty** | 2 |
| **Hero Type** | Mage |
| **Alt Type** | Support |
| **Adaptive Type** | Magic |
| **Damage** | 3 |
| **Toughness** | 1 |
| **Control** | 2 |
| **Mobility** | 1 |
| **Utility** | 1 |
| **Style** | 100 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $570.0$ | $+105.0$ |
| **Mana** | $469.0$ | $+21.0$ |
| **Health Regen** | $5.5$ | $+0.55$ |
| **Mana Regen** | $9.0$ | $+0.6$ |
| **Armor** | $27.0$ | $+4.2$ |
| **Magic Resist** | $30.0$ | $+1.3$ |
| **Attack Damage** | $57.0$ | $+3.0$ |
| **Attack Speed** | $0.681$ | |
| **Movement Speed** | $340.0$ | $+0.0$ |
| **Attack Range** | $550.0$ | $+0.0$ |
| **Base Attack Speed** | $0.681$ | |
| **Attack Speed Ratio** | $0.625$ | |
| **Bonus AS per Level** | $2.0\%$ | |
| **Missile Speed** | $2000$ units/second | |
| **Acquisition Radius** | $600$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $120$ units | |
| **Selection Height** | $302.778$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $95.0\%$ |
| **Damage Taken** | $110.0\%$ |

#### Ultimate Spellbook

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $95.0\%$ |

## Abilities

### Passive: Blaze

| Attribute | Value |
|-----------|------:|
| **Effect Radius** | 475 units |
| **Targeting** | Passive |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Special |

**INNATE:** **Brand**’s abilities apply a stack of *Ablaze* to enemies hit for 4 seconds, stacking up to 3 times and refreshing the duration of all stacks with each application. Subsequent abilities used against a target with *Ablaze* are empowered. Killing an enemy afflicted with *Ablaze* or any enemy with an ability restores 20–40@1–15 mana.

**ABLAZE:** While afflicted with *Ablaze*, the target is dealt magic damage equal to「 2% of their **maximum** health over 4 seconds. ⟷ 0.167% of their **maximum** health every $0.25$ seconds over 4 seconds. 」Each stack of *Ablaze* deals damage to the target individually and applies their damage ticks dependent on the time they were applied, even if the duration of all the stacks are refreshed. *Ablaze* deals 260% damage per tick to monsters; this damage is capped at「 40/4 to (40/4)*3 per tick against non-epic monsters and 80/4 to (80/4)*3 per tick against epic monsters. ⟷ a total of 40×4 to 40×4×3 against non-epic monsters and 80×4 to 80×4×3 against epic monsters. 」

Upon applying 3 stacks of *Ablaze* to a large monster or enemy champion, the fire becomes unstable, causing it form a fiery around the target that grants sight within its radius and, after 2 seconds, consumes their stacks to explode. All enemies within the detonation are applied a stack of *Ablaze* and dealt magic damage equal to 8%–12%@1–17 (+ 2% per 100 AP) of their **maximum** health, capped at 270 to 525 against monsters.

**Brand** cannot stack *Ablaze* more than once on enemies that have had a explode from them in the last 4 seconds.

*The will form even if the target dies to the ability applying the third stack, and will still trigger the explosion if the target dies before it.*

**Notes:**

- *Ablaze* is a debuff that persists through death.
- *Ablaze*’s damage cap against a monster is applied after the increased monster damage.
- The explosion is not considered a tether effect and will not affect untargetable targets, even if the explosion originated from them.
- Given the behavior of the stacking mechanic, the damage over time will deal varying amounts of damage based on the time the stacks were applied:
  - At sub-optimal applied times, 2 stacks will deal a minimum of 6% **maximum** health damage over 4 seconds, but will deal 7% **maximum** health at optimal applied times.
  - Against small-medium units 3 stacks will deal a minimum of 9% **maximum** health at sub-optimal times, but 11% **maximum** health at optimal times. If these stacks are further refreshed (By ablazing a target that's already at 3 stacks) at optimal times they can deal up to 12% **maximum** health over 4 seconds.
- The burn is persistent damage and the explosion is area damage.

---

### Q: Sear

| Attribute | Value |
|-----------|------:|
| **Range** | 1100 / er 1040 units |
| **Cast Time** | $0.25$ seconds |
| **Width** | 120 units |
| **Speed** | 1600 units/second |
| **Cost** | 70 mana |
| **Cooldown** | 8 / 7.5 / 7 / 6.5 / 6 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spell |
| **Projectile** | True |

**ACTIVE:** **Brand** launches a fireball in the target direction that deals magic damage to the first enemy hit.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 70 / 100 / 130 / 160 / 190 (+ 65% AP) |

**ABLAZE BONUS:** The target is stunned for $1.75$ seconds.

**Notes:**

- With an ability haste value of ((8 to 6)/(4-0.25+1100/1600)-1)*100 to ((8 to 6)/(4-0.25-1100/1600)-1)*100 based on the ranges of applications from the target, it is possible to apply the stun without having to use other abilities. *Sear* will apply before the duration of the pre-applied Blaze times out.
  - The needed ability haste value if you remained at the same range from the target for both casts would be ((8 to 6)/(4-0.25)-1)*100. Effect at cast time end

---

### W: Pillar of Flame

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 900 units |
| **Effect Radius** | 260 units |
| **Cost** | 60 / 70 / 80 / 90 / 100 mana |
| **Cooldown** | 10 / 9.5 / 9 / 8.5 / 8 seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Out of Range Behavior** | walk to location |

**ACTIVE:** After a delay, **Brand** erupts a pillar of flame at the target location that deals magic damage to enemies hit.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 75 / 120 / 165 / 210 / 255 (+ 60% AP) |

**ABLAZE BONUS:** The target takes 25% increased damage.

| Attribute | Value |
|-----------|------:|
| **Increased Damage** | 93.75 / 150 / 206.25 / 262.5 / 318.75 (+ 75% AP) |

**Notes:**

- The delay before the eruption does not include the cast time. The delay would be a total of if it included the cast time.

---

### E: Conflagration

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 675 units |
| **Effect Radius** | 300 (Secondary target spread radius) / 600 (Secondary target spread radius if primary target was Ablaze) units |
| **Speed** | 900 units/second |
| **Cost** | 70 / 75 / 80 / 85 / 90 mana |
| **Cooldown** | 13 / 12 / 11 / 10 / 9 seconds |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | aoe |
| **Projectile** | False |
| **Call For Help** | True |

**ACTIVE:** **Brand** sets the target enemy aflame, which creates a blast that deals magic damage to them and causes the flame to spread from the target to nearby enemies, dealing them the same damage.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 55 / 80 / 105 / 130 / 155 (+ 60% AP) |

**ABLAZE BONUS:** *Conflagration*’s spread range is doubled.

**Notes:**

- The flame spreads to enemies near the primary target at the time of cast.
  - Enemies that move out of range after the fact will still be hit, as the flame is spread as homing missiles to each of its targets.

---

### R: Pyroclasm

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 750 units |
| **Effect Radius** | 600 (Bounce range, checked when fireball arrives at a target) units |
| **Speed** | 750 - 3000 (Acceleration 500u/s) units/second |
| **Cost** | 100 mana |
| **Cooldown** | 100 / 95 / 90 / 85 / 80 seconds |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | Special |
| **Spell Effects** | spellaoe |
| **Projectile** | True |
| **Call For Help** | False |

**ACTIVE:** **Brand** launches a fireball at the target enemy that bounces between nearby enemies and **Brand** up to four times, dealing magic damage to enemies each time and having a $0.15$-second delay between bounces.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 100 / 137.5 / 175 / 212.5 / 250 (+ 25% AP) |
| **Total Single-Target Damage** | 300 / 412.5 / 525 / 637.5 / 750 (+ 75% AP) |

*Pyroclasm* prioritizes *Ablaze* enemy champions, then other enemy champions, then any valid bounce target, then **Brand**.

**ABLAZE BONUS:** The target is slowed for $0.25$ seconds. This can affect the same enemy more than once.

| Attribute | Value |
|-----------|------:|
| **Slow** | 30 / 37.5 / 45 / 52.5 / 60% |

*The target does not have to be visible for the fireball to bounce to them.*

**Notes:**

- The fireball acquires a new valid, bounce target once the bounce delay finishes.
- If the target the fireball is in flight towards becomes untargetable, the fireball missile will be destroyed but a new missile will be formed at the target's location that is able to bounce for up to the remaining bounces of the previous fireball.
  - The new missile is only formed if there is a valid target to bounce to at the time of the previous missile being destroyed.
  - This does not apply if the target is **Brand**, as the missile would not be destroyed and would bounce from him even if he is untargetable.
- The fireball will still bounce from the target even if they were dead upon arrival.
- The fireball may only bounce to valid targets, including **Brand**.
- If the initial cast target uses spell shield, *Pyroclasm* will not bounce. If the bounce is blocked by an enemy's *spell shield* it will not deal any damage, but it will keep on bouncing.
- *Pyroclasm*’s damage against enemy champions will not aggro nearby enemy minions.

---

## Patch History

### V25.16
- Blaze
  - Ablaze monster damage increased to 260% from 240%.
- Sear
  - Stun duration increased to $1.75$ seconds from $1.5$.
- Pyroclasm
  - Cooldown reduced to 100 / 90 / 80 seconds from 110 / 100 / 90.

### V25.12
- Pyroclasm
  - **Bug Fixes:** Missile now acquires a new bounce target after its bounce delay, rather than before it.
    - *[Note: The original target acquisition behavior lead to an issue where the missile would fizzle after the delay if its intended bounce target became invalid during it. The missile will now acquire a target and bounce to them at the same time, once the bounce delay finishes.*]

### V25.08
- Blaze
  - Ablaze monster damage increased to 240% from 215%.

### V25.07
- Blaze
  - Ablaze non-epic monster damage cap per second increased to 40 from 30.
  - Explosion monster damage cap increased to 270 to 525 from 250 to 475.

### V14.21
- Stats
  - Base mana regeneration reduced to 9 from $10.65$.
- Blaze
  - Explosion base damage reduced to 8% / 8.25% / 8.5% / 8.75% / 9% / 9.25% / 9.5% / 9.75% / 10% / 10.25% / 10.5% / 10.75% / 11% / 11.25% / 11.5% / 11.75% / 12% of target's **maximum** health from 9% / 9.25% / 9.5% / 9.75% / 10% / 10.25% / 10.5% / 10.75% / 11% / 11.25% / 11.5% / 11.75% / 12% / 12.25% / 12.5% / 12.75% / 13%.
- Sear
  - Mana cost increased to 70 from 50.

### V14.17
- Blaze
  - **New Effect:** Ablaze damage per second against non-epic monsters is now capped at 30.
    - *Ablaze damage cap against epic monsters unchanged.*
  - **New Effect:** Explosion now deals a maximum of 250 to 475 damage to monsters.
  - **Removed:*** Explosion no longer deals 50% damage to epic monsters.

### V14.14
- Conflagration
  - Base damage reduced to 55 / 80 / 105 / 130 / 155 from 60 / 85 / 110 / 135 / 160.
  - Cooldown increased to 13 / 12 / 11 / 10 / 9 seconds from 12 / 11 / 10 / 9 / 8.

### V14.13
- Blaze
  - Mana restore reduced to 20–40@1–15 from 30–50@1–18.
- Conflagration
  - Base damage reduced to 60 / 85 / 110 / 135 / 160 from 60 / 90 / 120 / 150 / 180.

### V14.9
- Stats
  - Selection radius increased to 120 units from $88.8889007568359$.
- Brand
  - Blaze
    - **Bug Fixes:** VFX no longer renders over terrain.
- Brand, Brand, Brand, Brand
  - Blaze
    - **Bug Fixes:** Targets' outline when hovered over using the cursor is no longer pixelated.

### V14.8
- Blaze
  - **Bug Fixes:** Can now once again trigger Dark Harvest.

## Trivia

- Brand is the second fire-themed champion, the first being Annie.
  - Sear’s animation resembles Annie’s basic attack one.
    - Brand's autoattack particle is the same as Morgana’s but orange instead of purple.
- *Brand* comes from Proto-Germanic **brandaz*, meaning "fire, torch, sword".
  - His host's given name *Mac Aodhagáin* is likely based on Gaelic patronymic * *, "son of Aed (god)" by coincidence;
    - His surname *Rodhe* possibly from Old Norse ** "praise, renown".
  - Pyroclasm derives from *pyroclastic*, a term used to describe fast-moving volcanic material, from Greek πῦρ "fire" & κλαστός "broken

---
*This page was automatically generated from League of Legends Wiki data.*