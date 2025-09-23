# Skarner

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
| **Champion** | Skarner |
| **Title** | the Primordial Sovereign |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2011-08-09 |
| **Release Patch** | V1.0.0.123 |
| **Latest Changes** | V25.09 |
| **Roles** | Vanguard, Juggernaut |
| **Riot Positions** | Jungle, Top |
| **External Positions** | Jungle, Top |
| **Blue Essence** | 2400 |
| **Riot Points** | 880 |
| **Difficulty** | 2 |
| **Hero Type** | Tank |
| **Alt Type** | Fighter |
| **Adaptive Type** | Physical |
| **Damage** | 2 |
| **Toughness** | 3 |
| **Control** | 3 |
| **Mobility** | 2 |
| **Utility** | 1 |
| **Style** | 45 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $630.0$ | $+110.0$ |
| **Mana** | $320.0$ | $+40.0$ |
| **Health Regen** | $7.5$ | $+0.75$ |
| **Mana Regen** | $7.2$ | $+0.75$ |
| **Armor** | $33.0$ | $+4.5$ |
| **Magic Resist** | $32.0$ | $+2.05$ |
| **Attack Damage** | $63.0$ | $+3.0$ |
| **Attack Speed** | $0.625$ | |
| **Movement Speed** | $335.0$ | $+0.0$ |
| **Attack Range** | $150.0$ | $+0.0$ |
| **Base Attack Speed** | $0.625$ | |
| **Attack Speed Ratio** | $0.625$ | |
| **Bonus AS per Level** | $2.0\%$ | |
| **Attack Windup** | $18.7\%$ | |
| **Acquisition Radius** | $600$ units | |
| **Gameplay Radius** | $80$ units | |
| **Pathing Radius** | $50$ units | |
| **Selection Radius** | $130$ units | |
| **Selection Height** | $150$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Taken** | $100.0\%$ |

#### URF

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $110.0\%$ |

## Abilities

### Passive: Threads of Vibration

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | Special |
| **Spell Effects** | proc |
| **Parry** | Special |

**INNATE:** **Skarner**’s basic attacks on-hit, *Shattered Earth*, *Upheaval*, and *Impale* apply a stack of *Quaking* to enemies hit for 4 seconds, refreshing on subsequent applications and stacking up to 3 times. Enemies afflicted with 3 stacks take magic damage equal to

**Notes:**

- Basic attacks that are dodged by the target or missed while **Skarner** is blinded do not apply stacks of *Quake*. Blocked attacks will still apply stacks.
- Spell shield prevents *Quaking*’s application from Upheaval and Impale only.

---

### Q: Crystal Slash

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 350 units |
| **Cost** | 10 Mana |
| **Cooldown** | 3.5 / 3.25 / 3 / 2.75 / 2.5 seconds |
| **Targeting** | Auto |
| **Affects** | Enemies, Self |
| **Damage Type** | Physical Magic |
| **Spell Shield** | Special |
| **Spell Effects** | spellaoe |

**ACTIVE:** **Skarner** slashes around himself, dealing physical damage to nearby enemies. If at least one enemy is hit, he becomes charged for 5 seconds, empowering subsequent casts of *Crystal Slash* to deal **bonus** magic damage.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 1 / 1.5 / 2 / 2.5 / 3% of target's **maximum** health (+ 20% AD) |

| Attribute | Value |
|-----------|------:|
| **Bonus Magic Damage** | 1 / 1.5 / 2 / 2.5 / 3% of target's **maximum** health (+ 20% AD) (+ 30% AP) |

Each instance of damage is capped at 200 against epic monsters.

Basic attacks reduce *Crystal Slash*’s **current** cooldown by $0.25$ seconds, increased to 1 second against champions.

**Scaling:**
| Attribute | Value |
|-----------|------:|
| **Total Mixed Damage** | 2 / 3 / 4 / 5 / 6% of target's **maximum** health (+ 40% AD) (+ 30% AP) |

**Notes:**

- Spell shield will block the damage but will not prevent **Skarner** from triggering the bonus effect.
- Basic attacks against structures will not trigger the cooldown reduction.
- As already noted, each damage instance is capped on its own, meaning that the maximum possible damage against an epic monster with one cast is 400.
- In order to hit the cap, the target needs to have at least (200 to 200)/((1 to 3)/100) health (without considering the ratios).

---

### Q: Shattered Earth

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.35$ seconds |
| **Effect Radius** | 300 (Attack splash radius, centered on target) units |
| **Cost** | 45 mana |
| **Cooldown** | 13 / 11.5 / 10 / 8.5 / 7 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | False |
| **Spell Effects** | proc |
| **Parry** | True |

**ACTIVE:** **Skarner** a boulder from the ground, empowering up to three of his next basic attacks within 5 seconds of each other. These attacks gain range, **bonus** attack speed, and deal **bonus** physical damage to the target and surrounding enemies, including structures.

| Attribute | Value |
|-----------|------:|
| **Bonus Physical Damage per Hit** | 10 / 20 / 30 / 40 / 50 (+ 80% **bonus** AD) (+ 3% of his **bonus** health) |
| **Total Bonus Physical Damage** | 30 / 60 / 90 / 120 / 150 (+240% **bonus** AD) (+ 9% of his **bonus** health) |

| Attribute | Value |
|-----------|------:|
| **Bonus Attack Speed** | 20 / 25 / 30 / 35 / 40% |

The final attack slams the boulder, dealing an additional 8% of each target's **maximum** health physical damage and slowing afflicted enemies by 40% for 1 second. The additional damage is capped against monsters and is not applied against structures.

| Attribute | Value |
|-----------|------:|
| **Monster Percent Health Damage Cap** | 150 / 200 / 250 / 300 / 350 |

After $0.5$ seconds of the cast, **Skarner** gains the ability to use *Upheaval* during *Shattered Earth*.

*Shattered Earth's duration is paused during Ixtal's Impact.*

**Notes:**

- The empowered attacks can be parried and will not trigger the area of effect if either the target blocks or dodges the attack, or if the attacks miss while **Skarner** is blinded.
  - The attacks will be still consumed in all cases regardless.
  - Note that if the main target is not blocking or dodging, then the area of effect can still deal damage to those who do.
- Casting *Shattered Earth* resets **Skarner**’s basic attack timer.
- The empowered attacks have an uncancellable windup against champions, minions, and turrets only.
  - Prior to Patch 14.9, the windup was uncancelable regardless of the target's type. It is currently unknown if the change was intentional or not, but seems to be partially intentional based on observation.
- The afformentioned unit types and inhibitors are considered "valid" for *Shattered Earth* and will only trigger its effects when the main target was one of those. This includes:
  - Triggering the damaging area of effect,
  - Refreshing the duration when an empowered attack has begun,
  - and consume empowered attacks only when they landed. (This way the attacks are not consumed when the target died beforehand.)
- Attacking any other units will cause **Skarner** to use his normal basic attacks instead while *Shattered Earth* is expiring (but still benefit from the increased attack speed and range).
- To be more precise about *Ixtal's Impact*’s duration pause, it adds $0.25$ seconds to the duration every $0.25$ seconds, but will set it to an estimated minimum of $1.5$ seconds if it would have been lower.
- The slow applied by *Shattered Earth* is a generic slow debuff. ;Interactions & Other
- The attack reset will not save time if **Skarner** possesses sufficiently high attack speed because of the ability's $0.35$ seconds cast time. Starting from around 271.4285% **bonus** attack speed, basic attacking is fast enough to match the time with the reset and will be even faster with more, causing *Shattered Earth* to actually slow down the user at that point.
  - However, this only takes attack speed into account and ignores various other factors, such as the damage and other bonuses from the ability and the fact that **Skarner** generally does not like building such high amounts of attack speed. Additionally, attempting to do the attack reset at such high attack speed is already a hard task, if not requires frame perfect execution to save any ticks worth of time. ;Known issues
- **Skarner**’s attack speed is not immediately updated upon casting *Shattered Earth*. Beginning the first attack shortly after casting results in an attack that does not benefit from the attack speed granted by the ability.
- If *Upheaval* hits an enemy while a new instance of *Shattered Earth* is active, *Shattered Earth*’s effects are immediately lost.
- While *Shattered Earth* is active, it will not benefit from cooldown refund effects such as Transcendence or Navori Flickerblade Transcendence.
  - This is not a **Skarner**-specific issue. Various abilities, including Ultimates, that grant a different ability in their slot after activation may suffer from this problem (e.g. Blood Frenzy and Snack Attack) and may be fixed or may have been fixed on a case-by-case basis.
- The empowered attacks lack spell effects as they deal proc damage, which does not match with how empowered attacks supposed to behave in general.
  - This is possibly a band-aid fix to avoid an issue where *Shattered Earth* would trigger various effects twice per attack instead of once, as it could be seen during the first PBE deploy of the rework's release, after which the ability was changed from spell damage to proc.
- The buff used for *Shattered Earth* is fuzzy. Each attack removes 1 stack on a delay of up to $0.25$ seconds. This has no known effects currently.
- The slow is also applied and removed in a fuzzy manner, meaning that sometimes it may last shorter or longer than intended, depending on the ticks.

---

### Q: Upheaval

| Attribute | Value |
|-----------|------:|
| **Range** | 1050 units |
| **Cast Time** | $0.25$ / none (If cast because of Impale) |
| **Effect Radius** | 300 (Boulder splash radius, centered on target) units |
| **Width** | 180 units |
| **Speed** | 1600 units/second |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | Special |
| **Spell Effects** | spellaoe |
| **Projectile** | True |

**ACTIVE:** **Skarner** ends *Shattered Earth* by throwing his boulder in the target direction. It explodes upon colliding with the first enemy hit, applying the same damage and slow to the target and surrounding enemies as the final attack would.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 10 / 20 / 30 / 40 / 50 (+ 80% **bonus** AD) (+ 3% of his **bonus** health)
(+ 8% of target's **maximum** health) |

| Attribute | Value |
|-----------|------:|
| **Maximum Monster Damage** | 160 / 220 / 280 / 340 / 400 (+ 80% **bonus** AD) (+ 3% of his **bonus** health) |

**Notes:**

- While *Upheaval* can deal damage to structures, the missile can't directly hit them. They have to be within the radius of the explosion.
- Spell shield will not prevent the explosion from occurring.Effect at cast time end
  - The direction of cast will be updated to go toward the cast location.
- The slow applied by *Shattered Earth* is a generic slow debuff. ;Known Issues
- If *Upheaval* hits an enemy while a new instance of *Shattered Earth* is active, *Shattered Earth*’s effects are immediately lost.

---

### W: Seismic Bastion

| Attribute | Value |
|-----------|------:|
| **Cast Time** | 0.3 seconds |
| **Effect Radius** | 600 (Radial missile range within an arc forward, increasing towards the edges of this angle) / 700 (Radial missile range at every other angle) units |
| **Speed** | 1650 (Radial missile speed) units/second |
| **Cost** | 60 / 65 / 70 / 75 / 80 mana |
| **Cooldown** | 10 / 9 / 8 / 7 / 6 seconds |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | Magical |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | False |

**ACTIVE:** **Skarner** slams his claws into the ground, shielding himself equal to 8% of his **maximum** health for $2.5$ seconds and releasing a shockwave that quickly (~0.25 seconds) expands in a radius around him to deal magic damage to enemies hit and slow them by 20% for 1 second.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 50 / 70 / 90 / 110 / 130 (+ 80% AP) |

***Skarner** can move during Seismic Bastion's cast time.*

**Notes:**

- The shield is granted at the start of the cast time.
- The shield ratio is equal to 0.08*( (+ 8% of his **bonus** health). Effect at cast time end

---

### E: Ixtal's Impact

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Collision Radius** | 160 (Grab range) units |
| **Cost** | 50 / 55 / 60 / 65 / 70 mana |
| **Cooldown** | 22 / 21 / 20 / 19 / 18 seconds |
| **Cooldown Start** | post-effect |
| **Targeting** | Direction |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | spell |
| **Grounded** | True |
| **Knockdown** | False |
| **Silence** | True |

**ACTIVE:** **Skarner** charges forward in the direction of the cursor for up to $2.75$ seconds, during which he gains slow immunity, becomes ghosted, ignores terrain collision, and has unobstructed vision of the surrounding 650 units. He automatically navigates his movement along the way and is able to steer himself with a slow turn rate that increases over time, resetting upon changing direction. *Ixtal's Impact* can be recast after 1 second, and does so automatically after the duration.

**Skarner** begins the charge at a static amount (Cannot be modified by any means) of 150 movement speed that increases by 100 every , up to 950 **total** movement speed by the end of the duration. If **Skarner** collides with an enemy champion or large monster, he grabs them with his claws for the time of the remaining charge, attaching them to himself, suppressing them, revealing them, increasing the charge's turn rate to the maximum, and gaining 300 **bonus** movement speed, with a minimum total of 650 and a maximum total of 1150. If a target has been grabbed before the first $1.75$ seconds of the charge, the remaining duration is set to 1 second, and set to $0.5$ seconds otherwise. The charge does not ramp up in speed while a target is grabbed.

If **Skarner**’s attached target collides with terrain, the charge ends, detaching them from him, dealing physical damage, stunning them for $1.1$ seconds, and reducing *Ixtal's Impact*’s cooldown by 35%.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 30 / 60 / 90 / 120 / 150 (+ 6% of his **maximum** health) |

**RECAST:** **Skarner** ends *Ixtal's Impact*. If a target has been grabbed, he detaches them from himself.

*Ixtal's Impact ends immediately if *’Skarner** becomes immobilized, grounded, or silenced during the charge, detaching the grabbed target. While the target is grabbed, they are attached to **Skarner** 200 units in front of him.*

**Notes:**

- While *Ixtal's Impact* is active, **Skarner**’s movement speed is set to a static amount that cannot change by any means other than by the ability itself.
  - Any movement speed modifiers gained before or during the charge are retained however, and will take effect after the charge if the duration permits.
- The **bonus** movement speed from grabbing a target is 300 only if **Skarner**’s movement speed was between 350 and 850 at the time.
  - If **Skarner** was at less than 350 movement speed when he grabbed the target, his total movement speed is instead set to 650. If he was at 950 movement speed, his total movement speed is set to the cap of 1150.
- **Skarner** gains a slightly larger field of view during *Ixtal's Impact*’s charge.
- *Ixtal's Impact*’s attachment depends on the application of the suppression; if the suppression is not applied, neither is the attachment. Similarly, if the suppression is removed, so is the attachment.
  - If the target resists the suppression through total crowd control immunity, displacement immunity, or a spell shield. **Skarner** will not attach them to himself, causing the charge to end instantly. The collision's damage is special-cased to still apply to a target under these effects immediately upon contact.
  - If the target removes the suppression by any means, including with an applicable cleanse effect or dispel, they will detach themselves from **Skarner** immediately which forces the charge to end instantly and not apply any damage.
- **Skarner** can move up to 550 units beyond the outer-edge of the battlefield with the charge.
  - Once he has surpassed this boundary, *Ixtal's Impact* ends immediately and **Skarner** is moved to the nearest valid space.
- *Ixtal's Impact* can interact with player-generated terrain.
- If **Skarner** is inside terrain when the effect ends, he will be moved to the nearest valid space.
- Enemies cannot see **Skarner** while he is inside terrain, unless they grant sight into the area of terrain he is inside (such as with Hawkshot or Heightened Senses) or have unobstructed vision.
  - However, enemies will be notified when **Skarner** is inside nearby terrain while within a certain distance of him. An indicator will be visible to the enemy player's perspective to represent this.
- The following table refers for interactions while **Skarner** is charging:
- A special indicator for *Ixtal's Impact*’s direction and expected ending location is visible to **Skarner** only during his charge.

---

### R: Impale

| Attribute | Value |
|-----------|------:|
| **Range** | -50 – 625 (Height placement relative to Skarner, 675 total length) units |
| **Cast Time** | $0.65$ seconds |
| **Width** | 350 (Larger base) / 200 (Smaller base) units |
| **Cost** | 100 mana |
| **Cooldown** | 120 / 112.5 / 105 / 97.5 / 90 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Direction |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | False |

**ACTIVE:** **Skarner** winds up his stingers over the cast time, then lashes them forward in an isosceles trapezoid in the target direction, dealing magic damage to enemies hit and impaling up to 3 of the closest enemy champions within the area to suppress them for $1.5$ seconds. While suppressed, the targets are revealed and attached to **Skarner**. If he successfully impales at least one enemy champion, **Skarner** gains 40% **bonus** movement speed for the same duration.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 150 / 200 / 250 / 300 / 350 (+ 100% AP) |

*If Impale is cast during Shattered Earth, *’Skarner** automatically casts (Without incurring Upheaval's cast time) Upheaval in the direction of Impale's cast. **Skarner** is unable to basic attack, cast Shattered Earth or Ixtal's Impact, or use Flash during Impale. While the targets are impaled, they are attached to **Skarner** 300 units in front of him (In the direction he cast Impale). Targets are placed 200 units in the same direction at the end of the effect.*

**Notes:**

- *Impale*’s attachment depends on the application of the suppression; if the suppression is not applied, neither is the attachment. Similarly, if the suppression is removed, so is the attachment.
  - If a target resists the suppression by being immune to crowd control, displacement immune, or having a spell shield, **Skarner** will not attach them to himself.
    - If all targets hit resist the suppression, **Skarner** will fail to attach any of them to himself and thus will not initiate *Impale*, causing the ability to behave as if no valid impale targets were hit.
  - If a target removes the suppression by any means, including with an applicable cleanse effect or dispel, they will detach themselves from **Skarner** immediately.
    - If all targets remove the suppression, *Impale* ends prematurely and is cancelled for **Skarner**.
- If a target would be inside of terrain at the end of the effect, they are moved to the nearest valid space.
- If a target would be on the other of a wall, they are moved to the same as **Skarner**.
- During *Impale*, **Skarner** always faces in the direction of the nearest attached target. Effect at cast time end
- The following table refers for interactions while **Skarner** is in cast time:
- The following table refers for interactions while **Skarner** is performing *Impale*:

---

## Patch History

### V25.09
- Shattered Earth
  - ***UNDOCUMENTED - BUG FIX:*** Now is properly tagged as an attack reset for the purposes of Hail of Blades.
- Upheaval
  - ***UNDOCUMENTED - BUG FIX:*** Slow no longer fails to apply when its damage is mitigated by a shield.
- Ixtal's Impact
  - ***UNDOCUMENTED - BUG FIX:*** Now properly refreshes the duration of *Shattered Earth* on-cast as well, rather than only periodically during the charge. Previously, if *Shattered Earth* expired up to $0.25$ seconds into the charge, the effect would be lost rather than refreshed.

### V25.05
- Shattered Earth
  - Final attack target health ratio reduced to 8% of target's **maximum** health from 10%.
  - Slow duration reduced to 1 second from $1.25$.
- Upheaval
  - Target health ratio reduced to 8% of target's **maximum** health from 10%.
- Ixtal's Impact
  - Cooldown increased to 22 / 21 / 20 / 19 / 18 seconds from 20 / 19 / 18 / 17 / 16.

### V25.S1.3
- Stats
  - Armor growth reduced to $4.5$ from $4.8$.
- Ixtal's Impact
  - Stun duration reduced to $1.1$ seconds from $1.5$.
- Impale
  - **Bug Fixes:** Now properly detaches enemies from him when his False Life is triggered.
  - **Bug Fixes:** If his False Life is triggered, the ability's cast time now properly cancels.

### V14.23
- Shattered Earth
  - Health ratio per hit reduced to 3% of his **bonus** health from 4%.
- Upheaval
  - Bonus health ratio reduced to 3% of his **bonus** health from 4%.

### V14.20
- Threads of Vibration
  - Health ratio reduced to 5 to 9 of target's **maximum** health from 5 to 11.
- Ixtal's Impact
  - Health ratio reduced to 6% of his **maximum** health from 8%.

### V14.18
- Ixtal's Impact
  - **Bug Fixes:** Stun now properly disables the target's actions.

### V14.16
- Stats
  - Base health increased to 630 from 610.
  - Base attack damage increased to 63 from 60.
- Shattered Earth
  - Bonus AD ratio increased to 80% **bonus** AD from 60%.
- Upheaval
  - Bonus AD ratio increased to 80% **bonus** AD from 60%.

### V14.15
- Stats
  - Health growth increased to 110 from 105.
- Shattered Earth
  - Mana cost reduced to 45 from 50.
  - First empowered attack after casting the ability now declares the automatic basic attack more quickly and smoothly.
- Ixtal's Impact
  - **Removed:*** Charge no longer collides with targets directly behind him.
    - Collision box is now offset significantly further in front of him.
  - **Bug Fixes:** Champions are no longer able to act during the beginning of the stun.

### V14.13
- Shattered Earth
  - Mana cost increased to 50 from 30.
  - Target health ratio reduced to 10% of target's **maximum** health from 15%.
- Upheaval
  - Target health ratio reduced to 10% of target's **maximum** health from 15%.<!--
- Ixtal's Impact
  - **Bug Fixes:** Now is properly canceled by Facebreaker.-->

### V14.12
- Stats
  - Base health reduced to 610 from 650.
  - Health growth increased to 105 from 102.
- Threads of Vibration
  - Health ratio changed to 5 to 11 of target's **maximum** health from 7 to 10.

## Trivia

- During the cast time, Impale’s decal displays the Ixtal Axiomata.
- His dance is a reference to Crab Rave. This dance can also be synchronized with other Skarners.
  - A side-by-side comparison can be seen here.
- Some animations resemble his pre-rework ones:
  - He has kept the same pose for his joke emote.
  - Part of the old dance where he swings his claws in a circle can be shown during the restart segment of his current dance loop.
  - His previous two 'slash' animations (one for the left claw, and one for the right) used during Crystal Slash’s cast resemble part of his laugh emote.
- Skarner was deemed overpowered and obnoxious in Ultra Rapid Fire (2014 edition) and was ultimately disabled in non-custom games.
- Skarner had his game assets reused for several featured game modes!
  - He was given a retexture and features as a monster called the "Draggy Tail Monster" in Invasion.
    - Skarner is the only unit that was retextured for this mode that isn't a Voidborn.
  - His animations were reused for the Skarner and Skarner monsters in Odyssey: Extraction.

---
*This page was automatically generated from League of Legends Wiki data.*