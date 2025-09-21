# Thresh

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
| **Champion** | Thresh |
| **Title** | the Chain Warden |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2013-01-23 |
| **Release Patch** | V1.0.0.154 |
| **Roles** | Catcher |
| **Riot Positions** | Support |
| **External Positions** | Support |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $620.0$ | $+120.0$ | $2660.0$ |
| **Mana** | $274.0$ | $+44.0$ | $1022.0$ |
| **Health Regen** | $7.0$ | $+0.55$ | $16.4$ |
| **Mana Regen** | $6.0$ | $+0.8$ | $19.6$ |
| **Armor** | $33.0$ | $+0.0$ | $33.0$ |
| **Magic Resist** | $30.0$ | $+1.55$ | $56.4$ |
| **Attack Damage** | $56.0$ | $+2.2$ | $93.4$ |
| **Attack Speed** | $0.625$ | $+3.5\%$ | $0.997$ |
| **Movement Speed** | $330.0$ | $+0.0$ | $330.0$ |
| **Attack Range** | $450.0$ | $+0.0$ | $450.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.625$ |
| **Attack Speed Ratio** | $0.625$ |
| **Bonus AS per Level** | $3.5\%$ |
| **Windup Modifier** | $0.25$ |
| **Missile Speed** | $0 units/second$ |
| **Acquisition Radius** | $475 units$ |
| **Pathing Radius** | $36 units$ |
| **Selection Radius** | $125 units$ |
| **Selection Height** | $150 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Damnation

**Innate:** **Thresh**’s does not increase by xp up.

**Innate:** Enemy champions and large enemies that die near **Thresh** will drop a *Soul* for a period. **Thresh** automatically collects *Souls* near him or **Dark Passage**.

**Innate:** ''Thresh's' armor does not increase through growth (per level). **Innate:** Enemy champions, large minions and large monsters that die near **Thresh** drop a *Soul* for 8 seconds. Epic monsters drop 2 *Souls* while lesser minions and monsters have a $33.3$% chance to drop a *Soul*. **Thresh** automatically collects *Souls* near him or a placed **Dark Passage**. **Soul:** For each stack, **Thresh** gains ap and armorbonus armor*.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Self |
| **Effect Radius** | 1900 units |

**Notes:**

- *Souls* grant a small area of sight.
- *Souls* are visible to allies and only become visible to enemies if their team has sight of **Thresh**.
- The probability for *souls* dropping from small minions and monster on Summoner's Rift starts at 33% but adjusts dynamically to match the expected quota of *souls* dropped (lowers if above, rises if below, remains if even). *Souls* collected are not considered when adjusting the probability.
- **Nunu** drop a soul each.

---

### Q: Death Sentence

**Active:** **Thresh** casts out his scythe in the target direction that catches the first enemy hit, dealing magic damage, cdr the cooldown, and briefly stun and true sight them. **Thresh** will also mark them *Shackled*, during which he is briefly slow and unable to declare basic attacks.

*While the target is stun, **Thresh** airborne them twice over a short distance. He can also recast *Death Sentence*.*

**Active:** **Thresh** throws out his scythe in the target direction, becoming lockout to move or attack while it is in flight. The scythe catches the first enemy hit to deal magic damage, stun and true sight them for $1.5$ seconds, and render them airborne for $0.4$ seconds, as well as reduce 'Death Sentence's* **current cooldown** by 2 seconds. **Thresh** will also mark the target *Shackled' for $1.5$ seconds, during which he is unable to declare basic attacks, and is slow by 20% for 1 second. While the target is stun, **Thresh** tugs his hook towards him twice, once after a $0.1$-second delay and again $0.6$ seconds after that. With each tug, the target is pulled a short distance towards **Thresh**. After $0.5$ seconds of hitting an enemy, or instantly after hitting a minion or monster, **Thresh** can recast the ability while the target is *Shackled*. **Recast - Deathly Leap:** **Thresh** stops tugging his hook and dashes to the *Shackled* enemy, becoming able to attack again upon arrival. He can cast **Dark Passage** and **Flay** during the dash.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $19-9$ seconds |
| **Cast Time** | $0.5$ seconds |
| **Cost** | 70 Mana |
| **Targeting** | Direction / Auto |
| **Affects** | Enemies / Self |
| **Damage Type** | Magic |
| **Speed** | 1900 / 1400 units/second |
| **Effect Radius** | Global / 3000 |
| **Spell Shield** | True |
| **Spell Effects** | spell |
| **Projectile** | True |

**Scaling:**
- **Magic Damage:** $100-300$ (+ 90% AP)

**Notes:**

- **Thresh** will track the target if they change locations.
  - He will dash to the target's previous location if the target is too far away or moves beyond 2000 units. Effect at cast time end
- The pull that occurs with each of ''Thresh's** tugs is a non-airborne displacement that displaces the target toward him. **Thresh'* will only tug if the target remains stun and he has not casted *Deathly Leap'.
  - **Thresh** will not tug his hook at all if the stun was not applied to the target.
  - These displacements will not be resisted by displacement immunity or cc-immune.
  - The first tug occurs after the $0.1$-second delay, while the second tug occurs after $0.7$ seconds of the target being stunned (or $0.6$ seconds from the end of the initial delay).
  - The initial airborne debuff that is applied to the target upon hitting them is removed if the displacement from the first pull is interrupted.
  - The target will not be pulled with the tugs if they are too close to **Thresh**.
- *Death Sentence* cannot be recast while ground or root.
- *Death Sentence* can be recast even while the target is untargetable.
- *Death Sentence* will not stun enemies that are displacement immune or cc-immune but the target will still be marked and **Thresh** is still granted the ability to recast for his dash.
- Spell shield will block all of 'Death Sentence's effects, including the mark as well as prevent the cooldown reduction.
- 'Death Sentence's stun duration is affected by tenacity but the mark will always persist for $1.5$ seconds, unless the target uses a cleanse effect, in which case the mark is removed.
- 'Death Sentence's stun will persist even if **Thresh** dies before the duration ends. The tugs will not occur as **Thresh** is not alive.
- The reveal lingers for $0.05$ seconds after it ends.
- *Deathly Leap* has a maximum cast range of ~3000 units. If **Thresh** or the target move a greater distance than this away from one another while the target is marked, *Deathly Leap* can still be cast but will fail to trigger the dash.
- **Thresh** will turn his facing direction towards the cast direction only after 'Death Sentence's cast is complete.
  - If **Thresh** is facing **more** than 90° away from the cast direction in either direction, he will turn to face *exactly* 90° away from the cast direction over the cast time, then turn towards the cast direction when the missile is fired.
  - If **Thresh** is facing **less** than 90° away from the cast direction, he will keep his facing direction until the missile is fired.
  - This prevents the enemy from knowing where exactly **Thresh** is aiming at before the cast animation is complete. **Death Sentence* triggers on-cast effects (such as Spellblade and triggering *Force Pulse*’s passive) once at the start of the cast, and once at the end of the cast. It may trigger on-cast effects a third time when casting *Deathly Leap*.
  - This is because a separate spell is cast to prevent **Thresh** from facing towards the target direction immediately, which is (incorrectly) flagged to trigger on-cast effects.

---

### W: Dark Passage

**Active:** **Thresh** throws his lantern to the target location that remains for a few seconds while he remains nearby. He and the first allied champion to come near the lantern are granted a shield for a few seconds, with the amount based on **Souls**.

*An ally can grab the lantern to dash to **Thresh** and gain the shield, but cannot do so while immobilize, ground, or silence.*

**Active:** **Thresh** throws his lantern to the target location over $0.5$ seconds, lasting for 6 seconds while he remains nearby and granting sight of its surroundings. If **Thresh** moves too far away from the lantern, it returns back to him immediately. **Thresh** and the first allied champion to come near the lantern are granted a shield for 4 seconds. An ally can select the lantern while in proximity of it, dash to **Thresh** and gaining the shield. *An ally cannot select the lantern while immobilize, ground, or silence. The lantern will not expire from **Thresh** moving too far away if he is dashing with *Deathly Leap*.*

| Attribute | Value |
|-----------|-------|
| **Range** | 950 units |
| **Cooldown** | $21-17$ seconds |
| **Cast Time** | none |
| **Cost** | $50-70$ Mana |
| **Targeting** | Location |
| **Affects** | Allies |
| **Effect Radius** | 350 / 150 units |
| **Projectile** | True |

**Scaling:**
- **Shield Strength:** $50-130$ (+ 2 per *Soul collected)*

**Notes:**

- The dashing ally will track **Thresh** if he changes locations.
  - They will dash to ''Thresh's' previous location if he is too far away or moves beyond 2200 units.
- The lantern is considered a unit and can be targeted by an allied Teleport, **Leap Strike**, **Shunpo**, and **Safeguard**.
  - It is untargetable to enemies.
- The lantern's duration and maximum leash range are each displayed as a circle on the ground.
- **Thresh** will gain 'Dark Passage's shield from moving out of leash range of the lantern.
- *Dark Passage* is special cased to trigger *Guardian*.

---

### E: Flay

**Passive:** **Thresh**’s basic attacks deal **bonus** magic damage based on **Souls**, which increases over a period without basic attacking enemies.

**Active:** **Thresh** sweeps his chain in a broad area, dealing magic damage to enemies hit and airborne them in the target direction, after which they are briefly slow.

**Passive:** ''Thresh's** basic attacks are empowered to deal **bonus'' magic damage, with the *AD* ratio increasing over 10 seconds without basic attacking enemies. **Active:** **Thresh** sweeps his chain across the ground in a broad line and a radius around him, starting behind him and towards the target direction. Enemies hit are dealt magic damage and airborne 200 units in the target direction, and then are slow for 1 second.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $13-10$ seconds |
| **Cast Time** | $0.3889$ seconds |
| **Cost** | $60-80$ Mana |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Speed** | 2000 units/second |
| **Effect Radius** | 270 units |
| **Spell Shield** | True |
| **Spell Effects** | Special |
| **Projectile** | False |

**Scaling:**
- **Minimum Bonus Magic Damage:** $1.7$ per *Soul collected (+ 0% AD)*
- **Maximum Bonus Magic Damage:** $1.7$ per *Soul collected (+ $90-210$% AD)*
- **Magic Damage:** $75-255$ (+ 70% AP)
- **Slow:** $20-40$%

**Notes:**

- 'Flay's effects start at the start of the cast time. **Thresh** can cast other spells once the cast time completes, but remains unable to attack and move and use mobility spells (such as Flash) until the chain completed its way entirely.
- Applies area damage on the ability and deals proc damage on the enhanced basic attack.
- The knockback's airborne debuff is set to last longer than the forced movement, but gets removed as soon as the forced movement from *Flay* ends or is overridden by another. *'Flay's passive's buff icon changes colors depending on charge level. At 100%, 'Thresh's scythe will glow green and a sound effect will play. Thresh Flay 2.png 0 - 50% Thresh Flay 3.png 50 - 75% Thresh Flay 4.png 75% - 100% Thresh Flay 5.png 100%
- *Runaan's Hurricane* Wind's Fury will apply 'Flay's passive to each enemy hit, with the secondary targets taking minimum damage (charge resets upon hitting the primary target).
- The enhanced attack applies other on-hit effects and can critical strike as normal (the bonus damage cannot).
- 'Flay's passive enhanced attack can be dodged (the enhanced attack is not consumed and the charge is not reset) and blocked (the enhanced attack is consumed and the charge is reset).
- The empowered attack will not trigger against structures nor wards.
  - : Enhanced attack's interactions with blind effects (regarding both bonus damage and charge reset).

---

### R: The Box

**Active:** **Thresh** erects a pentagon of spectral walls around him that each last a few seconds. A wall will break upon enemy champion contact, dealing magic damage and slow them for a short time. Subsequent wall breaks will not deal damage but still slow enemies briefly.

**Active:** **Thresh** erects a pentagon of spectral walls around him that each last for 5 seconds. A wall will break upon enemy champion contact, dealing magic damage and slow them by 99% for 2 seconds. After the first wall breaks, the rest will deal no damage and slow for only 1 second. Enemies that break a wall cannot do so again for 1 second.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $120-80$ seconds |
| **Cast Time** | $0.45$ seconds |
| **Cost** | 100 Mana |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |

**Scaling:**
- **Magic Damage:** $250-550$ (+ 100% AP)

**Notes:**

- *The Box* always uses quick cast, regardless of player settings.
  - The Box always faces the same direction Thresh is facing on cast, placing a corner directly in front of him. *** The cast indicator displays the opposite of the true behavior.
- If The Box hits multiple champions with different walls on cast, only one of the walls will deal damage and apply the extended slow duration. This wall is chosen by starting at the wall in front of and to the right of Thresh's facing direction on cast, and progressing clockwise.
- The corners of *The Box* are placed 400 units around **Thresh**.
  - Each wall is $470.228$ units long.
  - Each wall passes $323.607$ units from the center of *The Box* at the closest approach (ignoring its width).
  - As a regular pentagon, *The Box* has an angle of 72° between each corner.

---

## Patch History

### V25.18
- General
  - **Bug Fixes:** Soul absorb SFX no longer fails to play while he is performing the end of his Taunt expression in various skins.
- *Damnation*
  - **Bug Fixes:** Restored soul spawn SFX for allies and enemies.
- Thresh
  - *Death Sentence*
    - **Bug Fixes:** Hook projectile SFX no longer incorrectly plays on his location instead of the target's.
- Stats
  - Base health increased to 620 from 600.
- *Flay*
  - Minimum damage per soul increased to $1.7$ from $1.5$.
  - Maximum damage per soul increased to $1.7$ from $1.5$.
  - Maximum AD ratio increased to $90-210$% AD from $80-200$% AD.

### V14.9
- General
  - Adjusted splash artwork for Thresh.

### V14.8
- Stats
  - Base armor increased to 33 from 31.
  - Magic resistance growth increased to $1.55$ from $1.3$.

### V14.4
- Stats
  - Base armor increased to 31 from 28.
- *Flay*
  - Active base damage increased to $75-255$ from $75-235$.
- *The Box*
  - Cooldown reduced to $120-80 3$ seconds from $140-100 3$.

### V13.20
- Thresh
  - Skin renamed to *Worlds 2013 Thresh* from *Championship Thresh*.

### V13.18
- *Dark Passage*
  - Shield strength per soul increased to 2 from $1.5$.

### V13.7
- *Death Sentence*
  - Cooldown refund reduced to 2 seconds from 3.

### V13.4
- *Death Sentence*
  - Base damage increased to $100-300$ from $100-280$.
  - AP ratio increased to 90% AP from 80% AP.
  - Cooldown reduced to $19-9$ seconds from $19-11$.
- *Dark Passage*
  - Base shield reduced to $50-130$ from $50-150$.
  - Cooldown reduced to $21-17$ seconds from $22-16$.
- *Flay*
  - Base damage increased to $75-235$ from $75-215$.
  - AP ratio increased to 70% AP from 60% AP.
- *Flay*
  - AP ratio increased to 60% AP from 40% AP.

### V13.3
- *Death Sentence*
  - Base damage increased to $100-280$ from $100-260$.
  - AP ratio increased to 80% AP from 50% AP.
- *Dark Passage*
  - Shield strength per soul reduced to $1.5$ from 2.

### V12.18
- Stats
  - Health growth increased to 120 from 115.
- *Flay*
  - Active base damage increased to $75-215$ from $65-185$.

## Trivia

- 
  - In Thresh's case, *Damnation* infinitely stacks his ability power and armor.
- Thresh's name originated from the verb for harvesting (souls).
  - He may also be named after Dennis Fong, who popularized WASD movement keys and is considered the first professional gamer.
- Thresh was the first champion to:
  - Be released in 2013.
    - His base health at level 18 was also 2013 on release.
  - Cost on first week after release.
  - Feature ranged non-projectile basic attacks.
  - Feature an attack windup modifier (of 0.25).
  - Feature a Respawn animation on the Classic skin.
- Thresh and **Urgot** used to be the only champions classified as ranged tank before having their roles changes in 2013.
- Thresh has three different sets of basic attack animations depending on how far he is from the target he is hitting (melee, mid-range, full attack range).
- Thresh's dance references Poi (performance art) dancing (he swings his *scythe* and *lantern*, or his scythe and the lantern's hook if he cast *Dark Passage* before dancing).
  - A side-by-side comparison can be seen here.
- *Dark Passage* is the eldest ability that grants an allied displacement.
- Thresh's theme references Rain Rain Go Away and/or Ring a Ring o' Roses.
- Death Recap used to read *"You have opened *The Box*. Your prize: death"*.
- Thresh has a Nemesis Quest against **Senna** (optionally with **Lucian**).
  - Thresh's reward from Senna is called *Welcome Home*: *"Poor Senna, nobody escapes the lantern. Thresh has recaptured her soul and absorbed some of her power […]."*. A similar text reads for his reward from Lucian, which is called *Relax, just… let go.*

---
*This page was automatically generated from League of Legends Wiki data.*