# Zilean

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
| **Champion** | Zilean |
| **Title** | the Chronokeeper |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2009-04-18 |
| **Release Patch** | April 18, 2009 Patch |
| **Latest Changes** | V25.13 |
| **Roles** | Specialist |
| **Riot Positions** | Support |
| **External Positions** | Support |
| **Blue Essence** | 675 |
| **Riot Points** | 585 |
| **Difficulty** | 2 |
| **Hero Type** | Support |
| **Alt Type** | Mage |
| **Adaptive Type** | Magic |
| **Damage** | 2 |
| **Toughness** | 1 |
| **Control** | 2 |
| **Mobility** | 2 |
| **Utility** | 3 |
| **Style** | 100 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $574.0$ | $+96.0$ |
| **Mana** | $452.0$ | $+50.0$ |
| **Health Regen** | $5.5$ | $+0.5$ |
| **Mana Regen** | $11.35$ | $+0.8$ |
| **Armor** | $24.0$ | $+5.0$ |
| **Magic Resist** | $30.0$ | $+1.3$ |
| **Attack Damage** | $52.0$ | $+3.0$ |
| **Attack Speed** | $0.658$ | |
| **Movement Speed** | $335.0$ | $+0.0$ |
| **Attack Range** | $550.0$ | $+0.0$ |
| **Base Attack Speed** | $0.658$ | |
| **Attack Speed Ratio** | $0.625$ | |
| **Bonus AS per Level** | $2.1\%$ | |
| **Missile Speed** | $1500$ units/second | |
| **Acquisition Radius** | $550$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $135$ units | |
| **Selection Height** | $185$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $105.0\%$ |
| **Damage Taken** | $95.0\%$ |

#### URF

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $105.0\%$ |
| **Damage Taken** | $90.0\%$ |

## Abilities

### Passive: Time in a Bottle

| Attribute | Value |
|-----------|------:|
| **Target Range** | 875 units |
| **Static Cooldown** | 120 (Starts post-effect) |
| **Targeting** | Unit |
| **Affects** | Allies |
| **Silence** | True |

**INNATE:** **Zilean** generates (Experience) 2@1; 3.5@6; 5@11; 6@16; 12@18 experience (does not count towards his own level up.md)) every 5 seconds. *Time in a Bottle* is on cooldown when the game starts and becomes disabled when everyone on **Zilean**’s team including himself has reached level 18.

When he has stored enough to level up an allied champion and grant the same amount of experience to himself, he can select them (Default MB2/right-click) to channel for $1.2$ seconds after a $0.5$-second cast time. The channel is interrupted and disabled upon entering combat with enemy champions or taking damage from turrets, placing it on a 10-second cooldown. If **Zilean** interrupts it himself, it is placed on a 1-second cooldown, increased to 2 if he used a basic attack against a turret to do so.

A successful channel will grant an equal amount of experience to the ally to-level-up and **Zilean**, with a combined minimum of 15% his stored experience.

**Notes:**

- **Zilean** has a hidden passive that grants him 1 ability power for every enemy Volibear within 800 range of him.
  - Likewise, Volibear gains 1 armor for every nearby **Zilean**.
  - Neither Volibear nor **Zilean** need sight of one another to gain these bonuses.
- *Time in a Bottle* marks allied champions with two circles beneath them; the inner one represents how close they are to leveling up, turning golden when able to activate *Time in a Bottle* on them.
- The target allied champion is not required to be out of combat to be leveled up by *Time in a Bottle* - only **Zilean** is.
- **Zilean** only consumes the amount of experience required to level up an allied champion. He retains the excess for later use.
- The target allied champion is not required to be within range of **Zilean** for the channel to complete successfully, it will complete regardless of if they move to a far distance.
- The channel from *Time in a Bottle* is not interrupted even if the allied champion suddenly levels up. However, no experience will be granted to either player, and the passive will not go on cooldown.
- The following table refers for interactions while **Zilean** is channeling:

| Aspect | State / Notes |
|--------|---------------|
| **Type** | Charge channel |
| **Attacking** | Interrupts |
| **Movement** | Interrupts |
| **Abilities** | Interrupts |
| **Items** | Usable: Shurelya's Battlesong, Youmuu's Ghostblade, Randuin's Omen; Disabled: Zhonya's Hourglass; Other items: Interrupt |
| **Summoner Spells** | Usable: Barrier, Clarity, Cleanse, Exhaust, Ghost, Heal, Ignite, Smite; Interrupted by: Flash, Teleport, Recall, Hexflash |
| **Consumables** | Usable |
| **Interrupted by** | Death, Cast-inhibiting effects |

---

### Q: Time Bomb

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 900 units |
| **Collision Radius** | 140 units |
| **Effect Radius** | 350 units |
| **Cost** | 60 / 65 / 70 / 75 / 80 Mana |
| **Cooldown** | 10 / 9.5 / 9 / 8.5 / 8 seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | True |
| **Out of Range Behavior** | walk to location |

**ACTIVE:** **Zilean** throws a ticking time bomb to the target location that grants sight of its surroundings. The bomb will attach itself to units that move within the epicenter, or those hit directly, revealing them.

After 3 seconds, or when the attached unit dies, the bomb explodes to deal magic damage to nearby enemies.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 75 / 115 / 165 / 230 / 300 (+ 90% AP) |

The bomb detonates immediately if another bomb attaches itself to the same unit, stunning nearby enemies for a duration.

| Attribute | Value |
|-----------|------:|
| **Stun Duration** | 1.1 / 1.2 / 1.3 / 1.4 / 1.5 seconds |

**Notes:**

- *Time Bombs* landing directly on top of multiple units follow a priority order for whom to attach themselves to.
  1. Units already carrying a bomb
  1. Enemy champions
  1. Allied champions
  1. Enemy minions
  1. Allied minions
- *Time Bombs* are not visible to enemies if they attach to an ally that is stealthed and vice versa.
- When *Time Bomb* snaps onto an enemy, it deals 0 proc damage.
  - This triggers in-combat effects such as drawing turret aggro and drawing monster aggression.
  - It also triggers Sudden Impact and applies Elixir of Sorcery.
  - It does not trigger Cheap Shot, however, as proc damage doesn't trigger Cheap Shot.
- **Zilean** can detonate an enemy counterpart's *Time Bomb* by placing his own on the same unit (the first bomb still deals damage).
- *Time Bomb* will be dispelled if the holder takes fatal damage and is saved by resurrection before it explodes.
- *Time Bomb* can deal damage to the enemy holder through untargetablility.
- Spell shield will prevent the application of *Time Bomb* and it's detonation damage as well as block the stun, but not it's immediate detonation if the target already has a *Time Bomb* on them.
- While disguised as a non-champion, Neeko cannot be affected by *Time Bomb*.

---

### W: Rewind

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Cost** | 35 Mana |
| **Cooldown** | 14 / 12 / 10 / 8 / 6 seconds |
| **Targeting** | Auto |
| **Affects** | Self |

**ACTIVE:** **Zilean** reduces the (cd) **remaining** cooldowns of *Time Bomb* and *Time Warp* by 10 seconds each.

*Either Time Bomb or Time Warp must be on cooldown to cast this ability*.

**Notes:**

- No additional details.

---

### E: Time Warp

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 550 units |
| **Cost** | 50 Mana |
| **Cooldown** | 15 seconds |
| **Targeting** | Unit |
| **Affects** | Enemies, Allies |
| **Spell Shield** | True |
| **Projectile** | False |

**ACTIVE:** **Zilean** applies *Time Warp* to the target champion which lasts for $2.5$ seconds.

**TIME WARP:** If the target is an ally, they gain **bonus** movement speed. If the target is an enemy, they are slowed.

| Attribute | Value |
|-----------|------:|
| **Movement Speed Modifier** | 40 / 55 / 70 / 85 / 99% |

**Notes:**

- No additional notes.

---

### R: Chronoshift

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 900 units |
| **Cost** | 125 / 137.5 / 150 / 162.5 / 175 Mana |
| **Cooldown** | 120 / 105 / 90 / 75 / 60 seconds |
| **Targeting** | Unit |
| **Affects** | Self, Allies |

**ACTIVE:** **Zilean** places a protective time rune on the target allied champion or himself for 5 seconds.

If the target takes fatal damage within the duration, they enter resurrection for 3 seconds, becoming invulnerable, untargetable, and unable to act. Afterwards, they revive while being healed.

| Attribute | Value |
|-----------|------:|
| **Heal** | 600 / 725 / 850 / 975 / 1100 (+ 200% AP) |

**Notes:**

- Upon trigger, *Chronoshift* places all summoner spells that are not already on cooldown on a 3-second cooldown.
- *Chronoshift*’s untargetability does not destroy in-flight projectiles.
- *Chronoshift* cannot target clones.
- *Chronoshift* takes priority over all other resurrection effects as well as all zombie state effects.
- *Chronoshift* does not activate if the target is killed by the Nexus Obelisk.
- *Chronoshift* has a forgiveness radius of 175 units.
- While in resurrection, the target's health regeneration is set to 0.
- The target's screen will have a white tint.
- The following table refers for interactions while the target is unable to act:

| Aspect | State / Notes |
|--------|---------------|
| **Attacking** | Disabled |
| **Movement** | Disabled |
| **Abilities** | Disabled |
| **Items** | Disabled: All items |
| **Summoner Spells** | Usable: Barrier, Clarity, Cleanse, Exhaust, Ghost, Heal, Ignite, Smite; Disabled: Flash, Teleport, Recall, Hexflash, Mark, Dash |
| **Consumables** | Disabled |
| **Interrupted by** | Death |

---

## Patch History

### V25.13
- Time Bomb
  - **Bug Fixes:** Fixed an issue where the second bomb on the same target would not play the countdown SFX.

### V25.11
- Time Bomb
  - **Bug Fixes:** Restored countdown SFX on bombs when the explosion SFX triggers from a second bomb connecting.

### V25.04
- Time in a Bottle
  - **Bug Fixes:** If his False Life triggers, no longer causes the XP-share channel time to permanently increase to the stasis duration.

### V25.S1.2
- Time in a Bottle
  - **Bug Fixes:** Casting Teleport / Unleashed Teleport no longer causes the XP-share channel time to permanently increase to the last Teleport's / Unleashed Teleport's channel duration.
- Chronoshift
  - **Bug Fixes:** Now properly benefits from Axiom Arcanist.

### V14.24
- Time Bomb
  - **Bug Fixes:** No longer reveals the user if cast while the cursor is near an enemy champion.

### V14.20
- Chronoshift
  - **New Effect:** Cast now has a forgiveness radius of 175 units.

### V13.22
- Stats
  - Base attack speed increased to $0.658$ from $0.625$.
  - Basic attack missile speed increased to 1500 from 1200.

### V13.3
- Chronoshift
  - **Bug Fixes:** Now properly triggers after being consumed at the same time that the target's spell shield was consumed.

### V12.22
- Stats
  - Base mana regeneration increased to $11.35$ from $11.34$.

### V12.10
- Stats
  - Base health increased to 574 from 504.
  - Health growth increased to 96 from 82.
  - Armor growth increased to 5 from $3.8$.
  - Magic resistance growth increased to $1.3$ from $0.5$.

## Trivia

- The original icon for Time Bomb featured a soldier that was mirrored from the icon art of Jax’s Grandmaster's Might.
- Zilean is one of the four champions with a single damaging ability (Time Bomb) the others being Bard (Cosmic Binding) Taric (Dazzle) and Tryndamere (Spinning Slash).
  - Time Bomb is the only in-game ability to scale non-linearly per rank (75 / 40 / 50 / 65 / 70).
- Time in a Bottle references the eponymous song by Jim Croce.
- Zilean is the first champion to have a basic ability that cannot be ranked up at level 1 (Rewind) the second being Azir (Conquering Sands, Shifting Sands).
- Zilean's backstory resembles Slaughterhouse-Five by Kurt Vonnegut.

---
*This page was automatically generated from League of Legends Wiki data.*