# Brand

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
| **Champion** | Brand |
| **Title** | the Burning Vengeance |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2011-04-12 |
| **Release Patch** | V1.0.0.115 |
| **Roles** | Burst |
| **Riot Positions** | Jungle, Support |
| **External Positions** | Jungle, Middle, Support |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $570.0$ | $+105.0$ | $2355.0$ |
| **Mana** | $469.0$ | $+21.0$ | $826.0$ |
| **Health Regen** | $5.5$ | $+0.55$ | $14.9$ |
| **Mana Regen** | $9.0$ | $+0.6$ | $19.2$ |
| **Armor** | $27.0$ | $+4.2$ | $98.4$ |
| **Magic Resist** | $30.0$ | $+1.3$ | $52.1$ |
| **Attack Damage** | $57.0$ | $+3.0$ | $108.0$ |
| **Attack Speed** | $0.681$ | $+2.0\%$ | $0.913$ |
| **Movement Speed** | $340.0$ | $+0.0$ | $340.0$ |
| **Attack Range** | $550.0$ | $+0.0$ | $550.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.681$ |
| **Attack Speed Ratio** | $0.625$ |
| **Bonus AS per Level** | $2.0\%$ |
| **Missile Speed** | $2000 units/second$ |
| **Acquisition Radius** | $600 units$ |
| **Pathing Radius** | $35 units$ |
| **Selection Radius** | $120 units$ |
| **Selection Height** | $302.778 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Blaze

**Innate:** **Brand**’s abilities apply a stack of *Ablaze* to enemies hit. Subsequent abilities used against a target with *Ablaze* are empowered. Killing an enemy afflicted with *Ablaze* or with an ability restores .

*At three stacks, champion and large monsters become unstable and explode after a brief delay, dealing magic damage to nearby enemies and applying a stack of *Ablaze*. *Ablaze* cannot stack more than once on enemies who have recently exploded.*

**Innate:** ''Brand's* abilities apply a stack of *Ablaze* to enemies hit for 4 seconds, stacking up to 3 times and refreshing the duration of all stacks with each application. Subsequent abilities used against a target with *Ablaze* are empowered. Killing an enemy afflicted with *Ablaze' or any enemy with an ability restores 20–40@1–15 . **Ablaze:** While afflicted with *Ablaze, the target is dealt magic damage equal to Upon applying 3 stacks of *Ablaze* to a large monster or enemy champion, the fire becomes unstable, causing it form a fiery around the target that grants sight within its radius and, after 2 seconds, consumes their stacks to explode. All enemies within the detonation are applied a stack of *Ablaze* and dealt magic damage equal to key=% , capped at 270 to 525 against monsters. **Brand** cannot stack *Ablaze* more than once on enemies that have had a explode from them in the last 4 seconds. *The will form even if the target dies to the ability applying the third stack, and will still trigger the explosion if the target dies before it.*

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | 475 units |
| **Spell Shield** | True |
| **Spell Effects** | Special |

**Notes:**

- *Ablaze* is a debuff that persists through death.
- 'Ablaze's damage cap against a monster is applied after the increased monster damage.
- The explosion is not considered a tether effect and will not affect untargetable targets, even if the explosion originated from them.
- Given the behavior of the stacking mechanic, the damage over time will deal varying amounts of damage based on the time the stacks were applied:
  - At sub-optimal applied times, 2 stacks will deal a minimum of 6% **maximum** health damage over 4 seconds, but will deal 7% **maximum** health at optimal applied times.
  - Against small-medium units 3 stacks will deal a minimum of 9% **maximum** health at sub-optimal times, but 11% **maximum** health at optimal times. If these stacks are further refreshed at optimal times they can deal up to 12% **maximum** health over 4 seconds.
- The burn is persistent damage and the explosion is area damage.

---

### Q: Sear

**Active:** **Brand** launches a fireball in the target direction that deals magic damage to the first enemy hit.

**Ablaze Bonus:** An *Ablaze* target hit will also be briefly stun.

**Active:** **Brand** launches a fireball in the target direction that deals magic damage to the first enemy hit. **Ablaze Bonus:** The target is stun for $1.75$ seconds.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $8-6$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | 70 mana |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Speed** | 1600 units/second |
| **Spell Shield** | True |
| **Spell Effects** | spell |
| **Projectile** | True |

**Scaling:**
- **Magic Damage:** $70-190$ (+ 65% AP)

**Notes:**

- With an ability haste value of $((8-6)/(4-0.25+1100/1600)-1)100100 round=ceil$ based on the ranges of applications from the target, it is possible to apply the stun without having to use other abilities. *Sear* will apply before the duration of the pre-applied *Blaze* times out.
  - The needed ability haste value if you remained at the same range from the target for both casts would be $((8-6)/(4-0.25)-1)*100 round=ceil$. Effect at cast time end

---

### W: Pillar of Flame

**Active:** After a brief delay, **Brand** erupts a pillar of flame at the target location that deals magic damage to enemies hit.

**Ablaze Bonus:** An *Ablaze* target hit will take bonus damage.

**Active:* After a rutngt*Brand** erupts a pillar of flame at the target location that deals magic damage to enemies hit. **Ablaze Bonus:** The target takes 25% increased damage.

| Attribute | Value |
|-----------|-------|
| **Range** | 900 units |
| **Cooldown** | $10/9.5/9/8.5/8$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $60/70/80/90/100$ mana |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | 260 units |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |

**Scaling:**
- **Magic Damage:** $75/120/165/210/255$ (+ 60% AP)
- **Increased Damage:** $93.75/150/206.25/262.5/318.75$ (+ 75% AP)

**Notes:**

- The delay before the eruption does not include the cast time. The delay would be a total of if it included the cast time.

---

### E: Conflagration

**Active:** **Brand** sets the target enemy aflame, which creates a blast that deals magic damage to them and nearby enemies.

**Ablaze Bonus:** The blast spreads twice as far on an *Ablaze* target.

**Active:** **Brand** sets the target enemy aflame, which creates a blast that deals magic damage to them and causes the flame to spread from the target to nearby enemies, dealing them the same damage. **Ablaze Bonus:** 'Conflagration's spread range is doubled.

| Attribute | Value |
|-----------|-------|
| **Range** | 675 units |
| **Cooldown** | $13-9$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $70-90$ mana |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Speed** | 900 units/second |
| **Effect Radius** | 300 / 600 units |
| **Spell Shield** | True |
| **Spell Effects** | aoe |
| **Projectile** | False |

**Scaling:**
- **Magic Damage:** $55-155$ (+ 60% AP)

**Notes:**

- The flame spreads to enemies near the primary target at the time of cast.
  - Enemies that move out of range after the fact will still be hit, as the flame is spread as homing missiles to each of its targets.

---

### R: Pyroclasm

**Active:** **Brand** unleashes a devastating torrent of fire that bounces to nearby enemies, dealing magic damage each time it bounces.

**Ablaze Bonus:** An *Ablaze* target will also be briefly slow when hit.

**Active:** **Brand** launches a fireball at the target enemy that bounces between nearby enemies and **Brand** up to four times, dealing magic damage to enemies each time and having a $0.15$-second delay between bounces. *Pyroclasm* prioritizes **Ablaze** enemy champions, then other enemy champions, then any valid bounce target, then **Brand**. **Ablaze Bonus:** The target is slow for $0.25$ seconds. This can affect the same enemy more than once. *The target does not have to be sight for the fireball to bounce to them.*

| Attribute | Value |
|-----------|-------|
| **Range** | 750 units |
| **Cooldown** | $100-80$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | 100 mana |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Speed** | 750 - 3000 units/second |
| **Effect Radius** | 600 units |
| **Spell Shield** | Special |
| **Spell Effects** | spellaoe |
| **Projectile** | True |

**Scaling:**
- **Magic Damage:* $100-250$ (+ 25% AP)3-250×3$ (+ 75% AP)
- **Slow:** $30-60$%

**Notes:**

- The fireball acquires a new valid, bounce target once the bounce delay finishes.
- If the target the fireball is in flight towards becomes untargetable, the fireball missile will be destroyed but a new missile will be formed at the target's location that is able to bounce for up to the remaining bounces of the previous fireball.
  - The new missile is only formed if there is a valid target to bounce to at the time of the previous missile being destroyed.
  - This does not apply if the target is **Brand**, as the missile would not be destroyed and would bounce from him even if he is untargetable.
- The fireball will still bounce from the target even if they were dead upon arrival.
- The fireball may only bounce to valid targets, including **Brand**.
- If the initial cast target uses spell shield, *Pyroclasm* will not bounce. If the bounce is blocked by an enemy's *spell shield* it will not deal any damage, but it will keep on bouncing.
- 'Pyroclasm's damage against enemy champions will not aggro nearby enemy minions.

---

## Patch History

### V25.16
- *Blaze*
  - Ablaze monster damage increased to 260% from 240%.
- *Sear*
  - Stun duration increased to $1.75$ seconds from $1.5$.
- *Pyroclasm*
  - Cooldown reduced to $100-80 3$ seconds from $110-90 3$.

### V25.12
- *Pyroclasm*
  - **Bug Fixes:** Missile now acquires a new bounce target after its bounce delay, rather than before it.
    - *[Note: The original target acquisition behavior lead to an issue where the missile would fizzle after the delay if its intended bounce target became invalid during it. The missile will now acquire a target and bounce to them at the same time, once the bounce delay finishes.*]

### V25.08
- *Blaze*
  - Ablaze monster damage increased to 240% from 215%.

### V25.07
- *Blaze*
  - Ablaze non-epic monster damage cap per second increased to 40 from 30.
  - Explosion monster damage cap increased to 270 to 525 from 250 to 475.

### V14.21
- Stats
  - Base mana regeneration reduced to 9 from $10.65$.
- *Blaze*
  - Explosion base damage reduced to 8–12 of target's **maximum** health from 9–13.
- *Sear*
  - Mana cost increased to 70 from 50.

### V14.17
- *Blaze*
  - **New Effect:** Ablaze damage per second against non-epic monsters is now capped at 30.
    - *Ablaze damage cap against epic monsters unchanged.*
  - **New Effect:** Explosion now deals a maximum of 250 to 475 damage to monsters.
  - **Removed:*** Explosion no longer deals 50% damage to epic monsters.

### V14.14
- *Conflagration*
  - Base damage reduced to $55-155$ from $60-160$.
  - Cooldown increased to $13-9$ seconds from $12-8$.

### V14.13
- *Blaze*
  - Mana restore reduced to 20–40@1–15 from 30–50@1–18.
- *Conflagration*
  - Base damage reduced to $60-160$ from $60-180$.

### V14.9
- Stats
  - Selection radius increased to 120 units from $88.8889007568359$.
- Brand
  - *Blaze*
    - **Bug Fixes:** VFX no longer renders over terrain.
- Brand, Brand, Brand, Brand
  - *Blaze*
    - **Bug Fixes:** Targets' outline when hovered over using the cursor is no longer pixelated.

### V14.8
- *Blaze*
  - **Bug Fixes:** Can now once again trigger *Dark Harvest*.

## Trivia

- Brand is the second fire-themed champion, the first being **Annie**.
  - *Sear*’s animation resembles **Annie**’s basic attack one.
    - Brand's autoattack particle is the same as **Morgana**’s but orange instead of purple.
- *Brand* comes from Proto-Germanic **brandaz*, meaning "fire, torch, sword".
  - His host's given name *Mac Aodhagáin* is likely based on Gaelic patronymic *, "son of Aed (god)" by coincidence;
    - His surname *Rodhe* possibly from Old Norse * "praise, renown".
  - *Pyroclasm* derives from *pyroclastic*, a term used to describe fast-moving volcanic material, from Greek πῦρ "fire" & κλαστός "broken
  - Brand musical reference comes from the hip-hop group called **Brand Nubian**

---
*This page was automatically generated from League of Legends Wiki data.*