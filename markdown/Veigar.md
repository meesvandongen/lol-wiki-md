# Veigar

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
| **Champion** | Veigar |
| **Title** | the Tiny Master of Evil |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2009-07-24 |
| **Release Patch** | V0.8.22.115 |
| **Latest Changes** | V25.09 |
| **Roles** | Burst |
| **Riot Positions** | Middle |
| **External Positions** | Middle, Support |
| **Blue Essence** | 675 |
| **Riot Points** | 585 |
| **Difficulty** | 2 |
| **Hero Type** | Mage |
| **Adaptive Type** | Magic |
| **Damage** | 3 |
| **Toughness** | 1 |
| **Control** | 3 |
| **Mobility** | 1 |
| **Utility** | 1 |
| **Style** | 100 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $580.0$ | $+108.0$ |
| **Mana** | $490.0$ | $+26.0$ |
| **Health Regen** | $6.5$ | $+0.6$ |
| **Mana Regen** | $8.0$ | $+0.8$ |
| **Armor** | $18.0$ | $+5.2$ |
| **Magic Resist** | $32.0$ | $+1.3$ |
| **Attack Damage** | $52.0$ | $+2.7$ |
| **Attack Speed** | $0.625$ | |
| **Movement Speed** | $340.0$ | $+0.0$ |
| **Attack Range** | $550.0$ | $+0.0$ |
| **Base Attack Speed** | $0.625$ | |
| **Attack Speed Ratio** | $0.625$ | |
| **Bonus AS per Level** | $2.2\%$ | |
| **Missile Speed** | $1500$ units/second | |
| **Acquisition Radius** | $525$ units | |
| **Gameplay Radius** | $55$ units | |
| **Pathing Radius** | $30$ units | |
| **Selection Radius** | $100$ units | |
| **Selection Height** | $140$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $93.0\%$ |
| **Damage Taken** | $110.0\%$ |

#### URF

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $90.0\%$ |

## Abilities

### Passive: Phenomenal Evil Power

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Self |

**INNATE:** **Veigar** generates a stack of *Phenomenal Evil* for each enemy champion hit by his abilities, and 5 stacks whenever he scores an enemy champion takedown.

**PHENOMENAL EVIL:** For each stack, **Veigar** gains (AP) 1 ability power.

**Notes:**

- The two effects stack, granting 6 stacks for each enemy champion taken down by an ability, 7 with Baleful Strike.
- **Veigar**’s abilities do not have to deal damage nor affect their targets to grant a stack.

---

### Q: Baleful Strike

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 1050 / er 990 units |
| **Width** | 140 units |
| **Speed** | 2200 units/second |
| **Cost** | 30 / 35 / 40 / 45 / 50 mana |
| **Cooldown** | 6 / 5.5 / 5 / 4.5 / 4 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | True |

**ACTIVE:** **Veigar** blasts a dark bolt in the target direction that deals magic damage to the first two enemies hit.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 80 / 120 / 160 / 200 / 240 (+ 50 / 55 / 60 / 65 / 70% AP) |

If this kills an enemy, it generates 1 stack of *Phenomenal Evil*, tripled against large minions and monsters.

**Notes:**

- Killing two enemy champions with *Baleful Strike* grants .
- *Baleful Strike* will only grant stacks of Phenomenal Evil from its bolt, and not any additional effect that would kill other units from the same cast.
- The stacks are added immediately on enemy kill (on next game tick). **If the first target was killed, the second hit may deal 1 damage more on enemies further along the missile's flight path. - This ability will cast from wherever the caster is at the end of the cast time.

---

### W: Dark Matter

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 950 units |
| **Effect Radius** | $240$ units |
| **Cost** | 60 / 65 / 70 / 75 / 80 mana |
| **Cooldown** | 20pxNumber of times that 50 Phenomenal Evil stacks are earned.* |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |
| **Out of Range Behavior** | walk to location |

**ACTIVE:** **Veigar** casts down a mass of dark matter that strikes the target location after a delay (from the start of the cast time), dealing magic damage to enemies hit.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 85 / 140 / 195 / 250 / 305 (+ 60 / 70 / 80 / 90 / 100% AP) |

*Dark Matter*’s cooldown is reduced based on stacks of *Phenomenal Evil*.

**Notes:**

- The delay starts at the beginning of the cast time.
  - If **Veigar** dies during the cast time, *Dark Matter* will still successfully fall from the sky.
- *Dark Matter* grants sight of the area during the delay.

---

### E: Event Horizon

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 725 units |
| **Effect Radius** | 411 (Outer radius) / 290 (Safe zone radius) / 390 (Credit marker radius) units |
| **Cost** | 70 / 75 / 80 / 85 / 90 mana |
| **Cooldown** | 20 / 18.5 / 17 / 15.5 / 14 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Spell Shield** | True |
| **Out of Range Behavior** | walk to location |

**ACTIVE:** **Veigar** forms a cage at the target location that erects after a $0.5$ second delay, remaining there for 3 seconds. Enemies that collide with the edges of the cage are knocked down and stunned for a duration.

| Attribute | Value |
|-----------|------:|
| **Stun Duration** | 1.5 / 1.75 / 2 / 2.25 / 2.5 seconds |

*Event Horizon* can affect enemies only once per cast.

**Notes:**

- The stun and the knockdown are applied to units that are within a certain distance interval from the center of the area of effect.
- An enemy can be affected by *Event Horizon* only once every $4.5$ seconds.
- Units that negate the stun will still count as passing through the edges. They will become immune to the effects of *Event Horizon* for the period even after negating them.
- Displacement immune enemies will still be stunned, but not knocked down.
- **Veigar** marks enemy champions in a 390-radius from the zone's center in order to gain kill/assist credit, lasting for the standard credit timer.
- *Event Horizon* may fail to catch enemies that move very quickly through the boundary distance.
  - More modern area checks such as Thresh’s The Box or Caitlyn’s Yordle Snap Trap do not have this issue.

---

### R: Primordial Burst

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 650 units |
| **Cost** | 100 mana |
| **Cooldown** | 100 / 90 / 80 / 70 / 60 seconds |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Single target |
| **Projectile** | True |
| **Call For Help** | True |

**ACTIVE:** **Veigar** sends a primordial burst at the target enemy champion that deals magic damage, increased by 0% / 10% / 20% / 30% / 40% / 50% / 60% / 70% / 80% / 90% / 100%.

| Attribute | Value |
|-----------|------:|
| **Minimum Magic Damage** | 175 / 212.5 / 250 / 287.5 / 325 (+ 65 / 67.5 / 70 / 72.5 / 75% AP) |
| **Maximum Magic Damage** | 350 / 425 / 500 / 575 / 650 (+ 130 / 135 / 140 / 145 / 150% AP) |

**Notes:**

No additional notes.

---

## Patch History

### V25.09
- Veigar
  - **Bug Fixes:** Killing a minion using Baleful Strike no longer causes the minion gold bounty's floating text to appear above his head instead of on the minion's death location.

### V14.24
- Dark Matter
  - **Bug Fixes:** No longer reveals the user if cast while the cursor is near an enemy champion.
- Event Horizon
  - **Bug Fixes:** No longer reveals the user if cast while the cursor is near an enemy champion.

### V14.20
- Dark Matter
  - AP ratio reduced to 60 / 70 / 80 / 90 / 100% AP from 70 / 80 / 90 / 100 / 110% AP.

### V14.9
- Stats
  - Selection radius increased to 100 units from 93.

### V14.5
- Baleful Strike
  - AP ratio increased to 50 / 55 / 60 / 65 / 70% AP from 45 / 50 / 55 / 60 / 65% AP.
- Primordial Burst
  - Cooldown reduced to 100 / 80 / 60 seconds from 120 / 90 / 60.

### V14.2
- Stats
  - Base health increased to 580 from 550.

### V13.22
- Stats
  - Basic attack missile speed increased to 1500 from 1100.

### V13.7
- Baleful Strike
  - AP ratio changed to 45 / 50 / 55 / 60 / 65% AP from 60% AP at all ranks.
- Dark Matter
  - AP ratio changed to 70 / 80 / 90 / 100 / 110% AP from 100% AP at all ranks.
- Event Horizon
  - **Removed:*** Can no longer hide his VFX from enemies by placing it inside certain walls.
  - **Bug Fixes:** Zone placement particles are now properly visible to enemies from the Fog of War.
- Primordial Burst
  - AP ratio reduced to 65 / 70 / 75% AP from 75% AP at all ranks.

### V13.6
- Stats
  - Base health reduced to 550 from 575.
  - Base armor reduced to 18 from 21.
- Dark Matter
  - Base damage changed to 85 / 140 / 195 / 250 / 305 from 100 / 150 / 200 / 250 / 300.

### V13.5
- General
  - **Bug Fixes:** Now properly plays his laugh voice line upon scoring a killing blow with Primordial Burst.

## Trivia

- 
  - In Veigar's case, Phenomenal Evil Power infinitely stacks his ability power.
- When Veigar uses Primordial Burst to kill an enemy champion, he will laugh malevolently.
- Veigar possess the most laugh emotes of any champion, at 6.
- Veigar's Series 1 Eternals make the following references:
  - *Ctrl+Alt+Del* is a reference to the keyboard combination that would reboot most early versions of many computers, this would later be changed in Windows 95 where a task manager would be opened instead.

---
*This page was automatically generated from League of Legends Wiki data.*