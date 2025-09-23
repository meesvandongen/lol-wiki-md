# Urgot

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
| **Champion** | Urgot |
| **Title** | the Dreadnought |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2010-08-24 |
| **Release Patch** | V1.0.0.99 |
| **Latest Changes** | V25.06 |
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
| **Utility** | 1 |
| **Style** | 55 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $655.0$ | $+102.0$ |
| **Mana** | $340.0$ | $+45.0$ |
| **Health Regen** | $7.5$ | $+0.7$ |
| **Mana Regen** | $7.25$ | $+0.8$ |
| **Armor** | $36.0$ | $+5.0$ |
| **Magic Resist** | $32.0$ | $+2.05$ |
| **Attack Damage** | $63.0$ | $+4.0$ |
| **Attack Speed** | $0.625$ | |
| **Movement Speed** | $330.0$ | $+0.0$ |
| **Attack Range** | $350.0$ | $+0.0$ |
| **Base Attack Speed** | $0.625$ | |
| **Attack Speed Ratio** | $0.625$ | |
| **Bonus AS per Level** | $3.8\%$ | |
| **Missile Speed** | $2500$ units/second | |
| **Acquisition Radius** | $550$ units | |
| **Gameplay Radius** | $80$ units | |
| **Pathing Radius** | $50$ units | |
| **Selection Radius** | $136.111$ units | |
| **Selection Height** | $155.556$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Taken** | $105.0\%$ |

#### URF

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $115.0\%$ |
| **Damage Taken** | $85.0\%$ |

## Abilities

### Passive: Echoing Flames

| Attribute | Value |
|-----------|------:|
| **Effect Radius** | 700 (Effect range, estimated) units |
| **Angle** | cr 60° |
| **Targeting** | Passive |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | False |
| **Spell Effects** | spellaoe |
| **Projectile** | False |
| **Parry** | Special |
| **Per-Leg Cooldown (Unaffected by ability haste)** | 30–2.5@1–13 |

**INNATE:** **Urgot**’s legs each conceal a shotgun that covers an arc, together forming a full circle of cover around him. Each shotgun can fire once before incurring a unique cooldown.

**Urgot**’s next basic attack on-hit within the arc of a shotgun will cause it to fire in a cone, dealing key=% AD (+ changedisplay=true of target's **maximum** health) physical damage to enemies hit, capped at 100 to 360 against monsters.

**Notes:**

- *Echoing Flames*:
  - Shotgun attack range scales with **bonus** attack range.
  - Will not trigger if the target is too far away or they are dead when the basic attack hits.
  - Will not trigger while blinded or if the basic attack is blocked by Wind Wall or Blade Whirl.
    - It bypasses blocking and dodging effects.
  - Will not trigger against structures nor wards.
  - Will trigger from Runaan's Hurricane additional bolts.
  - Will not apply life steal, but does apply on spell vamp and omnivamp.
  - Will apply spell effects.
  - Will not trigger when basic attacking certain units (e.g Jack in the Box), however it will trigger when attacking them with Purge.
  - Will consume Manaflow Band if it is available.
  - Will not consume Tear of the Goddess charges.
- **Urgot**’s legs never change in orientation, with his body instead rotating on top of them; the leg that faces northeast will always face northeast. This does not affect **Urgot**’s facing direction for the purposes of abilities such as Petrifying Gaze.
- Unlike other innate abilities, *Echoing Flames* grants stacks of Spear of Shojin Focused Will.

---

### Q: Corrosive Charge

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 800 units |
| **Effect Radius** | 210 units |
| **Cost** | 70 Mana |
| **Cooldown** | 10 / 9.5 / 9 / 8.5 / 8 seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | Area of Effect |
| **Projectile** | True |
| **Out of Range Behavior** | walk to location |

**ACTIVE:** **Urgot** launches a canister at the target location. Upon landing, it explodes after $0.3$ seconds to deal physical damage to enemies hit and slow them for $1.25$ seconds.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 25 / 70 / 115 / 160 / 205 (+ 70% AD) |

| Attribute | Value |
|-----------|------:|
| **Slow** | 45 / 50 / 55 / 60 / 65% |

**Notes:**

- *Corrosive Charge* prevents Purge from firing for $0.05$ seconds after the cast time.

---

### W: Purge

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 490 (Center-to-edge; See notes for more details) units |
| **Speed** | 2500 (Automatic attacks missile speed) units/second |
| **Cost** | 40 / 30 / 20 / 10 / 0 Mana |
| **Cooldown** | 12 / 9 / 6 / 3 / 0 seconds |
| **Cooldown Start** | post-effect |
| **Static Cooldown** | $0.5$ (Rank 5 toggle on) / $0.25$ (Rank 5 toggle off) |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | False |
| **Spell Effects** | basic |
| **Projectile** | True |
| **Parry** | True |
| **Call For Help** | True |

**PASSIVE:** **Urgot**’s other abilities *mark* enemy champions hit for 5 seconds. Only one enemy can be *marked* at a time.

 Additionally, at maximum rank, *Purge* lasts indefinitely and becomes a toggled ability.

**ACTIVE:** **Urgot** equips his machine gun for 4 seconds, granting ghosting to minions and non-epic monsters around him and autonomously firing at the nearest enemy at a **fixed** attack speed, prioritizing *marked* enemy champions and refreshing the *mark* with every attack. While firing, **Urgot** is able to move and gains 40% slow resist, but his **base** movement speed is reduced by 125.

Attacks with *Purge* deal ***modified** physical damage, with a minimum threshold of 50 against monsters and minions, and cannot critically strike. Each attack applies on-hit effects, with on-hit damage reduced to 50% effectiveness, and triggers on-attack effects. **Urgot** cannot perform attacks while unable to declare basic attacks.

| Attribute | Value |
|-----------|------:|
| **Modified Physical Damage** | 12 (+ 20 / 23.5 / 27 / 30.5 / 34% AD) |

*Purge* can be recast after $0.5$ seconds within the duration, and does so automatically afterwards.

**RECAST:** **Urgot** ends *Purge*.

**Notes:**

- Autonomous attack range scales with **bonus** attack range.
- *Purge* uses edge range for enemy targets only; Its range is *center-to-edge*. ****Urgot**’s increasing does not increase its range.
- The initial cast and the manual recast count as ability activations for the purposes of on-cast effects such as Spellblade and triggering Force Pulse’s passive.
  - Before maximum rank, if *Purge* is not manually recasted, the effect will end without being considered as an ability activation.
- **Urgot**’s facing direction during *Purge* is in the direction he's moving and not in the direction of his attack target.
- Before maximum rank, if continuously attacking without being interrupted or interrupting or canceling the cast, **Urgot** will attack 13 times.
- The bolts are considered basic attacks and thus apply life steal at full efficiency.
- During *Purge*, **Urgot**’s attack range is reducedand his attack commands instead issue movement commands to walk into the respective range of his target.
- Slow resist does not modify flat reductions in movement speed.
- Percentage slows apply after the flat movement reduction.
- *Purge*’s attacks do not interact with **Urgot**’s basic attack timer.
  - After ending *Purge*, he can usually attack again immediately.
- If **Urgot** is berserked or taunted, he will fire at the unit he is forced to attack.
- *Purge* will fire while **Urgot** is dashing (e.g. while taking Dark Passage), with the exception of Disdain’s dash.
- Each shot counts as a separate hit for effects such as Electrocute, Muramana Shock, and Eclipse Ever Rising Moon.
- *Purge* will apply Runaan's Hurricane on every shot at 100% of the listed damage.
  - On-hit damage applied by the bolts are still reduced by *Purge*.
- The movement speed stat tooltip erroneously reads that the **bonus** movement speed is being reduced before **base** movement speed.

---

### E: Disdain

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.45$ seconds |
| **Target Range** | 475 (Maximum range) / 450 (Minimum range) units |
| **Speed** | 1200 + units/second |
| **Cost** | 60 / 70 / 80 / 90 / 100 Mana |
| **Cooldown** | 16 / 15.5 / 15 / 14.5 / 14 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | Area of Effect |
| **Grounded** | True |
| **Knockdown** | True |

**ACTIVE:** **Urgot** grants himself a shield for 4 seconds.

| Attribute | Value |
|-----------|------:|
| **Shield Strength** | 55 / 75 / 95 / 115 / 135 (+ 135% **bonus** AD) (+ $13.5$% **bonus** health) |

After the cast time, **Urgot** dashes in the target direction, though not through terrain, dealing physical damage to enemies he passes through, knocking them aside and stunning them for 1 second.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 90 / 120 / 150 / 180 / 210 (+ 100% **bonus** AD) |

**Urgot** will stop upon hitting an enemy champion, stunning them for $1.5$ seconds and, after $0.25$ seconds, also flinging them to the location 100 units behind him (this destination is a fixed location and is unaffected by modifiers on Urgot or the target moving away before the displacement started), though not through terrain.

*Purge can be cast during the dash.*

**Notes:**

- **Urgot** turns around when tossing an enemy over him, always facing his victim.
- The target will be flung to the location behind **Urgot** over $0.5$ seconds given that they do not move nor are moved from the position at which they collided with him while stunned.
- *Disdain* prevents Purge from firing for $1.25$ seconds.
- The range indicator length is 475 units, but the dash only moves **Urgot** 450 units. Effect at cast time end
- The shield is granted at the start of the cast time while the dash begins after.
- The fling will occur after $0.25$ seconds regardless of how far away the target is.
- Displacement immunity will not resist the application of the stun.

---

### R: Fear Beyond Death

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.5$ seconds |
| **Target Range** | 2500 units |
| **Effect Radius** | Global (Recast range) / cr 600 (Fear radius) |
| **Width** | 160 (Both initial cast and chains) units |
| **Speed** | 3200 units/second |
| **Cost** | 100 Mana |
| **Cooldown** | 100 / 92.5 / 85 / 77.5 / 70 (Starts on-cast if no target is hit, post-effect if it does) seconds |
| **Cooldown Start** | Special |
| **Targeting** | Direction / Auto |
| **Affects** | Enemies |
| **Damage Type** | Physical True |
| **Spell Shield** | Special |
| **Spell Effects** | Special |
| **Projectile** | True |
| **Call For Help** | Special |
| **Silence** | False |

**ACTIVE:** **Urgot** fires a chem-drill in the target direction that impales the first enemy champion hit, dealing physical damage and leashing the target for 4 seconds, during which they are revealed and slowed by type=target's **missing** health.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 100 / 162.5 / 225 / 287.5 / 350 (+ 50% **bonus** AD) |

*Fear Beyond Death* can be recast if the leashed target is below health, and does so automatically if the chem-drill would kill the target (Damage instance does not reduce their health below 1 in this case), the ability is cast again while the target was above the threshold and is then brought within it, or the target is within the threshold at the end of the duration.

**RECAST - MERCY:** **Urgot** launches chains at the leashed target, then channels for $1.5$ seconds once they latch on. During this time, he reels them toward him, suppressing them, revealing them, rendering them untargetable, and preventing them from taking damage from other sources (Negates application of damage from any source that is not the execution). If the target is near **Urgot** when the chains connect, they are pushed up to 1000 (Estimated) units away based on their proximity to him. This channel cannot be interrupted by crowd control.

Upon completing the channel, the target is execute. If the execution is successful, he fears nearby enemies for $1.5$ seconds, during which they are slowed by 75%.

*Fear Beyond Death's recast can be used while affected by cast-inhibiting crowd control.*

**Notes:**

- Unlike most projectiles, *Fear Beyond Death* **can** hit the edge of targets whose center is further than the missile range.
  - However, it still cannot hit targets whose center is behind **Urgot**.
- Applies spell damage on first cast and raw damage on recast.
- **Urgot** is revealed while the target is leashed and being reeled in.
- *Fear Beyond Death*’s execute against the enemy champion will aggro nearby enemy minions.
- While *Fear Beyond Death* is not on cooldown, all enemy champions will have an execution indicator on their health bar, and a visual indicator if below the execution threshold.
- *Mercy*’s granted untargetability does not destroy in-flight projectiles for the target.
- **Urgot** will place himself onto the ground and interrupts displacements affecting him upon starting the channel.
  - If the displacement effect is a *knock up* with no debuff source, the *airborne* status is not removed and he spends significantly less time in the air. Effect at cast time start
- *Fear Beyond Death* prevents Purge from firing for $0.8$ seconds.
  - The recast prevents Purge from firing for $1.5$ seconds.
- If **Urgot** is in cast time or is dashing while the chains are in transit, *Mercy*’s forced movement will be deferred (the debuff will continuously refresh) until the cast or dash is complete. The target will still be *untargetable*, prevented from taking damage from other sources, and *suppressed* during this time.
  - If **Urgot** is channeling a spell, the channel is interrupted when the chains hit.
- Removing or resisting the slow has no effect on the mark debuff.
- The initial cast always counts as an ability activation for the purposes of on-cast effects such as Spellblade and triggering Force Pulse’s passive.
  - If the ability hits a target who is below the threshold, it will automatically recast itself and it won't be considered as an ability activation.
  - If the ability hits a target who is above the threshold and it's recast early, the recast will happen automatically when they reach the threshold and it won't be considered as an ability activation.
  - If the target got hit while above the threshold and it is recast after they fall below it, the recast will be considered as an ability activation.
- Reactivating the ability early will reel the target as soon as they reach the 25% **maximum** health threshold.
  - *Mercy* can be cast and will automatically do so at the end of the duration if the target is below the execute threshold regardless of **Urgot** being affected by crowd control, including self or enemy-imposed stasis.
- *Fear Beyond Death* can affect clones and activates its fear when executing one.
- *Fear Beyond Death*’s recast cannot be used if the target is untargetable.
- *Fear Beyond Death*’s recast cannot be used during resurrection. Attempting to do so places the recast on a 5-second cooldown.
  - If the target is below the threshold at the end of the duration and **Urgot** is revived, the recast will initiate regardless of this cooldown.
- The recast's chains are classified as a projectile and they will fail to affect their target if:
  - **Urgot** or his target dies or enters resurrection.
  - The target becomes untargetable, even if the granted source does not destroy in-flight projectiles.
    - **Urgot** will also fail to execute the target if they become untargetable while the chains are being pulled.
  - They collide with Wind Wall or Unbreakable.
    - If Unbreakable intercepts the chains and Braum is not the target, they're destroyed.
    - If Braum is the target, he will still be reeled in and executed as normal.
  - The target is crowd control immune when the chains hit.
  - The target is a zombie.
- Spell shield will **not** block the recast's chains.
- If the target removes the suppression by any means they will prevent themselves from being executed and stop the forced movement from being renewed, though allowing the most recent one to be present and move them to **Urgot**’s previous location.
  - Their untargetability and damage prevention of other sources granted by *Fear Beyond Death* is removed in the process.
  - **Urgot** will still complete the full channel despite this.
    - The execution would not count as being successful in this case and thus he will not fear nearby enemies after the failed execution.
- If the target is displacement immune when the chains collide, **Urgot** will not begin the channel until after their displacement immunity ends.
  - The target will be unaffected by *Fear Beyond Death*’s forced movement during this time, but will still be suppressed, prevented from taking damage from other sources and rendered untargetable.
  - If the target's displacement immunity does not end within 10 seconds, *Fear Beyond Death* will cancel.
- When the chains start reeling in and suppressing a player's champion, chains will also cover the sides of their screen.
- The target champion shrinks in in several steps after reaching **Urgot**’s grinder and before being executed.
  - If the *suppression* ends in any way, the target returns to its normal size.
  - If the execution is successful, the victim's corpse will be effectively invisible unless they enter a zombie state.
- The target will die regardless of their **current** health once the recast's chains hit, even if they are healed.
  - If the target has invulnerability that persists for the duration of the channel, it will protect them from the execution.
    - Undying Rage will prevent Tryndamere from dying by the execute. In this case, the execute will bypass his minimum health threshold though and reduce him to 1 health.
    - The effect will also not prevent the activation of resurrection effects, such as Guardian Angel or Rebirth.
    - Post-death events such as Zombie State effects (e.g, Death Defied, Icathian Surprise, and Glory in Death) will be triggered.
    - Only the target's **current** health will count as **Urgot**’s damage in post-game statistics and spell vamp healing.
- *Fear Beyond Death*’s initial cast will not perform the automatic recast from dealing enough damage to kill Sion.
- *Fear Beyond Death*’s recast will be placed on a 5-second cooldown if the target is below the threshold and becomes targetable again after completing the dash from Hero's Entrance.
- *Fear Beyond Death* will not put the target inside terrain while reeling them in.
  - This does prevent the execution.
- The following table refers for interactions while **Urgot** is channeling:

---

## Patch History

### V25.06
- Purge
  - **Bug Fixes:** If toggled on and Atakhan resets, no longer causes the monster to become stuck in a resetting state.

### V25.04
- Fear Beyond Death
  - **Bug Fixes:** If his False Life triggers during the pull and the duration permits, enemies no longer blink to the caster's fountain alongside them upon reaching them and during the delay until execution occurs.

### V25.S1.3
- Purge
  - **Bug Fixes:** If toggled on and a jungle camp resets, no longer causes the monster to become stuck in a resetting state until it has collided with a unit or become displaced.

### V14.15
- Purge
  - **UNDOCUMENTED / BUG FIX:** Purge attacks are now properly disabled during Teleport’s and Unleashed Teleport’s channels.

### V14.10
- Purge
  - **Removed:*** No longer applies spell effects.
- Fear Beyond Death
  - **Bug Fixes:** No longer unintentionally repeatedly displaces **Urgot** and Skarner in the direction of Mercy and over large distances, if Mercy is cast while **Urgot** simultaneously becomes affected by Impale.
    - Both users will now stay at their original location.

### V14.9
- Stats
  - Armor growth reduced to 5 from $5.45$.

### V14.6
- Purge
  - **Bug Fixes:** Now properly counts as ability damage.

### V14.4
- Purge
  - **Bug Fixes:** No longer incorrectly applies Hullbreaker Skipper at full effectiveness.

### V13.7
- Purge
  - **Bug Fixes:** No longer continually triggers the on-hit healing of You and Me!.

### V12.12
- Disdain
  - **UNDOCUMENTED/BUG FIX:** Tooltip now lists the stun duration.

## Trivia

- Urgot and Thresh used to be the only champions classified as ranged tanks before having their roles changes in 2013.
  - Hyper-Kinetic Position Reverser and Death Sentence are the only two in-game tethered abilities without a minimum leash range.
- Urgot used to be the only champion in the game classified as a Marksman/Tank. However, his champion classification has since been changed to Juggernaut.
  - During the time that Gnar was revealed on PBE, he was classified as a Marksman/Tank. However, Gnar was released as a Fighter / Marksman, a champion classification he shared with Jayce.
- Urgot resembles a number of quadrupeds from various other games.
  - (leader of the ) from the *Quake (series)* video game series.
  - the Lord of Sin from *Diablo III*.
  - *"The Spineless Ones"* from the X-Men, the best-known of them being Mojo (comics). They are a fat alien race of rotten appearance who can not move on their own without advanced technology.
  - from **.
- According to Babaganoosh, Urgot was unavailable for the Ionia vs Noxus match due to being damaged and needing considerable time to be repaired. He received a major strength buff in the following patch release.
- Urgot's dance references Victory Dance.
  - A by comparison can be seen here.
- In the V1.0.0.115 April Fools' Day patch, he was jokingly listed to receive a skin called **"Urfgot"**, referencing Urf.
- This champion has no ability power ratio.
- His dance references the Can-Can.
  - He shares this dance with Elise and Veigar.
- His login screen theme is sung in Noxian. The repeated chant is "Stom Ur Got!"
- During his second champion spotlight for his rework, David 'Phreak' Turley jokingly said that Urgot will be the next Star Guardian, a reference to a long-standing joke in the community, which partially became true with Urgot being released for the 2020 April Fools' Day event.
- Urgot is able to kill the Enemy Target Dummies in the Practice Tool, by using Fear Beyond Death on them when they have 25% health or lower.
- Urgot's joke will cause him to retrieve an item:
  - Poison Puffcap.
  - .
  - .
  - Gold Duck.
    - This item is a rare variant of the animation.
- To date, it is the only one to have two joke skins released on April Fools' Day: Urgot and Urgot, respectively.

---
*This page was automatically generated from League of Legends Wiki data.*