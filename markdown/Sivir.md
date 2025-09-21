# Sivir

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
| **Champion** | Sivir |
| **Title** | the Battle Mistress |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2009-02-21 |
| **Release Patch** | Alpha Week 2 |
| **Roles** | Marksman |
| **Riot Positions** | Bottom |
| **External Positions** | Bottom |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $600.0$ | $+104.0$ | $2368.0$ |
| **Mana** | $340.0$ | $+45.0$ | $1105.0$ |
| **Health Regen** | $3.25$ | $+0.55$ | $12.6$ |
| **Mana Regen** | $6.0$ | $+0.8$ | $19.6$ |
| **Armor** | $30.0$ | $+4.45$ | $105.7$ |
| **Magic Resist** | $30.0$ | $+1.3$ | $52.1$ |
| **Attack Damage** | $60.0$ | $+2.5$ | $102.5$ |
| **Attack Speed** | $0.625$ | $+2.0\%$ | $0.838$ |
| **Movement Speed** | $335.0$ | $+0.0$ | $335.0$ |
| **Attack Range** | $500.0$ | $+0.0$ | $500.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.625$ |
| **Attack Speed Ratio** | $0.625$ |
| **Bonus AS per Level** | $2.0\%$ |
| **Missile Speed** | $1750 units/second$ |
| **Acquisition Radius** | $500 units$ |
| **Pathing Radius** | $35 units$ |
| **Selection Radius** | $115 units$ |
| **Selection Height** | $160 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Fleet of Foot

**Innate:** **Sivir**’s basic attacks and damaging abilities against enemy champions briefly grant her a burst of ms.

**Innate:** ''Sivir's** basic attacks and ability hits against enemy champions grant her ms*bonus'' movement speed* decaying over $1.5$ seconds, refreshing on subsequent hits.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Self |

**Notes:**

- No additional details.

---

### Q: Boomerang Blade

**Active:** **Sivir** hurls her crossblade in the target direction that deals physical damage to enemies hit.

*At max range, it homes back to **Sivir** to deal physical damage to enemies hit.*

**Active:** **Sivir** hurls her crossblade in the target direction, dealing physical damage to enemies within its path, increased by key=%. 'Boomerang Blade's damage is reduced by key=%. Upon reaching maximum range, the crossblade returns to her, resetting the damage modifier and dealing the same damage to enemies on its way back. *Enemies can be hit only once per pass.*

| Attribute | Value |
|-----------|-------|
| **Range** | 1250 units |
| **Cooldown** | $10-8$ seconds |
| **Cast Time** | type=*bonus attack speed |
| **Cost** | $55-75$ Mana |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Speed** | 1450 / 1200 units/second |
| **Spell Shield** | Special |
| **Spell Effects** | AoE |
| **Projectile** | True |

**Scaling:**
- **Physical Damage:** $60-160$
- *bonus AD) (+ 60% AP)
- **Minimum Damage:** $60×0.4-160×0.4$ (+ $100×0.4$%
- *bonus AD) (+ 24% AP)
- **Total Maximum Champion Damage:** $60×2-160×2$ (+ $100×2$%
- *bonus AD) (+ 120% AP)

**Notes:**

- Due to the cast time effectively rounding up to full game ticks, at undesturbed game tick interval times the technical minimum cast time is reached at $99.8$% *bonus attack speed. Effect at cast time end
- The damage reduction per unit hit resets back to full damage upon changing direction.
- *Boomerang Blade* will still return to **Sivir** even if she dies before it changes direction.
- Spell shield only blocks one instance of damage.

---

### W: Ricochet

**Active:** **Sivir** gains **bonus attack speed** for a few seconds. During this time, her basic attacks create projectiles that repeatedly bounce to nearby surrounding enemies, dealing physical damage.

**Active:** **Sivir** empowers her crossblade for the next 4 seconds, gaining **bonus attack speed* and causing her basic attacks to bounce to additional surrounding enemies, dealing physical damage to them. If the triggering attack critical strike, the bounces will do so as well for critical damageRicochet* deals 65% damage against minions and executes them if they would be left below *15 health*. Bounces occur only up to 8 times and can target each enemy up to one additional time per empowered attack. They prioritize the nearest new target, then the nearest target if no new targets are available. 'Gaining Ricochet's attack speed buff basic attack reset ''Sivir's* basic attack timer. The target does not have to be sight to be bounced to.*

| Attribute | Value |
|-----------|-------|
| **Cooldown** | 12 seconds |
| **Cast Time** | none |
| **Cost** | 60 Mana |
| **Targeting** | Auto |
| **Affects** | Self |
| **Damage Type** | Physical |
| **Speed** | missile speed, same as basic attack / 1000 units/second |
| **Effect Radius** | 500 units |
| **Spell Shield** | False |
| **Spell Effects** | default |
| **Projectile** | True |

**Scaling:**
- **Bonus Attack Speed:** $20-40$% **Bounce Damage:* $40-50$% AD
- **Minion Bounce Damage:** $40×0.65-500.65$% AD0.65)*

**Notes:**

- *Ricochet*-enhanced basic attacks can bounce from structures onto secondary targets but not from nearby units onto structures.
- : 'Ricochet's ' interactions with dodge, block, and blind effects.
- *Ricochet* do not apply to additional targets with *Runaan's Hurricane*.
- ''Sivir's* attack timer will only reset from casting *Ricochet' if the attack speed buff was not already active on her. Refreshing the buff will not grant an attack reset.

---

### E: Spell Shield

**Active:** **Sivir** briefly gains a spell shield.

**Sivir** heals herself and activates *Fleet of Foot* if it blocks an ability.

**Active:** **Sivir** gains a spell shield for $1.5$ seconds. Upon successfully blocking a hostile effect, she heals herself and activates **Fleet of Foot** after $0.25$ seconds.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $24-18$ seconds |
| **Cast Time** | none |
| **Targeting** | Auto |
| **Affects** | Self |

**Scaling:**
- **Heal:** $60-80$% AD (+ 50% AP)

**Notes:**

- *Spell Shield* will heal and activate *Fleet of Foot* if any effect is blocked. This includes effects such as champion abilities, item effects, *Dragon* basic attacks, as well as the abilities of Baron Nashor and Rift Herald.
- *Spell Shield* will still block abilities that have already been applied to **Sivir** if the delayed effects of that ability is area of effect (e.g. *Blaze's* explosion, *Chum the Waters*, and *Time Bomb*).

---

### R: On the Hunt

**Active:** **Sivir** gains *On the Hunt* for a period, and creates an aura that grants allied champions *On the Hunt* for the remaining duration. **Sivir** can refresh her duration whenever she scores an enemy takedown within a short time of damaging them.

**Active:** **Sivir** gains *On the Hunt* for a duration and grants it to nearby allied champions for the remaining duration. **On the Hunt:** Gain **bonus movement speed**. While active, ''Sivir's' basic attacks on-attack reduce her basic abilities' **current cooldowns** by $0.5$ seconds each. **Sivir** can refresh the duration of her *On the Hunt* buff whenever she scores a takedown against an enemy champion within 3 seconds of damaging them.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $120-80$ seconds |
| **Cast Time** | none |
| **Cost** | 100 Mana |
| **Targeting** | Auto |
| **Affects** | Allies |
| **Effect Radius** | 1000 units |

**Scaling:**
- **Buff Duration:** $8-12$ seconds
- **Bonus Movement Speed:** $20-30$%

**Notes:**

- No additional notes.

---

## Patch History

### V25.11
- *Ricochet*
  - AD ratio increased to $40-50$% AD from $30-50$% AD.

### V25.08
- *Spell Shield*
  - **Bug Fixes:** No longer fails to block and be consumed against Bola Strike.

### V14.24
- *Spell Shield*
  - **Bug Fixes:** Spell shield now properly blocks *Vision of Empire*’s reveal debuff when the damage has been successfully blocked.

### V14.21
- Stats
  - Base attack damage increased to 60 from 58.
- *Boomerang Blade*
  - Base damage increased to $60-160$ from $15-75$.
  - AD ratio changed to 100% *bonus AD at all ranks from $80-100$% **total** AD.

### V14.5
- Stats
  - Base armor increased to 30 from 26.

### V13.12
- Stats
  - Attack damage growth reduced to $2.5$ from $2.8$.
- *Ricochet*
  - AD ratio increased to $30-50$% AD from $25-45$% AD.

### V13.10
- *Boomerang Blade*
  - **Bug Fixes:** Return missile targeting back to **Sivir** can now properly be created when she is untargetable or has entered stasis while it was being sent out.

### V13.9
- General
  - New splash artwork for Sivir.

### V13.5
- General
  - **Bug Fixes:** Now properly plays her voiceover interaction with **Azir**.
- Stats
  - Mana growth increased to 45 from 40.
- *Ricochet*
  - Mana cost reduced to 60 at all ranks from $60-80$.

### V12.17
- Stats
  - Attack damage growth reduced to $2.8$ from 3.
- *Ricochet*
  - AD ratio reduced to $25-45$% AD from $30-50$% AD.

## Trivia

- Sivir is voiced.md) by Rebecca Schweitzer.
  - **Morgana** is also voiced by the same voice actress.
- Sivir was the first champion to have a 10-point-rating for Ability Power.
- In the 2009 Official Teaser Trailer for League of Legends as well as in her alpha model, Sivir had pink hair.
- In the Founding Interns of League of Legends YouTube video uploaded by the Riot Games Channel, concept art for Sivir was shown to depict her with blonde hair and an overall wildly different design.
  - The moments she appears in the video are here and here.
- Sivir is the second to feature two 'Champion Spotlights' due to significant gameplay changes (the others being **Ezreal**, **Karma**, **Katarina**, and **Lee Sin**).
- Sivir was one of the first champions designed, together with **Annie**, **Lee Sin**, **Singed**, **Sion**, and **Twisted Fate**.
- *Sivir* resembles Latin adjective *severus* "grim, stern, strict".
- Sivir's *Chalicar*’s name and design were inspired by the Indian chakram.
- Her dance references the Arm Wave dance.
  - A side-by-side comparison can be seen here.
- Sivir was going to have an e-sports skin to celebrate SK Telecom T1 winning the 2015 World Championship.
  - SKT T1 Bang (the one she represented) requested she be swapped for Kalista after the whole set was pulled from PBE back in V6.8 to be updated.
- Sivir is voiced by Alexa Kahn in Legends of Runeterra.

---
*This page was automatically generated from League of Legends Wiki data.*