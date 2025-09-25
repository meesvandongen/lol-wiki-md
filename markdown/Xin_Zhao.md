# Xin_Zhao

## Table of Contents
- [Basic Information](#basic-information)
- [Abilities](#abilities)
- [Patch History](#patch-history)
- [Trivia](#trivia)

## Basic Information

| Attribute | Value |
|-----------|------:|
| **Champion** | Xin Zhao |

## Abilities

### Passive: Determination

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Self |
| **Damage Type** | Physical |
| **Spell Shield** | Special |
| **Spell Effects** | proc |
| **Parry** | unknown |

**INNATE:** **Xin Zhao**’s basic attacks on-hit and *Wind Becomes Lightning’s* first slash hit and thrust on at least one enemy hit each generate a stack of *Determination*, stacking up to 3 times. The third stack consumes them all to deal 15%–60%@1–16 AD **bonus** physical damage on-attack and heal **Xin Zhao** for 3 to 4 of his **maximum** health (+ 65% AP) on-hit.

**INNATE - CHALLENGE:** *Wind Becomes Lightning* and, if *Crescent Guard* has been learned, **Xin Zhao**’s basic attacks and *Audacious Charge*, apply a Challenged mark to certain enemies hit for 3 seconds. Only one enemy can be marked as Challenged at a time; applying the mark to a new target removes it from the previous one. *Audacious Charge* and *Crescent Guard* have interactions against Challenged targets.

**Notes:**

;Determination Details
- The *healing* and **bonus** damage work on different stacking systems, each counting its stacks and being consumed separately. While both stack on-hit, the **bonus** damage stacks are consumed on-attack, and the healing ones on-hit.
  - The *healing* stacks are denoted in the buff-bar.
  - If Guinsoo's Rageblade were to trigger when the stacks are at maximum, they will grant the healing but not the **bonus** damage, thus causing an out-of-sync between the stacks shown in-game versus when the **bonus** damage is dealt.
- Only the initial slash of Wind Becomes Lightning can generate a *Determination* stack. Slashes after the first do not count for stack generation.
- The stack counter displays 1–3 rather than the usual 0–2, meaning that a count of 1 does not translate to having 1 stack.
- Stacks reset when **Xin Zhao** dies.
- Stacks of *Determination* will be gained even if the attack is blocked.
- : *Determination*’s interaction with *parrying* effects (dodge, blind).
- *Determination* deals proc damage.
  - Since it is a separate instance of damage when applied by basic attack or either of W’s strikes, flat damage reduction that applies to both the main and bonus damage such as Tantrum’s passive will reduce each, for twice the reduction.
  - The bonus damage cannot critically strike.
- When applied by a basic attack:
  - The attack uses a unique animation.
  - The bonus damage benefits from life steal.
    - It applies life steal specifically.
  - Neither the bonus damage or heal will be blocked by spell shield.
- When applied by W:
  - Triggers against the closest target struck by the first slash/thrust (equals first target to receive spell effects).
  - Does not benefit from life steal, including not healing from *Wind Becomes Lightning*’s 33% conversion for this bonus damage.
  - Since proc damage is 'single target', Vamp will heal from 100% of the bonus damage.
  - The bonus damage will be blocked by spell shield, the heal will not. ;Challenged Details
- Spell shield will block the mark application from W, but not from basic attacks and E.

---

### Q: Three Talon Strike

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Cost** | 30 Mana |
| **Cooldown** | 7 / 6.5 / 6 / 5.5 / 5 seconds |
| **Cooldown Start** | post-effect |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | Special |
| **Spell Effects** | spell |
| **Parry** | True |

**ACTIVE:** **Xin Zhao** empowers his next three basic attacks on-attack within 5 seconds to each have an uncancellable windup, deal **bonus** physical damage and reduce his other abilities' **current** cooldowns by 1 second. Each attack refreshes the duration.

| Attribute | Value |
|-----------|------:|
| **Bonus Physical Damage** | 15 / 30 / 45 / 60 / 75 (+ 40% **bonus** AD) |
| **Total Bonus Physical Damage** | 45 / 90 / 135 / 180 / 225 (+ 120% **bonus** AD) |

The third attack knocks up the target for $0.75$ seconds.

*Three Talon Strike*’s **bonus** damage is affected by critical strike modifiers.

*Three Talon Strike resets **Xin Zhao**’s basic attack timer.*

**Notes:**

- Spell shield will only block the knock up.
- The enhanced attacks present input buffering during their wind-up animation (**Xin Zhao** cannot be issued any commands until after it ends).

---

### W: Wind Becomes Lightning

| Attribute | Value |
|-----------|------:|
| **Range** | 1000 (Thrust range) / er 940 (Maximum possible range; See notes) units |
| **Cast Time** | $0.5$ seconds |
| **Effect Radius** | 275 (Slash semi-circle radius) / cr 125 (Slash and thrust extra check radius) units |
| **Angle** | er 160º |
| **Width** | 80 (Thrust missile width) units |
| **Speed** | 6250 (Thrust missile speed) units/second |
| **Cost** | 60 / 55 / 50 / 45 / 40 Mana |
| **Cooldown** | 12 / 11 / 10 / 9 / 8 seconds |
| **Queue Time** | $0.3$ seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |
| **Projectile** | False |

**ACTIVE:** **Xin Zhao** unleashes 4 slashes (First slash happens on-cast instantly, while the remaining 3 are applied every 0.05 seconds thereafter) in an arc over the first $0.15$ seconds of the cast time, each dealing physical damage to enemies within. After the remaining cast time, he then thrusts his spear in a line in the target direction, dealing physical damage to enemies hit, increased by 0%–33.3%@0–100 (@=critical strike chance), and slowing them by 50% for $1.5$ seconds. *Wind Becomes Lightning* deals 50% / 53.33% / 56.67% / 60% / 63.33% / 66.67% / 70% / 73.33% / 76.67% / 80% / 83.33% / 86.67% / 90% / 93.33% / 96.67% / 100% damage to minions.

The farthest champion or large monster hit within the thrust's area is also marked as *Challenged* and is revealed for 3 seconds.

| Attribute | Value |
|-----------|------:|
| **Physical Damage per Slash** | 7.5 / 10 / 12.5 / 15 / 17.5 (+ 7.5% AD) |
| **Slash Total Physical Damage** | 30 / 40 / 50 / 60 / 70 (+ 30% AD) |

| Attribute | Value |
|-----------|------:|
| **Thrust Physical Damage** | 50 / 85 / 120 / 155 / 190 (+ 90% AD) (+ 65% AP) |

| Attribute | Value |
|-----------|------:|
| **Total Physical Damage** | 80 / 125 / 170 / 215 / 260 (+ 120% AD) (+ 65% AP) |

*Wind Becomes Lightning*’s damage heals **Xin Zhao** for $33.3$% of his life steal.

*The durations of Three Talon Strike and Audacious Charge’s **bonus** attack speed are delayed relative to Wind Becomes Lightning's cast time. 
**Xin Zhao** reveals himself during the cast if there is an enemy champion nearby (see details)*.

**Notes:**

- The self-reveal has not been fully tested for complete confirmation. Is the ability invisible when hitting minions and there is no enemy champion nearby? Is the tip of the ability enough to reveal him? Whether it can hit a minion or not? Just if he's around the exit of the fog of war, and even if he points towards the inside of the fog of war? [https://youtu.be/GvLedSeuQ0s]
- Spell shield will block both the slashes and the thrust. - This ability will cast from wherever the caster is at the end of the cast time.
  - The thrust will occur from wherever **Xin Zhao** is at the end of the cast time.
  - The slashes will occur from wherever **Xin Zhao** is at the time during the first $0.15$ seconds.
- The first strike's hitbox includes a er 275 radius 160° cone in front of **Xin Zhao** and an additional cr 125 radius around him.
- The second strike's hitbox includes the same cr 125 radius around **Xin Zhao**, a missile with cr 1000 range and er 80 total width, and a er -60 radius area check upon the end of the missile.
  - The 125 radius check does not hit champion summoned units.
- Because *Wind Becomes Lightning* uses an older type of 'healing based on life steal' and not *applies life steal*, Spirit Visage will amplify the life gain twice (+25% life steal and +25% healing), for a total of +$56.25$%.
- Each of *Wind Becomes Lightning*’s slashes are applied in a separate damage instance from each other.
  - This causes effects like Bone Plating and Black Cleaver Carve to be applied multiple times.

---

### E: Audacious Charge

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 650 (Normal range) / 1100 (Enhanced range) units |
| **Effect Radius** | 250 (Damage radius) units |
| **Speed** | 2500 units/second |
| **Cost** | 50 Mana |
| **Cooldown** | 11 seconds |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |
| **Call For Help** | True |
| **Grounded** | True |
| **Knockdown** | True |

**ACTIVE:** **Xin Zhao** dashes to the target enemy's location, dealing magic damage to enemies near them and slowing them by 30% for $0.5$ seconds. *Audacious Charge*’s range is increased against *Challenged* targets.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 50 / 75 / 100 / 125 / 150 (+ 60% AP) |

**Xin Zhao** then gains **bonus** attack speed for 5 seconds.

| Attribute | Value |
|-----------|------:|
| **Bonus Attack Speed** | 40 / 45 / 50 / 55 / 60% |

*Three Talon Strike and Crescent Guard can be cast during the dash.*

**Notes:**

- The primary target cannot dodge *Audacious Charge*’s effects.
  - The target will always emit the area of effect from their position.
- *Audacious Charge* has a forgiveness radius of 200 units.

---

### R: Crescent Guard

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.35$ seconds |
| **Effect Radius** | 500 (Knockback radius) / 450 (Immunity minimum radius) units |
| **Cost** | 100 Mana |
| **Cooldown** | 120 / 115 / 110 / 105 / 100 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | False |

**PASSIVE:** **Xin Zhao**’s basic attacks and *Audacious Charge* apply the *Challenged* mark to enemy champions hit, with the latter applying the mark to the farthest champion hit in the area of effect or the primary target if they are a champion.

**ACTIVE:** **Xin Zhao** sweeps his spear around him, dealing physical damage to nearby enemies, capped at 600 against minions and monsters, and knocking back all targets hit that are not *Challenged* up-to 700 units over $0.75$ seconds, as well as stunning them for the same duration.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 75 / 125 / 175 / 225 / 275 (+ 100% **bonus** AD) (+ 110% AP) (+ 15% of target's **current** health) |

For the next 4 seconds, **Xin Zhao** is invulnerable against enemy champions far away from him.

**Notes:**

- Pets, traps, ground AoEs, DoTs, and delayed-damage abilities (e.g. Death Mark, Explosive Charge, Hemoplague) will not damage **Xin Zhao** if the enemy champion source is outside the circle when the damage applies. - This ability will cast from wherever the caster is at the end of the cast time.
- Displacement immunity will also resist the application of the stun.

---

## Patch History

### V25.18
- Three Talon Strike
  - **Bug Fixes:** When used during Audacious Charge’s active buff, if he is displaced during the dash and collides with another champion (ally or enemy), *Three Talon Strike*’s active VFX no longer lingers at the displacement's end location.
- Wind Becomes Lightning
  - **Bug Fixes:** No longer illegally causes more slashes than intended if used in combination with Flash.
- Crescent Guard
  - **Bug Fixes:** Invulnerability shield VFX is now properly visible when an enemy **Xin Zhao** moves in and out of an allied Mordekaiser’s Realm of Death.

### V25.17
- General
  - Updated ability icons.
  - Complete visual update across all skins.
    - New splash artwork for Xin Zhao, Xin Zhao, Xin Zhao, and Xin Zhao.
    - Adjusted splash artwork for Xin Zhao, Xin Zhao, Xin Zhao, Xin Zhao, and Xin Zhao.
  - New voice-over.
  - Updated sound effects.
  - Xin Zhao cost increased to from .
  - Xin Zhao cost increased to from .
  - All Legacy skins will be unvaulted temporarily for the duration of the 25.17 and 25.18 patch cycles.
- Wind Becomes Lightning
  - **New Effect:** Now causes him to slash four times rather than only once, slashing once on-cast and then 3 times over the next $0.15$ seconds.
    - Total slash damage is unchanged.
    - Only the initial slash can generate a Determination stack.
- Audacious Charge
  - **Bug Fixes:** No longer fails to cast when used on a target outside of its maximum target range during Crescent Guard’s cast time.
- Crescent Guard
  - **Bug Fixes:** Tooltip now correctly mentions that the damage cap also applies to minions and not only monsters.

### V25.16
- Stats
  - Base health reduced to 620 from 640.
- Three Talon Strike
  - Base damage per hit reduced to 15 / 30 / 45 / 60 / 75 from 20 / 35 / 50 / 65 / 80.
    - Total base damage reduced to 45 / 90 / 135 / 180 / 225 from 60 / 105 / 150 / 195 / 240.

### V25.14
- Wind Becomes Lightning
  - **Bug Fixes:** Increased Audacious Charge dash range indicator now properly does not appear when the ability is blocked by spell shield.

### V25.10
- Stats
  - Armor growth reduced to $4.4$ from 5.

### V25.06
- Three Talon Strike
  - Base damage per hit increased to 20 / 35 / 50 / 65 / 80 from 16 / 29 / 42 / 55 / 68.

### V14.24
- Wind Becomes Lightning
  - **Bug Fixes:** No longer reveals the user if cast while the cursor is near an enemy champion.
    - Note: the ability will continue to self-reveal if there is an enemy champion nearby.

### V14.18
- Stats
  - Armor growth increased to 5 from $4.7$.
- Three Talon Strike
  - Base damage increased to 16 / 29 / 42 / 55 / 68 from 16 / 25 / 34 / 43 / 52.
    - Total base damage increased to 48 / 87 / 126 / 165 / 204 from 48 / 75 / 102 / 129 / 156.

### V14.17
- Crescent Guard
  - **Bug Fixes:** Invulnerability now properly functions when the ability is cast while inside Mordekaiser’s Realm of Death.

### V14.5
- Three Talon Strike
  - **New Effect:** Now triggers spell effects upon dealing damage.

## Trivia

- Xin Zhao was released when Season One started.
- Xin Zhao's dance references dance from .
  - A side-by-side comparison can be seen here.
- During development, he was called *XenZiou* or *XenZhao*.

---
*This page was automatically generated from League of Legends Wiki data.*