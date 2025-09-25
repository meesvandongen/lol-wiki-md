# Talon

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
| **Champion** | Talon |
| **Title** | the Blade's Shadow |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2011-08-24 |
| **Release Patch** | V1.0.0.124 |
| **Latest Changes** | V25.11 |
| **Roles** | Assassin |
| **Riot Positions** | Middle |
| **External Positions** | Jungle |
| **Blue Essence** | 1575 |
| **Riot Points** | 790 |
| **Difficulty** | 2 |
| **Hero Type** | Assassin |
| **Adaptive Type** | Physical |
| **Damage** | 3 |
| **Toughness** | 1 |
| **Control** | 1 |
| **Mobility** | 2 |
| **Utility** | 1 |
| **Style** | 65 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $658.0$ | $+109.0$ |
| **Mana** | $400.0$ | $+37.0$ |
| **Health Regen** | $8.5$ | $+0.75$ |
| **Mana Regen** | $7.6$ | $+0.8$ |
| **Armor** | $30.0$ | $+4.7$ |
| **Magic Resist** | $36.0$ | $+2.05$ |
| **Attack Damage** | $68.0$ | $+3.1$ |
| **Attack Speed** | $0.625$ | |
| **Movement Speed** | $335.0$ | $+0.0$ |
| **Attack Range** | $125.0$ | $+0.0$ |
| **Base Attack Speed** | $0.625$ | |
| **Attack Speed Ratio** | $0.625$ | |
| **Bonus AS per Level** | $2.9\%$ | |
| **Attack Windup** | $12.4\%$ | |
| **Acquisition Radius** | $400$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $120$ units | |
| **Selection Height** | $155$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $105.0\%$ |
| **Damage Taken** | $95.0\%$ |

## Abilities

### Passive: Blade's End

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | Special |
| **Spell Effects** | default |
| **Parry** | Special |

**INNATE:** **Talon**’s abilities apply a stack of *Wound* to enemy champions and large monsters hit for 6 seconds, refreshing on basic attacks and subsequent applications and stacking up to 3 times.

**Talon**’s next basic attack against an enemy with 3 *Wound* stacks is empowered to consume them all to cause the target to bleed, dealing「 80 to 280 (+ 210% **bonus** AD) **total** physical damage over 2 seconds, ⟷ 80/16 to 280/16 (+ 13.125% **bonus** AD) physical damage every $0.125$ seconds over 2 seconds, 」increased to 110% damage against monsters. The target cannot gain *Wound* stacks during this time.

**Notes:**

- *Blade End*’s enhanced attack will still trigger if blocked, but not if they are dodged or missed while **Talon** is blinded.
  - Duration of the stacks is not refreshed if the attack is dodged or missed.
- Both passes of Rake and Shadow Assault apply a stack of *Wound*.
- Spell shield prevents *Wound* stacks from being applied.
  - Spell shield prevents *Blade End*’s enhanced attack from triggering though still consuming all *Wound* stacks.

---

### Q: Noxian Diplomacy

| Attribute | Value |
|-----------|------:|
| **Cast Time** | None (Ranged dash attack) / at **base** attack speed)|Close dash attack |
| **Target Range** | 575 (Ranged dash attack range) / er 170 (Close dash attack range) units |
| **Speed** | 1400 (Dash speed, estimated) units/second |
| **Cost** | 40 Mana |
| **Cooldown** | 8 / 7.5 / 7 / 6.5 / 6 seconds |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | spell |
| **Call For Help** | True |
| **Grounded** | Special |
| **Knockdown** | True |

**ACTIVE:** **Talon** dashes toward the target enemy, stabbing the target upon arrival to deal physical damage. If cast within close range, **Talon** over the cast time instead and *Noxian Diplomacy* critically strikes for damage, as well as grants him 75 **bonus** attack range on his next basic attack within 1 second.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 65 / 85 / 105 / 125 / 145 (+ 100% **bonus** AD) |
| **Critical Physical Damage** | 97.5 / 127.5 / 157.5 / 187.5 / 217.5 (+ 150% **bonus** AD) |

If *Noxian Diplomacy* kills the target, **Talon** heals for 9 to 55 and the ability's cooldown is reduced by 50%.

*Noxian Diplomacy resets **Talon**’s basic attack timer. Shadow Assault can be cast during the dash.*

**Notes:**

- **Talon** will track the target if they change locations with the ranged version of *Noxian Diplomacy*, landing at an offset of 100 units before reaching them.
  - He will dash to the target's previous location while still dealing damage if the target is too far away or moves beyond 800 (Estimated) units.
- When cast in close range, *Noxian Diplomacy* deals its damage at the completion of the cast time, rather than at the dash's completion.
  - The damage will be dealt even if **Talon**’s dash is interrupted, so long as the cast time is completed.
- **Talon** will be ordered to basic attack the target at the end of the dash.
- While grounded or rooted, *Noxian Diplomacy* can only be cast within close range.
  - **Talon** will still dash to the target in both cases.
- *Noxian Diplomacy* will also grant the heal if the target dies upon **Talon**’s completion of the dash.** This only applies to the close dash attack.

---

### W: Rake

| Attribute | Value |
|-----------|------:|
| **Range** | 900 (Center missile) / 850 (Side missiles) units |
| **Cast Time** | $0.25$ seconds |
| **Angle** | 22° |
| **Width** | 150 (Individual missile width) units |
| **Speed** | 2571 (Center missile outward; 0.35 seconds fixed travel time) / 2429 (Side missiles outward; 0.35 seconds fixed travel time) / 3000 (Returning missiles) units/second |
| **Cost** | 50 / 55 / 60 / 65 / 70 Mana |
| **Cooldown** | 9 / 8.5 / 8 / 7.5 / 7 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | True |

**ACTIVE:** **Talon** throws a fan of daggers in a cone in the target direction that deals physical damage to enemies hit.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 50 / 60 / 70 / 80 / 90 (+ 40% **bonus** AD) |

At maximum range, the daggers linger for $0.7$ seconds before homing back to **Talon**, dealing physical damage to enemies hit and slowing them for 1 second.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 60 / 90 / 120 / 150 / 180 (+ 90% **bonus** AD) |
| **Total Physical Damage** | 110 / 150 / 190 / 230 / 270 (+ 130% **bonus** AD) |

| Attribute | Value |
|-----------|------:|
| **Slow** | 40 / 45 / 50 / 55 / 60% |

*Enemies can be hit only once per pass.*

**Notes:**

- This ability will cast from wherever the caster is at the end of the cast time.
- Casting *Rake* when an enemy is within 650 radius and 26° angle in front of *Talon* will reveal him from the fog of war like a targeted cast.
  - This is because of *Rake*’s internal Cone targeting type, which defaults to "targeting" all units in the preset area and thereby trigger revealing the caster if not toggled off for the spell.
- If **Talon** dies while *Rake* is mid-air, the projectiles will still complete their travel.
- The blades each will fail to return to **Talon** if he is somehow too far from them after the delay.
  - This maximum distance is 5000 units at ranks 1-4 of *Rake*, or 20000 units at rank 5 of *Rake*.
  - The latter distance is barely achievable between fountains on Summoner's Rift.

---

### E: Assassin's Path

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 725 (Cast range) units |
| **Speed** | 100% movement speed |
| **Cooldown** | 2 seconds |
| **Targeting** | Location |
| **Affects** | Self |
| **Out of Range Behavior** | See notes |
| **Grounded** | True |
| **Knockdown** | True |
| **On-Terrain Cooldown (Affected by ability haste)** | 160 / 135 / 110 / 85 / 60 |

**ACTIVE:** **Talon** dashes up to 800 units over the target area of terrain while within proximity of it, during which he gains unobstructed vision.

**Talon** cannot cast *Assassin's Path* on the same area of terrain for a set duration.

*Shadow Assault can be cast during the dash.*

**Notes:**

- When *Assassin's Path* is cast on terrain that is out of range, **Talon** will walk to the location where he is able to cast the ability and dash over this area of terrain. If he encounters other terrain along the way, however, he will cast *Assassin's Path* over the obstructive area of terrain instead.
  - This override will fail if his path to the location would be obstructed by terrain that is on cooldown. In this case, the ability will not cast at all.
- *Assassin's Path* can interact with player-generated terrain.
- **Talon** will stop dashing upon entering stasis but will continue to do so afterwards.
- **Talon** will reveal himself for $0.6$ seconds upon casting *Assassin's Path* if he is visible to an enemy champion or there is a nearby enemy champion or minion within 400 units of the landing position.
- Walls eligible for *Assassin's Path* display a border indicator that fades over the cooldown. The closer the wall is to becoming traversable, the lighter the indicator becomes.

---

### R: Shadow Assault

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 550 (Cast range of outward missiles) units |
| **Width** | 280 (Width of the individual missiles, both outward and returning) units |
| **Speed** | 2400 (Outward missiles speed) / 4000 (Returning missiles speed) units/second |
| **Cost** | 100 Mana |
| **Cooldown** | 100 / 90 / 80 / 70 / 60 seconds |
| **Cooldown Start** | post-effect |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | True |

**ACTIVE:** **Talon** disperses a of blades around him that deals physical damage to enemies hit and lingers at maximum range for up to $2.5$ seconds, during which he gains invisibility and (ms) **bonus** movement speed.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 90 / 112.5 / 135 / 157.5 / 180 (+ 100% **bonus** AD) |

| Attribute | Value |
|-----------|------:|
| **Bonus Movement Speed** | 40 / 47.5 / 55 / 62.5 / 70% |

*Shadow Assault* can be recast after 1 second within the duration, and does so automatically after the duration or if **Talon** breaks stealth.

**RECAST:** **Talon** breaks stealth and converges the blades to him, dealing the same physical damage to enemies hit. Breaking stealth with a basic attack or *Noxian Diplomacy* will converge the blades to the target instead.

| Attribute | Value |
|-----------|------:|
| **Total Physical Damage** | 180 / 225 / 270 / 315 / 360 (+ 200% **bonus** AD) |

**Notes:**

- *Shadow Assault* casts a total of 12 blades evenly distributed in all directions. 4 of them travel down the cardinal directions (Relative to the map, **not** relative to **Talon**), while all blades have an angle of 30° to one another.
- Spell shield will block a single pass.
- *Shadow Assault* does not cancel **Talon**’s basic attacks.
- *Shadow Assault* has a minimum lifetime of $0.8$ seconds before convergence.
- The blades each will fail to converge on their target (**Talon** or the attacked target) if the target is more than 20000 units from them.
  - This distance is barely achievable between fountains on Summoner's Rift.
- Using a basic attack breaks the stealth at the end of the attack windup.

---

## Patch History

### V25.11
- Assassin's Path
  - **UNDOCUMENTED / BUG FIX:** Tooltip now shows the actual cooldown of the active instead of "No Cooldown".

### V25.08
- Talon
  - Adjustments made to his fire and VFX elements throughout his kit.

### V14.24
- Stats
  - Base magic resistance reduced to 36 from 39.
- Blade's End
  - Damage against monsters reduced to 110% from 120%.

### V14.21
- Rake
  - Return base damage increased to 60 / 90 / 120 / 150 / 180 from 50 / 80 / 110 / 140 / 170.
  - Return bonus AD ratio increased to 90% **bonus** AD from 80%.

### V14.17
- Noxian Diplomacy
  - **Bug Fixes:** Ranged dash now once again places the ability on cooldown immediately after it is cast.

### V14.15
- Noxian Diplomacy
  - **New Effect:** Now always dashes to the target within 150 units regardless of cast distance.
  - **Undocumented:** Ranged dash now places the ability on cooldown after the dash is completed, rather than immediately after casting.

### V14.12
- Noxian Diplomacy
  - Stab cast time changed to attack windup from $0.25$ seconds.
- Rake
  - Outgoing base damage increased to 50 / 60 / 70 / 80 / 90 from 40 / 50 / 60 / 70 / 80.
  - Monster damage reduced to 100% from 105%.

### V14.2
- Talon
  - Rake
    - **Bug Fixes:** VFX no longer extends past the spell's hitbox.

### V13.24
- Noxian Diplomacy
  - **New Effect:** Dash cast now resets his basic attack timer as well.

### V13.6
- Stats
  - Base mana increased to 400 from 377.
- Blade's End
  - Base damage increased to 80 to 280 from 75 to 255.
  - Bonus AD ratio increased to 210% **bonus** AD from 200%.
- Rake
  - Mana cost reduced to 50 / 55 / 60 / 65 / 70 from 55 / 60 / 65 / 70 / 75.

## Trivia

- Mercy.png references euthanasia.
- Noxian Diplomacy’s bleed icon is shaped into Talon's arm blade.
- Cutthroat.png makes use of a double entendre:
  - The word 'cutthroat' is defined as murderous or cruel, which describes Talon well.
  - The ability has Talon literally cut the throat of his target. This is made more clear in the first iteration of the ability, which also silenced the target.
- This champion has no ability power ratio.
- His official name *Talon* originates from Latin *talus* "ankle"; which possibly later underwent semantic shift to "heel > Spur (zoology) > claw".
  - His title might be referencing his loyalty to General Du Couteau (the General being the 'Blade' - *couteau* is French for 'knife' - and Talon being the 'Shadow').
- **Talon**’s dance references Billie Jean by Michael Jackson.
  - A side-by-side comparison can be seen here.
- Some scrapped spells tested on **Talon** include:
  - A spell where Talon throws out knives to create a slow field, but if he left its proximity it would break the field.
  - A spell where Talon could throw out a knife that could be reactivated and allow him to blink to its location.
  - Shadow Assault was originally tested as a basic spell.
- Talon was named after his bird like appearance and his large hand blade. His spells carry a metaphor by utilizing the concept of his cloak blades as a string of “feathers”.

---
*This page was automatically generated from League of Legends Wiki data.*