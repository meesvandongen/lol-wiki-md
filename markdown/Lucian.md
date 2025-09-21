# Lucian

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
| **Champion** | Lucian |
| **Title** | the Purifier |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2013-08-22 |
| **Release Patch** | V3.10a |
| **Roles** | Marksman |
| **Riot Positions** | Bottom |
| **External Positions** | Bottom |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $641.0$ | $+100.0$ | $2341.0$ |
| **Mana** | $320.0$ | $+43.0$ | $1051.0$ |
| **Health Regen** | $3.75$ | $+0.65$ | $14.8$ |
| **Mana Regen** | $7.0$ | $+0.8$ | $20.6$ |
| **Armor** | $28.0$ | $+4.2$ | $99.4$ |
| **Magic Resist** | $30.0$ | $+1.3$ | $52.1$ |
| **Attack Damage** | $60.0$ | $+2.9$ | $109.3$ |
| **Attack Speed** | $0.638$ | $+3.3\%$ | $0.996$ |
| **Movement Speed** | $335.0$ | $+0.0$ | $335.0$ |
| **Attack Range** | $500.0$ | $+0.0$ | $500.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.638$ |
| **Attack Speed Ratio** | $0.638$ |
| **Bonus AS per Level** | $3.3\%$ |
| **Missile Speed** | $2800 units/second$ |
| **Acquisition Radius** | $800 units$ |
| **Pathing Radius** | $40.68 units$ |
| **Selection Radius** | $100 units$ |
| **Selection Height** | $122.222 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Lightslinger

**Innate:** After casting an ability, **Lucian**’s next basic attack within a few seconds fires a second attack that deals **reduced** physical damage.

**Innate - Vigilance:** Whenever **Lucian** is heal or shield, or when a nearby enemy is immobilized, his next two shots within a few seconds are empowered to deal **bonus** magic damage. The shots can stack up to a cap.

**Innate:** After casting an ability, ''Lucian's' next basic attack within $3.5$ seconds fires an additional shot on-attack after $0.25$ seconds, which deals key=% AD , increased to 100% AD against minion. He will shoot the same target, else another target within 50 units beyond his range automatically. The second shot applies on-hit effects, triggers on-attack effects, and is affected by critical strike modifiers. **Innate - Vigilance:** Whenever **Lucian** is heal or shield by an ally, or an enemy champion within 1000 units is immobilized, his next two shots within 6 seconds are empowered to deal 15 (+ 20% AD) **bonus** magic damage on-hit. He can store up to 4 empowered shots at a time.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Effects** | attack |

**Notes:**

- *Lightslinger* is triggered and consumed by the first attack, even if no target can be found upon firing the second attack.
- The second attack, if the initial target was killed, will prioritize enemy champions regardless of having sight of them or not and enemy minion with health.
- **Lucian** can perform actions freely while firing 'Lightslinger's second attack.
  - The second attack has special movement animations depending on the direction **Lucian** is moving while firing.
- The second attack counts as a separate hit for effects such as *Electrocute*, *Muramana* Shock, and *Eclipse* Ever Rising Moon.
- The second attack separately rolls a critical strike.
- The second attack does not trigger *Sundered Sky* Lightshield Strike.
- The empowered shots will apply against structures.

---

### Q: Piercing Light

**Active:** **Lucian** fires a laser in a line in the direction of the target enemy, dealing physical damage to enemies hit.

**Active:** **Lucian** fires a laser in a line in the direction of the target enemy that deals physical damage to enemies hit.

| Attribute | Value |
|-----------|-------|
| **Range** | 500 units |
| **Cooldown** | $9-5$ seconds |
| **Cast Time** | 0.4 to 0.25 seconds |
| **Cost** | $48-80$ Mana |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | AoE |
| **Projectile** | False |

**Scaling:**
- **Physical Damage:** $85-205$ bonus AD)

**Notes:**

- *Piercing Light* will attempt to lead the target if it is moving but does not adjust further during the cast time (enemies can dodge the laser if they change their position by a sufficient amount during the cast time).
- **Lucian** can buffer *W* or *R* during the cast time. *E* is locked out.
  - This is why **Lucian** can use them early if the cast time prematurely ends.
- 'Piercing Light's cast time ends prematurely if the initial target dies, allowing **Lucian** to attack and move freely, or to immediately begin casting either *W* or *R*.'

---

### W: Ardent Blaze

**Active:** **Lucian** fires a missile in the target direction that explodes in a cross pattern upon hitting an enemy or maximum range, dealing magic damage and marking enemies hit.

**Active:** **Lucian** fires a missile in the target direction that explodes in a cross pattern upon hitting an enemy or reaching maximum range, dealing magic damage to enemies hit and granting sight of the area for 1 second. Enemies hit are marked for 6 seconds. **Lucian** gains **bonus movement speed** for 1 second when he or allied champions damage a marked target. Allied champions triggering this effect grant **Lucian** **Vigilance**.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $14-10$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | 60 Mana |
| **Targeting** | Direction |
| **Affects** | Enemies / Self, Allies |
| **Damage Type** | Magic |
| **Speed** | 1600 units/second |
| **Spell Shield** | Special |
| **Spell Effects** | spellaoe |
| **Projectile** | True |

**Scaling:**
- **Magic Damage:** $75-215$ (+ 90% AP)
- **Bonus Movement Speed:** $60-80$

**Notes:**

  - Like most missiles, it will not collide with enemies whose center is beyond the maximum range (not behind **Lucian**), but this determines only the center and timing of the explosion.
- *Ardent Blaze* will cast from wherever **Lucian** is at the end of the cast time.
- **Lucian** will not gain 'Ardent Blaze's bonus movement speed if attacks on marked targets are dodge, block, or if missed while the attacker is blind.
- persistent damage will proc *Ardent Blaze* only once.
- Spell shield will not prevent the mark.

---

### E: Relentless Pursuit

**Passive:** 'Relentless Pursuit's* cooldown is cdr for each **Lightslinger*' shot hit, doubled against enemy champions.

**Active:** **Lucian** dash in the target direction.

**Passive:** 'Relentless Pursuit's* **current cooldown** is reduced by 1 second for each **Lightslinger*' shot hit, doubled to 2 seconds against enemy champions. **Active:** **Lucian** dash in the target direction. *Relentless Pursuit basic attack reset *'Lucian's** basic attack timer. **Lucian'* can cast any of his abilities during the dash. Relentless Pursuit can be cast during *W*’s cast time and during *R*’s channel.*

| Attribute | Value |
|-----------|-------|
| **Range** | 200 / 425 units |
| **Cooldown** | $18-14$ seconds |
| **Cast Time** | none |
| **Cost** | $40-0$ Mana |
| **Targeting** | Location |
| **Affects** | Self |
| **Speed** | 1350 units/second |

**Notes:**

- Additional on-hits during *Lightslinger* are not considered for the cooldown reduction (i.e *Runaan's Hurricane*, *Guinsoo's Rageblade*).
- Unlike some dashes, ''Lucian's* dash speed does *not' scale with his movement speed.
- 'Relentless Pursuit's dash can go through terrain, but will not be extended further if the target location is within terrain, unlike other dashes, like **Ekko**'s *Phase Dive*.
- Casting *Relentless Pursuit* directly after attacking with *Lightslinger* will reduce Relentless Pursuit's cooldown when the projectiles hit the target.
- *Relentless Pursuit* can be cast during cast time.
  - If *Relentless Pursuit* is cast just as the cast time ends, **Lucian** will not perform his Recall animation, instead staying idle. This allows **Lucian** to use his emotes during his Recall.

---

### R: The Culling

**Active:** **Lucian** channels for a few seconds to rapidly fire a torrent of bullets in the target direction. Each shot deals physical damage to the first enemy hit. *The Culling* can be recast within the duration.

*While channeling, **Lucian** is ghosted and may still move.*

**Active:** **Lucian** channels for up to 3 seconds, rapidly firing up to (+ 1 per shots in the target direction. Each shot deals physical damage to the first enemy hit, doubled against minions. *The Culling* can be recast after $0.75$ seconds during the channel, and does so automatically when the channel ends. While channeling, **Lucian** is ghosted and may still move. **Recast:** **Lucian** ends *The Culling*.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $110-90$ seconds |
| **Cast Time** | none |
| **Cost** | 100 Mana |
| **Targeting** | Direction / Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical |
| **Speed** | 2800 units/second |
| **Spell Shield** | Special |
| **Spell Effects** | aoedot |
| **Projectile** | True |

**Scaling:**
- **Physical Damage Per Shot:* $ (+ (+ (+ $% AD) (+ $% AP) Total Physical Damage $()* (+ $% AD) (+ $% AP) + $ (+ (+ per **Minion Damage Per Shot:* $( (+ $% AD) (+ $% AP) (+ $% AD) (+ $% AP)* (+ $% AD) (+ $% AP) + $()* (+ $% AD) (+ $% AP) per

**Notes:**

- At 100% critical strike chance, *The Culling* fires up to $100/ additional shots, for a total of shots.
  - The **total** damage at the maximum number of shots (at 100% critical strike chance) is $( physical damage (+ $% AD) (+ $% AP) physical damage.
  - Against minions, this is increased to $(* physical damage (+ $% AD) (+ $% AP).
- The fire rate increases with the number of shots.
- During *The Culling*, ''Lucian's' facing direction is in the direction that he is firing.
- **Lucian** can interact with *Dark Passage* or allied *Devour* without interrupt *The Culling*.
- *The Culling* creates the shots at an offset of 35 units to the front plus 35 units towards each of **Lucian** (sides alternating, first shot fired from ''Lucian's' right gun), unlike the missiles of most abilities.
- The damage dealt by each bullet of *The Culling* is calculated when the bullet reaches a target, just like single missile abilities.
- Spell shield only blocks one instance of damage.
- The following table refers for interactions while **Lucian** is channel:

---

## Patch History

### V25.11
- *Relentless Pursuit*
  - Cooldown increased to $18-14$ seconds from $16-14$.

### V25.06
- *Relentless Pursuit*
  - Cooldown reduced to $16-14$ seconds from $19-14$.

### V25.04
- *Lightslinger*
  - **Bug Fixes:** Secondary attack now properly targets ward-like units and plants.

### V14.16
- *Lightslinger*
  - Vigilance AD ratio increased to 20% AD from 15% AD.
- *Piercing Light*
  - Base damage reduced to $85-205$ from $95-215$.

### V14.2
- *Lightslinger*
  - **Bug Fixes:** VFX is no longer played at unnecessary times.

### V13.24
- *Relentless Pursuit*
  - Cooldown reduced to $19-14$ seconds from $22-14$.

### V13.18
- *The Culling*
  - **Bug Fixes:** Now properly triggers *Ardent Blaze*’s bonus movement speed again.

### V13.17
- *Lightslinger*
  - **Bug Fixes:** Now is properly triggered by *Font of Life*.

### V13.16
- *Lightslinger*
  - Vigilance base damage increased to 15 from 10.

### V13.13
- *Lightslinger*
  - **Bug Fixes:** Vigilance no longer sometimes repeats its VFX on the target.
- *Ardent Blaze*
  - **Bug Fixes:** Mark now gets properly triggered from ''Lucian's' own attacks to grant him movement speed.

## Trivia

- Lucian's is the first login screen to start with an animated sequence (his shows his wife Senna's soul being dragged into Dark Passage).
- *The Culling* used to be the first ability in the game to directly scale with attack speed (until V5.22) and is the first that allows firing backwards while moving.
- Lucian's Recall references Equilibrium (film).
  - A by comparison can be seen here.
  - The sloped front of Lucian's guns is also very reminiscent of the weapons wielded by the Clerics in Equilibrium.
- Lucian's dance references gun spinning from Metal Gear (series).
  - A by comparison can be seen here.
- Lucian was the first champion to ever get a Chroma Pack for his Classic skin.
- Of Lucian's guns the less ornate one is his (an L is engraved on its side) and the more intricate one was Senna's (an S instead); the letters were at first thought to mean *Light* and *Shadow*.
- People in Lucian's social circle possess light-theme names:
  - *Lucian* derives from Latin praenomen *Lucius*, from lux (cognate with English light), sharing one same Proto-Indo-European language root **leuk-* as **Lux**.
  - *Senna* from Arabic سناء *Sanāʼ* "brightness, sublimity", from root *s-n-y* (> Sin (mythology) "moon" & *seneh* "burning bush").
  - Lucian's father's name *Urias* Ουριας is the Greek rendition of Hebrew אוריה Uriah "my light is Yahweh".
- Lucian, **Mel** and **Zed** perform the 'Naruto run' when having very high movement speed.
- If **Senna** and **Thresh** are played on opposing teams, a quest will trigger for Lucian as well. This in-game quest consists of a battle of himself vs Thresh to earn the opposing champion's stack of souls, Lucian's death will grant Senna's stacks to Thresh.

---
*This page was automatically generated from League of Legends Wiki data.*