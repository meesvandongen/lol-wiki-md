# Kha'Zix

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
| **Champion** | Kha'Zix |
| **Title** | the Voidreaver |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2012-09-27 |
| **Release Patch** | V1.0.0.148 |
| **Latest Changes** | V25.06 |
| **Roles** | Assassin |
| **Riot Positions** | Jungle |
| **External Positions** | Jungle |
| **Blue Essence** | 2400 |
| **Riot Points** | 880 |
| **Difficulty** | 2 |
| **Hero Type** | Assassin |
| **Adaptive Type** | Physical |
| **Damage** | 3 |
| **Toughness** | 1 |
| **Control** | 1 |
| **Mobility** | 2 |
| **Utility** | 1 |
| **Style** | 55 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $643.0$ | $+99.0$ |
| **Mana** | $327.0$ | $+40.0$ |
| **Health Regen** | $7.5$ | $+0.75$ |
| **Mana Regen** | $7.59$ | $+0.5$ |
| **Armor** | $32.0$ | $+4.2$ |
| **Magic Resist** | $32.0$ | $+2.05$ |
| **Attack Damage** | $60.0$ | $+3.1$ |
| **Attack Speed** | $0.668$ | |
| **Movement Speed** | $350.0$ | $+0.0$ |
| **Attack Range** | $125.0$ | $+0.0$ |
| **Base Attack Speed** | $0.668$ | |
| **Attack Speed Ratio** | $0.668$ | |
| **Bonus AS per Level** | $2.7\%$ | |
| **Acquisition Radius** | $400$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $130$ units | |
| **Selection Height** | $150$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $110.0\%$ |
| **Damage Taken** | $90.0\%$ |
| **Healing** | $120.0\%$ |

## Abilities

### Passive: Unseen Threat

| Attribute | Value |
|-----------|------:|
| **Effect Radius** | 375 (Non-isolated radius between enemies / monsters) units |
| **Targeting** | Passive |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | False |
| **Spell Effects** | spell |
| **Parry** | True |

**INNATE:** **Kha'Zix** gains *Unseen Threat* whenever the enemy loses sight of him or he activates *Void Assault*.

**UNSEEN THREAT:** **Kha'Zix** empowers his next basic attack against an enemy champion to deal 17 to 136 (+ 50% **bonus** AD) **bonus** magic damage and slow them by 25% for 2 seconds.

**INNATE:** **Kha'Zix** considers any enemy unit to be Isolated if they are not nearby to one of their allies. *Taste Their Fear*, *Evolved Reaper Claws*, and *Evolved Spike Racks* have special interactions against Isolated targets.

**Notes:**

- Any form of vision loss may trigger *Unseen Threat*, such as Curse of the Black Mist, nearsight and Brushmaker.
- Void Assault grants *Unseen Threat* even if **Kha'Zix** never becomes unseen (e.g. affected by true sight).
  - Other stealth such as E does not do this.
- Both the attack's damage and bonus spell damage are grouped under the same Spell ID.
  - Because of this, a single *Unseen Threat* attack does **not** trigger two Electrocute stacks.
- A team's allies are champions, pets, minions and outer turrets.
  - Monsters are considered allies for other monsters.
  - Wards do not prevent *Isolation*.
- A number of targetable champion summoned units are specifically tagged to not be a valid ally of a potentially *Isolated* target. These units are:
  - Powder Keg
  - Prophet of an Elder God
  - Captive Audience
  - Bushwhack
  - Tunnel
  - Absolution’s *Mist Wraiths*
  - Noxious Trap
  - Sovereign's Domination’s *Mist Wraiths*
  - Dark Procession
- Isolation effects are registered at the following timings for each ability:
  - For *Evolved Reaper Claws*, the start of cast.
  - For *Evolved Spike Racks*, when the enemy is hit.
  - For *Taste Their Fear*, the end of cast. This means that if the target is no longer Isolated when the ability hits them, it will not apply the respective bonuses. <!--To do - Interactions with: enemy decoys, hiding in brush (and whether seeing an enemy with Oracle Lens affects it), and stealthed or Senna-wraithed enemies not visible to Kha'Zix's team -->

---

### Q: Evolved Reaper Claws

| Attribute | Value |
|-----------|------:|
| **Target Range** | 375 units |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | Single target |
| **Call For Help** | True |

**PASSIVE:** **Kha'Zix** gains range on his basic attacks and *Taste Their Fear*. 

**EVOLVED BONUS:** If the target is Isolated, the cooldown is reduced by 45%.

**Notes:**

- No additional details.

---

### Q: Taste Their Fear

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 325 units |
| **Effect Radius** | 2500 (Isolated indicator range, estimated) units |
| **Cost** | 20 Mana |
| **Cooldown** | 4 seconds |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | Single target |
| **Call For Help** | True |

**PASSIVE:** Isolated targets are marked by an indicator shown to **Kha'Zix**.

**ACTIVE:** **Kha'Zix** slashes the target enemy, dealing physical damage, increased by 110% against Isolated targets.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 80 / 105 / 130 / 155 / 180 (+ 110% **bonus** AD) |
| **Increased Damage** | 168 / 220.5 / 273 / 325.5 / 378 (+ 231% **bonus** AD) |

**Notes:**

- The isolation indicator is independent of the actual effects and its timing may not be fully accurate. <!--To do - Isolated indicator. - Interactions with: enemy decoys, hiding in brush (and whether seeing an enemy with Oracle Lens affects it), and stealthed or Senna-wraithed enemies not visible to Kha'Zix's team -->

---

### W: Evolved Spike Racks

| Attribute | Value |
|-----------|------:|
| **Targeting** | Direction |
| **Affects** | Enemies, Self |
| **Damage Type** | physical |
| **Spell Shield** | Special |
| **Spell Effects** | aoe |
| **Projectile** | True |

**EVOLVED BONUS:** *Void Spike* now fires three clusters in a cone, slows by 40% and reveals enemy champions hit for 2 seconds. Multiple explosions do not deal extra damage to the same target nor provide **Kha'Zix** with additional healing.

Isolated targets hit by *Evolved Spike Racks* are slowed by 60% instead.

**Notes:**

- *Evolved Spike Racks** effect radius is centered around the location of the missile as it collides.
- Spell shield will not block the reveal.

---

### W: Void Spike

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 1025 units |
| **Effect Radius** | 275 units |
| **Width** | 140 units |
| **Speed** | 1700 units/second |
| **Cost** | 55 / 60 / 65 / 70 / 75 Mana |
| **Cooldown** | 9 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies, Self |
| **Damage Type** | physical |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | True |

**ACTIVE:** **Kha'Zix** fires a bolt of spikes in the target direction that explodes upon hitting an enemy, dealing physical damage to nearby enemies.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 85 / 115 / 145 / 175 / 205 (+ 100% **bonus** AD) |

**Kha'Zix** heals himself if he is within the explosion.

| Attribute | Value |
|-----------|------:|
| **Heal** | 55 / 75 / 95 / 115 / 135 (+ 50% AP) |

**Notes:**

- *Void Spike*’s effect radius is centered around the location of the missile as it collides.

---

### E: Evolved Wings

| Attribute | Value |
|-----------|------:|
| **Target Range** | 900 units |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | physical |
| **Spell Shield** | True |
| **Spell Effects** | aoe |
| **Out of Range Behavior** | cast at max |
| **Grounded** | True |
| **Knockdown** | True |

**EVOLVED BONUS:** *Leap* gains 200 **bonus** cast range, and the cooldown resets upon scoring a champion takedown.

**Notes:**

- **Kha'Zix** can use his summoner spells and item actives while leaping. **Buffering Interactions**
- Taste Their Fear can be buffered to cast after the leap ends if it is cast while the target is out of range. There is no check for the target coming in range during the leap.
  - If the target is still out of range after landing, there will be a $0.5$ second delay before **Kha'Zix** starts moving towards the target compared to manually casting Taste Their Fear after landing.
- Void Spike & Void Assault can be buffered to cast after the leap ends.

---

### E: Leap

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 700 units |
| **Effect Radius** | 300 units |
| **Cost** | 50 Mana |
| **Cooldown** | 20 / 18 / 16 / 14 / 12 seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | physical |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Out of Range Behavior** | cast at max |
| **Grounded** | True |
| **Knockdown** | True |

**ACTIVE:** **Kha'Zix** leaps to the target location, dealing physical damage to nearby enemies upon arrival.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 65 / 100 / 135 / 170 / 205 (+ 40% **bonus** AD) |

*Taste Their Fear can be cast during the dash.*

**Notes:**

- **Kha'Zix** can use his summoner spells and item actives while leaping. **Buffering Interactions**
- Taste Their Fear can be buffered to cast after the leap ends if it is cast while the target is out of range. There is no check for the target coming in range during the leap.
  - If the target is still out of range after landing, there will be a $0.5$ second delay before **Kha'Zix** starts moving towards the target compared to manually casting Taste Their Fear after landing.
- Void Spike & Void Assault can be buffered to cast after the leap ends.

---

### R: Evolved Adaptive Cloaking

| Attribute | Value |
|-----------|------:|
| **Targeting** | Auto |
| **Affects** | Self |

**EVOLVED BONUS:** The invisibility now lasts 2 seconds, and *Void Assault* can be recast twice.

**Notes:**

No additional notes.

---

### R: Void Assault

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Cost** | 100 Mana |
| **Cooldown** | 100 / 92.5 / 85 / 77.5 / 70 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Auto |
| **Affects** | Self |

**PASSIVE:** Each rank in *Void Assault* allows **Kha'Zix** to evolve one of his abilities, granting it additional effects. Evolving an ability causes him to enter a 2-second cast time. **Kha'Zix** cannot evolve while he is unable to cast abilities.

**ACTIVE:** **Kha'Zix** gains Unseen Threat and becomes invisible for 1.25 seconds, during which he gains ms.

After 2 seconds of leaving invisibility, and for the next 12 seconds, *Void Assault* can be recast at no additional cost.

**RECAST**: **Kha'Zix** mimics the first cast's effects.

**Notes:**

- **Kha'Zix** receives the evolution even if he dies while in cast time.
- Every time *Void Assault* is ranked, a secondary menu will pop up for **Kha'Zix** to select an ability to evolve, this can only be done once per ability.
  - The only way for **Kha'Zix** to evolve all of his abilities is to gain the fourth evolution point by being victorious in The Hunt is On! (by scoring a takedown on Rengar).
- Using a basic attack breaks stealth at the end of the attack windup.

---

## Patch History

### V25.06
- Unseen Threat
  - Base damage increased to 17 to 136 from 14 to 116.
  - Bonus AD ratio increased to 50% **bonus** AD from 40%.
- Leap
  - Bonus AD ratio increased to 40% **bonus** AD from 20%.

### V14.23
- Stats
  - Base armor reduced to 32 from 36.
- Evolved Spike Racks
  - Isolated slow reduced to 60% from 75%.

### V14.19
- Kha'Zix
  - **Bug Fixes:** Resolved VO audio being too low in certain languages due to the audio filter.

### V14.11
- Taste Their Fear
  - Base damage increased to 80 / 105 / 130 / 155 / 180 from 70 / 95 / 120 / 145 / 170.

### V14.6
- Kha'Zix
  - **Bug Fixes:** Evolving Void Assault now correctly changes his textures.

### V13.20
- Kha'Zix
  - Skin renamed to *Worlds 2018 Kha'Zix* from *Championship Kha'Zix*.

### V13.17
- Taste Their Fear
  - Bonus AD ratio reduced to 110% **bonus** AD from 115%.
    - Isolated bonus AD ratio reduced to $231$% **bonus** AD from 241.5%.

### V13.12
- Stats
  - Base attack damage reduced to 60 from 63.

### V13.8
- Taste Their Fear
  - Base damage increased to 70 / 95 / 120 / 145 / 170 from 60 / 85 / 110 / 135 / 160.

### V13.7
- Taste Their Fear
  - Isolation range reduced to 375 units from 425.
- Evolved Spike Racks
  - Slow reduced to 40% from 60%.
  - Isolated slow reduced to 75% from 90%.
- Void Assault
  - Recast duration increased to 12 seconds from 10.

## Trivia

- Kha'Zix's production name was 'Teek'.
- Ryan 'Morello' Scott stated Kha'Zix is completely unrelated to canceled champion Omen.
- Kha'Zix was the last champion to feature an 'Art Spotlight'.
- When concealing himself (using Void Assault) Kha'Zix will run in a different manner than usual.
- When acquiring Evolved Wings, Kha'Zix will fly for a few seconds while moving (Ezreal and Kog'Maw do the same).
- The Hunt is On! between Kha'Zix and Rengar references Alien vs Predator.
- Isolation references Alien (franchise) (Kha'Zix's main inspiration).
- Being an arthropodic invader from another world with scythes as his primary weapon, he also bears a striking resemblance to the real-life mantis & Metroid's Space Pirates, though he also possesses beetles' hardened outer wings.
- Void Spike’s projectiles might be referencing a from StarCraft.
- Kha'Zix can spend evolution points on abilities he has not learned yet (doing this only benefits Taste Their Fear for the bonus range on basic attacks).
  - Kha'Zix must spend evolution points before ranking up any ability if he has any unspent points after ranking up Void Assault.
- Kha'Zix is the only Voidborn champion without true damage on any ability (unlike Feast, Icathian Surprise, Endless Banquet, Organic Deconstruction, and Lifeform Disintegration Ray).
- Kha'Zix's name means "You face yourself" in an old mortal language, possibly Shuriman or Ixtalian
- Kha'Zix had his game assets reused for several featured game modes.
  - He was given a retexture and features as a monster called the "Jumpy Slashy Monster" in Invasion.
  - His animations were reused for the Kha'Zix monster in Odyssey: Extraction.

---
*This page was automatically generated from League of Legends Wiki data.*