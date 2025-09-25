# Caitlyn

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
| **Champion** | Caitlyn |
| **Title** | the Sheriff of Piltover |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2011-01-04 |
| **Release Patch** | V1.0.0.108 |
| **Latest Changes** | V25.06 |
| **Roles** | Marksman |
| **Riot Positions** | Bottom |
| **External Positions** | Bottom |
| **Blue Essence** | 225 |
| **Riot Points** | 260 |
| **Difficulty** | 1 |
| **Hero Type** | Marksman |
| **Adaptive Type** | Physical |
| **Damage** | 3 |
| **Toughness** | 1 |
| **Control** | 2 |
| **Mobility** | 2 |
| **Utility** | 1 |
| **Style** | 10 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $580.0$ | $+107.0$ |
| **Mana** | $315.0$ | $+40.0$ |
| **Health Regen** | $3.5$ | $+0.55$ |
| **Mana Regen** | $7.4$ | $+0.7$ |
| **Armor** | $27.0$ | $+4.7$ |
| **Magic Resist** | $30.0$ | $+1.3$ |
| **Attack Damage** | $60.0$ | $+3.8$ |
| **Attack Speed** | $0.681$ | |
| **Movement Speed** | $325.0$ | $+0.0$ |
| **Attack Range** | $650.0$ | $+0.0$ |
| **Base Attack Speed** | $0.681$ | |
| **Attack Speed Ratio** | $0.625$ | |
| **Bonus AS per Level** | $4.0\%$ | |
| **Missile Speed** | $2500$ units/second | |
| **Acquisition Radius** | $800$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $120$ units | |
| **Selection Height** | $155$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $100.0\%$ |
| **Damage Taken** | $100.0\%$ |

## Abilities

### Passive: Headshot

| Attribute | Value |
|-----------|------:|
| **Speed** | 3000 (Headshots missile speed) units/second |
| **Targeting** | Passive |
| **Affects** | Self |
| **Damage Type** | Physical |
| **Spell Shield** | False |
| **Spell Effects** | Basic |
| **Projectile** | True |
| **Parry** | True |

**INNATE:** **Caitlyn**’s basic attacks generate a stack of *Count* on-attack, doubled if she is within brush. At 5 stacks, or 4 while in *brush*, her next basic attack consumes all stacks on-attack to become a *Headshot*.

**HEADSHOT:** **Caitlyn**’s basic attack is empowered to have an uncancellable windup and deal 60%–120%@1–13 (+ critical strike chance) AD **bonus** physical damage, increased to 110%–120%@1–13 (+ critical strike chance) AD against non-champions.

Enemies that step over a *Yordle Snap Trap* or are hit by *90 Caliber Net* can grant an additional *Headshot* against them at (range) 1300 range within $1.8$ seconds, without consuming stacks. Each method grants only one *Headshot* at a time.

**Notes:**

- If **Caitlyn**’s current target becomes trapped by Yordle Snap Trap or 90 Caliber Net while her attack is on cooldown, this ongoing cooldown will be refunded partially so that her next attack on them can begin earlier.
  - Swapping the target to an enemy trapped by Yordle Snap Trap or 90 Caliber Net also reduces her ongoing attack cooldown.
    - : The exact attack cooldown refund appears to be 50% of the attack cooldown at current attack speed, but at a minimum amount it lets **Caitlyn** start the attack against the trapped target within $0.5$ seconds of her previous attack windup completing.
- The bonus range from Rapid Firecannon will not have any effect during trap and net extended *Headshot* attacks.
- Only the ordinary 100% attack damage portion of the attack can critically strike. The bonus from *Headshot* is added after the critical strike is rolled.
  - The full damage of *Headshot* applies life steal.
  - *Headshot* is a single instance of damage, thus does not trigger Tantrum’s or Warden's Mail passives twice.
- The basic attack triggering *Headshot* can critically strike, but *Headshot*’s damage remains unchanged if it does.
- Since **Caitlyn**’s Art and Sustainability Update, Ability-granted *Headshots* are the same as the original *Headshot* override attack, though what lets them cast at high range is still relatively special. They trigger on-attack effects (including stacking *Headshot*).
  - Runaan's Hurricane Wind's Fury, is special cased to search targets with the increased range as well.
- Because *Headshot* stacks are generated on-attack, Runaan's Hurricane bolts will not generate any.
- The empowered attack will not trigger against wards.
  - Stacks will still be generated towards *Headshot* when attacking them.

---

### Q: Piltover Peacemaker

| Attribute | Value |
|-----------|------:|
| **Range** | 1300 / er 1240 units |
| **Cast Time** | $0.625$ seconds |
| **Width** | 120 (Initial missile) / 180 (Expanded missile) units |
| **Speed** | 2200 units/second |
| **Cost** | 55 / 60 / 65 / 70 / 75 mana |
| **Cooldown** | 10 / 9 / 8 / 7 / 6 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |
| **Projectile** | True |

**ACTIVE:** **Caitlyn** fires a piercing shot in the target direction that deals physical damage to the first enemy it passes through, after which it expands in width but deals only 60% damage to enemies it hits thereafter.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 50 / 90 / 130 / 170 / 210 (+ 125 / 145 / 165 / 185 / 205% AD) |
| **Reduced Damage** | 30 / 54 / 78 / 102 / 126 (+ 75 / 87 / 99 / 111 / 123% AD) |

Enemies revealed by *Yordle Snap Trap* always take full damage from *Piltover Peacemaker*.

**Notes:**

- Upon *Piltover Peacemaker* dealing damage to an enemy, **Caitlyn**’s attack timer is forcibly reset.
- Enemies hit while protected by a spell shield will still be considered a target for calculating damage for subsequent hits. - This ability will cast from wherever the caster is at the start of the cast time.

---

### W: Yordle Snap Trap

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 800 units |
| **Collision Radius** | 15 units |
| **Cost** | 20 Mana + 1 Charge |
| **Cooldown** | $0.5$ seconds |
| **Recharge** | 26 / 22 / 18 / 14 / 10 seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Spell Shield** | True |
| **Out of Range Behavior** | walk to location |

**ACTIVE:** **Caitlyn** sets a visible trap at the target location that is untargetable and arms after 1 second, lasting for a duration. The trap grants sight of the area for 1 second after being placed.

| Attribute | Value |
|-----------|------:|
| **Trap Duration** | 30 / 35 / 40 / 45 / 50 seconds |

**Caitlyn** periodically stocks a *Yordle Snap Trap* charge, up to a maximum amount. Deploying traps beyond the maximum destroys the oldest one.

| Attribute | Value |
|-----------|------:|
| **Maximum Number of Traps** | 3 / 3 / 4 / 4 / 5 |

The next enemy champion that springs the trap is rooted for $1.5$ seconds and revealed for 3 seconds, as well as takes additional damage from the *Headshot* granted by this ability. Trapped targets become immune to further *Yordle Snap Traps* for 3 seconds, preventing them from springing subsequent traps for the duration.

| Attribute | Value |
|-----------|------:|
| **Headshot Damage Increase** | 35 / 80 / 125 / 170 / 215 (+ 30% **bonus** AD) |

**Notes:**

- Multiple armed traps on the same location will spring all at once if an enemy steps on one of them.

---

### E: 90 Caliber Net

| Attribute | Value |
|-----------|------:|
| **Range** | 800 / er 740 units |
| **Cast Time** | $0.15$ seconds |
| **Effect Radius** | 140 units |
| **Speed** | 1600 (Missile speed) units/second |
| **Cost** | 75 mana |
| **Cooldown** | 16 / 14 / 12 / 10 / 8 seconds |
| **Targeting** | Direction |
| **Affects** | Self, Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Single target |
| **Projectile** | True |
| **Grounded** | True |
| **Knockdown** | True |

**ACTIVE:** **Caitlyn** fires a net in the target direction and recoils 390 units in the opposite direction. The net deals magic damage to the first enemy hit and slows them by 50% for 1 second.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 80 / 130 / 180 / 230 / 280 (+ 80% AP) |

***Caitlyn** will not dash backwards if she is immobilized or grounded during the cast time. She can cast any of her abilities during the dash.*

**Notes:**

- This ability will cast from wherever the caster is at the end of the cast time.
- The recoil can be used to pass through terrain.
- If **Caitlyn** buffers Piltover Peacemaker during *90 Caliber Net's* cast time, Piltover Peacemaker will start after the cast time.
  - If **Caitlyn** buffers an ability or basic attack during this ability's cast time, the buffered ability or basic attack will instead start after the dash ends.
  - Other abilities can be cast while dashing if they are input after the cast time.
- *90 Caliber Net's* missile will fail to fire if **Caitlyn** is suppressed during the cast time.

---

### R: Ace in the Hole

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.375$ seconds |
| **Target Range** | 3500 units |
| **Width** | 80 (Collision width) units |
| **Speed** | 3200 units/second |
| **Cost** | 100 mana |
| **Cooldown** | 90 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | spell |
| **Projectile** | True |
| **Call For Help** | True |
| **Silence** | True |

**ACTIVE:** **Caitlyn** locks onto the target enemy champion and channels for 1 second, revealing them as well as revealing herself.

*Ace in the Hole* is placed on a 5-second cooldown if canceled.

Once **Caitlyn** completes the channel, she fires a homing bullet toward the target that deals physical damage to the first enemy champion it hits, increased by 0%–50%@0–100 (@=critical strike chance).

| Attribute | Value |
|-----------|------:|
| **Physical damage** | 300 / 387.5 / 475 / 562.5 / 650 (+ 100% **bonus** AD) |

**Notes:**

- The following will cancel *Ace in the Hole*’s channel (mana expenditure is not compensated):
  - **Caitlyn** is interrupted.
  - The target becomes untargetable.
  - **Caitlyn** or her target dies.
- *Ace in the Hole* reveals the target and **Caitlyn** through a buff that lasts for up to 4 seconds.
  - The buff is applied to the target from the start of the cast time.
  - The buff ends prematurely when the channel is canceled or the bullet hits an enemy.
    - The target is revealed for 1 second if the buff is ended prematurely.
    - It is not removed if the cast time is cancelled or the bullet missile is destroyed.
- The bullet missile will fail to hit the primary target if they are untargetable upon arrival, dealing no damage.
- The ability also goes on a 5-second cooldown if **Caitlyn** enters resurrection during the cast time.
  - The cast time does not end prematurely but the channel will be immediately canceled the moment the cast time completes.
- If the target dies after the bullet has been fired, the shot will continue towards their corpse and may still hit other enemy champions.
- The bullet has a sight radius of 1500 attached to it. ** This excludes losing sight of the target.
- The following table refers for interactions while **Caitlyn** is channeling:

#### Channel Behavior (channel)

| Aspect | State / Notes |
|--------|---------------|
| **Attacking** | Disabled |
| **Movement** | Disabled |
| **Abilities** | Disabled |
| **Items** | Disabled / Allowed |
| **Summoner Spells** | Allowed / Disabled |

---

## Patch History

### V25.06
- Yordle Snap Trap
  - Headshot base bonus damage reduced to 35 / 80 / 125 / 170 / 215 from 40 / 85 / 130 / 175 / 220.
  - Bonus Headshot damage bonus AD ratio reduced to 30% **bonus** AD from 40%.
- Ace in the Hole
  - Base damage reduced to 300 / 475 / 650 from 300 / 500 / 700.

### V14.24
- Caitlyn
  - Renamed to *Arcane Enforcer* from *Arcane*.

### V14.23
- Ace in the Hole
  - Bonus AD ratio reduced to 100% **bonus** AD from 150%.

### V14.20
- Stats
  - Attack speed ratio increased to $0.625$ from $0.61$.
- Piltover Peacemaker
  - Damage against secondary targets increased to 60% from 50%.

### V14.17
- Stats
  - Attack speed ratio increased to $0.61$ from $0.594$.
- Yordle Snap Trap
  - Recharge reduced to 26 / 22 / 18 / 14 / 10 seconds from 30 / 24 / 19 / 15 / 12. *Now scales linearly.*

### V14.11
- Stats
  - Attack speed ratio increased to $0.594$ from $0.568$.
- Yordle Snap Trap
  - Headshot bonus AD ratio reduced to 40% **bonus** AD at all ranks from 40 / 50 / 60 / 70 / 80%.
- Ace in the Hole
  - Bonus AD ratio reduced to 150% **bonus** AD from 170%.

### V14.7
- Headshot
  - **Bug Fixes:** A headshot gained from this ability is no longer incorrectly consumed alongside any headshots primed by Yordle Snap Trap or 90 Caliber Net.

### V14.3
- Headshot
  - **Bug Fixes:** Buff is now properly consumed on-attack. She can no longer fire 2 empowered attacks by resetting her attack timer with Titanic Hydra *Titanic Crescent* while an empowered missile was in-flight.

### V13.21
- Headshot
  - Critical strike ratio formula changed to (100% **total** critical damage×85% critical strike chance) from (100% **total** critical damage×$81.25$% critical strike chance).
    - Critical strike ratio increased to from .
- Ace in the Hole
  - Cooldown increased to 90 seconds at all ranks from 90 / 75 / 60.
  - Base damage reduced to 300 / 500 / 700 from 300 / 525 / 750.
  - AD ratio reduced to 170% **bonus** AD from 200%.
  - Critical strike chance ratio increased to 0%–50%@0–100 (@=critical strike chance) from 0%–35%@0–100 (@=critical strike chance).

### V13.18
- Headshot
  - Critical strike ratio formula changed to (100% total crit damage x $81.25$% total crit chance) from ($62.5$% total crit damage x 130% total crit chance).
    - Damage unchanged.

## Trivia

- In Western culture, the name Caitlyn is believed to be derived from the Greek Αἰκατερίνα (Aikaterine).
  - Katarina’s name has the same roots.
- Caitlyn's accent is a Received Pronunciation spoken in south of England.
- Caitlyn's rifle can be seen in the trailer for the game's Mac version.
  - A Yordle Snap Trap can be seen shortly after.
- Caitlyn has the longest base basic attack range in the game at (range) 650 units.
  - Her dance and taunts resemble several rifle moves in modern Color guard corps.
- Headshots actually fires at the target's head; this is most noticeable with tall targets like Baron Nashor or Enemy Turrets.
- When Caitlyn and Vi are on the same team, they are granted the cosmetic buff On the Case. This buff gives them extra gold gold when they score a takedown together.
  - If there is a Jinx on the enemy team, Caitlyn gets the cosmetic debuff Catch me if you can! This debuff displays a counter of 'criminals apprehended' (kills/assists scored against Jinx).
- Caitlyn, Blitzcrank, Lissandra, Rumble, Sion, Varus, Vi, Viego, Xerath, and Ziggs are the only champions who can apply crowd control on themselves.

---
*This page was automatically generated from League of Legends Wiki data.*