# Gangplank

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
| **Champion** | Gangplank |
| **Title** | the Saltwater Scourge |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2009-08-19 |
| **Release Patch** | V0.9.22.15 |
| **Latest Changes** | V25.17 |
| **Roles** | Specialist |
| **Riot Positions** | Top |
| **External Positions** | Top |
| **Blue Essence** | 1575 |
| **Riot Points** | 790 |
| **Difficulty** | 3 |
| **Hero Type** | Fighter |
| **Adaptive Type** | Physical |
| **Damage** | 3 |
| **Toughness** | 1 |
| **Control** | 1 |
| **Mobility** | 1 |
| **Utility** | 2 |
| **Style** | 75 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $630.0$ | $+114.0$ |
| **Mana** | $280.0$ | $+60.0$ |
| **Health Regen** | $6.0$ | $+0.6$ |
| **Mana Regen** | $8.0$ | $+0.7$ |
| **Armor** | $31.0$ | $+4.7$ |
| **Magic Resist** | $32.0$ | $+2.05$ |
| **Attack Damage** | $64.0$ | $+4.2$ |
| **Attack Speed** | $0.658$ | |
| **Movement Speed** | $345.0$ | $+0.0$ |
| **Attack Range** | $125.0$ | $+0.0$ |
| **Base Attack Speed** | $0.658$ | |
| **Attack Speed Ratio** | $0.69$ | |
| **Bonus AS per Level** | $3.2\%$ | |
| **Attack Windup** | $16.4\%$ | |
| **Acquisition Radius** | $400$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $120$ units | |
| **Selection Height** | $145$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $105.0\%$ |

## Pets

### Powder Keg

| Attribute | Value |
|-----------|------:|
| **Gold** | 10 |
| **Experience** | 0 |
| **Health** | 3 |
| **Control** | Autonomous (static) |
| **Targeting** | Minion |

**Abilities:**

- **Countdown:** The *Powder Keg* loses 1 **maximum** health every 2@1; 1@7; 0.5@13 seconds (minimum 1 **current** health).
- **Fire in the Hole:** **Gangplank** can attack a *Powder Keg* with basic attacks and Parrrley. If he destroys it, the keg no longer grants kill gold but instead explodes, inflicting the triggering strike upon all nearby (400 radius) enemies, ignoring 40% of their armor, slowing them by 40 / 50 / 60 / 70 / 80% for 2 seconds, dealing 75 / 105 / 135 / 165 / 195 **bonus** physical damage to enemy champions and applying spell effects as area damage. Magic damage effects are not applied. If **Gangplank** destroys a *Powder Keg* with Parrrley then every enemy killed by the *Powder Keg* will also be plundered and grants him Silver Serpents as if he had killed them with the initial ability.
- **Powder Trail:** *Powder Kegs* connect to each other when nearby (650 range), and explode when a *Powder Keg* they are connected to is detonated, though they do not affect enemies hit by one of the previous explosions.

---

## Abilities

### Passive: Trial by Fire

| Attribute | Value |
|-----------|------:|
| **Static Cooldown** | 15 |
| **Targeting** | Passive |
| **Affects** | Self, Enemies |
| **Damage Type** | True |
| **Spell Shield** | False |
| **Spell Effects** | default |
| **Parry** | Special |

**INNATE:** Periodically, **Gangplank** empowers his next basic attack to set the target on fire, dealing them「 50 to 250 (+ 100% **bonus** AD) (+ 2 per 1% critical strike chance) **bonus** true damage over $2.5$ seconds. ⟷ 50/10 to 250/10 (+ 10% **bonus** AD) (+ $0.2$ per 1% critical strike chance) **bonus** true damage every $0.25$ seconds over $2.5$ seconds. 」Turrets are dealt 50% damage.

If **Gangplank** successfully hits a target with *Trial by Fire*, he gains (ms) 15 to 30 **bonus** movement speed for 2 seconds. Every time a *Powder Keg* explodes, the cooldown for *Trial by Fire* resets and **Gangplank** gains its **bonus** movement speed.

*Trial by Fire cannot be applied with Parrrley nor Powder Keg.*

**Notes:**

- Reapplying *Trial by Fire* before a previous application has run out will stack the full damage, instead of refreshing the duration.
- The bonus true damage scaling based on critical strike chance is capped at 100% of the stat.
- If the attack is dodged or if **Gangplank** is blinded, the effect is not applied nor does *Trial by Fire* go on cooldown. If the attack is blocked, the effect is not applied, but *Trial by Fire* will go on cooldown.
- The empowered attack will not trigger against non-turret structures nor wards.

---

### Q: Parrrley

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 625 units |
| **Speed** | 2600 (Missile speed) units/second |
| **Cost** | 50 / 45 / 40 / 35 / 30 Mana |
| **Cooldown** | $4.5$ seconds |
| **Targeting** | Unit |
| **Affects** | Enemies, Self |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | Special |
| **Projectile** | True |
| **Parry** | True |
| **Call For Help** | True |

**ACTIVE:** **Gangplank** fires a shot at the target enemy that deals physical damage, applies on-hit effects as a ranged attack, and triggers on-attack effects. *Parrrley* can critically strike for damage.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 10 / 40 / 70 / 100 / 130 (+ 100% AD) |

If *Parrrley* kills the target, **Gangplank** plunders gold gold and *Silver Serpents*. Each enemy killed by a *Powder Keg* explosion that was originally set off by *Parrrley* also counts for the plunder.

| Attribute | Value |
|-----------|------:|
| **Gold Plunder** | 3 / 4 / 5 / 6 / 7 gold |

| Attribute | Value |
|-----------|------:|
| **Silver Serpent Plunder** | Silver Serpents 4 / 5 / 6 / 7 / 8 |

*Silver Serpents* can be spent in the shop to upgrade *Cannon Barrage*.

**Notes:**

- *Parrrley* deals basic damage, but also triggers spell effects by also being tagged as spell damage.
- Fleet Footwork will always trigger its melee effect, even when it is triggered by *Parrrley*.
- On-hit effects that deal damage to secondary targets will allow *Parrrley* to plunder from enemies killed that way, much like with Powder Kegs.
  - Ravenous Hydra and Titanic Hydra work like this and will grant additional plunder from enemies they kill when triggered by *Parrrley* as their cast instance.
  - Even spell effects such as Luden's Companion will work with this when triggered by *Parrrley* as their cast instance. Spell effects that do not belong to their triggering cast instance, such as Liandry's Torment damage debuff, do not grant plunder for enemies they kill.

---

### W: Remove Scurvy

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Cost** | 60 / 70 / 80 / 90 / 100 Mana |
| **Cooldown** | 22 / 20 / 18 / 16 / 14 seconds |
| **Targeting** | Auto |
| **Affects** | Self |

**ACTIVE:** **Gangplank** consumes a large quantity of citrus fruit, cleansing himself from all crowd control and healing himself.

| Attribute | Value |
|-----------|------:|
| **Heal** | 45 / 70 / 95 / 120 / 145 (+ 90% AP) (+ 13% **missing** health) |

**Notes:**

- *Remove Scury* does not remove debuffs other than crowd control, even if both occur from the same effect. For example, *Remove Scurvy* will dispel the slow from Exhaust, but not its damage reduction.
- *Remove Scurvy* can remove the underlying stun from airborne, but a blink or dash ability is required to override the displacement.

---

### E: Powder Keg

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 1000 units |
| **Effect Radius** | 345 (Connection radius) / cr 360 (Explosion radius) units |
| **Cost** | 1 Charge |
| **Recharge** | 17 / 16 / 15 / 14 / 13 seconds |
| **Static Cooldown** | $0.5$ |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | False |
| **Out of Range Behavior** | walk to location |

**ACTIVE:** **Gangplank** consumes a charge to place a powder keg at the target location that lasts for 25 seconds and will connect to other kegs with overlapping connection radius through a trail of black powder. The keg starts with 3 health and loses 1 every 2@1; 1@7; 0.5@13 seconds until it is left with 1 health.

**Gangplank** periodically stocks a *Powder Keg* charge, up to a maximum.

| Attribute | Value |
|-----------|------:|
| **Maximum charges** | 3 / 3 / 4 / 4 / 5 |

Kegs can be basic attacked by enemies or **Gangplank** (including his *Parrrley*), dealing 1 damage to it. When an enemy destroys it, it is safely defused. When **Gangplank** destroys it, it explodes and also triggers a chain reaction that explodes other nearby connected kegs with a -delay between explosions. The explosions also grant sight of their radiuses for 2 seconds.

Enemies caught in an explosion are dealt the triggering attack's damage and slowed for 2 seconds. Against champions, the explosion also deals **bonus** physical damage. Each enemy can only be hit once per chain and the damage dealt ignores (armor penetration) 40% of the target's armor.

| Attribute | Value |
|-----------|------:|
| **Slow** | 40 / 50 / 60 / 70 / 80% |

| Attribute | Value |
|-----------|------:|
| **Bonus Champion Damage** | 75 / 105 / 135 / 165 / 195 |

If the triggering attack was a critical strike, the explosions deal 5% more damage on top of the critical damage (See notes).

**Notes:**

- The triggering attack's damage can include Parrrley’s base damage and Spellblade.
- The percentage armor penetration stacks multiplicatively with other sources of percentage armor penetration.
- When triggered with critical strike, *Spellblade* and the bonus damage to champions are added after the calculations, meaning that these effects are not increased by 5%.
- The total damage can be calculated with this formula:
{aDMG * [1.75 (Infinity Edge)] * 1.05} + sDMG + cDMG
Where:
  - aDMG = Triggering attack's damage
  - sDMG = Spellblade's damage
  - cDMG = Bonus damage to champions
- Kegs have special interactions with Spellblade:
  - If the keg destroying attack was empowered with *Spellblade*, its damage will be added to the explosions' damage.
  - If the keg was not destroyed, it will consume the effect and the keg will store the damage it would have dealt. If the storing keg is the initial detonation of the chain, it will add its damage to the explosions' damage as if it was triggered with *Spellblade*.
  - If the storing keg was attacked with a new instance of *Spellblade*, it will override the previously stored damage (if different).
  - Damage can only be stored if *Spellblade* was consumed by **Gangplank**.
  - The stored damage is static and will not change if **Gangplank**’s **base** attack damage or ability power (see below) changes.
- Specific Spellblade item interactions:
  - Bloodsong: Dealing damage with kegs to champions won't apply *Expose Weakness* to them.
  - Iceborn Gauntlet: The frost field will be created on the location of the keg that is attacked.
  - Lich Bane: The magic damage will be added to the explosions' damage, but will be dealt as physical damage.
  - Trinity Force: Attacking kegs grants the movespeed from Quicken; dealing damage with the explosion does not since it does not trigger on-hit effects.
- Barrels triggering a chain reaction will show a lit fuse traveling toward other barrels.
  - The lit fuse is only visual and does not affect the time it takes for barrels to explode.
- Placed barrels do **not** grant vision until they explode, but they are revealed to **Gangplank**.
  - If **Gangplank** loses allied vision, for instance due to nearsight, he will also lose vision on his barrels temporarily.
  - Barrels prematurely grant sight of their explosion radius (including across terrain and into bushes) if Parrrley or a lit fuse is traveling towards them.
    - This area reveal happens even if Parrrley is not going to make it explode.
- An exploding *Powder Keg* will splash Parrrley’s **bonus** damage, and will also be modified if the attack critically strikes.
- Attacks against *Powder Kegs* will apply on-hit effects (such as Tiamat Cleave), but proc damage against the barrel itself is reduced to 0.
  - Damage from on-hit effects will **not** increase the damage of the explosion.
  - Spellblade is a special-cased exception (Since late Season 9 - previously, Spellblade worked simply because it would add to the attack's damage directly).
- Dead Man's Plate Crushing Blow will not trigger from *Powder Kegs* destroyed from **Gangplank**, even with Parrrley.
- The number of *Powder Kegs* in stock is visible under **Gangplank**’s health bar for all players.
- **Gangplank** stocks *Powder Kegs* even if the ability hasn't been learned yet.
  - This is due to the recharge rate at level 1 being set the same as level 0.
  - While at maximum charges, ranking up the ability to rank 3 or 5 grants the remaining charge immediately regardless of the recharge timer.
- If a barrel is destroyed during the -delay between explosions, **Gangplank** will plunder gold gold and *Silver Serpents* upon killing enemies as if he had done so using Parrrley.
  - This effect persists through death and will only end upon killing an enemy with Parrrley.

---

### R: Cannon Barrage

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | Global |
| **Effect Radius** | 580 (Volley damage and Raise Morale effect radius) units |
| **Inner Radius** | 170 (Death's Daughter) units |
| **Cost** | 100 Mana |
| **Cooldown** | 160 / 150 / 140 / 130 / 120 (Starts on cast) seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | AOE |

**ACTIVE:** **Gangplank** shoots a flare into the air, signaling his ship off-shore to fire upon the target location for 8 seconds, calling down 12 waves of cannonballs in clusters of 3 every 2 seconds, and granting sight of the area for the duration. Each wave deals magic damage to all enemies within the area and slows them by 30% for $0.5$ seconds.

| Attribute | Value |
|-----------|------:|
| **Magic Damage Per Wave** | 40 / 55 / 70 / 85 / 100 (+ 10% AP) |
| **Magic Damage Per Cluster** | 120 / 165 / 210 / 255 / 300 (+ 30% AP) |

| Attribute | Value |
|-----------|------:|
| **Total Magic Damage** | 480 / 660 / 840 / 1020 / 1200 (+ 120% AP) |

**Gangplank** can purchase upgrades for his ship in the store at the cost of 500 *Silver Serpents* each, which improve *Cannon Barrage*:

**DEATH'S DAUGHTER:** A large cannonball lands in the center of the barrage after the first cluster of waves occur, dealing a cluster's worth of true damage to enemies within the impact and slowing them by 75% for 1 second.

| Attribute | Value |
|-----------|------:|
| **True Damage with Death's Daughter** | 120 / 165 / 210 / 255 / 300 (+ 30% AP) |

| Attribute | Value |
|-----------|------:|
| **Total Mixed Damage with Death's Daughter** | 600 / 825 / 1050 / 1275 / 1500 (+ 150% AP) |

**FIRE AT WILL:** *Cannon Barrage* fires「 6 additional waves ⟷ 2 additional clusters 」 over its duration; 18 waves of cannonballs are called down in clusters of 3 every $1.33$ seconds.

| Attribute | Value |
|-----------|------:|
| **Total Magic Damage with Fire at Will** | 720 / 990 / 1260 / 1530 / 1800 (+ 180% AP) |

| Attribute | Value |
|-----------|------:|
| **Maximum Mixed Total Damage with Fire at Will and Death's Daughter** | 840 / 1155 / 1470 / 1785 / 2100 (+ 210% AP) |

**RAISE MORALE:** *Cannon Barrage* grants **Gangplank** and all allies within the area 40% **bonus** movement speed, lingering for 2 seconds.

**Notes:**

- **Gangplank**’s upgrades for Cannon Barrage are offered through the shop and additionally from a special menu in the HUD which is only visible while inside the shop's area. Within the special menu, the player can purchase an upgrade by clicking on its portrait or via the Champion Specific Interaction hotkeys (default: **SHIFT**+**F1–F3**).
  - Purchased upgrades have a trim around their portrait. Upgrades that he does not have enough currency to purchase are greyed out.
- *Cannon Barrage*’s slow lingers for $0.25$ seconds after affected enemies leave the target area.
- If an Attack order is issued and is being cycled through casting *Cannon Barrage*, and no other order is issued after casting it, **Gangplank**’s next basic attack is delayed by 1 second.

---

## Patch History

### V25.17
- Powder Keg
  - **Bug Fixes:** Explosion no longer sometimes cancels his and enemies' attack.

### V25.14
- Parrrley
  - **Bug Fixes:** HUD now displays the proper mana cost, when the option is enabled.

### V25.13
- Parrrley
  - Mana cost reduced to 50 / 45 / 40 / 35 / 30 from 55 / 50 / 45 / 40 / 35.
- Cannon Barrage
  - Cooldown reduced to 160 / 140 / 120 seconds from 170 / 150 / 130.

### V25.05
- Powder Keg
  - **Bug Fixes:** Enemy kills from a chained Powder Keg after buffering Parrrley from out-of-range no longer fail to grant gold.

### V25.04
- Stats
  - Attack damage growth increased to $4.2$ from $3.7$.

### V25.S1.1
- Parrrley
  - **Bug Fixes:** Now properly benefits from Spear of Shojin Focused Will.
- Powder Keg
  - **Bug Fixes:** Now properly benefits from Spear of Shojin Focused Will.

### V14.21
- Stats
  - Base mana regeneration increased to 8 from $7.5$.
- Powder Keg
  - Recharge timer reduced to 17 / 16 / 15 / 14 / 13 seconds from 18 / 17 / 16 / 15 / 14.

### V14.13
- Powder Keg
  - Base slow increased to 40 / 50 / 60 / 70 / 80% from 30 / 37.5 / 45 / 52.5 / 60%.
  - **Removed:*** Slow no longer increases by 0 to 25 by 2.5.

### V14.11
- Stats
  - Base health increased to 630 from 600.
  - Armor growth increased to $4.7$ from $4.2$.

### V14.10
- Powder Keg
  - **Bug Fixes:** Phantom barrel chaining is once again possible.

## Trivia

- During development he was simply called *Pirate* or *Rogal Cheastbeard*.
- Gangplank was named after the eponymous disembarkation device.
- Gangplank is voiced.md) by Dennis Collins Johnson, who also voices Heimerdinger and Taric.
- Parrrley being spelled with three R's references Pirates in popular culture and how Gangplank ironically negotiates (as per the word's definition) by killing his opponents and plundering them.
- Remove Scurvy references the real-world disease of scurvy, a disease common in sailors caused by a deficiency in Vitamin C. The high Vitamin C content in oranges is in turn a tongue-in-cheek way to say that eating them can "remove scurvy".
- NA Summoner 'SantiagoBR' is the creator of the But I ate some oranges and it was K' meme.
- During Closed Beta 1 of development, Gangplank didn't have a beard. This was changed in Closed Beta 3. An image of him in this state can be viewed here.
- His secondary role is Support.
- In the now-removed official League of Legends forums, the original icon of Parrrley was used to represent the "Bug Reports" section.
- Gangplank is voiced by Matthew Mercer, who also voices Gangplank and Kindred.
- He became the first champion to:
  1. Receive two visual updates (the first based on his Chinese artwork and the second based on Burning Tides).
  1. Have four Classic skins (counting Gangplank before being disabled in-game after 'dying' in Burning Tides).
  1. Temporarily replace the announcer in every single map.
- One of the scrapped spells tested during Gangplank's visual gameplay update summoned a "pirate Gundam" that fired mid-range cannonballs from its arm and attacked with a miniature version of Unstoppable Force.

---
*This page was automatically generated from League of Legends Wiki data.*