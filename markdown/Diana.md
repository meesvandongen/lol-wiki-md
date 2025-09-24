# Diana

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
| **Champion** | Diana |
| **Title** | Scorn of the Moon |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2012-08-07 |
| **Release Patch** | V1.0.0.144 |
| **Latest Changes** | V25.18 |
| **Roles** | Assassin, Diver |
| **Riot Positions** | Jungle, Middle |
| **External Positions** | Jungle, Middle |
| **Blue Essence** | 225 |
| **Riot Points** | 260 |
| **Difficulty** | 1 |
| **Hero Type** | Fighter |
| **Alt Type** | Assassin |
| **Adaptive Type** | Magic |
| **Damage** | 3 |
| **Toughness** | 2 |
| **Control** | 2 |
| **Mobility** | 2 |
| **Utility** | 1 |
| **Style** | 65 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $640.0$ | $+109.0$ |
| **Mana** | $375.0$ | $+25.0$ |
| **Health Regen** | $6.5$ | $+0.85$ |
| **Mana Regen** | $8.0$ | $+0.8$ |
| **Armor** | $31.0$ | $+4.3$ |
| **Magic Resist** | $32.0$ | $+2.05$ |
| **Attack Damage** | $57.0$ | $+3.0$ |
| **Attack Speed** | $0.625$ | |
| **Movement Speed** | $345.0$ | $+0.0$ |
| **Attack Range** | $150.0$ | $+0.0$ |
| **Base Attack Speed** | $0.625$ | |
| **Attack Speed Ratio** | $0.694$ | |
| **Bonus AS per Level** | $2.0\%$ | |
| **Acquisition Radius** | $600$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $120$ units | |
| **Selection Height** | $188.889$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $100.0\%$ |
| **Damage Taken** | $100.0\%$ |

## Abilities

### Passive: Moonsilver Blade

| Attribute | Value |
|-----------|------:|
| **Effect Radius** | 175 units |
| **Targeting** | Passive |
| **Affects** | Self / Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Parry** | True |

**INNATE:** **Diana** gains (attack speed) 15 to 35 **bonus** attack speed. After casting an ability, this bonus is tripled to 15×3 to 35×3 for 5 seconds.

**INNATE - MOONSILVER BLADE:** **Diana**’s basic attacks generate a stack of *Moonsilver Blade* for 5 seconds, refreshing on subsequent attacks and stacking up to 2 times. At 2 stacks, **Diana** empowers her next basic attack to consume the stacks on-hit to additionally cleave nearby enemies, dealing them 20+5*(x-1) for 6 / then + 10*x for 5 / then + 15*x for 5 / then + 25*x (+ 50% AP) magic damage. *Moonsilver Blade* deals 260% damage against monsters.

**Notes:**

- *Moonsilver Blade* will affect structures.
- Each attack has a distinct animation and she will glow when the empowered attack is ready.
- Sometimes, the empowered basic attack will not consume the stacks.
  - In this case, the attack will also not deal its damage.

---

### Q: Crescent Strike

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 900 units |
| **Speed** | 1900 (Inner arc speed) / 2100 (Outer arc speed) units/second |
| **Cost** | 50 Mana |
| **Cooldown** | 8 / 7.5 / 7 / 6.5 / 6 seconds |
| **Queue Time** | $0.5$ seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | Special |
| **Spell Effects** | aoe |
| **Projectile** | True |
| **Out of Range Behavior** | cast at max |

**ACTIVE:** **Diana** unleashes a bolt of lunar energy that travels in a counter-clockwise arc before exploding at the target location, granting sight of the area for $0.5$ seconds and dealing magic damage to enemies hit and afflicting them with *Moonlight* for 3 seconds, during which they are revealed.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 70 / 105 / 140 / 175 / 210 (+ 70% AP) |

*Lunar Rush* interacts with *Moonlight*.

**Notes:**

- Even if the target spell shields the bolt, they may still be damaged by the explosion. *Moonlight* is not applied to the protected target if blocked.
- The bolt consists of two projectiles with slightly different trajectories. Effect at cast time end

---

### W: Pale Cascade

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 200 (Orbit and explosion) units |
| **Cost** | 40 / 45 / 50 / 55 / 60 mana |
| **Cooldown** | 15 / 13.5 / 12 / 10.5 / 9 seconds |
| **Cooldown Start** | After sphere detonation |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | True |

**ACTIVE:** **Diana** grants herself a shield for up to 5 seconds and creates three spheres that orbit her counterclockwise for the same duration, detonating upon contact with an enemy to deal magic damage to nearby enemies.

| Attribute | Value |
|-----------|------:|
| **Shield Strength** | 45 / 60 / 75 / 90 / 105 (+ 30% AP) (+ 9% **bonus** health) |

| Attribute | Value |
|-----------|------:|
| **Magic Damage per Orb** | 20 / 32 / 44 / 56 / 68 (+ 18% AP) |
| **Total Magic Damage** | 60 / 96 / 132 / 168 / 204 (+ 54% AP) |

If all three spheres detonate, *Pale Cascade*’s shield is reapplied, stacking with its original shield and refreshing the duration.

| Attribute | Value |
|-----------|------:|
| **Maximum Shield Strength** | 90 / 120 / 150 / 180 / 210 (+ 60% AP) (+ 18% **bonus** health) |

**Notes:**

- On the second application of *Pale Cascade*’s shield, Shield Power will apply to both the new shield amount and the shield amount remaining from the first shield, which has already benefited from *shield power*.
  - Because of this, *shield power* effectively applies to the second shield twice, but with reduced efficiency, for up to an increase of 50% of *shield power* (maximum benefit if no damage was mitigated by the first shield). With 10% *shield power*, the second shield's total amount will be increased by up to an additional 5%, for a total of $15.5%$ (1.1×1.05) **bonus** shield.
  - The formula for the total amount of shield **Diana** will receive from both shield applications is:
    - *Total Shield = (((Shield Amount × (1 + Shield Power)) - Damage Blocked by Shield) + Shield Amount) × (1 + Shield Power)*

---

### E: Lunar Rush

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 825 units |
| **Cost** | 40 / 45 / 50 / 55 / 60 mana |
| **Cooldown** | 22 / 20 / 18 / 16 / 14 seconds |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Single target |
| **Call For Help** | True |
| **Grounded** | True |
| **Knockdown** | True |

**ACTIVE:** **Diana** dashes to the target enemy's location, and upon completion, she deals them magic damage and consumes *Moonlight* from all enemies. If the target is within 400 range, **Diana** will dash through their location.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 50 / 70 / 90 / 110 / 130 (+ 60% AP) |

If *Moonlight* is consumed from the target, *Lunar Rush*’s **current** cooldown is reduced to $0.25$ seconds.

***Diana** can cast any of her abilities during the dash.*

**Notes:**

- **Diana** will attempt to basic attack the target.
- *Lunar Rush*’s cooldown will also be reset if the target dies during the dash while being affected by *Moonlight*.
- *Lunar Rush* consumes *Moonlight* upon ending the dash (even if interrupted).
- *Lunar Rush* will still deal damage even if the target is untargetable by the end of the dash.
- If *Lunar Rush* is blocked by spell shield the *Moonlight* debuff is still consumed but *Lunar Rush*’s cooldown is not reset.

---

### R: Moonfall

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Effect Radius** | 475 (Effect radius) / 225 (Maximum Pull distance) units |
| **Cost** | 100 Mana |
| **Cooldown** | 100 / 95 / 90 / 85 / 80 seconds |
| **Queue Time** | $0.5$ seconds |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | Special |
| **Spell Effects** | spellaoe |

**ACTIVE:** **Diana** pulls in all nearby enemies, during which they are revealed, then slows them for 2 seconds.

| Attribute | Value |
|-----------|------:|
| **Slow** | 40 / 45 / 50 / 55 / 60% |

If an enemy champion is pulled in, she calls down a beam of moonlight to strike upon the area around her after 1 second, dealing magic damage to all nearby enemies, increased for each champion pulled beyond the first.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 200 / 250 / 300 / 350 / 400 (+ 60% AP) |

| Attribute | Value |
|-----------|------:|
| **Bonus Damage Per Champion** | 35 / 47.5 / 60 / 72.5 / 85 (+ 15% AP) |
| **Total Damage Vs. 5 Champions** | 340 / 440 / 540 / 640 / 740 (+ 120% AP) |

**Notes:**

- Enemies stay in the area for a maximum of approximately $0.5$ seconds while being pulled.
- Crowd control immunity and displacement immunity still count as being "pulled in" by the ability.
- Clones count as champions for increasing this ability's damage.
- *Moonfall*’s additional damage is not capped at 4 champions.
- The beam of moonlight does not crash down if champions hit blocked the initial effect with spell shield.
  - Enemy champions protected by *spell shield* do not count towards the damage increase. Effect at cast time end
- A lunar phase of the moon will appear above **Diana** while she is casting *Moonfall*, each phase is based on the number of enemy champions pulled: New_Moon_Moonfall_(1_Enemy_Champion).png|**NEW MOON:** **1** Enemy Champion Pulled Waning_Crescent_Moonfall_(2_Enemy_Champions).png|**WANING CRESCENT:** **2** Enemy Champions Pulled Third_Quarter_Moonfall_(3_Enemy_Champions).png|**THIRD QUARTER:** **3** Enemy Champions Pulled Waning_Gibbous_Moonfall_(4_Enemy_Champions).png|**WANING GIBBOUS:** **4** Enemy Champions Pulled Full_Moon_Moonfall_(5_Enemy_Champions).png|**FULL MOON:** **5** Enemy Champions Pulled

---

## Patch History

### V25.18
- Moonfall
  - **Bug Fixes:** Corrected damage area warning VFX, which was previously smaller than the actual hit area.

### V25.11
- Moonsilver Blade
  - Cleave monster damage increased to 260% from 225%.

### V25.09
- Crescent Strike
  - **Bug Fixes:** Missile range is no longer shorter or longer than intended if Flash is used during the cast time.
    - *This bug fix had been noted before on patch

### V25.04
- Moonsilver Blade
  - Cleave monster damage reduced to 225% from 300%.
- Crescent Strike
  - Base damage increased to 70 / 105 / 140 / 175 / 210 from 60 / 95 / 130 / 165 / 200.
- Pale Cascade
  - Base damage per hit increased to 20 / 32 / 44 / 56 / 68 from 18 / 30 / 42 / 54 / 66.
    - Total base damage increased to 60 / 96 / 132 / 168 / 204 from 54 / 90 / 126 / 162 / 198.

### V14.9
- Stats
  - Selection radius increased to 120 units from 75.

### V14.7
- Moonsilver Blade
  - **Bug Fixes:** Tooltip now notes the correct duration of the buff due to 14.6 changes.

### V14.6
- Stats
  - Attack speed ratio increased to $0.694$ from $0.625$.
  - Attack speed growth reduced to 2% from $2.25$%.
- Moonsilver Blade
  - Bonus attack speed reduced to 15 to 35 from 15 to 40.
    - Empowered bonus attack speed reduced to 45 to 105 from 45 to 120.
  - Empowered bonus attack speed duration increased to 5 seconds from 3.
- Pale Cascade
  - **Bug Fixes:** The initial shield no longer lasts 1 second shorter than intended.
- Lunar Rush
  - **Bug Fixes:** The cooldown reset now properly takes place upon colliding with the target.

### V14.5
- Moonsilver Blade
  - **Bug Fixes:** Buff no longer desyncs to trigger the empowered attack every 2 or 4 basic attacks, instead of the intended 3.
- Lunar Rush
  - Cooldown reset upon consuming Moonlight reduced to $0.25$ seconds from $0.5$.
  - **Bug Fixes:** Cooldown is no longer sometimes reset a second time after only consuming Moonlight once.

### V14.4
- Diana
  - **Bug Fixes:** Pentakill VFX on the river terrain in Summoner's Rift is now properly rendered and no longer renders slightly higher than intended on normal elevated terrain.

### V13.14
- Pale Cascade
  - Base shield increased to 45 / 60 / 75 / 90 / 105 from 40 / 55 / 70 / 85 / 100.
  - Shield AP ratio increased to 30% AP from 25% AP.
  - Damage AP ratio per orb increased 18% AP from 15% AP.
- Lunar Rush
  - AP ratio increased to 60% AP from 50% AP.

## Trivia

- *Diana* derives from Proto-Indo-European language root Dyeus "to shine".
  - She shares her name with Diana). Roman goddess of the Moon.
- Diana is the first champion to feature animations for when affected by both crowd control and movement speed buffs.
- Moonfall’s indicator above Diana references the Lunar phase.
  - 1 enemy: New moon
  - 2 enemies: Waning crescent
  - 3 enemies: Last quarter
  - 4 enemies: Waning gibbous
  - 5 enemies: Full moon
- A functional replica of Moonfall old.png was crafted in an episode of YouTube series Man At Arms: Reforged.
  - There are also videos where the following are crafted:
    - Katarina’s Daggers (functional)
    - Leona’s Zenith Blade (functional)
    - Master Yi’s Highlander' Ring Sword (functional)
    - Poppy’s Hammer of Orlon (functional)
    - Yasuo’s Last Breath' Sword (functional)
    - Ziggs’ Hexplosive Bomb (prop)

---
*This page was automatically generated from League of Legends Wiki data.*