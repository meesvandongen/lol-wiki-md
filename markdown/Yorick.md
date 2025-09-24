# Yorick

## Table of Contents
- [Basic Information](#basic-information)
- [Statistics](#statistics)
- [Pets](#pets)
- [Abilities](#abilities)
- [Patch History](#patch-history)
- [Trivia](#trivia)

## Basic Information

| Attribute | Value |
|-----------|------:|
| **Champion** | Yorick |
| **Title** | Shepherd of Souls |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2011-06-22 |
| **Release Patch** | V1.0.0.120 |
| **Latest Changes** | V25.13 |
| **Roles** | Juggernaut |
| **Riot Positions** | Top |
| **External Positions** | Top |
| **Blue Essence** | 1575 |
| **Riot Points** | 790 |
| **Difficulty** | 2 |
| **Hero Type** | Fighter |
| **Alt Type** | Tank |
| **Adaptive Type** | Physical |
| **Damage** | 2 |
| **Toughness** | 2 |
| **Control** | 2 |
| **Mobility** | 1 |
| **Utility** | 2 |
| **Style** | 55 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $650.0$ | $+114.0$ |
| **Mana** | $300.0$ | $+60.0$ |
| **Health Regen** | $8.0$ | $+0.8$ |
| **Mana Regen** | $7.5$ | $+0.75$ |
| **Armor** | $36.0$ | $+4.5$ |
| **Magic Resist** | $32.0$ | $+2.05$ |
| **Attack Damage** | $62.0$ | $+5.0$ |
| **Attack Speed** | $0.625$ | |
| **Movement Speed** | $340.0$ | $+0.0$ |
| **Attack Range** | $175.0$ | $+0.0$ |
| **Base Attack Speed** | $0.625$ | |
| **Attack Speed Ratio** | $0.625$ | |
| **Bonus AS per Level** | $2.0\%$ | |
| **Attack Windup** | $20.6\%$ | |
| **Acquisition Radius** | $400$ units | |
| **Gameplay Radius** | $80$ units | |
| **Pathing Radius** | $50$ units | |
| **Selection Radius** | $125$ units | |
| **Selection Height** | $190$ units | |
| **Critical Damage** | $175.0\%$ | |

## Pets

### Mist Walker

| Attribute | Value |
|-----------|------:|
| **Range** | 125 units |
| **Gold** | 2 |
| **Experience** | 0 |
| **Health** | 110 / 300 (+ 15% of **Yorick**’s **bonus** health) |
| **Armor** | 0 |
| **Magic Resist** | 0 |
| **Damage** | 15 / 100 (+ 20% of **Yorick**’s **bonus** AD) |
| **Damage Type** | Physical |
| **Attack Speed** | 0.54 / 0.9 attack speed |
| **Move Speed** | 325 (Higher when within er 1600 range of **Yorick**) |
| **Control** | Autonomous |
| **Targeting** | Minion, does not count towards creep score |
| **Spell Effects** | *Mist Walkers' * basic attacks apply spell effects as area damage. When leaping to *marked* enemies, their attacks are also tagged as basic damage. |
| **On-Hit** | *Mist Walkers' * attacks are mitigated by dodge, block, and blind. They also deal 60% damage against monsters and 50% damage against turrets. |

**Abilities:**

- **Endless March:** *Mist Walkers* remain near **Yorick** (preferably 200 units behind him) until they enter combat in a lane, whereupon they proceed along the lane and attack any enemy in their path.
- **Ravenous:** While in a 1500-radius of Mourning Mist’s mark, *Mist Walkers* benefit from the ability's (movement speed) **bonus** movement speed, and have the ability to leap to *marked* enemy champions and large monsters, dealing 100% of their AD upon arrival. While *Mist Walkers* are targeting a unit due to this effect, an exclamation mark indicator will be visible over their head. If *Mist Walkers* are attacking a jungle monster when Yorick casts Recall, they will continue attacking it until his channel completes, after which they withdraw to him, or they die.

While *Mist Walkers* are within a er 1600-radius of **Yorick**, they gain 60 movement speed and benefit from **Yorick**’s flat movement speed bonuses.
Phase Rush is a percentual exception.
Summons gain **bonus** movement speed when returning to **Yorick**. This movement speed bonus is based on the distance between **Yorick** and the summon.
- **Unholy Covenant:** Unlike other pets, *Mist Walkers* are not prioritized by turrets over Blue Siege Minion, though they are prioritized over the *Maiden of the Mist*.
- **Uniform Leveling:** *Mist Walkers* will level up with **Yorick**.
- **Intercontinental Kids:** At the end of **Yorick**’s teleport, the *Mist Walkers* blink to teleport's target location.

---

### Maiden of the Mist

| Attribute | Value |
|-----------|------:|
| **Range** | 600 units |
| **Gold** | 50 |
| **Experience** | 0 |
| **Health** | 1050–then +250*x for 2@6–18 (+ 60% of **Yorick**’s **bonus** health) |
| **Armor** | 30–90@6–18 |
| **Magic Resist** | 30–90@6–18 |
| **Damage** | 50 / 75 / 100 (+ 30% of **Yorick**’s **bonus** AD) |
| **Damage Type** | Magic |
| **Attack Speed** | $1.0$ attack speed |
| **Move Speed** | 300 (Higher when near (1200 range) **Yorick**) |
| **Control** | Autonomous |
| **Targeting** | Minion |
| **Spell Effects** | The *Maiden of the Mist*’s basic attacks apply spell effects as area damage. |
| **On-Hit** | The *Maiden of the Mist*’s attacks are mitigated by dodge and blind, but not mitigated by block. They also deal 50% damage against turrets. |

**Abilities:**

- **Natural Recovery:** The *Maiden of the Mist* regenerates $2.5$ health per second.
- **Endless March:** The *Maiden of the Mist* is permanently ghosted and follows **Yorick** (preferably idle within 100 units of the location 500 units behind him), leashing back to him if he moves 1200 (Estimated, pending for test) units away from her.
- **Touch of the Maiden:** The *Maiden of the Mist*’s attacks mark an enemy champion with a debuff, which is lost from the current target whenever *Yorick* commands her to a new target or her acquisition of that current target otherwise ends. **Yorick**’s next basic attack, Q or E hit against a marked unit consumes it to deal 2 / 2.5 / 3% of target's **maximum** health **bonus** magic damage, capped at 30 against monsters.
- **Undead Army:** While there are fewer than 4 *Mist Walkers* under the *Maiden of the Mist*’s command, she raises another *Mist Walker* each time a nearby enemy dies.
- **Ravenous:** While nearby *Mourning Mist’s* mark, the *Maiden of the Mist* benefits from the ability's **bonus** movement speed, and actively seeks to assist **Yorick** if he is nearby and in combat with an enemy champion.

While the *Maiden of the Mist* is within a er 2000-radius of **Yorick**, she gains 60 movement speed and benefits from **Yorick**’s flat movement speed bonuses.
- **Unholy Covenant:** Unlike other pets, the *Maiden of the Mist* is not prioritized by turrets over any type of minion.
- **Uniform Leveling:** The *Maiden of the Mist* will level up with **Yorick**.
- **Intercontinental Wife:** At the end of **Yorick**’s teleport, the *Maiden of the Mist* blinks to teleport's target location.

---

## Abilities

### Passive: Shepherd of Souls

| Attribute | Value |
|-----------|------:|
| **Effect Radius** | 1000 (Grave summon range) / 2500 (Grave despawn range, pending for test) units |
| **Targeting** | Passive |
| **Affects** | Self |

**INNATE - FINAL SERVICE:** Whenever **Yorick** uses *Last Rites* to score the killing blow on any enemy or damage an enemy champion or large monster, he raises a *grave* at their location (Grave is spawned on top of the killed enemy and within close proximity of the damaged champion or monster). Additionally, a grave is passively raised for every 8th@1; 7th@3; 6th@5; 5th@7; 4th@9; 3rd@11; 2nd@13 enemy that dies otherwise near him. Up to 4 *graves* can exist at once, lasting until **Yorick** exits the general area or dies, with the furthest one always being replaced by the newest upon raising one over the limit.

**Yorick** can use *Awakening* and *Mourning Mist’s* mark to raise 1 Mist Walker from each *grave* after a 1-second delay, consuming the *graves* in the process.

**INNATE - THE CURSED HORDE:** Up to 4 Mist Walkers can be active at once near **Yorick** or near **Maiden of the Mist**, with the nearest one always being replaced by the newest upon raising one over the limit.

*See [Pets](#Pets) for more details about Mist Walkers.*

**Notes:**

- Damaging a large monster or champion with Last Rites will only spawn 1 grave, even if it kills them.

---

### Q: Awakening

| Attribute | Value |
|-----------|------:|
| **Range** | 1000 units |
| **Cast Time** | none |
| **Targeting** | Proximity |
| **Affects** | Self |

**ACTIVE:** **Yorick** exhumes a Mist Walker under his command from each nearby *grave*.

**Notes:**

- *Awakening* counts as an ability activation for the purposes of on-cast effects such as Spellblade and triggering Force Pulse’s passive.
- *Mist Walkers* update their statistics correspondingly to **Yorick's ** during the periodic stat update, with the exception of their maximum health.
- *Awakening* cannot be cast without enough *graves* in **range**.

---

### Q: Last Rites

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Cost** | 20 Mana |
| **Cooldown** | 6 / 5.5 / 5 / 4.5 / 4 seconds |
| **Cooldown Start** | post-effect |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | False |
| **Spell Effects** | spell |
| **Parry** | True |

**ACTIVE:** **Yorick** empowers his next basic attack within 5 seconds to have an uncancellable windup, gain (range) 50 **bonus** range, deal **bonus** physical damage, and heal him. The heal is reduced by 50% against non-champions.

| Attribute | Value |
|-----------|------:|
| **Bonus Physical Damage** | 30 / 50 / 70 / 90 / 110 (+ 50% AD) |

| Attribute | Value |
|-----------|------:|
| **Heal** | 10 to 20 for 6 / 23 to 38 for 6 / 43 to 68 (+ 6 / 7 / 8 / 9 / 10% of his **missing** health) |
| **Non-Champion Heal** | 10×0.5 to 20×0.5 for 6 / 23×0.5 to 38×0.5 for 6 / 43×0.5 to 68×0.5 (+ 3 / 3.5 / 4 / 4.5 / 5% of his **missing** health) |

While there are at least 3 *graves* nearby and *Last Rites* is active or on cooldown, **Yorick** can cast *Awakening*.

*Last Rites resets **Yorick**’s basic attack timer.*

**Notes:**

- No additional notes.

---

### W: Dark Procession

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 600 units |
| **Effect Radius** | 225 (Spirits spawn radius) / 200 (Sight radius from center of ring) units |
| **Cost** | 70 Mana |
| **Cooldown** | 20 / 18 / 16 / 14 / 12 seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Spell Shield** | False |
| **Out of Range Behavior** | walk to location |
| **Impassable perimeter** | pathing radius 125 (Inner radius) to 325 (Outer radius) |

**ACTIVE:** **Yorick** summons a of spirits around the target location that arises after a $0.75$-second delay, knocking aside enemies hit by the walls over $0.25$ seconds, though not through terrain. Within 210-units (Estimated), they are pulled inside; otherwise they are pushed away. The remains for 4 seconds as impassible terrain against enemies, granting sight inside the ring.

*Dark Procession* can only be targeted by enemy champion basic attacks and turrets. The wall takes 1 damage per attack from champions and is destroyed instantly from turret attacks.

| Attribute | Value |
|-----------|------:|
| **Wall Health** | 2 / 2 / 3 / 3 / 4 |

**Notes:**

- *Dark Procession* has a turret prioritization value of 6, equal to that of Blue Super Minion.
  - **Yorick**’s other pets each have a lower prioritization value, allowing him to delay the turret from targeting the The Maiden or a Mist Walker for one attack.
- *Dark Procession* deals 0 proc true damage to enemies it knocks aside, which triggers in-combat effects such as drawing turret and monster aggression, Sudden Impact or applying Elixir of Sorcery.
- The perimeter is made up of 18 units with 100 pathing radius and 20 gameplay radius each.
- **Yorick** marks any enemy champion within repeatedly while the cage persists in order to gain assist credit, lasting for the standard credit timer.

---

### E: Mourning Mist

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 700 units |
| **Effect Radius** | 1500 (Bonus movement speed range) units |
| **Cost** | 50 / 55 / 60 / 65 / 70 Mana |
| **Cooldown** | 12 / 11 / 10 / 9 / 8 seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | True |
| **Out of Range Behavior** | cast at max |

**ACTIVE:** **Yorick** hurls a globule of Black Mist that splashes across an area at the target location, dealing magic damage to enemies hit, down to a minimum threshold against minions, and capped against monsters. Enemy champions and monsters hit are slowed by 30% for $1.5$ seconds and marked as *Cursed* for 4 seconds.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 6 / 6.5 / 7 / 7.5 / 8% (+ 3% per 100 AP) of target's **maximum** health |

| Attribute | Value |
|-----------|------:|
| **Minimum Minion Damage** | 70 / 105 / 140 / 175 / 210 (+ 100% AP) |

| Attribute | Value |
|-----------|------:|
| **Capped Monster Damage** | 50 / 75 / 100 / 125 / 150 (+ 100% AP) |

**CURSED:** The target is revealed, has (armor penetration) reduced armor, and continually raises a Mist Walker from each nearby *grave*, up to a maximum of 4.


**Yorick**, Mist Walkers and the **Maiden of the Mist** gain (ms) **bonus** movement speed while facing nearby *Cursed* enemies. Mist Walkers will leap to nearby *Cursed* enemy champions and large monsters, attacking them upon arrival.

| Attribute | Value |
|-----------|------:|
| **Armor Reduction** | 13 / 16 / 19 / 22 / 25% of target's armor |

| Attribute | Value |
|-----------|------:|
| **Bonus Movement Speed** | 18 / 21 / 24 / 27 / 30% |

**Notes:**

- **Yorick** will snap his facing direction towards the target location at the start of the cast time.
- The speed boost toward marked targetes does not require sight of them.
- The Mist Walkers do not leap if they are immobilized or grounded, or if the *marked* target is untargetable.

---

### R: Eulogy of the Isles

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.5$ seconds |
| **Target Range** | 600 (Maximum summon range) units |
| **Cost** | 100 Mana |
| **Cooldown** | 160 / 145 / 130 / 115 / 100 (Starts after Yorick or the Maiden of the Mist dies) seconds |
| **Cooldown Start** | Special |
| **Targeting** | Location |
| **Affects** | Self |
| **Out of Range Behavior** | cast at max |

**ACTIVE:** **Yorick** summons the **Maiden of the Mist** over $1.5$ seconds (From the start of the cast time; summons become alive 1 second after cast.), along with a number of Mist Walkers surrounding her at the target location, which remain on the battlefield until she or **Yorick** dies. *Eulogy of the Isles* can be recast once while the *Maiden* is alive after 10 seconds of the activation.

| Attribute | Value |
|-----------|------:|
| **Mist Walkers** | 2 / 3 / 4 |

**RECAST:** **Yorick** sets the *Maiden* free, allowing her to push the nearest lane until she dies.

*See [Pets](#Pets) for more details about The **Maiden of the Mist**.*

**Notes:**

- See the *Pets* section below for more details about *The Maiden of the Mist* and Mist Walkers.
- The extra Mist Walkers spawn 250 units from the *Maiden* summon location.
- The *Maiden* updates her stats when **Yorick**’s stats change, and as *Eulogy of the Isles* is ranked up.
- When **Yorick** dies, *The Maiden* rapidly loses portions of her **maximum** health over 3 seconds until she dies.
  - The loss of **maximum** health occurs in $0.6$-second intervals, in which she loses 20% on the first, 25% on the second, $33.3$% on the third, 50% on the fourth, and 100% on the fifth.
- **Yorick** will snap his facing direction towards the target location at the start of the cast time.

---

## Patch History

### V25.13
- Stats
  - Base armor reduced to 36 from 39.

### V25.12
- Shepherd of Souls
  - Mist Walker bonus attack speed changed to 8% / 80% **bonus** attack speed from 100% of Yorick's **bonus** attack speed.
- Last Rites
  - Base damage reduced to 30 / 50 / 70 / 90 / 110 from 30 / 55 / 80 / 105 / 130.

### V25.11
- Dark Procession
  - **Undocumented:** The no longer follows Yorick into Mordekaiser’s Realm of Death.

### V25.10
- Shepherd of Souls
  - **Bug Fixes:** Mist Walkers no longer deal an inconsistent amount of damage to monsters.

### V25.09
- Shepherd of Souls
  - Mist Walker monster damage reduced to 60% from 70%.
- Dark Procession
  - **UNDOCUMENTED / NEW EFFECT:** The now follows Yorick into Mordekaiser’s Realm of Death.
- Eulogy of the Isles
  - **UNDOCUMENTED / NEW EFFECT:** The Maiden now follows Yorick into Mordekaiser’s Realm of Death.

### V25.08
- Shepherd of Souls
  - Mist Walker monster damage increased to 70% from 60%.
  - Mist Walker incoming non-epic monster damage modifier increased to 100% from 60%.
  - **Bug Fixes:** Mist Walkers are now properly able to leap to targets beyond terrain when commanded via Mourning Mist.
- Mourning Mist
  - **Removed:*** No longer deals 70 / 105 / 140 / 175 / 210 (+ 70% AP) base damage.
  - **New Effect:** Now deals 6 / 6.5 / 7 / 7.5 / 8% (+ 3% per 100 AP) of the target's **maximum** health damage. This is capped at 50 / 75 / 100 / 125 / 150 (+ 100% AP) against monsters and has a minimum damage of 70 / 105 / 140 / 175 / 210 (+ 100% AP) against minions.
- Eulogy of the Isles
  - **Removed:*** The Maiden can no longer mark non-champion units.

### V25.07#April 4th Hotfix|V25.07
- Shepherd of Souls
  - Mist Walker incoming minion damage modifier increased to 60% from 40%.
  - Mist Walker incoming non-epic monster damage modifier increased to 60% from 40%.
  - **Bug Fixes:** Mist Walkers' leap attack now correctly deals 60% damage against monsters when commanded via Mourning Mist.
- Mourning Mist
  - Slow duration reduced to $1.5$ seconds from 2.
  - Bonus movement speed reduced to 18 / 21 / 24 / 27 / 30% from 30% at all ranks.
  - Armor reduction reduced to 13 / 16 / 19 / 22 / 25% from 18 / 21 / 24 / 27 / 30%.
- Eulogy of the Isles
  - Maiden mark monster damage cap reduced to 30 from 50.

### V25.07#April 2nd Hotfix|V25.07
- Shepherd of Souls
  - Mist Walker monster damage reduced to 60% from 100%.
  - Mist Walker base attack damage increased to 15 / 100 from 15 / 75.
- Last Rites
  - Heal health ratio increased to 6 / 7 / 8 / 9 / 10% **missing** health from 4 / 5 / 6 / 7 / 8%.

### V25.07
- Stats
  - Armor growth reduced to $4.5$ from $5.2$.
- Shepherd of Souls
  - Nearby deaths per grave reduced to 8@1; 7@3; 6@5; 5@7; 4@9; 3@11; 2@13 from 12@1; 6@7; 2@13.
    - Changed to every 5th on ARAM.
  - Mist Walker base attack damage changed to 15 / 75 from 4 / 5 / 6 / 7 / 8 / 9 / 10 / 15 / 20 / 25 / 30 / 35 / 40 / 50 / 60 / 70 / 80 / 901 per level up to 7, +5 per level up to 13, +10 per level up to 18. *Now uses stat growth.*
  - Mist Walker AD ratio reduced to 20% of Yorick's **bonus** AD from 20% of Yorick's **total** AD.
  - Mist Walker base health increased to 110 / 300 from 110 to 212. *Now uses stat growth.*
  - Mist Walker health ratio reduced to 15% of Yorick's **bonus** health from 20% of Yorick's **maximum** health.
  - Mist Walker base attack speed reduced to $0.5$ at all levels from 0.5+0.5×0.08×1 to 0.5+0.5×0.08×18. *Attack speed ratio unchanged at $0.5$.*
  - **New Effect:** Mist Walker attack speed now scales with 100% of Yorick's **bonus** attack speed.
  - Mist Walker incoming non-epic monster damage modifier reduced to 40% from 50%.
    - **Removed:*** Mist Walkers no longer receive 50% damage from **epic** monsters.
  - **New Effect:** Mist Walkers now receive 40% damage from lane minions.
  - **Removed:*** Mist Walkers are no longer instantly killed by single-target champion spells. *Still instantly killed by turret attacks.*
  - **New Effect:** Mist Walkers now receive 200% damage from melee champion basic attacks.
  - Mist Walker incoming area damage modifier changed to 66% / 64% / 62% / 60% / 58% / 56% / 54% / 52% / 50% / 48% / 46% / 44% / 42% / 40% from 50% at all levels.
  - **New Effect:** Pinging the ability now displays in chat the number of Mist Walkers alive near (Within 1600 units) **Yorick** and overall on the map.
  - **New Effect:** When Yorick casts Recall, Mist Walkers no longer stop attacking jungle monsters and withdraw to him.
  - **Bug Fixes:** Mist Walker leaps via Mourning Mist now target units instead of locations. Previously this caused them to sometimes fail to reach their target over terrain.
  - **Bug Fixes:** Attacks from different Mist Walkers that occur simultaneously or in quick succession due to regular commands and the leap granted by Mourning Mist now correctly apply as separate instances for the purposes of Black Cleaver Carve.
  - **Bug Fixes:** Mist Walkers now properly update their stats on Yorick's level-up.
- Last Rites
  - AD ratio increased to 50% AD from 40% AD.
  - Cooldown reduced to 6 / 5.5 / 5 / 4.5 / 4 seconds from 7 / 6.25 / 5.5 / 4.75 / 4.
  - **New Effect:** Now also raises a grave if it damages a champion or large monster.
    - Killing a target with the spell will only raise 1 grave.
- Mourning Mist
  - **Removed:*** Globule no longer deals 15% of the target's **current** health.
  - **New Effect:** Globule now deals 70 / 105 / 140 / 175 / 210 (+ 100% AP) base damage.
    - Globule AP ratio increased to 100% AP from 70% AP.
    - **Removed:*** Globule no longer has a *minimum* damage of 70 / 105 / 140 / 175 / 210 (+ 70% AP).
  - **Removed:*** Globule no longer has a monster damage cap of 70 / 105 / 140 / 175 / 210.
  - **Removed:*** While the mark is active, Mist Walkers no longer deal 20% **bonus** damage for the next 8 attacks against the target.
  - **New Effect:** While the mark is active, now reduces the target's armor by 18 / 21 / 24 / 27 / 30%.
  - Bonus movement speed increased to 30% from 20%.
- Eulogy of the Isles
  - Maiden base attack damage increased to 50 / 75 / 100 from 0 / 10 / 40.
  - Maiden AD ratio reduced to 30% of Yorick's **bonus** AD from 50% of Yorick's **total** AD.
  - Maiden base health increased to 1050–then +250*x for 2@6–18 from 400–1650@6–18.
  - Maiden health ratio reduced to 60% of Yorick's **bonus** health from 60% of Yorick's **maximum** health.

### V25.06
- Mourning Mist
  - Monster damage cap reduced to 70 / 105 / 140 / 175 / 210 from 300 at all ranks.

## Trivia

- Yorick is named after Yorick from Hamlet by William Shakespeare.
  - His surname *Mori* references the Latin phrase memento mori "be mindful of dying", befitting his association with death and lost souls.
- His current weapon is based on the Chinese monk's spade.
- Internally, Yorick's summons are called "ghouls", which was their name in his previous version. The Maiden of the Mist is referred to as "big ghoul".
- Due to David 'Phreak' Turley being delayed on a flight back from DreamHack (Sweden) Yorick became the first champion to be released before his 'Champion Spotlight'.
- Yorick's ability names are references to the Four Horsemen of the Apocalypse.

---
*This page was automatically generated from League of Legends Wiki data.*