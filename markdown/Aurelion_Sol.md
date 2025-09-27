# Aurelion_Sol

## Table of Contents
- [Basic Information](#basic-information)
- [Abilities](#abilities)
- [Patch History](#patch-history)
- [Trivia](#trivia)

## Basic Information

| Attribute | Value |
|-----------|------:|
| **Champion** | Aurelion Sol |

## Abilities

### Passive: Cosmic Creator

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Self |

**INNATE:** **Aurelion Sol**’s damaging abilities against enemies generate him permanent stacks of Stardust, which gradually augments his abilities.
- *Breath of Light*: Bursts deal **bonus** magic damage based on the target's **maximum** health.
- *Astral Flight*: Range is increased equal to $62.5$% Stardust.
- *Singularity*: Outer and inner radius increased equal to 15% Stardust, and the execution threshold is increased.
- *Falling Star/The Skies Descend*: Impact radius increased equal to 15% Stardust.

**Notes:**

- Singularity’s relative and ratio stays the same when affected by *Cosmic Creator*’s modifier.

---

### Q: Breath of Light

| Attribute | Value |
|-----------|------:|
| **Range** | 750 to 920 units |
| **Cast Time** | none |
| **Cost** | 8.75 / 10 / 11.25 / 12.5 / 13.75 Mana per $0.25$ seconds |
| **Cooldown** | 3 seconds |
| **Cooldown Start** | post-effect |
| **Queue Time** | $0.25$ seconds |
| **Targeting** | Direction |
| **Affects** | Self, Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | Special |
| **Spell Effects** | Special |
| **Projectile** | False |
| **Call For Help** | Special |
| **Silence** | True |
| **Initial Cost (Mana cost paid upon starting the channel. Cost for mana per 0.25 seconds then starts being paid after 1.25 seconds into the channel.)** | 30 / 35 / 40 / 45 / 50 Mana |
| **Turn rate** | 180°/s |

**ACTIVE:** **Aurelion Sol** charges for up to $3.25$ seconds to exhale a beam of starfire, during which he can steer the beam in the target direction. The beam collides with the first enemy hit to reveal them and deal magic damage every $0.125$ seconds, reduced to 50% for other surrounding enemies hit by the beam.

| Attribute | Value |
|-----------|------:|
| **Magic Damage per Tick** | 5.625 / 7.5 / 9.375 / 11.25 / 13.125 (+ 6.875% AP) |
| **Reduced Damage per Tick** | 2.8125 / 3.75 / 4.6875 / 5.625 / 6.5625 (+ 3.4375% AP) |

| Attribute | Value |
|-----------|------:|
| **Total Maximum Magic Damage (Ranks 1-4)** | 146.25 / 195 / 243.75 / 292.5 (+ 178.75% AP) |

The beam will deal a burst of **bonus** magic damage for each full second that it damages the same enemy, and generates 2 Stardust if the target is a champion. The damage based on the target's health ratio is capped at 300 against monsters.

| Attribute | Value |
|-----------|------:|
| **Bonus Magic Damage** | 55 / 65 / 75 / 85 / 95 (+ 30% AP)
(+ ($3.1$% Stardust)% of target's **maximum** health |

*Breath of Light* can be recast within the duration, and does so automatically afterwards.

**RECAST:** **Aurelion Sol** ends *Breath of Light* early.

At rank 5, *Breath of Light*’s channel duration is increased to 160 seconds.

*Breath of Light cannot be cast for 1 second if the channel is cancelled within the first $0.25$ seconds.*

**Notes:**

- Applies spell damage on the burst and persistent area damage on the beam.
- *Breath of Light*’s bursts against an enemy champion will aggro nearby enemy minions.
- *Breath of Light* places a timer on each target, which ticks in $0.2$ second intervals. Thus, at 「 5 completed intervals ⟷ 1 full second 」, the burst damage is procced.
  - The timer's visuals themselves otherwise serve no other purpose than gameplay clarity.
  - The timer falls off and resets to zero immediately upon no longer damaging affected targets.
- Spell shield will only block the burst damage.
- The following table refers to interactions while **Aurelion Sol** is charging.

| Aspect | State / Notes |
|--------|---------------|
| **Type** | Charge channel |
| **Attacking** | Interrupts |
| **Movement** | Interrupts |
| **Abilities** | Interrupts |
| **Items** | Interrupts / Allowed / Disabled |
| **Summoner Spells** | Allowed / Interrupts / Disabled |

---

### W: Astral Flight

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.4$ (Initial cast) / None (Recast) |
| **Target Range** | 1500 (+ $7.5$ × Stardust) units |
| **Speed** | 335 + / $167.5$ + units/second |
| **Cost** | 50 / 55 / 60 / 65 / 70 Mana |
| **Cooldown** | 22 / 20.5 / 19 / 17.5 / 16 seconds |
| **Cooldown Start** | post-effect |
| **Queue Time** | $0.35$ (Initial cast) / $0.35$ (Recast) seconds |
| **Targeting** | Direction |
| **Affects** | Self |
| **Grounded** | True |
| **Knockdown** | True |

**ACTIVE:** **Aurelion Sol** dashes in the target direction and resets *Breath of Light’s* cooldown. During flight, he has unobstructed vision and *Breath of Light* has no cooldown and maximum channel duration and its flat damage is increased, but *Astral Flight*’s dash speed is reduced by 50% during its channel.

| Attribute | Value |
|-----------|------:|
| **Breath of Light Flat Damage Modifier** | 108 / 109 / 110 / 111 / 112% |

***Aurelion Sol** will be knocked down by any immobilizing crowd control during the dash.*

*Astral Flight* can be recast after $0.5$ seconds during the dash, and does so automatically upon arrival.

**RECAST:** **Aurelion Sol** ends *Astral Flight*.

Scoring a champion takedown within 3 seconds of damaging them reduces *Astral Flight*’s **current** cooldown by 90% of its **total** cooldown.

***Aurelion Sol** will not dash if he is immobilized or grounded during the cast time. He can cast any of his abilities during the dash. Breath of Light’s channel will be interrupted if Astral Flight ends or Singularity is cast during the flight, but the ability will automatically be cast afterwards.*

**Notes:**

- If a takedown is scored while *Astral Flight* is active, the cooldown will be reduced after it ends.
- *Astral Flight* cannot be used in the first 15 seconds of the game.
- The following table refers for interactions while **Aurelion Sol** is dashing:

| Aspect | State / Notes |
|--------|---------------|
| **Attacking** | Interrupts |
| **Movement** | Interrupts |
| **Abilities** | Allowed |
| **Items** | Allowed / Interrupts |
| **Summoner Spells** | Allowed / Interrupts / Disabled |
| **Consumables** | Usable |
| **Interrupted by** | death,root |

---

### E: Singularity

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.2$ seconds |
| **Target Range** | 750 to 920 units |
| **Effect Radius** | √(275 (Radius at 0 Stardust)² + $16.93$² × Stardust) units |
| **Inner Radius** | √($137.5$ (Radius at 0 Stardust)² + $8.46$² × Stardust) units |
| **Cost** | 90 Mana |
| **Cooldown** | 12 seconds |
| **Cooldown Start** | post-effect |
| **Queue Time** | $0.35$ seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | Special |
| **Spell Effects** | aoedot |
| **Out of Range Behavior** | Target at maximum range (clamped) if cast within 200 units beyond it or during *Astral Flight*. Otherwise, walk in range of the target location to cast. |
| **Call For Help** | Special |

**ACTIVE:** **Aurelion Sol** conjures a black hole at the target location after a $0.5$-second delay (From start of cast time) that lasts for 5 seconds, granting sight of the area and dealing magic damage every $0.25$ seconds to enemies within.

| Attribute | Value |
|-----------|------:|
| **Magic Damage per Tick** | 2.5 / 3.75 / 5 / 6.25 / 7.5 (+ 3% AP) |
| **Total Magic Damage** | 50 / 75 / 100 / 125 / 150 (+ 60% AP) |

Enemies in the black hole are dragged inward until they reach the center, and minions and monsters also have their movement speed **reduced** to 0. Enemies within the center are executed below 5% (+ $2.6$% Stardust)% of their **maximum** health, excluding epic monsters.

After *Singularity* ends, it generates 1 Stardust for each full second that enemy champions were inside it, as well as varying amounts for enemy units that were killed while being within the area:
- Champions and epic monsters grant 2 Stardust.
- Large minions and monsters grant 2 Stardust.
- Small minions and monsters grant 1 Stardust.

**Notes:**

- The damage area hits targets by their edge, while the kinematics area hits targets by their center. This makes a slight fringe where targets may be damaged without displacing them.
- Spell shield will block a single instance of damage.
- *Singularity*’s execute against an enemy champion will aggro nearby enemy minions.

---

### R: Falling Star

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 1250 units |
| **Effect Radius** | √(275 (Radius at 0 Stardust)² + $16.93$² × Stardust) units |
| **Cost** | 100 Mana |
| **Cooldown** | 120 / 115 / 110 / 105 / 100 seconds |
| **Queue Time** | $0.35$ seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | False |
| **Out of Range Behavior** | Target at maximum range (clamped) if cast within 200 units beyond it or during *Astral Flight*. Otherwise, walk in range of the target location to cast. |
| **Call For Help** | True |

**ACTIVE:** **Aurelion Sol** calls down a star that strikes the target location after $1.25$ seconds, dealing magic damage to enemies hit and stunning them for 1 second. This generates 5 Stardust for each enemy champion hit.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 150 / 200 / 250 / 300 / 350 (+ 75% AP) |

Once *Falling Star* has been learned, gathering 75 Stardust causes the next cast of *Falling Star* to transform into *The Skies Descend*, empowering the impact with new effects.

**Notes:**

- **Aurelion Sol** will transform *Falling Star* into *The Skies Descend* each time he gathers 75 Stardust from the last time he used the empowered cast, or once he has gathered that amount after learning the ability.
- Stardust required for transformation is denoted by a progress bar over *Falling Star* in the HUD.

---

### R: The Skies Descend

| Attribute | Value |
|-----------|------:|
| **Effect Radius** | 5000 (Shockwave radius, pending for test) units |
| **Inner Radius** | √($388.91$ (Radius at 0 Stardust)² + $21.85$² × Stardust) units |
| **Queue Time** | $0.35$ seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | False |
| **Out of Range Behavior** | Target at maximum range (clamped) if cast within 200 units beyond it or during *Astral Flight*. Otherwise, walk in range of the target location to cast. |

**ACTIVE:** **Aurelion Sol** calls down a giant star that strikes the target location after 2 seconds, dealing 25% increased damage in a larger area and knocking up enemies hit for 1 second.

| Attribute | Value |
|-----------|------:|
| **Empowered Magic Damage** | 187.5 / 250 / 312.5 / 375 / 437.5 (+ 93.75% AP) |

Additionally, the impact sends a massive shockwave that rapidly expands from the area over 3 seconds, dealing magic damage to enemy champions and epic monsters hit, slowing all enemies hit by 50% for 1 second, and revealing them for $1.5$ seconds.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 135 / 180 / 225 / 270 / 315 (+ 67.5% AP) |

*Enemies hit by the star are immune to the shockwave.*

**Notes:**

- The shockwave radius and path are globally visible on the minimap.
- The shockwave's radius is fixed.
  - The shockwave is not blocked.

---

## Patch History

### V25.12
- Singularity
  - **Bug Fixes:** Execute threshold indicator on targets based on Cosmic Creator stacks while the singularity is damaging them and he is simultaneously earning stacks from another damaging ability on them no longer causes the display to update to an incorrect health percentage due to not considering the targets' health bonuses, and no longer requires the player to earn at least one more Cosmic Creator stack from another damaging ability in order for the affected targets' indicator to update to the correct threshold.
    - *[Note: Actual threshold was not affected.]*

### V14.21
- Stats
  - Base health reduced to 600 from 620.
- Singularity
  - AP ratio per tick reduced to 3% AP from 4% AP.
    - Total AP ratio reduced to 60% AP from 80% AP.

### V14.15
- Breath of Light
  - Initial mana cost increased to 35 / 40 / 45 / 50 / 55 from 30 / 35 / 40 / 45 / 50.
  - Mana cost per $0.25$ seconds increased to 8.75 / 10 / 11.25 / 12.5 / 13.75 from 7.5 / 8.75 / 10 / 11.25 / 12.5.
- Astral Flight
  - Base range reduced to 1500 units at all ranks from 1500 / 1600 / 1700 / 1800 / 1900.
- Singularity
  - Mana cost increased to 90 from 80.

### V14.14
- Stats
  - Base attack damage increased to 58 from 55.
- Breath of Light
  - Initial mana cost increased to 30 / 35 / 40 / 45 / 50 from 7.5 / 8.75 / 10 / 11.25 / 12.5.
  - Mana cost interval rate reduced to $0.25$ seconds from $0.5$.
    - Mana cost per interval reduced to 7.5 / 8.75 / 10 / 11.25 / 12.5 from 15 / 17.5 / 20 / 22.5 / 25.
  - Initial delay for mana cost per interval increased to $1.25$ seconds from $0.5$.
- Astral Flight
  - Base range increased to 1500 / 1600 / 1700 / 1800 / 1900 from 1200 at all ranks.
- Singularity
  - **Removed:*** Champion-summoned units classified as minions no longer have their movement speed reduced to 0 while within the area.
  - Mana cost reduced to 80 at all ranks from 80 / 85 / 90 / 95 / 100.
- Falling Star
  - AP ratio increased to 75% AP from 65% AP.
- The Skies Descend
  - Star AP ratio increased to 93.75% AP from 81.25% AP.
  - Shockwave damage reduced to 90% of Falling Star’s damage from 100%.
    - Shockwave base damage reduced to 135 / 225 / 315 from 150 / 250 / 350.
    - Shockwave AP ratio increased to 67.5% AP from 65% AP.

### V14.9
- Stats
  - Gameplay radius increased to 80 units from 65.
  - Selection radius reduced to 150 units from 165.
- Singularity
  - AP ratio per tick reduced to 4% AP from 5% AP.
    - Total AP ratio reduced to 80% AP from 100% AP.

### V14.7#April 9th Hotfix|V14.7
- Breath of Light
  - **Bug Fixes:** Burst now properly draws minion call for help signals.
  - **Bug Fixes:** Beam now correctly respects hitboxes; it properly is stopped by the first enemy hit.

### V14.7
- Aurelion Sol
  - The Skies Descend
    - **Bug Fixes:** VFX is now properly aligned for both teams.

### V14.4
- Breath of Light
  - Burst base damage reduced to 55 / 65 / 75 / 85 / 95 from 60 / 70 / 80 / 90 / 100.
  - Burst AP ratio reduced to 30% AP from 35% AP.

### V14.3#February 7th Hotfix|V14.3
- Breath of Light
  - Stardust stack per champion burst reduced to 2 from 3.
- Astral Flight
  - Cooldown increased to 22 / 20.5 / 19 / 17.5 / 16 from 15 / 14 / 13 / 12 / 11.
  - Breath of Light bonus damage reduced to 8 / 9 / 10 / 11 / 12% from 18 / 20 / 22 / 24 / 26%.

### V14.3
- Cosmic Creator
  - **Bug Fixes:** No longer can acquire Stardust stacks from Voidmite.
- Breath of Light
  - Mana cost per second reduced to 30 / 35 / 40 / 45 / 50 from 45 / 50 / 55 / 60 / 65.
  - Stardust stack per champion burst increased to 3 from 1.
- Astral Flight
  - Mana cost reduced to 50 / 55 / 60 / 65 / 70 from 80 / 85 / 90 / 95 / 100.
  - Cooldown reduced to 15 / 14 / 13 / 12 / 11 seconds from 22 / 20.5 / 19 / 17.5 / 16.
  - Breath of Light bonus damage increased to 18 / 20 / 22 / 24 / 26% from 14 / 15.5 / 17 / 18.5 / 20%.
  - **Bug Fixes:** No longer plays his respawn animation but sped up when crowd control interrupted this ability.
- Singularity
  - Stardust stacks per champion and epic monster takedown reduced to 2 from 5.
  - Stardust stacks per large minion and monster takedown reduced to 2 from 3.

## Trivia

- 
  - In Aurelion Sol's case, Stardust stacks gained through I permanently increases Q's bonus magic damage, W's range, E's outer and inner radius and execution threshold and R and The Skies Descend's effect radius.
- The Skies Descend was named after his champion spell of the same The Skies Descend in Legends of Runeterra.
- In 'Aurelion Sol: The Star Forger Returns' his hand gesture resembles the old Riot Games Inc. fist intro.
- A lot of his animations and visual effects were inspired by Astronomical object and Atom.
- Aurelion Sol's lower body is not part of his in-game hitbox, which is why it is semi-transparent.
  - A demonstration can be seen here.
- Prior to his CGU, in Ultra Rapid Fire his Center of the Universe passive used to have 6 stars orbiting around him with increased speed, simply by leveling up.
  - Interestingly, this is the only non-stat modification made to any champion in URF, suggesting this may have originally been an intended feature which was scrapped from the champion before release.
- The name of the concept character from which Aurelion Sol spurred from, ​​​​​​Ao Shin, could be a reference to Dragon_King#Red_Dragon, the *Dragon King of the South China Sea*.
- Aurelion Sol's Series 1 and 2 Eternals make the following references:
  - *Spaghettification* is a reference to the homonym Spaghettification in astrophysics that refers to objects becoming vertically stretched and horizontally compressed in very strong gravitational fields, such as black holes.
  - *Flying in the 90s* is a reference to the song Running in the 90s by italian singer .
- Aurelion Sol's temporary internal name was "Heaven's Coil" which was chosen to intentionally be a "god-awful temp name."

---
*This page was automatically generated from League of Legends Wiki data.*