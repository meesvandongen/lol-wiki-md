# Jayce

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
| **Champion** | Jayce |
| **Title** | the Defender of Tomorrow |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2012-07-07 |
| **Release Patch** | V1.0.0.142 |
| **Latest Changes** | V25.18 |
| **Roles** | Artillery |
| **Riot Positions** | Top |
| **External Positions** | Top, Middle |
| **Blue Essence** | 2400 |
| **Riot Points** | 880 |
| **Difficulty** | 3 |
| **Hero Type** | Fighter |
| **Alt Type** | Marksman |
| **Adaptive Type** | Physical |
| **Damage** | 3 |
| **Toughness** | 1 |
| **Control** | 1 |
| **Mobility** | 2 |
| **Utility** | 2 |
| **Style** | 45 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $590.0$ | $+109.0$ |
| **Mana** | $375.0$ | $+45.0$ |
| **Health Regen** | $6.0$ | $+0.6$ |
| **Mana Regen** | $6.0$ | $+0.8$ |
| **Armor** | $22.0$ | $+5.0$ |
| **Magic Resist** | $30.0$ | $+1.3$ |
| **Attack Damage** | $59.0$ | $+4.25$ |
| **Attack Speed** | $0.658$ | |
| **Movement Speed** | $335.0$ | $+0.0$ |
| **Attack Range** | $125.0$ | $+0.0$ |
| **Base Attack Speed** | $0.658$ | |
| **Attack Speed Ratio** | $0.658$ | |
| **Bonus AS per Level** | $3.0\%$ | |
| **Windup Modifier** | $0.005$ | |
| **Acquisition Radius** | $200$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $120$ units | |
| **Selection Height** | $188.889$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $105.0\%$ |

#### URF

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $92.0\%$ |
| **Damage Taken** | $105.0\%$ |

## Abilities

### Passive: Hextech Capacitor

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Self |

**INNATE:** Whenever **Jayce** switches between either **HAMMER STANCE** or **CANNON STANCE**, he gains ghosting and (ms) 40 **bonus** movement speed for $0.75$ seconds.

**Notes:**

- No additional details.

---

### Q: Shock Blast

| Attribute | Value |
|-----------|------:|
| **Range** | 1050 (Standard Range) / 1600 (Enhanced Total Range) units |
| **Cast Time** | $0.2143$ seconds |
| **Effect Radius** | 170 (Standard Blast Radius) / 250 (Enhanced Blast Radius) / sight 160 (Standard Blast Sight Radius) / 250 (Enhanced Blast Sight Radius) units |
| **Width** | 140 (Both standard and enhanced missile) units |
| **Speed** | 1450 (Standard Speed) / 2350 (Enhanced Speed) units/second |
| **Cost** | 55 / 60 / 65 / 70 / 75 / 80 Mana |
| **Cooldown** | 8 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | physical |
| **Spell Shield** | True |
| **Spell Effects** | aoe |
| **Projectile** | True |

**ACTIVE:** **Jayce** fires an electric bolt in the target direction that detonates upon hitting an enemy or reaching maximum range, dealing physical damage to nearby enemies and granting sight of the area for $1.25$ seconds.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 80 / 126 / 172 / 218 / 264 / 310 (+ 140% **bonus** AD) |

If the bolt passes through *Acceleration Gate*, it becomes supercharged: increasing its damage by 40% as well as its speed, range and explosion radius.

| Attribute | Value |
|-----------|------:|
| **Increased Damage** | 112 / 176.4 / 240.8 / 305.2 / 369.6 / 434 (+ 196% **bonus** AD) |

**Notes:**

- Upon touching Acceleration Gate, the enhanced missile gains its remaining range equal to [ 1600 units - travelled distance of the standard missile ].
  - This means that the range of the enhanced *Shock Blast* is always the same, regardless of how early or late it interacted with *Acceleration Gate*.
  - Supercharging the *Shock Blast* as soon as possible, however, will improve its average speed significantly.
    - For this, **Jayce** must stand inside or very close to the *Acceleration Gate*, or place it very close to himself right after *Shock Blast*’s cast time.
- *Shock Blast*’s effect radius is centered around the location of the missile as it collides.
  - *Shock Blast* will **always** damage the enemy it collided with, even if it is so large that it is outside the effect radius.
- The non-enhanced *Shock Blast* detonation has a slightly smaller sight radius than the radius in which it deals damage.

---

### Q: To the Skies!

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 600 units |
| **Effect Radius** | 300 (Damage radius in front of Jayce) units |
| **Cost** | 40 Mana |
| **Cooldown** | 16 / 14 / 12 / 10 / 8 / 6 seconds |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | physical |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Call For Help** | True |
| **Grounded** | True |
| **Knockdown** | True |

**ACTIVE:** **Jayce** leaps to the target enemy's location over $0.5$ seconds. Upon arrival, he smashes his hammer to the ground to deal physical damage to all enemies within an area and slow them for 2 seconds.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 60 / 105 / 150 / 195 / 240 / 285 (+ 135% **bonus** AD) |

| Attribute | Value |
|-----------|------:|
| **Slow** | 35 / 40 / 45 / 50 / 55 / 60% |

*Lightning Field can be cast during the dash.*

**Notes:**

- *To The Skies!* will always direct **Jayce** linearly to his target's location.

---

### W: Hyper Charge

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Cost** | 40 Mana |
| **Cooldown** | 13 / 11.4 / 9.8 / 8.2 / 6.6 / 5 seconds |
| **Cooldown Start** | post-effect |
| **Targeting** | Auto |
| **Affects** | Self |
| **Damage Type** | physical |
| **Spell Effects** | Proc |
| **Parry** | True |

**ACTIVE:** **Jayce** empowers his next 3 basic attacks within 4 seconds to deal ***modified** physical damage and gain (attack speed) 360% **bonus** attack speed.

*Hyper Charge*’s **total** damage is affected by critical strike modifiers.

| Attribute | Value |
|-----------|------:|
| **Damage Modifier** | 70 / 78 / 86 / 94 / 102 / 110% AD |

*Hyper Charge resets **Jayce**’s basic attack timer.*

**Notes:**

- The last basic attack is checked on-hit rather than on-attack.
  - If Runaan's Hurricane or Guinsoo's Rageblade hit the target before the basic attack does, the basic attack damage will not be modified.

---

### W: Lightning Field

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 350 units |
| **Cost** | 40 Mana |
| **Cooldown** | 10 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | magic |
| **Spell Effects** | aoedot |

**PASSIVE:** **Jayce**’s basic attacks restore mana on-hit.

| Attribute | Value |
|-----------|------:|
| **Mana Restored** | 15 / 17 / 19 / 21 / 23 / 25 |

**ACTIVE:** **Jayce** surrounds himself with an electric field for 4 seconds that deals magic damage every second to nearby enemies.

| Attribute | Value |
|-----------|------:|
| **Magic Damage Per Tick** | 35 / 50 / 65 / 80 / 95 / 110 (+ 25% AP) |
| **Total Magic Damage** | 140 / 200 / 260 / 320 / 380 / 440 (+ 100% AP) |

**Notes:**

- The mana restore triggers on structures.

---

### E: Acceleration Gate

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 650 units |
| **Width** | 750 units |
| **Cost** | 50 Mana |
| **Cooldown** | 16 seconds |
| **Targeting** | Location |
| **Affects** | Allies |
| **Out of Range Behavior** | cast at max |

**ACTIVE:** **Jayce** deploys an energy gate centered at the target location, lasting for 4 seconds and granting sight of its surroundings. **Jayce** and all allied champions can move through it to gain (ms) **bonus** movement speed that decays over 3 seconds.

| Attribute | Value |
|-----------|------:|
| **Bonus Movement Speed** | 35 / 40 / 45 / 50 / 55 / 60% |

*The buff is continuously refreshed while inside the gate.*

**Notes:**

- The *Gate* grants the bonus in an area er 750 units wide and er 100 units thick.

---

### E: Thundering Blow

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 240 units |
| **Cost** | 55 Mana |
| **Cooldown** | 20 / 18 / 16 / 14 / 12 / 10 seconds |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | magic |
| **Spell Shield** | True |
| **Spell Effects** | single |
| **Call For Help** | True |

**ACTIVE:** **Jayce** roots the target enemy over the cast time, then swings his hammer at them to deal magic damage, capped against monsters, and knock them back 600 units.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 8 / 10.8 / 13.6 / 16.4 / 19.2 / 22% of target's **maximum** health (+ 100% **bonus** AD) |
| **Maximum Monster Damage** | 200 / 300 / 400 / 500 / 600 / 700 |

***Jayce** is unable to cast To the Skies! or Shock Blast for $0.4$ seconds after Thundering Blow's cast time.*

**Notes:**

No additional notes.

---

### R: Transform Mercury Cannon

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Cooldown** | 6 seconds |
| **Targeting** | Auto |
| **Affects** | Self |
| **Parry** | True |

**ACTIVE:** **Jayce** transforms into Transform Mercury Cannon.png, receiving access to its abilities, becoming ranged with 500 attack range, and empowering his next basic attack to reduce the target's (armor penetration) armor and (magic penetration) magic resistance by 10%–25%@1–16 for 5 seconds.

**Notes:**

- Transformations do not count as ability activations for the purposes of on-cast effects such as Spellblade and triggering Force Pulse’s passive.
- The resistance reduction does not apply to structures.
- Both *Transform* on-hit effects have no set duration and will only be consumed when *Jayce* either lands a basic attack or switches stances.
- The enhanced attack will not be consumed if it is parried (dodge, block, blind, and Riposte).
- The empowered attack will not trigger against structures nor wards.

---

### R: Transform Mercury Hammer

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Cooldown** | 6 seconds |
| **Targeting** | Auto |
| **Affects** | Self |
| **Parry** | True |

**ACTIVE:** **Jayce** transforms into Transform Mercury Hammer.png, receiving access to its abilities, becoming melee with 125 attack range, gaining 5–35@1–16 (+ $7.5$% **bonus** AD) (armor) **bonus** armor and (mr) **bonus** magic resistance, and empowering his next basic attack to deal 25–130@1–16 (+ 30% **bonus** AD) **bonus** magic damage.

**Jayce** begins the game with *Transform* but cannot increase its rank. Instead, his basic abilities each have 6 ranks.

**Notes:**

- The enhanced attack will not be consumed if it is parried (dodge, block, blind, and Riposte).
- The *Mercury Hammer*’s bonus damage can affect structures.
- Both *Transform* on-hit effects have no set duration and will only be consumed when **Jayce** either lands a basic attack or switches stances.
- The empowered attack will not trigger against wards.

---

### Hextech Capacitor 2

**Notes:**

No additional notes.

---

## Patch History

### V25.18
- Shock Blast
  - Base damage increased to 80 / 126 / 172 / 218 / 264 / 310 from 60 / 110 / 160 / 210 / 260 / 310.
    - Empowered damage increased to 112 / 176.4 / 240.8 / 305.2 / 369.6 / 434 from 84 / 154 / 224 / 294 / 364 / 434.

### V25.17
- General
  - **Bug Fixes:** Other players no longer see a debug icon on his status bar in his target frame.
- Jayce
  - Transform Mercury Cannon
    - **Bug Fixes:** Restored empowered attack primed SFX.

### V25.14
- General
  - Default range type changed to ranged from melee.
    - *[Note: Jayce starts the game in melee form, so this is automatically overriden.]*

### V25.04
- Shock Blast
  - **Bug Fixes:** Corrected the projectile's rendering when it passes over impassable terrain and structures.
- Hyper Charge
  - Bonus attack speed increased to 360% from 300%.

### V25.S1.3
- To the Skies!
  - Base damage reduced to 60 / 105 / 150 / 195 / 240 / 285 from 60 / 110 / 160 / 210 / 260 / 310.
  - Bonus AD ratio increased to 135% **bonus** AD from 120% **bonus** AD.
- Shock Blast
  - Base damage reduced to 60 / 110 / 160 / 210 / 260 / 310 from 60 / 115 / 170 / 225 / 280 / 335.
    - Increased base damage reduced to 84 / 171.5 / 259 / 346.5 / 434 from 84 / 180.25 / 276.5 / 372.75 / 469.
  - Bonus AD ratio increased to 140% **bonus** AD from 125% **bonus** AD.
    - Increased bonus AD ratio increased to 196% **bonus** AD from 175% **bonus** AD
- Lightning Field
  - Base damage per tick reduced to 35 / 50 / 65 / 80 / 95 / 110 from 40 / 55 / 70 / 85 / 100 / 115.
    - Total damage reduced to 140 / 200 / 260 / 320 / 380 / 440 from 160 / 220 / 280 / 340 / 400 / 460.
- Transform Mercury Hammer
  - On-hit base damage reduced to 25–130@1–16 from 25–145@1–16.
  - On-hit bonus AD ratio increased to 30% **bonus** AD from 25% **bonus** AD.

### V25.S1.1
- General
  - Now swaps Axiom Arcanist with Nimbus Cloak.

### V14.24
- General
  - **Bug Fixes:** No longer has a "jayceismelee" buff on the status bar, which also had an empty icon, while in Hammer form.
- Jayce
  - Renamed to *Arcane Inventor* from *Arcane*.

### V14.18
- To the Skies!
  - Slow increased to 35 / 40 / 45 / 50 / 55 / 60% from 30 / 35 / 40 / 45 / 50 / 55%.
- Acceleration Gate
  - Bonus movement speed increased to 35 / 40 / 45 / 50 / 55 / 60% from 30 / 35 / 40 / 45 / 50 / 55%.

### V14.14
- Shock Blast
  - Base damage increased to 60 / 115 / 170 / 225 / 280 / 335 from 55 / 110 / 165 / 220 / 275 / 330.
  - Bonus AD ratio increased to 125% **bonus** AD from 120%.
- Thundering Blow
  - **Bug Fixes:** Now always turns him to face its target upon using the ability.

### V14.9
- Stats
  - Selection radius increased to 120 units from 75.

## Trivia

- Jayce, Elise, Gnar, Nidalee, and Kayle are the only champions to be conditionally considered both ranged and melee.
- Jayce was the 100th released champion.
- Jayce's release marks...
  - The first champion to have access to eight abilities (Transform Mercury Cannon and Transform Mercury Hammer counted separately).
  - The third champion to have a dedicated Recall animation on release, the previous ones being Darius and Draven.
  - The third champion able to start the game with an unlocked Ultimate ability, the previous ones being Udyr and Karma. Nidalee's ultimate at the time still required level 6.
  - The fourth champion to have access to all four abilities at Level 1, the other ones being Elise, Karma, Nidalee, and Udyr.
- Jayce's dance references U Can't Touch This by MC Hammer.
  - A side-by-side comparison can be seen here.
  - He shares this dance with Malzahar (specifically his Voidlings’ part).
  - Coincidentally, 'Mercury Cannon / Hammer' can be shortened to 'MC Hammer'.
  - One of Jayce's Series 2 Eternals: *Hammer Time* also references this.
- Jayce's title, *"the Defender of Tomorrow"*, likely references Superman: The Man of Tomorrow.

---
*This page was automatically generated from League of Legends Wiki data.*