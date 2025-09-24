# Teemo

## Table of Contents
- [Basic Information](#basic-information)
- [Statistics](#statistics)
  - [Map-specific Stats](#map-specific-stats)
- [Pets](#pets)
- [Abilities](#abilities)
- [Patch History](#patch-history)
- [Trivia](#trivia)

## Basic Information

| Attribute | Value |
|-----------|------:|
| **Champion** | Teemo |
| **Title** | the Swift Scout |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2009-02-21 |
| **Release Patch** | Alpha Week 2 |
| **Latest Changes** | V25.18 |
| **Roles** | Specialist |
| **Riot Positions** | Top |
| **External Positions** | Top, Jungle, Support |
| **Blue Essence** | 225 |
| **Riot Points** | 260 |
| **Difficulty** | 1 |
| **Hero Type** | Marksman |
| **Alt Type** | Mage |
| **Adaptive Type** | Magic |
| **Damage** | 3 |
| **Toughness** | 1 |
| **Control** | 2 |
| **Mobility** | 1 |
| **Utility** | 2 |
| **Style** | 30 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $615.0$ | $+104.0$ |
| **Mana** | $334.0$ | $+25.0$ |
| **Health Regen** | $5.5$ | $+0.65$ |
| **Mana Regen** | $9.6$ | $+0.45$ |
| **Armor** | $24.0$ | $+4.95$ |
| **Magic Resist** | $30.0$ | $+1.3$ |
| **Attack Damage** | $54.0$ | $+3.0$ |
| **Attack Speed** | $0.690$ | |
| **Movement Speed** | $330.0$ | $+0.0$ |
| **Attack Range** | $500.0$ | $+0.0$ |
| **Base Attack Speed** | $0.69$ | |
| **Attack Speed Ratio** | $0.69$ | |
| **Bonus AS per Level** | $3.4\%$ | |
| **Missile Speed** | $1500$ units/second | |
| **Acquisition Radius** | $500$ units | |
| **Gameplay Radius** | $55$ units | |
| **Pathing Radius** | $30$ units | |
| **Selection Radius** | $100$ units | |
| **Selection Height** | $100$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $95.0\%$ |
| **Damage Taken** | $110.0\%$ |

#### URF

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $90.0\%$ |
| **Damage Taken** | $100.0\%$ |

## Pets

### Mushroom Trap

| Attribute | Value |
|-----------|------:|
| **Range** | 75 (Collision radius) / 450 (Blast radius) units |
| **Health** | 6 |
| **Control** | Autonomous (static) |
| **Targeting** | Ward |
| **Spell Effects** | A mushroom's poison applies spell effects as persistent area damage. |

**Abilities:**

- **Stealth Trap:** The mushroom trap stealths and becomes untargetable after 1 second, remaining vulnerable to trap-revealing effects at all times.
- **Ricochet:** If the mushroom trap lands on another mushroom after being placed, it bounces forward 300 / 400 / 500 units.
- **Poisonous Blast:** If an enemy walks over the mushroom trap, it creates a blast to slow all nearby enemies by 30 / 40 / 50% for 4 seconds and inflicting them with poison for the same duration. Poisoned enemies take 200 / 325 / 450 (+ 50% AP) **total** magic damage over the duration and are revealed.

---

## Abilities

### Passive: Guerrilla Warfare

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Self |

**INNATE:** **Teemo** gains invisibility after $1.5$ seconds without moving, taking damage, performing actions that break stealth, channeling, or being in stasis. **Teemo** will maintain stealth so long as he remains idle and is not displaced. While in brush, **Teemo** gains the stealth even while moving and can move without breaking stealth, but will lose the stealth in this case if he is knocked up.

**INNATE - ELEMENT OF SURPRISE:** When **Teemo** breaks the stealth, he gains 20%@1; 40%@5; 60%@10; 80%@15 **bonus** attack speed for 5 seconds.

**Notes:**

- *Element of Surprise*’s buff does not stack with itself, only refreshes on subsequent triggers within the duration.
- All forced action effects will break *Guerilla Warfare*’s stealth while **Teemo** is standing outside of brush. While he is inside brush, all of them will also break the stealth if he is *forced* to move outside brush.
  - Berserk and taunt can additionally break the stealth if **Teemo** is *forced* to use a basic attack, which is an action that breaks stealth by default.
- *Guerilla Warfare*’s stealth is special-cased to be interrupted whenever **Teemo** moves from his location by any means (Move order, dash, or blink), thus, performing actions that do not normally break stealth which also involve movement will cause the stealth to break. If **Teemo** is in a brush, however, the stealth is not broken by the above effects so long as his destination location is still inside brush.
- Due to the unique implementation of *Guerilla Warfare*’s stealth:
  - Some area-of-effect (AoE) knockups, may **not** be able to break **Teemo**’s stealth.
    - While **Teemo** is airborne without his stealth broken, other area-of-effect spells (mostly older ones) **are** able to break **Teemo**’s stealth, such as Randuin's Omen Humility.
  - A highly technical explanation can be found here.
- Using a basic attack breaks the stealth at the end of the attack windup.

---

### Q: Blinding Dart

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 680 units |
| **Speed** | 2500 units/second |
| **Cost** | 70 / 75 / 80 / 85 / 90 Mana |
| **Cooldown** | 7 seconds |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Single target |
| **Projectile** | True |
| **Call For Help** | True |

**ACTIVE:** **Teemo** shoots a dart at the target enemy that deals magic damage and blinds them for a duration.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 80 / 125 / 170 / 215 / 260 (+ 70% AP) |

| Attribute | Value |
|-----------|------:|
| **Blind Duration** | 2 / 2.25 / 2.5 / 2.75 / 3 seconds |

The duration of the blind is doubled against minions and monsters.

| Attribute | Value |
|-----------|------:|
| **Increased Blind Duration** | 4 / 4.5 / 5 / 5.5 / 6 seconds |

**Notes:**

- *Blinding Dart* will not make abilities that can trigger on-hit effects (Parrrley, Mystic Shot) miss.
- When blinded, enemies have a green tint on their screen.
- Because *Blinding Dart* uses center range, it has 45 to 70 more range than his basic attacks, which use edge range, against other champions.
  - This bonus becomes lower if **Teemo** or his target have bonuses.

---

### W: Move Quick

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Cost** | 40 mana |
| **Cooldown** | 14 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Auto |
| **Affects** | Self |

**PASSIVE:** **Teemo** gains (ms) **bonus** movement speed after 5 seconds without taking damage from enemy champions or turrets.

| Attribute | Value |
|-----------|------:|
| **Bonus Movement Speed** | 12 / 16 / 20 / 24 / 28% |

**ACTIVE:** **Teemo** doubles the **bonus** movement speed for 3 seconds, preventing it from being removed for the duration.

| Attribute | Value |
|-----------|------:|
| **Enhanced Bonus Movement Speed** | 24 / 32 / 40 / 48 / 56% |

*Casting Move Quick does not interrupt Guerrilla Warfare.*

**Notes:**

- When *Move Quick*’s active bonus movement speed ends **Teemo** will not regain the passive one if he was damaged during the active's duration. **Move Quick*’s passive will be disabled even if the damage is blocked with a spell shield.

---

### E: Toxic Shot

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Self |
| **Damage Type** | Magic |
| **Spell Shield** | False |
| **Spell Effects** | proc |
| **Parry** | True |

**PASSIVE:** **Teemo**’s basic attacks are empowered to deal **bonus** magic damage on-hit and inflict poison.


** The target takes magic damage every second over 4 seconds. Subsequent inflictions refresh the duration.

| Attribute | Value |
|-----------|------:|
| **Magic Damage On-Hit** | 9 / 23 / 37 / 51 / 65 (+ 30% AP) |

| Attribute | Value |
|-----------|------:|
| **Magic Damage per Tick** | 6 / 12 / 18 / 24 / 30 (+ 10% AP) |
| **Total Poison Damage** | 24 / 48 / 72 / 96 / 120 (+ 40% AP) |

*Toxic Shot* deals 145% damage against monsters.

| Attribute | Value |
|-----------|------:|
| **Monster Damage On-Hit** | 13.05 / 33.35 / 53.65 / 73.95 / 94.25 (+ 43.5% AP) |

| Attribute | Value |
|-----------|------:|
| **Monster Damage per Tick** | 8.7 / 17.4 / 26.1 / 34.8 / 43.5 (+ 14.5% AP) |
| **Total Monster Poison Damage** | 34.8 / 69.6 / 104.4 / 139.2 / 174 (+ 58% AP) |

**Notes:**

- The initial hit from *Toxic Shot* will consume Manaflow Band if it is available.
- The attacks do not affect structures nor wards.
- Despite dealing proc damage, the damage over time is special-cased to trigger Dark Harvest and **not** to trigger Summon Aery.
  - The damage over time from *Toxic Shot* counts as proc damage for all other interactions.

---

### R: Noxious Trap

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 600 / 675 / 750 / 825 / 900 units |
| **Collision Radius** | 160 (Triggering radius) / 150 (Bounce collision radius) units |
| **Effect Radius** | 450 units |
| **Cost** | 75 / 65 / 55 / 45 / 35 Mana + 1 Charge |
| **Cooldown** | $0.25$ seconds |
| **Recharge** | 35 / 32.5 / 30 / 27.5 / 25 seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | Special |
| **Spell Effects** | aoedot |
| **Projectile** | True |
| **Out of Range Behavior** | walk to location |

**ACTIVE:** **Teemo** tosses a poisonous mushroom to the target location that becomes stealthed after arming over 1 second, lasting for up to 5 minutes and granting sight of its surroundings. If the mushroom lands on an already-placed one, it will bounce forward again for its cast distance, up to a cap, which can happen repeatedly.

| Attribute | Value |
|-----------|------:|
| **Bounce Distance Cap** | 350 / 400 / 450 / 500 / 550 |

**Teemo** periodically stocks a *Noxious Trap* charge, up to a maximum amount.

| Attribute | Value |
|-----------|------:|
| **Maximum Charges** | 3 / 3.5 / 4 / 4.5 / 5 |

The mushroom will explode upon enemy contact, inflicting poison to nearby enemies and slowing them for 4 seconds, as well as revealing them.

| Attribute | Value |
|-----------|------:|
| **Slow** | 30 / 35 / 40 / 45 / 50% |

** The target takes magic damage every second over 4 seconds. Subsequent inflictions refresh the duration.

| Attribute | Value |
|-----------|------:|
| **Magic Damage per Tick** | 50 / 65.625 / 81.25 / 96.875 / 112.5 (+ 12.5% AP) |
| **Total Magic Damage** | 200 / 262.5 / 325 / 387.5 / 450 (+ 50% AP) |

A mushroom has (health) 6 **maximum** health and can only be damaged by champion basic attacks (2 damage from ranged and 3 from melee).

*See [Pets](#Pets) for more details about mushroom traps.*

**Notes:**

- The bounce can occur an indefinite amount of times while **Teemo** is alive. If **Teemo** dies before *Noxious Trap* bounces on another, the trap will disappear.
- Rift Scuttler will not trigger *Noxious Trap*’s explosion if it is not being attacked.
- Enemies who step on multiple *Noxious Traps* will only refresh the duration of the damage over time and slow.
- *Noxious Trap*’s damage is determined when it detonates and not when planted, meaning if **Teemo**’s (AP) ability power changes, the scaling is also altered to affect the active damage of all *Noxious Traps* so long as they do not explode.
- Spell shield will prevent the damage over time and slow for all units but will not prevent the detonation itself.

---

## Patch History

### V25.18
- Noxious Trap
  - **Bug Fixes:** Explosion VFX is no longer missing when the trap is placed in Fog of War and is triggered shortly after.

### V25.17
- Noxious Trap
  - Mushroom object stealth opacity increased to 50% from 30%.

### V25.04
- Toxic Shot
  - Monster damage increased to 145% from 125%.
  - **Bug Fixes:** Critical strike attacks no longer fail to apply the poison.

### V25.S1.3
- Toxic Shot
  - On-hit base damage reduced to 9 / 23 / 37 / 51 / 65 from 14 / 27 / 40 / 53 / 66.

### V14.24
- Noxious Trap
  - **Bug Fixes:** No longer reveals the user if cast while the cursor is near an enemy champion.

### V14.23
- Toxic Shot
  - Damage against monsters reduced to 125% from 150%.

### V14.21
- General
  - Re-added previous laugh VO due to popular request.
- Stats
  - Base health increased to 615 from 598.
- Move Quick
  - Bonus movement speed increased to 12 / 16 / 20 / 24 / 28% from 10 / 14 / 18 / 22 / 26%.
    - Increased bonus movement speed increased to 24 / 32 / 40 / 48 / 56% from 20 / 28 / 36 / 44 / 52%.

### V14.20
- General
  - Updated ability icons.
  - Complete visual update across all skins.
  - New splash artwork for Teemo, Teemo, Teemo, Teemo, Teemo, Teemo, and Teemo.
    - Adjusted splash artwork for Teemo, Teemo, Teemo, Teemo, Teemo, and Teemo.
  - Teemo RP cost increased to from .
  - Teemo RP cost increased to from .
  - Teemo RP cost increased to from .
  - New lore.
  - New voice-over.
  - Updated sound effects.

### V14.8
- Toxic Shot
  - **Bug Fixes:** Can now once again trigger Dark Harvest.

### V14.5
- Toxic Shot
  - **Undocumented:** Now deals proc damage instead of ability damage.

## Trivia

- Teemo and Teemo in Legends of Runeterra are voiced by Melissa Hutchison, who also voices Ashe.
- Teemo’s dance references Badgers by Jonti Picking.
  - A side-by-side comparison can be seen here.
- Teemo features in the Season 3 mastery Scout's icon.
- Teemo, Akshan, and Evelynn are the only champions who can be permanently stealthed (through Guerrilla Warfare, Going Rogue, and Demon Shade respectively).
- Fiora draws Teemo's face in the in her joke emote.
- Teemo's blowgun can be seen inside a weapons cabinet in the game's Mac Version trailer.
- Teemo being the smallest champion in-game has led to the creation of a measurement unit named after him (used to calculate ranges).
  - Despite the measurement unit, Teemo's actual is 110 units in diameter - "For another point of reference, Teemo has a radius of 55 units and his is approximately 9,503 units".
- In the V1.0.0.115 April Fools' Day patch, the following changes regarding Teemo were jokingly listed:
  - Cuteness increased by 10%.
  - Teemo may no longer be the target of hostile attacks or abilities.
- Over the years, player frustration from Teemo's abilities and backstory have led to meme and perception that he is an evil individual akin to a demonic figure (specifically Satan)
  - Riot Games later officially honored this with the release of Teemo.
- Teemo is voiced by a new actor because the previous one no longer works for Riot Games.
- Teemo is an anagram for the word emote, befitting his recognizable and iconic face which has led to him being an unofficial mascot of sorts.
- On Teemo's day, February 21, RiotGames Twitter share Teemo facts throughout the day.
- Teemo's Series 2 Eternals make the following references:
  - *Toxic Personality* is a reference to the harmful conduct that each player does, performs, or receives in each match within the game.
  - *Crouching Teemo, Hidden Critter* is a reference to the movie titled Crouching Tiger, Hidden Dragon.

---
*This page was automatically generated from League of Legends Wiki data.*