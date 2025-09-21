# Kha'Zix

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
| **Champion** | Kha'Zix |
| **Title** | the Voidreaver |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2012-09-27 |
| **Release Patch** | V1.0.0.148 |
| **Roles** | Assassin |
| **Riot Positions** | Jungle |
| **External Positions** | Jungle |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $643.0$ | $+99.0$ | $2326.0$ |
| **Mana** | $327.0$ | $+40.0$ | $1007.0$ |
| **Health Regen** | $7.5$ | $+0.75$ | $20.2$ |
| **Mana Regen** | $7.59$ | $+0.5$ | $16.1$ |
| **Armor** | $32.0$ | $+4.2$ | $103.4$ |
| **Magic Resist** | $32.0$ | $+2.05$ | $66.8$ |
| **Attack Damage** | $60.0$ | $+3.1$ | $112.7$ |
| **Attack Speed** | $0.668$ | $+2.7\%$ | $0.975$ |
| **Movement Speed** | $350.0$ | $+0.0$ | $350.0$ |
| **Attack Range** | $125.0$ | $+0.0$ | $125.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.668$ |
| **Attack Speed Ratio** | $0.668$ |
| **Bonus AS per Level** | $2.7\%$ |
| **Acquisition Radius** | $400 units$ |
| **Pathing Radius** | $35 units$ |
| **Selection Radius** | $130 units$ |
| **Selection Height** | $150 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Unseen Threat

**Innate:** **Kha'Zix** gains *Unseen Threat* whenever the enemy loses sight of him causing his next basic attack against an enemy Champion to deal bonus magic damage and slow.

**Unseen Threat:** ''Kha'Zix's** next basic attack against an enemy champion deals **bonus'' magic damage and slow them for a short time.

**Innate:** ''Kha'Zix'* gains *Unseen Threat* whenever the enemy loses sight of him or he activates **Void Assault*'. **Unseen Threat:** ''Kha'Zix** empowers his next basic attack against an enemy champion to deal 17 to 136 (+ 50% *bonus AD) **bonus'' magic damage and slow them by 25% for 2 seconds. **Innate:** ''Kha'Zix'* considers any enemy unit to be Isolated if they are not nearby to one of their allies. **Taste Their Fear**, **Evolved Reaper Claws**, and **Evolved Spike Racks*' have special interactions against Isolated targets.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | 375 units |
| **Spell Shield** | False |
| **Spell Effects** | spell |

**Notes:**

- Any form of vision loss may trigger *Unseen Threat*, such as *Curse of the Black Mist*, nearsight and *Brushmaker*.
- *Void Assault* grants *Unseen Threat* even if ''Kha'Zix'' never becomes unseen (e.g. affected by true sight).
  - Other stealth such as E does not do this.
- Both the attack's damage and bonus spell damage are grouped under the same Spell ID.
  - Because of this, a single *Unseen Threat* attack does **not** trigger two *Electrocute* stacks.
- A team's allies are champions, pet, minions and outer turret.
  - Monsters are considered allies for other monsters.
  - Wards do not prevent *Isolation*.
- A number of targetable champion summoned units are specifically tagged to not be a valid ally of a potentially *Isolated* target. These units are:
  - Powder Keg
  - Prophet of an Elder God
  - Captive Audience
  - Bushwhack
  - Tunnel
  - *Mist Wraiths*
  - Noxious Trap
  - *Mist Wraiths*
  - Dark Procession
- Isolation effects are registered at the following timings for each ability:
  - For **Evolved Reaper Claws**, the start of cast.
  - For **Evolved Spike Racks**, when the enemy is hit.
  - For **Taste Their Fear**, the end of cast. This means that if the target is no longer Isolated when the ability hits them, it will not apply the respective bonuses. <!--To do - Interactions with: enemy decoys, hiding in brush (and whether seeing an enemy with Oracle Lens affects it), and stealthed or Senna-wraithed enemies not visible to Kha'Zix's team -->

---

### Q: Evolved Reaper Claws

**Passive:** ''Kha'Zix** gains *range **bonus'* range* on his basic attacks and *Taste Their Fear'.

**Evolved Bonus:** If the target is Isolated, the cooldown is cooldown.

**Passive:** ''Kha'Zix** gains range*bonus'* range* on his basic attacks and *Taste Their Fear'. **Evolved Bonus:** If the target is Isolated, the *cooldown* is reduced by 45%.

| Attribute | Value |
|-----------|-------|
| **Range** | 375 units |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | Single target |

**Notes:**

- No additional details.

---

### Q: Taste Their Fear

**Passive:** Isolated targets are marked by an indicator shown to ''Kha'Zix''.

**Active:** ''Kha'Zix'' slashes the target enemy, dealing physical damage, increased against Isolated targets.

**Passive:** Isolated targets are marked by an indicator shown to ''Kha'Zix''. **Active:** ''Kha'Zix'' slashes the target enemy, dealing physical damage, increased by 110% against Isolated targets.

| Attribute | Value |
|-----------|-------|
| **Range** | 325 units |
| **Cooldown** | 4 seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | 20 Mana |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Effect Radius** | 2500 units |
| **Spell Shield** | True |
| **Spell Effects** | Single target |

**Scaling:**
- **Physical Damage:** $80-180$ (+ 110% bonus AD)

**Notes:**

- The isolation indicator is independent of the actual effects and its timing may not be fully accurate. <!--To do - Isolated indicator. - Interactions with: enemy decoys, hiding in brush (and whether seeing an enemy with Oracle Lens affects it), and stealthed or Senna-wraithed enemies not visible to Kha'Zix's team -->

---

### W: Evolved Spike Racks

**Evolved Bonus:** *Void Spike* now fires three clusters in a cone, slow and standard sight enemy champions hit for a short time.

*The slow is more potent against Isolated targets.*

**Evolved Bonus:** *Void Spike* now fires three clusters in a cone, slow by 40% and standard sight enemy champions hit for 2 seconds. Multiple explosions do not deal extra damage to the same target nor provide ''Kha'Zix'' with additional healing. Isolated targets hit by *Evolved Spike Racks* are slow by 60% instead.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Direction |
| **Affects** | Enemies, Self |
| **Damage Type** | physical |
| **Spell Shield** | Special |
| **Spell Effects** | aoe |
| **Projectile** | True |

**Notes:**

- *Evolved Spike Racks*' effect radius is centered around the location of the missile as it collides.
- Spell shield will not block the sight.

---

### W: Void Spike

**Active:** **Kha'Zix** fires a bolt of spikes in the target direction that explodes upon hitting an enemy, dealing physical damage to nearby enemies.

*''Kha'Zix'' heals himself if he is within the explosion.*

**Active:** ''Kha'Zix'' fires a bolt of spikes in the target direction that explodes upon hitting an enemy, dealing physical damage to nearby enemies. ''Kha'Zix'' heals himself if he is within the explosion.

| Attribute | Value |
|-----------|-------|
| **Range** | 1025 units |
| **Cooldown** | 9 seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $55-75$ Mana |
| **Targeting** | Direction |
| **Affects** | Enemies, Self |
| **Damage Type** | physical |
| **Speed** | 1700 units/second |
| **Effect Radius** | 275 units |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | True |

**Scaling:**
- **Physical Damage:** $85-205$ bonus AD)
- **Heal:** $55-135$ (+ 50% AP)

**Notes:**

- 'Void Spike's effect radius is centered around the location of the missile as it collides.

---

### E: Evolved Wings

**Evolved Bonus:** *Leap* has increased range, and the cooldown cdr upon scoring a champion takedown.

**Evolved Bonus:** *Leap* gains 200 **bonus** cast range, and the *cooldown* resets upon scoring a champion takedown.

| Attribute | Value |
|-----------|-------|
| **Range** | 900 units |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | physical |
| **Spell Shield** | True |
| **Spell Effects** | aoe |

**Notes:**

- ''Kha'Zix'' can use his summoner spells and item actives while dash. **Buffering Interactions**
- *Taste Their Fear* can be buffered to cast after the dash ends if it is cast while the target is out of range. There is no check for the target coming in range during the dash.
  - If the target is still out of range after landing, there will be a $0.5$ second delay before ''Kha'Zix'' starts moving towards the target compared to manually casting *Taste Their Fear* after landing.
- *Void Spike* & *Void Assault* can be buffered to cast after the dash ends.

---

### E: Leap

**Active:** **Kha'Zix** dash to the target location, dealing physical damage to nearby enemies upon landing.

**Active:** ''Kha'Zix'' dash to the target location, dealing physical damage to nearby enemies upon arrival. **Taste Their Fear* can be cast during the dash.*

| Attribute | Value |
|-----------|-------|
| **Range** | 700 units |
| **Cooldown** | $20-12$ seconds |
| **Cast Time** | none |
| **Cost** | 50 Mana |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | physical |
| **Effect Radius** | 300 units |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |

**Scaling:**
- **Physical Damage:** $65-205$ bonus AD)

**Notes:**

- ''Kha'Zix'' can use his summoner spells and item actives while dash. **Buffering Interactions**
- *Taste Their Fear* can be buffered to cast after the dash ends if it is cast while the target is out of range. There is no check for the target coming in range during the dash.
  - If the target is still out of range after landing, there will be a $0.5$ second delay before ''Kha'Zix'' starts moving towards the target compared to manually casting *Taste Their Fear* after landing.
- *Void Spike* & *Void Assault* can be buffered to cast after the dash ends.

---

### R: Evolved Adaptive Cloaking

**Evolved Bonus:** The invisibility is extended, and *Void Assault* can be cast up to 3 times.

**Evolved Bonus:** The invisibility now lasts 2 seconds, and *Void Assault* can be recast twice.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Auto |
| **Affects** | Self |

**Notes:**

No additional notes.

---

### R: Void Assault

**Passive:** Each rank in *Void Assault* allows ''Kha'Zix'' to evolve one of his abilities, granting it additional effects.

**Active:** **Kha'Zix** gains *Unseen Threat* and becomes invisible and gains *ms **bonus** movement speed* for a brief time. Upon breaking invisibility, *Void Assault* can be cast a second time at no additional cost.

**Passive:** Each rank in *Void Assault* allows ''Kha'Zix** to evolve one of his abilities, granting it additional effects. Evolving an ability causes him to enter a 2-second cast time. **Kha'Zix'' cannot evolve while he is unable to cast abilities. **Active:** ''Kha'Zix** gains *Unseen Threat* and becomes invisible for $1.25$ seconds, during which he gains ms*bonus'' movement speed*. After 2 seconds of leaving invisibility, and for the next 12 seconds, *Void Assault* can be recast at no additional cost. **Recast**: ''Kha'Zix'' mimics the first cast's effects.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $100-70$ seconds |
| **Cast Time** | none |
| **Cost** | 100 Mana |
| **Targeting** | Auto |
| **Affects** | Self |

**Notes:**

- ''Kha'Zix'' receives the evolution even if he dies while in cast time.
- Every time *Void Assault* is ranked, a secondary menu will pop up for ''Kha'Zix'' to select an ability to evolve, this can only be done once per ability.
  - The only way for ''Kha'Zix'' to evolve all of his abilities is to gain the fourth evolution point by being victorious in The Hunt is On! (by scoring a takedown on **Rengar**).
- Using a basic attack breaks stealth at the end of the attack windup.

---

## Patch History

### V25.06
- *Unseen Threat*
  - Base damage increased to 17 to 136 from 14 to 116.
  - Bonus AD ratio increased to 50% *bonus AD from 40%.
- *Leap*
  - Bonus AD ratio increased to 40% *bonus AD from 20%.

### V14.23
- Stats
  - Base armor reduced to 32 from 36.
- *Evolved Spike Racks*
  - Isolated slow reduced to 60% from 75%.

### V14.19
- Kha'Zix
  - **Bug Fixes:** Resolved VO audio being too low in certain languages due to the audio filter.

### V14.11
- *Taste Their Fear*
  - Base damage increased to $80-180$ from $70-170$.

### V14.6
- Kha'Zix
  - **Bug Fixes:** Evolving *Void Assault* now correctly changes his textures.

### V13.20
- Kha'Zix
  - Skin renamed to 'Worlds 2018 Kha'Zix* from *Championship Kha'Zix'.

### V13.17
- *Taste Their Fear*
  - Bonus AD ratio reduced to 110% *bonus AD from 115%.
    - Isolated bonus AD ratio reduced to $231$% *bonus AD from 241.5%.

### V13.12
- Stats
  - Base attack damage reduced to 60 from 63.

### V13.8
- *Taste Their Fear*
  - Base damage increased to $70-170$ from $60-160$.

### V13.7
- *Taste Their Fear*
  - Isolation range reduced to 375 units from 425.
- *Evolved Spike Racks*
  - Slow reduced to 40% from 60%.
  - Isolated slow reduced to 75% from 90%.
- *Void Assault*
  - Recast duration increased to 12 seconds from 10.

## Trivia

- Kha'Zix's production name was 'Teek'.
- Ryan 'Morello' Scott stated Kha'Zix is completely unrelated to canceled champion Omen.
- Kha'Zix was the last champion to feature an 'Art Spotlight'.
- When concealing himself (using *Void Assault*) Kha'Zix will run in a different manner than usual.
- When acquiring *Evolved Wings*, Kha'Zix will fly for a few seconds while moving (Ezreal and Kog'Maw do the same).
- The Hunt is On! between Kha'Zix and **Rengar** references Alien vs Predator.
- *Isolation* references Alien (franchise) (Kha'Zix's main inspiration).
- Being an arthropodic invader from another world with scythes as his primary weapon, he also bears a striking resemblance to the real-life mantis & Metroid's Space Pirates, though he also possesses beetles' hardened outer wings.
- *Void Spike*’s projectiles might be referencing a from StarCraft.
- Kha'Zix can spend evolution points on abilities he has not learned yet (doing this only benefits *Taste Their Fear* for the bonus range on basic attacks).
  - Kha'Zix must spend evolution points before ranking up any ability if he has any unspent points after ranking up *Void Assault*.
- Kha'Zix is the only Voidborn champion without true damage on any ability (unlike *Feast*, *Icathian Surprise*, *Endless Banquet*, *Organic Deconstruction*, and *Lifeform Disintegration Ray*).
- Kha'Zix's name means "You face yourself" in an old mortal language, possibly Shuriman or Ixtalian
- Kha'Zix had his game assets reused for several featured game modes.
  - He was given a retexture and features as a monster called the "Jumpy Slashy Monster" in Invasion.
  - His animations were reused for the Kha'Zix monster in Odyssey: Extraction.

---
*This page was automatically generated from League of Legends Wiki data.*