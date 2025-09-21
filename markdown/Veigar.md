# Veigar

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
| **Champion** | Veigar |
| **Title** | the Tiny Master of Evil |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2009-07-24 |
| **Release Patch** | V0.8.22.115 |
| **Roles** | Burst |
| **Riot Positions** | Middle |
| **External Positions** | Middle, Support |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $580.0$ | $+108.0$ | $2416.0$ |
| **Mana** | $490.0$ | $+26.0$ | $932.0$ |
| **Health Regen** | $6.5$ | $+0.6$ | $16.7$ |
| **Mana Regen** | $8.0$ | $+0.8$ | $21.6$ |
| **Armor** | $18.0$ | $+5.2$ | $106.4$ |
| **Magic Resist** | $32.0$ | $+1.3$ | $54.1$ |
| **Attack Damage** | $52.0$ | $+2.7$ | $97.9$ |
| **Attack Speed** | $0.625$ | $+2.2\%$ | $0.863$ |
| **Movement Speed** | $340.0$ | $+0.0$ | $340.0$ |
| **Attack Range** | $550.0$ | $+0.0$ | $550.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.625$ |
| **Attack Speed Ratio** | $0.625$ |
| **Bonus AS per Level** | $2.2\%$ |
| **Missile Speed** | $1500 units/second$ |
| **Acquisition Radius** | $525 units$ |
| **Gameplay Radius** | $55 units$ |
| **Pathing Radius** | $30 units$ |
| **Selection Radius** | $100 units$ |
| **Selection Height** | $140 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Phenomenal Evil Power

**Innate:** **Veigar** generates a stack of *Phenomenal Evil* for each enemy champion hit by his abilities, and more stacks when he scores an enemy champion takedown.

**Innate:** **Veigar** generates a stack of *Phenomenal Evil* for each enemy champion hit by his abilities, and 5 stacks whenever he scores an enemy champion takedown. **Phenomenal Evil:** For each stack, **Veigar** gains AP.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Self |

**Notes:**

- The two effects stack, granting 6 stacks for each enemy champion taken down by an ability, 7 with *Baleful Strike*.
- ''Veigar's' abilities do not have to deal damage nor affect their targets to grant a stack.

---

### Q: Baleful Strike

**Active:** **Veigar** blasts a dark bolt in the target direction that deals magic damage to the first two enemies hit.

*If this kills an enemy, it generates a stack of **Phenomenal Evil**. Large minions and monsters generate more stacks.*

**Active:** **Veigar** blasts a dark bolt in the target direction that deals magic damage to the first two enemies hit. If this kills an enemy, it generates 1 stack of **Phenomenal Evil**, tripled against large minions and monsters.

| Attribute | Value |
|-----------|-------|
| **Range** | 1050 / er 990 units |
| **Cooldown** | $6-4$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $30-50$ mana |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Speed** | 2200 units/second |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | True |

**Scaling:**
- **Magic Damage:** $80-240$

**Notes:**

- Killing two enemy champions with *Baleful Strike* grants +12 from Phenomenal Evil Power and +2 from Baleful Strike.
- *Baleful Strike* will only grant stacks of *Phenomenal Evil* from its bolt, and not any additional effect that would kill other units from the same cast.
- The stacks are added immediately on enemy kill (on next game tick). **If the first target was killed, the second hit may deal 1 damage more on enemies further along the missile's flight path. Effect at cast time end

---

### W: Dark Matter

**Active:** **Veigar** casts down a mass of dark matter that strikes the target location after a brief delay, dealing magic damage to enemies hit.

*The cooldown is cdr based on stacks of **Phenomenal Evil**.*

**Active:** **Veigar** casts down a mass of dark matter that strikes the target location after a delay, dealing magic damage to enemies hit. 'Dark Matter's* *cooldown* is reduced based on stacks of **Phenomenal Evil*'.

| Attribute | Value |
|-----------|-------|
| **Range** | 950 units |
| **Cooldown** | type=[File:Phenomenal_Evil_Power.pngNumber of times that 50 Phenomenal Evil stacks are earned.' |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $60-80$ mana |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | $240$ units |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |

**Scaling:**
- **Magic Damage:** $85-305$

**Notes:**

- The delay starts at the beginning of the cast time.
  - If **Veigar** dies during the cast time, *Dark Matter* will still successfully fall from the sky.
- *Dark Matter* grants sight of the area during the delay.

---

### E: Event Horizon

**Active:** After a brief delay, **Veigar** erects a cage at the target location that lasts for a short time. Enemies that touch the cage edges are briefly knockdown and stun.

**Active:** **Veigar** forms a cage at the target location that erects after a $0.5$ second delay, remaining there for 3 seconds. Enemies that collide with the edges of the cage are knockdown and stun for a duration. *Event Horizon* can affect enemies only once per cast.

| Attribute | Value |
|-----------|-------|
| **Range** | 725 units |
| **Cooldown** | $20-14$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $70-90$ mana |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Effect Radius** | 411 / 290 / 390 units |
| **Spell Shield** | True |

**Scaling:**
- **Stun Duration:** $1.5-2.5$ seconds

**Notes:**

- The stun and the knockdown are applied to units that are within a certain distance interval from the center of the area of effect.
- An enemy can be affected by *Event Horizon* only once every $4.5$ seconds.
- Units that negate the stun will still count as passing through the edges. They will become immune to the effects of *Event Horizon* for the period even after negating them.
- Displacement immune enemies will still be stun, but not knockdown.
- **Veigar** marks enemy champions in a 390-radius from the zone's center in order to gain [kill/assist credit, lasting for the standard credit timer.
- *Event Horizon* may fail to catch enemies that move very quickly through the boundary distance.
  - More modern area checks such as The Box or Yordle Snap Trap do not have this issue.

---

### R: Primordial Burst

**Active:** **Veigar** sends a primordial burst at the target enemy champion that deals magic damage based on their **missing** health.

**Active:** **Veigar** sends a primordial burst at the target enemy champion that deals magic damage, increased by type=target's **missing** health.

| Attribute | Value |
|-----------|-------|
| **Range** | 650 units |
| **Cooldown** | $100-60$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | 100 mana |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Single target |
| **Projectile** | True |

**Scaling:**
- **Minimum Magic Damage:* $175-325$ (+ $65-75$% AP)2-325×2$ (+ $65×2-75×2$% AP)

**Notes:**

No additional notes.

---

## Patch History

### V25.09
- Veigar
  - **Bug Fixes:** Killing a minion using *Baleful Strike* no longer causes the minion gold bounty's floating text to appear above his head instead of on the minion's death location.

### V14.24
- *Dark Matter*
  - **Bug Fixes:** No longer reveals the user if cast while the cursor is near an enemy champion.
- *Event Horizon*
  - **Bug Fixes:** No longer reveals the user if cast while the cursor is near an enemy champion.

### V14.20
- *Dark Matter*
  - AP ratio reduced to $60-100$% AP from $70-110$% AP.

### V14.9
- Stats
  - Selection radius increased to 100 units from 93.

### V14.5
- *Baleful Strike*
  - AP ratio increased to $50-70$% AP from $45-65$% AP.
- *Primordial Burst*
  - Cooldown reduced to $100-60 3$ seconds from $120-60 3$.

### V14.2
- Stats
  - Base health increased to 580 from 550.

### V13.22
- Stats
  - Basic attack missile speed increased to 1500 from 1100.

### V13.7
- *Baleful Strike*
  - AP ratio changed to $45-65$% AP from 60% AP at all ranks.
- *Dark Matter*
  - AP ratio changed to $70-110$% AP from 100% AP at all ranks.
- *Event Horizon*
  - **Removed:*** Can no longer hide his VFX from enemies by placing it inside certain walls.
  - **Bug Fixes:** Zone placement particles are now properly visible to enemies from the Fog of War.
- *Primordial Burst*
  - AP ratio reduced to $65-75 3$% AP from 75% AP at all ranks.

### V13.6
- Stats
  - Base health reduced to 550 from 575.
  - Base armor reduced to 18 from 21.
- *Dark Matter*
  - Base damage changed to $85-305$ from $100-300$.

### V13.5
- General
  - **Bug Fixes:** Now properly plays his laugh voice line upon scoring a killing blow with *Primordial Burst*.

## Trivia

- 
  - In Veigar's case, *Phenomenal Evil Power* infinitely stacks his ability power.
- When Veigar uses *Primordial Burst* to kill an enemy champion, he will laugh malevolently.
- Veigar possess the most laugh emotes of any champion, at 6.
- Veigar's Series 1 Eternals make the following references:
  - *Ctrl+Alt+Del* is a reference to the keyboard combination that would reboot most early versions of many computers, this would later be changed in Windows 95 where a task manager would be opened instead.

---
*This page was automatically generated from League of Legends Wiki data.*