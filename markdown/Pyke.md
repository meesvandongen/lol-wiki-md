# Pyke

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
| **Champion** | Pyke |
| **Title** | the Bloodharbor Ripper |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2018-05-31 |
| **Release Patch** | V8.11 |
| **Latest Changes** | V25.11 |
| **Roles** | Assassin, Catcher |
| **Riot Positions** | Support |
| **External Positions** | Support |
| **Blue Essence** | 1575 |
| **Riot Points** | 790 |
| **Difficulty** | 2 |
| **Hero Type** | Support |
| **Alt Type** | Assassin |
| **Adaptive Type** | Physical |
| **Damage** | 2 |
| **Toughness** | 1 |
| **Control** | 3 |
| **Mobility** | 3 |
| **Utility** | 1 |
| **Style** | 50 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $670.0$ | $+110.0$ |
| **Mana** | $415.0$ | $+50.0$ |
| **Health Regen** | $7.0$ | $+0.5$ |
| **Mana Regen** | $8.0$ | $+1.0$ |
| **Armor** | $43.0$ | $+4.7$ |
| **Magic Resist** | $32.0$ | $+2.05$ |
| **Attack Damage** | $62.0$ | $+2.0$ |
| **Attack Speed** | $0.667$ | |
| **Movement Speed** | $330.0$ | $+0.0$ |
| **Attack Range** | $150.0$ | $+0.0$ |
| **Base Attack Speed** | $0.667$ | |
| **Attack Speed Ratio** | $0.667$ | |
| **Bonus AS per Level** | $2.5\%$ | |
| **Attack Windup** | $20.0\%$ | |
| **Acquisition Radius** | $400$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $120$ units | |
| **Selection Height** | $155$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $110.0\%$ |
| **Damage Taken** | $90.0\%$ |

#### URF

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $100.0\%$ |
| **Damage Taken** | $95.0\%$ |

## Abilities

### Passive: Gift of the Drowned Ones

| Attribute | Value |
|-----------|------:|
| **Effect Radius** | 1500 units |
| **Targeting** | Passive |
| **Affects** | Self |

**INNATE:** **Pyke**’s health cannot increase except through growth (per level), instead he gains「 1 **bonus** attack damage per 14 **bonus** health. ⟷ **bonus** attack damage equal to $7.143$% of **bonus** health. 」

**INNATE:** **Pyke** gains 1 **bonus** movement speed while in the river.

**INNATE:** **Pyke** stores 9% (+ $0.2$% per 1 Lethality) of the post-mitigation damage (Damage calculated after modifiers) he takes from enemy champions as grey health on his health bar, increased to 40% (+ $0.4$% per 1 Lethality) while there are two or more visible enemy champions nearby. He can store up to 80 (+ 800% **bonus** AD) grey health, with an upper cap of 55% of his **maximum** health.

While **Pyke** is not visible to enemies, he rapidly consumes his grey health to heal for the same amount.

**Notes:**

- Game elements that increase health based on the champion's health (e.g Overgrowth) will increase the health before getting converted by *Gift of the Drowned Ones*, thus granting increased bonus AD.
- Spirit Visage and Grievous Wounds take effect after *Gift of the Drowned Ones*’s healing cap. ***Pyke**’s *grey health* does not have a time limit, and will stay on his health bar until **Pyke** leaves enemy vision or dies.
- Since **Pyke**’s **maximum** health cannot be increased, *Gift of the Drowned Ones* reaches the upper cap of 55% of **maximum** health at (0.6*( **bonus** attack damage.
- Untargetable champions count towards **Pyke**’s modified *grey health* storing.
- Amount of attack damage granted from list of items that grants health, converted with *Gift of the Drowned Ones*:

---

### Q: Bone Skewer

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ (Quick-release) seconds |
| **Target Range** | channel time |
| **Effect Radius** | 550 (Pull range) units |
| **Width** | 140 (Harpoon missile) / 200 (Thrust hitbox) units |
| **Cost** | 70 / 75 / 80 / 85 / 90 Mana |
| **Cooldown** | 10 / 9.5 / 9 / 8.5 / 8 seconds |
| **Cooldown Start** | post-effect |
| **Queue Time** | $0.275$ seconds |
| **Targeting** | Auto / Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | Spell |
| **Projectile** | Special |
| **Silence** | True |

**ACTIVE:** **Pyke** charges while being slowed by 20% for up to 3 seconds to increase *Bone Skewer*’s range over the first second of the channel. *Bone Skewer* can be recast within the duration.

**RECAST:** **Pyke** hurls his harpoon in the target direction, becoming unable to act while it is in flight and dealing physical damage to the first enemy hit and pulling them, during which they are also revealed, then slowing them by 90% for 1 second.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 100 / 150 / 200 / 250 / 300 (+ 75% **bonus** AD) |

Releasing the ability within $0.4$ seconds causes **Pyke** to instead thrust his blade in the target direction, dealing the same damage to the closest enemy champion, or closest enemy otherwise.

If the charge is interrupted or completes without reactivation, *Bone Skewer* is cancelled and the ability is put on cd cooldown but refunds mana. *Bone Skewer*’s mana cost is also refunded if it hits an enemy champion.

**Notes:**

- Only the charged version of *Bone Skewer* will be intercepted.
- The pull can fling an enemy behind **Pyke** if the harpoon hits at close range. Effect at cast time end
- The following table refers for interactions while **Pyke** is channeling:
- The following table refers for interactions while **Pyke** is locked out from the harpoon being in flight:
  - The lockout ends as soon as the missile fizzles from hitting a target or reaching maximum range, or being intercepted.

---

### W: Ghostwater Dive

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Cost** | 65 Mana |
| **Cooldown** | 14 / 13 / 12 / 11 / 10 seconds |
| **Queue Time** | $0.275$ seconds |
| **Targeting** | Auto |
| **Affects** | Self |
| **Detection Radius** | 600 |

**ACTIVE:** **Pyke** submerges into water for 5 seconds, entering camouflage and gaining ms that decays over the duration. Attacking or casting abilities ends *Ghostwater Dive* immediately.

*Enemy champions within 1500 units of **Pyke** are alerted of his presence and whether he can execute them with Death from Below.*

**Notes:**

- When an enemy is alerted, sharks will circle underneath their feet while a sound will be played to them that changes based on health threshold.
- Using a basic attack breaks the stealth at the end of the attack windup.
- While submerged, a unique visual indicator will appear above **Pyke** if there are enemy champions closer than 700 units but farther than 600 units.

---

### E: Phantom Undertow

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 550 units |
| **Effect Radius** | er 110 (Phantom unit checking for nearby units) |
| **Width** | 110 (Phantom tracking missile) units |
| **Speed** | 3000 (Both dash and returning phantom's missile speed) units/second |
| **Cost** | 40 Mana |
| **Cooldown** | 15 / 14 / 13 / 12 / 11 seconds |
| **Queue Time** | $0.275$ seconds |
| **Targeting** | Direction |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | True |
| **Terrain Grace** | True |
| **Grounded** | True |
| **Knockdown** | True |

**ACTIVE:** **Pyke** dashes in the target direction, leaving behind a *Phantom*. **Pyke** is ghosted while the *Phantom* is active.

After 1 second, the *Phantom* homes back to **Pyke** to stun enemies around it and those it passes through along the way for $1.25$ (+ $0.1$ per 10 Lethality) seconds. Enemy champions hit also take physical damage.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 100 / 150 / 200 / 250 / 300 (+ 100% **bonus** AD) |

**Notes:**

- The dash distance can be extended to up to 900 units (estimated) when targeting across terrain.
- Like most projectiles, the flying *Phantom* cannot hit targets whose center is behind the position it originates at or beyond the position it catches up to **Pyke** at, but the spell *can* hit targets behind its origin as long as their hitbox radius overlaps with the original unit *Phantom*’s 110-radius check at the end of its lifespan.

---

### R: Death from Below

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.5$ seconds |
| **Target Range** | 750 units |
| **Effect Radius** | 125 (Circle check in the center) / $282.5$ units |
| **Cost** | 100 mana |
| **Cooldown** | 100 / 92.5 / 85 / 77.5 / 70 seconds |
| **Cooldown Start** | on-cast |
| **Queue Time** | $0.25$ seconds |
| **Targeting** | Location |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical True |
| **Spell Shield** | True |
| **Spell Effects** | Special |
| **Out of Range Behavior** | walk to location |
| **Call For Help** | Special |
| **Grounded** | True |

**ACTIVE:** **Pyke** marks the target location with the shape of an *X* before striking it, executing enemy champions within the area that are below 250–550@6–18 (+ 80% **bonus** AD) (+ $1.5$ per 1 Lethality) health. Other enemies hit and enemy champions above the threshold are instead dealt「 50% of the amount as physical damage. ⟷ 125–275@6–18 (+ 40% **bonus** AD) (+ $0.75$ per 1 Lethality) physical damage. 」

If *Death from Below* hits an enemy champion or at least one is killed inside the *X* by the *execution* or an ally, **Pyke** will blink to the center of the *X*. For the latter case **Pyke** can also recast the ability within 20 seconds at no cost.

Each successful *execution* grants one *Your Cut* to the last assisting ally, instead of its natural assist gold gold. Otherwise, **Pyke** is granted a *Your Cut* for each enemy champion killed inside the *X* by an ally during *Death from Below*’s cast time.

**Notes:**

- Applies raw damage to executed targets and area damage to targets above the threshold.
- *Death from Below*’s execute against an enemy champion will aggro nearby enemy minions.
- *Death from Below* cannot execute clones. They are always dealt 50% physical damage.
- If the target is in a zombie state, does not enter the death state, or is killed by the ability while *above* the execution threshold, *Death from Below* does not grant Your Cut nor the recast.
- Spell shield does not prevent **Pyke** from blinking.
- If an enemy champion dies inside the *X* during *Death from Below*’s cast time, **Pyke** will be shown as having received an assist in the kill feed, but this will not trigger takedown effects.
- *Death from Below* cannot be interrupted; **Pyke** will always appear at the target location if enemy champions are caught inside the *X* even if he is moved (e.g. by being displaced) during the cast time.
  - The only exception is if he is being suppressed, in which case he will not blink.
- Axiom Arcanist does not refund the cooldown while **Pyke** is able to recast.
  - Axiom Arcanist tooltip counts it refunding *Death from Below* even when it doesn't.

---

## Patch History

### V25.11
- Gift of the Drowned Ones
  - Added new heal loop SFX.
- Phantom Undertow
  - Phantom charge and dash SFX now follow the phantom's movement.
- Ghostwater Dive
  - **Bug Fixes:** Stealth shimmer VFX no longer sometimes fails to appear.
- Death from Below
  - **Bug Fixes:** Corrected the origin point of the self cast audio for several skins.

### V25.08
- Bone Skewer
  - **Bug Fixes:** Stab now refunds the correct amount of mana.

### V25.S1.1
- Death from Below
  - **Removed:*** Your Cut no longer grants an additional 100 gold if it was awarded from a kill that counted as *First Blood*.

### V14.20
- Gift of the Drowned Ones
  - **Bug Fixes:** No longer incorrectly reduces health upon consuming a Total Biscuit of Everlasting Will.

### V14.18
- General
  - **Bug Fixes:** Voice lines upon killing an champion and for execution through Death from Below are now properly played at the same time for enemy players.

### V14.17
- Stats
  - Base armor reduced to 43 from 47.

### V14.14
- Bone Skewer
  - **Bug Fixes:** No longer pulls enemies to his current location instead of the missile's cast location.

### V14.9
- Ghostwater Dive
  - Cooldown increased to 14 / 13 / 12 / 11 / 10 seconds from 12 / 11 / 10 / 9 / 8.
  - Mana cost increased to 65 from 50.

### V14.3
- Stats
  - Base armor increased to 47 from 45.
- Bone Skewer
  - Bonus AD ratio increased to 75% **bonus** AD from 60%.
  - Mana cost reduced to 70 / 75 / 80 / 85 / 90 from 74 / 78 / 82 / 86 / 90.
- Ghostwater Dive
  - Base bonus movement speed increased to 45% from 40%.
  - Movement speed lethality ratio increased to 2% per 1 Lethality from $1.5$% per 1 Lethality.
- Phantom Undertow
  - Base damage changed to 100 / 150 / 200 / 250 / 300 from 105 / 145 / 185 / 225 / 265.

### V13.19
- Stats
  - Health growth increased to 110 from 104.
- Ghostwater Dive
  - Cooldown reduced to 12 / 11 / 10 / 9 / 8 seconds from 12 / 11.5 / 11 / 10.5 / 10.

## Trivia

- This champion has no ability power ratio.
- Pyke's name is a pun on the Pike (weapon) and the Esox, both from Proto-Indo-European languages root **(s)pey-* "sharp, pointy stick".
- Using the *Toggle* key bind (default [Ctrl] + [5]) will display Pyke's critical strike animation on his next attack.
- Bone Skewer can be toggled to stop glowing with the *Toggle* key bind (default [Ctrl] + [5]). It will revert to glowing at 1:00 or whenever he respawns.
- Pyke's dance references the traditional Māori people Haka warrior dance.
  - A side-by-side comparison can be seen here.
  - He shares this dance with Rengar and K'Sante.
- Bone Skewer’s shape is possibly based on those of harpoon-heads carved from bone, antler, & walrus ivory, though his is made of metal and magnified to be effective against gigantic in-universe sea-monsters.
- Pyke is one of the two champions in the game whose health can not be improved except through growth per level, the other being Kled.
- Pyke quite coincidentally shares his name with a in George R.R. Martin's novel series A Song of Ice and Fire.
  - Namely, *Pyke*, the capital of the Iron Islands, an island state within the continent of Westeros; *Pyke*’s inhabitants also worship a deity called "the Drowned God".
- Gift of the Drowned Ones grants 1 **bonus** movement speed to Pyke while in river.
- Gift of the Drowned Ones is 93.75% gold efficient.
  - 100% efficiency would be 8 AD for every 105 HP.

---
*This page was automatically generated from League of Legends Wiki data.*