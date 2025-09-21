# Jhin

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
| **Champion** | Jhin |
| **Title** | the Virtuoso |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2016-02-01 |
| **Release Patch** | V6.2 |
| **Roles** | Marksman, Catcher |
| **Riot Positions** | Bottom |
| **External Positions** | Bottom |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $655.0$ | $+107.0$ | $2474.0$ |
| **Mana** | $300.0$ | $+50.0$ | $1150.0$ |
| **Health Regen** | $3.75$ | $+0.55$ | $13.1$ |
| **Mana Regen** | $6.0$ | $+0.8$ | $19.6$ |
| **Armor** | $24.0$ | $+4.7$ | $103.9$ |
| **Magic Resist** | $30.0$ | $+1.3$ | $52.1$ |
| **Attack Damage** | $59.0$ | $+4.4$ | $133.8$ |
| **Attack Speed** | $0.625$ | $+3.0\%$ | $0.944$ |
| **Movement Speed** | $330.0$ | $+0.0$ | $330.0$ |
| **Attack Range** | $550.0$ | $+0.0$ | $550.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.625$ |
| **Attack Speed Ratio** | $0$ |
| **Bonus AS per Level** | $3.0\%$ |
| **Attack Windup** | $15.6\%$ |
| **Missile Speed** | $2600 units/second$ |
| **Acquisition Radius** | $800 units$ |
| **Pathing Radius** | $35 units$ |
| **Selection Radius** | $120 units$ |
| **Selection Height** | $155 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Whisper

**Innate - Death in 4 Acts:** **Jhin**’s gun may fire up to 4 times before needing to reload. The final round will critically strike, dealing **bonus** physical damage based on the target's **missing** health.

**Innate - Every Moment Matters:** **Jhin** gains **bonus attack damage**, increased by his *critical strike chance* and **bonus attack speed**. Whenever he critically strike, he gains a burst of *movement speed* based on his **bonus attack speed**.

**Innate - Death In 4 Acts:** ''Jhin's' basic attacks consume ammunition within 4 rounds. He will reload over $2.5$ seconds immediately after expending all rounds or withholding leftover rounds after 10 seconds of being out of combat and not being affected by crowd control. The latter reload can be interrupted by declaring an attack or casting an ability. ''Jhin's** final round attack has an uncancellable windup, always critically strike, including against turrets, and deals **bonus'' physical damage equal to key=% of the target's . The damage based on the target's health is capped at 800 against monsters. **Innate - Every Moment Matters:** **Jhin** gains *ad **bonus** attack damage equal to key=%AD. Additionally, critical strikes against enemies grant **Jhin** 14% (+ $0.4$% per 1% *bonus attack speed) **bonus movement speed** for 2 seconds.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical |
| **Spell Effects** | Proc |

**Notes:**

- The penalty to ''Jhin's' critical damage also reduces the base damage ((100 + 75)×0.86 rather than 100 + (75×0.86)) and stacks with other sources (i.e *Infinity Edge*) ((100 + 75 + 40)×0.86).
  - This penalty does not affect *Curtain Call*.
- The damage benefits from *life steal*.
- The 4th attack cannot be canceled by **Jhin** himself.
- The 4th attack versus a structure is classified as a critical strike.
- The attack damage multiplier stacks additively with Dragon Slayer's.
- 'Every Moment Matters' bonus AD will benefit from any amount of *bonus attack speed, even in excess of an amount that would usually reach the attacks per second cap ($3$).
  - The maximum modifier without attack speed is 79%. *** 44% base modifier at level 18. *** 35% from critical strike chance on account of the 100% cap. *** Attack speed slows will not affect ''Jhin's' bonus attack damage from his passive and will ignore his attack speed as it is fixed.
- Attack damage reductions apply before the multiplier causing them to be stronger against **Jhin** than advertised or versus other champions.
- The bonus damage based on the target's missing health does not affect structures.

---

### Q: Dancing Grenade

**Active:** **Jhin** throws a grenade at the target enemy that can bounce to up to three additional nearby enemies, dealing physical damage.

*The grenade's damage increases any time an enemy dies by any means after being hit by the grenade before it strikes its next target.*

**Active:** **Jhin** throws a grenade at the target enemy that deals physical damage and can bounce to up to three additional nearby enemies, prioritizing the closest enemy that has not been hit. The grenade's damage is increased by 35% any time an enemy dies by any means after being hit by the grenade before it strikes its next target.

| Attribute | Value |
|-----------|-------|
| **Range** | 550 units |
| **Cooldown** | $7-5$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $40-60$ Mana |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Speed** | 1800 / 600 units/second |
| **Effect Radius** | 450 units |
| **Spell Shield** | Special |
| **Spell Effects** | spellaoe |
| **Projectile** | True |

**Scaling:**
- **Physical Damage:** $44-144$ (+ 60% AP)
- **Bonus Damage per Target Death:** $(44-144)*0.35$ (+ $(44-74)*0.35$% AD) (+ $60×0.35$% AP)
- **Maximum Final Bounce Physical Damage:** $(44-144)*(1+0.35×3)$ (+ $(44-74)*(1+0.35×3)$% AD) (+ $60*(1+0.35×3)$% AP)

**Notes:**

- Spell shield will not prevent the canister from bouncing.

---

### W: Deadly Flourish

**Passive:** Enemy champions that trigger a **Lotus Trap**, or take damage from **Jhin** or his allies, are marked with *Caught Out* for a few seconds.

**Active:** **Jhin** fires a shot in the target direction that deals physical damage to all enemies in a line until colliding with an enemy champion.

**Passive:** Enemy champions that trigger a **Lotus Trap** to bloom or are damaged by **Jhin** or allied champions are *marked* for 4 seconds. **Active:** **Jhin** fires a shot in the target direction that deals physical damage to all enemies in a line until colliding with an enemy champion, reduced by 25% against minions. Hitting a *marked* champion root them for a duration and grants **Jhin** *Every Moment Matters*’s *ms **bonus** movement speed*.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | 12 seconds |
| **Cast Time** | $0.75$ seconds |
| **Cost** | $50-70$ Mana |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | Special |
| **Projectile** | True |

**Scaling:**
- **Physical Damage:* $60-200$ (+ 50% AD)0.75-200×0.75$ (+ $37.5$% AD)
- **Root Duration:** $1.25-2.25$ seconds

**Notes:**

- Applies spell damage to the enemy champion and area damage to all other enemy units.
- The *Caught Out* mark only appears if *Deadly Flourish* can potentially root the target, and will not if the ability's cooldown is longer than 4 seconds or isn't ranked up. Effect at cast time start
- The ability will not preserve the caster's facing direction when using Flash and similar effects.
- Uniquely, *Deadly Flourish* is special-cased to be projectile by valid counters because it looks like a blockable missile, even though it functionally is not a missile.
- While *disguised* as a non-champion, **Neeko** cannot be affected by *Deadly Flourish*.

---

### E: Captive Audience

**Passive - Beauty in Death:** Killing an enemy champion summons a free *Blooming Lotus Trap* under them.

**Active:** **Jhin** places a stealthed trap *Lotus Trap* at the target location. When an enemy crosses, it will *bloom*, slow and standard sight them. The trap detonates shortly after, dealing magic damage to enemies.

**Passive - Beauty in Death:** Whenever **Jhin** kills an enemy champion, he summons a *Blooming Lotus Trap* on their corpse. **Active:** **Jhin** places a *Lotus Trap* at the target location which, upon landing, becomes stealthed trap after arming over 1 second, lasting for up to 180 seconds and granting sight within its radius. The *Lotus Trap blooms* upon enemy contact, with enemies in the area at the time of its trigger becoming standard sight for 4 seconds. **Jhin** periodically stocks a *Lotus Trap* charge, up to a maximum of 2. *Blooming Lotus Traps* slow enemies within the area by 35% for 2 seconds before exploding, dealing magic damage to enemies. *Lotus Traps* deal 65% damage against minions as well as against champions and monsters who have been struck by another *Lotus Trap* in the last 1 second. *See [Pets](#Pets) for more details about Lotus Traps.*

| Attribute | Value |
|-----------|-------|
| **Range** | 750 units |
| **Cooldown** | 2 seconds |
| **Recharge** | $24-14$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | 30 Mana + 1 Charge |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | 260 / 260 units |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | Special |

**Notes:**

- *Beauty in Death* does not consume *Lotus Trap* charges.
- The *Lotus Trap* will stop upon encountering a *Wind Wall* or *Unbreakable*.
- *Beauty in Death* does not trigger after killing a clone.
- The reveal debuff is named *Caught Out*.
  - This debuff is shared between *Captive Audience* and *Curtain Call*.

---

### R: Curtain Call

**Active:** **Jhin** channels up to a period, being able to fire up to four shots in a large cone in front of him.

**Recast:** **Jhin** fires a round in the target direction that deals physical damage to enemies hit based on their **missing** health. It stops upon hitting an enemy champion, slow and standard sight them for a short time.

**Active:** **Jhin** channel for up to 10 seconds, transforming his weapon into a cannon and gaining the ability to recast *Curtain Call* 4 times within the duration. **Recast:** After $0.2$ seconds into the cast time, **Jhin** fires a round in the target direction that grants sight around its trajectory for $0.5$ seconds and deals physical damage to enemies hit, increased by type=target's **missing** health. The bullet stops upon hitting an enemy champion, slow them by 80% for $0.5$ seconds and standard sight them for 2 seconds. Each cast has a static cooldown of 1 second. The fourth shot critical strike for damage.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $120-90$ seconds |
| **Cast Time** | 1 / $0.25$ seconds |
| **Cost** | 100 Mana |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Speed** | 5000 units/second |
| **Spell Shield** | True |
| **Spell Effects** | Special |
| **Projectile** | True |

**Scaling:**
- **Minimum Physical Damage per Bullet:* $64-192$ (+ 25% AD)4-192×4$ (+ $25×4$% AD)
- **Minimum Fourth Shot Damage:** $64×2-192×2$ (+ $252$% AD)2)*4-(192×2)*4$ (+ $(25×2)*4$% AD)

**Notes:**

- Applies spell damage to enemy champions and area damage to other enemy units. *Each cast counts as an ability activation for the purposes of on-cast effects such as Spellblade and triggering *Force Pulse*’s passive.
- 'Curtain Call's shooting range is fixed in front of **Jhin**.
- The opening sound effect and area of range are audible and visible to both teams.
- Leveling up the ability while channeling will change the final cooldown.
- **Jhin** will turn to face in the target direction whenever he uses a recast.
- **Jhin** gains a wider field of view during *Curtain Call*.
- The reveal debuff is named *Caught Out*.
  - This debuff is shared between *Captive Audience* and *Curtain Call*. Effect at cast time end
- The following table refers for interactions while **Jhin** is channel:

---

### A

---

### Basic Attack

**Basic Attack:** **Jhin** fires at the target with *Whisper*, dealing 100% AD physical damage, applying on-hit effects, and triggering on-attack effects. ''Jhin's' basic attacks can critically strike. These attacks have slightly increased missile speed, but deal only AD physical damage. $% of the *critical damage* champions usually have. ''Jhin's' attack speed cannot be improved at all except through growth.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Speed** | Non-critical attacks / 3000 units/second |
| **Spell Effects** | attack |
| **Projectile** | True |

**Notes:**

- The 3000 missile speed for critical strikes also applies to *Whisper*’s 4th shot, which is a guaranteed critical strike.

---

## Patch History

### V25.14
- *Whisper*
  - **Bug Fixes:** Now properly notes the damage cap against monsters.

### V25.12
- *Deadly Flourish*
  - **Undocumented / New Effect:** Now has projectile interactions under all circumstances. Previously, Wind Wall was exclusively special-cased to intercept it.

### V25.06
- *Whisper*
  - Bonus attack damage critical strike chance ratio increased to $0.35$% per 1% critical strike chance from $0.3$% per 1% critical strike chance.
  - Bonus attack damage attack speed ratio increased to $0.3$% per 1% *bonus attack speed from $0.25$% per 1% *bonus attack speed.

### V14.20
- Stats
  - Attack damage growth reduced to $4.4$ from $4.7$.
- *Curtain Call*
  - Minimum base damage reduced to $64-192 3$ from $64-244 3$.
    - Maximum base damage reduced to $64×4-192×4 3$ from $64×4-244×4 3$.

### V14.19
- *Captive Audience*
  - **Bug Fixes:** Triggering the trap over an area wherein an enemy ward is placed on top of an ally ward no longer temporarily reveals the enemy ward for the duration of the trap's effect.

### V14.8
- *Whisper*
  - Base bonus movement speed increased to 14% from 10%.
- *Dancing Grenade*
  - Minimum base damage reduced to $44-144$ from $45-145$.
  - Minimum AD ratio increased to $44-74$% AD from $35-65$% AD.

### V14.2
- *Whisper*
  - **Bug Fixes:** Fourth shot no longer fails if it is fired right before it expires.
  - **Undocumented / New Effect:** Fourth shot now has an uncancellable windup against all targets instead of against champions only.

### V13.19
- General
  - **Bug Fixes:** Now plays the correct animation for his running instead of using the one for homeguard.
- *Curtain Call*
  - Minimum base damage increased to $64-244 3$ from $50-200 3$.

### V13.17
- Jhin
  - **Bug Fixes:** Now properly has a pulsating overlay.
- Jhin
  - *Whisper* and *Curtain Call*
    - **Bug Fixes:** Voiceover playback for ability casts have been corrected.

### V13.14
- *Deadly Flourish*
  - **Bug Fixes:** VFX now appears correctly if the target was displaced after being rooted.

## Trivia

- Jhin's Deutsch/German title, *Der Virtuose*, is the masculine form of **Sona**’s title, *Die Virtuosin*.
- His dance references the Lezginka dance.
  - A side-by-side comparison can be seen here.
- Jhin is the first champion who does not actually appear in his own Splash Art.
  - Jhin is the second champion to have a Loading screen and Portrait icon not focus the actual champion, in his case, focus on the mirror image. The first one is **LeBlanc**.
- Jhin is the first champion to feature a takedown animation on his Classic skin, via *Beauty in Death*. It should be noted however that the passive effect is not solely for cosmetic purposes.
  - Despite being the only one seen in-game, the rose is just one of the many things he sees when killing someone.
- Occasionally when playing as Jhin, at the start of the game (between your team spawning at the fountain and then minions doing so at nexus) the player can hear a version of the map's theme that fits the style of Jhin's own.
- As Jhin, triggering *The Collector* Death and Taxes will cause the damage indicator to display as "4444" instead of "999". This change was added on V12.19.
- If Jhin loses the Golden Ratio quest to **Hwei**, he gains the "A Mask Exposed" buff whose icon is a reference to the Masked Crying Wojak meme.
- Jhin's cost is 1234.

---
*This page was automatically generated from League of Legends Wiki data.*