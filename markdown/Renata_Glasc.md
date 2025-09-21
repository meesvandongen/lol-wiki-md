# Renata_Glasc

## Table of Contents
- [Basic Information](#basic-information)
- [Abilities](#abilities)
- [Patch History](#patch-history)
- [Trivia](#trivia)

## Basic Information

| Attribute | Value |
|-----------|-------|
| **Champion** | Renata Glasc |

## Abilities

### Passive: Leverage

**Innate:** **Renata Glasc**’s basic attacks apply a mark that lasts a few seconds and expires when attacking a new enemy. If the enemy was unmarked, this also deals **bonus** magic damage based on the target's **maximum** health.

*Allied champions damaging attacks and abilities against a marked target will consume the mark to deal magic damage based on the target's **maximum** health.*

**Innate:** ''Renata's** basic attacks are empowered to apply a mark that lasts 6 seconds, refreshes on subsequent hits and expires when attacking a new enemy. If the enemy was unmarked, the attack also deals **bonus'' magic damage equal to key=% (+ 2% per 100 AP) of the target's **maximum** health. Allied champions damaging attacks and abilities against a marked target will consume the mark to deal additional **bonus** magic damage equal to key=% (+ 2% per 100 AP) of the target's **maximum** health. 'Leverage's damage is capped at 150 against epic monsters.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Allies, Enemies |
| **Damage Type** | magic |
| **Spell Shield** | Special |
| **Spell Effects** | proc |

**Notes:**

- Spell shield will not block the mark's application.
  - : Interaction with mark consumption
- The mark's consumption by an ally respects enchantment redirection.

---

### Q: Handshake

**Active:** **Renata Glasc** fires a hook in the target direction that deals magic damage to the first enemy hit and briefly root them. She can recast the ability while the target is rooted.

**Recast:** **Renata** airborne the target in the target direction that deals magic damage to enemies hit. If the thrown target is a champion, enemies hit are briefly stun.

**Active:** **Renata** fires a hook in the target direction that deals magic damage to the first enemy hit and root them for 1 second, during which they are true sight. If the root was applied, **Renata** forms a tether between her and the target for the same duration, causing **Renata** to become unable to declare attacks and have her movement speed **reduced** by 30%. *Handshake* can be recast while the tether is active. **Recast:** **Renata** breaks the tether to airborne the target in the target direction, though not through terrain, dealing the same damage to enemies they pass through. If the thrown target is a champion, all secondary targets hit are stun for $0.5$ seconds.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | 16 seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | 80 Mana |
| **Targeting** | Direction |
| **Affects** | Self, Ememies |
| **Damage Type** | magic |
| **Speed** | 1450 units/second |
| **Spell Shield** | Special |
| **Spell Effects** | Special |
| **Projectile** | True |

**Scaling:**
- **Magic Damage:** $80-260$ (+ 80% AP)

**Notes:**

- Applies spell damage to the primary target and area damage to secondary targets.
- 'Handshake's interaction between its tether and root:
  - If the root is not applied, neither is the tether.
  - The tether's duration lasts the same as the root duration, even if it is modified by tenacity.
  - If the root is removed, the tether is as well, but not vice versa.
- Spell shield will block the hook but not the recast's effects as the primary target.
  - As a secondary target, the recast's effects will be blocked.
- **Renata** may still move while the hook is in flight.
  - Her facing direction is locked towards the target direction of the hook.
- While the target is hooked, ''Renata's' facing direction is considered to be in their direction and not in the one she is moving.
- The reduction stacks additively with other *movement speed* bonuses.
  - It is a negative bonus, not a slow, and is thus not reduced by slow resist. Effect at cast time start

---

### W: Bailout

**Active:** **Renata Glasc** grants herself or the target allied champion ramping **bonus attack speed** in addition to **bonus movement speed** toward enemies. 'Bailout's duration resets whenever the target scores a takedown against an enemy champion.

*If the target would death while *Bailout* is active, their health is set back to full but they suffer a true damage burn that would kill them over a short time. The target can stop the burn by scoring a takedown.*

**Active:** **Renata** infuses herself or the target allied champion with a chemtech formula for 5 seconds, granting the target **bonus attack speed** and **bonus movement speed** while they are facing nearby sight enemy champions or minions, with both of the bonuses increasing in effectiveness by key=%. 'Bailout's duration resets whenever the target scores a takedown against an enemy champion within 6 seconds of damaging them. If the target takes death while *Bailout* is active, they are restored to 100% of their **maximum** health but suffer a *true damage* burn equal to 10% of their **maximum* health every rutngt*0** health, during which 'Bailout's* duration is reset every $0.25$ seconds. This effect may occur only *once* per application of *Bailout' while the target already has the buff and is not burning. The burn will stop once the target scores a takedown against an enemy champion within 6 seconds of damaging them, setting their **current** health to 20% of their **maximum** health immediately afterwards.

| Attribute | Value |
|-----------|-------|
| **Range** | 800 units |
| **Cooldown** | $28-24$ seconds |
| **Cast Time** | none |
| **Cost** | 80 Mana |
| **Targeting** | Unit |
| **Effect Radius** | 1300 units |

**Scaling:**
- **Bonus Attack Speed:* $10-30$% (+ 1% per 100 AP)2-30×2$% (+ $1×2$% per 100 AP) **Bonus Movement Speed:* $10-20$% (+ 1% per 100 AP)2-20×2$% (+ $1×2$% per 100 AP)

**Notes:**

- Damage taken in excess of the fatal damage taken does not apply to the target's health after it was restored.
- The self-damage taken is considered raw damage and is calculated based on the target's **maximum** health at the time of taking lethal damage.
- 'Bailout's bonuses will not reset in effectiveness if its duration is refreshed.
- *Bailout* takes priority over all resurrection and zombie state effects.
- *Bailout* will stop refreshing its duration while the target is burning after 250 seconds have elapsed.
- *Bailout* cannot be used on clone nor zombie state units.
- If *Bailout* is cast on a target that is already burning from a previous *Bailout* cast, they can trigger a subsequent health restore as well as be inflicted with another burn. This may occur an infinite number of times as long as the target stays burning and does not reach 0 health.
  - Casting *Bailout* on a target that already has the buff will only refresh the duration (without resetting the bonuses).
- If the target takes fatal damage during *Bailout*, upon destroying a turret while being targeted by it, they will receive their own Shut-Down gold (specifically ignoring the *base* bounty) instead of being executed.

---

### E: Loyalty Program

**Active:** **Renata Glasc** sends out two chemtech rockets that strike targets around her, then converge into a single missile that explodes at the target location.

**Renata** and allies struck are granted a shield, and enemies struck are dealt magic damage and slow for a short time.

**Active:** **Renata** sends out chemtech rockets from either of her that instantly strike targets around her. After they converge and travel to the target location as a single missile, striking targets along its path and exploding upon reaching the target location. **Renata** and allies struck are granted a shield for 3 seconds and enemies struck are dealt magic damage and slow by 30% for 2 seconds.

| Attribute | Value |
|-----------|-------|
| **Range** | 800 units |
| **Cooldown** | $14-10$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $70-110$ Mana |
| **Targeting** | Location |
| **Affects** | Allies, Ememies |
| **Damage Type** | Magic |
| **Speed** | 1450 units/second |
| **Effect Radius** | 325 / 225 units |
| **Spell Shield** | Special |
| **Spell Effects** | spellaoe |
| **Projectile** | True |

**Scaling:**
- **Magic Damage:** $65-185$ (+ 55% AP)
- **Shield Strength:** $50-110$ (+ 50% AP)

**Notes:**

- Spell shield will block either the rockets while they are in-flight or their explosion. Effect at cast time start

---

### R: Hostile Takeover

**Active:** **Renata Glasc** launches a cloud of potent chemicals that travels in the target direction. Enemy champions and minions hit briefly become berserk, gaining *as *bonus attack speed*.

**Active:** **Renata** launches a cloud of potent chemicals that travels in the target direction, granting sight of its surroundings. Enemy champions and minions hit become berserk for a duration. *Berserked* units gain *100% *bonus attack speed* and 25% increased size. Within their targeting radius, they prioritize attacking the closest unit by the following categories in descending order: # Their allied champions # Allied non-champions # Allied wards # Enemy units (including monster)

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $150-110$ seconds |
| **Cast Time** | $0.75$ seconds |
| **Cost** | 100 Mana |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Speed** | 650 - 1000 units/second |
| **Effect Radius** | 750 / cr 1000 units |
| **Spell Shield** | True |
| **Projectile** | True |

**Scaling:**
- **Berserk Duration:** $1.25-2.25$ seconds

**Notes:**

- Typically, any enemies killed that directly result from *Berserk* are credited to **Renata Glasc**.

---

## Patch History

### V25.14
- *Bailout*
  - **Bug Fixes:** No longer breaks **Kled**’s health bar if he takes fatal damage while affected by *Bailout*.
  - **Bug Fixes:** No longer shows *Skaarl* health while its burn is active on **Kled** only after he takes fatal damage.

### V25.13
- *Bailout*
  - **Bug Fixes:** If a champion affected by *Bailout* is last-hit by an enemy summon, the summoner is now properly credited the champion kill.
- *Hostile Takeover*
  - **Bug Fixes:** If an enemy **Mel** damages and kills one of her allies with any number of projectiles via a *Searing Brilliance*-empowered attack while under the effects of *Hostile Takeover*, Mel is no longer incorrectly credited the kill instead of **Renata Glasc**.

### V25.11
- Renata Glasc
  - Adjusted splash art.
  - Updated models and VFX.

### V25.04
- *Hostile Takeover*
  - **Bug Fixes:** No longer sometimes bypasses Wind Wall.

### V14.7
- *Bailout*
  - **Bug Fixes:** Champions within Lamb's Respite that are affected by Bailout and reach the threshold after scoring a champion takedown while still inside its area of effect no longer sometimes forcibly die.

### V14.4
- *Handshake*
  - Tether now innately checks every $0.05$ seconds for valid range conditions, and no longer waits for every 8th server tick.

### V13.22
- *Leverage*
  - **Bug Fixes:** No longer instantly kills an enemy **Zac** if her ally attacked one of his *Bloblets* that was marked with the debuff.

### V13.21
- *Leverage*
  - Damage now triggers ''Renata Glasc's' own damage effects but still grants kill credit to the ally applying it.

### V13.19
- *Hostile Takeover*
  - **Bug Fixes:** Casting it after her running animation no longer causes her model to have visual issues.

### V13.7
- *Hostile Takeover*
  - **Bug Fixes:** Berserk effect no longer causes an enemy **Caitlyn** to gain kill credit for allies she kills after it interrupted her *Ace in the Hole* channel.

## Trivia

- All of Renata's abilities are references to real-world financial terms.
  - *Leveraged* finance is the use of an above-normal amount of debt, as opposed to equity or cash, to finance the purchase of investment assets.
  - *Handshake* is the famous accordance social act between two negotiators.
  - *Bailout* is when a business, an individual, or a government provides money and/or resources (also known as a capital injection) to a failing company.
  - *Loyalty Program* is a marketing strategy designed to encourage customers to continue to shop at or use the services of a business associated with the program.
  - *Hostile Takeover* occurs when an acquiring company attempts to take over a target company against the wishes of the target company's management.
- Renata orders her device to shoot for marking targets with basic attacks, otherwise shooting with her pistol.
- Renata is the second champion who does not actually appear in her own Splash Art, the first being **Jhin**.
  - Renata is the third champion to have a Loading screen and Portrait icon not focus the actual champion, in her case instead focusing on the mirror image of her in a pool of water. The first two are **LeBlanc** and **Jhin**.

---
*This page was automatically generated from League of Legends Wiki data.*