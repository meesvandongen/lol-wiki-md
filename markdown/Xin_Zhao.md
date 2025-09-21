# Xin_Zhao

## Table of Contents
- [Basic Information](#basic-information)
- [Abilities](#abilities)
- [Patch History](#patch-history)
- [Trivia](#trivia)

## Basic Information

| Attribute | Value |
|-----------|-------|
| **Champion** | Xin Zhao |

## Abilities

### Passive: Determination

**Innate:** **Xin Zhao**’s basic attacks on-hit and **Wind Becomes Lightning** strikes generate a stack of *Determination*, stacking up to 3 times.

*Upon generating the final stack, **Xin Zhao** consumes them to heal himself and deal **bonus** physical damage.*

**Innate:** ''Xin Zhao's* basic attacks on-hit and **Wind Becomes Lightning*’s* first slash hit and thrust on at least one enemy hit each generate a stack of *Determination', stacking up to 3 times. The third stack consumes them all to deal 15–60 AD **bonus** physical damage on-attack and heal **Xin Zhao** for 3 to 4 of his **maximum** health (+ 65% AP) on-hit. **Innate - Challenge:** **Wind Becomes Lightning** and, if **Crescent Guard** has been learned, ''Xin Zhao's* basic attacks and **Audacious Charge**, apply a Challenged mark to certain enemies hit for 3 seconds. Only one enemy can be marked as Challenged at a time; applying the mark to a new target removes it from the previous one. **Audacious Charge** and **Crescent Guard*' have interactions against Challenged targets.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Self |
| **Damage Type** | Physical |
| **Spell Shield** | Special |
| **Spell Effects** | proc |

**Notes:**

- The *healing* and **bonus** damage work on different stacking systems, each counting its stacks and being consumed separately. While both stack on-hit, the **bonus** damage stacks are consumed on-attack, and the healing ones on-hit.
  - The *healing* stacks are denoted in the buff-bar.
  - If *Guinsoo's Rageblade* were to trigger when the stacks are at maximum, they will grant the healing but not the **bonus** damage, thus causing an out-of-sync between the stacks shown in-game versus when the **bonus** damage is dealt.
- Only the initial slash of *Wind Becomes Lightning* can generate a *Determination* stack. Slashes after the first do not count for stack generation.
- The stack counter displays 1–3 rather than the usual 0–2, meaning that a count of 1 does not translate to having 1 stack.
- Stacks reset when **Xin Zhao** death.
- Stacks of *Determination* will be gained even if the attack is blocked.
- : 'Determination's* interaction with *parrying' effects (dodge, blind).
- *Determination* deals proc damage.
  - Since it is a separate instance of damage when applied by basic attack or either of *W*’s strikes, flat damage reduction that applies to both the main and bonus damage such as passive will reduce each, for twice the reduction.
  - The bonus damage cannot critical strike.
- When applied by a basic attack:
  - The attack uses a unique animation.
  - The bonus damage benefits from . *** It applies life steal specifically.
  - Neither the bonus damage or heal will be blocked by spell shield.
- When applied by *W*:
  - Triggers against the closest target struck by the first slash/thrust (equals first target to receive spell effects).
  - Does not benefit from *life steal*, including not healing from 'Wind Becomes Lightning's 33% conversion for this bonus damage.
  - Since proc damage is 'single target', drain will heal from 100% of the bonus damage.
  - The bonus damage will be blocked by spell shield, the heal will not. ;Challenged Details
- Spell shield will block the mark application from *W*, but not from basic attacks and *E*. *

---

### Q: Three Talon Strike

**Active:** **Xin Zhao** empowers his next few basic attacks within a duration to deal **bonus** physical damage and ah his other ability cooldowns.

*The last empowered attack will also briefly airborne the target.*

**Active:** **Xin Zhao** empowers his next three basic attacks on-attack within 5 seconds to each have an uncancellable windup, deal **bonus** physical damage and reduce his other ability **current cooldowns** by 1 second. Each attack refreshes the duration. The third attack airborne the target for $0.75$ seconds. 'Three Talon Strike's **bonus** damage is affected by critical strike modifiers. *Three Talon Strike basic attack reset *'Xin Zhao's* basic attack timer.*

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $7-5$ seconds |
| **Cast Time** | none |
| **Cost** | 30 Mana |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | Special |
| **Spell Effects** | spell |

**Scaling:**
- **Bonus Physical Damage:** $15-75$ (+ 40% bonus AD)3-75×3$ (+ 120% bonus AD)

**Notes:**

- Spell shield will only block the knock up.
- The enhanced attacks present input buffering during their wind-up animation (**Xin Zhao** cannot be issued any commands until after it ends).

---

### W: Wind Becomes Lightning

**Active:** **Xin Zhao** slashes in an arc, then thrusts his spear in a line in the target direction. Each strike deals physical damage to enemies hit and heals **Xin Zhao** based on his .

*The thrust deals increased damage based on and briefly slows enemies hit, enemy champions or monsters hit are also standard sight and marked Challenged for a short time.*

**Active:** **Xin Zhao** unleashes 4 slashes in an arc over the first $0.15$ seconds of the cast time, each dealing physical damage to enemies within. After the remaining cast time, he then thrusts his spear in a line in the target direction, dealing physical damage to enemies hit, increased by key=%, and slow them by 50% for $1.5$ seconds. *Wind Becomes Lightning* deals 50–100 damage to minions. The farthest champion or large monster hit within the thrust's area is also marked as **border=false** and is standard sight for 3 seconds. 'Wind Becomes Lightning's damage healing **Xin Zhao** for $33.3$% of his . 'The durations of *Three Talon Strike* and *Audacious Charge*’s *bonus attack speed are delayed relative to Wind Becomes Lightning's cast time. **Xin Zhao** sight himself during the cast if there is an enemy champion nearby'.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $12-8$ seconds |
| **Cast Time** | $0.5$ seconds |
| **Cost** | $60-40$ Mana |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Speed** | 6250 units/second |
| **Effect Radius** | 275 / cr 125 units |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |
| **Projectile** | False |

**Scaling:**
- ** $50-190$ (+ 90% AD) (+ 65% AP) ** Total Physical Damage:** $30+50-70+190$ (+ 120% AD) (+ 65% AP)

**Notes:**

- The self-reveal has not been fully tested for complete confirmation. Is the ability invisible when hitting minions and there is no enemy champion nearby? Is the tip of the ability enough to reveal him? Whether it can hit a minion or not? Just if he's around the exit of the fog of war, and even if he points towards the inside of the fog of war? [https://youtu.be/GvLedSeuQ0s]
- Spell shield will block both the slashes and the thrust. Effect at cast time end
  - The thrust will occur from wherever **Xin Zhao** is at the end of the cast time.
  - The slashes will occur from wherever **Xin Zhao** is at the time during the first $0.15$ seconds.
- The first strike's hitbox includes a er 275 radius 160° cone in front of **Xin Zhao** and an additional cr 125 radius around him.
- The second strike's hitbox includes the same cr 125 radius around **Xin Zhao**, a missile with cr 1000 range and er 80 total width, and a er -60 radius area check upon the end of the missile.
  - The 125 radius check does not hit champion summoned units.
- Because *Wind Becomes Lightning* uses an older type of 'healing based on life steal' and not *applies life steal*, *Spirit Visage* will amplify the life gain twice (+25% life steal and +25% healing), for a total of +$56.25$%.
- Each of 'Wind Becomes Lightning's slashes are applied in a separate damage instance from each other.
  - This causes effects like *Bone Plating* and *Black Cleaver* Carve to be applied multiple times.

---

### E: Audacious Charge

**Active:** **Xin Zhao** dashes to the target enemy, dealing magic damage to nearby enemies and briefly slow them. He then gains **bonus attack speed** for a few seconds.

*'Audacious Charge's *range* is increased against a Challenged target.*

**Active:** **Xin Zhao** dashes to the target enemy's location, dealing magic damage to enemies near them and slow them by 30% for $0.5$ seconds. 'Audacious Charge's* range is increased against **border=false*' targets. **Xin Zhao** then gains **bonus attack speed** for 5 seconds. **Three Talon Strike* and *Crescent Guard* can be cast during the dash.*

| Attribute | Value |
|-----------|-------|
| **Range** | 650 / 1100 units |
| **Cooldown** | 11 seconds |
| **Cast Time** | none |
| **Cost** | 50 Mana |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Speed** | 2500 units/second |
| **Effect Radius** | 250 units |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |

**Scaling:**
- **Magic Damage:** $50-150$ (+ 60% AP)
- **Bonus Attack Speed:** $40-60$%

**Notes:**

- The primary target cannot dodge 'Audacious Charge's effects.
  - The target will always emit the area of effect from their position.
- *Audacious Charge* has a forgiveness radius of 200 units.

---

### R: Crescent Guard

**Passive - Challenge:** The last enemy champion hit by **Xin Zhao**’s basic attacks or **Audacious Charge** is marked Challenged for a short time.

**Active:** **Xin Zhao** sweeps his spear to deal physical damage to nearby enemies based on their **current** health, this will also briefly airborne and stun those not marked Challenged.

**Passive:** ''Xin Zhao's* basic attacks and **Audacious Charge** apply the **border=false*' mark to enemy champions hit, with the latter applying the mark to the farthest champion hit in the area of effect or the primary target if they are a champion. **Active:** **Xin Zhao** sweeps his spear around him, dealing physical damage to nearby enemies, capped at 600 against minions and monsters, and airborne all targets hit that are not **border=false** up-to 700 units over $0.75$ seconds, as well as stun them for the same duration. For the next 4 seconds, **Xin Zhao** is invulnerable against enemy champions far away from him.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $120-100$ seconds |
| **Cast Time** | $0.35$ seconds |
| **Cost** | 100 Mana |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Effect Radius** | 500 / 450 units |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | False |

**Scaling:**
- **Physical Damage:** $75-275$
- *bonus AD) (+ 110% AP) (+ 15% of target's
- **current** health)

**Notes:**

- Pet, stealthed trap, ground-targeted area of effect, damage over time, and delayed-damage abilities (e.g. *Death Mark*, *Explosive Charge*, *Hemoplague*) will not damage **Xin Zhao** if the enemy champion source is outside the circle when the damage applies. Effect at cast time end
- Displacement immunity will also resist the application of the stun.

---

## Patch History

### V25.18
- *Three Talon Strike*
  - **Bug Fixes:** When used during *Audacious Charge*’s active buff, if he is airborne during the dash and collides with another champion (ally or enemy), 'Three Talon Strike's active VFX no longer lingers at the displacement's end location.
- *Wind Becomes Lightning*
  - **Bug Fixes:** No longer illegally causes more slashes than intended if used in combination with Flash.
- *Crescent Guard*
  - **Bug Fixes:** Invulnerability shield VFX is now properly visible when an enemy **Xin Zhao** moves in and out of an allied Realm of Death.

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
- *Wind Becomes Lightning*
  - **New Effect:** Now causes him to slash four times rather than only once, slashing once on-cast and then 3 times over the next $0.15$ seconds.
    - Total slash damage is unchanged.
    - Only the initial slash can generate a *Determination* stack.
- *Audacious Charge*
  - **Bug Fixes:** No longer fails to cast when used on a target outside of its maximum target range during *Crescent Guard*’s cast time.
- *Crescent Guard*
  - **Bug Fixes:** Tooltip now correctly mentions that the damage cap also applies to minions and not only monsters.

### V25.16
- Stats
  - Base health reduced to 620 from 640.
- *Three Talon Strike*
  - Base damage per hit reduced to $15-75$ from $20-80$.
    - Total base damage reduced to $15×3-75×3$ from $20×3-80×3$.

### V25.14
- *Wind Becomes Lightning*
  - **Bug Fixes:** Increased *Audacious Charge* dash range indicator now properly does not appear when the ability is blocked by spell shield.

### V25.10
- Stats
  - Armor growth reduced to $4.4$ from 5.

### V25.06
- *Three Talon Strike*
  - Base damage per hit increased to $20-80$ from $16-68$.

### V14.24
- *Wind Becomes Lightning*
  - **Bug Fixes:** No longer reveals the user if cast while the cursor is near an enemy champion.
    - Note: the ability will continue to self-reveal if there is an enemy champion nearby.

### V14.18
- Stats
  - Armor growth increased to 5 from $4.7$.
- *Three Talon Strike*
  - Base damage increased to $16-68$ from $16-52$.
    - Total base damage increased to $16×3-68×3$ from $16×3-52×3$.

### V14.17
- *Crescent Guard*
  - **Bug Fixes:** Invulnerability now properly functions when the ability is cast while inside Realm of Death.

### V14.5
- *Three Talon Strike*
  - **New Effect:** Now triggers spell effects upon dealing damage.

## Trivia

- Xin Zhao was released when Season One started.
- Xin Zhao's dance references dance from .
  - A side-by-side comparison can be seen here.
- During development, he was called *XenZiou* or *XenZhao*.

---
*This page was automatically generated from League of Legends Wiki data.*