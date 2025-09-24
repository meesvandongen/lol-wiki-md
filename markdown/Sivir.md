# Sivir

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
| **Champion** | Sivir |
| **Title** | the Battle Mistress |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2009-02-21 |
| **Release Patch** | Alpha Week 2 |
| **Latest Changes** | V25.11 |
| **Roles** | Marksman |
| **Riot Positions** | Bottom |
| **External Positions** | Bottom |
| **Blue Essence** | 225 |
| **Riot Points** | 260 |
| **Difficulty** | 2 |
| **Hero Type** | Marksman |
| **Adaptive Type** | Physical |
| **Damage** | 3 |
| **Toughness** | 1 |
| **Control** | 1 |
| **Mobility** | 1 |
| **Utility** | 2 |
| **Style** | 10 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $600.0$ | $+104.0$ |
| **Mana** | $340.0$ | $+45.0$ |
| **Health Regen** | $3.25$ | $+0.55$ |
| **Mana Regen** | $6.0$ | $+0.8$ |
| **Armor** | $30.0$ | $+4.45$ |
| **Magic Resist** | $30.0$ | $+1.3$ |
| **Attack Damage** | $60.0$ | $+2.5$ |
| **Attack Speed** | $0.625$ | |
| **Movement Speed** | $335.0$ | $+0.0$ |
| **Attack Range** | $500.0$ | $+0.0$ |
| **Base Attack Speed** | $0.625$ | |
| **Attack Speed Ratio** | $0.625$ | |
| **Bonus AS per Level** | $2.0\%$ | |
| **Missile Speed** | $1750$ units/second | |
| **Acquisition Radius** | $500$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $115$ units | |
| **Selection Height** | $160$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $93.0\%$ |
| **Damage Taken** | $105.0\%$ |

#### URF

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $85.0\%$ |
| **Damage Taken** | $105.0\%$ |

## Abilities

### Passive: Fleet of Foot

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Self |

**INNATE:** **Sivir**’s basic attacks and ability hits against enemy champions grant her (ms) 55@1; 60@6; 65@11; 70@16; 75@18 **bonus** movement speed decaying over $1.5$ seconds, refreshing on subsequent hits.

**Notes:**

- No additional details.

---

### Q: Boomerang Blade

| Attribute | Value |
|-----------|------:|
| **Cast Time** | 0.25–0.1@0–120 (@=**bonus** attack speed) seconds |
| **Target Range** | 1250 units |
| **Width** | 180 (Outgoing missile) / 200 (Returning missile) units |
| **Speed** | 1450 (Outgoing missile) / 1200 (Returning missile) units/second |
| **Cost** | 55 / 60 / 65 / 70 / 75 Mana |
| **Cooldown** | 10 / 9.5 / 9 / 8.5 / 8 seconds |
| **Queue Time** | $0.5$ seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | Special |
| **Spell Effects** | AoE |
| **Projectile** | True |

**ACTIVE:** **Sivir** hurls her crossblade in the target direction, dealing physical damage to enemies within its path, increased by 0%–50%@0–100 (@=critical strike chance).

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 60 / 85 / 110 / 135 / 160 (+ 100% **bonus** AD) (+ 60% AP) |

*Boomerang Blade*’s damage is reduced by 0%–60%@0–4 (@=non-champions hit). Upon reaching maximum range, the crossblade returns to her, resetting the damage modifier and dealing the same damage to enemies on its way back.

| Attribute | Value |
|-----------|------:|
| **Minimum Damage** | 24 / 34 / 44 / 54 / 64 (+ 40% **bonus** AD) (+ 24% AP) |

| Attribute | Value |
|-----------|------:|
| **Total Maximum Champion Damage** | 120 / 170 / 220 / 270 / 320 (+ 200% **bonus** AD) (+ 120% AP) |

*Enemies can be hit only once per pass.*

**Notes:**

- Due to the cast time effectively rounding up to full game ticks, at undesturbed game tick interval times the technical minimum cast time is reached at $99.8$% **bonus** attack speed. Effect at cast time end
- The damage reduction per unit hit resets back to full damage upon changing direction.
- *Boomerang Blade* will still return to **Sivir** even if she dies before it changes direction.
- Spell shield only blocks one instance of damage.

---

### W: Ricochet

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 500 (Bounce range) units |
| **Speed** | / 1000 (Bounced missile speed) units/second |
| **Cost** | 60 Mana |
| **Cooldown** | 12 seconds |
| **Cooldown Start** | on-cast |
| **Queue Time** | $0.5$ seconds |
| **Targeting** | Auto |
| **Affects** | Self |
| **Damage Type** | Physical |
| **Spell Shield** | False |
| **Spell Effects** | default |
| **Projectile** | True |
| **Parry** | unknown |

**ACTIVE:** **Sivir** empowers her crossblade for the next 4 seconds, gaining **bonus** attack speed and causing her basic attacks to bounce to additional surrounding enemies, dealing physical damage to them. If the triggering attack critically strikes, the bounces will do so as well for damage. *Ricochet* deals 65% damage against minions and executes them if they would be left below 15 health.

| Attribute | Value |
|-----------|------:|
| **Bonus Attack Speed** | 20 / 25 / 30 / 35 / 40% |

| Attribute | Value |
|-----------|------:|
| **Bounce Damage** | 40 / 42.5 / 45 / 47.5 / 50% AD |
| **Bounce Critical Damage** | 70 / 74.375 / 78.75 / 83.125 / 87.5% AD |

| Attribute | Value |
|-----------|------:|
| **Minion Bounce Damage** | 26 / 27.625 / 29.25 / 30.875 / 32.5% AD |
| **Minion Bounce Critical Damage** | 45.5 / 48.3438 / 51.1875 / 54.0312 / 56.875% AD |

Bounces occur only up to 8 times and can target each enemy up to one additional time per empowered attack. They prioritize the nearest new target, then the nearest target if no new targets are available.

*Gaining Ricochet's attack speed buff resets *’Sivir's** basic attack timer. The target does not have to be visible to be bounced to.*

**Notes:**

- *Ricochet*-enhanced basic attacks can bounce from structures onto secondary targets but not from nearby units onto structures.
- : *Ricochet's * interactions with dodging, blocking, and blinding effects.
- *Ricochet* do not apply to additional targets with Runaan's Hurricane.
- **Sivir**’s attack timer will only reset from casting *Ricochet* if the attack speed buff was not already active on her. Refreshing the buff will not grant an attack reset.

---

### E: Spell Shield

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Cooldown** | 24 / 22.5 / 21 / 19.5 / 18 seconds |
| **Targeting** | Auto |
| **Affects** | Self |

**ACTIVE:** **Sivir** gains a spell shield for $1.5$ seconds. Upon successfully blocking a hostile effect, she heals herself and activates *Fleet of Foot* after $0.25$ seconds.

| Attribute | Value |
|-----------|------:|
| **Heal** | 60 / 65 / 70 / 75 / 80% AD (+ 50% AP) |

**Notes:**

- *Spell Shield* will heal and activate Fleet of Foot if any effect is blocked. This includes effects such as champion abilities, item effects, Dragon basic attacks, as well as the abilities of Baron Nashor and Rift Herald.
- *Spell Shield* will still block abilities that have already been applied to **Sivir** if the delayed effects of that ability is area of effect (e.g. Blaze's explosion, Chum the Waters, and Time Bomb).

---

### R: On the Hunt

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 1000 units |
| **Cost** | 100 Mana |
| **Cooldown** | 120 / 110 / 100 / 90 / 80 seconds |
| **Cooldown Start** | on-cast |
| **Queue Time** | $0.5$ seconds |
| **Targeting** | Auto |
| **Affects** | Allies |

**ACTIVE:** **Sivir** gains *On the Hunt* for a duration and grants it to nearby allied champions for the remaining duration.

| Attribute | Value |
|-----------|------:|
| **Buff Duration** | 8 / 9 / 10 / 11 / 12 seconds |

**ON THE HUNT:** Gain **bonus** movement speed.

| Attribute | Value |
|-----------|------:|
| **Bonus Movement Speed** | 20 / 22.5 / 25 / 27.5 / 30% |

While active, **Sivir**’s basic attacks on-attack reduce her basic abilities' **current** cooldowns by $0.5$ seconds each.

**Sivir** can refresh the duration of her *On the Hunt* buff whenever she scores a takedown against an enemy champion within 3 seconds of damaging them.

**Notes:**

- No additional notes.

---

## Patch History

### V25.11
- Ricochet
  - AD ratio increased to 40 / 42.5 / 45 / 47.5 / 50% AD from 30 / 35 / 40 / 45 / 50% AD.

### V25.08
- Spell Shield
  - **Bug Fixes:** No longer fails to block and be consumed against Rengar’s Bola Strike.

### V14.24
- Spell Shield
  - **Bug Fixes:** Spell shield now properly blocks Vision of Empire’s reveal debuff when the damage has been successfully blocked.

### V14.21
- Stats
  - Base attack damage increased to 60 from 58.
- Boomerang Blade
  - Base damage increased to 60 / 85 / 110 / 135 / 160 from 15 / 30 / 45 / 60 / 75.
  - AD ratio changed to 100% **bonus** AD at all ranks from 80 / 85 / 90 / 95 / 100% **total** AD.

### V14.5
- Stats
  - Base armor increased to 30 from 26.

### V13.12
- Stats
  - Attack damage growth reduced to $2.5$ from $2.8$.
- Ricochet
  - AD ratio increased to 30 / 35 / 40 / 45 / 50% AD from 25 / 30 / 35 / 40 / 45% AD.

### V13.10
- Boomerang Blade
  - **Bug Fixes:** Return missile targeting back to **Sivir** can now properly be created when she is untargetable or has entered stasis while it was being sent out.

### V13.9
- General
  - New splash artwork for Sivir.

### V13.5
- General
  - **Bug Fixes:** Now properly plays her voiceover interaction with Azir.

### V13.1b
- Stats
  - Mana growth increased to 45 from 40.
- Ricochet
  - Mana cost reduced to 60 at all ranks from 60 / 65 / 70 / 75 / 80.

## Trivia

- Sivir is voiced.md) by Rebecca Schweitzer.
  - Morgana is also voiced by the same voice actress.
- Sivir was the first champion to have a 10-point-rating for Ability Power.
- In the 2009 Official Teaser Trailer for League of Legends as well as in her alpha model, Sivir had pink hair.
- In the Founding Interns of League of Legends YouTube video uploaded by the Riot Games Channel, concept art for Sivir was shown to depict her with blonde hair and an overall wildly different design.
  - The moments she appears in the video are here and here.
- Sivir is the second to feature two 'Champion Spotlights' due to significant gameplay changes (the others being Ezreal, Karma, Katarina, and Lee Sin).
- Sivir was one of the first champions designed, together with Annie, Lee Sin, Singed, Sion, and Twisted Fate.
- *Sivir* resembles Latin adjective *severus* "grim, stern, strict".
- Sivir's Chalicar’s name and design were inspired by the Indian chakram.
- Her dance references the Arm Wave dance.
  - A side-by-side comparison can be seen here.
- Sivir was going to have an e-sports skin to celebrate SK Telecom T1 winning the 2015 World Championship.
  - SKT T1 Bang (the one she represented) requested she be swapped for Kalista after the whole set was pulled from PBE back in V6.8 to be updated.
- Sivir is voiced by Alexa Kahn in Legends of Runeterra.

---
*This page was automatically generated from League of Legends Wiki data.*