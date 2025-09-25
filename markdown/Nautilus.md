# Nautilus

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
| **Champion** | Nautilus |
| **Title** | the Titan of the Depths |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2012-02-14 |
| **Release Patch** | V1.0.0.134 |
| **Latest Changes** | V25.16 |
| **Roles** | Vanguard |
| **Riot Positions** | Support |
| **External Positions** | Support |
| **Blue Essence** | 2400 |
| **Riot Points** | 880 |
| **Difficulty** | 2 |
| **Hero Type** | Tank |
| **Alt Type** | Support |
| **Adaptive Type** | Magic |
| **Damage** | 1 |
| **Toughness** | 3 |
| **Control** | 3 |
| **Mobility** | 1 |
| **Utility** | 1 |
| **Style** | 80 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $646.0$ | $+100.0$ |
| **Mana** | $400.0$ | $+47.0$ |
| **Health Regen** | $8.5$ | $+0.55$ |
| **Mana Regen** | $8.65$ | $+0.5$ |
| **Armor** | $39.0$ | $+4.95$ |
| **Magic Resist** | $32.0$ | $+2.05$ |
| **Attack Damage** | $61.0$ | $+3.3$ |
| **Attack Speed** | $0.706$ | |
| **Movement Speed** | $325.0$ | $+0.0$ |
| **Attack Range** | $175.0$ | $+0.0$ |
| **Base Attack Speed** | $0.706$ | |
| **Attack Speed Ratio** | $0.612$ | |
| **Bonus AS per Level** | $1.0\%$ | |
| **Acquisition Radius** | $400$ units | |
| **Gameplay Radius** | $80$ units | |
| **Pathing Radius** | $50$ units | |
| **Selection Radius** | $125$ units | |
| **Selection Height** | $180$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $95.0\%$ |
| **Damage Taken** | $110.0\%$ |

## Abilities

### Passive: Staggering Blow

| Attribute | Value |
|-----------|------:|
| **On-target CD Static** | 6 |
| **Targeting** | Passive |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | Special |
| **Spell Effects** | Proc |
| **Parry** | True |

**INNATE:** **Nautilus' **basic attacks are empowered to deal 14 to 116 **bonus** physical damage and root the target for 0.75–1.5@1–16 seconds.

This effect cannot occur on the same target more than once every few seconds.

**Notes:**

- Spell shield will block the root but not the bonus damage.
- *Staggering Blow*’s enhanced attack can be blocked and dodged (the on-target cooldown will still be applied).
  - It will fail to trigger while he is blinded.
- The empowered attack will not trigger against structures. *

---

### Q: Dredge Line

| Attribute | Value |
|-----------|------:|
| **Range** | 1122 (The missile is destroyed on the tick it passes 1100 distance, resulting in this unorthodox max-range collision behaviour compared to other missiles) units |
| **Cast Time** | $0.25$ seconds |
| **Width** | 180 (Unit collision) / cr 0 (Terrain collision) units |
| **Speed** | 2000 units/second |
| **Cost** | 60 Mana |
| **Cooldown** | 14 / 13 / 12 / 11 / 10 seconds |
| **Targeting** | Direction |
| **Affects** | Self, Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | Special |
| **Spell Effects** | spell |
| **Projectile** | True |
| **Grounded** | True |
| **Knockdown** | True |

**ACTIVE:** **Nautilus** hurls his anchor in the target direction that stops on the first enemy or terrain hit.

If the anchor hits an enemy, it deals magic damage, reveals them for $1.15$ seconds, stuns them for 1 second, and drags them toward **Nautilus** while he also dashes toward them, both over $0.9$ seconds.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 85 / 130 / 175 / 220 / 265 (+ 90% AP) |

If the anchor hits terrain, **Nautilus** dashes to that location and 50% of *Dredge Line*’s cooldown and (mana) mana cost are refunded.

***Nautilus** is unable to move or attack while Dredge Line is in flight. He can cast any of his abilities during the dash.*

**Notes:**

- *Dredge Line* has two hitboxes. The outer hitbox, shown by a visual effect on the ground, will hit champions edge-to-edge. The inner hitbox, shown by the anchor, can hit terrain. Near thinner walls, this can cause the ability to hit a champion on the other side.
- *Dredge Line* turns **Nautilus**' facing direction accordingly upon hitting a valid target.
- *Dredge Line* can interact with player-generated terrain.
- **Nautilus** will dash all the way to the target's location if *Dredge Line* executed them or they negated the displacement.
- *Dredge Line* will not interrupt any movement commands **Nautilus** was issued pre-cast so long as their directions somewhat correlate (this is intentional to smooth out traveling/escaping).
- Spell shield will block the ability but **Nautilus** will still dash partway to the target.
- The anchor projectile will stop if **Nautilus** dies while the projectile is traveling. Enemies hit will still be dealt damage. The drag and root are not applied if **Nautilus** did not initiate the dash.
- Displacement immunity will not resist the application of the stun. Effect at cast time start
- The following table refers for interactions during *Dredge Line*’s cast time.

#### Channel Behavior (cast)

| Aspect | State / Notes |
|--------|---------------|
| **Summoner Spells** | Allowed / Disabled |

---

### W: Titan's Wrath

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 250 (Centered on target) units |
| **Cost** | 60 Mana |
| **Cooldown** | 12 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | Special |
| **Spell Effects** | Special |
| **Parry** | Special |

**ACTIVE:** **Nautilus** grants himself a shield for 6 seconds, and while it holds, his basic attacks are empowered to apply *Pain of Wrath* to the target and enemies near them.

| Attribute | Value |
|-----------|------:|
| **Shield Strength** | 50 / 60 / 70 / 80 / 90 (+ 8 / 9 / 10 / 11 / 12% **maximum** health) |

**PAIN OF WRATH:** The target takes magic damage over time, half dealt immediately and the other half dealt after $1.25$ seconds.

| Attribute | Value |
|-----------|------:|
| **Total Magic Damage** | 30 / 40 / 50 / 60 / 70 (+ 40% AP) |
| **Magic Damage per Instance** | 15 / 20 / 25 / 30 / 35 (+ 20% AP) |

*Titan's Wrath resets **Nautilus' ** basic attack timer.*

**Notes:**

- *Pain of Wrath*’s first damage instance is tagged as spell damage, while the second damage instance is tagged as persistent damage.
- *Pain of Wrath* damage over time debuff replaces itself when reapplied to the target, restarting its tick timer. This will prevent the second instance of damage from the previous debuff.
- *Pain of Wrath* will be applied by **Nautilus**’s basic attack if he is affected by *Titan's Wrath* at the start of the attack windup, even if *Titan's Wrath* expires during the windup.
- *Pain of Wrath* will not be applied if the basic attack is dodged or blocked, but will be applied if it is missed.
- Spell shield will block the ability (but only a single application).
- The empowered attack does not apply *Pain of Wrath* to structures.

---

### E: Riptide

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Effect Radius** | cr 0 - 350 (Wave 1) / 215 - 465 (Wave 2, offset by 25 units) / 350 - 590 (Wave 3, offset by 25 units) |
| **Cost** | 50 / 60 / 70 / 80 / 90 Mana |
| **Cooldown** | 7 / 6.5 / 6 / 5.5 / 5 seconds |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |

**ACTIVE:** **Nautilus** sends three waves of explosions that radiate from him over $0.561$ seconds (See notes). Each wave deals magic damage to enemies hit, reduced to 50% against those hit by subsequent waves beyond the first, and slows them by an amount that decays over $1.5$ seconds.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 55 / 90 / 125 / 160 / 195 (+ 50% AP) |
| **Reduced Damage** | 27.5 / 45 / 62.5 / 80 / 97.5 (+ 25% AP) |
| **Maximum Total Damage** | 110 / 180 / 250 / 320 / 390 (+ 100% AP) |

| Attribute | Value |
|-----------|------:|
| **Slow** | 30 / 35 / 40 / 45 / 50% |

The first hit against monsters deals **bonus** magic damage.

| Attribute | Value |
|-----------|------:|
| **Monster Bonus Damage** | 125 / 165 / 205 / 245 / 285 (+ 50% AP) |

**Notes:**

Effect at cast time end
- The first wave is centered around **Nautilus**, while the second and third wave are centered at an offset of 25 units in front of **Nautilus** at the original cast location.
  - Because of this, there are a zone each in which you can be hit by only 1 wave, or all 3 waves, respectively, without moving.
- The first wave happens at 0 seconds after the cast time ends, the second wave at $0.297$ seconds and the third wave at $0.561$ seconds.
  - The intended timing may be $0.25$ seconds for each delay, however these are the measured, effective times.
- The *explosions* are only a visual effect. The first wave has 8, the seconds has 9, and the third wave has 10 explosions, equally distributed around the rings, starting with one explosion directly in **Nautilus**' facing direction on each.

---

### R: Depth Charge

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.46$ seconds |
| **Target Range** | 825 units |
| **Effect Radius** | cr 225 (Underway eruptions, estimated) / 300 (Final eruption) /sight 750 (Seeking charge sight radius) |
| **Speed** | 275 + $466.67$ per second units/second |
| **Cost** | 100 Mana |
| **Cooldown** | 120 / 110 / 100 / 90 / 80 seconds |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | magic |
| **Spell Shield** | True |
| **Spell Effects** | aoe |
| **Projectile** | False |

**ACTIVE:** **Nautilus** sends out a *Depth Charge* that chases the target enemy champion, accelerating over time and creating eruptions every in its wake that also briefly grant sight of the area. Enemies hit by the eruptions are dealt magic damage, knocked up for 1 second, and stunned for a duration.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 125 / 150 / 175 / 200 / 225 (+ 40% AP) |

| Attribute | Value |
|-----------|------:|
| **Stun Duration** | 1 / 1.25 / 1.5 / 1.75 / 2 seconds |

Upon reaching the primary target, the *Depth Charge* erupts a final time at their location. The primary target takes increased damage, is stunned for the same duration, and knocked up for a modified duration.

| Attribute | Value |
|-----------|------:|
| **Increased Damage** | 150 / 212.5 / 275 / 337.5 / 400 (+ 80% AP) |

| Attribute | Value |
|-----------|------:|
| **Knock Up Duration** | 1 / 1.25 / 1.5 / 1.75 / 2 seconds |

**Notes:**

- *Depth Charge* starts in an offset from whichever direction **Nautilus** is facing at the end of the cast time.
  - Since Dredge Line’s causes **Nautilus** to change his facing direction upon hitting a valid target, *Depth Charge*’s starting point can be changed with a quick Dredge Line + *Depth Charge* combo.
- Enemies a certain distance away from the charge / eruption are instead knocked back.
- *Depth Charge*’s final eruption will occur prematurely if the primary target moves more than 3000 units away from it.
- *Depth Charge* will not stop chasing if the target dies or becomes untargetable.
- Dredge Line will not override *Depth Charge*’s knock up.
- Displacement immunity will not resist the application of the stun.

---

## Patch History

### V25.16
- Riptide
  - Monster damage reduced to 100% from 150%.
    - *[Note: This affected **all** three hits.]*
  - **New Effect:** The first hit against monsters now deals 125 / 165 / 205 / 245 / 285 (+ 50% AP) **bonus** magic damage.
    - *[Note: This affects the first **hit**, and not specifically the first **wave** only.]*

### V25.04
- Dredge Line
  - Base damage increased to 85 / 130 / 175 / 220 / 265 from 70 / 115 / 160 / 205 / 250.

### V14.19
- Nautilus
  - **Bug Fixes:** Resolved VO audio being too low in certain languages due to the audio filter.

### V14.12
- Titan's Wrath
  - **New Effect:** Now applies spell effects.

### V13.17
- Nautilus
  - Dredge Line
    - **Bug Fixes:** No longer plays incorrect model animations when it was cast.

### V13.3
- Staggering Blow
  - **UNDOCUMENTED/BUG FIX:** No longer locks his facing direction after the attack has landed, which would previously cause Dredge Line’s cast direction to be obscured if cast towards another target right after.

### V13.3
- Staggering Blow
  - Base damage increased to 14 to 116 from 8 to 110.
- Titan's Wrath
  - Mana cost reduced to 60 from 80.
  - Base shield increased to 50 / 60 / 70 / 80 / 90 from 40 / 50 / 60 / 70 / 80.
- Riptide
  - Base damage increased to 55 / 90 / 125 / 160 / 195 from 55 / 85 / 115 / 145 / 175.
  - AP ratio increased to 50% AP from 30% AP.

### V12.22
- Stats
  - Base mana regeneration increased to $8.65$ from $8.63$.

### V12.10
- Stats
  - Base health increased to 646 from 576.
  - Health growth increased to 100 from 86.
  - Armor growth increased to $4.95$ from $3.75$.
  - Magic resistance growth increased to $2.05$ from $1.25$.
- Titan's Wrath
  - Base shield reduced to 40 / 50 / 60 / 70 / 80 from 45 / 55 / 65 / 75 / 85.
  - Shield health ratio reduced to 8 / 9 / 10 / 11 / 12% **maximum** health from 9 / 10 / 11 / 12 / 13%.

### V11.15
- Nautilus, Nautilus, Nautilus, Nautilus, and Nautilus
  - **Bug Fixes:** Recall sound effects now play correctly.

## Trivia

- Nautilus' dance references the 'Peeparoonie'.
  - A side-by-side comparison can be seen here.
- A tab for Urgot can be seen in Nautilus'Art Spotlight'.
- Nautilus has the longest basic attack animation in-game (which results in enemies being rooted by Staggering Blow when they visibly got out of range).
- Nautilus was released without the 'Tank' tag, though he actually is one. It was added shortly after.
- Nautilus does the Captain Morgan pose when standing idle for a long while.
- *Nautilus* comes from Greek word for sailor, ναυτίλος.
  - Out of universe, Nautilus is also the name of Nautilus, the submarine from Twenty Thousand Leagues Under the Sea by Jules Verne, and subsequently the first USS Nautilus (SSN-571)
- Nautilus and Ziggs were first conceived as 'Ivan the Mad Bomber'.
- During Nautilus' joke, when he is swimming in the air, he is humming a tune. This is the same tune that Squidward hums to himself in the shower during the Spongebob Movie.
- Nautilus is voiced by Richard Newman (actor) in Legends of Runeterra.

---
*This page was automatically generated from League of Legends Wiki data.*