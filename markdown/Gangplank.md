# Gangplank

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
| **Champion** | Gangplank |
| **Title** | the Saltwater Scourge |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2009-08-19 |
| **Release Patch** | V0.9.22.15 |
| **Roles** | Specialist |
| **Riot Positions** | Top |
| **External Positions** | Top |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $630.0$ | $+114.0$ | $2568.0$ |
| **Mana** | $280.0$ | $+60.0$ | $1300.0$ |
| **Health Regen** | $6.0$ | $+0.6$ | $16.2$ |
| **Mana Regen** | $8.0$ | $+0.7$ | $19.9$ |
| **Armor** | $31.0$ | $+4.7$ | $110.9$ |
| **Magic Resist** | $32.0$ | $+2.05$ | $66.8$ |
| **Attack Damage** | $64.0$ | $+4.2$ | $135.4$ |
| **Attack Speed** | $0.658$ | $+3.2\%$ | $1.016$ |
| **Movement Speed** | $345.0$ | $+0.0$ | $345.0$ |
| **Attack Range** | $125.0$ | $+0.0$ | $125.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.658$ |
| **Attack Speed Ratio** | $0.69$ |
| **Bonus AS per Level** | $3.2\%$ |
| **Attack Windup** | $16.4\%$ |
| **Acquisition Radius** | $400 units$ |
| **Pathing Radius** | $35 units$ |
| **Selection Radius** | $120 units$ |
| **Selection Height** | $145 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Trial by Fire

**Innate:** Every few seconds, **Gangplank**’s next basic attack will burn the target, continually dealing **bonus** true damage and granting him *ms **bonus** movement speed*.

*Whenever a **Powder Keg** explodes, 'Trial by Fire's cooldown will ah and **Gangplank** will gain the movement speed.*

**Innate:** Periodically, **Gangplank* empowers his next basic attack to set the target on fire, dealing themturret are dealt 50% damage. If **Gangplank** successfully hits a target with *Trial by Fire*, he gains ms*bonus** movement speed* for 2 seconds. Every time a **Powder Keg** explodes, the *cooldown* for *Trial by Fire* resets and **Gangplank** gains its **bonus** movement speed. *Trial by Fire cannot be applied with *Parrrley* nor *Powder Keg*.*

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Self, Enemies |
| **Damage Type** | True |
| **Spell Shield** | False |
| **Spell Effects** | default |

**Notes:**

- Reapplying *Trial by Fire* before a previous application has run out will stack the full damage, instead of refreshing the duration.
- The bonus true damage scaling based on critical strike chance is capped at 100% of the stat.
- If the attack is dodge or if **Gangplank** is blind, the effect is not applied nor does *Trial by Fire* go on cooldown. If the attack is block, the effect is not applied, but *Trial by Fire* will go on cooldown.
- The empowered attack will not trigger against non-turret structures nor wards.

---

### Q: Parrrley

**Active:** **Gangplank** shoots the target enemy with his pistol, dealing physical damage and applying on-hit and on-attack effects.

*If this attack kills the target, he gains gold gold and *Silver Serpents*.*

**Active:** **Gangplank** fires a shot at the target enemy that deals physical damage, applies on-hit effects as a ranged attack, and triggers on-attack effects. *Parrrley* can critically strike for critical damage. If *Parrrley* kills the target, **Gangplank** plunders gold gold and **Silver Serpents**. Each enemy killed by a **Powder Keg** explosion that was originally set off by *Parrrley* also counts for the plunder. **Silver Serpents** can be spent in the shop to upgrade **Cannon Barrage**.

| Attribute | Value |
|-----------|-------|
| **Range** | 625 units |
| **Cooldown** | $4.5$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $50-30$ Mana |
| **Targeting** | Unit |
| **Affects** | Enemies, Self |
| **Damage Type** | Physical |
| **Speed** | 2600 units/second |
| **Spell Shield** | True |
| **Spell Effects** | Special |
| **Projectile** | True |

**Scaling:**
- **Physical Damage:** $10-130$ (+ 100% AD)
- **Gold Plunder:** $3-7$ gold
- **Silver Serpent Plunder:** *icononly=yes $4-8$*

**Notes:**

- *Parrrley* deals basic damage, but also triggers spell effects by also being tagged as spell damage.
- *Fleet Footwork* will always trigger its melee effect, even when it is triggered by *Parrrley*.
- On-hit effects that deal damage to secondary targets will allow *Parrrley* to plunder from enemies killed that way, much like with *Powder Kegs*.
  - *Ravenous Hydra* and *Titanic Hydra* work like this and will grant additional plunder from enemies they kill when triggered by *Parrrley* as their cast instance.
  - Even spell effects such as *Luden's Companion* will work with this when triggered by *Parrrley* as their cast instance. Spell effects that do not belong to their triggering cast instance, such as *Liandry's Torment* damage debuff, do not grant plunder for enemies they kill.

---

### W: Remove Scurvy

**Active:** **Gangplank** eats citrus to cleanse crowd control effects and heal based on his **missing** health.

**Active:** **Gangplank** consumes a large quantity of citrus fruit, cleanse himself from all crowd control and heal himself.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $22-14$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $60-100$ Mana |
| **Targeting** | Auto |
| **Affects** | Self |

**Scaling:**
- **Heal:** $45-145$ (+ 90% AP) (+ 13%
- **missing** health)

**Notes:**

- *Remove Scury* does not remove debuffs other than crowd control, even if both occur from the same effect. For example, *Remove Scurvy* will dispel the slow from Exhaust, but not its damage reduction.
- *Remove Scurvy* can remove the underlying stun from airborne, but a blink or dash ability is required to override the displacement.

---

### E: Powder Keg

**Active:** **Gangplank** places a *Powder Keg* at the target location, which can be basic attacked or *shot* to damage it. If an enemy destroys a *Powder Keg*, it is safely dismantled.

*If **Gangplank** destroys a *Powder Keg*, it will explode to deal the attack's damage to nearby enemies and slow them. Against an enemy champion, the explosion additionally deals **bonus** physical damage and ignores a armor penetration. The explosion will cause nearby *Powder Kegs* to also explode.*

**Active:** **Gangplank** consumes a charge to place a powder keg at the target location that lasts for 25 seconds and will connect to other kegs with overlapping connection radius through a trail of black powder. The keg starts with 3 health and loses 1 every 2@1; 1@7; 0.5@13 seconds until it is left with 1 health. **Gangplank** periodically stocks a *Powder Keg* charge, up to a maximum. Kegs can be basic attacked by enemies or **Gangplank** (including his **Parrrley**), dealing 1 damage to it. When an enemy destroys it, it is safely defused. When **Gangplank** destroys it, it explodes and also triggers a chain reaction that explodes other nearby connected kegs with a -delay between explosions. The explosions also grant sight of their radiuses for 2 seconds. Enemies caught in an explosion are dealt the triggering attack's damage and slow for 2 seconds. Against champions, the explosion also deals **bonus** physical damage. Each enemy can only be hit once per chain and the damage dealt ignores armor penetration. If the triggering attack was a critical strike, the explosions deal 5% more damage on top of the critical damage.

| Attribute | Value |
|-----------|-------|
| **Range** | 1000 units |
| **Recharge** | $17-13$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | 1 Charge |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Effect Radius** | 345 / cr 360 units |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | False |

**Scaling:**
- **Maximum charges:** $3/3/4/4/5$

**Notes:**

- The triggering attack's damage can include *Parrrley*’s base damage and Spellblade.
- The percentage armor penetration stacks multiplicatively with other sources of percentage armor penetration.
- When triggered with critical strike, *Spellblade* and the bonus damage to champions are added after the calculations, meaning that these effects are not increased by 5%.
- The total damage can be calculated with this formula: {aDMG * [1.75 (*Infinity Edge*)] * 1.05} + sDMG + cDMG Where:
  - aDMG = Triggering attack's damage
  - sDMG = Spellblade's damage
  - cDMG = Bonus damage to champions
- Kegs have special interactions with Spellblade:
  - If the keg destroying attack was empowered with *Spellblade*, its damage will be added to the explosions' damage.
  - If the keg was not destroyed, it will consume the effect and the keg will store the damage it would have dealt. If the storing keg is the initial detonation of the chain, it will add its damage to the explosions' damage as if it was triggered with *Spellblade*.
  - If the storing keg was attacked with a new instance of *Spellblade*, it will override the previously stored damage (if different).
  - Damage can only be stored if *Spellblade* was consumed by **Gangplank**.
  - The stored damage is static and will not change if ''Gangplank's** **base'' attack damage or ability power (see below) changes.
- Specific Spellblade item interactions:
  - *Bloodsong*: Dealing damage with kegs to champions won't apply *Expose Weakness* to them.
  - *Iceborn Gauntlet*: The frost field will be created on the location of the keg that is attacked.
  - *Lich Bane*: The magic damage will be added to the explosions' damage, but will be dealt as physical damage.
  - *Trinity Force*: Attacking kegs grants the movespeed from Quicken; dealing damage with the explosion does not since it does not trigger on-hit effects.
- Barrels triggering a chain reaction will show a lit fuse traveling toward other barrels.
  - The lit fuse is only visual and does not affect the time it takes for barrels to explode.
- Placed barrels do **not** grant sight until they explode, but they are revealed to **Gangplank**.
  - If **Gangplank** loses allied vision, for instance due to nearsight, he will also lose vision on his barrels temporarily.
  - Barrels prematurely grant sight of their explosion radius (including across terrain and into bushes) if *Parrrley* or a lit fuse is traveling towards them. *** This area reveal happens even if *Parrrley* is not going to make it explode.
- An exploding *Powder Keg* will splash *Parrrley*’s **bonus** damage, and will also be modified if the attack critical strike.
- Attacks against *Powder Kegs* will apply on-hit effects (such as *Tiamat* Cleave), but proc damage against the barrel itself is reduced to 0.
  - Damage from on-hit effects will **not** increase the damage of the explosion.
  - Spellblade is a special-cased exception.
- *Dead Man's Plate* Crushing Blow will not trigger from *Powder Kegs* destroyed from **Gangplank**, even with *Parrrley*.
- The number of *Powder Kegs* in stock is visible under ''Gangplank's' health bar for all players.
- **Gangplank** stocks *Powder Kegs* even if the ability hasn't been learned yet.
  - This is due to the recharge rate at level 1 being set the same as level 0.
  - While at maximum charges, ranking up the ability to rank 3 or 5 grants the remaining charge immediately regardless of the recharge timer.
- If a barrel is destroyed during the -delay between explosions, **Gangplank** will plunder gold gold and **Silver Serpents** upon killing enemies as if he had done so using *Parrrley*.
  - This effect persists through death and will only end upon killing an enemy with *Parrrley*.

---

### R: Cannon Barrage

**Active:** **Gangplank** orders a cannon bombardment to the target location, each blast deals magic damage and slow enemies within.

**Gangplank** can improve *Cannon Barrage* by purchasing upgrades in the shop with *Silver Serpents*.

**Active:** **Gangplank** shoots a flare into the air, signaling his ship off-shore to fire upon the target location for 8 seconds, calling down 12 waves of cannonballs in clusters of 3 every 2 seconds, and granting sight of the area for the duration. Each wave deals magic damage to all enemies within the area and slow them by 30% for $0.5$ seconds. **Gangplank** can purchase upgrades for his ship in the store at the cost of 500 **Silver Serpents** each, which improve *Cannon Barrage*: **Death's Daughter:** A large cannonball lands in the center of the barrage after the first cluster of waves occur, dealing a cluster's worth of true damage to enemies within the impact and slow them by 75% for 1 second. **Fire at Will:** *Cannon Barrage* fires over its duration; 18 waves of cannonballs are called down in clusters of 3 every $1.33$ seconds. **Raise Morale:** *Cannon Barrage* grants **Gangplank** and all allies within the area *40% **bonus** movement speed*, lingering for 2 seconds.

| Attribute | Value |
|-----------|-------|
| **Range** | Global |
| **Cooldown** | $160-120$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | 100 Mana |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | 580 units |
| **Spell Shield** | True |
| **Spell Effects** | AOE |

**Scaling:**
- **Magic Damage Per Wave:* $40-100$ (+ 10% AP)3-100×3$ (+ $10×3$% AP) **Total Magic Damage:** $40×12-100×12$ (+ $10×12$% AP)
- **True Damage with **Total Mixed Damage with

**Notes:**

- ''Gangplank's' upgrades for Cannon Barrage are offered through the shop and additionally from a special menu in the HUD which is only visible while inside the shop's area. Within the special menu, the player can purchase an upgrade by clicking on its portrait or via the Champion Specific Interaction hotkeys (default: **Shift**+**F1–F3**).
  - Purchased upgrades have a trim around their portrait. Upgrades that he does not have enough currency to purchase are greyed out.
- 'Cannon Barrage's slow lingers for $0.25$ seconds after affected enemies leave the target area.
- If an Attack order is issued and is being cycled through casting *Cannon Barrage*, and no other order is issued after casting it, ''Gangplank's' next basic attack is delayed by 1 second.

---

## Patch History

### V25.17
- *Powder Keg*
  - **Bug Fixes:** Explosion no longer sometimes cancels his and enemies' attack.

### V25.14
- *Parrrley*
  - **Bug Fixes:** HUD now displays the proper mana cost, when the option is enabled.

### V25.13
- *Parrrley*
  - Mana cost reduced to $50-30$ from $55-35$.
- *Cannon Barrage*
  - Cooldown reduced to $160-120 3$ seconds from $170-130 3$.

### V25.05
- *Powder Keg*
  - **Bug Fixes:** Enemy kills from a chained *Powder Keg* after buffering *Parrrley* from out-of-range no longer fail to grant gold.

### V25.04
- Stats
  - Attack damage growth increased to $4.2$ from $3.7$.
- *Parrrley*
  - **Bug Fixes:** Now properly benefits from *Spear of Shojin* Focused Will.
- *Powder Keg*
  - **Bug Fixes:** Now properly benefits from *Spear of Shojin* Focused Will.

### V14.21
- Stats
  - Base mana regeneration increased to 8 from $7.5$.
- *Powder Keg*
  - Recharge timer reduced to $17-13$ seconds from $18-14$.

### V14.13
- *Powder Keg*
  - Base slow increased to $40-80$% from $30-60$%.
  - **Removed:*** Slow no longer increases by 0 to 25 by 2.5.

### V14.11
- Stats
  - Base health increased to 630 from 600.
  - Armor growth increased to $4.7$ from $4.2$.

### V14.10
- *Powder Keg*
  - **Bug Fixes:** Phantom barrel chaining is once again possible.

### V14.9
- General
  - Adjusted splash artwork for Gangplank.
- Stats
  - Selection radius increased to 120 units from 85.

## Trivia

- During development he was simply called *Pirate* or *Rogal Cheastbeard*.
- Gangplank was named after the eponymous disembarkation device.
- Gangplank is voiced.md) by Dennis Collins Johnson, who also voices **Heimerdinger** and **Taric**.
- *image=Parrrley old.png* being spelled with three R's references Pirates in popular culture and how Gangplank ironically negotiates (as per the word's definition) by killing his opponents and plundering them.
- *image=Remove Scurvy old.png* references the real-world disease of scurvy, a disease common in sailors caused by a deficiency in Vitamin C. The high Vitamin C content in oranges is in turn a tongue-in-cheek way to say that eating them can "remove scurvy".
- NA Summoner 'SantiagoBR' is the creator of the *'But I ate some oranges and it was K'* meme.
- During Closed Beta 1 of development, Gangplank didn't have a beard. This was changed in Closed Beta 3. An image of him in this state can be viewed here.
- His secondary role is Support.
- In the now-removed official League of Legends forums, the original icon of *image=Parrrley.png* was used to represent the "Bug Reports" section.
- Gangplank is voiced by Matthew Mercer, who also voices Gangplank and **Kindred**.
- He became the first champion to:
- # Receive two visual updates (the first based on his Chinese artwork and the second based on Burning Tides).
- # Have four Classic skins (counting Gangplank before being disabled in-game after 'dying' in Burning Tides).
- # Temporarily replace the announcer in every single map.
- One of the scrapped spells tested during Gangplank's visual gameplay update summoned a "pirate Gundam" that fired mid-range cannonballs from its arm and attacked with a miniature version of *Unstoppable Force*.

---
*This page was automatically generated from League of Legends Wiki data.*