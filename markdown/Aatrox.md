# Aatrox

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
| **Champion** | Aatrox |
| **Title** | the Darkin Blade |
| **Resource** | Blood Well |
| **Range Type** | Melee |
| **Release Date** | 2013-06-13 |
| **Release Patch** | V3.8 |
| **Roles** | Juggernaut |
| **Riot Positions** | Top |
| **External Positions** | Top |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $650.0$ | $+114.0$ | $2588.0$ |
| **Mana** | $0.0$ | $+0.0$ | $0.0$ |
| **Health Regen** | $3.0$ | $+0.5$ | $11.5$ |
| **Armor** | $38.0$ | $+4.8$ | $119.6$ |
| **Magic Resist** | $32.0$ | $+2.05$ | $66.8$ |
| **Attack Damage** | $60.0$ | $+5.0$ | $145.0$ |
| **Attack Speed** | $0.651$ | $+2.5\%$ | $0.928$ |
| **Movement Speed** | $345.0$ | $+0.0$ | $345.0$ |
| **Attack Range** | $175.0$ | $+0.0$ | $175.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.651$ |
| **Attack Speed Ratio** | $0.651$ |
| **Bonus AS per Level** | $2.5\%$ |
| **Attack Windup** | $19.7\%$ |
| **Acquisition Radius** | $475 units$ |
| **Pathing Radius** | $35 units$ |
| **Selection Radius** | $135 units$ |
| **Selection Height** | $180 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Deathbringer Stance

**Innate:** Periodically, **Aatrox**’s next basic attack deals **bonus** magic damage based on the target's **maximum** health, him for the damage dealt.

*''Aatrox's* basic attacks and ability hits against enemy champions or large monsters ah *Deathbringer Stance's cd.*

**Innate:** Periodically, **Aatrox** empowers his next basic attack to gain range*bonus** range* and deal **bonus** magic damage equal to key=% of the target's **maximum** health, capped at 100 against monsters. 'Deathbringer Stance's damage applies at 100% effectiveness. **Aatrox** heals for $% of the post-mitigation **bonus** damage dealt, reduced to $*100% against minions. Whenever **Aatrox** hits at least one enemy champion or large monster with a basic attack on-hit or an ability, 'Deathbringer Stance's* **current cooldown** is reduced by 2 seconds, doubled to 4 if he hits with the Sweetspot of **The Darkin Blade*'.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Self, Enemies |
| **Damage Type** | magic |
| **Spell Shield** | False |
| **Spell Effects** | proc |

**Notes:**

- **Aatrox** will assume stance when an enemy champion is in range of *Infernal Chains*.
  - The toggle expression (default **CTRL + 5**) switches between stances manually.
- If *Deathbringer Stance* becomes available during a standard attack's windup, it will not be consumed or trigger the **bonus** damage.
- Even if the ability hit is spell shield 'Deathbringer Stance's cooldown will still be reduced.
- This ability goes on cooldown on death and refreshes upon respawn.
- The empowered attack will not trigger against structures and wards.

---

### Q: The Darkin Blade

**Aatrox** can cast this ability up to three times in a short amount of time before it goes on *cooldown*.

**Active:** **Aatrox** performs an attack with his greatsword in the target direction, dealing physical damage to enemies within an area. The area varies and the damage increases per cast. Enemies within each area's Sweetspot receive increased damage and are briefly airborne. * **First Cast:** **Aatrox** strikes his greatsword in a long rectangle. * **Second Cast:** **Aatrox** slashes his greatsword in a wide trapezoid. * **Third Cast:** **Aatrox** slams his greatsword in a circle.

**Aatrox** can activate *The Darkin Blade* three times before the ability goes on cooldown, with a 1-second static cooldown between casts. If **Aatrox** does not recast the ability within 4 seconds of the previous cast, it goes on cooldown. **Active:** **Aatrox** performs a strike with his greatsword for each of the three casts, dealing physical damage to enemies hit within an area. Enemies hit within a Sweetspot of the area take 70% **bonus** damage and also airborne for $0.25$ seconds. Each subsequent cast gains $% more damage. **First Cast:** ''Aatrox's' first strike affects a 625×180-unit rectangular area in the target direction, with him centered on the back line and the Sweetspot at the farthest edge. **Second Cast:** ''Aatrox's** second strike affects a trapezoidal area in the target direction, with the Sweetspot at the farthest edge. The hitbox begins 100-units behind **Aatrox'' and extends 475-units in front of him, measuring between 300 and 500-units wide from behind to in front. **Third Cast:** ''Aatrox's' third strike affects a 300-radius circular area centered on a target location that is 200 units in front of him, with a 180-radius Sweetspot within.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $14-6$ seconds |
| **Cast Time** | $0.6$ seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |

**Scaling:**
- **First Cast Damage:* $ (+ $% AD) (+ $*% AD)

**Notes:**

- The first and second casts function like any point-blank AOE (i.e. an effect centered on the caster). An enemy is considered to be hit by the ability based on er - i.e. if any part of your gameplay radius is within the hitbox, you are affected.
- The third cast, as well as the Sweetspot for the first and second cast, function like ground-targeted abilities. An enemy is considered to be hit by the ability based on cr - i.e. an enemy's center has to be within the hitbox to be affected.
- As implied by the previous point, the Sweetspot for the first and second cast is implemented as separate areas of effect to the main component of the ability. Enemies must be within both areas to trigger the bonus damage, airborne and *Deathbringer Stance*’s cooldown reduction.
  - The 'target gets hit' SFX plays whenever an enemy is within the Sweetspot - meaning it is possible to trigger the sound effect without affecting an enemy.
- In the game, Sweetspot damage is incorrectly displayed as a real critical strike.
- Each cast counts as an ability activation for the purposes of on-cast effects such as Spellblade and triggering *Force Pulse*’s passive.
- The hitbox and ''Aatrox's' model are fixed to the initial target direction.
  - ''Aatrox's' facing-direction, for effects such as *Petrifying Gaze*, is the direction he is moving, and not the direction the model is facing.
- All damage modifiers stacks multiplicatively.
- There's a small period of time in which Aatrox can't declare basic attacks after casting *The Darkin Blade*.
- This ability's damage is calculated based on the caster's current stats and changes dynamically. Effect at cast time end

---

### W: Infernal Chains

**Active:** **Aatrox** sends a chain in the target direction that deals physical damage and slow the first enemy hit.

*If this hits an enemy champion or large monster, it creates a true sight tether between them and the ground. If the tether is not broken after a brief moment, they are dealt physical damage and airborne to the center.*

**Active:** **Aatrox** sends a chain in the target direction that deals physical damage to the first enemy hit, doubled against minions, and slow them for $1.5$ seconds. If this hits an enemy champion or large monster, a tether is formed between the target and the ground beneath them for $1.5$ seconds, during which they are true sight. If the tether is not broken by the end of its duration, the target is dealt the same physical damage again and airborne to the center of the area.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $20-12$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Speed** | 1800 units/second |
| **Spell Shield** | Special |
| **Spell Effects** | spell |
| **Projectile** | True |

**Scaling:**
- **Physical Damage:* $30-70$ (+ 40% AD)2-70×2$ (+ 80% AD) **Slow:** $25-35$%
- **Total Damage:** $30×2-70×2$ (+ 80% AD)

**Notes:**

- The impact area is oriented relative to ''Aatrox's' position when the projectile hits, not where the projectile originated from.
- The location that the target is dragged to is not at the target's original location, but slightly closer towards ''Aatrox's' position when the zone expires.
- Spell shield will block the chain's application and initial damage but not the aftereffects of one already applied.
- This ability's damage is calculated based on the caster's current stats and changes dynamically. Effect at cast time end

---

### E: Umbral Dash

**Passive:** **Aatrox** for the portion of the damage he deals to champions.

**Active:** **Aatrox** dash in the target direction.

**Passive:** **Aatrox** heals for 16% (+ $1.1$% per 100 **bonus** health) of non-persistent damage post-mitigation damage he deals against enemy champions. **Active:** **Aatrox** dash in the target direction. *Umbral Dash basic attack reset *'Aatrox's* basic attack timer and can be cast during his other abilities without cancelling them and vice versa.*

| Attribute | Value |
|-----------|-------|
| **Range** | 75 / 300 / 500 units |
| **Cooldown** | $9-5$ seconds |
| **Cast Time** | none |
| **Targeting** | Location |
| **Affects** | Self |
| **Speed** | 800 / Up to 1340 units/second |

**Notes:**

- The dash distance can be extended to up-to 500 units when targeting across terrain. It will only extend for the distance needed to cross the wall.
  - This may upscale his dash speed so that the dash takes the same total time as the standard maximum-range dash.
- If *Umbral Dash* is cast towards a movement command's end point, *Umbral Dash* will re-issue a movement command to that point when the dash ends.
  - He will be unable to buffer any commands during the dash if it is casted this way.
- ''Aatrox's* model darkens for $1.5$ seconds upon casting *Umbral Dash', which is a remnant of an attack damage buff he received before it got removed in V9.9.

---

### R: World Ender

**Active:** **Aatrox** unleashes his true form for a period, gaining a burst of *move speed*. During this time, **Aatrox** has increased , , and is ghosted.

*These effects are refreshed and the duration is extended whenever **Aatrox** scores a champion takedown.*

**Active:** **Aatrox** unleashes his true form for 10 seconds, fearing nearby enemy minions and monsters for 3 seconds, during which they are gradually slow by up to 99% over the duration. He also gains **bonus movement speed** that decays by 10% of the **current bonus** every $0.25$ seconds, lasting until *World Ender* has ended. Whenever **Aatrox** scores a champion takedown, he extends the duration by 5 seconds, up to its original length, and becomes unleashed again. During *World Ender*, **Aatrox** gains , has 5% increased size, is ghosted, and receives increased heal from all sources.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $120-80$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Effect Radius** | 600 units |

**Scaling:**
- **Bonus Movement Speed:** $60-100$%
- **Bonus Attack Damage:** $20-40$% AD
- **Increased Healing:** $50-100$%

**Notes:**

- His resource bar indicates the remaining duration of *World Ender*.
- While *World Ender* is active, the screen will have a red tint. Near the end of the duration, the tint is intensified.
- *World Ender* persists through resurrection effects.
  - 'World Ender's healing amplification applies to the health gained by resurrecting.
- *World Ender* will cast from wherever **Aatrox** is at the end of the cast time.
- Casting *World Ender* while it is still active removes the timer on the resource bar.
  - If this happens while using Aatrox, the intensified tint effect of *World Ender* will stay on the screen for the rest of the game.

---

## Patch History

### V25.12
- *Deathbringer Stance*
  - Heal increased to 100% of post-mitigation damage from 80%.
  - Minion heal increased to 25% of post-mitigation damage from 20%.
- *Umbral Dash*
  - Heal health ratio increased to $1.1$% per 100 **bonus** health from $0.9$%.

### V25.11
- *Deathbringer Stance*
  - Damage health ratio reduced to 4 to 8 of the target's **maximum** health from 4 to 12.
- *The Darkin Blade*
  - Sweetspot bonus damage increased to 70% from 60%.
    - First cast sweetspot base damage increased to $10×1.7-70×1.7$ from $10×1.6-70×1.6$.
      - Second cast sweetspot base damage increased to $10×1.7×1.25-70×1.7×1.25$ from $10×1.6×1.25-70×1.6×1.25$.
      - Third cast sweetspot base damage increased to $10×1.7×1.5-70×1.7×1.5$ from $10×1.6×1.5-70×1.6×1.5$.
      - Maximum base damage increased to $10×6.375-70×6.375$ from $10×6-70×6$.
    - First cast sweetspot AD ratio increased to $60×1.7-100×1.7$% AD from $60×1.6-100×1.6$% AD.
      - Second cast sweetspot AD ratio increased to $60×1.7×1.25-100×1.7×1.25$% AD from $60×1.6×1.25-100×1.6×1.25$% AD.
      - Third cast sweetspot AD ratio increased to $60×1.7×1.5-100×1.7×1.5$% AD from $60×1.6×1.5-100×1.6×1.5$% AD.
      - Maximum AD ratio increased to $60×6.375-100×6.375$% AD from $60×6-100×6$% AD.
- *Infernal Chains*
  - Damage type changed to physical from magic.

### V25.10
- *Infernal Chains*
  - **Bug Fixes:** Now once again has the proper tooltip description for the debuff after a tooltip change to **Aurora** in patch 25.09 unintentionally affected it.

### V25.04
- *Infernal Chains*
  - **Bug Fixes:** Corrected tether visual.
- *The Darkin Blade*
  - **Undocumented / Bug Fix:** Minion damage modifier now properly applies to non-*Sweetspot* hits.
  - **Undocumented:** In the floating text display, now once again uses the incorrect physical critical strike icon instead of the proper enhanced physical damage icon when enemies are hit by a *Sweetspot*.
- *The Darkin Blade*
  - **Bug Fixes:** In the floating text display, now uses the proper enhanced physical damage icon when enemies are hit by a *Sweetspot*, instead of the physical critical strike icon.

### V14.23
- *Deathbringer Stance*
  - **Bug Fixes:** Attack can now trigger both this ability's damage and any and on-hit effects when the attack kills the target. 'Deathbringer Stance's damage is no longer discarded, previously causing **Aatrox** to lose the healing.

### V14.22
- *World Ender*
  - Bonus attack damage reduced to $20-40 3$% AD from $20-45 3$% AD.

### V14.17
- *Umbral Dash*
  - **Bug Fixes:** Issued basic attacks no longer unintentionally cancel when dashing towards an enemy.

### V14.14
- *Deathbringer Stance*
  - **Bug Fixes:** Attack now triggers attack effects if damage originating from *Deathbringer Stance* would have killed the target.
    - 'The attack effects' damage now applies first/is prioritized, which may lead to losing the ability's healing.'

### V14.12
- Stats
  - Health regeneration growth reduced to $0.5$ from 1.
  - Armor growth increased to $4.8$ from $4.45$.
- *Umbral Dash*
  - Healing reduced to 16% at all ranks from $18-24$%.
  - **Removed:*** Healing is no longer increased to $20-36$% during *World Ender*.
  - **New Effect:** Healing now scales with $0.9$% per 100 **bonus** health.
- *World Ender*
  - Healing amplification increased to $50-100 3$% from $25-45 3$%.

### V14.9
- *Infernal Chains*
  - **Bug Fixes:** The timing of the damage is no longer unsynced with the tether. Previously, the target would still sometimes be dealt damage after properly breaking the tether.
- *Umbral Dash*
  - **Bug Fixes:** No longer references to his long-deprecated ammo system.

## Trivia

- Aatrox is the first champion, since **Lulu**’s patch introduced champion specific login screens, to receive two login screens and themes released for his Classic skin.
- In-game, Aatrox can toggle his banner-style wings and his sword stance [default: Ctrl + 5].
- This champion has no ability power ratio.
- The icon for *World Ender* is reused for the Teamfight Tactics item *Darkin*.
- Aatrox's passive, Deathbringer Stance, is a reference to Final Fantasy IV in both name and function. "Deathbringer" is the name of the last sword you get in the game as a Dark Knight and the icon and ability looks exactly how Cecil holds his sword using his "Darkness" ability.
- The Darkin Blade, Grandmaster-at-Arms, Night Hunter, and The Relentless Storm are the only abilities in *League of Legends* that have the same name as their champion's title.
- Aatrox's Series 1 Eternals make the following references:
  - *Chains Yanked* references the idiom yank one's chain as well as the pull of *Infernal Chains*.
  - 'QQ's references the QQ emoticon as well the multi-cast nature of *The Darkin Blade*.
- Out-of-universe, *Aatrox* is a word play on Latin *atrox* "fierce, savage, cruel" in turn from *ater* "dull black, dark", from root Proto-Indo-European languages **h₂eh₁ter-* "fire";
  - All of these reference the Darkin weapon's physical manifestation and demonic nature (punning on *dark kin*), various early thermal weapons, the scorched earth strategy, and his 'The Art of War' philosophy. Latin *atrox* gives to Anglo-French *atrocity.*
- Enemies hit by *Massacre* will leave a blood trail between themselves and Aatrox that he will rapidly absorb (this is a remaining visual effect from PBE testing, where Aatrox would gain attack speed based on how many enemies had been hit by *Massacre*, that was recycled in V5.6).
- Aatrox can still use emotes during *Blood Well*’s revive.
- *Sterak's Gage* is speculated to have been ''Aatrox's' missing left arm glove. Its passive was also coincidentally similar to Aatrox's.
- Prior to his rework, on the statement "This champion needs an update", players ranked Aatrox 5th in NA; 4th in BR; 3rd in KR and 14th in CN. He ranked 4th overall.

---
*This page was automatically generated from League of Legends Wiki data.*