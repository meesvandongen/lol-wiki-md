# Pantheon

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
| **Champion** | Pantheon |
| **Title** | the Unbreakable Spear |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2010-02-02 |
| **Release Patch** | V1.0.0.72 |
| **Latest Changes** | V25.15 |
| **Roles** | Diver |
| **Riot Positions** | Top, Support |
| **External Positions** | Top, Jungle, Middle, Support |
| **Blue Essence** | 675 |
| **Riot Points** | 585 |
| **Difficulty** | 1 |
| **Hero Type** | Fighter |
| **Alt Type** | Assassin |
| **Adaptive Type** | Physical |
| **Damage** | 3 |
| **Toughness** | 2 |
| **Control** | 2 |
| **Mobility** | 2 |
| **Utility** | 1 |
| **Style** | 75 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $650.0$ | $+109.0$ |
| **Mana** | $317.0$ | $+31.0$ |
| **Health Regen** | $6.0$ | $+0.65$ |
| **Mana Regen** | $7.35$ | $+0.45$ |
| **Armor** | $40.0$ | $+4.95$ |
| **Magic Resist** | $28.0$ | $+2.05$ |
| **Attack Damage** | $64.0$ | $+3.3$ |
| **Attack Speed** | $0.658$ | |
| **Movement Speed** | $345.0$ | $+0.0$ |
| **Attack Range** | $175.0$ | $+0.0$ |
| **Base Attack Speed** | $0.658$ | |
| **Attack Speed Ratio** | $0.658$ | |
| **Bonus AS per Level** | $3.0\%$ | |
| **Acquisition Radius** | $600$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $125$ units | |
| **Selection Height** | $175$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $100.0\%$ |
| **Damage Taken** | $100.0\%$ |

## Abilities

### Passive: Mortal Will

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Self |

**INNATE:** **Pantheon** generates a stack of *Mortal Will* whenever he lands a basic attack on-hit or casts an ability, stacking up to 5 times. At 5 stacks, **Pantheon**’s next basic ability consumes the stacks to become empowered with an additional effect.

**Pantheon** gains maximum stacks of *Mortal Will* upon starting the game, completing a Recall channel, and respawning.

**Notes:**

- The current number of *Mortal Will* stacks is represented by a counter under **Pantheon**’s health bar, visible to all players. It will light up when he reaches maximum stacks and the empowered effect is ready.

---

### Q: Comet Spear

| Attribute | Value |
|-----------|------:|
| **Range** | er -40 - 560 (Rectangle starts at 40 units behind Pantheon and has 600 units length) / cr 1200 (Spear missile range) |
| **Cast Time** | $0.2$ (Release) seconds |
| **Width** | 120 (Thrust width) / 110 (Spear missile width) units |
| **Speed** | 2700 (Spear missile speed) units/second |
| **Cost** | 25 Mana |
| **Cooldown** | 11 / 10.25 / 9.5 / 8.75 / 8 seconds |
| **Cooldown Start** | post-effect |
| **Queue Time** | $0.5$ seconds |
| **Targeting** | Auto / Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | Special |
| **Silence** | True |

**ACTIVE:** **Pantheon** charges while being slowed by 10% for up to 4 seconds to increase *Comet Spear*’s range after $0.35$ seconds of channeling. *Comet Spear* can be recast within the duration.

**RECAST:** **Pantheon** hurls his spear in the target direction that deals physical damage to enemies hit, increased against enemies below 20% of their **maximum** health but reduced by 50% against enemies beyond the first.

| Attribute | Value |
|-----------|------:|
| **Hurl Physical Damage** | 70 / 100 / 130 / 160 / 190 (+ 115% **bonus** AD) (+ 50% AP) |
| **Increased Hurl Damage** | 155 / 230 / 305 / 380 / 455 (+ 230% **bonus** AD) (+ 100% AP) |

| Attribute | Value |
|-----------|------:|
| **Hurl Secondary Physical Damage** | 35 / 50 / 65 / 80 / 95 (+ 57.5% **bonus** AD) (+ 25% AP) |
| **Increased Hurl Secondary Damage** | 77.5 / 115 / 152.5 / 190 / 227.5 (+ 115% **bonus** AD) (+ 50% AP) |

Releasing the ability within $0.35$ seconds causes **Pantheon** to instead thrust his spear in the target direction, dealing physical damage to enemies hit, increased against enemies below 20% of their **maximum** health, and refunding 60% of *Comet Spear*’s cooldown. The thrust's damage is not reduced against enemies beyond the first.

| Attribute | Value |
|-----------|------:|
| **Thrust Physical Damage** | 70 / 100 / 130 / 160 / 190 (+ 115% **bonus** AD) |
| **Increased Thrust Damage** | 155 / 230 / 305 / 380 / 455 (+ 230% **bonus** AD) |

*Comet Spear*’s total damage is reduced to 90% against monsters and to 70% against minions.

If the charge is interrupted or completes without reactivation, *Comet Spear* is cancelled and the ability is put on full cooldown but refunds (mana) half the mana cost.

**MORTAL WILL:** *Comet Spear* deals 20 to 240 (+ 115% **bonus** AD) **bonus** physical damage, affected by the previous damage reductions. Consumes the stacks upon recasting.

**Notes:**

- Only the ranged version of *Comet Spear* can be intercepted.
- This ability will cast from wherever the caster is at the end of the cast time.
- The following table refers for interactions while **Pantheon** is channeling:
  - Item actives that interrupt and those with cast times as well as Titanic Hydra, Hexflash, and Recall will cause *Comet Spear* to recast automatically and the active or spell to buffer to cast afterwards.
    - Teleport will cancel the channel entirely.

| Aspect | State / Notes |
|--------|---------------|
| **Type** | Charge channel |
| **Items** | Usable: Shurelya's Battlesong, Youmuu's Ghostblade, Randuin's Omen; Disabled: Hextech Rocketbelt; Interrupted by: Zhonya's Hourglass; Other items: Usable |
| **Summoner Spells** | Usable: Barrier, Clarity, Cleanse, Exhaust, Ghost, Heal, Ignite, Smite, Flash; Interrupted by: Teleport, Recall, Hexflash (recasts) |

---

### W: Shield Vault

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 600 units |
| **Cost** | 55 Mana |
| **Cooldown** | 13 / 12 / 11 / 10 / 9 seconds |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | physical |
| **Spell Shield** | True |
| **Spell Effects** | Special |
| **Parry** | True |
| **Call For Help** | True |
| **Grounded** | True |
| **Knockdown** | True |

**ACTIVE:** **Pantheon** leaps to the front of the target enemy's location. If the target is within 800 units upon arrival, he deals physical damage and stuns them for 1 second. Against minions and monsters, *Shield Vault* has a minimum damage of 60 and is capped at 200.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 6 / 6.5 / 7 / 7.5 / 8% of target's **maximum** health (+ $1.5$% per 100 AP) (+ $0.4$% per 100 **Pantheon**’s **bonus** health) |

**MORTAL WILL:** **Pantheon** empowers his next basic attack within 4 seconds to have a $0.375$-second cast time and strike the target 3 times over a brief period, dealing 120/3 to 165/3 AD physical damage per hit, up to 120 to 165 AD. Each hit is affected by critical strike modifiers and applies on-hit effects.

*Empowered Shield Vault resets **Pantheon**’s basic attack timer. **Pantheon** will attempt to basic attack the target at the end of the leap.*

**Notes:**

- Applies spell damage on the initial vault. Deals basic damage on all the attacks from the empowered ability.
- The empowered attack rolls a critical strike once for all strikes; either all will critically strike, or none will. If Sundered Sky Lightshield Strike is ready, all three strikes will critically strike.
- Despite the multi-hit attack only having a hidden cast time, **Pantheon** will remain unable to act until the last strike has been dealt.
  - The empowered attack's total time is affected by **Pantheon**’s attack speed.
  - The attack's total time is the cast time plus **Pantheon**’s attack windup time.
- The 3 strikes that occurs after *Empowered Shield Vault* each apply a stack of Mortal Will, even if the attacks are dodged, blocked, or missed while **Pantheon** is blinded.
- The empowered attack will not trigger against structures.

---

### E: Aegis Assault

| Attribute | Value |
|-----------|------:|
| **Cast Time** | None (Initial cast) / $0.25$ (Recast) |
| **Effect Radius** | 525 (Cone radius, truncated in the back so it cannot hit behind Pantheon) units |
| **Angle** | 60° |
| **Cost** | 80 Mana |
| **Cooldown** | 22 / 21 / 20 / 19 / 18 seconds |
| **Cooldown Start** | post-effect |
| **Queue Time** | $0.5$ (Initial cast) / $0.5$ (Recast) seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | physical |
| **Spell Shield** | Special |
| **Spell Effects** | aoe |
| **Projectile** | False |
| **Silence** | False |

**ACTIVE:** **Pantheon** braces his shield in the target direction and channels for $1.5$ seconds, during which he becomes invulnerable to non-turret damage dealt by enemies from the target direction. He also continually performs strikes in a cone in front of him, dealing $8.3$% AD physical damage every $0.125$ seconds to enemies hit, reduced by 50% against minions and up to 100% AD **total** physical damage.

*Aegis Assault* can be recast after $0.3$ seconds, and does so automatically after the duration. *Aegis Assault*’s channel cannot be interrupted by crowd control.

**RECAST:** **Pantheon** slams with his shield in a cone in front of him, dealing physical damage to enemies hit.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 55 / 105 / 155 / 205 / 255 (+ 150% **bonus** AD) |

**MORTAL WILL:** After recasting, **Pantheon** gains 5 to 30 (+ $2.5$% **bonus** health) **bonus** armor and **bonus** magic resistance for 4 seconds as well as 60% **bonus** movement speed for $1.5$ seconds.

**Notes:**

*The initial cast count as an ability activation for the purposes of on-cast effects such as Spellblade and triggering Force Pulse’s passive. **Detonating the ability manually is not.
- **Pantheon** retains his invulnerability during the recast's cast time.
- This ability will cast from wherever the caster is at the start of the cast time.
  - **Pantheon** will slam from wherever he was at the start of the recast's cast time.
- Spell shield will only block the slam's damage.
- The following table refers for interactions while **Pantheon** is channeling:

| Aspect | State / Notes |
|--------|---------------|
| **Type** | Charge channel |
| **Attacking** | Disabled |
| **Movement** | Allowed |
| **Abilities** | Disabled |
| **Items** | Disabled: Shurelya's Battlesong, Youmuu's Ghostblade, Randuin's Omen; Other items: Usable |
| **Summoner Spells** | Usable: Barrier, Clarity, Cleanse, Exhaust, Ghost, Heal, Ignite, Smite, Flash, Teleport; Disabled: Recall, Hexflash |
| **Interrupted by** | Death |

---

### R: Grand Starfall

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.1$ seconds |
| **Target Range** | 5500 / 1350 (Landing) + 450 (Landing impact radius) units |
| **Effect Radius** | 450 (Outer width, reduced damage) / 125 (Inner width, maximum damage) units |
| **Inner Radius** | 225 (Initial spear impact radius) / 450 (Landing impact radius (if not hit by the shockwave prior to landing)) units |
| **Cost** | 100 Mana |
| **Cooldown** | 180 / 172.5 / 165 / 157.5 / 150 seconds |
| **Cooldown Start** | on-cast |
| **Queue Time** | $0.5$ seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Physical Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | False |
| **Out of Range Behavior** | cast at max |
| **Grounded** | True |
| **Silence** | Special |

**PASSIVE:** **Pantheon** gains armor penetration.

| Attribute | Value |
|-----------|------:|
| **Armor Penetration** | 10 / 15 / 20 / 25 / 30% |

**ACTIVE:** **Pantheon** channels for 2 seconds, then leaps high into the air, vanishing and becoming immune to crowd control until he reappears. While in the air, **Pantheon** channels again for $2.25$ seconds and grants sight around the target location $0.5$ seconds into the channel.

After $0.8$ (Estimated) seconds into the channel, **Pantheon** prepares for landing by hurling his spear to the target location over $0.2$ (Estimated) seconds that deals 20px (+ 115% **bonus** AD) (+ 50% AP) physical damage to enemies near its landing point and slows them by 50% for 2 seconds.

Over the remaining $1.25$ (Estimated) seconds, **Pantheon** crashes down after $0.55$ (Estimated) seconds and creates a shockwave alongside himself that travels toward the target location over $0.7$ (Estimated) seconds, dealing magic damage to enemies hit, reduced by up to 50% for those hit at the edge of the area. Upon completion of the channel, **Pantheon** reappears at the target location and gains maximum stacks of *Mortal Will*.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 300 / 400 / 500 / 600 / 700 (+ 100% AP) |
| **Reduced Damage** | 150 / 200 / 250 / 300 / 350 (+ 50% AP) |

*Grand Starfall* is placed on a (cd) 30-second cooldown if the first channel is canceled.

**Notes:**

- *Grand Starfall*’s first channel can be interrupted by cast-inhibiting crowd control, but its second channel cannot because **Pantheon** gains crowd control immunity during it.
- The spear's damage is not empowered by Mortal Will, nor increased against enemies below 20% of their **maximum** health.
- During the second channel, **Pantheon** gains a minimum health threshold of 1.
  - **Pantheon** will instantly die upon reappearing if he reaches this threshold.
- Abilities that target **Pantheon** will redirect to the last location.
- Pinging the ability will inform allies in chat which visible enemy champions are in range of *Grand Starfall*.
- **Pantheon** is considered to be in his original casting position even after leaping, but he will blink to the target location the moment the second channel completes.
- Using *Grand Starfall* will inform allies with a ping.
- If Pantheon leaves the Death Realm during *Grand Starfall*’s shockwave, its hitbox will become invisible but it will still deal damage.
- The following table refers for interactions while **Pantheon** is channeling (first channel):

| Aspect | State / Notes |
|--------|---------------|
| **Type** | Charge channel |
| **Attacking** | Disabled |
| **Movement** | Disabled |
| **Abilities** | Disabled |
| **Items** | Disabled: All items |
| **Summoner Spells** | Disabled: All summoner spells |
| **Consumables** | Disabled |
| **Interrupted by** | Death, Grounding effects, Immobilizing effects, Cast-inhibiting effects |
- The cast time and second channel have the same interactions except they can only be interrupted by death.
- *Grand Starfall*’s first channel will also be interrupted if he enters or leaves the Death Realm.

---

## Patch History

### V25.15
- Comet Spear
  - Monster damage reduced to 90% from 105%.

### V14.20
- Shield Vault
  - Base damage increased to 6 / 6.5 / 7 / 7.5 / 8% of target's **maximum** health from 5 / 5.5 / 6 / 6.5 / 7%.

### V14.13
- Shield Vault
  - Minimum non-champion damage increased to 60 from 50.

### V14.12
- Shield Vault
  - **Bug Fixes:** Tooltip now notes that the damage cap also applies against minions. *Actual effect is unchanged.*

### V14.11
- Comet Spear
  - **New Effect:** Throw damage now scales with 50% AP.
- Shield Vault
  - Base damage changed to 5 / 5.5 / 6 / 6.5 / 7% of target's **maximum** health from 60 / 100 / 140 / 180 / 220.
    - **New Effect:** Now deals a minimum of 50 and a maximum of 200 damage against minions and monsters.
  - **Removed:*** Damage no longer scales with 100% AP.
  - **New Effect:** Now scales with $0.4$% per 100 user **bonus** health.
  - **New Effect:** Now scales with $1.5$% per 100 AP.
- Aegis Assault
  - **New Effect:** Now grants 5 to 30 (+ $2.5$% **bonus** health) **bonus** armor and **bonus** magic resistance for 4 seconds after slamming his shield if empowered by Mortal Will.
- Grand Starfall
  - **New Effect:** Spear damage now scales with 50% AP.

### V13.5
- Stats
  - Base health regeneration reduced to 6 from $7.5$.
  - Base attack speed increased to $0.658$ from $0.644$.
  - Attack speed ratio increased to $0.658$ from $0.644$.
- Comet Spear
  - Cooldown reduced to 11 / 10.25 / 9.5 / 8.75 / 8 seconds from 13 / 11.75 / 10.5 / 9.25 / 8.
  - Mana cost reduced to 25 from 30.
  - Cast time reduced to $0.2$ seconds from $0.25$.
- Aegis Assault
  - Cooldown increased to 22 / 21 / 20 / 19 / 18 seconds from 22 / 20.5 / 19 / 17.5 / 16.

### V12.22
- Stats
  - Base mana reduced to 317 from $317.12$.
  - Base mana regeneration reduced to $7.35$ from $7.36$.
- Grand Starfall
  - **Bug Fixes:** No longer is unable to cast for a few frames after the second channel completes.

### V12.14
- Comet Spear
  - Damage against monsters increased to 105% from 70%.

### V12.13
- Comet Spear
  - **Bug Fixes:** Length and width values have now been corrected.
    - Thrust length increased to 600 units from 550.
      - Thrust range increased by 25 (to 560 from 535).
      - Thrust backwards range increased by 25 (to -40 from -15).
    - Thrust width reduced to 120 units from 150.

### V12.10
- Stats
  - Base health increased to 650 from 580.
  - Health growth increased to 109 from 95.
  - Armor growth increased to $4.95$ from $3.75$.
  - Magic resistance growth increased to $2.05$ from $1.25$.
- Comet Spear
  - **Undocumented:** Thrust range and width reverted to pre-

## Trivia

- was named after Pantheon (religion) (a noun originally referring to all deities from Ancient Greek religion, later Twelve Olympians).
  - His human name comes from pre-Greek Ἀτρεύς, analyzed by folk-etymology as "untrembling"; based on the name of an Achaeans warlord known to the Hittites as Attarsiya.
  - Atreus' friend's name Πύλας *Pylas* also derives from the pre-Greek architectural term πῠ́λη *pylē* "gate", from which we have "Pylon (architecture)".Beekes, R.S.P.

---
*This page was automatically generated from League of Legends Wiki data.*