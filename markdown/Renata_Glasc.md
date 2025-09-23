# Renata_Glasc

## Table of Contents
- [Basic Information](#basic-information)
- [Abilities](#abilities)
- [Patch History](#patch-history)
- [Trivia](#trivia)

## Basic Information

| Attribute | Value |
|-----------|------:|
| **Champion** | Renata Glasc |

## Abilities

### Passive: Leverage

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Allies, Enemies |
| **Damage Type** | magic |
| **Spell Shield** | Special |
| **Spell Effects** | proc |

**INNATE:** **Renata**’s basic attacks are empowered to apply a mark that lasts 6 seconds, refreshes on subsequent hits and expires when attacking a new enemy. If the enemy was unmarked, the attack also deals **bonus** magic damage equal to key=% (+ 2% per 100 AP) of the target's **maximum** health.

Allied champions' damaging attacks and abilities against a marked target will consume the mark to deal additional **bonus** magic damage equal to key=% (+ 2% per 100 AP) of the target's **maximum** health.

*Leverage*’s damage is capped at 150 against epic monsters.

**Notes:**

- Spell shield will not block the mark's application.
  - : Interaction with mark consumption
- The mark's consumption by an ally respects enchantment redirection.

---

### Q: Handshake

| Attribute | Value |
|-----------|------:|
| **Range** | 900 (Missile range) / 275 (Recast throw range) units |
| **Cast Time** | $0.25$ seconds |
| **Tether Radius** | 1200 units |
| **Width** | 140 (Missile width) units |
| **Speed** | 1450 (Missile speed) units/second |
| **Cost** | 80 Mana |
| **Cooldown** | 16 (Starts post-effect if an enemy is hit, on-cast if not) seconds |
| **Cooldown Start** | Special |
| **Queue Time** | $0.5$ (First cast) / $0.3$ (Recast) seconds |
| **Targeting** | Direction |
| **Affects** | Self, Ememies |
| **Damage Type** | magic |
| **Spell Shield** | Special |
| **Spell Effects** | Special |
| **Projectile** | True |

**ACTIVE:** **Renata** fires a hook in the target direction that deals magic damage to the first enemy hit and roots them for 1 second, during which they are revealed.

If the root was applied, **Renata** forms a tether between her and the target for the same duration, causing **Renata** to become unable to declare attacks and have her movement speed **reduced** by 30%.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 80 / 125 / 170 / 215 / 260 (+ 80% AP) |

*Handshake* can be recast while the tether is active.

**RECAST:** **Renata** breaks the tether to knock the target in the target direction, though not through terrain, dealing the same damage to enemies they pass through. If the thrown target is a champion, all secondary targets hit are stunned for $0.5$ seconds.

**Notes:**

- Applies spell damage to the primary target and area damage to secondary targets.
- *Handshake*’s interaction between its tether and root:
  - If the root is not applied, neither is the tether.
  - The tether's duration lasts the same as the root duration, even if it is modified by tenacity.
  - If the root is removed, the tether is as well, but not vice versa.
- Spell shield will block the hook but not the recast's effects as the primary target.
  - As a secondary target, the recast's effects will be blocked.
- **Renata** may still move while the hook is in flight.
  - Her facing direction is locked towards the target direction of the hook.
- While the target is hooked, **Renata**’s facing direction is considered to be in their direction and not in the one she is moving.
- The movement speed reduction stacks additively with other movement speed bonuses.
  - It is a negative bonus, not a slow, and is thus not reduced by slow resist. Effect at cast time start

---

### W: Bailout

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 800 units |
| **Effect Radius** | 1300 (Radius for movement speed towards enemies) units |
| **Cost** | 80 Mana |
| **Cooldown** | 28 / 27 / 26 / 25 / 24 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Unit |

**ACTIVE:** **Renata** infuses herself or the target allied champion with a chemtech formula for 5 seconds, granting the target **bonus** attack speed and **bonus** movement speed while they are facing nearby visible enemy champions or minions, with both of the bonuses increasing in effectiveness by key=%. *Bailout*’s duration resets whenever the target scores a takedown against an enemy champion within 6 seconds of damaging them.

| Attribute | Value |
|-----------|------:|
| **Bonus Attack Speed** | 10 / 15 / 20 / 25 / 30% (+ 1% per 100 AP) |
| **Maximum Bonus Attack Speed** | 20 / 30 / 40 / 50 / 60% (+ 2% per 100 AP) |

| Attribute | Value |
|-----------|------:|
| **Bonus Movement Speed** | 10 / 12.5 / 15 / 17.5 / 20% (+ 1% per 100 AP) |
| **Maximum Bonus Movement Speed** | 20 / 25 / 30 / 35 / 40% (+ 2% per 100 AP) |

If the target takes fatal damage while *Bailout* is active, they are restored to 100% of their **maximum** health but suffer a true damage burn equal to 10% of their **maximum** health every until they reach **0** health, during which *Bailout*’s duration is reset every $0.25$ seconds. This effect may occur only *once* per application of *Bailout* while the target already has the buff and is not burning.

The burn will stop once the target scores a takedown against an enemy champion within 6 seconds of damaging them, setting their **current** health to 20% of their **maximum** health immediately afterwards.

**Notes:**

- Damage taken in excess of the fatal damage taken does not apply to the target's health after it was restored.
- The self-damage taken is considered raw damage and is calculated based on the target's **maximum** health at the time of taking lethal damage.
- *Bailout*’s bonuses will not reset in effectiveness if its duration is refreshed.
- *Bailout* takes priority over all resurrection and zombie state effects.
- *Bailout* will stop refreshing its duration while the target is burning after 250 seconds have elapsed.
- *Bailout* cannot be used on clones nor zombie state units.
- If *Bailout* is cast on a target that is already burning from a previous *Bailout* cast, they can trigger a subsequent health restore as well as be inflicted with another burn. This may occur an infinite number of times as long as the target stays burning and does not reach 0 health.
  - Casting *Bailout* on a target that already has the buff will only refresh the duration (without resetting the bonuses).
- If the target takes fatal damage during *Bailout*, upon destroying a turret while being targeted by it, they will receive their own Shut-Down gold (specifically ignoring the *base* bounty) instead of being executed.

---

### E: Loyalty Program

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 800 units |
| **Effect Radius** | cr 325 (Initial hit radius around Renata) / 225 (Missile explosion radius upon arriving at the target location) |
| **Width** | 220 (Missile width) units |
| **Speed** | 1450 (Missile speed, spawns up-to 200 units closer to the cast location already) units/second |
| **Cost** | 70 / 80 / 90 / 100 / 110 Mana |
| **Cooldown** | 14 / 13 / 12 / 11 / 10 seconds |
| **Queue Time** | $0.5$ seconds |
| **Targeting** | Location |
| **Affects** | Allies, Ememies |
| **Damage Type** | Magic |
| **Spell Shield** | Special |
| **Spell Effects** | spellaoe |
| **Projectile** | True |
| **Out of Range Behavior** | cast at max |

**ACTIVE:** **Renata** sends out chemtech rockets from either of her that instantly strike targets around her. After they converge and travel to the target location as a single missile, striking targets along its path and exploding upon reaching the target location.

**Renata** and allies struck are granted a shield for 3 seconds and enemies struck are dealt magic damage and slowed by 30% for 2 seconds.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 65 / 95 / 125 / 155 / 185 (+ 55% AP) |

| Attribute | Value |
|-----------|------:|
| **Shield Strength** | 50 / 65 / 80 / 95 / 110 (+ 50% AP) |

**Notes:**

- Spell shield will block either the rockets while they are in-flight or their explosion. Effect at cast time start

---

### R: Hostile Takeover

| Attribute | Value |
|-----------|------:|
| **Range** | 2000 (Range of individual cloud missiles) units |
| **Cast Time** | $0.75$ seconds |
| **Effect Radius** | 750 (Cloud missiles sight radius, pending for test) / cr 1000 (Berserker seek target radius) units |
| **Angle** | 14° |
| **Width** | 500 (Individual cloud missiles) units |
| **Speed** | 650 - 1000 (Acceleration by 50 per second, pending for test) units/second |
| **Cost** | 100 Mana |
| **Cooldown** | 150 / 140 / 130 / 120 / 110 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Spell Shield** | True |
| **Projectile** | True |

**ACTIVE:** **Renata** launches a cloud of potent chemicals that travels in the target direction, granting sight of its surroundings. Enemy champions and minions hit become berserk for a duration.

| Attribute | Value |
|-----------|------:|
| **Berserk Duration** | 1.25 / 1.5 / 1.75 / 2 / 2.25 seconds |

*Berserked* units gain 100% **bonus** attack speed and 25% increased size. Within their targeting radius, they prioritize attacking the closest unit by the following categories in descending order:
1. Their allied champions 
1. Allied non-champions
1. Allied wards
1. Enemy units (including monsters)

**Notes:**

- Typically, any enemies killed that directly result from *Berserk* are credited to **Renata Glasc**.

---

## Patch History

### V25.14
- Bailout
  - **Bug Fixes:** No longer breaks Kled’s health bar if he takes fatal damage while affected by *Bailout*.
  - **Bug Fixes:** No longer shows Skaarl health while its burn is active on Kled only after he takes fatal damage.

### V25.13
- Bailout
  - **Bug Fixes:** If a champion affected by *Bailout* is last-hit by an enemy summon, the summoner is now properly credited the champion kill.
- Hostile Takeover
  - **Bug Fixes:** If an enemy Mel damages and kills one of her allies with any number of projectiles via a Searing Brilliance-empowered attack while under the effects of *Hostile Takeover*, Mel is no longer incorrectly credited the kill instead of **Renata Glasc**.

### V25.11
- Renata Glasc
  - Adjusted splash art.
  - Updated models and VFX.

### V25.04
- Hostile Takeover
  - **Bug Fixes:** No longer sometimes bypasses Wind Wall.

### V14.7
- Bailout
  - **Bug Fixes:** Champions within Lamb's Respite that are affected by Bailout and reach the threshold after scoring a champion takedown while still inside its area of effect no longer sometimes forcibly die.

### V14.4
- Handshake
  - Tether now innately checks every $0.05$ seconds for valid range conditions, and no longer waits for every 8th server tick.

### V13.22
- Leverage
  - **Bug Fixes:** No longer instantly kills an enemy Zac if her ally attacked one of his Bloblets that was marked with the debuff.

### V13.21
- Leverage
  - Damage now triggers **Renata Glasc**’s own damage effects but still grants kill credit to the ally applying it.

### V13.19
- Hostile Takeover
  - **Bug Fixes:** Casting it after her running animation no longer causes her model to have visual issues.

### V13.7
- Hostile Takeover
  - **Bug Fixes:** Berserk effect no longer causes an enemy Caitlyn to gain kill credit for allies she kills after it interrupted her Ace in the Hole channel.

## Trivia

- All of Renata's abilities are references to real-world financial terms.
  - Leveraged finance is the use of an above-normal amount of debt, as opposed to equity or cash, to finance the purchase of investment assets.
  - Handshake is the famous accordance social act between two negotiators.
  - Bailout is when a business, an individual, or a government provides money and/or resources (also known as a capital injection) to a failing company.
  - Loyalty Program is a marketing strategy designed to encourage customers to continue to shop at or use the services of a business associated with the program.
  - Hostile Takeover occurs when an acquiring company attempts to take over a target company against the wishes of the target company's management.

---
*This page was automatically generated from League of Legends Wiki data.*