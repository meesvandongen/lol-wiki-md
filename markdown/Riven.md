# Riven

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
| **Champion** | Riven |
| **Title** | the Exile |
| **Resource** | None |
| **Range Type** | Melee |
| **Release Date** | 2011-09-14 |
| **Release Patch** | V1.0.0.125 |
| **Latest Changes** | V25.14 |
| **Roles** | Skirmisher |
| **Riot Positions** | Top |
| **External Positions** | Top |
| **Blue Essence** | 2400 |
| **Riot Points** | 880 |
| **Difficulty** | 3 |
| **Hero Type** | Fighter |
| **Alt Type** | Assassin |
| **Adaptive Type** | Physical |
| **Damage** | 3 |
| **Toughness** | 2 |
| **Control** | 2 |
| **Mobility** | 3 |
| **Utility** | 1 |
| **Style** | 75 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $630.0$ | $+100.0$ |
| **Mana** | $0.0$ | $+0.0$ |
| **Health Regen** | $8.5$ | $+0.5$ |
| **Armor** | $33.0$ | $+4.4$ |
| **Magic Resist** | $32.0$ | $+2.05$ |
| **Attack Damage** | $64.0$ | $+3.0$ |
| **Attack Speed** | $0.625$ | |
| **Movement Speed** | $340.0$ | $+0.0$ |
| **Attack Range** | $125.0$ | $+0.0$ |
| **Base Attack Speed** | $0.625$ | |
| **Attack Speed Ratio** | $0.625$ | |
| **Bonus AS per Level** | $3.5\%$ | |
| **Acquisition Radius** | $600$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $130$ units | |
| **Selection Height** | $150$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $105.0\%$ |
| **Damage Taken** | $92.0\%$ |

#### URF

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $100.0\%$ |
| **Damage Taken** | $95.0\%$ |

## Abilities

### Passive: Runic Blade

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Enemies |
| **Damage Type** | physical |
| **Spell Effects** | Proc |
| **Parry** | True |

**INNATE:** **Riven**’s ability casts generate a stack of *Charge* for 6 seconds, refreshing on subsequent casts and stacking up to 3 times. **Riven**’s basic attacks are empowered to each consume a stack to deal **bonus** physical damage equal to key=% AD, reduced toagainst structures.

The **bonus** damage is affected by critical strike modifiers and applies life steal at 100% effectiveness.

**Notes:**

- *Runic Blade*’s duration also refreshes when **Riven** consumes a stack.
- The empowered attack will not trigger against wards.
- The number of empowered attacks available is represented by a counter under her health bar, only visible to the player.

---

### Q: Broken Wings

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | $150$ (Standard first cast's radius) / 250 (Standard final cast's radius) / $200$ (Enhanced first cast's radius) / 300 (Enhanced final cast's radius) |
| **Cooldown** | 13 (Starts after first cast) seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Auto / Unit |
| **Affects** | Enemies |
| **Damage Type** | physical |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Out of Range Behavior** | The ability will cast in the direction of the targeted enemy regardless of range |
| **Terrain Grace** | True |
| **Grounded** | True |
| **Knockdown** | True |

**ACTIVE:** **Riven** can activate *Broken Wings* three times before the ability goes on cooldown, with a $0.3125$-second static cooldown (Unaffected by ability haste) between casts. If **Riven** does not recast the ability within 4 seconds of the previous cast, it goes on cooldown.

**Riven** slashes with her sword for each of the three casts, dealing physical damage to enemies struck within an area, resetting her basic attack timer, and ordering her to basic attack the target of *Broken Wings* if there are any.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 45 / 75 / 105 / 135 / 165 (+ 65 / 70 / 75 / 80 / 85% **bonus** AD) |
| **Total Physical Damage** | 135 / 225 / 315 / 405 / 495 (+ 195 / 210 / 225 / 240 / 255% **bonus** AD) |

**FIRST CAST:** **Riven** dashes 225 units in the direction she is currently facing, or up to 225 units towards the target enemy, striking enemies in the target area 100 units away. This cast cannot cross terrain.

**SECOND CAST:** **Riven** mimics the first cast's effects.

**THIRD CAST:** **Riven** mimics the first cast's effects in a larger area while also knocking back enemies hit 75 units over $0.5$ seconds. This cast can cross terrain.

**Notes:**

- Each cast counts as a single ability activation for the purposes of on-cast effects such as Spellblade and triggering Force Pulse’s passive.
- The basic attack reset is not considered one for Hail of Blades.
- Moving after using *Broken Wings* allows Riven to do other actions faster.
- The **THIRD CAST** prevents **Riven** from using basic attacks and abilities for longer than the other 2 casts.
- The **THIRD CAST** may have an extended dash range when crossing terrain.

---

### W: Ki Burst

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Effect Radius** | cr |
| **Cooldown** | 11 / 10 / 9 / 8 / 7 seconds |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | physical |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |

**ACTIVE:** **Riven** emits a flash of runic energy, dealing physical damage to nearby enemies and stunning them for $0.75$ seconds.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 65 / 95 / 125 / 155 / 185 (+ 100% **bonus** AD) |

**Notes:**

- **Riven** is briefly unable to basic attack after the cast time.
- Q may be cast while basic attacking briefly after casting *Ki Burst*.
- *Ki Burst*’s effects occur before the cast time.

---

### E: Valor

| Attribute | Value |
|-----------|------:|
| **Range** | 250 units |
| **Cast Time** | none |
| **Cooldown** | 10 / 9 / 8 / 7 / 6 seconds |
| **Targeting** | Direction |
| **Affects** | Self |
| **Grounded** | True |
| **Knockdown** | True |

**ACTIVE:** **Riven** dashes in the target direction, though not through terrain, while granting herself a shield for $1.5$ seconds.

| Attribute | Value |
|-----------|------:|
| **Shield Strength** | 70 / 95 / 120 / 145 / 170 (+ 110% **bonus** AD) |

***Riven** can cast any of her abilities during the dash. Valor will cast at max range if cast beyond that.*

**Notes:**

- Q may be cast during W or R cast times if they're used at the end or briefly after the dash.

---

### R: Blade of the Exile

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Cooldown** | 120 / 105 / 90 / 75 / 60 seconds |
| **Cooldown Start** | post-effect |
| **Targeting** | Auto |
| **Affects** | Self |

**ACTIVE:** **Riven** empowers her blade for 15 seconds, gaining 25% AD ad, range, and increased range on *Broken Wings* and *Ki Burst*.

 After $0.5$ seconds, she can cast *Wind Slash* within the duration.

**Notes:**

- All bonuses are gained after $0.5$ seconds of the ability starting its cast.
  - The **bonus** attack damage amount is factored upon cast, and does not change.
- If **Riven** has a basic attack buffered before the end of *Blade of the Exile*’s cast time, the buffered basic attack will start $0.33$ seconds after the end of the cast time.
  - The delay will not apply to basic attack commands input after *Blade of the Exile*’s cast time.

---

### R: Wind Slash

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 1100 (Range of each missile (see notes)) units |
| **Angle** | 18° |
| **Width** | 200 (Width of each missile (see notes)) units |
| **Speed** | 1600 units/second |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | physical |
| **Spell Shield** | True |
| **Spell Effects** | aoe |
| **Projectile** | True |

**ACTIVE:** **Riven** unleashes a wave of energy in a cone in the target direction that deals physical damage to enemies hit, increased by type= target's **missing** health.

| Attribute | Value |
|-----------|------:|
| **Minimum Physical Damage** | 100 / 125 / 150 / 175 / 200 (+ 60% **bonus** AD) |
| **Maximum Physical Damage** | 300 / 375 / 450 / 525 / 600 (+ 180% **bonus** AD) |

**Notes:**

- *Wind Slash* counts as an ability activation for the purposes of on-cast effects such as Spellblade and triggering Force Pulse’s passive. Effect at cast time end
- *Wind Slash*’s wave is made up of three individual missiles spread in a cone.
  - Each enemy can be damaged by only one missile.
- If *Wind Slash* hits an enemy while *Riven* is not visible, it will briefly grant sight of the area around her to enemies.
- If **Riven** has a basic attack buffered before the end of *Wind Slash*’s cast time, the buffered basic attack will start $0.528$ seconds after the end of the cast time.
  - The delay is not applied if a basic attack command is input after the cast time.

---

## Patch History

### V25.14#July 21st Hotfix|V25.14
- Broken Wings
  - **Bug Fixes:** Third cast no longer sometimes has a greatly increased knockback distance against Blue Siege Minion and champions.

### V25.14
- Runic Blade
  - AD ratio reduced to key=% from key=%. *Formula is now also fully linear and no longer features a breakpoint at level 18.*
- Broken Wings
  - The third cast's displacement no longer prematurely ends when the affected enemy collides with a wall during it.
  - Occasionally, the third cast's knockback distance will be greatly increased.
    - This is not intended.

### V14.24
- Valor
  - Base shield reduced to 70 / 95 / 120 / 145 / 170 from 80 / 105 / 130 / 155 / 180.

### V14.20
- General
  - Recommended runes updated.
    - Legend: Alacrity to Legend: Haste.
    - Last Stand to Cut Down.
    - Nimbus Cloak to Gathering Storm.
    - Bone Plating to Second Wind.
    - Unflinching to Shield Bash.
- Broken Wings
  - Base damage increased to 45 / 75 / 105 / 135 / 165 from 15 / 35 / 55 / 75 / 95.
  - AD ratio changed to 65 / 70 / 75 / 80 / 85% **bonus** AD from 50 / 55 / 60 / 65 / 70% **total** AD.

### V14.2
- Riven and Riven
  - Valor
    - **Bug Fixes:** VFX no longer unintentionally renders over impassable terrain and structures.
- Riven
  - **Bug Fixes:** VFX for her dance emote, Recall, and respawn at the base no longer become cut-off when triggered on lower graphic settings.
- Riven
  - **Bug Fixes:** No longer causes her feet to slide on the ground during her idle animations.

### V13.23
- Broken Wings
  - AD ratio increased to 50 / 55 / 60 / 65 / 70% AD from 45 / 50 / 55 / 60 / 65% AD.
- Blade of the Exile
  - Bonus attack damage increased to 25% AD from 20% AD.

### V13.22
- Riven
  - Skin renamed to *Reignited Worlds 2012 Riven* from *Worlds 2016 Riven*.

### V13.20
- Riven
  - Skin renamed to *Worlds 2012 Riven* from *Championship Riven*.
- Riven
  - Skin renamed to *Worlds 2016 Riven* from *Championship Riven 2016*.

### V13.4
- Runic Blade
  - AD ratio increased to key=% AD from key=% AD. *Now scales linearly.*
  - **New Effect:** Now applies bonus damage against structures at 50% effectiveness.
  - **New Effect:** Stacks now display on health bar for the player.

### V12.10
- Stats
  - Base health increased to 630 from 560.
  - Health growth increased to 100 from 86.
  - Armor growth increased to $4.4$ from $3.2$.
  - Magic resistance growth increased to $2.05$ from $1.25$.
- Valor
  - Base shield reduced to 80 / 105 / 130 / 155 / 180 from 85 / 115 / 145 / 175 / 205.
  - Shield bonus AD ratio reduced to 110% **bonus** AD from 120%.

## Trivia

- This champion has no ability power ratio.
- Riven's dance resembles the Water Dance of Braavos from Game of Thrones.
- Runic Blade used to be called 'Art of War' (can be seen in Riven's Champion Spotlight) while Ki Burst used to be called 'Ki Shout'.
- During development she was simply called *Marth* or *Exile*.
- *Riven* is the past participle of English verb *to rive* "split or tear apart violently".
  - *Riven, the Myst III: Exile* might be referencing Myst (series).

---
*This page was automatically generated from League of Legends Wiki data.*