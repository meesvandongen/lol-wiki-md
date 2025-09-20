# Aurora

## Table of Contents
- [Basic Information](#basic-information)
- [Statistics](#statistics)
- [Abilities](#abilities)
- [Patch History](#patch-history)

## Basic Information

| Attribute | Value |
|-----------|-------|
| **Champion** | Aurora |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $550$ | $+85$ | $1995.0$ |
| **Mana** | $350$ | $+50$ | $1200.0$ |
| **Armor** | $22$ | $+3.5$ | $81.5$ |
| **Magic Resist** | $30$ | $+0.5$ | $38.5$ |
| **Attack Damage** | $56$ | $+3.1$ | $108.7$ |
| **Attack Speed** | $0.625$ | $+3.2\%$ | $0.965$ |

## Abilities

### Passive: Spirit Abjuration

**Innate:* cisSpirit Abjuration*, stacking up to 3 times.

*The third stack consumes them all to deal **bonus** magic damage based on the target's **maximum** health. If the target was a champion, she exorcises them to free a Spirit that follows her for a few seconds. **Aurora** heal over time based on her Spirits.*

**Innate:** ''Aurora's'* damaging basic attacks and abilities apply a stack of *Spirit Abjuration' to enemies for 4 seconds, refreshing on subsequent applications and stacking up to 3 times. The third stack against a target consumes them all to deal **bonus** magic damage equal to 1% , capped at $90+10*x$ against monsters. Upon consuming the stacks from a champion, **Aurora** additionally exorcises them, freeing a Spirit from the target that follows her for the same duration. For each active Spirit, **Aurora** is heal for $3 to 20$ (+ 2% AP) every second. Subsequent exorcisms free further Spirits and refresh all active Spirits. **Aurora** may have up to 4 Spirits at a time, for a maximum heal per tick of $3*4 to 20*4$ (+ $2*4$% AP).

| Attribute | Value |
|-----------|-------|
| **Cooldown** | targeting = Passive |

**Notes:**

- The spirits grant vision of a small area around themselves.

---

### Q: Twofold Hex

**Active:** **Aurora** fires a bolt of energy that deals magic damage to enemies hit and marks them for a short time. *Twofold Hex* can be recast while at least one target is marked.

**Recast:** **Aurora** expunges the curse from all marked targets, drawing back bolts that each deal magic damage to enemies they pass through based on their **missing** health.

**Active:** **Aurora** fires a bolt of energy in the target direction that deals magic damage to enemies hit and marks them with a curse for $3.5$ seconds. *Twofold Hex* can be recast after $0.1$ seconds while at least one target is marked, and does so automatically at the end of the mark's duration. **Recast:** **Aurora** expunges the curse from all marked targets, drawing the spiritual energy back to her as bolts that each deal magic damage to all enemies they pass through, reduced to 40% against minions and 50% against monsters and increased by $type=target's **missing* health$%-1) for 11*missing* health100% damage.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $9-7$ seconds |
| **Cast Time** | $0.25$ / None |
| **Cost** | 60 mana |

**Scaling:**
- **Magic Damage:** $ (+
- **Subsequent Bolt Minimum Magic Damage:** $* (+ $% AP)* to ** (+ $% AP)

**Notes:**

- *Twofold Hex* can also be recast for $0.66$ seconds after the $0.1$-second delay. If the recast is used during this time window, *Twofold Hex* will be buffered to recast automatically as soon as the bolt missile from the first cast has fizzled from reaching maximum range, even if no targets are marked. This automatic recast can also be buffered during 'Twofold Hex's' cast time.
- Spell shield can block the effects of the active and the recast.
- The marked targets do not need to be visible in order for **Aurora** to recast *Twofold Hex*.
  - They do however need to be targetable; if all marked targets are untargetable, 'Twofold Hex's' recast is disabled.
- Once **Aurora** has successfully recast *Twofold Hex*, she will not be able to recast it again for the duration regardless of any number of marked enemies still remaining.
  - If all enemies marked by *Twofold Hex* are untargetable when **Aurora** manually recasts *Twofold Hex*, the ability will be consumed but evidently without effect. It will however automatically recast again at the end of the marks' duration to expunge any remaining marks.

---

### W: Across the Veil

**Active:** **Aurora** dash into invisibility for a moment and gains *Realm Hopper* for a few seconds.

*'Across the Veil's' *cooldown* will reset upon scoring a champion takedown within a short time.*

**Active:** **Aurora** dash in the target direction inside a portal to the Spirit Realm. Upon completing the dash, she becomes invisible for a duration and gains *Realm Hopper* for 4 seconds. **Realm Hopper:** **Aurora** becomes ghosted and gains **bonus movement speed** for the duration. Scoring a takedown against an enemy champion within 3 seconds of damaging them will ah 'Across the Veil's' cooldown.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $22-18$ seconds |
| **Cast Time** | none |
| **Cost** | 80 mana |

**Scaling:**
- **Invisibility Duration:** $1-1.6$ seconds
- **Bonus Movement Speed:** $20-40$%

**Notes:**

- **Aurora** will **not** gain the stealth if her dash is interrupted.
  - She will however gain the stealth despite the dash being interrupted from colliding with border. *** Her dash is interrupted in this case from being overridden by the dash.
- **Aurora** is locked out of using all abilities, summoner spells, and item actives during the dash.
  - She cannot buffer these spells to cast after the lockout. This is in place to prevent her from breaking the stealth from certain spell casts. *** This is the case even for spells that do not break stealth when cast.
- Using a basic attack breaks the stealth at the end of the attack windup.
- At rank 0, *Realm Hopper* will grant **Aurora** 15% **bonus** movement speed. This is only relevant for using **Between Worlds**.
- If the **total** duration of *Realm Hopper* gained from a new cast of *Across the Veil* or **Between Worlds** is longer than the **remaining** duration of the one gained from a previous cast, then re-gaining *Realm Hopper* will override the previous buff's duration.
  - If the **remaining** duration from a previous cast is shorter, then the same buff remains in place without refreshing. *** In both cases, the buff cannot stack with itself.

---

### E: The Weirding

**Active:** **Aurora** blasts energy and dash backward. The blast deals magic damage to enemies hit and slow them for a moment.

**Active:** **Aurora** blasts energy in a line in the target direction and dash 250 units in the opposite direction. The blast deals magic damage to enemies hit and slow them by 80% for 1 second decaying after the first $0.15$ seconds.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $15-11$ seconds |
| **Cast Time** | $0.35$ seconds |
| **Cost** | 80 mana |

**Scaling:**
- **Magic Damage:** $70-230$ (+ 70% AP)

**Notes:**

- **Aurora** is locked out of using all abilities, summoner spells, and item actives during the dash.
- While ground or root, *The Weirding* can be cast but **Aurora** will not dash.
  - Similarly, if **Aurora** is affected by ground or root during the cast time, she will not dash afterwards. *** This only applies to ground and root. Immobilize effects will not prevent the dash afterwards.

---

### R: Between Worlds

**Active:** **Aurora** becomes displacement immunity and dash, unleashing an expanding shockwave that deals magic damage to enemies hit and slow them briefly, and gaining **Across the Veil** for a few seconds.

*A circular then forms in the area. If **Aurora** touches the border, she dash to the opposite of it. If an enemy touches the border, they are slow briefly. If **Aurora** death or leaves the area, the will immediately close.*

**Active:** **Aurora** dash in the target direction with displacement immunity over $0.4$ seconds. She then gains **Across the Veil** for a duration and unleashes a shockwave from 425 units in front of her, expanding over $0.75$ seconds to deal magic damage to enemies hit and slow them by 30% for 2 seconds. The shockwave leaves behind a circular to the Spirit Realm for a duration, forming borders that interact with **Aurora** and enemies. If **Aurora** collides with the border, she dash to the diametrically opposite of the over $0.4$ seconds, becoming untargetable and lockout during the travel. If an enemy collides with the border, they are slow by 50% for a duration. *Between Worlds* can be recast after 1 second of unleashing the shockwave while the is active. **Recast:** **Aurora** closes the prematurely. *The closes immediately if **Aurora** death, enters resurrection, leaves the area, or is not within the area upon creation.*

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $140-100$ seconds |
| **Cast Time** | none |
| **Cost** | 100 mana |

**Scaling:**
- **Magic Damage:** $175-375$ (+ 70% AP) *
- **Rift Duration:** $2.5-0.75-4-0.75$ seconds
- **Slow Duration:** $1.5-2$ seconds

**Notes:**

- Collision with the border is essentially moving within range of it while being targetable, including via normal movement, dash, or displacement.
  - **Aurora** and enemies can collide with the border any number of times, as much as the duration permits. *** Enemies can be applied the slow by colliding with the border only once after moving within collision radius of it. The slow may be applied again from collision only until after they have left the collision radius; it cannot be continuously applied while enemies remain within collision radius of the border.
  - Untargetable units cannot collide with the border, including **Aurora** herself.
- The in-game tooltip incorrectly lists the duration as $0.75$ seconds longer than it actually is.
  - It is seemingly including the time required for the shockwave to fully expand. However, the and its borders will only form after the fact, so the actual duration of the is shorter.
- 'Between Worlds' ' untargetability from the dash does not homing projectile destruction in-flight projectiles.
- 'Between Worlds' ' first cast is disabled while ground or root.
- **Aurora** can still dash from colliding with the border even if she is immobilize or ground.
- Spell shield can block both the shockwave and border collision.
- A pair of portals signifying ''Aurora's'' nearest possible entry and its corresponding exit is visible to her at all times.
- The following table refers for interactions while **Aurora** is dashing:
  - This lockout applies to both the initial dash and the dashes in the formed rift. *** The lockout during the initial cast's dash persists for another $0.5$ seconds after the dash ends. *** The lockout during the dash persists for another $0.15$ seconds after the dash ends.

---

## Patch History

### V25.18
- *Spirit Abjuration*
  - Mark target health ratio reduced to 1% of target's **maximum** health from $2.5$%.
  - Mark AP ratio increased to $2.7$% per 100 AP from 2% per 100 AP.
- *Between Worlds*
  - ***Bug Fixes:*** Tooltip now notes the proper *Realm Hopper* buff duration gained and no longer incorrectly notes that she gains it for the rift's duration only.

### V25.17
- *Between Worlds*
  - ***Bug Fixes:*** Tooltip now notes the correct duration.

### V25.15
- *Spirit Abjuration*
  - **Undocumented / Bug Fix:** Tooltip now notes the mark consumption's monster damage cap.
    - The correct *current* numbers are noted, but the tooltip is not set up to automatically update if they change in the future.

### V25.14
- *Between Worlds*
  - ***Bug Fixes:*** timer expiring.

### V25.12
- Stats
  - Base magic resistance increased to 32 from 30.
- *Between Worlds*
  - AP ratio increased to 70% AP from 60% AP.

### V25.10
- *Twofold Hex*
  - ***Bug Fixes:*** No longer fails to trigger Hypershot when under valid conditions.
- *The Weirding*
  - ***Bug Fixes:*** No longer fails to trigger Hypershot when under valid conditions.

### V25.09
- General
  - ***Bug Fixes:*** Realm Hopper's buff tooltip now has a proper name and description.

### V25.07
- *Across the Veil*
  - ***Bug Fixes:*** Corrected the capitalization of "move speed" to "Move Speed" in the tooltip.

### V25.05
- *Spirit Abjuration*
  - ***Bug Fixes:*** Stack application from basic attacks is now properly prevented by block.
- *The Weirding*
  - AP ratio reduced to 70% AP from 80% AP.
- *Between Worlds*
  - Exit slow strength reduced to 50% from 75%.

### V25.04
- General
  - ***Bug Fixes:*** Resolved several missing or incorrect SFX.
- 
  - *The Weirding*
    - ***Bug Fixes:*** Adjusted VFX to more correctly match the area of effect.

---
*This page was automatically generated from League of Legends Wiki data.*