# Nautilus

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
| **Champion** | Nautilus |
| **Title** | the Titan of the Depths |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2012-02-14 |
| **Release Patch** | V1.0.0.134 |
| **Roles** | Vanguard |
| **Riot Positions** | Support |
| **External Positions** | Support |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $646.0$ | $+100.0$ | $2346.0$ |
| **Mana** | $400.0$ | $+47.0$ | $1199.0$ |
| **Health Regen** | $8.5$ | $+0.55$ | $17.9$ |
| **Mana Regen** | $8.65$ | $+0.5$ | $17.1$ |
| **Armor** | $39.0$ | $+4.95$ | $123.2$ |
| **Magic Resist** | $32.0$ | $+2.05$ | $66.8$ |
| **Attack Damage** | $61.0$ | $+3.3$ | $117.1$ |
| **Attack Speed** | $0.706$ | $+1.0\%$ | $0.826$ |
| **Movement Speed** | $325.0$ | $+0.0$ | $325.0$ |
| **Attack Range** | $175.0$ | $+0.0$ | $175.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.706$ |
| **Attack Speed Ratio** | $0.612$ |
| **Bonus AS per Level** | $1.0\%$ |
| **Acquisition Radius** | $400 units$ |
| **Gameplay Radius** | $80 units$ |
| **Pathing Radius** | $50 units$ |
| **Selection Radius** | $125 units$ |
| **Selection Height** | $180 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Staggering Blow

**Innate:** **Nautilus**’s basic attacks deal **bonus** physical damage and briefly root the target.

*This effect cannot occur on the same target more than once every few seconds.*

**Innate:** ''Nautilus' **basic attacks are empowered to deal 14 to 116 **bonus'' physical damage and root the target for changedisplay=true seconds. This effect cannot occur on the same target more than once every few seconds.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | Special |
| **Spell Effects** | Proc |

**Notes:**

- Spell shield will block the root but not the bonus damage.
- 'Staggering Blow's enhanced attack can be blocked and dodge (the on-target cooldown will still be applied).
  - It will fail to trigger while he is blind.
- The empowered attack will not trigger against structures. *

---

### Q: Dredge Line

**Active:** **Nautilus** hurls his anchor in the target direction that stops on the first enemy or terrain hit.

*If the anchor hits an enemy, it deals magic damage and airborne them toward **Nautilus** while he also dash toward them. Afterwards, the target is briefly root.*

**Active:** **Nautilus** hurls his anchor in the target direction that stops on the first enemy or terrain hit. If the anchor hits an enemy, it deals magic damage, standard sight them for $1.15$ seconds, stun them for 1 second, and airborne them toward **Nautilus** while he also dash toward them, both over $0.9$ seconds. If the anchor hits terrain, **Nautilus** dash to that location and 50% of 'Dredge Line's *cooldown* and mana are refunded. **Nautilus is lockout to move or attack while Dredge Line is in flight. He can cast any of his abilities during the dash.**

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $14-10$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | 60 Mana |
| **Targeting** | Direction |
| **Affects** | Self, Enemies |
| **Damage Type** | Magic |
| **Speed** | 2000 units/second |
| **Spell Shield** | Special |
| **Spell Effects** | spell |
| **Projectile** | True |

**Scaling:**
- **Magic Damage:** $85-265$ (+ 90% AP)

**Notes:**

- *Dredge Line* has two hitboxes. The outer hitbox, shown by a visual effect on the ground, will hit champions edge-to-edge. The inner hitbox, shown by the anchor, can hit terrain. Near thinner walls, this can cause the ability to hit a champion on the other side.
- *Dredge Line* turns **Nautilus**' facing direction accordingly upon hitting a valid target.
- *Dredge Line* can interact with player-generated terrain.
- **Nautilus** will dash all the way to the target's location if *Dredge Line* executed them or they negated the airborne.
- *Dredge Line* will not interrupt any movement commands **Nautilus** was issued pre-cast so long as their directions somewhat correlate (this is intentional to smooth out traveling/escaping).
- Spell shield will block the ability but **Nautilus** will still dash partway to the target.
- The anchor projectile will stop if **Nautilus** dies while the projectile is traveling. Enemies hit will still be dealt damage. The airborne and root are not applied if **Nautilus** did not initiate the dash.
- Displacement immunity will not resist the application of the stun. Effect at cast time start
- The following table refers for interactions during 'Dredge Line's cast time.

---

### W: Titan's Wrath

**Active:** **Nautilus** shields himself for a few seconds. While the shield holds, his basic attacks apply *Pain of Wrath* to the target and surrounding enemies.

**Active:** **Nautilus** grants himself a shield for 6 seconds, and while it holds, his basic attacks are empowered to apply *Pain of Wrath* to the target and enemies near them. **Pain of Wrath:** The target takes magic damage over time, half dealt immediately and the other half dealt after $1.25$ seconds. 'Titan's Wrath basic attack reset ''Nautilus'* basic attack timer.*

| Attribute | Value |
|-----------|-------|
| **Cooldown** | 12 seconds |
| **Cast Time** | none |
| **Cost** | 60 Mana |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | 250 units |
| **Spell Shield** | Special |
| **Spell Effects** | Special |

**Scaling:**
- **Shield Strength:** $50-90$ (+ $8-12$%
- **maximum** health)
- **Total Magic Damage:** $30-70$ (+ 40% AP) Magic Damage per Instance $30/2-70/2$ (+ $40/2$% AP)

**Notes:**

- 'Pain of Wrath's first damage instance is tagged as spell damage, while the second damage instance is tagged as persistent damage.
- *Pain of Wrath* damage over time debuff replaces itself when reapplied to the target, restarting its tick timer. This will prevent the second instance of damage from the previous debuff.
- *Pain of Wrath* will be applied by ''Nautilus's* basic attack if he is affected by *Titan's Wrath* at the start of the attack windup, even if *Titan's Wrath' expires during the windup.
- *Pain of Wrath* will not be applied if the basic attack is dodged or blocked, but will be applied if it is blind.
- Spell shield will block the ability (but only a single application).
- The empowered attack does not apply *Pain of Wrath* to structures.

---

### E: Riptide

**Active:** **Nautilus** sends three waves of explosions that radiate from him, each dealing magic damage and briefly slow enemies hit.

**Active:** **Nautilus** sends three waves of explosions that radiate from him over $0.561$ seconds. Each wave deals magic damage to enemies hit, reduced to 50% against those hit by subsequent waves beyond the first, and slow them by an amount that decays over $1.5$ seconds. The first hit against monsters deals **bonus** magic damage.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $7-5$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $50-90$ Mana |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | 0 - 350 / 215 - 465 / 350 - 590 units |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |

**Scaling:**
- **Magic Damage:* $55-195$ (+ 50% AP)0.5-195×0.5$ (+ $500.5$% AP)2-195×2$ (+ $50×2$% AP) **Slow:** $30-50$%
- **Monster Bonus Damage:** $125-285$ (+ 50% AP)

**Notes:**

- The first wave is centered around **Nautilus**, while the second and third wave are centered at an offset of 25 units in front of **Nautilus** at the original cast location.
  - Because of this, there are a zone each in which you can be hit by only 1 wave, or all 3 waves, respectively, without moving.
- The first wave happens at 0 seconds after the cast time ends, the second wave at $0.297$ seconds and the third wave at $0.561$ seconds.
  - The intended timing may be $0.25$ seconds for each delay, however these are the measured, effective times.
- The *explosions* are only a visual effect. The first wave has 8, the seconds has 9, and the third wave has 10 explosions, equally distributed around the rings, starting with one explosion directly in **Nautilus**' facing direction on each.

---

### R: Depth Charge

**Active:** **Nautilus** sends a *Depth Charge* that chases the target enemy champion. It creates eruptions in its wake that deal magic damage to enemies hit, as well as briefly airborne and stun them.

*Upon reaching the target, the *Depth Charge* erupts a final time for increased effect against the target.*

**Active:** **Nautilus** sends out a *Depth Charge* that chases the target enemy champion, accelerating over time and creating eruptions every in its wake that also briefly grant sight of the area. Enemies hit by the eruptions are dealt magic damage, airborne for 1 second, and stun for a duration. Upon reaching the primary target, the *Depth Charge* erupts a final time at their location. The primary target takes increased damage, is stun for the same duration, and airborne for a modified duration.

| Attribute | Value |
|-----------|-------|
| **Range** | 825 units |
| **Cooldown** | $120-80$ seconds |
| **Cast Time** | $0.46$ seconds |
| **Cost** | 100 Mana |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | magic |
| **Speed** | 275 + $466.67$ per second units/second |
| **Effect Radius** | 225 / 300 /sight 750 units |
| **Spell Shield** | True |
| **Spell Effects** | aoe |
| **Projectile** | False |

**Scaling:**
- **Magic Damage:** $125-225$ (+ 40% AP)
- **Stun Duration:** $1-2$ seconds
- **Increased Damage:** $150-400$ (+ 80% AP)
- **Knock Up Duration:** $1-2$ seconds

**Notes:**

- *Depth Charge* starts in an offset from whichever direction **Nautilus** is facing at the end of the cast time.
  - Since *Dredge Line*’s causes **Nautilus** to change his facing direction upon hitting a valid target, 'Depth Charge's* starting point can be changed with a quick *Dredge Line* + *Depth Charge' combo.
- Enemies a certain distance away from the charge / eruption are instead airborne.
- 'Depth Charge's final eruption will occur prematurely if the primary target moves more than 3000 units away from it.
- *Depth Charge* will not stop chasing if the target dies or becomes untargetable.
- *Dredge Line* will not override 'Depth Charge's airborne.
- Displacement immunity will not resist the application of the stun.

---

## Patch History

### V25.16
- *Riptide*
  - Monster damage reduced to 100% from 150%.
    - *[Note: This affected **all** three hits.]*
  - **New Effect:** The first hit against monsters now deals $125-285$ (+ 50% AP) **bonus** magic damage.
    - *[Note: This affects the first **hit**, and not specifically the first **wave** only.]*

### V25.04
- *Dredge Line*
  - Base damage increased to $85-265$ from $70-250$.

### V14.19
- Nautilus
  - **Bug Fixes:** Resolved VO audio being too low in certain languages due to the audio filter.

### V14.12
- *Titan's Wrath*
  - **New Effect:** Now applies spell effects.

### V13.17
- Nautilus
  - *Dredge Line*
    - **Bug Fixes:** No longer plays incorrect model animations when it was cast.

### V13.3
- *Staggering Blow*
  - **Undocumented/Bug Fix:** No longer locks his facing direction after the attack has landed, which would previously cause *Dredge Line*’s cast direction to be obscured if cast towards another target right after.

### V13.3
- *Staggering Blow*
  - Base damage increased to 14 to 116 from 8 to 110.
- *Titan's Wrath*
  - Mana cost reduced to 60 from 80.
  - Base shield increased to $50-90$ from $40-80$.
- *Riptide*
  - Base damage increased to $55-195$ from $55-175$.
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
- *Titan's Wrath*
  - Base shield reduced to $40-80$ from $45-85$.
  - Shield health ratio reduced to $8-12$% **maximum** health from $9-13$%.

### V11.15
- Nautilus, Nautilus, Nautilus, Nautilus, and Nautilus
  - **Bug Fixes:** Recall sound effects now play correctly.
- *Titan's Wrath*
  - **Bug Fixes:** Fixed the AP ratio to its intended value (40%).
- *Riptide*
  - **Bug Fixes:** Fixed the slow to its intended value (30%).

## Trivia

- Nautilus' dance references the 'Peeparoonie'.
  - A side-by-side comparison can be seen here.
- A tab for Urgot can be seen in Nautilus'Art Spotlight'.
- Nautilus has the longest basic attack animation in-game (which results in enemies being rooted by *Staggering Blow* when they visibly got out of range).
- Nautilus was released without the 'Tank' tag, though he actually is one. It was added shortly after.
- Nautilus does the Captain Morgan pose when standing idle for a long while.
- *Nautilus* comes from Greek word for sailor, ναυτίλος.
  - Out of universe, Nautilus is also the name of Nautilus, the submarine from Twenty Thousand Leagues Under the Sea by Jules Verne, and subsequently the first USS Nautilus (SSN-571)
- Nautilus and **Ziggs** were first conceived as 'Ivan the Mad Bomber'.
- During Nautilus' joke, when he is swimming in the air, he is humming a tune. This is the same tune that Squidward hums to himself in the shower during the Spongebob Movie.
- Nautilus is voiced by Richard Newman (actor) in Legends of Runeterra.

---
*This page was automatically generated from League of Legends Wiki data.*