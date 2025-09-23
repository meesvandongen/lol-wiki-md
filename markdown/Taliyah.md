# Taliyah

## Table of Contents
- [Basic Information](#basic-information)
- [Statistics](#statistics)
  - [Map-specific Stats](#map-specific-stats)
- [Abilities](#abilities)
- [Patch History](#patch-history)

## Basic Information

| Attribute | Value |
|-----------|------:|
| **Champion** | Taliyah |
| **Title** | the Stoneweaver |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2016-05-18 |
| **Release Patch** | V6.10 |
| **Latest Changes** | V25.18 |
| **Roles** | Battlemage |
| **Riot Positions** | Jungle, Middle |
| **External Positions** | Jungle, Middle |
| **Blue Essence** | 2400 |
| **Riot Points** | 880 |
| **Difficulty** | 2 |
| **Hero Type** | Mage |
| **Alt Type** | Support |
| **Adaptive Type** | Magic |
| **Damage** | 3 |
| **Toughness** | 1 |
| **Control** | 2 |
| **Mobility** | 1 |
| **Utility** | 3 |
| **Style** | 100 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $550.0$ | $+104.0$ |
| **Mana** | $470.0$ | $+30.0$ |
| **Health Regen** | $6.5$ | $+0.65$ |
| **Mana Regen** | $8.0$ | $+0.8$ |
| **Armor** | $18.0$ | $+4.7$ |
| **Magic Resist** | $28.0$ | $+1.3$ |
| **Attack Damage** | $58.0$ | $+3.3$ |
| **Attack Speed** | $0.658$ | |
| **Movement Speed** | $330.0$ | $+0.0$ |
| **Attack Range** | $525.0$ | $+0.0$ |
| **Base Attack Speed** | $0.658$ | |
| **Attack Speed Ratio** | $0.625$ | |
| **Bonus AS per Level** | $1.4\%$ | |
| **Missile Speed** | $2100$ units/second | |
| **Acquisition Radius** | $550$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $120$ units | |
| **Selection Height** | $145$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $100.0\%$ |

## Abilities

### Passive: Rock Surfing

| Attribute | Value |
|-----------|------:|
| **Static Cooldown** | 2 |
| **Targeting** | Passive |
| **Affects** | Self |

**INNATE:** While near terrain, **Taliyah** gains ms **bonus** movement speed, which builds up over 1 second while in range for at least $0.4$ seconds, and decays at the same rate (See notes) once out of range for at least 3 seconds.

If **Taliyah** is casting an ability or enters champion combat, *Rock Surfing* cannot occur again for a few seconds.

**Notes:**

- The movement speed builds up and decays linearly (by the same amount in the same interval), 25% of the maximum value every $0.25$ seconds.
- *Rock Surfing* can interact with player-generated terrain.
- Dealing default or proc damage does not put *Rock Surfing* on cooldown.

---

### Q: Threaded Volley

| Attribute | Value |
|-----------|------:|
| **Range** | cr 1000 (From Taliyah, when each missile is launched. Missiles originate 50 units in front of Taliyah) |
| **Cast Time** | $0.25$ seconds |
| **Effect Radius** | 175 (Stone Shard radius) / cr 225 (Boulder radius) units |
| **Width** | 200 (Missile width, no offset to either side) units |
| **Speed** | 3600 - 1800 (5000 units per second² deceleration, minimum is 1500 speed but it reaches its destination earlier than it can slow down to it) / 2000 (Empowered missile) |
| **Cost** | 55 / 60 / 65 / 70 / 75 Mana |
| **Cooldown** | 7 / 6 / 5 / 4 / 3 seconds |
| **Queue Time** | $0.5$ seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | magic |
| **Spell Shield** | Special |
| **Spell Effects** | Special |
| **Projectile** | True |

**ACTIVE:** **Taliyah** barrages 5 *Stone Shards* in the target direction over $1.5$ seconds that each shatter upon the first enemy hit, dealing magic damage to nearby enemies and revealing them for $0.5$ seconds. Subsequent hits deal 40% damage.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 55 / 72.5 / 90 / 107.5 / 125 (+ 50% AP) |
| **Reduced Damage** | 22 / 29 / 36 / 43 / 50 (+ 20% AP) |
| **Total Magic Damage** | 143 / 188.5 / 234 / 279.5 / 325 (+ 130% AP) |

**Taliyah** can move and cast other abilities while launching *Stone Shards*, and is unable to basic attack until she launches the third *Stone Shard*.

Casting *Threaded Volley* creates an area of Worked Ground at **Taliyah**’s cast location that has a radius of 400 units and lasts 30 seconds. While on Worked Ground, *Threaded Volley*’s cast consumes the area to become empowered with a new effect, costing 10 mana and having cd reduced cooldown, though not below $0.75$ seconds.

**EMPOWERED ACTIVE:** **Taliyah** hurls a *Boulder* that explodes upon the first enemy hit, dealing 180% damage to them and normal damage to nearby enemies, slowing all targets hit for $1.5$ seconds, and revealing them for $0.5$ seconds. Monsters hit are also stunned for 3 seconds.

| Attribute | Value |
|-----------|------:|
| **Empowered Damage** | 99 / 130.5 / 162 / 193.5 / 225 (+ 90% AP) |

| Attribute | Value |
|-----------|------:|
| **Slow** | 20 / 25 / 30 / 35 / 40% |

Against monsters, *Threaded Volley* deals 20 **bonus** magic damage per *Stone Shard*, and the *Boulder* deals 36 **bonus** magic damage against the primary target. This **bonus** damage is unaffected by the damage modifier from subsequent hits.

**Notes:**

- *Threaded Volley* will continue casting while in stasis.
- The first *Stone Shard* hit on each target is considered area damage, while additional ones of the same cast are considered persistent area damage.
  - The *Boulder* applies area damage.
- The first *Stone Shard* is launched instantly after the cast time ends, then the second and third *Stone Shards* are both launched over 1 second, and finally the fourth and fifth *Stone Shards* are both launched over $0.5$ seconds.
- Spell shield only prevents one instance of damage.

---

### W: Seismic Shove

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 900 units |
| **Effect Radius** | 225 units |
| **Cost** | 40 / 30 / 20 / 10 / 0 Mana |
| **Cooldown** | 14 / 12.5 / 11 / 9.5 / 8 seconds |
| **Queue Time** | $0.5$ seconds |
| **Targeting** | Vector |
| **Affects** | Enemies |
| **Damage Type** | magic |
| **Spell Shield** | True |
| **Spell Effects** | aoe |
| **Out of Range Behavior** | walk to location |

**ACTIVE:** **Taliyah** marks the target location and selects a direction. After $0.792 seconds$ (Under normal tick interval circumstances, both cast time and 0.5s delay are rounding up), a ledge erupts from the area that knocks enemies hit 400 units in the target direction over 1 second.

**Notes:**

- The delay is measured from the start of the cast, before the cast time.

---

### E: Unraveled Earth

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Effect Radius** | 800 units |
| **Angle** | 80° |
| **Cost** | 90 Mana |
| **Cooldown** | 14 seconds |
| **Queue Time** | $0.5$ seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | False |

**ACTIVE:** **Taliyah** scatters a field of 22 stones across the ground in the target direction that deals magic damage to enemies hit while they erupt. The stones then remain for 4 seconds and slow enemies within the area by 20%.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 60 / 105 / 150 / 195 / 240 (+ 60% AP) |

Enemies that dash or are knocked over a stone will detonate it, taking magic damage and becoming stunned for $0.75$ seconds, increased to 2 seconds if they are a monster. The stun is applied once the displacement ends.

| Attribute | Value |
|-----------|------:|
| **Detonation Magic Damage** | 25 / 40 / 55 / 70 / 85 (+ 30% AP) |

An enemy can detonate up to 4 stones, but the damage is reduced by changedisplay=true. *Unraveled Earth* can affect targets only once per cast; the stones will still detonate but not apply their effects.

| Attribute | Value |
|-----------|------:|
| **Total Maximum Detonation Damage** | 62.5 / 100 / 137.5 / 175 / 212.5 (+ 75% AP) |

*Unraveled Earth* deals 190% damage against monsters.

**Notes:**

- *Unraveled Earth* fires 6 rows of stones: the first row has 2 stones, and the rest have 4 each.
  - The stones spawn in rows that cascade in $0.17$ second intervals.
- *Unraveled Earth* will not detonate if the target dashes over the stones while being untargetable.
- *Unraveled Earth* will not detonate against enemies that blink onto the stones. Effect at cast time start

---

### R: Weaver's Wall

| Attribute | Value |
|-----------|------:|
| **Range** | 700 (Jump off dash range) / 1000 (Maximum extended jump off dash range through terrain) units |
| **Cast Time** | none |
| **Target Range** | 2500 / 3500 / 4500 / 5500 / 6500 (Wall length) // 900 (Jump off command) units |
| **Speed** | 1500 / 1200 (Jump off dash speed) units/second |
| **Cost** | 100 Mana |
| **Cooldown** | 180 / 165 / 150 / 135 / 120 (Starts after wall expires or is destroyed) seconds |
| **Cooldown Start** | post-effect |
| **Queue Time** | $0.5$ (Initial cast) seconds |
| **Targeting** | Direction / Auto |
| **Affects** | Self, Enemies |
| **Spell Shield** | False |
| **Projectile** | True |
| **Terrain Grace** | True |
| **Grounded** | Special |
| **Knockdown** | True |
| **Silence** | True |

**ACTIVE:** **Taliyah** summons a torrent of spiraling rocks that cascades in the target direction, knocking aside champions hit and erecting a wall of terrain in its wake. The wall lasts for 4 seconds after completion, then slowly disintegrates from its starting point.

**Taliyah** also channels for 1 second, during which *Weaver's Wall* can be recast and she reveals herself.

**RECAST:** Upon completing the channel, **Taliyah** surfs on the wall as it emerges, revealing herself in the process. After $0.75$ seconds, she may input a movement command to jump off the wall to the target location (Will cast at maximum range (clamped) if ordered location is beyond this jump off command's range), and automatically does so upon being immobilized or silenced or reaching maximum range.

Once the wall has fully formed, **Taliyah** can recast to destroy the wall instantly.

*Weaver's Wall* is placed on a cd static cooldown (Unaffected by ability haste) upon taking champion or turret damage.

**Notes:**

- If a champion is standing next to a portion of the generated terrain and that portion expires by any means, enemies' spell shield will be consumed.
- The initial cast counts as an ability activation for the purposes of on-cast effects such as Spellblade and triggering Force Pulse’s passive.
  - However neither recasts do.
- **Taliyah** can recast to destroy the wall even if she did not surf it or it is in the process of disintegrating.
- **Taliyah** almost always jumps off of the wall to her left from when it reaches maximum range. If she is forced to jump off of the wall, she will do so towards the source that dealt damage or applied the crowd control to her.
- *Weaver's Wall*, while channeling, can be recast while grounded but not while rooted.
  - The channel cannot be started while affected by either crowd control effects.
- Wind Wall and Blade Whirl will prevent *Weaver's Wall* from summoning any further but does not destroy any initial terrain it spawned. If **Taliyah** is surfing during the collision, she immediately becomes stunned for $0.25$ seconds, interrupting the surf.
- If **Taliyah** would enter terrain and the wall reaches maximum range inside, she is preemptively forced off the wall before entering.
  - The wall can surpass the edges of the map, but **Taliyah** cannot.
- The following table refers for interactions while **Taliyah** is channeling:
  - The channel is not interrupted by ground despite being a movement channel.
  - This lockout persists for another $0.65$ seconds during the surf after the channel is completed.
- The following table refers for interactions while **Taliyah** is surfing after the first $0.65$ seconds (Estimated).
  - Once $0.35$ seconds have elapsed afterwards, her lockout is modified for the rest of the surf.
- The following table refers for interactions while **Taliyah** is surfing after 1 second from the channel's completion.
  - Interrupting the surf by casting a non-auto targeted ability, spell, or item active causes her to jump off the wall to the location of where the spell was targeted. She will dash to maximum range if the spell was cast outside of it.

---

## Patch History

### V25.18
- Threaded Volley
  - Initial rock base damage reduced to 55 / 72.5 / 90 / 107.5 / 125 from 56 / 74.5 / 93 / 111.5 / 130.
    - Subsequent rock base damage reduced to 22 / 29 / 36 / 43 / 50 from 22.4 / 29.8 / 37.2 / 44.6 / 52.
    - Boulder base damage reduced to 99 / 130.5 / 162 / 193.5 / 225 from 100.8 / 134.1 / 167.4 / 200.7 / 234.
  - Bonus monster damage increased to 20 from 10.
  - Volley mana cost reduced to 55 / 60 / 65 / 70 / 75 from 65 / 70 / 75 / 80 / 85.
  - Boulder mana cost reduced to 10 from 20.
- Unraveled Earth
  - Detonation base damage reduced to 25 / 40 / 55 / 70 / 85 from 25 / 45 / 65 / 85 / 105.
  - Monster damage increased to 190% from 175%.
  - Mana cost reduced to 90 at all ranks from 90 / 95 / 100 / 105 / 110.

### V25.12
- Stats
  - Base magic resistance redued to 28 from 30.
- Threaded Volley
  - Boulder damage reduced to 180% from 190%.
    - Boulder base damage reduced to 100.8 / 134.1 / 167.4 / 200.7 / 234 from 106.4 / 141.55 / 176.7 / 211.85 / 247.
    - Boulder AP ratio reduced to 90% AP from 95% AP.

### V25.04
- Threaded Volley
  - **Bug Fixes:** Now cancels the remaining tosses upon her False Life triggering.

### V14.16
- Weaver's Wall
  - **Bug Fixes:** Consumables no longer interrupt the dash on the wall.

### V14.15
- General
  - **Bug Fixes:** Restored first encounter VO.

### V14.13
- Threaded Volley
  - Base bonus monster damage reduced to 10 from 25.
  - **Removed:*** Monster damage no longer scales with 5% AP.

### V14.11
- Threaded Volley
  - Base damage reduced to 56 / 74.5 / 93 / 111.5 / 130 from 60 / 78 / 96 / 114 / 132.

### V14.9
- Threaded Volley
  - Mana cost increased to 65 / 70 / 75 / 80 / 85 from 55 / 60 / 65 / 70 / 75.
  - Bonus monster AP ratio reduced to 5% AP from 10% AP.
    - Boulder bonus monster AP ratio reduced to $9.5$% AP from 19% AP.

### V14.8
- General
  - No longer has VO for her dance emote.

### V14.3
- Threaded Volley
  - Base damage increased to 60 / 78 / 96 / 114 / 132 from 50 / 70 / 90 / 110 / 130.
- Unraveled Earth
  - Cooldown reduced to 14 seconds at all ranks from 16 / 15.5 / 15 / 14.5 / 14.

---
*This page was automatically generated from League of Legends Wiki data.*