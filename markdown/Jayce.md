# Jayce

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
| **Champion** | Jayce |
| **Title** | the Defender of Tomorrow |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2012-07-07 |
| **Release Patch** | V1.0.0.142 |
| **Roles** | Artillery |
| **Riot Positions** | Top |
| **External Positions** | Top, Middle |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $590.0$ | $+109.0$ | $2443.0$ |
| **Mana** | $375.0$ | $+45.0$ | $1140.0$ |
| **Health Regen** | $6.0$ | $+0.6$ | $16.2$ |
| **Mana Regen** | $6.0$ | $+0.8$ | $19.6$ |
| **Armor** | $22.0$ | $+5.0$ | $107.0$ |
| **Magic Resist** | $30.0$ | $+1.3$ | $52.1$ |
| **Attack Damage** | $59.0$ | $+4.25$ | $131.2$ |
| **Attack Speed** | $0.658$ | $+3.0\%$ | $0.994$ |
| **Movement Speed** | $335.0$ | $+0.0$ | $335.0$ |
| **Attack Range** | $125.0$ | $+0.0$ | $125.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.658$ |
| **Attack Speed Ratio** | $0.658$ |
| **Bonus AS per Level** | $3.0\%$ |
| **Windup Modifier** | $0.005$ |
| **Acquisition Radius** | $200 units$ |
| **Pathing Radius** | $35 units$ |
| **Selection Radius** | $120 units$ |
| **Selection Height** | $188.889 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Hextech Capacitor

**Innate:** Whenever **Jayce** switches between ***Hammer Stance*** and ***Cannon Stance***, he briefly gains **bonus movement speed** and ghosting unit collision.

**Innate:** Whenever **Jayce** switches between either ***Hammer Stance*** or ***Cannon Stance***, he gains ghosting and ms*bonus** movement speed* for $0.75$ seconds.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Self |

**Notes:**

- No additional details.

---

### Q: Shock Blast

**Active:** **Jayce** fires an electric bolt in the target direction that detonates upon hitting an enemy or max range, dealing physical damage to nearby enemies.

*If the bolt passes through **Acceleration Gate**, it supercharges: instantly increasing its speed, travel distance and detonation.*

**Active:** **Jayce** fires an electric bolt in the target direction that detonates upon hitting an enemy or reaching maximum range, dealing physical damage to nearby enemies and granting sight of the area for $1.25$ seconds. If the bolt passes through **Acceleration Gate**, it becomes supercharged: increasing its damage by 40% as well as its speed, range and explosion radius.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | 8 seconds |
| **Cast Time** | $0.2143$ seconds |
| **Cost** | $55-80 6$ Mana |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | physical |
| **Speed** | 1450 / 2350 units/second |
| **Effect Radius** | 170 / 250 / sight 160 / 250 units |
| **Spell Shield** | True |
| **Spell Effects** | aoe |
| **Projectile** | True |

**Scaling:**
- **Physical Damage:** $80-310 6$ bonus AD)
- **Increased Damage:** $80×1.4-310×1.4 6$ (+ $140×1.4$% bonus AD)

**Notes:**

- Upon touching *Acceleration Gate*, the enhanced missile gains its remaining range equal to [ 1600 units - travelled distance of the standard missile ].
  - This means that the range of the enhanced *Shock Blast* is always the same, regardless of how early or late it interacted with *Acceleration Gate*.
  - Supercharging the *Shock Blast* as soon as possible, however, will improve its average speed significantly. ** For this, Jayce*** must stand inside or very close to the *Acceleration Gate*, or place it very close to himself right after 'Shock Blast's cast time.
- 'Shock Blast's effect radius is centered around the location of the missile as it collides.
  - *Shock Blast* will **always** damage the enemy it collided with, even if it is so large that it is outside the effect radius.
- The non-enhanced *Shock Blast* detonation has a slightly smaller sight radius than the radius in which it deals damage.

---

### Q: To the Skies!

**Active:** **Jayce** leaps to the target enemy's location and smashes his hammer to the ground, dealing physical damage and slow nearby enemies.

**Active:** **Jayce** dash to the target enemy's location over $0.5$ seconds. Upon arrival, he smashes his hammer to the ground to deal physical damage to all enemies within an area and slow them for 2 seconds. **Lightning Field* can be cast during the dash.*

| Attribute | Value |
|-----------|-------|
| **Range** | 600 units |
| **Cooldown** | $16-6 6$ seconds |
| **Cast Time** | none |
| **Cost** | 40 Mana |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | physical |
| **Effect Radius** | 300 units |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |

**Scaling:**
- **Physical Damage:** $60-285 6$ bonus AD)
- **Slow:** $35-60 6$%

**Notes:**

- *To The Skies!* will always direct **Jayce** linearly to his target's location.

---

### W: Hyper Charge

**Active:** **Jayce** gains a massive burst of **bonus attack speed** on his next few basic attacks, each one dealing *modified physical damage.

**Active:** **Jayce** empowers his next 3 basic attacks within 4 seconds to deal *modified physical damage and gain attack speedbonus attack speed*. 'Hyper Charge's **total** damage is affected by critical strike modifiers. *Hyper Charge basic attack reset *'Jayce's* basic attack timer.*

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $13-5 6$ seconds |
| **Cast Time** | none |
| **Cost** | 40 Mana |
| **Targeting** | Auto |
| **Affects** | Self |
| **Damage Type** | physical |
| **Spell Effects** | Proc |

**Scaling:**
- **Damage Modifier:** $70-110 6$% AD

**Notes:**

- The last basic attack is checked on-hit rather than on-attack.
  - If *Runaan's Hurricane* or *Guinsoo's Rageblade* hit the target before the basic attack does, the basic attack damage will not be modified.

---

### W: Lightning Field

**Passive:** **Jayce**’s basic attacks restore *mana* on-hit.

**Active:** **Jayce** surrounds himself with an electric field for a few seconds, continually dealing magic damage to nearby enemies.

**Passive:** ''Jayce's' basic attacks restore *mana* on-hit. **Active:** **Jayce** surrounds himself with an electric field for 4 seconds that deals magic damage every second to nearby enemies.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | 10 seconds |
| **Cast Time** | none |
| **Cost** | 40 Mana |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | magic |
| **Effect Radius** | 350 units |
| **Spell Effects** | aoedot |

**Scaling:**
- **Mana Restored:** $15-25 6$
- **Magic Damage Per Tick:* $35-110 6$ (+ 25% AP)4-110×4 6$ (+ 100% AP)

**Notes:**

- The restore triggers on structures.

---

### E: Acceleration Gate

**Active:** **Jayce** deploys an energy gate centered at the target location for a few seconds. All allied champions that touch the gate will gain a burst of *movement speed*.

**Active:** **Jayce** deploys an energy gate centered at the target location, lasting for 4 seconds and granting sight of its surroundings. **Jayce** and all allied champions can move through it to gain *ms **bonus** movement speed* that decays over 3 seconds. *The buff is continuously refreshed while inside the gate.*

| Attribute | Value |
|-----------|-------|
| **Range** | 650 units |
| **Cooldown** | 16 seconds |
| **Cast Time** | none |
| **Cost** | 50 Mana |
| **Targeting** | Location |
| **Affects** | Allies |

**Scaling:**
- **Bonus Movement Speed:** $35-60 6$%

**Notes:**

- The *Gate* grants the bonus in an area er 750 units wide and er 100 units thick.

---

### E: Thundering Blow

**Active:** **Jayce** swings his hammer at the target enemy, dealing magic damage based on the ''target's' maximum health and airborne them back.

**Active:** **Jayce** root the target enemy over the cast time, then swings his hammer at them to deal magic damage, capped against monsters, and airborne 600 units. '**Jayce** is unable to cast *To the Skies!* or *Shock Blast* for $0.4$ seconds after Thundering Blow's cast time.'

| Attribute | Value |
|-----------|-------|
| **Range** | 240 units |
| **Cooldown** | $20-10 6$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | 55 Mana |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | magic |
| **Spell Shield** | True |
| **Spell Effects** | single |

**Scaling:**
- **Magic Damage:** $8-22 6$
- **maximum** health (+ 100%
- *bonus AD) Maximum Monster Damage $200-700 6$

**Notes:**

No additional notes.

---

### R: Transform Mercury Cannon

**Active:** **Jayce** transforms into Transform Mercury Cannon.png, becoming ranged with greater *attack range* and gaining new abilities.

*His next basic attack will reduce the target's lethality and mpen for a few seconds.*

**Active:** **Jayce** transforms into Transform Mercury Cannon.png, receiving access to its abilities, becoming ranged with *500 attack range*, and empowering his next basic attack to reduce the target's armor penetration and magic penetration by key=% for 5 seconds.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | 6 seconds |
| **Cast Time** | none |
| **Targeting** | Auto |
| **Affects** | Self |

**Notes:**

- Transformations do not count as ability activations for the purposes of on-cast effects such as Spellblade and triggering *Force Pulse*’s passive.
- The resistance reduction does not apply to structures.
- Both *Transform* on-hit effects have no set duration and will only be consumed when *Jayce* either lands a basic attack or switches stances.
- The enhanced attack will not be consumed if it is parried (dodge, block, blind, and *Riposte*).
- The empowered attack will not trigger against structures nor wards.

---

### R: Transform Mercury Hammer

**Active:** **Jayce** transforms into Transform Mercury Hammer.png, becoming melee and gaining new abilities.

*His next basic attack will deal **bonus** magic damage.*

**Active:** **Jayce** transforms into Transform Mercury Hammer.png, receiving access to its abilities, becoming melee with *125 attack range*, gaining 5–35@1–16 (+ $7.5$% *bonus AD) *armor *bonus armor* and *mr **bonus** magic resistance*, and empowering his next basic attack to deal 25–130@1–16 (+ 30% *bonus AD) **bonus** magic damage. **Jayce** begins the game with *Transform* but cannot increase its rank. Instead, his basic abilities each have 6 ranks.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | 6 seconds |
| **Cast Time** | none |
| **Targeting** | Auto |
| **Affects** | Self |

**Notes:**

- The enhanced attack will not be consumed if it is parried (dodge, block, blind, and *Riposte*).
- The 'Mercury Hammer's bonus damage can affect structures.
- Both *Transform* on-hit effects have no set duration and will only be consumed when **Jayce** either lands a basic attack or switches stances.
- The empowered attack will not trigger against wards.

---

### Hextech Capacitor 2

**Notes:**

No additional notes.

---

## Patch History

### V25.18
- *Shock Blast*
  - Base damage increased to $80-310 6$ from $60-310 6$.
    - *Empowered* damage increased to $80×1.4-310×1.4 6$ from $60×1.4-310×1.4 6$.

### V25.17
- General
  - **Bug Fixes:** Other players no longer see a debug icon on his status bar in his target frame.
- Jayce
  - *Transform Mercury Cannon*
    - **Bug Fixes:** Restored empowered attack primed SFX.

### V25.14
- General
  - Default range type changed to ranged from melee.
    - *[Note: Jayce starts the game in melee form, so this is automatically overriden.]*

### V25.04
- *Shock Blast*
  - **Bug Fixes:** Corrected the projectile's rendering when it passes over impassable terrain and structures.
- *Hyper Charge*
  - Bonus attack speed increased to 360% from 300%.
- *To the Skies!*
  - Base damage reduced to $60-285 6$ from $60-310 6$.
  - Bonus AD ratio increased to 135% *bonus AD from 120% *bonus AD.
- *Shock Blast*
  - Base damage reduced to $60-310 6$ from $60-335 6$.
    - Increased base damage reduced to $60×1.4-310×1.4$ from $60×1.4-335×1.4$.
  - Bonus AD ratio increased to 140% *bonus AD from 125% *bonus AD.
    - Increased bonus AD ratio increased to $140×1.4$% *bonus AD from $125×1.4$% *bonus AD
- *Lightning Field*
  - Base damage per tick reduced to $35-110 6$ from $40-115 6$.
    - Total damage reduced to $35×4-110×4 6$ from $40×4-115×4 6$.
- *Transform Mercury Hammer*
  - On-hit base damage reduced to 25–130@1–16 from 25–145@1–16.
  - On-hit bonus AD ratio increased to 30% *bonus AD from 25% *bonus AD.
- General
  - Now swaps *Axiom Arcanist* with *Nimbus Cloak*.

### V14.24
- General
  - **Bug Fixes:** No longer has a "jayceismelee" buff on the status bar, which also had an empty icon, while in Hammer form.
- Jayce
  - Renamed to *Arcane Inventor* from *Arcane*.

### V14.18
- *To the Skies!*
  - Slow increased to $35-60 6$% from $30-55 6$%.
- *Acceleration Gate*
  - Bonus movement speed increased to $35-60 6$% from $30-55 6$%.

### V14.14
- *Shock Blast*
  - Base damage increased to $60-335 6$ from $55-330 6$.
  - Bonus AD ratio increased to 125% *bonus AD from 120%.
- *Thundering Blow*
  - **Bug Fixes:** Now always turns him to face its target upon using the ability.

### V14.9
- Stats
  - Selection radius increased to 120 units from 75.

### V14.4
- Stats
  - Base attack damage increased to 59 from 57.
- *Lightning Field*
  - Mana restoration increased to $15-25 6$ from $10-20 6$.
  - Base damage per tick increased to $40-115 6$ from $35-110 6$.
    - Total base damage increased to $160-460 6$ from $140-440 6$.
- *Thundering Blow*
  - **New Effect:** Now root the target during the cast time.

### V14.2
- *Thundering Blow*
  - Knock back distance increased to 600 units from 500.
  - **Removed:*** No longer has a fixed displacement location for the target.

## Trivia

- Jayce, **Elise**, **Gnar**, **Nidalee**, and **Kayle** are the only champions to be conditionally considered both ranged and melee.
- Jayce was the 100th released champion.
- Jayce's release marks...
  - The first champion to have access to eight abilities (*Transform Mercury Cannon* and *Transform Mercury Hammer* counted separately).
  - The third champion to have a dedicated Recall animation on release, the previous ones being **Darius** and **Draven**.
  - The third champion able to start the game with an unlocked Ultimate ability, the previous ones being **Udyr** and **Karma**. **Nidalee**'s ultimate at the time still required level 6.
  - The fourth champion to have access to all four abilities at Level 1, the other ones being **Elise**, **Karma**, **Nidalee**, and **Udyr**.
- Jayce's dance references U Can't Touch This by MC Hammer.
  - A side-by-side comparison can be seen here.
  - He shares this dance with **Malzahar** (specifically his *Voidlings*’s part).
  - Coincidentally, 'Mercury *Cannon* / *Hammer*' can be shortened to 'MC Hammer'.
  - One of Jayce's Series 2 Eternals: *Hammer Time* also references this.
- Jayce's title, *"the Defender of Tomorrow"*, likely references Superman: The Man of Tomorrow.

---
*This page was automatically generated from League of Legends Wiki data.*