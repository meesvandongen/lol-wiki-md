# Twitch

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
| **Champion** | Twitch |
| **Title** | the Plague Rat |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2009-05-01 |
| **Release Patch** | May 1, 2009 Patch |
| **Roles** | Marksman |
| **Riot Positions** | Bottom |
| **External Positions** | Bottom, Support |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $630.0$ | $+104.0$ | $2398.0$ |
| **Mana** | $300.0$ | $+40.0$ | $980.0$ |
| **Health Regen** | $3.75$ | $+0.6$ | $13.9$ |
| **Mana Regen** | $7.25$ | $+0.7$ | $19.1$ |
| **Armor** | $27.0$ | $+4.2$ | $98.4$ |
| **Magic Resist** | $30.0$ | $+1.3$ | $52.1$ |
| **Attack Damage** | $59.0$ | $+3.1$ | $111.7$ |
| **Attack Speed** | $0.679$ | $+3.4\%$ | $1.069$ |
| **Movement Speed** | $330.0$ | $+0.0$ | $330.0$ |
| **Attack Range** | $550.0$ | $+0.0$ | $550.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.679$ |
| **Attack Speed Ratio** | $0.679$ |
| **Bonus AS per Level** | $3.4\%$ |
| **Missile Speed** | $2500 units/second$ |
| **Acquisition Radius** | $575 units$ |
| **Pathing Radius** | $35 units$ |
| **Selection Radius** | $135 units$ |
| **Selection Height** | $120 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Deadly Venom

**Innate:** **Twitch**’s basic attacks on-hit apply stacks of *Deadly Venom* for a few seconds, which stacks up to a cap.

**Innate:** ''Twitch's* basic attacks on-hit apply a stack of *Deadly Venom' for 6 seconds, refreshing on subsequent applications and stacking up to 6 times. **Deadly Venom:** For each stack, the target is dealt **total* true damage over the duration,for a maximum of true damage with each tick.6×6 to 5×6×6 for 5 (+ **total** true damage over the duration.This effect is considered a poison.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Enemies |
| **Damage Type** | True |
| **Spell Shield** | False |
| **Spell Effects** | default |

**Notes:**

- The first 5 stacks on a target are indicated each by a small mark around them, while a target affected by the maximum stacks of 6 is indicated by a single large mark above them instead.
- *Deadly Venom* does not affect structures.
- Being applied on-hit, *Deadly Venom* stacks will still be applied if the attack was *parried* or blocked, but not if dodged and/or missed if **Twitch** is blind.

---

### Q: Ambush

**Active:** **Twitch** becomes camouflage for some time, gaining *ms **bonus** movement speed*, increased while facing enemy champions that cannot see him.

*Upon breaking stealth, **Twitch** gains **bonus attack speed** for a few seconds.*

**Active:** After a 1-second delay, **Twitch** becomes camouflage for a duration. Attacking or casting **Venom Cask** or **Contaminate** ends *Ambush* immediately. During this time, **Twitch** gains ms*bonus** movement speed*, increased to 30% while facing enemy champions within a 1000-unit radius who cannot see him. Upon breaking stealth, **Twitch** gains **bonus attack speed** for 6 seconds. When an enemy champion dies while afflicted with **Deadly Venom**, 'Ambush's *cooldown* is reset.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | 16 seconds |
| **Cast Time** | none |
| **Cost** | 40 Mana |
| **Targeting** | Auto |
| **Affects** | Self |

**Scaling:**
- **Stealth Duration:** $10-14$ seconds
- **Bonus Attack Speed:** $45-65$%

**Notes:**

- Entering stealth cancels ''Twitch's' current basic attack.
- *Ambush* follows the same rules as stealth but he can still perform actions normally before entering camouflage. Activating Recall during the 1-second delay allows him to channel it while stealthed.
- If **Twitch** enters stasis during the delay, he will gain the camouflage after the stasis ends.
- Using a basic attack breaks the stealth at the start of the attack windup.

---

### W: Venom Cask

**Active:** **Twitch** tosses a cask that explodes at the target location, inflicting **Deadly Venom** to enemies hit.

*The area then becomes contaminated for a short time, which continually slows enemies and inflicts **Deadly Venom**.*

**Active:** **Twitch** hurls a cask of venom that explodes at the target location, applying **Deadly Venom** to enemies hit and granting sight of the area. The area then becomes contaminated for 3 seconds, applying a **Deadly Venom** stack each second to enemies within and slow them.

| Attribute | Value |
|-----------|-------|
| **Range** | 950 units |
| **Cooldown** | $13-9$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | 70 Mana |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Speed** | 1400 units/second |
| **Effect Radius** | 300 units |
| **Spell Shield** | True |
| **Projectile** | True |

**Scaling:**
- **Slow:** $30-50$% (+ 6% per 100 AP)

**Notes:**

- *Venom Cask* can apply a maximum of 4 *Deadly Venom* stacks per enemy per cast.
- 'Venom Cask's missile will fail to fire if **Twitch** is suppression during the cast time.

---

### E: Contaminate

**Active:** **Twitch** spreads a toxin to nearby enemies afflicted with **Deadly Venom**, dealing physical damage. This deals additional physical and magic damage based on stacks of **Deadly Venom**.

**Active:** **Twitch** sends out a lethal toxin to each nearby enemy afflicted by **Deadly Venom**, dealing them physical damage. *Contaminate* deals additional physical damage and 35% AP magic damage for each stack of **Deadly Venom** on the target. *A nearby enemy with *Deadly Venom* is required to cast this ability. The target does not have to be sight to be targeted by this ability.*

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $12-8$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $50-90$ Mana |
| **Targeting** | Proximity |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Effect Radius** | 1200 units |
| **Spell Shield** | True |
| **Spell Effects** | Aoe |
| **Projectile** | False |

**Scaling:**
- **Base Physical Damage:** $20-60$
- **Physical Damage Per Stack:** $15-35$ (+ 35% bonus AD)bonus AD) (+ 35% AP)6-60+35×6$ (+ 210%
- *bonus AD) (+ $35×6$% AP)

**Notes:**

- *Contaminate* will deal the additional damage to targets based on the number of *Deadly Venom* stacks they had at the start of the cast time.
- **Twitch** is given a range indicator for 'Contaminate's radius upon infecting an enemy champion with *Deadly Venom* (actual range is slightly larger than shown by the indicator).
- *Contaminate* will not deal damage to enemies that are not within range of the ability before the cast time completes.
  - If the target moves out of range after the cast time, they are still dealt the damage.

---

### R: Spray and Pray

**Active:** **Twitch** gains **bonus attack damage** and *range **bonus** attack range* for a few seconds, during which his basic attacks are replaced with bolts that deal the attack's damage to all enemies in a line.

**Active:** **Twitch** gains **bonus attack damage** and range*bonus** attack range* for 6 seconds, during which his basic attack are replaced by *bolts* that travel slightly further than his attack range in a straight line, dealing damage to every enemy unit hit. The *bolts* deal key=% of the triggering attack's damage, apply on-hit effects, and can critically strike for critical damage.

| Attribute | Value |
|-----------|-------|
| **Range** | er Twitch's range |
| **Cooldown** | 90 seconds |
| **Cast Time** | none |
| **Cost** | 100 Mana |
| **Targeting** | Auto |
| **Affects** | Self |
| **Damage Type** | Physical |
| **Speed** | 5000 units/second |
| **Spell Effects** | basic |
| **Projectile** | True |

**Scaling:**
- **Bonus Attack Damage:** $30-60$

**Notes:**

- The extra distance that the *bolts* travel scales with **bonus** attack range.
- *Spray and Pray* allows **Twitch** to outrange turret by 50 units, allowing him to hit them without them returning fire.
- **Twitch** targets his line attack at his target's location at the *start* of his attack windup.
  - The *bolts* reach targets at a delay, composed of ''Twitch's* windup time and distance the *bolt' has to travel at finite speed. Even the primary target can dodge the attack if they are moving quickly enough.
- The *bolt* projectiles will naturally pass through terrain and enemy structures (the latter will be damaged by *bolts* even if they are not directly targeted).
- If **Twitch** is blind before winding up the attack, the hits will *miss* against **all** targets.
- *Runaan's Hurricane* Wind's Fury interacts with 'Spray and Pray's **bonus attack range** but not with the modified missile effect (the secondary bolts will not have pass-through effects).
- Whenever the *bolts* penetrate a target, a small elongated cloud appears at the location, which's VFX and SFX **can** be seen and heard inside the Fog of War.
- The 'bolts** travel distance scales with the full value of **Twitch's' range increases such as *Rapid Firecannon*, but not with increases (which only increase his effective attack range, thereby the target range).
- (Outdated as of V10.13, now can hit everything with edge range, except for turrets) The cr center of a unit must be within the maximum travel distance of the *bolt* missile, and in front of the spawn location (''Twitch's* cr center) for the *bolt' to be able to hit them.
  - Other than this condition, the *bolt* missile has to only touch (pass within its half width of 60 units) the er edge of the unit's radius.
  - This is standard behaviour for *linear skillshots*.
- *Malignance* Hatefog is special cased to work with *Spray and Pray*.
- *Axiom Arcanist* amplifies *bolt* damage as area of effect.

---

## Patch History

### V25.13
- *Ambush*
  - **Bug Fixes:** Starting the charge of Rift Herald Rodeo during 'Ambush's stealth delay no longer causes the monster's model to freeze from enemies' point of view despite moving properly from ''Twitch's' and the allied team's point of view, then suddenly re-appear at the charge's collision location without its movement being visible.
    - 'This bug fix had been noted before on patch

### V25.11
- *Contaminate*
  - AP ratio per stack increased to 35% AP from 30% AP.
    - Maximum AP ratio increased to $35×6$% AP from $30×6$% AP.

### V25.06
- *Contaminate*
  - **Bug Fixes:** If there are multiple targets affected by *Deadly Venom* and the ability is cast shortly before any target's Deadly Venom would expire, no longer always applies the would-be damage from the highest number of Deadly Venom stacks on targets affected by the lowest number of Deadly Venom stacks and vice versa.

### V25.04
- *Spray and Pray*
  - **Bug Fixes:** Now counts as an area of effect spell for *Axiom Arcanist* instead of single-target.
- *Deadly Venom*
  - **Bug Fixes:** Fixed an issue that prevented it from triggering *Runaan's Hurricane* Wind's Fury bolts.
- *Venom Cask*
  - Slow AP ratio increased to 6% per 100 AP from 5% per 100 AP.

### V14.24
- *Ambush*
  - Bonus attack speed increased to $45-65$% from $40-60$%.
  - Attack speed duration increased to 6 seconds from 5.

### V14.12
- *Spray and Pray*
  - **Bug Fixes:** Fixed a bug that caused attacks that tag multiple targets to not receive the damage amp from *Press the Attack*.

### V14.9
- Stats
  - Base health reduced to 630 from 682.
  - Health growth increased to 104 from 100.
- *Venom Cask*
  - Slow AP ratio reduced to 5% per 100 AP from 6% per 100 AP.
- *Spray and Pray*
  - Bonus attack damage reduced to $30-60 3$ from $40-70 3$.

### V14.7
- Twitch, Twitch, Twitch
  - *Ambush*
    - **Bug Fixes:** Camouflage detection indicator is now properly visible when rendered on terrain.

### V14.5
- General
  - **Bug Fixes:** Emote SFX no longer continues to play even after the emote was stopped.

### V14.3
- *Venom Cask*
  - **Bug Fixes:** Corrected the phantom missile's cast timing to properly coincide with the VFX, which would previously break spell shields earlier than expected.
- *Contaminate*
  - **New Effect:** Now calculates its damage based on the number of *Deadly Venom* stacks the target had at the time of cast rather than after the cast time.

## Trivia

- Twitch was voiced.md) by the late Doug Boyd.
  - Twitch is voiced by an unknown voice actor in the Wild Rift Chat shorts.
- *Deadly Venom* displaying an X when fully stacked on a target might be referencing the Black Death (the doors of those afflicted were marked as such).

---
*This page was automatically generated from League of Legends Wiki data.*