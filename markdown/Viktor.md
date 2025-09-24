# Viktor

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
| **Champion** | Viktor |
| **Title** | the Herald of the Arcane |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2011-12-29 |
| **Release Patch** | V1.0.0.131 |
| **Latest Changes** | V25.17 |
| **Roles** | Battlemage |
| **Riot Positions** | Middle |
| **External Positions** | Middle |
| **Blue Essence** | 2400 |
| **Riot Points** | 880 |
| **Difficulty** | 3 |
| **Hero Type** | Mage |
| **Adaptive Type** | Magic |
| **Damage** | 3 |
| **Toughness** | 1 |
| **Control** | 2 |
| **Mobility** | 1 |
| **Utility** | 1 |
| **Style** | 90 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $600.0$ | $+100.0$ |
| **Mana** | $405.0$ | $+45.0$ |
| **Health Regen** | $8.0$ | $+0.65$ |
| **Mana Regen** | $8.0$ | $+0.8$ |
| **Armor** | $23.0$ | $+4.4$ |
| **Magic Resist** | $30.0$ | $+1.3$ |
| **Attack Damage** | $53.0$ | $+3.0$ |
| **Attack Speed** | $0.658$ | |
| **Movement Speed** | $335.0$ | $+0.0$ |
| **Attack Range** | $525.0$ | $+0.0$ |
| **Base Attack Speed** | $0.658$ | |
| **Attack Speed Ratio** | $0.658$ | |
| **Bonus AS per Level** | $2.1\%$ | |
| **Missile Speed** | $2300$ units/second | |
| **Acquisition Radius** | $525$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $120$ units | |
| **Selection Height** | $140$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $95.0\%$ |
| **Damage Taken** | $105.0\%$ |

#### URF

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $95.0\%$ |

## Abilities

### Passive: Glorious Evolution

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Self |

**INNATE:** **Viktor** can augment each of his abilities at the cost of 100 *Hex Fragments*, which he gains from killing enemies:
- Minions and monsters generate 1 *Hex Fragment*.
- Siege minions, super minions, and epic monsters generate 10 *Hex Fragments*.
- Enemy champion takedowns generate 20 *Hex Fragments*.

*Arcane Storm* can be augmented after all of **Viktor**’s basic abilities have been augmented.

**Notes:**

No additional notes.

---

### Q: Siphon Power

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 600 units |
| **Speed** | 2000 units/second |
| **Cost** | 45 / 50 / 55 / 60 / 65 mana |
| **Cooldown** | 9 / 8 / 7 / 6 / 5 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Unit |
| **Affects** | Enemies / Self |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spell |
| **Projectile** | Special |
| **Parry** | True |
| **Call For Help** | True |

**ACTIVE:** **Viktor** throws a draining device at the target enemy that deals magic damage. He also grants himself a shield for 40 to 115 (+ 18% AP) for $2.5$ seconds and gains *Discharge* for 4 seconds.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 60 / 75 / 90 / 105 / 120 (+ 40% AP) |

**DISCHARGE:** **Viktor**’s next basic attack is empowered to become non-projectile and deal ***modified** magic damage.

| Attribute | Value |
|-----------|------:|
| **Modified Magic Damage** | 20 / 45 / 70 / 95 / 120 (+ 100% AD) (+ 50% AP) |

| Attribute | Value |
|-----------|------:|
| **Total Magic Damage** | 80 / 120 / 160 / 200 / 240 (+ 100% AD) (+ 90% AP) |

** After the device hits, **Viktor** gains (ms) 30% **bonus** movement speed for $2.5$ seconds and the shield's strength is increased「 by 60%. ⟷ to 40×1.6 to 115×1.6 (+ 32% AP). 」

**Notes:**

- *Discharge* does not convert the bonus damage dealt by Spellblade to magic damage from physical.
- *Discharge* disables the **bonus** damage from critical strikes.
- Spell shield does not prevent **Viktor** from gaining *Discharge*.
- Only the initial projectile can be intercepted.
- The empowered attack will trigger against turrets but not be consumed nor apply its effects against wards.

---

### W: Gravity Field

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 800 units |
| **Effect Radius** | 340 units |
| **Cost** | 65 mana |
| **Cooldown** | 17 / 16 / 15 / 14 / 13 seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Spell Shield** | True |
| **Out of Range Behavior** | walk to location |

**ACTIVE:** **Viktor** deploys a gravity field at the target location for $4.5$ seconds. After a $0.75$-second delay, it activates to slow enemies within for 1 second, refreshing every $0.25$ seconds while they remain inside.

| Attribute | Value |
|-----------|------:|
| **Slow** | 33 / 36 / 39 / 42 / 45% |

The field applies a stack to enemies within every $0.25$ seconds, stacking up to 5 times. The fifth stack consumes them all to knock down and stun the target for $1.5$ seconds. The knockdown and stun can be triggered only once per cast per enemy.

** **Viktor**’s other abilities, excluding *Arcane Storm’s* persistent damage, slow enemies hit by 20% for 1 second.

**Notes:**

- *Gravity Field* applies a single debuff for both the slow and the stack.
  - Targets can be applied further stacks after having their stacks consumed for the first time.
    - Stacks are always consumed upon the fifth stack, but only the first instance of stacks being consumed can trigger the crowd control effects.
  - The debuff lasts for 1 second.
- *Gravity Field* will remain active if **Viktor** dies.
- *Gravity Field*’s animation can be seen by both teams through fog of war.
- If an enemy enters the area of effect while untargetable, they may generate stacks even after becoming stunned once, causing them to get stunned up-to 3 times by a single *Gravity Field*.

---

### E: Hextech Ray

| Attribute | Value |
|-----------|------:|
| **Range** | 500 (Missile range (both)) units |
| **Cast Time** | none |
| **Target Range** | 550 units |
| **Effect Radius** | 500 units |
| **Width** | 90 (Missile width (both)) units |
| **Speed** | 1050 (Laser missile speed) / 1500 (Aftershock missile speed) units/second |
| **Cost** | 60 / 70 / 80 / 90 / 100 mana |
| **Cooldown** | 12 / 11 / 10 / 9 / 8 seconds |
| **Targeting** | Vector |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | True |
| **Out of Range Behavior** | walk to location |

**ACTIVE:** **Viktor** fires an energy beam along the target path that deals magic damage to enemies hit and briefly grants sight of the area.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 70 / 110 / 150 / 190 / 230 (+ 50% AP) |

** The path explodes along the beam's wake after 1 second, dealing magic damage to enemies hit.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 20 / 50 / 80 / 110 / 140 (+ 80% AP) |
| **Total Magic Damage** | 90 / 160 / 230 / 300 / 370 (+ 130% AP) |

**Notes:**

- **Viktor** may freely act while the ray is traveling.
  - The effected area is set upon casting, no matter how far away **Viktor** moves. Unlike a leash, *Hextech Ray* will not break off its own effect.
- The laser grants a moderate area of sight around where the beam strikes the ground.
- If *Hextech Ray* is intercepted, the initial ray is not destroyed.
  - However, the capability to be blocked only applies to the laser as it hits the ground. **Viktor** can cast the ability *over* the wall and it will not be destroyed or intercepted.
  - *Augmented Hextech Ray* will still cause the entire target area to explode even if the laser is blocked.
- If **Viktor** dies while he is firing the beam it only deals damage up to the location at the time of his death.

---

### R: Arcane Storm

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 700 units |
| **Effect Radius** | 325 (Storm radius) units |
| **Speed** | 200 to 300 / 200×1.25 to 300×1.25 units/second |
| **Cost** | 100 mana |
| **Cooldown** | 120 / 110 / 100 / 90 / 80 seconds |
| **Cooldown Start** | post-effect |
| **Targeting** | Location / Unit |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Out of Range Behavior** | Walk in range of the target location to cast (first cast) |
| **Perfect Storm Effect Radius** | 325 to 1105 |
| **Sight radius** | 700 (If the storm radius is below 700 units) / Storm radius (If the storm radius is above 700 units) |

**ACTIVE:** **Viktor** conjures an arcane singularity at the target location, dealing magic damage to enemies within the area and disrupting their channeled abilities.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 100 / 137.5 / 175 / 212.5 / 250 (+ 50% AP) |

The singularity then remains active for $6.5$ seconds, dealing magic damage to enemies within it every second and granting sight of the area. The singularity targets the nearest champion hit by its initial damage and will follow them. The singularity moves faster based on its proximity to **Viktor**.

| Attribute | Value |
|-----------|------:|
| **Magic Damage Per Tick** | 65 / 85 / 105 / 125 / 145 (+ 35% AP) |
| **Total Magic Damage** | 490 / 647.5 / 805 / 962.5 / 1120 (+ 260% AP) |

*Arcane Storm* can be recast at any time while the singularity is active.

**RECAST:** **Viktor** directs the singularity to the target location, or to follow the target enemy champion or himself.

** The singularity moves 25% faster. Additionally, whenever an enemy champion is taken down after being damaged by *Arcane Storm*, the singularity increases in by 40% and extends its duration by 3 seconds. The singularity radius can be increased up to 6 times and its duration can be extended only up to the original duration.

*Arcane Storm's recast can be used while affected by cast-inhibiting crowd control.*

**Notes:**

- The disrupt is 'wrapped' into a status effect that says the target is Silenced for $0.25$ seconds, but it does not actually *silence*. It however makes sure that the *disrupt* is prevented by immunity to silences.
- **Viktor** can command the singularity regardless of how far away he is from it.
- The singularity will remain active even if **Viktor** dies and will automatically follow the nearest enemy champion in range at the time of his death, if it wasn't locked on already.
  - The singularity will only be issued to automatically follow a nearby champion once; it will not lock on to a new target if its current target becomes invalid.
  - Any commands it was issued prior to his death will not be interrupted.
- The base duration allows for 7 strikes (including the initial one).
- The storm will follow its target even if they are not visible, effectively giving away their position.
- *Arcane Storm* is not obstructed by terrain.
- After becoming augmented, an alternate icon is used for the recast of *Arcane Storm*.
- The champion that *Arcane Storm* is following will have an overhead indicator.

---

## Patch History

### V25.17
- Siphon Power
  - **Bug Fixes:** Tooltip now notes the correct buff duration for Discharge's attack of 4 seconds instead of $3.5$.

### V25.15
- Siphon Power
  - **UNDOCUMENTED / BUG FIX:** Tooltip has been corrected to note the stated damage is the total damage dealt instead of an "additional" amount.
- Arcane Storm
  - **Bug Fixes:** Spell shields are no longer consumed by a strike without blocking its damage.

### V25.06
- Glorious Evolution
  - If a level up occurs while an Augment point is gained, no longer requires the player to have to gain another Augment point before they are able to apply it to any ability, and causing the Augment point icons in the HUD to remain greyed out and unusable until that point.
- Viktor
  - **Bug Fixes:** Corrected homeguard VFX.

### V25.04
- Arcane Storm
  - **Bug Fixes:** No longer always targets Viktor after he hits Rift Herald (including via Arcane Storm).

### V25.S1.2
- Siphon Power
  - Discharge AP ratio reduced to 50% AP from 60% AP.
- Arcane Storm
  - AP ratio per second reduced to 35% AP from 45% AP.

### V25.S1.1
- General
  - **Bug Fixes:** Corrected various clipping issues with the model.

### V14.24#December 18th Hotfix|V14.24
- Stats
  - Health growth reduced to 100 from 104.
  - Armor growth reduced to $4.4$ from $5.2$.

### V14.24
- General
  - Title changed to *The Herald of the Arcane* from *The Machine Herald*.
  - Updated ability icons.
  - Complete visual update across all skins.
    - New splash artwork for Viktor, Viktor, Viktor, and Viktor.
    - Adjusted splash artwork for Viktor, Viktor, and Viktor.
  - New lore.
  - New voice-over.
  - Updated sound effects.
- Viktor
  - Price increased to from .
- Glorious Evolution
  - Hex Fragments generated per large minion and epic monster kill increased to 10 from 5.
  - Hex Fragments generated per champion takedown reduced to 20 from 25.
  - **Removed:*** Arcane Storm is no longer automatically augmented once all of his basic abilities have been augmented; it requires gaining a fourth augment.
    - Arcane Storm cannot be augmented until after all of his basic abilities have been augmented.
  - **Bug Fixes:** Killing a champion clone no longer improperly generates 25 Hex Fragments.
- Siphon Power
  - Base shield increased to 40 to 115 from 27 to 105.
    - Empowered base shield increased to 40×1.6 to 115×1.6 from 27×1.6 to 105×1.6.
- Gravity Field
  - **Undocumented:** Zone radius increased to 340 units from 310.
  - Zone duration increased to $4.5$ seconds from $3.75$ seconds.
  - Slow strength increased to 33 / 36 / 39 / 42 / 45% from 30 / 34 / 38 / 42 / 45%.
  - Slow duration increased to 1 second from $0.25$ seconds.
  - Stack application rate increased to one per $0.25$ seconds from one per $0.5$ seconds.
  - Maximum stacks increased to 5 from 3.
    - *Stacks are now consumed upon the fifth stack.*
  - **Removed:*** Enemies no longer become immune to the slow after their stacks are consumed.
  - **Bug Fixes:** Stack application time is now consistent and is no longer improperly extended by up to $0.25$ seconds (total $0.75$ seconds of inconsistency over previously 3 ticks).
- Hextech Ray
  - Renamed to *Hextech Ray* from *Death Ray*.
  - Mana cost reduced to 60 / 70 / 80 / 90 / 100 from 70 / 80 / 90 / 100 / 110.
- Arcane Storm
  - Renamed to *Arcane Storm* from *Chaos Storm*.
  - **New Effect:** Targeting the user now properly follows them instead of stopping upon reaching their location at the time of casting.
  - **New Effect:** Augment: Perfect Storm — The singularity now increases in by 40% and extends its duration by 3 seconds whenever an enemy champion is taken down after having been damaged by the ability. Size can only be increased up to 6 times and the duration can only be extended up to the original duration.
- Viktor
  - Arcane Storm
    - **Bug Fixes:** Singularity VFX no longer sometimes desyncs for allies.

### V14.12
- Siphon Power
  - **New Effect:** *Discharge* attack is now consumed against turrets, dealing 100% damage.

### V14.9
- Stats
  - Selection radius reduced to 120 units from 160.

## Trivia

- Pre-rework Viktor's robotic hand from the Prototype skin can be seen in the video for the Mac Version of League of Legends.
- Viktor's dance references the *Melbourne Shuffle*, a dance originating from Australia.
  - A side-by-side comparison can be seen here.
- Viktor was the first champion who had a unique item available only for him (Prototype Hex Core), sharing this feature with Kalista.

---
*This page was automatically generated from League of Legends Wiki data.*