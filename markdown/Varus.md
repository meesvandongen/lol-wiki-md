# Varus

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
| **Champion** | Varus |
| **Title** | the Arrow of Retribution |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2012-05-08 |
| **Release Patch** | V1.0.0.139 |
| **Latest Changes** | V25.16 |
| **Roles** | Marksman, Artillery |
| **Riot Positions** | Bottom |
| **External Positions** | Bottom |
| **Blue Essence** | 2400 |
| **Riot Points** | 880 |
| **Difficulty** | 2 |
| **Hero Type** | Marksman |
| **Alt Type** | Mage |
| **Adaptive Type** | Physical |
| **Damage** | 3 |
| **Toughness** | 1 |
| **Control** | 2 |
| **Mobility** | 1 |
| **Utility** | 1 |
| **Style** | 20 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $600.0$ | $+105.0$ |
| **Mana** | $320.0$ | $+40.0$ |
| **Health Regen** | $3.5$ | $+0.55$ |
| **Mana Regen** | $8.0$ | $+0.8$ |
| **Armor** | $24.0$ | $+4.6$ |
| **Magic Resist** | $30.0$ | $+1.3$ |
| **Attack Damage** | $59.0$ | $+3.4$ |
| **Attack Speed** | $0.658$ | |
| **Movement Speed** | $330.0$ | $+0.0$ |
| **Attack Range** | $575.0$ | $+0.0$ |
| **Base Attack Speed** | $0.658$ | |
| **Attack Speed Ratio** | $0.658$ | |
| **Bonus AS per Level** | $3.5\%$ | |
| **Missile Speed** | $2000$ units/second | |
| **Acquisition Radius** | $600$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $120$ units | |
| **Selection Height** | $155$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $100.0\%$ |
| **Damage Taken** | $100.0\%$ |

## Abilities

### Passive: Living Vengeance

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Self |

**INNATE:** When **Varus** kills an enemy, he gains (attack speed) 10 to 20 **bonus** attack speed as well as **bonus** attack damage and ability power equal to 10% **bonus** attack speed for 5–11@1–16 seconds.

This is increased to 50% **bonus** attack speed and **bonus** attack damage and ability power equal to 25% **bonus** attack speed upon scoring a champion takedown.

While *Living Vengeance*’s bonus is active, **Varus' **attack speed cap is increased to $3.33$.

**Notes:**

- *Living Vengeance*’s buff can be refreshed and is triggered when **Varus** kills any unit.
  - The increased bonus gained from taking down an enemy champion takes priority over the lesser bonus. That bonus can only be refreshed by scoring another takedown.

---

### Q: Piercing Arrow

| Attribute | Value |
|-----------|------:|
| **Range** | 895 to 159570+(116.66 per 0.25 seconds). *This is capped at 1.5 seconds.* units |
| **Cast Time** | none |
| **Width** | 140 units |
| **Speed** | 1900 (Missile speed) units/second |
| **Cost** | 50 / 55 / 60 / 65 / 70 mana |
| **Cooldown** | 16 / 15 / 14 / 13 / 12 (Starts post-effect, after charge ends the cooldown is reduced equal to the charge duration) seconds |
| **Cooldown Start** | post-effect |
| **Targeting** | Auto / Direction |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |
| **Projectile** | True |
| **Silence** | True |

**ACTIVE:** **Varus** charges while being slowed by 20% for up to 4 seconds to increase *Piercing Arrow*’s range over the first $1.5$ seconds and its effects over the first $1.25$ seconds of the channel.

*Piercing Arrow* can be recast within the duration. If the charge completes without reactivation, *Piercing Arrow* is cancelled and refunds (mana) 50% of the mana cost.

**RECAST:** **Varus** fires a piercing arrow in the target direction that deals physical damage to enemies hit. The damage of the arrow as well as any detonated *Blight* stacks are both increased by 0% / 10% / 20% / 30% / 40% / 50%, and the arrow's damage is reduced by 0 to 60 for 5%–67%@0–5 (@=number of enemies hit).

| Attribute | Value |
|-----------|------:|
| **Minimum Physical Damage** | 53.33 / 100 / 146.67 / 193.33 / 240 (+ 86.67 / 93.33 / 100 / 106.67 / 113.33% **bonus** AD) |
| **Maximum Physical Damage** | 80 / 150 / 220 / 290 / 360 (+ 130 / 140 / 150 / 160 / 170% **bonus** AD) |

| Attribute | Value |
|-----------|------:|
| **Minimum Reduced Damage** | 17.6 / 33 / 48.4 / 63.8 / 79.2 (+ 28.6 / 30.8 / 33 / 35.2 / 37.4% **bonus** AD) |
| **Maximum Reduced Damage** | 26.4 / 49.5 / 72.6 / 95.7 / 118.8 (+ 42.9 / 46.2 / 49.5 / 52.8 / 56.1% **bonus** AD) |

**Notes:**

- The arrow missile range is 825 to 1525; The range increases by 140 per $0.25$ seconds for the first $1.25$ seconds. Upon reaching its maximum range, it also strikes additional targets in a er 70 radius, rounding off the struck area.
- The indicator for the range of the spell will be displayed for the entire channel.
- *Piercing Arrow* will cast from wherever **Varus** is at the end of the channel.
- At maximum damage charge, all damage caused by *Piercing Arrow* will trigger cosmetic critical strike text.
- The following table refers for interactions while **Varus** is channeling:

#### Channel Behavior (charge)

| Aspect | State / Notes |
|--------|---------------|
| **Attacking** | Disabled |
| **Movement** | Allowed |
| **Abilities** | Blighted Quiver is usable during the first $0.85$ seconds, otherwise, it's disabled. Hail of Arrows and Chain of Corruption are disabled. This ability recasts to end channel. |
| **Items** | Disabled |
| **Summoner Spells** | Allowed / Disabled / Recasts |
| **Consumables** | Disabled |
| **Notes** | but can still use trinkets. |

---

### W: Blighted Quiver

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Cooldown** | 40 (Starts after the effect is used with Piercing Arrow) seconds |
| **Cooldown Start** | post-effect |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | Special |
| **Spell Effects** | Proc |
| **Parry** | True |

**PASSIVE:** **Varus**' basic attacks are empowered to deal **bonus** magic damage and apply a stack of *Blight* on-hit for 6 seconds, refreshing on subsequent applications and stacking up to 3 times. **Varus' ** abilities detonate all *Blight* stacks on enemies hit.

| Attribute | Value |
|-----------|------:|
| **Bonus Magic Damage** | 6 / 14 / 22 / 30 / 38 (+ 35% AP) |

**BLIGHT:** For each stack consumed, the target is dealt **bonus** magic damage. If the stacks were consumed with *Piercing Arrow*, this damage is increased by 0% / 5% / 10% / 15% / 20% / 25% / 30% / 35% / 40% / 45% / 50%, for a total cap of 360 at maximum stacks.

Additionally, each stack consumed against a champion or epic monster reduces the of **Varus' ** basic abilities by 13% of each of their **total** cooldowns, up to a 39% cooldown reduction per target. If the stacks were consumed with **, the cooldown reduction is increased by 0% / 5% / 10% / 15% / 20% / 25% / 30% / 35% / 40% / 45% / 50% cooldown reduction per target from consuming maximum stacks against them.

| Attribute | Value |
|-----------|------:|
| **Bonus Magic Damage per Stack** | 3 / 3.5 / 4 / 4.5 / 5% (+ $1.5$% per 100 AP) of the target's **maximum** health |
| **Bonus Magic Damage at Max Stacks** | 9 / 10.5 / 12 / 13.5 / 15% (+ 4.5% per 100 AP) of the target's **maximum** health |

| Attribute | Value |
|-----------|------:|
| ** Maximum Bonus Magic Damage per Stack |
| **4.5 / 5.25 / 6 / 6.75 / 7.5% (+ 2.25% per 100 AP) of the target's **maximum** health** | Maximum Bonus Magic Damage at Max Stacks** | 13.5 / 15.75 / 18 / 20.25 / 22.5% (+ 6.75% per 100 AP) of the target's **maximum** health |

**ACTIVE:** **Varus**' next *Piercing Arrow* within $5.5$ seconds is empowered to deal **additional bonus** magic damage, increased by 0% / 5% / 10% / 15% / 20% / 25% / 30% / 35% / 40% / 45% / 50%.

| Attribute | Value |
|-----------|------:|
| **Active Minimum Magic Damage** | 6 / 8 / 10 / 12 / 14% of target's **missing** health |
| **Active Maximum Magic Damage** | 9 / 12 / 15 / 18 / 21% of target's **missing** health |

If **Varus** does not cast *Piercing Arrow*, *Blighted Quiver* can be recast after 1 second within the duration, and does so automatically afterwards or when he dies. *Blighted Quiver* can be cast during the first $0.85$ seconds of *Piercing Arrow’s* charge, and will be placed on full cooldown after the charge ends even if *Piercing Arrow* is not recast.

**RECAST:** **Varus** ends *Blighted Quiver* and places it on a 1-second cooldown.

*Blighted Quiver's active and recast can both be used while affected by cast-inhibiting crowd control.*

**Scaling:**
| Attribute | Value |
|-----------|------:|
| **[[File:Varus Piercing Arrow.png** | 20px |
| **border** | link=]] Maximum Bonus Magic Damage per Stack |
| **4.5 / 5.25 / 6 / 6.75 / 7.5% (+ 2.25% per 100 AP) of the target's **maximum** health** | [[File:Varus Piercing Arrow.png |
| **20px** | border |
| **link=]] Maximum Bonus Magic Damage at Max Stacks** | 13.5 / 15.75 / 18 / 20.25 / 22.5% (+ 6.75% per 100 AP) of the target's **maximum** health |

**Notes:**

- The monster damage cap is also increased by 0% / 5% / 10% / 15% / 20% / 25% / 30% / 35% / 40% / 45% / 50%*.
- Spell shield will not block the on-hit damage but will block the consumption damage if the target is hit by his abilities.
- *Blighted Quiver* will not apply stacks nor deal on-hit damage if blocked by blind, block or dodge effects.
  - The consumption damage will not be blocked.
- *Blighted Quiver*’s active damage is dealt after the damage of Piercing Arrow.
- *Blighted Quiver* does not count as an ability activation for the purposes of on-cast effects such as Spellblade and triggering Force Pulse’s passive.
- The attacks do not affect structures nor wards.

---

### E: Hail of Arrows

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.2419$ seconds |
| **Target Range** | 925 units |
| **Effect Radius** | 300 units |
| **Cost** | 90 mana |
| **Cooldown** | 18 / 16 / 14 / 12 / 10 seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | True |
| **Out of Range Behavior** | cast at max |

**ACTIVE:** **Varus** fires a hail of arrows at the target location that land after $0.5$ seconds, dealing physical damage to enemies hit.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 60 / 90 / 120 / 150 / 180 (+ 90% **bonus** AD) |

The area then becomes desecrated for 4 seconds, slowing enemies within and inflicting them with Grievous Wounds.

| Attribute | Value |
|-----------|------:|
| **Slow** | 30 / 35 / 40 / 45 / 50% |

**Notes:**

- Slow lingers for $0.25$ seconds after leaving the marked area.
- *Hail of Arrows* consumes Blighted Quiver stacks for bonus damage a second time after ~$0.3$ seconds of hitting, allowing stacks applied during this time to be triggered.

---

### R: Chain of Corruption

| Attribute | Value |
|-----------|------:|
| **Range** | 1370 (Missile plus area check total range) units |
| **Cast Time** | $0.2419$ seconds |
| **Tether Radius** | 650 (Tether seeking and forming radius) / er 600 (Tether escape radius after being registered) units |
| **Width** | 240 (Missile width) units |
| **Speed** | 1500 (Missile speed) units/second |
| **Cost** | 100 mana |
| **Cooldown** | 100 / 90 / 80 / 70 / 60 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | True |

**ACTIVE:** **Varus** unleashes a tendril of corruption in the target direction that infects the first enemy champion hit, dealing magic damage and rooting them for 2 seconds, during which they are revealed. Over the first $1.5$ seconds of the root, they are also inflicted with maximum stacks of *Blight*.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 150 / 200 / 250 / 300 / 350 (+ 100% AP) |

Upon impact, the tendril roots into the ground from which it seeks out nearby enemy champions. If a target does not leave the area within 2 seconds, they subsequently become infected as well, taking the same damage and effects. A new tendril is then spread out from the infected target that is closest to a nearby uninfected enemy champion. The infection can spread repeatedly until there are no new targets.

*The target does not have to be visible to be caught by the tendril.*

**Notes:**

- The tendril seeks and registers valid targets every .
- Enemy champions can be targeted again by the corruption's tether if they were not infected the previous time, even if they were targeted and exited its range.
- A tendril that roots into the ground will only seek nearby enemy champions that were already in the area.
- *Chain of Corruption* will not infect crowd control immune targets or those protected by spell shields (but will still seek them).
- **Varus** will turn to face the target direction at the start of the cast.
- The tendril cannot seek enemy champions that are untargetable, and will also stop seeking a target if it becomes untargetable (fails to apply its effects if they remain in the area). - This ability will cast from wherever the caster is at the end of the cast time.
- *Chain of Corruption* can generate *Blight* stacks for **Varus** to detonate with any ability, even if *Blighted Quiver* has not been learned. The **base** detonation damage per *Blight* stack scales with *Blighted Quiver*’s rank, which has a value at rank 0 of $2.5$% of the target's **maximum** health per stack detonated.

---

## Patch History

### V25.16
- Piercing Arrow
  - Minimum base damage reduced to 53.33 / 100 / 146.67 / 193.33 / 240 from 60 / 106.67 / 153.33 / 200 / 246.67.
  - Maximum base damage reduced to 80 / 150 / 220 / 290 / 360 from 90 / 160 / 230 / 300 / 370.
- Blighted Quiver
  - On-hit base damage increased to 6 / 14 / 22 / 30 / 38 from 6 / 12 / 18 / 24 / 30.
- Hail of Arrows
  - Base damage reduced to 60 / 90 / 120 / 150 / 180 from 60 / 100 / 140 / 180 / 220.
  - Bonus AD ratio reduced to 90% **bonus** AD from 100%.
- Chain of Corruption
  - **Bug Fixes:** Now correctly consumes *Blight* stacks on targets it spreads to.

### V25.12
- Stats
  - Base armor reduced to 24 from 27.

### V25.07
- Hail of Arrows
  - **Bug Fixes:** Resolved some inconsistencies within the tooltip.

### V25.S1.3
- Living Vengeance
  - Attack speed cap increased to $3.33$ from $3.0$.
- Blighted Quiver
  - **Bug Fixes:** In the floating text display, now uses the proper

### File:Critical strike magic icon.png|20px|link=
  - **Bug Fixes:** In the floating text display, now uses the proper

### V25.S1.1
- Stats
  - Base mana reduced to 320 from 360.
- Piercing Arrow
  - Mana cost reduced to 50 / 55 / 60 / 65 / 70 from 65 / 70 / 75 / 80 / 85.
- Blighted Quiver
  - On-hit base damage changed to 6 / 12 / 18 / 24 / 30 from 8 / 13 / 18 / 23 / 28.
- Hail of Arrows
  - Mana cost increased to 90 from 80.

### V14.22
- Blighted Quiver
  - On-hit base damage increased to 8 / 13 / 18 / 23 / 28 from 5 / 10 / 15 / 20 / 25.

### V14.18
- Piercing Arrow
  - Minimum bonus AD ratio reduced to 86.67 / 93.33 / 100 / 106.67 / 113.33% **bonus** AD from 100 / 106.67 / 113.33 / 120 / 126.67%.
  - Maximum bonus AD ratio reduced to 130 / 140 / 150 / 160 / 170% **bonus** AD from 150 / 160 / 170 / 180 / 190%.
- Hail of Arrows
  - Bonus AD ratio reduced to 100% **bonus** AD from 110%.

### V14.17
- Piercing Arrow
  - Minimum base damage increased to 60 / 106.67 / 153.33 / 200 / 246.67 from 10 / 46.67 / 83.33 / 120 / 156.67.
    - Maximum base damage increased to 90 / 160 / 230 / 300 / 370 from 15 / 70 / 125 / 180 / 235.
  - Minimum AD ratio changed to 100 / 106.67 / 113.33 / 120 / 126.67% **bonus** AD from 83.33 / 86.67 / 90 / 93.33 / 96.67% **total** AD.
    - Maximum AD ratio changed to 150 / 160 / 170 / 180 / 190% **bonus** AD from 125 / 130 / 135 / 140 / 145% **total** AD.
- Hail of Arrows
  - Bonus AD ratio increased to 110% **bonus** AD from 90%.

### V14.16
- Piercing Arrow
  - **Bug Fixes:** No longer casts in a different direction from the cursor if used while ending a long distance movement.

## Trivia

- Varus was the first champion to be released in Season Two (2012) with only one alternative skin, with subsequent champions following this pattern.
- Varus' title "*The Arrow of Retribution"* strongly resembles Kalista’s title *"The Spear of Vengeance"*.
- Varus is, along with Blitzcrank, Caitlyn, Lissandra, Rumble, Sion, Vi, Xerath, and Ziggs, one of the few champions that can apply crowd control on themselves.
- Varus' body style is similar to the one of *Solus*, the T'Lan leader from the game *Breakdown*.
- Varus is the first of three dark-themed champions with a light-themed skin, the second being Syndra and the third being Aatrox.
  - Of these three, Varus' skin is the only one not of the Justicar theme.
- Varus is the only Darkin champion whose abilities currently have ability power ratios.
  - This is most likely because Varus was retconned into being a Darkin in 2017, 5 years after his release in 2012.
  - Prior to Aatrox’s rework in V8.13, his then-ultimate Massacre also had an ability power ratio.

---
*This page was automatically generated from League of Legends Wiki data.*