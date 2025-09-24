# Lee_Sin

## Table of Contents
- [Basic Information](#basic-information)
- [Abilities](#abilities)
- [Patch History](#patch-history)
- [Trivia](#trivia)

## Basic Information

| Attribute | Value |
|-----------|------:|
| **Champion** | Lee Sin |

## Abilities

### Passive: Flurry

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Self |
| **Parry** | Special |

**INNATE:** After casting an ability, **Lee Sin**’s next 2 basic attacks within 3 seconds gain (attack speed) 40% **bonus** attack speed and restore energy.

The first attack restores 20–40@1–13 energy and the second attack restores「 10–20@1–13 energy. ⟷ half of the previous amount. 」

**Notes:**

- *Flurry* will still restore energy even if the attacks are blocked or missed while **Lee Sin** is blinded, but not dodged.
- Energy restore is on-hit or on-attack.

---

### Q: Resonating Strike

| Attribute | Value |
|-----------|------:|
| **Range** | 1250 (Cast range) units |
| **Cast Time** | none |
| **Speed** | 1350 + units/second |
| **Cost** | 25 Energy |
| **Targeting** | Proximity |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | Spell |
| **Grounded** | True |
| **Knockdown** | True |

**ACTIVE:** **Lee Sin** dashes to the nearby enemy marked by *Sonic Wave*. Upon arrival, he consumes the mark and deals physical damage, increased by 0%–100%@0–100 (@=target's **missing** health).

| Attribute | Value |
|-----------|------:|
| **Minimum Physical Damage** | 55 / 80 / 105 / 130 / 155 (+ 115% **bonus** AD) |
| **Maximum Physical Damage** | 110 / 160 / 210 / 260 / 310 (+ 230% **bonus** AD) |

*The marked enemy must be within range in order to cast this ability. Iron Will and Cripple can be cast during the dash.*

**Notes:**

- **Lee Sin** will track the target if they change locations.
  - He will dash to the target's previous location without consuming the mark if the target is too far away or moves beyond 2000 units.
- **Lee Sin** will attempt to place himself on top of the target upon arrival.
  - The damage dealt by *Resonating Strike* can be negated if the target blinks. Sometimes, **Lee Sin** will end the dash at their last location but appear on top of the target after a short delay.
  - Originally, **Lee Sin** would land 50 units in front of his target. Since there are no patch notes where this is noted to have changed, landing on top of his target may be a bug.
- *Resonating Strike* cannot be cast without the marked target nearby or if the target is untargetable.
- If *Resonating Strike* interrupts another dash, Tempest and Dragon's Rage can be cast during it (both otherwise cannot be cast).

---

### Q: Sonic Wave

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 1200 units |
| **Effect Radius** | 400 (Reveal radius; does not see through terrain or brush) units |
| **Width** | 120 units |
| **Speed** | 1800 units/second |
| **Cost** | 50 Energy |
| **Cooldown** | 10 / 9 / 8 / 7 / 6 (Starts on first cast) seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | Spell |
| **Projectile** | True |

**ACTIVE:** **Lee Sin** fires a sonic blast in the target direction that deals physical damage to the first enemy hit and marks them for 3 seconds, during which they are revealed.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 55 / 80 / 105 / 130 / 155 (+ 115% **bonus** AD) |

While the target is marked, **Lee Sin** can cast *Resonating Strike*.

**Notes:**

No additional notes.

---

### W: Iron Will

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Cost** | 25 Energy |
| **Targeting** | Auto |
| **Affects** | Self |

**ACTIVE:** **Lee Sin** gains life steal and spell vamp for 4 seconds.

| Attribute | Value |
|-----------|------:|
| **Life steal and spell vamp** | 10 / 14 / 18 / 22 / 26% |

**Notes:**

- No additional details.

---

### W: Safeguard

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 700 units |
| **Speed** | 1350 + 100% movement speed |
| **Cost** | 50 Energy |
| **Cooldown** | 12 (Starts on first cast) seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Unit |
| **Grounded** | True |
| **Knockdown** | True |

**ACTIVE:** **Lee Sin** dashes to the target allied unit's location. If the ally is a champion upon arrival, both they and **Lee Sin** gain a shield for 2 seconds and *Safeguard*’s cooldown is halved.

**Lee Sin** can self-cast *Safeguard* to shield himself.

| Attribute | Value |
|-----------|------:|
| **Shield Strength** | 70 / 115 / 160 / 205 / 250 (+ 80% AP) |

After *Safeguard* is cast, **Lee Sin** can cast *Iron Will* within the next 3 seconds.

*Iron Will and Cripple can be cast during the dash.*

**Notes:**

- **Lee Sin** cannot self-cast *Safeguard* while grounded or rooted.
- *Safeguard* can be used on any targetable allied unit excluding structures.
  - This includes champions, minions, wards, and champion summoned units (e.g. Noxious Trap, Demacian Standard, Dark Passage).
- If no unit is directly targeted, *Safeguard* will search for wards in a 100 radius to target.
  - This **Lee Sin** in performing the dash to a ward he just placed.
- *Safeguard* will not grant its shield if the dash is interrupted.
  - If the dash overrides or is overridden by Resonating Strike, he will grant the shield immediately rather than on arrival.
- If *Safeguard*’s dash interrupts another dash, Tempest and Dragon's Rage can be cast during it (both otherwise cannot be cast).

---

### E: Cripple

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 550 (Cast requirement) / 600 (Effect radius when cast) units |
| **Speed** | 1600 (Wave missile speed) units/second |
| **Cost** | 25 energy |
| **Targeting** | Proximity |
| **Affects** | Enemies |
| **Spell Shield** | True |

**ACTIVE:** **Lee Sin** slows nearby enemies marked by *Tempest*, decaying over 4 seconds.

| Attribute | Value |
|-----------|------:|
| **Slow** | 35 / 45 / 55 / 65 / 75% |

*A nearby marked enemy is required to cast this ability.*

**Notes:**

- *Tempest*’s reveal is not removed when **Lee Sin** casts *Cripple*.
- *Cripple* cannot be cast without a marked target within 550 range.

---

### E: Tempest

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Effect Radius** | 450 units |
| **Cost** | 50 Energy |
| **Cooldown** | 8 (Starts on first cast) seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |

**ACTIVE:** **Lee Sin** smashes the ground beneath him, dealing magic damage to nearby enemies, marking them for 4 seconds and, if they are not invisible when struck, revealing them for the same duration.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 35 / 60 / 85 / 110 / 135 (+ 100% AD) |

If *Tempest* hits an enemy, **Lee Sin** can cast *Cripple* within the next 3 seconds after a $0.1$-second delay.

**Notes:**

- *Tempest*’s sight reveal only reveals the affected targets, but visually also lights up the Fog of War in a 400 radius.
- *Tempest*’s sight reveal is not removed when *Lee Sin* casts *Cripple* (unlike Sonic Wave’s). Effect at cast time end

---

### R: Dragon's Rage

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 375 units |
| **Collision Radius** | 160 (Checks for collision every 0.25 seconds, estimated) units |
| **Cooldown** | 110 / 97.5 / 85 / 72.5 / 60 seconds |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | Special |
| **Call For Help** | True |

**ACTIVE:** **Lee Sin** roots the target enemy champion over the cast time, then roundhouse kicks them to deal physical damage and knock them back up to 800 units (see notes) over $0.8$ seconds (Estimated), rendering them airborne for 1 second.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 175 / 287.5 / 400 / 512.5 / 625 (+ 200% **bonus** AD) |

Enemies that collide with the displaced enemy while it is airborne take the same damage plus **bonus** physical damage and are knocked up for 1 second.

| Attribute | Value |
|-----------|------:|
| **Collision Physical Damage** | 175 / 287.5 / 400 / 512.5 / 625 (+ 200% **bonus** AD) (+ 12 / 13.5 / 15 / 16.5 / 18% of primary target's **bonus** health) |

**Notes:**

- Applies spell damage to the primary target and area damage to secondary targets.
- The spell indicator for this ability also displays the direction for the knock back relative to **Lee Sin**’s position.
- If the target of *Dragon Rage* dies initially on the first hit there will be no displacement of any kind.
- The displacement's direction is decided at the moment of impact and not by **Lee Sin** and his target's relative locations at the start of the cast (the only way for **Lee Sin** to control where his target is displaced is by using Flash before the cast time of *Dragon's Rage* ends).
- If the airborne debuff is removed from the target, its ability to collide with enemies is removed aswell, even if the target has not come to a stop yet.
  - The airborne debuff and with it collision of the target remains for the normal duration even if its trajectory is displaced by another source (i.e - Headbutt).
  - The target may still collide with other enemies for the remaining airborne duration after they reach their destination, as the airborne debuff lasts longer than the displacement.
- The knockback distance is intended to be 800, but due to a bug, this is reduced by up-to the sum of **Lee Sin**’s and his target's gameplay radius, depending on how far they are away from one another at the end of the cast time.

---

## Patch History

### V25.14
- Stats
  - Base attack damage reduced to 66 from 69.

### V25.12
- Tempest
  - Cooldown reduced to 8 seconds from 9.
- Cripple
  - Slow changed to 35 / 45 / 55 / 65 / 75% from 20 / 35 / 50 / 65 / 80%.

### V14.20
- Stats
  - Base attack damage increased to 69 from 66.

### V14.14
- Sonic Wave
  - **Bug Fixes:** No longer incorrectly reveals nearby wards and traps.

### V14.13
- Stats
  - Health growth increased to 108 from 105.
- Sonic Wave
  - **New Effect:** Now grants vision in a small radius around the target.
- Safeguard
  - Base shield increased to 70 / 115 / 160 / 205 / 250 from 50 / 100 / 150 / 200 / 250.

### V14.10
- Dragon's Rage
  - **Bug Fixes:** No longer repeats its damage on every frame that the missile collides with another enemy unit which has crowd control immunity.

### V14.9#May 2nd Hotfix|V14.9
- Resonating Strike
  - Bonus AD ratio reduced to 115% **bonus** AD from 200%.

### V14.9
- General
  - Updated model and animations for all skins.
  - New splash artwork for Lee Sin, Lee Sin, Lee Sin, Lee Sin, and Lee Sin.
  - Adjusted splash artwork for Lee Sin, Lee Sin, Lee Sin, Lee Sin, Lee Sin, Lee Sin, Lee Sin, Lee Sin, Lee Sin, and Lee Sin.
  - New lore.
  - New voice-over.
  - Updated sound effects.
  - Lee Sin cost increased to from .
  - Lee Sin cost increased to from .
- Resonating Strike
  - Bonus AD ratio increased to 200% **bonus** AD from 115%.
- Cripple
  - **New Effect:** Now applies the debuff to Tempest’s targets directly.
    - **Removed:*** No longer apples the debuff through an (unblockable) targeted missile.
  - **Bug Fixes:** An inconsistency with the debuff's application that caused it to overcompensate its value has been resolved, which would previously grant targets a positive bonus at the last stat update tick.

### V13.19
- Stats
  - Base armor increased to 36 from 34.
- Iron Will
  - Life steal and spell vamp changed to 10 / 14 / 18 / 22 / 26% from 5 / 10.5 / 16 / 21.5 / 27%.
- Tempest
  - Base damage reduced to 35 / 60 / 85 / 110 / 135 from 35 / 65 / 95 / 125 / 155.

### V13.18
- General
  - Updated ability icons.

## Trivia

- Lee Sin was one of the first champions designed (the others being Annie, Singed, Sion, Sivir, and Twisted Fate)
  - Lee Sin is the only allegedly canceled champion reintroduced with his original name.
  - Lee Sin is the first champion to be redesigned and to be technically given a relaunch skin (Lee Sin), although it was done prior to his actual release.
  - Lee Sin is the first champion to feature two 'Champion Spotlights' (an April Fools' Day one and a real one).
- Lee Sin is the second of eight champions to have more than four abilities (the others being Elise, Gnar, Heimerdinger, Jayce, Karma, Nidalee, and Rek'Sai).
- Lee Sin is the first monk type champion announced but the second to be released (after Udyr).
- Lee Sin is the first non-ninja Ionian champion to use energy.
- Lee Sin's dance references Shaolin Soccer.
  - A side-by-side comparison can be seen here.
- An inside joke is that Lee Sin was taught Dragon's Rage by Jesse Perring (yet the Blind Monk does not kick players off the map).
- Lee Sin's Series 1 Eternals make the following references:
  - *Bicycle Kick* associated with the Bicycle kick in soccer, known to return the trajectory of a complicated motion to the moving object.
- Lee Sin's Series 2 Eternals make the following references:
  - *Yaa-KUH* is a reference to his **"HIKUH!"** cry, becoming a community meme-status some time later.

---
*This page was automatically generated from League of Legends Wiki data.*