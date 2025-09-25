# Yone

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
| **Champion** | Yone |
| **Title** | the Unforgotten |
| **Resource** | Flow |
| **Range Type** | Melee |
| **Release Date** | 2020-08-06 |
| **Release Patch** | V10.16 |
| **Latest Changes** | V25.15 |
| **Roles** | Assassin, Skirmisher |
| **Riot Positions** | Top, Middle |
| **External Positions** | Top, Middle |
| **Blue Essence** | 1575 |
| **Riot Points** | 790 |
| **Difficulty** | 3 |
| **Hero Type** | Fighter |
| **Alt Type** | Assassin |
| **Adaptive Type** | Physical |
| **Damage** | 3 |
| **Toughness** | 1 |
| **Control** | 2 |
| **Mobility** | 3 |
| **Utility** | 1 |
| **Style** | 45 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $620.0$ | $+105.0$ |
| **Mana** | $500.0$ | $+0.0$ |
| **Health Regen** | $7.5$ | $+0.75$ |
| **Armor** | $33.0$ | $+4.6$ |
| **Magic Resist** | $32.0$ | $+2.05$ |
| **Attack Damage** | $60.0$ | $+2.0$ |
| **Attack Speed** | $0.625$ | |
| **Movement Speed** | $345.0$ | $+0.0$ |
| **Attack Range** | $175.0$ | $+0.0$ |
| **Base Attack Speed** | $0.625$ | |
| **Attack Speed Ratio** | $0.625$ | |
| **Bonus AS per Level** | $3.5\%$ | |
| **Acquisition Radius** | $400$ units | |
| **Pathing Radius** | $32$ units | |
| **Selection Radius** | $120$ units | |
| **Selection Height** | $180$ units | |
| **Critical Damage** | $160.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $105.0\%$ |
| **Damage Taken** | $97.0\%$ |

#### URF

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $110.0\%$ |
| **Damage Taken** | $90.0\%$ |

## Abilities

### Passive: Way of the Hunter

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical Magic |
| **Spell Effects** | basic |
| **Parry** | True |

**INNATE - INTENT:** **Yone**’s **total** critical strike chance is doubled from all other sources. Additionally,「 every 1% critical strike chance in excess of 100% is converted into $0.5$ **bonus** attack damage. ⟷ every 50% critical strike chance in excess of 100% is converted into 25 **bonus** attack damage. 」

**INNATE - STEEL AND SPIRIT:** **Yone**’s basic attacks alternate between his Steel Sword and Azakana Sword on-attack. **Yone** begins attacking with Steel Sword, and basic attacks with Azakana Sword deal 50% AD physical damage and 50% AD magic damage.

**Notes:**

- There is no attack time-out between alternating swords. It will *persist through death*.
- 50% of Spellblade and Demolish damage will be converted to magic damage if applied by the Azakana Sword.
  - This will also occur if applied by Guinsoo's Rageblade *Phantom Hit* after having last attacked with the Azakana Sword.
  - This will also occur if applied by Mortal Steel after having last attacked with the Azakana Sword.
- The mixed damage is dealt in two simultaneous instances of damage, but will pretend to be a single instance for most effects (such as Conqueror’s stacks).
  - Both instances deal basic damage and thus natively apply life steal.
  - The attack applies on-hit effects only once, right at the start.
  - Even if the target dies from the magic damage, the physical damage will still be applied to it.
    - If the target dies from on-hit damage, the magic damage portion will be skipped and only the physical damage applied.
- **Yone** requires at least 50% critical strike chance in order to reach 100% critical strike chance from *Way of the Hunter*’s multiplier.

---

### Q: Mortal Steel

| Attribute | Value |
|-----------|------:|
| **Range** | 450 (Thrust range) / 450 (Dash distance) / cr 1050 (Whirlwind total range) units |
| **Collision Radius** | 100 (Q3 dash collision radius) units |
| **Width** | 80 (Thrust width) / 160 (Missile width) units |
| **Speed** | 1500 (Dash speed and missile speed) units/second |
| **Static Cooldown** | 4 to 1.48 for 8–4/3@0–105 (@=**bonus** attack speed) |
| **Targeting** | Direction |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | Special |
| **Projectile** | Special |
| **Parry** | Special |
| **Grounded** | Special |
| **Knockdown** | True |

**ACTIVE:** **Yone** thrusts his Steel Sword in a line in the target direction that deals physical damage to enemies hit, applies on-hit effects to the first enemy hit, and triggers on-attack effects once. *Mortal Steel*’s damage based on its AD ratio can critically strike for damage.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 20 / 45 / 70 / 95 / 120 (+ 105% AD) |

If this hits at least one enemy, **Yone** generates a stack of *Gathering Storm* for 6 seconds, stacking up to 2 times and refreshing on subsequent hits. At 2 stacks, the next *Mortal Steel* cast consumes them all to become empowered with a new effect.

**GATHERING STORM BONUS:** **Yone** dashes a fixed distance in the target direction and unleashes a whirlwind in the same direction, both applying and triggering the same effects and additionally knocking up enemies hit in their path for $0.75$ seconds, but being unable to affect the same target twice.

*Mortal Steel*’s thrust will fail to hit targets after the cast time if **Yone** was affected by disarming crowd control during it, but the cooldown of the ability is reset to $0.1$ seconds.

**Notes:**

- *Mortal Steel* applies basic damage for the first (closest) enemy hit (including enemies hit by the dash) and area damage for secondary enemies:
  - *Mortal Steel* will apply on-hit effects to the first target hit, but will not do so to the secondary ones.
  - *Mortal Steel* will not apply spell effects to the first target hit, but will do so to the secondary ones.
  - Spell vamp will only grant healing from the damage dealt to secondary targets, and healing is reduced to 33% effectiveness, accordingly.
  - Life steal will heal based on the damage dealt to the first target hit.
- *Mortal Steel* will not benefit from the bonus attack speed gained by Hail of Blades.
- All three of the thrust, whirlwind and dash roll for a critical strike on each individual target hit.
- Blinking during *Mortal Steel*’s cast with two stacks of *Gathering Storm* (after the cast time) will end the dash prematurely but enemies in range of **Yone** at the new location are affected. The whirlwind's trajectory cannot be changed once it has already been unleashed.
- Only the whirlwind can be intercepted.
- Each parry has different interactions with this ability, whether it's the first target of *Mortal Steel* or the secondary one. In either case **Yone** still gains a stack of *Gathering Storm* and his whirlwind knock up cannot be negated by parries:
  - Dodge and Block: first target **does not** take damage, secondary target **does** take damage.
  - Blind: both first target and secondary target take damage.
- Spell shield does not prevent **Yone** from receiving a stack.
- *Mortal Steel* is disabled while grounded or rooted if **Yone** has two stacks of *Gathering Storm*.
- While at two stacks, a range indicator will be shown for the effective range of the whirlwind.
- The dash can cross terrain.
- If Yone gets knocked back during the 3rd cast, the dash will only damage and knockup around Yone's current position then will stop colliding, making Yone play *Mortal Steel*’s dash winddown animation while still visually dashing.
- This ability will cast from wherever the caster is at the end of the cast time.

  - **Yone** will always cast *Mortal Steel* in his facing direction, and he will turn to face the target direction at the start of the cast time. While at two stacks, **Yone** will always cast it in the target direction.

---

### W: Spirit Cleave

| Attribute | Value |
|-----------|------:|
| **Cast Time** | 0.5–0.19@0–105 (@=**bonus** attack speed) seconds |
| **Effect Radius** | 600 units |
| **Angle** | er 80 (Estimated, see notes)° |
| **Static Cooldown** | 14 / 13.2 / 12.4 / 11.6 / 10.8 / 10 / 9.2 / 8.4 / 7.6 / 6.8 / 6 |
| **Targeting** | Direction |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical Magic |
| **Spell Shield** | True |
| **Spell Effects** | aoe |
| **Projectile** | False |

**ACTIVE:** **Yone** cleaves with his Azakana Sword in a cone in the target direction, dealing equal parts physical and magic damage to enemies hit. The **total** mixed damage has a minimum threshold of 30+10*x for 8 / then + 20*x for 5 / then + 40*x for 5 against minions and a maximum threshold of 140+10*x for 18 against monsters.

| Attribute | Value |
|-----------|------:|
| **Total Mixed Damage** | 10 / 20 / 30 / 40 / 50 (+ 8 / 9 / 10 / 11 / 12% of target's **maximum** health) |
| **Physical Damage** | 5 / 10 / 15 / 20 / 25 (+ 4 / 4.5 / 5 / 5.5 / 6% of target's **maximum** health) |
| **Magic Damage** | 5 / 10 / 15 / 20 / 25 (+ 4 / 4.5 / 5 / 5.5 / 6% of target's **maximum** health) |

If this hits an enemy, **Yone** grants himself a shield for 40 to 90 (+ 65% **bonus** AD) for $1.5$ seconds, increased by 100% if it hits a champion and by 50% for each subsequent champion hit.

**Notes:**

- Despite *Spirit Cleave* generally hitting targets' edge range, it *cannot* hit targets whose center is behind him.
  - This behaviour is common with missiles, however some cone spells such as this one also do it.
- *Spirit Cleave* will not benefit from the bonus attack speed gained by Hail of Blades.
- Spell shield does not prevent **Yone** from receiving and/or increasing the shield.
- The shield's amount will also increase for each clone hit.
- This ability will cast from wherever the caster is at the end of the cast time.

  - *Spirit Cleave* will not preserve the caster's initial facing direction when using Flash and similar effects.
    - It will cast in the caster's new facing direction at the end of the cast time.
- While casting *Spirit Cleave*, **Yone**’s facing direction cannot be shifted by displacements (eg. Vault, Last Breath).
- The mixed damage is applied in two instances, and the physical damage will be dealt first within the same game tick.
  - If the physical damage finishes off the unit, the magic damage will not be dealt against it.

---

### E: Soul Unbound

| Attribute | Value |
|-----------|------:|
| **Range** | 300 (Initial dash range and body distance in the opposite direction) units |
| **Cast Time** | None (Initial cast) / $0.225$ (Recast) |
| **Effect Radius** | Global (Return dash and detonation damage radius) |
| **Speed** | 1200 (Initial and return dash speed, pending for test) units/second |
| **Cooldown** | 22 / 19 / 16 / 13 / 10 (Starts after recasting) seconds |
| **Cooldown Start** | post-effect |
| **Targeting** | Direction / Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | True |
| **Spell Shield** | False |
| **Spell Effects** | aoe |
| **Grounded** | True |
| **Knockdown** | Special |

**ACTIVE:** **Yone** dashes a fixed distance in the target direction, including through terrain, discarding his body and entering *Spirit Form* for 5 seconds. **Yone**’s body is untargetable and is sent the same distance behind the cast location, though not through terrain, and is reclaimed when *Soul Unbound* ends.

**SPIRIT FORM:** **Yone** becomes ghosted and gains (ms) 10%–30%@0–3 (@=seconds active) **bonus** movement speed. His damaging basic attacks and abilities against enemy champions apply a mark that stores a portion of the post-mitigation damage (Damage calculated after modifiers) dealt to the target by his attacks and abilities.

| Attribute | Value |
|-----------|------:|
| **Damage Stored** | 25 / 27.5 / 30 / 32.5 / 35% of damage dealt |

Activation resets *Way of the Hunter’s* current sword state. *Soul Unbound* can be recast after $0.5$ seconds, and automatically does so after the duration.

**RECAST:** **Yone** dashes back to his body with displacement immunity, ending *Spirit Form* and consuming the marks on each champion to deal true damage equal to the total amount of damage stored against each of them.

*The automatic recast is delayed if **Yone** is winding up a basic attack or is unable to recast Soul Unbound under any circumstances, which includes if he cannot move or cast abilities. Soul Unbound will also immediately recast upon death or entering resurrection.*

**Notes:**

- The *marks* store physical damage, magic damage, and true damage.
- *Soul Unbound* stays on cooldown for the first 15 seconds of the game.
- Recall and Teleport are disabled during *Soul Unbound*.
- Only **Yone**’s initial dash can be interrupted.
- **Yone** is displacement immune during the recast's cast time.
- **Yone** uses Flow as his resource to indicate the remaining amount of time in centiseconds (100 centiseconds = 1 second) that *Soul Unbound* can be active for before the ability automatically recasts.
- With 1 second remaining before the recast occurs automatically, a soft, dark vignette will pulse over the screen, accompanied by a distinct audio cue (The actual sound varies by skin). This warning is only visible and audible to **Yone**.
- **Yone**’s body unit cannot be interacted with by any means. It will despawn as soon as **Yone**’s dash back ends.
- *Soul Unbound*’s recast, even when cast automatically, counts as an ability activation for the purposes of on-cast effects such as Spellblade and triggering Force Pulse’s passive.
- *Soul Unbound*’s recast does not interrupt **Yone**’s last movement or attack order.
- **Yone** will place himself onto the ground and interrupt displacements affecting him upon recasting.
- Some examples of effects that can delay *Soul Unbound*’s automatic recast:
  - Attack windup
  - Cast time
  - Channels (excluding Hexflash)
  - Lockouts
  - Dashes
  - Immobilizes, Silence, Ground, and Stasis
  - Devour
  - Fate's Call
    - **Yone** instantly appears at his body's location the moment this effect ends.
  - Realm Warp
- *Soul Unbound*’s recast is disabled (cannot be manually nor automatically cast) while **Yone** is in the Realm of Death or The Hextech Ultimatum. The recast will be deferred until those effects end.
  - The Hextech Ultimatum will specifically end if **Yone** recasts *Soul Unbound*.
- If **Yone** uses Mortal Steel at two *Gathering Storm* stacks to delay *Soul Unbound*’s automatic recast, he will be able to basic attack once in a short time period before the automatic recast triggers.
- If **Yone** dies or enters resurrection with *Spirit Form*, he will immediately initiate the recast and dash back to his body while dead or resurrecting.
  - If he dies during the recast, the dash back is not interrupted.
  - His death animation will play upon arriving to his body.

- The marks do not store damage from items, runes and summoner spells.
- The mark also automatically detonates without **Yone** having to recast when the marked champion dies or enters resurrection.
- The detonation damage cannot be dodged by becoming untargetable.
- The marks will not store damage dealt to shields.
- If the mark's damage is higher than the target's **current** health, the mark indicator will have a slightly different appearance.
  - This does not take into account effects that would amplify the damage of the mark, such as Coup de Grace.

- The following table refers for interactions while **Yone** is in cast time and dashing back to his body:

---

### R: Fate Sealed

| Attribute | Value |
|-----------|------:|
| **Range** | 1000 units |
| **Cast Time** | $0.75$ seconds |
| **Width** | 225 units |
| **Cooldown** | 120 / 110 / 100 / 90 / 80 seconds |
| **Targeting** | Direction |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical Magic |
| **Spell Shield** | Special |
| **Spell Effects** | aoe |
| **Projectile** | False |
| **Grounded** | True |

**ACTIVE:** **Yone** prepares a strike over the cast time, then *marks* all enemies within an area in the target direction, knocking them down and stunning them for 1 second. He blinks 200 units (Can place him beyond maximum range) beyond the center of the last enemy champion struck, or else to maximum range (Uses center range) instead.

After $0.3$ seconds (From the end of the cast time), a gust rushes along the same area that deals equal parts physical and magic damage to *marked* enemies within and pulls them towards the location **Yone** blinked to, then knocks them up for $0.75$ seconds.

| Attribute | Value |
|-----------|------:|
| **Total Mixed Damage** | 200 / 300 / 400 / 500 / 600 (+ 80% **bonus** AD) |
| **Physical Damage** | 100 / 150 / 200 / 250 / 300 (+ 40% **bonus** AD) |
| **Magic Damage** | 100 / 150 / 200 / 250 / 300 (+ 40% **bonus** AD) |

*The stun ends prematurely upon the pull.*

**Notes:**

- **Yone** will blink after a $0.05$-second delay of the cast time being completed.
- *Marked* enemies that are not within the area will not be affected by the second portion of the ability.
- If *Fate Sealed*’s maximum range is inside terrain, enemies will not be pulled through walls.
- *Fate Sealed* pulls targets to the location **Yone** *would* blink to, and not towards his current position by the end of the cast time.
- *Fate Sealed* **can** hit targets whose center is behind **Yone**, if their radius overlaps with the rectangle hitbox in front of him.
- Enemies are pulled with 3000 speed.
- This ability will cast from wherever the caster is at the end of the cast time.
- *Fate Sealed* will not stun enemies that are displacement immune.
- **Yone** can blink up to 1200 units + enemy size, if the last enemy champion's radius intersects with the maximum range slightly.
- The mixed damage is applied in two instances, and the magic damage will be dealt first within the same game tick.
  - If the magic damage finishes off the unit, the physical damage will not be dealt against it.
- **Yone** will reveal himself during the cast time if there is an enemy champion nearby.
- When striking against spell shield, **Yone** will still blink to the last champion hit even if they block the effects to themselves.
- The knock up starts once the pull ends.
- **Yone** is locked out of performing actions for $0.45$ seconds after the cast time. All abilities that have been buffered during this time will cast shortly before the lock out ends.
  - The following table refers for interactions while **Yone** is in cast time:

---

## Patch History

### V25.15
- Mortal Steel
  - **Bug Fixes:** Critical strikes for the first and second cast now properly roll separately on each target hit instead of always critically striking on all subsequent targets if a critical strike was rolled on the first one hit.

### V25.09
- Spirit Cleave
  - **Bug Fixes:** Quinn’s Vault hitting him during its cast time is no longer able to override its initial casting direction.

### V25.08
- Spirit Cleave
  - Target health ratio reduced to 8 / 9 / 10 / 11 / 12% of target's **maximum** health from 10 / 11 / 12 / 13 / 14%.

### V25.07
- Way of the Hunter
  - **Removed:*** No longer grants Yone a critical damage penalty of 10%.
    - This penalty would also apply to Mortal Steel.
- Soul Unbound
  - **Removed:*** Upon completing the cast time for the recast (return), Yone is no longer is able to cleanse the following disables: all immobilizing effects; polymorphs (excluding their accompanying disarm); blind, cripple and drowsy.
  - Tooltip now properly clarifies that the marks only store damage from his attacks and abilities specifically.

### V14.21
- Spirit Cleave
  - Health ratio reduced to 10 / 11 / 12 / 13 / 14% of target's **maximum** health from 11 / 12 / 13 / 14 / 15%.

### V14.20
- Mortal Steel
  - Total critical damage penalty changed to -10% from -20%. No longer applies an *additional* critical damage penalty; only applies Way of the Hunter’s penalty.
    - Critical strike AD ratio increased to 165.375% (175% base critical damage AD from 147% (175% base critical damage AD.

### V14.17
- Mortal Steel
  - **Bug Fixes:** Using *Mortal Steel* after dashing or blinking and without inputting a new command while clamp casting is enabled no longer causes it to target the mouse position prior to their movement and thus cast in a wrong direction.

### V14.15
- Mortal Steel
  - Base damage increased to 20 / 45 / 70 / 95 / 120 from 20 / 40 / 60 / 80 / 100.
- Fate Sealed
  - Cooldown increased to 120 / 100 / 80 seconds from 120 / 90 / 60.

### V14.12
- Stats
  - Base armor increased to 33 from 30.

### V14.10
- Way of the Hunter
  - Critical strike chance multiplier reduced to 2 from $2.5$.
  - Bonus attack damage per excess critical strike chance increased to $0.5$ from $0.4$.

## Trivia

- This champion has no ability power ratio.
- Yone - Yasuo is one of seven pairs of sibling champions (the others being Cassiopeia - Katarina, Kayle - Morgana, Garen - Lux, Nasus - Renekton, Darius - Draven, and Vi - Jinx).
  - Though not a pair, Anivia, Ornn, and Volibear are also siblings.
- Yone is the first non-champion in Legends of Runeterra to become a champion in League of Legends.
- Yone was first confirmed to be the next champion with Thresh voice lines.
- On VPBE during the V11.24 cycle, a new feature was added where whenever Yone scored a pentakill, a dramatic scene of him performing his *Toggle* emote was played for 7 seconds. This was later removed due to controversy.
  - During this time, Yone would become unable to act and negate all incoming damage and debuffs.
  - After the scene ended, his abilities' cooldowns would reset after 1 second.
- Fate Sealed is based on an earlier exploration of Yasuo’s ultimate ability.

---
*This page was automatically generated from League of Legends Wiki data.*