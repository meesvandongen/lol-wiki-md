# Naafiri

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
| **Champion** | Naafiri |
| **Title** | the Hound of a Hundred Bites |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2023-07-19 |
| **Release Patch** | V13.14 |
| **Roles** | Assassin |
| **Riot Positions** | Middle |
| **External Positions** | Middle |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $610.0$ | $+105.0$ | $2395.0$ |
| **Mana** | $400.0$ | $+55.0$ | $1335.0$ |
| **Health Regen** | $7.5$ | $+0.7$ | $19.4$ |
| **Mana Regen** | $7.5$ | $+1.0$ | $24.5$ |
| **Armor** | $28.0$ | $+4.2$ | $99.4$ |
| **Magic Resist** | $32.0$ | $+2.05$ | $66.8$ |
| **Attack Damage** | $55.0$ | $+2.0$ | $89.0$ |
| **Attack Speed** | $0.663$ | $+2.1\%$ | $0.900$ |
| **Movement Speed** | $340.0$ | $+0.0$ | $340.0$ |
| **Attack Range** | $125.0$ | $+0.0$ | $125.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.663$ |
| **Attack Speed Ratio** | $0.625$ |
| **Bonus AS per Level** | $2.1\%$ |
| **Acquisition Radius** | $400 units$ |
| **Pathing Radius** | $35 units$ |
| **Selection Radius** | $120 units$ |
| **Selection Height** | $191.667 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: We Are More

**Innate:** **Naafiri** periodically summons a *[Packmate](#Pets)* that attacks the target of her attacks and abilities.

*Hitting champions or large monsters with abilities or killing enemies reduces the *cooldown*.*

**Innate:** Periodically, **Naafiri** summons a *Packmate* that fights alongside her. She may have up to 2@1; 3@9; 4@12; 5@15 *Packmates* at a time. **Naafiri** summons the maximum number of *Packmates* when the game starts and upon respawning. Hitting champions or large monsters with abilities reduces the *cooldown* by 4 seconds. Killing enemies reduces it by 1 second. *See [Pets](#Pets) for full details on Packmates.*

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Self, Enemies |

**Notes:**

- *Packmates* do not count as abilities for triggering the cooldown reduction.
- Only the initial hit of **Darkin Daggers** counts for reducing 'We Are More's cooldown; the damage over time will not trigger the cooldown reduction.
- Both hits of **Eviscerate** will trigger the cooldown reduction.

---

### Q: Darkin Daggers

**Active:** **Naafiri** hurls a dagger that deals physical damage to enemies hit and inflicts a bleed that deals **bonus** physical damage over a short time. *Darkin Daggers* can be recast at no additional cost.

**Recast:** **Naafiri** mimics the first cast's effects.

**Active:** **Naafiri** hurls a dagger in the target direction that deals physical damage to enemies hit and inflicts them with a bleed that deals **bonus** physical damage every $0.5$ seconds for 5 seconds. The bleed executes minions and non-epic monsters if they would be damaged below health. *Darkin Daggers* can be recast after $0.5$ seconds and within 4 seconds at no additional cost. If the dagger hits a target that is already bleeding, they are instead dealt the remaining bleed damage plus **additional bonus** physical damage, with the **base bonus** damage increased by key=% and the *scaling* for the **bonus** damage increased by key=%. If that target is also a champion or large monster, **Naafiri** heal herself. **Recast:** **Naafiri** mimics the first cast's effects.

| Attribute | Value |
|-----------|-------|
| **Range** | 900 units |
| **Cooldown** | $9-7$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $55-75$ Mana |
| **Targeting** | Direction |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical |
| **Speed** | 1700 units/second |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |
| **Projectile** | True |

**Scaling:**
- **Initial Physical Damage:** $35-55$ (+ 20% bonus AD)
- **Bleed Physical Damage per Tick:** $35/10-135/10$ (+ $80/10$% bonus AD)bonus AD)bonus AD)
- **Minimum Bonus Physical Damage:** $30-90$ (+ 40% bonus AD)2-90×2$ (+ $40×3.5$% bonus AD)
- **Minimum Total Physical Damage:** $35+35+30-55+135+90$ (+ $20+80+40$% bonus AD)2)-55+135+(90×2)$ (+ $20+80+(40×3.5)$% bonus AD)
- **Heal:** $45-105$ (+ 40% bonus AD)

**Notes:**

- *Darkin Daggers* calculates its additional damage based on missing health before applying the remaining bleed damage.
- 'Darkin Daggers' execute against minions considers both the damage of the initial hit of the dagger and any bleed tick for putting the target under the threshold.

---

### W: The Call of the Pack

**Active:** **Naafiri** goes on the hunt for a period, briefly becoming untargetable and vanish her **Packmates**, gaining **bonus attack damage** and granting herself and her *Packmates* **bonus move speed**. She will also summon extra *Packmates* that last for the hunt's duration.

*'Casting *Hounds' Pursuit* briefly extends The Call of the Pack's hunt duration.'*

**Active:** **Naafiri** goes on the hunt for the next 5 seconds, becoming untargetable for the first 1 second and vanish all of her active **Packmates** in the first $0.9$ seconds. Her active *Packmates* will blink near her after their vanish. After $1.25$ seconds into the hunt, **Naafiri** summons 2 additional *Packmates* that last for the remaining duration and can exceed **We Are More*’s* summon cap. While on the hunt, **Naafiri** gains 20% AD **bonus attack damage** and grants herself and all *Packmates* **bonus movement speed**. 'Casting *Hounds' Pursuit* extends The Call of the Pack's hunt duration by $1.75$ seconds, up to its original duration.'**Naafiri** can move during The Call of the Pack's cast time.'

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $26-18$ seconds |
| **Cast Time** | $0.75$ seconds |
| **Cost** | 60 Mana |
| **Targeting** | Auto |
| **Affects** | Self |

**Scaling:**
- **Bonus Movement Speed:** $20-30$%

**Notes:**

- While *The Call of the Pack* is active, it increases **We Are More*’s* summon cap to 4@1; 5@9; 6@12; 7@15.
- The order of additional *Packmates* that despawn is predetermined, most likely their internal spawn IDs. The ability does not consider which *Packmates* are the closest or healthiest. Effect at cast time end

---

### E: Eviscerate

**Active:** **Naafiri** dash to deal physical damage to enemies she passes through. Upon arrival, she explodes in a flurry of blades to deal physical damage to nearby enemies.

**Packmates** briefly vanish and heal to full health.

**Active:** **Naafiri** dash in the target direction and deals physical damage to enemies she passes through. Upon arrival, she explodes in a flurry of blades to deal physical damage to nearby enemies. **Packmates** vanish during the dash and blink near **Naafiri** on her arrival. They are also heal to full health upon their reappearance.

| Attribute | Value |
|-----------|-------|
| **Range** | 250 – 450 / $450+200$ units |
| **Cooldown** | $11-7$ seconds |
| **Cast Time** | none |
| **Cost** | 35 Mana |
| **Targeting** | Location |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical |
| **Speed** | 900 units/second |
| **Effect Radius** | 230 units |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |

**Scaling:**
- **Dash Physical Damage:** $15-55$ (+ 40% bonus AD)
- **Flurry Physical Damage:** $60-160$ (+ 80% bonus AD)
- **Total Physical Damage:** $15+60-55+160$ (+ $40+80$% bonus AD)

**Notes:**

- *Packmates* will not be affected by *Eviscerate* while under the effect of '*Hounds' Pursuit*'.

---

### R: Hounds' Pursuit

**Active:** **Naafiri** and her **Packmates** briefly channel to single out an enemy. They then dash together toward the target. Upon colliding with the target, **Naafiri** deals physical damage to the target and briefly slows them, whereas *Packmates* deal a small portion of that damage. *Packmates* are untargetable during their windup and dash.

*If **Naafiri** scores a champion takedown shortly after, she can recast 'Hounds' Pursuit' once more within a duration at no cost.*

**Active:** **Naafiri** and her **Packmates** channel for $0.75$ seconds to single out the target enemy champion, true sight them in the duration. Upon completion of the channel, they dash to the target; upon arrival **Naafiri** deals physical damage and slows the target by 99% for $0.25$ seconds, whereas each *Packmate* deals $% of that damage. *Packmates* become untargetable during their channel and dash. If **Naafiri** scores a champion takedown within 7 seconds of cast, she triggers a pulse that grants sight of the surrounding area for 1 second and standard sight enemy champions within for 4 seconds. She can also recast 'Hounds' Pursuit' once from triggering this effect within 12 seconds at no cost. **Recast:** **Naafiri** immediately grants herself a shield for 3 seconds before mimicking the first cast's effects.

| Attribute | Value |
|-----------|-------|
| **Range** | 900 units |
| **Cooldown** | $110-80$ seconds |
| **Cast Time** | none |
| **Cost** | 100 Mana |
| **Targeting** | Unit |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical |
| **Speed** | 1800 units/second |
| **Effect Radius** | 2100 units |
| **Spell Shield** | Special |
| **Spell Effects** | spell |

**Scaling:**
- **Physical Damage:** $ (+ **Physical Damage per * (+ $
- **Shield:** $100-200$ (+ 150% bonus AD)

**Notes:**

- 'Hounds' Pursuit's* theoretical **total** physical damage from **Packmates** **only**, assuming **Naafiri** has the maximum number of them active and *Hounds' Pursuit' is always ranked at the earliest *experience* permitted (ranks $1/2/3$ at levels $6/11/16$):
  - **Levels 6–8:** $ (+ $&emsp;/&emsp;**icononly=true** $ (+ $.
  - **Levels 9–10:** $ (+ $&emsp;/&emsp;**icononly=true** $ (+ $.
  - **Level 11:** $()*(1/2))3*icononly=true** $()*(1/2))*5 physical damage (+ $.
  - **Levels 12–14:** $()*(1/2))4*icononly=true** $()*(1/2))*6 physical damage (+ $.
  - **Level 15:** $()*(1/2))5*icononly=true** $()*(1/2))*7 physical damage (+ $.
  - **Levels 16–18:** $ (+ $&emsp;/&emsp;**icononly=true** $ (+ $.
- **Naafiri** marks the enemy champion targeted by 'Hounds' Pursuit' in order to gain assist credit on them, lasting for the standard credit timer.
- Spell shield will block ''Naafiri's* collision damage but cannot prevent collision damage from *Packmates'.
- **Naafiri** will track the target if they change locations, up to a maximum distance.
  - She will dash to the target's previous location if they move too far away during the dash.
- **Naafiri** will be ordered to basic attack the target at the end of the dash.
- **Naafiri** will windup a basic attack while dashing.
- *Packmates* hitting the same target as **Naafiri** will count as dealing "additional" damage.
- The target is standard sight for another $1.5$ seconds after the channel completes.
- The following table refers for interactions while **Naafiri** is channel:
  - If ''Naafiri's* channel is interrupted, the *Packmates' channels will be canceled as well.
  - The channel is cancelled if the target dies or moves too far away, but not if they become untargetable.

---

## Patch History

### V25.11
- *We Are More*
  - **Bug Fixes:** Move orders no longer persist on Packmates if the player issues a move order within a tiny radius of their current position, allowing the ordered Packmates to freely leave Naafiri's radius of influence and remain at their destination location indefinitely so long as the player does not issue a new move order (which would subsequently cause the Packmates to re-calculate their distance from Naafiri's radius of influence and return to her).
- *Hounds' Pursuit*
  - **Bug Fixes:** Naafiri and her Packmates no longer ignore collision with Emperor's Divide during their dash, which would previously allow them to bypass the displacement.

### V25.10
- *We Are More*
  - Packmate monster damage reduced to 135% from 155%.

### V25.09
- *The Call of the Pack*
  - Cooldown increased to $26-18$ seconds from $22-18$.
- *Eviscerate*
  - Dash AD ratio reduced to 40% *bonus AD from 50%.
- *Hounds' Pursuit*
  - ***Undocumented - Removed:*** Her own and her *Packmates'* dash speed no longer scales with 100% of Naafiri's **total** movement speed.
  - ***Undocumented - Removed:*** Tooltip no longer mentions the window for unlocking the recast is timed. *Actual effect unchanged.*
- *Darkin Daggers*
  - Initial base damage reduced to $35-55$ from $35-75$.
  - Total bleed base damage changed to $35-135$ from $30-150$.
    - Bleed base damage per tick changed to $3.5-13.5$ from $3-15$.
- *Eviscerate*
  - Flurry base damage reduced to $60-160$ from $60-180$.
  - Cooldown increased to $11-7$ seconds from $9-7$.

### V25.07
- Stats
  - Base health reduced to 610 from 635.
  - Base armor reduced to 28 from 30.
- *We Are More*
  - Packmate monster damage reduced to 155% from 165%.
  - **Bug Fixes:** Packmates no longer sometimes fail to dash alongside her during her abilities.
- *Darkin Daggers*
  - Minion damage increased to 100% from 80%.
- *The Call of the Pack*
  - Cooldown increased to $22-18$ seconds from $20-18$.
  - **Bug Fixes:** No longer creates a redundant buff icon.
- *Hounds' Pursuit*
  - **Bug Fixes:** Now properly crosses terrain when the target is beyond it.
- Stats
  - Base attack damage reduced to 55 from 57.
  - Base health regeneration reduced to $7.5$ from 9.
  - Health regeneration growth reduced to $0.7$ from $0.9$.

### V25.06
- Stats
  - Base attack damage increased to 57 from 55.
  - Attack damage growth reduced to 2 from $2.1$.
  - Armor growth reduced to $4.2$ from $4.7$.
  - Health growth reduced to 105 from 120.
- *We Are More*
  - Maximum Packmates increased to 2@1; 3@9; 4@12; 5@15 from 2@1; 3@9.
  - Packmate base attack damage reduced to 10 to 20 from 12 to 32.
  - Packmate attack damage bonus AD ratio reduced to 4% *bonus AD from 5%.
  - Packmate health reduced to 80+13*(x-1)*(0.7025+0.0175*(x-1)) from 80+16*(x-1)*(0.7025+0.0175*(x-1)).
  - Packmate resistances reduced to 0+1.8*(x-1)*(0.7025+0.0175*(x-1)) from 0+2*(x-1)*(0.7025+0.0175*(x-1)).
  - Packmate incoming area damage modifier increased to 76–55@1–15 (@=%) from 76–50@1–14 (@=%).
  - Packmate monster damage increased to 165% from 100%.
  - Packmate attack command duration reduced to 2 seconds from 3.
  - **Undocumented:** Packmate structure damage increased to 50% from 25%.
- *Darkin Daggers*
  - Minion damage increased to 80% from 60%.
- *The Call of the Pack* (W)
  - 'Swapped with *Hounds' Pursuit*.'
  - **New Effect:** Naafiri now becomes untargetable in the first second of the hunt's duration.
  - **Removed:*** No longer despawns any active Packmates.
  - **Removed:*** No longer spawns the maximum number of Packmates *We Are More* allows before spawning additional ones.
  - **New Effect:** Any active Packmates are now vanish in the first $0.9$ seconds of the hunt.
  - Additional Packmates summoned reduced to 2 at all ranks from $2-4 3$.
  - Hunt duration reduced to 5 seconds from 15.
  - Hunt duration now begins at the start of the cast time, rather than afterwards.
  - **Removed:*** Bonus movement speed no longer decays over 4 seconds.
  - **Removed:*** Bonus movement speed is no longer reduced by 50% upon receiving non-minion damage.
  - Bonus movement speed reduced to $20-30$% from $70-100 3$%.
  - Bonus movement speed duration increased to 5 seconds from 4.
  - **Removed:*** No longer grants a base amount of $5-25 3$ **bonus** attack damage.
  - Bonus attack damage AD ratio changed to 20% AD at all ranks from $8-24 3$% AD.
  - Cooldown reduced to $20-18$ seconds from $120-100 3$.
  - Mana cost reduced to 60 from 100.
  - **Removed:*** Her first basic attack or ability hit against a champion no longer grants her a $125-525 3$ (+ 50% *bonus AD) shield for 3 seconds.
  - **Removed:*** No longer refreshes the hunt's duration and reapplies the effects (excluding Packmate summoning) upon scoring the first champion takedown within the duration.
  - **Removed:*** No longer grants sight of the surrounding 2100 units for 2 seconds and reveals enemy champions within for 4 seconds.
  - **Removed:*** No longer increases Packmates' **maximum** health by 25%.
  - **Removed:*** No longer restores Packmates' health to full upon cast and upon the hunt ending.
  - **Removed:*** No longer reduces *We Are More*’s cooldown by 50%.
  - **Removed:*** No longer increases *Hounds' Pursuit*’s cast range by $80-240 3$.
- *Eviscerate*
  - Cooldown reduced to $9-7$ seconds from $10-8$.
  - Target range increased to 450 units from 350.
    - **New Effect:** Now dashes to cursor instead of always to maximum range.
  - Terrain grace increased to 200 units from 150.
  - Dash base damage reduced to $15-55$ from $35-95$.
- *Hounds' Pursuit* (R)
  - *Swapped with *The Call of the Pack*.*
  - **New Effect:** If Naafiri scores a champion takedown within 7 seconds of casting, she grants sight over a 2100 unit radius for 1 second and reveals enemy champions within for 4 seconds. Additionally, she is allowed to recast Hounds' Pursuit once in the next 12 seconds.
  - **New Effect:** Upon recasting, Naafiri gains a $100-200 3$ (+ 150 *bonus AD) shield for 3 seconds.
  - **New Effect:** Casting/recasting the ability now extends *The Call of the Pack*’s duration by $1.75$ seconds (up to its maximum).
  - **New Effect:** Dash speed now scales with 100% of her movement speed.
  - Cooldown increased to $110-80 3$ seconds from $22-14$.
  - Mana cost increased to 100 at all ranks from $70-30$.
  - Target range changed to 900 units at all ranks from $700-940$.
  - Base damage changed to $150-350 3$ from $30-190$.
    - Packmate base damage changed to $150×0.1-350×0.1 3$ from $30×0.1-190×0.1$.
  - Damage bonus AD ratio increased to 120% *bonus AD from 80%.
    - Packmate damage bonus AD ratio increased to $120×0.1$% *bonus AD from $80×0.1$%.
  - **Removed:*** Naafiri and her Packmates no longer collide with the first champion hit; they can only collide with the targeted enemy.
  - **Removed:*** Can no longer be cast on non-champion units.
  - **Removed:*** No longer refunds 50% of its cooldown if the channel is interrupted.
- *We Are More*
  - **Bug Fixes:** Packmate attacks against champions no longer trigger minion call-for-help against the summoner (if in valid range).
- *We Are More*
  - **Bug Fixes:** / dash now transports her Packmates to the target location.

### V14.22
- *Darkin Daggers*
  - **Bug Fixes:** Missile longer fails to apply its effects after **Naafiri** dies.

### V14.21
- *Hounds' Pursuit*
  - **Bug Fixes:** Using *Stridebreaker* Breaking Shockwave on a target affected by 'Hounds' Pursuit's slow no longer causes Breaking Shockwave's slow to last for its duration, which is longer than 'Hounds' Pursuit's*, but at *Hounds' Pursuit's 99% potency.

### V14.12
- *We Are More*
  - Packmate base attack damage increased to 12 to 32 from 6 to 30.
  - Packmate bonus AD ratio increased to 5% *bonus AD from $4.5$%.
- *Darkin Daggers*
  - Cooldown reduced to $9-7$ seconds from $11-7$.
  - **New Effect:** Heal now triggers against large and epic monsters.
  - **New Effect:** Execute can now also trigger against lesser monsters.
  - **Bug Fixes:** Tooltip now includes information about its minion execution effect.

### V14.9
- Stats
  - Gameplay radius increased to 65 units from 55.
  - Selection radius increased to 120 units from $111.11109924316406$.
- Stats
  - Base movement speed increased to 340 from 335.
- *We Are More*
  - Packmate base attack damage increased to 6 to 30 from 5 to 25.
  - Cooldown reduced to 30 to 10 seconds from 30 to 15.
- *Eviscerate*
  - Cooldown reduced to $10-8$ from $11-9$.

### V13.23
- *We Are More*
  - Packmate base attack damage reduced to 5 to 25 from 6 to 29.8.
- *Eviscerate*
  - Flurry base damage reduced to $60-180$ from $65-205$.

## Trivia

- This champion has no ability power ratio.
- Naafiri has a special joke animation which comes in a sequence of three animations, playing the next animation in the sequence once the appropriate key combination is pressed just as the previous animation ends.
  - The first animation plays by pressing Ctrl + 1 twice, which shows Naafiri being pet on the head by a hand resembling the Legacy cursor.
  - The second animation plays by pressing Ctrl + 2 as the first animation ends, with Naafiri being petted on the neck before falling on her back.
  - The third animation plays by pressing Ctrl + 3 as the second animation ends, showing Naafiri being rubbed on the belly while still lying down. Pressing Ctrl + 3 again during the end of the animation will allow it to be repeated again as many times as the player keeps pressing the key.
- Naafiri's Series 2 Eternals make the following references:
  - *Let the Dogs Out* is a reference to the song called Who Let The Dogs Out by Baha Men.
  - *What the dog doin?* is a reference to the What the Dog Doin' meme.

---
*This page was automatically generated from League of Legends Wiki data.*