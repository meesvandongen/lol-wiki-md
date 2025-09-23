# Kindred

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
| **Champion** | Kindred |
| **Title** | The Eternal Hunters |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2015-10-14 |
| **Release Patch** | V5.20 |
| **Latest Changes** | V25.13 |
| **Roles** | Marksman |
| **Riot Positions** | Jungle |
| **External Positions** | Jungle |
| **Blue Essence** | 2400 |
| **Riot Points** | 880 |
| **Difficulty** | 3 |
| **Hero Type** | Marksman |
| **Adaptive Type** | Physical |
| **Damage** | 3 |
| **Toughness** | 1 |
| **Control** | 2 |
| **Mobility** | 3 |
| **Utility** | 2 |
| **Style** | 20 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $595.0$ | $+104.0$ |
| **Mana** | $300.0$ | $+35.0$ |
| **Health Regen** | $7.0$ | $+0.55$ |
| **Mana Regen** | $7.0$ | $+0.4$ |
| **Armor** | $29.0$ | $+4.7$ |
| **Magic Resist** | $30.0$ | $+1.3$ |
| **Attack Damage** | $65.0$ | $+3.25$ |
| **Attack Speed** | $0.625$ | |
| **Movement Speed** | $325.0$ | $+0.0$ |
| **Attack Range** | $500.0$ | $+0.0$ |
| **Base Attack Speed** | $0.625$ | |
| **Attack Speed Ratio** | $0.625$ | |
| **Bonus AS per Level** | $3.5\%$ | |
| **Missile Speed** | $2000$ units/second | |
| **Acquisition Radius** | $525$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $120$ units | |
| **Selection Height** | $155$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $110.0\%$ |
| **Damage Taken** | $90.0\%$ |

#### Ultimate Spellbook

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $95.0\%$ |
| **Damage Taken** | $105.0\%$ |

## Abilities

### Passive: Mark of the Kindred

| Attribute | Value |
|-----------|------:|
| **On-target CD Static** | 240 (Lamb's mark) |
| **Targeting** | Passive |
| **Affects** | Self, Enemies |

**INNATE:** Both **Lamb** and **Wolf** mark targets to *hunt*. Scoring a takedown against a *hunted* target collects a stack of *Mark of the Kindred*. 

**MARK OF THE KINDRED:** **Lamb** gains type=marks range on her basic attacks and *Mounting Dread*.

**INNATE - LAMB:** **Lamb** is offered a selection of enemy champions to *hunt* if she has not been in combat with them in the last 6 seconds. Once selected, the mark is applied after an 8-second delay and thereafter it can be collected by **Kindred**. **Lamb** gains the ability to select a new target every 75 seconds. *Targets successfully hunted cannot be marked again for 4 minutes.*

**INNATE - WOLF:** Starting at **3:15**, **Wolf** periodically marks a random large monster within the enemy team's jungle for 180 seconds. The *hunted* camp is highlighted on the mini map.md) to both teams. Once the *hunted* target is slain or the mark expires, **Wolf** will wait 45 seconds before selecting a new target. 
The type of monster that **Wolf** can mark changes based on **Kindred**’s current *Mark of the Kindred* stacks:
- **0 :** Rift Scuttler
- **1 – 3:** Rift Scuttler, Crimson Raptor, Gromp
- **4 – 7:** Ancient Krug, Blue Sentinel, Greater Murk Wolf, or Red Brambleback
- **8 + :** Rift Herald or Baron Nashor, Dragon or Elder Dragon

**Notes:**

- The selection of enemy champions to mark is offered through a special menu in the HUD that displays the portraits of up to 5 enemy champions. The player can mark an enemy by clicking on their portrait or via the Champion Specific Interaction hotkeys (default: **SHIFT**+**F1–F5**).
  - Marked enemies have a trim around their portrait. Unavailable selections are greyed out.
  - The portraits are ordered based on their spawn ID.
- Takedowns on monsters occur if **Kindred** had damaged them within the last 6 seconds.
  - This only includes if the monster was slain by **Kindred** or an ally.
- Only killing the large monster is required to claim **Wolf**’s mark.
- Gaining stacks from champions that would change what constitutes a valid target for **Wolf** will not affect **Kindred**’s ability to claim **Wolf**’s current target.
- **Wolf** cannot mark monsters that have been slain, even if **Kindred**’s team doesn't know that the camp has been slain.
- **Wolf**’s cooldown is not displayed in-game. Combined with the above point and the target restrictions, **Wolf** may go extended periods of time without seeming to do anything.
- **Wolf**’s marked target will always have its mark removed from the map 15 seconds after dying for both teams, regardless of whether they have vision of it or not.

---

### Q: Dance of Arrows

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 300 (Dash range, no minimum dash range) units |
| **Effect Radius** | er Kindred's |
| **Speed** | 500 + units/second |
| **Cost** | 35 Mana |
| **Cooldown** | 9 seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |
| **Projectile** | True |
| **Out of Range Behavior** | cast at max |
| **Terrain Grace** | True |
| **Grounded** | True |
| **Knockdown** | True |

**ACTIVE:** **Lamb** dashes toward the target location, gaining attack speed (+ 5% per mark) **bonus** attack speed for 4 seconds and firing an arrow at up to 3 nearby visible enemies that deals physical damage. Her current attack target within any proximity will be prioritized by one of the arrows.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 40 / 65 / 90 / 115 / 140 (+ 75% **bonus** AD) |

*Dance of Arrows' * **total** cooldown is reduced to an amount while **Lamb** is within the area of *Wolf's Frenzy*. Casting *Wolf's Frenzy* reduces *Dance of Arrows' * **current** cooldown to the same amount.

| Attribute | Value |
|-----------|------:|
| **Static Cooldown** | 4 / 3.5 / 3 / 2.5 / 2 seconds |

*Dance of Arrows resets **Lamb**’s basic attack timer. **Lamb** can cast any of her abilities during the dash.*

**Notes:**

- The dash distance can be extended to up to 400 units when dashing across terrain.
- The cooldown will not be modified if it cannot be reduced (the current cooldown would have to be less than the reduction amount in this case).

---

### W: Wolf's Frenzy

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 500 units |
| **Effect Radius** | 800 units |
| **Speed** | 1400 units/second |
| **Cost** | 40 Mana |
| **Cooldown** | 18 / 17 / 16 / 15 / 14 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |
| **Projectile** | False |
| **Out of Range Behavior** | cast at max |

**PASSIVE:** **Lamb** generates 1 stack of *Hunter's Vigor* for every 27 units (Estimated) she travels by any means and 5 stacks on-attack, up to a maximum of 100 stacks. At maximum stacks, her next basic attack heals her for 0 to 100 by 25 of 47 to 81. The heal is not triggered if **Kindred** is at full health.

**ACTIVE:** **Wolf** dashes to the target location, then claims the surrounding area as his territory for the next $8.5$ seconds, separating from **Lamb**. He automatically attacks the closest nearby visible enemy within the area, prioritizing the last enemy **Lamb** has attacked, then enemy champions, then non-champions.

**Wolf**’s attacks deal magic damage and the rate at which he attacks scale with 25% of **Kindred's bonus** attack speed. Against monsters, his attacks deal 150% damage and slow the target by 50% for 2 seconds. The damage based on the target's health ratio is capped at $112.5$ (+ 15 per mark) against monsters.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 25 / 30 / 35 / 40 / 45 (+ 20% **bonus** AD) (+ 20% AP) (+ $1.5$% (+ 1% per mark) of target's **current** health) |
| **Monster Damage** | 37.5 / 45 / 52.5 / 60 / 67.5 (+ 30% **bonus** AD) (+ 30% AP) (+ 2.25% (+ 1.5% per mark) of target's **current** health) |

*Wolf's Frenzy* ends immediately if **Lamb** leaves the area or dies.

**Notes:**

- *Hunter's Vigor*’s heal is not triggered on attacks against structures and wards.
- **Wolf** is untargetable and ghosted but he cannot move through terrain other than the initial dash on activation.
- **Wolf** is not able to re-target Rift Scuttler if he loses sight of her while he's attacking it.
- **Wolf** grants the same vision as a champion.
- *Wolf's Frenzy* does not end if **Lamb** enters resurrection.
- *Wolf's Frenzy* applies ability haste to its cooldown post-effect.
  - Given a base cooldown of 14 seconds at rank 5 with 100 ability haste, casting the ability and then ending it after 4 seconds will result in a remaining cooldown of (14 - 4) \times 0.5 = 5 seconds, where 0.5 is derived from ability haste.
  - Because of this, there will always be some cooldown time remaining post-effect regardless of the amount of ability haste, unlike certain other abilities with cooldowns that start on-cast.
- : Interaction with enemy vision and spectator mode.

---

### E: Mounting Dread

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 500@0; 575@4; 600@7; 625@10; 650@13; 675@16; 700@19; 725@22; 750@25 (@=Marks) units |
| **Cost** | 50 Mana |
| **Cooldown** | 14 / 12.5 / 11 / 9.5 / 8 seconds |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | spell |
| **Projectile** | True |
| **Parry** | True |

**ACTIVE:** **Lamb** fires a shot at the target enemy that slows them by 30% (+ 5% per 100 AP) for 1 second and marks them for 4 seconds. Her basic attacks against the marked target each apply a stack, refreshing the duration and stacking up to 3 times.

The third stack directs **Wolf** to pounce on the target, consuming all stacks to deal **additional** physical damage, capped at 300 against monsters.

| Attribute | Value |
|-----------|------:|
| **Additional Physical Damage** | 80 / 110 / 140 / 170 / 200 (+ 100% **bonus** AD) (+ 5% (+ $0.5$% per Mark) of target's **missing** health) |
| **Enhanced damage below threshold** | 80 / 110 / 140 / 170 / 200 (+ 100% **bonus** AD) critical strike (+ $7.5$% (+ Infinity Edge 2%) (+ $0.75$% (+ Infinity Edge $0.2$%) per Mark) of target's **missing** health) |

The **missing** health portion of the **additional** damage will critically strike for damage if the target is below type=critical strike chance of their **maximum** health and cannot critically strike otherwise. The **base** damage of the pounce can independently critically strike for damage.

**Notes:**

- If the target becomes untargetable, dies, or is too far away or no longer in sight during the cast time, this ability will cancel and not pay its cost but still go on cooldown.
- This ability cannot target a unit with *Mounting Dread* stacks.
- *Mounting Dread* will prioritize casting on the closest champion within cr 100 (estimated) radius of the cursor, even if other eligible targets are closer.
  - As a consequence of this mechanic, *Mounting Dread* can still be cast on champions with *Mounting Dread* stacks.
    - Reapplying *Mounting Dread* through this method will apply the slow, but will not reset the stack count.
- Reapplying *Mounting Dread* against disguised Neeko will reset the stack count.

---

### R: Lamb's Respite

| Attribute | Value |
|-----------|------:|
| **Cast Time** | None |
| **Effect Radius** | 535 units |
| **Cost** | 100 Mana |
| **Cooldown** | 160 / 150 / 140 / 130 / 120 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Auto |
| **Affects** | Self, Allies, Enemies |

**ACTIVE:** **Lamb** blesses the ground underneath herself and enters a cast time, creating a sacred zone at her location at the time of cast that lasts for 4 seconds. **All** units (Ally, enemy, or neutral) inside the zone gain a minimum health threshold equal to health, and will also become invulnerable while remaining in the area when they reach or are at the threshold, during which they also cannot be healed, but can still health regeneration.

All targetable units within the zone are healed when the blessing ends.

| Attribute | Value |
|-----------|------:|
| **Heal** | 225 / 262.5 / 300 / 337.5 / 375 |

**Notes:**

- *Lamb's Respite* will affect all champions, minions, and monsters, but not wards, turrets, or jungle plants.
- *Lamb's Respite* will affect untargetable units but does not grant them the heal at the end of the blessing.
  - It will specifically not affect champions that are resurrecting from Guardian Angel.
- Health costs will ignore *Lamb's Respite*’s threshold.
  - The cost will not be paid if the caster is below the threshold.
- *Lamb's Respite* has no effect on zombie state or clone units.
- The effects are applied before the cast time.
- *Lamb's Respite* will have no effect if cast while **Kindred** is untargetable.

---

## Patch History

### V25.13
- Lamb's Respite
  - Cooldown reduced to 160 / 140 / 120 seconds from 180 / 150 / 120.

### V25.S1.3
- Mark of the Kindred
  - **Bug Fixes:** Marked buff is no longer invisible on Atakhan status bar.

### V14.17
- Dance of Arrows
  - Base bonus attack speed increased to 35% from 30%.
- Mounting Dread
  - Cooldown reduced to 14 / 12.5 / 11 / 9.5 / 8 seconds from 14 / 13 / 12 / 11 / 10.
  - Base damage increased to 80 / 110 / 140 / 170 / 200 from 80 / 100 / 120 / 140 / 160.
  - Bonus AD ratio increased to 100% **bonus** AD from 80%.

### V14.14
- Stats
  - Base health reduced to 595 from 610.
- Dance of Arrows
  - Base bonus attack speed reduced to 30% from 35%.

### V14.10
- General
  - **Bug Fixes:** Restored idle VO quotes.

### V14.7
- Lamb's Respite
  - **Bug Fixes:** Champions within that are affected by Bailout and reach the threshold after scoring a champion takedown while still inside its area of effect no longer sometimes forcibly die.

### V14.3
- Mark of the Kindred
  - **Bug Fixes:** No longer sometimes marks enemies' Hubris statues.

### V13.17
- Mounting Dread
  - Base slow reduced to 30% from 50%.
  - Health ratio reduced to 5% of target's **missing** health from 8%.

### V13.16
- Mounting Dread
  - **Bug Fixes:** VFX now properly renders above impassable terrain.

### V13.14
- Stats
  - Base health increased to 610 from 580.
  - Base armor increased to 29 from 26.
- Mark of the Kindred
  - **Bug Fixes:** Can no longer re-mark targets before their hunt timer came off cooldown.
- Dance of Arrows
  - Cooldown increased to 9 seconds from 8.
  - Base damage reduced to 40 / 65 / 90 / 115 / 140 from 50 / 75 / 100 / 125 / 150.

## Trivia

- 
  - In Kindred's case, Mark of the Kindred infinitely stacks the bonus attack speed of Dance of Arrows, magic damage of Wolf's Frenzy and bonus physical damage of Mounting Dread.
- Kindred is the first champion to not have their icon derived from their artwork (considering theirs is a two-for-one case) as well as the first whose release was celebrated with summoner icons (one for Lamb and one for Wolf).
- Kindred's dance references the 'Cups' tap dance performed by Christopher Rice.New Champion Q&A: Kindred, the Eternal Hunters
  - A side-by-side comparison can be seen here.
- In the classic splash art, Lamb's hand has 4 fingers (with a possible hidden thumb) while in-game and in Kindred splash art she has 3 fingers.

---
*This page was automatically generated from League of Legends Wiki data.*