# Zed

## Table of Contents
- [Basic Information](#basic-information)
- [Statistics](#statistics)
  - [Map-specific Stats](#map-specific-stats)
- [Pets](#pets)
- [Abilities](#abilities)
- [Patch History](#patch-history)
- [Trivia](#trivia)

## Basic Information

| Attribute | Value |
|-----------|------:|
| **Champion** | Zed |
| **Title** | the Master of Shadows |
| **Resource** | Energy |
| **Range Type** | Melee |
| **Release Date** | 2012-11-13 |
| **Release Patch** | V1.0.0.151 |
| **Latest Changes** | V25.15 |
| **Roles** | Assassin |
| **Riot Positions** | Middle |
| **External Positions** | Jungle, Middle |
| **Blue Essence** | 675 |
| **Riot Points** | 585 |
| **Difficulty** | 3 |
| **Hero Type** | Assassin |
| **Adaptive Type** | Physical |
| **Damage** | 3 |
| **Toughness** | 1 |
| **Control** | 1 |
| **Mobility** | 3 |
| **Utility** | 1 |
| **Style** | 55 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $654.0$ | $+99.0$ |
| **Energy** | $200.0$ | $+0.0$ |
| **Health Regen** | $7.0$ | $+0.65$ |
| **Energy Regen** | $50.0$ | $+0.0$ |
| **Armor** | $32.0$ | $+4.7$ |
| **Magic Resist** | $29.0$ | $+2.05$ |
| **Attack Damage** | $63.0$ | $+3.4$ |
| **Attack Speed** | $0.651$ | |
| **Movement Speed** | $345.0$ | $+0.0$ |
| **Attack Range** | $125.0$ | $+0.0$ |
| **Base Attack Speed** | $0.651$ | |
| **Attack Speed Ratio** | $0.651$ | |
| **Bonus AS per Level** | $3.3\%$ | |
| **Acquisition Radius** | $400$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $100$ units | |
| **Selection Height** | $100$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $105.0\%$ |
| **Damage Taken** | $95.0\%$ |

#### URF

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $85.0\%$ |
| **Damage Taken** | $110.0\%$ |

## Pets

### Shadow

| Attribute | Value |
|-----------|------:|
| **Sight** | 700 |
| **Control** | Mimics Razor Shuriken and Shadow Slash whenever **Zed** casts them, regardless of the range between each other. |
| **Targeting** | Minion (untargetable) |

**Abilities:**

- **Mimic:** The *Shadow* casts Razor Shuriken and Shadow Slash whenever **Zed** does.
- It fires Razor Shuriken towards the same targeted location. 
- If it hits a mimicked ability on the same target as **Zed** or another *Shadow* does, a portion of **Zed**’s energy is restored.
- Mimicked Shadow Slashes slow enemies hit briefly. Multiple *slashes* do not deal additional damage, but the slow's strength is increased.
- Mimicked abilities:
  - Store damage dealt on Death Mark.
  - Do not count as ability activations for the purposes of on-cast effects such as Force Pulse’s passive.
  - Are considered separate casts when they hit the same target (e.g. for the purposes of Electrocute, Phase Rush, Eclipse, etc).
- **Shadow Swap:** The *Shadow* can swap its position with **Zed** whenever he recasts either Living Shadow or Death Mark, depending on which ability it was summoned from. **Zed** may only swap places with a *Shadow* once.
- **Shadow Limitation:** The *Shadow* disappears as soon as another *Shadow* is summoned by the same ability it spawned from.
- Only 1 *Shadow* each from Living Shadow and Death Mark may be active at a time.

---

## Abilities

### Passive: Contempt for the Weak

| Attribute | Value |
|-----------|------:|
| **On-target CD Static** | 10 |
| **Targeting** | Passive |
| **Affects** | Enemies |
| **Damage Type** | magic |
| **Spell Shield** | False |
| **Spell Effects** | proc |
| **Parry** | Special |

**INNATE:** **Zed**’s basic attacks against targets below 50% **maximum** health are empowered to deal 6%@1; 8%@7; 10%@17 of the target's **maximum** health as **bonus** magic damage.

Against monsters, *Contempt for the Weak* deals 200% damage, capped at 300.

*Contempt for the Weak* cannot occur on the same champion more than once every few seconds.

**Notes:**

- The enhanced attack applies other on-hit effects and can critically strike as normal (the bonus damage cannot).
- *Contempt for the Weak* is not consumed when dodged or missed, but it is consumed when blocked.
- *Contempt for the Weak* considers the target's health upon landing the attack, not when it's declared.
  - If the target falls below the threshold while **Zed** is winding up the attack, *Contempt for the Weak* applies, and vice versa.
- The empowered attack will not trigger against structures.

---

### Q: Razor Shuriken

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 925 units |
| **Width** | 100 units |
| **Speed** | 1700 units/second |
| **Cost** | 75 / 70 / 65 / 60 / 55 energy |
| **Cooldown** | 6 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |
| **Projectile** | True |

**ACTIVE:** **Zed** throws a shuriken in the target direction that deals physical damage to enemies hit, reduced to 60% against targets beyond the first.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 80 / 120 / 160 / 200 / 240 (+ 100% **bonus** AD) |
| **Reduced Damage** | 48 / 72 / 96 / 120 / 144 (+ 60% **bonus** AD) |

**Notes:**

- *Shurikens* blocked by spell shield still count as being hit for the reduced pass-through damage.
- If **Zed** buffers an attack command during *Razor Shuriken*’s cast time the attack will perform slower than if **Zed** had attacked after the cast time.
  - The same does not happen if **Zed** recasts Living Shadow during *Razor Shuriken*’s cast time.

- This ability will cast from wherever the caster is at the end of the cast time.

---

### W: Living Shadow

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 650 (Maximum shadow placement range) units |
| **Effect Radius** | 2000 (Recast range) / Global (Ability mimic range) |
| **Speed** | 2500 units/second |
| **Cost** | 40 / 35 / 30 / 25 / 20 energy |
| **Cooldown** | 20 / 19 / 18 / 17 / 16 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Location / Auto |
| **Affects** | Self |
| **Out of Range Behavior** | Target at maximum range (first cast clamped) |
| **Grounded** | Special |

**PASSIVE:** **Zed** restores energy whenever he and his *Shadows* or multiple *Shadows* hit an ability on the same target. This may only occur once per cast ability.

| Attribute | Value |
|-----------|------:|
| **Energy Restored** | 30 / 35 / 40 / 45 / 50 |

**ACTIVE:** **Zed** creates a *Shadow* that dashes to the target location and remains there for $5.25$ seconds, being able to mimic *Razor Shuriken* and *Shadow Slash* regardless of range. *Living Shadow* can be recast while within range of the *Shadow*.

**RECAST:** **Zed** and the *Shadow* blink to swap places. If *Living Shadow* was recast while the *Shadow* is dashing, the recast will instead occur once it has been placed.

*If an ability is cast while the Shadow is dashing, it will cast that ability once it has been placed. See [Pets](#Pets) for more details about Shadows.*

**Notes:**

- **Zed** gains a buff for $5.25$ seconds that indicates and determines the duration in which he may recast *Living Shadow*.
  - The buff is granted briefly after *Living Shadow* is cast.
    - If cast at minimum distance, the buff and the *Shadow*’s individual durations will effectively start at the same time.
    - If cast at greater than the minimum distance, then the buff's duration will start before the *Shadow*’s duration.
- The passive will grant its energy refund at rank 1 even if *Living Shadow* hasn't been learned yet.
- The energy refund is not granted if the mimicked ability is blocked by a spell shield.
- *Living Shadow* cannot be recast while grounded or rooted.
- If *Living Shadow* is recast during Q’s cast, **Zed** will instantly release the shuriken from the cast location of Q while still remaining in cast time. Once he has swapped places, he will then throw the shuriken that the *Shadow* would have thrown instead.
  - **Zed** will cast Razor Shuriken as he normally would from the new location, but over the remaining cast time and towards the same point of cast.
  - The *Shadow* will not throw a shuriken from its new location, but will continue to play the cast animation for Q.
  - This special interaction does not occur with the swap recast of Death Mark; the recast will be buffered if used during Q’s cast.
- **Zed** will swap places with the *Shadow* by all means if he buffers *Living Shadow*’s recast during its dash.
  - *Living Shadow* cannot be recast while **Zed** is marking his target with R, but may still be recast if it was buffered.
- If **Zed** casts *Living Shadow* while a non-*Death Mark Shadow* is already present, the previous *Shadow* will instantly disappear.
- If a *Shadow* expires while in the cast time of Q, the mimicked cast will be cancelled.
- *Shadows* will cast E independently of being in Q’s cast time or not.
- *Shadows* spawned from *Living Shadow* may sometimes last for up to an additional $0.25$ seconds.
- *Shadows* will not disappear when **Zed** dies.

---

### E: Shadow Slash

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 315 (From Zed) / 290 (From shadow clones) units |
| **Cost** | 40 energy |
| **Cooldown** | 5 / 4.5 / 4 / 3.5 / 3 seconds |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | Special |
| **Spell Effects** | aoe |

**ACTIVE:** **Zed** slashes to deal physical damage to nearby enemies.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 70 / 95 / 120 / 145 / 170 (+ 80% **bonus** AD) |

Enemies hit by a *Shadow*’s slash are slowed for $1.5$ seconds, with multiple slashes (See notes) dealing no additional damage but the slow's effectiveness being increased by 50%.

| Attribute | Value |
|-----------|------:|
| **Slow** | 20 / 25 / 30 / 35 / 40% |
| **Enhanced Slow** | 30 / 37.5 / 45 / 52.5 / 60% |

*Living Shadow*’s **current** cooldown is reduced by 3 seconds for each enemy champion hit by **Zed**’s slash.

**Notes:**

- An enemy counts as being hit by *multiple* slashes at once when they are hit by a *Shadow Slash* from **Zed** and at least one of his *Shadows*, or two of his *Shadows*. In any case, a target can only be damaged by one slash per cast of *Shadow Slash*.
  - The enhanced slow strength is applied as long as the enemy is hit by at least two slashes, even if **Zed**’s slash is one of them despite him not being able to apply the base slow himself.
    - Slashes beyond the second have no additional effect on the target; a third slash against an enemy does not deal damage nor increase the slow's strength (slow effectiveness is capped at 150% of the base strength).
- Spell shield will only block one *Shadow Slash* even if multiple of them hit at the same time, prioritizing blocking the slash from **Zed**’s *Shadow*.
  - The damage from at least one slash will always be blocked but still counts for preventing other slashes from applying the damage (other slashes are considered to be *additional* slashes which are not eligible for dealing the damage).
  - The slow will be blocked if hit by **Zed**’s slash and a *Shadow*’s slash, due to the latter slash being blocked.
  - The slow will be applied at base strength if hit by the slashes of two *Shadows*, due to at least one slash from a *Shadow* successfully hitting the target with the other slash being blocked (thus preventing the slow from being enhanced).
  - The slow will be applied at enhanced strength if hit by the slashes of **Zed** and two of his *Shadows*, due at least two of the slashes successfully hitting the target (in this case, the slashes from **Zed** and one of the *Shadows*).
- *Shadow Slash* does not interrupt attack, movement, or cast commands.

---

### R: Death Mark

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 625 units |
| **Effect Radius** | Global (Recast and ability mimick range) |
| **Cooldown** | 120 / 115 / 110 / 105 / 100 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Unit / Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | Special |
| **Spell Effects** | Single target |
| **Out of Range Behavior** | Walk in range of the target unit to cast (first cast) |
| **Grounded** | True |
| **Knockdown** | Special |

**ACTIVE:** **Zed** becomes untargetable and, after a $0.6$-second delay, he dashes to 125 units beyond the target enemy champion from his position at the end of the delay over $0.35$ seconds. Afterwards, he becomes targetable again, renders the target *Marked for Death* for 3 seconds, and gains ghosting for the same duration.

**Zed** also spawns a *Shadow* at his casting position for 9 seconds which is able to mimic *Razor Shuriken* and *Shadow Slash* regardless of range. *Death Mark* can be recast after $0.5$ seconds of his reappearance while the *Shadow* is active, though not in the last $1.25$ seconds of its duration.

**MARKED FOR DEATH:** **Zed** stores a portion of all pre-mitigation (Damage calculated before modifiers) physical damage and magic damage he and his *Shadows* deal to the target, detonating at the end of the duration to deal physical damage.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 100% AD (+ 25 / 40 / 55% of damage stored) |

**RECAST:** **Zed** and the *Shadow* blink to swap places.

*If the target dies or is 1950 or more units away before the dash begins, Death Mark is cancelled, causing the ability to go on a (cd) $0.5$-second cooldown, **Zed** to reappear at the cast location, and the Shadow he spawned to disappear.*

**Notes:**

- **Zed** gains a buff for $7.5$ seconds that indicates and determines the duration in which he may recast *Death Mark*.
  - The buff is granted on-cast and lingers for $0.25$ seconds after it expires.
- The *Shadow* is spawned the moment *Death Mark* is cast.
- **Zed** will track the target if they change locations.
  - He will dash through the target's previous location while still applying the mark if they are 2200 or more units away.
- **Zed** will become targetable, apply the mark, and gain the ghosting as soon as his dash ends.
  - The mark will not be applied to the target if they are untargetable by the time **Zed**’s dash ends.
- **Zed** will be ordered to basic attack his target after reappearing.
- *Death Mark*’s stored damage derives from all basic attacks, abilities, items, runes and buffs.
- *Death Mark*’s *Shadow* is functionally the same as a regular one, but it lasts longer and can be swapped to regardless of range.
  - The range at which it mimics **Zed**’s abilities does not change.
- Spell shield will only block the mark's application. They will not prevent **Zed** from initiating *Death Mark* against the target nor block the detonation from a mark that is already applied.
- If **Zed** dies during *Death Mark*, the ability will cancel instantly and he will reappear at his current location. The cooldown is not refunded.
  - If he enters resurrection before the dash, *Death Mark* does not cancel. If he enters it while dashing, the dash is interrupted.
- If the dash is interrupted, **Zed** stops, places himself to the ground (ends all displacements affecting him, but does not remove airborne effects), and reappears prematurely at his current location. The mark will still be applied regardless.
  - **Zed** will remove airborne effects from himself immediately upon starting the dash.
- Once **Zed** begins the dash, two *Shadows* will also appear to be dashing through his target, each one spawning a certain distance behind the target (from the right and left). These *shadows* do **not** mimic abilities and disappear immediately after they stop dashing but they grant sight of their surroundings.
  - These *shadows* have a tracking distance of 2200 and a dash speed of 1750.
- *Death Mark* does not cancel if the target enters a zombie state before the dash begins.
- If the mark's post-mitigation damage (Damage calculated after modifiers) is higher than the target's **current** health, a *spinning shuriken* will appear above the victim. This does not necessarily mean the target will die, as it does not consider shields or forms of damage modifiers that are not respected by this predictive calculation.
- This following table refers for interactions while **Zed** is performing *Death Mark*:

---

## Patch History

### V25.15
- Zed
  - **Bug Fixes:** Haste animations are no longer missing and no longer always use the base walk animation when at higher movement speeds and/or during Homestart/Homeguard.

### V25.08
- Shadow Slash
  - **Bug Fixes:** Now correctly reduces Living Shadow’s cooldown by 3 seconds instead of 2 even if a *Shadow* from *Living Shadow* is present while the recast is available.

### V25.05
- Shadow Slash
  - Base damage increased to 70 / 95 / 120 / 145 / 170 from 65 / 90 / 115 / 140 / 165.
  - Bonus AD ratio increased to 80% **bonus** AD from 65%.

### V14.15
- Razor Shuriken
  - Base damage increased to 80 / 120 / 160 / 200 / 240 from 80 / 115 / 150 / 185 / 220.
    - Subsequent target base damage increased to 48 / 72 / 96 / 120 / 144 from 48 / 69 / 90 / 111 / 132.
  - Bonus AD ratio reduced to 100% **bonus** AD from 110% **bonus** AD.
    - Subsequent target bonus AD ratio reduced to 60% **bonus** AD from 66% **bonus** AD.
- Death Mark
  - Cooldown increased to 120 / 110 / 100 seconds from 120 / 100 / 80.

### V14.14
- Razor Shuriken
  - Base damage increased to 80 / 115 / 150 / 185 / 220 from 70 / 105 / 140 / 175 / 210.
    - Subsequent target base damage increased to 48 / 69 / 90 / 111 / 132 from 42 / 63 / 84 / 105 / 126.
- Shadow Slash
  - Living Shadow cooldown reduction increased to 3 seconds from 2.

### V14.1#January 12th Hotfix|V14.1
- Living Shadow
  - Cooldown reduced to 20 / 19 / 18 / 17 / 16 seconds from 20 / 19.25 / 18.5 / 17.75 / 17.
- Shadow Slash
  - Base damage increased to 65 / 90 / 115 / 140 / 165 from 65 / 85 / 105 / 125 / 145.
- Death Mark
  - AD ratio increased to 100% AD from 65% AD.

### V13.20
- Living Shadow
  - Cooldown increased to 20 / 19.25 / 18.5 / 17.75 / 17 seconds from 20 / 18.5 / 17 / 15.5 / 14.
- Zed
  - Skin renamed to *Worlds 2016 Zed* from *Championship Zed*.

### V13.18
- General
  - Updated ability icons.

### V13.17
- Zed
  - Contempt for the Weak
    - **Bug Fixes:** Model's lower body no longer becomes frozen in motion when performing the empowered attack.

### V13.5
- Stats
  - Base magic resistance reduced to 29 from 32.
- Shadow Slash
  - Cooldown reduced to 5 / 4.5 / 4 / 3.5 / 3 seconds from 5 / 4.75 / 4.5 / 4.25 / 4.

## Trivia

- This champion has no ability power ratio.
- Zed's dance was inspired by various martial arts (the animator is a Black belt (martial arts)).
- Zed is the first champion to not actually 'die' in his death animation, instead he drops into a shadow portal (Shockblade Zed disappears after struck by a lightning bolt). The second is Ekko, who rewinds time. Willump does die, but Nunu does not.
- Zed, Sona, Garen and Vi were targeted by Jhin in his teaser.
  - After Zed was shot, his champion icon on his League of Legends page and the champion list was updated to a gif with petals blowing past him.
- Zed is the fifth champion to use **energy** as a resource for abilities, after Akali, Kennen, Lee Sin, and Shen, and the next one being Ambessa.
- Before the release of Zed and Shen, neither him nor Shen reveal their faces in any of their skins, unlike Akali (Akali, Akali, Akali) and Kennen (Kennen,Kennen).
- Zed, Lucian, and Mel perform the 'Naruto run' run when having very high movement speed.
- Razor Shuriken, Living Shadow, and Shadow Slash respectively mirroring Thundering Shuriken, Shadow Dash, and Crescent Slash, as well as Death Mark, Assassin's Mark, and Mark of the Storm marking their targets reference Zed having been a Kinkou Order member.
  - Death Mark was conceived with spawning two Living Shadows and was animated accordingly on release.
  - Clicking one of the Living Shadows during Death Marks animation used to display Akali’s portrait. This could only be done while spectating in slow motion as his shadows in an actual game were just simply too fast to be clicked on. It was removed after the new HUD came in 2015 and replaced the old one.
  - Zed and his enemy counterpart could theoretically increase their AD infinitely with Reaper of Shadows by alternating killing each other, before it was removed from the game in V9.4.
- Zed used to gain the 'Law of Inverse Ninja Strength' cosmetic Easter egg debuff (*"This unit is a flippin' ninja!"* - *"Ninjas are more effective when they work alone. For every Ninja on your team beyond yourself, you lose 1 health."*) when he, Akali, Kennen, and/or Shen found themselves on the same team. It was removed in V3.14 for unknown reasons.
- At 23 August 2018 players who selected "Assassin" as their favorite class rated Zed 27% higher than the other assassins as "One of my favorite champions". This difference is the largest for a champion by class.

---
*This page was automatically generated from League of Legends Wiki data.*