# Teemo

## Table of Contents
- [Basic Information](#basic-information)
- [Statistics](#statistics)
  - [Advanced Stats](#advanced-stats)
- [Abilities](#abilities)
- [Patch History](#patch-history)
- [Trivia](#trivia)

## Basic Information

| Attribute | Value |
|-----------|-------|
| **Champion** | Teemo |
| **Title** | the Swift Scout |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2009-02-21 |
| **Release Patch** | Alpha Week 2 |
| **Roles** | Specialist |
| **Riot Positions** | Top |
| **External Positions** | Top, Jungle, Support |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $615.0$ | $+104.0$ | $2383.0$ |
| **Mana** | $334.0$ | $+25.0$ | $759.0$ |
| **Health Regen** | $5.5$ | $+0.65$ | $16.6$ |
| **Mana Regen** | $9.6$ | $+0.45$ | $17.2$ |
| **Armor** | $24.0$ | $+4.95$ | $108.2$ |
| **Magic Resist** | $30.0$ | $+1.3$ | $52.1$ |
| **Attack Damage** | $54.0$ | $+3.0$ | $105.0$ |
| **Attack Speed** | $0.690$ | $+3.4\%$ | $1.086$ |
| **Movement Speed** | $330.0$ | $+0.0$ | $330.0$ |
| **Attack Range** | $500.0$ | $+0.0$ | $500.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.69$ |
| **Attack Speed Ratio** | $0.69$ |
| **Bonus AS per Level** | $3.4\%$ |
| **Missile Speed** | $1500 units/second$ |
| **Acquisition Radius** | $500 units$ |
| **Gameplay Radius** | $55 units$ |
| **Pathing Radius** | $30 units$ |
| **Selection Radius** | $100 units$ |
| **Selection Height** | $100 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Guerrilla Warfare

**Innate:** **Teemo** becomes invisible after a brief moment remaining idle without taking damage, and will maintain so long as he remains idle or not airborne.

*While **Teemo** is in brush, he will gain and maintain stealth even while moving.*

**Innate:** **Teemo** gains invisibility after $1.5$ seconds without moving, taking damage, performing actions that break stealth, channel, or being in stasis (buff). **Teemo** will maintain stealth so long as he remains idle and is not airborne. While in brush, **Teemo** gains the stealth even while moving and can move without breaking stealth, but will lose the stealth in this case if he is airborne. **Innate - Element of Surprise:** When **Teemo** breaks the stealth, he gains *key=% *bonus attack speed* for 5 seconds.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Self |

**Notes:**

- 'Element of Surprise's buff does not stack with itself, only refreshes on subsequent triggers within the duration.
- All forced action effects will break 'Guerilla Warfare's* stealth while **Teemo** is standing outside of brush. While he is inside brush, all of them will also break the stealth if he is *forced' to move outside brush.
  - Berserk and taunt can additionally break the stealth if **Teemo** is *forced* to use a basic attack, which is an action that breaks stealth by default.
- 'Guerilla Warfare's stealth is special-cased to be interrupted whenever **Teemo** moves from his location by any means (Move order, dash, or blink), thus, performing actions that do not normally break stealth which also involve movement will cause the stealth to break. If **Teemo** is in a brush, however, the stealth is not broken by the above effects so long as his destination location is still inside brush.
- Due to the unique implementation of 'Guerilla Warfare's stealth:
  - Some area-of-effect (AoE) airborne, may **not** be able to break ''Teemo's' stealth. ** While Teemo*** is airborne without his stealth broken, other area-of-effect spells (mostly older ones) **are** able to break ''Teemo's' stealth, such as *Randuin's Omen* Humility.
  - A highly technical explanation can be found here.
- Using a basic attack breaks the stealth at the end of the attack windup.

---

### Q: Blinding Dart

**Active:** **Teemo** shoots a dart at the target enemy that deals magic damage and blind them for a short time.

**Active:** **Teemo** shoots a dart at the target enemy that deals magic damage and blind them for a duration. The duration of the blind is doubled against minions and monsters.

| Attribute | Value |
|-----------|-------|
| **Range** | 680 units |
| **Cooldown** | 7 seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $70-90$ Mana |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Speed** | 2500 units/second |
| **Spell Shield** | True |
| **Spell Effects** | Single target |
| **Projectile** | True |

**Scaling:**
- **Magic Damage:** $80-260$ (+ 70% AP)
- **Blind Duration:** $2-3$ seconds
- **Increased Blind Duration:** $2×2-3×2$ seconds

**Notes:**

- *Blinding Dart* will not make abilities that can trigger on-hit effects (*Parrrley*, *Mystic Shot*) miss.
- When blinded, enemies have a green tint on their screen.
- Because *Blinding Dart* uses center range, it has 45 to 70 more range than his basic attack, which use edge range, against other champions.
  - This bonus becomes lower if **Teemo** or his target have bonuses.

---

### W: Move Quick

**Passive:** **Teemo** gains *ms **bonus** movement speed* after a few seconds without taking damage from enemy champions or turrets.

**Active:** **Teemo** doubles the *ms **bonus** movement speed* for a short time. The passive effect cannot be removed during this time.

**Passive:** **Teemo** gains *ms **bonus** movement speed* after 5 seconds without taking damage from enemy champions or turrets. **Active:** **Teemo** doubles the **bonus** movement speed for 3 seconds, preventing it from being removed for the duration. *Casting Move Quick does not interrupt *Guerrilla Warfare*.*

| Attribute | Value |
|-----------|-------|
| **Cooldown** | 14 seconds |
| **Cast Time** | none |
| **Cost** | 40 mana |
| **Targeting** | Auto |
| **Affects** | Self |

**Scaling:**
- **Bonus Movement Speed:** $12-28$%
- **Enhanced Bonus Movement Speed:** $12×2-28×2$%

**Notes:**

- When 'Move Quick's active bonus movement speed ends **Teemo** will not regain the passive one if he was damaged during the active's duration. *'Move Quick's passive will be disabled even if the damage is blocked with a spell shield.

---

### E: Toxic Shot

**Passive:** **Teemo**’s basic attacks deal **bonus** magic damage and inflict poison.

**Passive:** ''Teemo's** basic attacks are empowered to deal **bonus'' magic damage on-hit and inflict poison. **poison The target takes magic damage every second over 4 seconds. Subsequent inflictions refresh the duration. *Toxic Shot* deals $% damage against monsters.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Self |
| **Damage Type** | Magic |
| **Spell Shield** | False |
| **Spell Effects** | proc |

**Scaling:**
- **Magic Damage On-Hit:** $ (+ **Magic Damage per Tick:* $ (+ (+ $% AP)
- **Monster Damage On-Hit:** $* (+ $% AP)
- **Monster Damage per Tick:** $ (+ $% AP) to ** (+ $% AP)

**Notes:**

- The initial hit from *Toxic Shot* will consume *Manaflow Band* if it is available.
- The attacks do not affect structures nor wards.
- Despite dealing proc damage, the damage over time is special-cased to trigger *Dark Harvest* and **not** to trigger *Summon Aery*.
  - The damage over time from *Toxic Shot* counts as proc damage for all other interactions.

---

### R: Noxious Trap

**Active:** **Teemo** tosses a poisonous mushroom to the target location that becomes stealthed trap and lasts for a while. It will bounce forward upon landing on another mushroom.

**Teemo** periodically stocks a charge of *Noxious Trap*, up to a cap.

**Active:** **Teemo** tosses a poisonous mushroom to the target location that becomes stealthed trap after arming over 1 second, lasting for up to 5 minutes and granting sight of its surroundings. If the mushroom lands on an already-placed one, it will bounce forward again for its cast distance, up to a cap, which can happen repeatedly. **Teemo** periodically stocks a *Noxious Trap* charge, up to a maximum amount. The mushroom will explode upon enemy contact, inflicting poison to nearby enemies and slow them for 4 seconds, as well as standard sight them. **poison The target takes magic damage every second over 4 seconds. Subsequent inflictions refresh the duration. A mushroom has health*maximum** health* and can only be damaged by champion basic attacks (2 damage from ranged and 3 from melee).

| Attribute | Value |
|-----------|-------|
| **Range** | $600-900$ units |
| **Cooldown** | $0.25$ seconds |
| **Recharge** | $35-25$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $75-35$ Mana + 1 Charge |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | 450 units |
| **Spell Shield** | Special |
| **Spell Effects** | aoedot |
| **Projectile** | True |

**Scaling:**
- **Bounce Distance Cap:** $350-550$
- **Maximum Charges:** $3-5$
- **Slow:** $30-50$%

**Notes:**

- The bounce can occur an indefinite amount of times while **Teemo** is alive. If **Teemo** dies before *Noxious Trap* bounces on another, the trap will disappear.
- Rift Scuttler will not trigger 'Noxious Trap's explosion if it is not being attacked.
- Enemies who step on multiple *Noxious Traps* will only refresh the duration of the damage over time and slow.
- 'Noxious Trap's* damage is determined when it detonates and not when planted, meaning if *'Teemo's* AP changes, the scaling is also altered to affect the active damage of all *Noxious Traps' so long as they do not explode.
- Spell shield will prevent the damage over time and slow for all units but will not prevent the detonation itself.

---

## Patch History

### V25.18
- *Noxious Trap*
  - **Bug Fixes:** Explosion VFX is no longer missing when the trap is placed in Fog of War and is triggered shortly after.

### V25.17
- *Noxious Trap*
  - Mushroom object stealth opacity increased to 50% from 30%.

### V25.04
- *Toxic Shot*
  - Monster damage increased to 145% from 125%.
  - **Bug Fixes:** Critical strike attacks no longer fail to apply the poison.
- *Toxic Shot*
  - On-hit base damage reduced to $9-65$ from $14-66$.

### V14.24
- *Noxious Trap*
  - **Bug Fixes:** No longer reveals the user if cast while the cursor is near an enemy champion.

### V14.23
- *Toxic Shot*
  - Damage against monsters reduced to 125% from 150%.

### V14.21
- General
  - Re-added previous laugh VO due to popular request.
- Stats
  - Base health increased to 615 from 598.
- *Move Quick*
  - Bonus movement speed increased to $12-28$% from $10-26$%.
    - Increased bonus movement speed increased to $12×2-28×2$% from $10×2-26×2$%.

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
- *Toxic Shot*
  - **Bug Fixes:** Can now once again trigger *Dark Harvest*.

### V14.5
- *Toxic Shot*
  - **Undocumented:** Now deals proc damage instead of ability damage.

### V14.3
- *Blinding Dart*
  - **New Effect:** Blind should now immediately apply to targets hit, preventing him from taking damage from a basic attack after the blind has been applied. Blind duration should now also be consistent with the debuff duration in the status indicator above the target's health bar.
- *Toxic Shot*
  - Base on-hit damage increased to $14-66$ from $14-58$.
  - On-hit AP ratio increased to 30% AP from 25% AP.

## Trivia

- Teemo and Teemo in Legends of Runeterra are voiced by Melissa Hutchison, who also voices **Ashe**.
- **Teemo**’s dance references Badgers by Jonti Picking.
  - A side-by-side comparison can be seen here.
- Teemo features in the Season 3 mastery 's icon.
- Teemo, , and **Evelynn** are the only champions who can be permanently stealthed (through *Guerrilla Warfare, AiDemon Shade* respectively).
- **Fiora** draws Teemo's face in the in her joke emote.
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