# Taliyah

## Table of Contents
- [Basic Information](#basic-information)
- [Statistics](#statistics)
  - [Advanced Stats](#advanced-stats)
- [Abilities](#abilities)
- [Patch History](#patch-history)

## Basic Information

| Attribute | Value |
|-----------|-------|
| **Champion** | Taliyah |
| **Title** | the Stoneweaver |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2016-05-18 |
| **Release Patch** | V6.10 |
| **Roles** | Battlemage |
| **Riot Positions** | Jungle, Middle |
| **External Positions** | Jungle, Middle |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $550.0$ | $+104.0$ | $2318.0$ |
| **Mana** | $470.0$ | $+30.0$ | $980.0$ |
| **Health Regen** | $6.5$ | $+0.65$ | $17.6$ |
| **Mana Regen** | $8.0$ | $+0.8$ | $21.6$ |
| **Armor** | $18.0$ | $+4.7$ | $97.9$ |
| **Magic Resist** | $28.0$ | $+1.3$ | $50.1$ |
| **Attack Damage** | $58.0$ | $+3.3$ | $114.1$ |
| **Attack Speed** | $0.658$ | $+1.4\%$ | $0.810$ |
| **Movement Speed** | $330.0$ | $+0.0$ | $330.0$ |
| **Attack Range** | $525.0$ | $+0.0$ | $525.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.658$ |
| **Attack Speed Ratio** | $0.625$ |
| **Bonus AS per Level** | $1.4\%$ |
| **Missile Speed** | $2100 units/second$ |
| **Acquisition Radius** | $550 units$ |
| **Pathing Radius** | $35 units$ |
| **Selection Radius** | $120 units$ |
| **Selection Height** | $145 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Rock Surfing

**Innate:** **Taliyah** gains *ms **bonus** movement speed* while near terrain. This effect will be put on *cooldown* while in combat or casting an ability.

**Innate:** While near terrain, **Taliyah** gains ms **bonus** movement speed, which builds up over 1 second while in range for at least $0.4$ seconds, and decays at the same rate once out of range for at least 3 seconds. If **Taliyah** is casting an ability or enters champion combat, *Rock Surfing* cannot occur again for a few seconds.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Self |

**Notes:**

- The movement speed builds up and decays linearly (by the same amount in the same interval), 25% of the maximum value every $0.25$ seconds.
- *Rock Surfing* can interact with player-generated terrain.
- Dealing default damage or proc damage does not put *Rock Surfing* on cooldown.

---

### Q: Threaded Volley

**Active:** **Taliyah** barrages out 5 *Stone Shards* that each explode upon the first enemy hit, dealing magic damage to nearby enemies. This also creates an area of Worked Ground under ''Taliyah's' location that lasts for a while.

*Casting *Threaded Volley* on Worked Ground will consume the area to become empowered with a new effect as well as reduced *mana cost* and *cooldown*.*

**Active:** **Taliyah** barrages 5 *Stone Shards* in the target direction over $1.5$ seconds that each shatter upon the first enemy hit, dealing magic damage to nearby enemies and standard sight them for $0.5$ seconds. Subsequent hits deal 40% damage. **Taliyah** can move and cast other abilities while launching *Stone Shards*, and is unable to basic attack until she launches the third *Stone Shard*. Casting *Threaded Volley* creates an area of Worked Ground at ''Taliyah's* cast location that has a radius of 400 units and lasts 30 seconds. While on Worked Ground, *Threaded Volley's cast consumes the area to become empowered with a new effect, costing *10 mana* and having cd reduced cooldown, though not below $0.75$ seconds. **Empowered Active:** **Taliyah** hurls a *Boulder* that explodes upon the first enemy hit, dealing 180% damage to them and normal damage to nearby enemies, slow all targets hit for $1.5$ seconds, and standard sight them for $0.5$ seconds. Monsters hit are also stun for 3 seconds. Against monsters, *Threaded Volley* deals 20 **bonus** magic damage per *Stone Shard*, and the *Boulder* deals $20×1.8$ **bonus** magic damage against the primary target. This **bonus** damage is unaffected by the damage modifier from subsequent hits.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $7-3$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $55-75$ Mana |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | magic |
| **Speed** | 3600 - 1800 / 2000 units/second |
| **Effect Radius** | 175 / cr 225 units |
| **Spell Shield** | Special |
| **Spell Effects** | Special |
| **Projectile** | True |

**Scaling:**
- **Magic Damage:* $55-125$ (+ 50% AP)0.4-125×0.4$ (+ $500.4$% AP)2.6-125×2.6$ (+ 130% AP)

**Notes:**

- *Threaded Volley* will continue casting while in stasis.
- The first *Stone Shard* hit on each target is considered area damage, while additional ones of the same cast are considered persistent area damage.
  - The *Boulder* applies area damage.
- The first *Stone Shard* is launched instantly after the cast time ends, then the second and third *Stone Shards* are both launched over 1 second, and finally the fourth and fifth *Stone Shards* are both launched over $0.5$ seconds.
- Spell shield only prevents one instance of damage.

---

### W: Seismic Shove

**Active:** **Taliyah** creates a ledge at the target location that erects after a brief delay, briefly airborne enemies hit in the target direction.

**Active:** **Taliyah** marks the target location and selects a direction. After $0.792 seconds$, a ledge erupts from the area that airborne enemies hit 400 units in the target direction over 1 second.

| Attribute | Value |
|-----------|-------|
| **Range** | 900 units |
| **Cooldown** | $14-8$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $40-0$ Mana |
| **Targeting** | Vector |
| **Affects** | Enemies |
| **Damage Type** | magic |
| **Effect Radius** | 225 units |
| **Spell Shield** | True |
| **Spell Effects** | aoe |

**Notes:**

- The delay is measured from the start of the cast, before the cast time.

---

### E: Unraveled Earth

**Active:** **Taliyah** scatters a field of stones in the target direction that deals magic damage to enemies hit. The stones remain for a few seconds and slow enemies within.

*Enemies that dash or are airborne into a stone will detonate it, dealing magic damage and stun them for a short time.*

**Active:** **Taliyah** scatters a field of 22 stones across the ground in the target direction that deals magic damage to enemies hit while they erupt. The stones then remain for 4 seconds and slow enemies within the area by 20%. Enemies that dash or are airborne over a stone will detonate it, taking magic damage and becoming stun for $0.75$ seconds, increased to 2 seconds if they are a monster. The stun is applied once the displacement ends. An enemy can detonate up to 4 stones, but the damage is reduced by changedisplay=true. *Unraveled Earth* can affect targets only once per cast; the stones will still detonate but not apply their effects. *Unraveled Earth* deals 190% damage against monsters.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | 14 seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | 90 Mana |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | magic |
| **Effect Radius** | 800 units |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | False |

**Scaling:**
- **Magic Damage:** $60-240$ (+ 60% AP)
- **Detonation Magic Damage:** $25-85$ (+ 30% AP)
- **Total Maximum Detonation Damage:** $25*(1+0.75+0.5+0.25)-85*(1+0.75+0.5+0.25)$ (+ 75% AP)

**Notes:**

- *Unraveled Earth* fires 6 rows of stones: the first row has 2 stones, and the rest have 4 each.
  - The stones spawn in rows that cascade in $0.17$ second intervals.
- *Unraveled Earth* will not detonate if the target dashes over the stones while being untargetable.
- *Unraveled Earth* will not detonate against enemies that blink onto the stones. Effect at cast time start

---

### R: Weaver's Wall

**Active:** **Taliyah** summons a torrent of spiraling rocks that cascades in the target direction, which airborne champions hit and erects a wall of terrain in its wake. The wall lasts a few seconds before slowly disintegrating from the starting end.

**Taliyah** will also briefly channel, during which 'Weaver's Wall' can be recast. While the wall remains fully formed, she gains another recast empowered with a new effect.

**Active:** **Taliyah** summons a torrent of spiraling rocks that cascades in the target direction, airborne champions hit and erecting a wall of terrain in its wake. The wall lasts for 4 seconds after completion, then slowly disintegrates from its starting point. **Taliyah** also channels for 1 second, during which 'Weaver's Wall' can be recast and she sight herself. **Recast:** Upon completing the channel, **Taliyah** dash on the wall as it emerges, sight herself in the process. After $0.75$ seconds, she may input a movement command to dash off the wall to the target location, and automatically does so upon being immobilize or silence or reaching maximum range. Once the wall has fully formed, **Taliyah** can recast to destroy the wall instantly. 'Weaver's Wall' is placed on a cd static cooldown upon taking champion or turret damage.

| Attribute | Value |
|-----------|-------|
| **Range** | $2500-6500$ // 900 units |
| **Cooldown** | $180-120$ seconds |
| **Cast Time** | none |
| **Cost** | 100 Mana |
| **Targeting** | Direction / Auto |
| **Affects** | Self, Enemies |
| **Speed** | 1500 / 1200 units/second |
| **Spell Shield** | False |
| **Projectile** | True |

**Notes:**

- If a champion is standing next to a portion of the generated terrain and that portion expires by any means, enemies' spell shield will be consumed.
- The initial cast counts as an ability activation for the purposes of on-cast effects such as Spellblade and triggering *Force Pulse*’s passive.
  - However neither recasts do.
- **Taliyah** can recast to destroy the wall even if she did not surf it or it is in the process of disintegrating.
- **Taliyah** almost always jumps off of the wall to her left from when it reaches maximum range. If she is forced to jump off of the wall, she will do so towards the source that dealt damage or applied the crowd control to her.
- 'Weaver's Wall', while channel, can be recast while ground but not while root.
  - The channel cannot be started while affected by either crowd control effects.
- *Wind Wall* and *Blade Whirl* will prevent 'Weaver's Wall' from summoning any further but does not destroy any initial terrain it spawned. If **Taliyah** is surfing during the collision, she immediately becomes stun for $0.25$ seconds, interrupting the surf.
- If **Taliyah** would enter terrain and the wall reaches maximum range inside, she is preemptively forced off the wall before entering.
  - The wall can surpass the edges of the map, but **Taliyah** cannot.
- The following table refers for interactions while **Taliyah** is channel:
  - The channel is not interrupt by ground despite being a movement channel.
  - This lockout persists for another $0.65$ seconds during the surf after the channel is completed.
- The following table refers for interactions while **Taliyah** is dash after the first $0.65$ seconds.
  - Once $0.35$ seconds have elapsed afterwards, her lockout is modified for the rest of the surf.
- The following table refers for interactions while **Taliyah** is dash after 1 second from the channel's completion.
  - Interrupting the surf by casting a non-auto targeted ability, spell, or item active causes her to jump off the wall to the location of where the spell was targeted. She will dash to maximum range if the spell was cast outside of it.

---

## Patch History

### V25.18
- *Threaded Volley*
  - Initial rock base damage reduced to $55-125$ from $56-130$.
    - Subsequent rock base damage reduced to $55×0.4-125×0.4$ from $56×0.4-130×0.4$.
    - Boulder base damage reduced to $55×1.8-125×1.8$ from $56×1.8-130×1.8$.
  - Bonus monster damage increased to 20 from 10.
  - Volley mana cost reduced to $55-75$ from $65-85$.
  - Boulder mana cost reduced to 10 from 20.
- *Unraveled Earth*
  - Detonation base damage reduced to $25-85$ from $25-105$.
  - Monster damage increased to 190% from 175%.
  - Mana cost reduced to 90 at all ranks from $90-110$.

### V25.12
- Stats
  - Base magic resistance redued to 28 from 30.
- *Threaded Volley*
  - Boulder damage reduced to 180% from 190%.
    - Boulder base damage reduced to $56×1.8-130×1.8$ from $56×1.9-130×1.9$.
    - Boulder AP ratio reduced to $50×1.8$% AP from $50×1.9$% AP.

### V25.04
- *Threaded Volley*
  - **Bug Fixes:** Now cancels the remaining tosses upon her False Life triggering.

### V14.16
- *Weaver's Wall*
  - **Bug Fixes:** Consumables no longer interrupt the dash on the wall.

### V14.15
- General
  - **Bug Fixes:** Restored first encounter VO.

### V14.13
- *Threaded Volley*
  - Base bonus monster damage reduced to 10 from 25.
  - **Removed:*** Monster damage no longer scales with 5% AP.

### V14.11
- *Threaded Volley*
  - Base damage reduced to $56-130$ from $60-132$.

### V14.9
- *Threaded Volley*
  - Mana cost increased to $65-85$ from $55-75$.
  - Bonus monster AP ratio reduced to 5% AP from 10% AP.
    - Boulder bonus monster AP ratio reduced to $9.5$% AP from 19% AP.

### V14.8
- General
  - No longer has VO for her dance emote.

### V14.3
- *Threaded Volley*
  - Base damage increased to $60-132$ from $50-130$.
- *Unraveled Earth*
  - Cooldown reduced to 14 seconds at all ranks from $16-14$.

---
*This page was automatically generated from League of Legends Wiki data.*