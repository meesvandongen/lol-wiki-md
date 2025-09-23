# Ambessa

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
| **Champion** | Ambessa |
| **Title** | Matriarch of War |
| **Resource** | Energy |
| **Range Type** | Melee |
| **Release Date** | 2024-11-06 |
| **Release Patch** | V14.22 |
| **Latest Changes** | V25.15 |
| **Roles** | Diver, Skirmisher |
| **Riot Positions** | Top |
| **External Positions** | Top |
| **Blue Essence** | 3150 |
| **Riot Points** | 975 |
| **Difficulty** | 3 |
| **Hero Type** | Fighter |
| **Alt Type** | Assassin |
| **Adaptive Type** | Physical |
| **Damage** | 3 |
| **Toughness** | 2 |
| **Control** | 1 |
| **Mobility** | 3 |
| **Utility** | 1 |
| **Style** | 55 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $630.0$ | $+110.0$ |
| **Energy** | $200.0$ | $+0.0$ |
| **Health Regen** | $8.5$ | $+0.75$ |
| **Energy Regen** | $50.0$ | $+0.0$ |
| **Armor** | $35.0$ | $+4.9$ |
| **Magic Resist** | $32.0$ | $+2.05$ |
| **Attack Damage** | $63.0$ | $+3.0$ |
| **Attack Speed** | $0.625$ | |
| **Movement Speed** | $335.0$ | $+0.0$ |
| **Attack Range** | $125.0$ | $+0.0$ |
| **Base Attack Speed** | $0.625$ | |
| **Attack Speed Ratio** | $0.625$ | |
| **Bonus AS per Level** | $2.5\%$ | |
| **Attack Windup** | $15.6\%$ | |
| **Acquisition Radius** | $600$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $100$ units | |
| **Selection Height** | $100$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Taken** | $105.0\%$ |

## Abilities

### Passive: Drakehound's Step

| Attribute | Value |
|-----------|------:|
| **Target Range** | 175 (Minimum dash distance) / 350 (Maximum dash distance) units |
| **Speed** | 770@1; 830@6; 890@11; 950@16 + 100% movement speed |
| **Targeting** | Passive |
| **Affects** | Self, Enemies |
| **Damage Type** | physical |
| **Spell Shield** | False |
| **Spell Effects** | proc |
| **Parry** | True |
| **Grounded** | True |
| **Knockdown** | True |

**INNATE:** During the lockout of **Ambessa**’s abilities, inputting an attack or movement command (Default MB2/right click) causes her to dash to or towards (If inputted outside of the maximum dash range, she will dash the maximum distance towards the target/location) the target or location, respectively, after the lockout ends. This dash cannot pass through terrain.
***Ambessa** cannot dash (See notes) while immobilized or grounded.*

**INNATE:** Whenever **Ambessa** casts an ability, she generates a stack of *Medarda Maxim* after the ability's respective lockout for 4 seconds, refreshing with subsequent casts and stacking up to 3 times. Basic attacks consume one stack each to become empowered.

**MEDARDA MAXIM:** **Ambessa**’s next basic attack on-attack is empowered to have an uncancellable windup, gain 75 **bonus** range and 50% **bonus** attack speed, and deal 5 to 30 (+ 30% **bonus** AD) **bonus** physical damage and restore energy.

**Notes:**

- If an Attack command has been queued during an ability lockout and both:
  1. the player issues a Hold (default **J**) or Stop (default **S**) command, **and**
  1. the acquired target is within **Ambessa**’s attack range after the end of the lockout
  - then **Ambessa**’s dash will cancel.
- If **Ambessa** is immobilized or grounded during an ability's lockout, any attack or movement commands inputted during the lockout will not be buffered to cast at the end of the lockout, even if the crowd control has worn off by the time she is able to move (and thus dash).
  - This also applies vice versa where she has successfully buffered an inputted attack or movement command but is afterwards affected with the aforementioned crowd control types during the remaining lockout time. This causes the buffer to be cancelled and the dash to fail to trigger as a consequence.
  - Attack commands will not be buffered to cast at the end of the lockout even while grounded.
- If multiple attack and movement commands are inputted during the lockout of an ability, the most recent one is used for the dash's targeting.
  - If the most recent input is not an attack or movement command, the dash will not trigger.
    - In this case, the buffer for the previous attack/movement command was cancelled by a new non-attack/movement command such as an input for an ability cast.
- During *Drakehound's Step*, a visual indicator is visible to **Ambessa** displaying the eventual dash's direction and destination location.
- The number of available attacks empowered by *Medarda Maxim* are visible as pips under **Ambessa**’s health bar, only visible to the player.
- The following table refers for interactions while **Ambessa** is dashing:

---

### Q: Cunning Sweep

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 275 (Inner radius) / 135 (Radius around Ambessa) / 400 (Outer radius) units |
| **Angle** | 180° |
| **Cost** | 70 Energy |
| **Cooldown** | 14 / 13 / 12 / 11 / 10 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | False |

**ACTIVE:** **Ambessa** enters a $0.225$-second lockout, at the end of which she slashes with her twin drakehounds in a cone in the target direction, dealing physical damage to enemies hit. The damage is doubled against enemies hit by the outer edge.

*Drakehound's Step’s* dash may be buffered during the lockout or initiated within $0.275$ seconds of the lockout ending, and **Ambessa** will perform the dash after her slash completes.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 20 / 30 / 40 / 50 / 60 (+ 30% **bonus** AD) (+ 1 / 1.5 / 2 / 2.5 / 3% (+ $1.5$% per 100 **bonus** AD) of the target's **maximum** health) |
| **Increased Physical Damage** | 40 / 60 / 80 / 100 / 120 (+ 60% **bonus** AD) (+ 2 / 3 / 4 / 5 / 6% (+ 3% per 100 **bonus** AD) of the target's **maximum** health) |

If this hits at least one enemy, **Ambessa** can cast *Sundering Slam* within the next 4 seconds.

Against monsters, *Cunning Sweep* deals 125 **bonus** physical damage and the damage based on their health ratio is capped at 100 to 300.

**Notes:**

- *Cunning Sweep* will slash from wherever **Ambessa** is at the end of the lockout.
  - If her facing direction changes before the lockout completes, the slash will be performed towards the new direction.
- The following table refers for interactions while **Ambessa** is locked out:

---

### Q: Sundering Slam

| Attribute | Value |
|-----------|------:|
| **Range** | 650 units |
| **Cast Time** | none |
| **Width** | 40 units |
| **Cost** | 70 Energy |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | False |

**ACTIVE:** **Ambessa** enters a $0.225$-second lockout, at the end of which she slams her twin drakehounds in a line in the target direction, dealing physical damage to enemies hit. The damage is doubled against the first enemy hit.

*Drakehound's Step’s* dash may be buffered during the lockout or initiated within $0.275$ seconds of the lockout ending, and **Ambessa** will perform the dash after her slam completes.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 25 / 37.5 / 50 / 62.5 / 75 (+ 45% **bonus** AD) (+ 1 / 1.5 / 2 / 2.5 / 3% (+ 2% per 100 **bonus** AD) of the target's **maximum** health) |
| **Increased Physical Damage** | 50 / 75 / 100 / 125 / 150 (+ 90% **bonus** AD) (+ 2 / 3 / 4 / 5 / 6% (+ 4% per 100 **bonus** AD) of the target's **maximum** health) |

Against monsters, *Sundering Slam* deals 125 **bonus** physical damage and the damage based on their health ratio is capped at 100 to 300.

**Notes:**

- *Sundering Slam* will slam from wherever **Ambessa** is at the end of the lockout.
  - If her facing direction changes before the lockout completes, the slam will be performed towards the new direction.
- The following table refers for interactions while **Ambessa** is locked out:

---

### W: Repudiation

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 325 units |
| **Cost** | 70 Energy |
| **Cooldown** | 18 / 17 / 16 / 15 / 14 seconds |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |

**ACTIVE:** **Ambessa** braces herself for up to $0.5$ seconds, during which in the first $0.225$ seconds she is unable to act. After **Ambessa** finishes bracing, she smashes the ground beneath her, dealing physical damage to nearby enemies.

*Drakehound's Step’s* dash may be buffered during the lockout or initiated within $0.25$ seconds of the lockout ending.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 50 / 75 / 100 / 125 / 150 (+ 50% **bonus** AD) |

Additionally, **Ambessa** shields herself at the time of cast for 50 to 320 (+ 150% **bonus** AD) for $1.5$ seconds. If the shield mitigates any amount of damage taken from champions, large monsters, or turrets before **Ambessa** smashes the ground, *Repudiation*’s damage is increased by 50%.

| Attribute | Value |
|-----------|------:|
| **Increased Physical Damage** | 75 / 112.5 / 150 / 187.5 / 225 (+ 75% **bonus** AD) |

***Ambessa** will smash the ground at the end of Drakehound's Step’s dash if she would finish bracing during it, and otherwise always smashes the ground from wherever she is at the end of the bracing time.*

**Notes:**

- *Repudiation*’s bracing time is unaffected by the initiation of *Drakehound's Step* dash.
  - **Ambessa** will smash the ground from wherever she is at the end of the bracing time even if the dash ends or completes its travel before the brace is finished.
    - She will remain in place until the brace is finished in this case.
    - If the brace would however finish during the dash, due to the specific timing of the dash's initiation, **Ambessa** will smash the ground at the end of the dash instead.
      - This would apply in cases where the dash is initiated right before or shortly before the bracing is finished.
- *Repudiation*’s shield buff and shield health on the health bar UI are not granted to and displayed on the caster on-cast, but rather only after a very short, inconsistent delay from the time of cast.
- The following table refers for interactions while **Ambessa** is locked out:
  - **Ambessa** will smash the ground instantly upon entering resurrection.

---

### E: Lacerate

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 325 units |
| **Cost** | 70 Energy |
| **Cooldown** | 13 / 12 / 11 / 10 / 9 seconds |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |

**ACTIVE:** **Ambessa** enters a $0.225$-second lockout and spins her twin drakehounds around her to deal physical damage to nearby enemies and slow them by 99% decaying over 1 second.

*Drakehound's Step’s* dash may be buffered during the lockout or initiated within $0.275$ seconds of the lockout ending; in either case, she will spin a second time at the end of the dash to apply the same effects at no additional cost.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 40 / 60 / 80 / 100 / 120 (+ 40 / 45 / 50 / 55 / 60% **bonus** AD) |

| Attribute | Value |
|-----------|------:|
| **Total Physical Damage** | 80 / 120 / 160 / 200 / 240 (+ 80 / 90 / 100 / 110 / 120% **bonus** AD) |

**Notes:**

- **Ambessa** will cast the second spin of *Lacerate* from wherever she is at the end of the dash.
  - She will spin a second time even if the dash is interrupted.
- The following table refers for interactions while **Ambessa** is locked out:

---

### R: Public Execution

| Attribute | Value |
|-----------|------:|
| **Range** | 1250 units |
| **Cast Time** | $0.55$ seconds |
| **Width** | 65 units |
| **Cooldown** | 130 / 122.5 / 115 / 107.5 / 100 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | spell |
| **Grounded** | True |

**PASSIVE:** **Ambessa** gains armor penetration and heals herself for a percentage of the post-mitigation damage (Damage calculated after modifiers) she deals to enemies with her active abilities. The healing effectiveness is reduced to 25% against minions and 40% against monsters.

| Attribute | Value |
|-----------|------:|
| **Armor Penetration** | 10 / 15 / 20 / 25 / 30% |

| Attribute | Value |
|-----------|------:|
| **Healing Percentage** | 10 / 11.25 / 12.5 / 13.75 / 15% (+ 50% life steal) |
| **Minion Healing Percentage** | 2.5 / 2.8125 / 3.125 / 3.4375 / 3.75% (+ 12.5% life steal) |
| **Monster Healing Percentage** | 4 / 4.5 / 5 / 5.5 / 6% (+ 20% life steal) |

**ACTIVE:** **Ambessa** prepares a strike in a line in the target direction, then blinks behind the farthest enemy champion within the area and seizes them. If she successfully (See notes) seizes the target, **Ambessa** attaches them to herself and suppresses them for $0.75$ seconds. While the target is suppressed, they are revealed and **Ambessa** picks them up off the ground before crashing them back down, afterwards dealing physical damage and stunning them for $0.4$ seconds.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 150 / 200 / 250 / 300 / 350 (+ 80% **bonus** AD) |

***Ambessa** is displacement immune and unable to act during the cast time and while the target is suppressed, lingering for $0.1$ seconds afterwards. Drakehound's Step’s dash may be buffered during the cast time or lockout, as well as initiated within $0.275$ seconds of the lockout ending.*

**Notes:**

- *Public Execution*’s attachment depends on the application of the suppression; if the suppression is not applied, neither is the attachment. Similarly, if the suppression is removed, so is the attachment.
  - If the target resists the suppression by being immune to crowd control, displacement immune, or having a spell shield, **Ambessa** will not attach them to herself.
    - She will be unable to apply the damage and stun in this case since the suppression was not applied in the first place.
    - She will still blink to the target in this case.
  - If the target removes the suppression by any means, including with an applicable cleanse effect or dispel, they will instantly detach themselves from **Ambessa**, which causes the ability to cancel immediately.
    - She will fail to apply the damage and stun to the target since the suppression was removed from them early in this case.
    - Her lockout and displacement immunity also ends prematurely in this case.
- If **Ambessa** would blink inside the attack range of an enemy turret, including the Nexus Obelisk, *Public Execution* will instead attempt to position her outside of it.
- **Ambessa** will reveal herself during the cast time if there is an enemy champion nearby.
- **Ambessa** will search for enemies in the direction she is facing at the end of cast time.
  - This direction can be changed by Encore.
- If **Ambessa** successfully seizes a target, any regular movement commands issued before doing so are discarded upon the lockout ending.
  - This does not affect movement orders issued during the lockout.
- The following table refers for interactions while **Ambessa** is in cast time and during the lockout:

---

## Patch History

### V25.15
- Public Execution
  - **Bug Fixes:** Hit no longer fails to generate Conqueror stacks.

### V25.11
- General
  - Updated recommended items.

### V25.05
- Lacerate
  - Base damage per hit reduced to 40 / 60 / 80 / 100 / 120 from 40 / 65 / 90 / 115 / 140.

### V25.S1.3
- General
  - **Bug Fixes:** Voicelines that trigger from Mel are no longer missing their effect processing.
  - **Bug Fixes:** Non-unique-event voicelines that trigger from Mel no longer fail to play more lines than one per event.
- Drakehound's Step
  - **New Effect:** If an Attack command has been queued during an ability lockout and both **1**) the player issues a Hold (default **J**) or Stop (default **S**) command and **2**) the acquired target is within **Ambessa**’s attack range after the end of the lockout, then **Ambessa**’s dash will cancel.
    - Hold/Stop commands will now also properly invalidate the acquisition if the target is not within attack range under these conditions.

### V25.S1.2
- Drakehound's Step
  - **Bug Fixes:** No longer ignores block and dodge and can now properly miss while Ambessa is blinded.

### V14.24
- Drakehound's Step
  - **Bug Fixes:** Dash pathfinding no longer ignores Emperor's Divide.
- Repudiation
  - Base shield reduced to 50 to 320 from 85 to 350.
  - Shield bonus AD ratio reduced to 150% **bonus** AD from 175%.
  - Shield duration reduced to $1.5$ seconds from 2.
- Lacerate
  - Bonus AD ratio reduced to 40 / 45 / 50 / 55 / 60% **bonus** AD from 40 / 50 / 60 / 70 / 80%.
    - Total bonus AD ratio reduced to 80 / 90 / 100 / 110 / 120% **bonus** AD from 80 / 100 / 120 / 140 / 160%.

### V14.23
- Ambessa
  - **Bug Fixes:** Various VO events no longer fail to trigger.
  - **Bug Fixes:** Homeguard trail VFX now displays correctly.
- Drakehound's Step
  - **Bug Fixes:** Dash pathfinding no longer ignores champion-generated terrain.
  - **Bug Fixes:** Using Hextech Rocketbelt Supersonic during *Drakehound's Step* but after issuing a movement order no longer causes **Ambessa**’s pathing speed to remain locked at the value of Supersonic's dash speed for the duration required to reach the movement order's destination, whatever distance that may have been (or otherwise until the order completes by any means, such as being canceled by the player, immobilizing crowd control, displacement, etc), thus illegally allowing her to move at an extraordinary speed. *[Note: Ambessa's actual movement speed statistic did not change during this interaction.]*
  - **Bug Fixes:** Dash can now properly be buffered and cast during Dash’s travel.
- Sundering Slam
  - Recast time increased to 4 seconds from $3.5$.
- Public Execution
  - **Bug Fixes:** If the cast is successful, all movement orders issued before seizing the target are now discarded upon the lockout ending (target being slammed onto the ground).
    - *[Note: this does not affect any orders issued during the lockout.]*

### V14.22
- Drakehound's Step - Innate
  - **INNATE:** During the lockout of **Ambessa**’s abilities, inputting an attack or movement command (Default MB2/right click) causes her to dash to or towards (If inputted outside of the maximum dash range, she will dash the maximum distance towards the target/location) the target or location, respectively, after the lockout ends. This dash cannot pass through terrain. **Ambessa** cannot dash while immobilized or grounded.
  - **INNATE:** Whenever **Ambessa** casts an ability, she generates a stack of *Medarda Maxim* after the ability's respective lockout for 4 seconds, refreshing with subsequent casts and stacking up to 3 times.
  - **MEDARDA MAXIM:** **Ambessa**’s next basic attack on-attack is empowered to have an uncancellable windup, gain 75 **bonus** range and 50% **bonus** attack speed, and consume a stack to deal 5 to 30 (+ 30% **bonus** AD) **bonus** physical damage and restore energy.
  - **TARGET RANGE:** 175 (Minimum dash distance) / 350 (Maximum dash distance), **SPEED:** 770@1; 830@6; 890@11; 950@16 + 100% movement speed.
- Cunning Sweep - Q1
  - **ACTIVE:** **Ambessa** enters a $0.225$-second lockout, at the end of which she slashes with her twin drakehounds in a cone in the target direction, dealing 20 / 30 / 40 / 50 / 60 (+ 30% **bonus** AD) (+ 1 / 1.5 / 2 / 2.5 / 3% (+ $1.5$% per 100 **bonus** AD) of the target's **maximum** health) physical damage to enemies hit. The damage is doubled against enemies hit by the outer edge.
    - *Drakehound's Step’s* dash may be buffered during the lockout or initiated within $0.275$ seconds of the lockout ending, and **Ambessa** will perform the dash after her slash completes.
  - If this hits at least one enemy, **Ambessa** can cast *Sundering Slam* within the next $3.5$ seconds.
  - Against monsters, *Cunning Sweep* deals 125 **bonus** physical damage and the damage based on their health ratio is capped at 100 to 300.
  - **COST:** 70 energy.
  - **COOLDOWN:** 14 / 13 / 12 / 11 / 10 seconds.
  - **CAST TIME:** None, **EFFECT RADIUS:** er 275 (Inner radius) / 400 (Outer radius).
- Sundering Slam - Q2
  - **ACTIVE:** **Ambessa** enters a $0.225$-second lockout, at the end of which she slams her twin drakehounds in a line in the target direction, dealing 25 / 37.5 / 50 / 62.5 / 75 (+ 45% **bonus** AD) (+ 1 / 1.5 / 2 / 2.5 / 3% (+ 2% per 100 **bonus** AD) of the target's **maximum** health) physical damage to enemies hit. The damage is doubled against the first enemy hit.
    - *Drakehound's Step’s* dash may be buffered during the lockout or initiated within $0.275$ seconds of the lockout ending, and **Ambessa** will perform the dash after her slam completes.
  - Against monsters, *Sundering Slam* deals 125 **bonus** physical damage and the damage based on their health ratio is capped at 100 to 300.
  - **COST:** 70 energy.
  - **CAST TIME:** None, **RANGE:** er 650, **WIDTH:** er 40.
- Repudiation - W
  - **ACTIVE:** **Ambessa** braces herself for up to $0.5$ seconds, during which in the first $0.225$ seconds she is unable to act. After **Ambessa** finishes bracing, she smashes the ground beneath her, dealing 50 / 75 / 100 / 125 / 150 (+ 50% **bonus** AD) physical damage to nearby enemies.
    - *Drakehound's Step’s* dash may be buffered during the lockout or initiated within $0.25$ seconds of the lockout ending.
  - **Ambessa** additionally shields herself for 85 to 350 (+ 175% **bonus** AD) for 2 seconds upon cast. If the shield mitigates any amount of damage taken from champions, large monsters, or turrets before **Ambessa** smashes the ground, *Repudiation*’s damage is increased by 50%.
  - ***Ambessa** will smash the ground at the end of Drakehound's Step’s dash if she would finish bracing during it, and otherwise always smashes the ground from wherever she is at the end of the bracing time.*
  - **COST:** 70 energy.
  - **COOLDOWN:** 18 / 17 / 16 / 15 / 14 seconds.
  - **CAST TIME:** None, **EFFECT RADIUS:** er 325.
- Lacerate - E
  - **ACTIVE:** **Ambessa** enters a $0.225$-second lockout and spins her twin drakehounds around her to deal 40 / 65 / 90 / 115 / 140 (+ 40 / 50 / 60 / 70 / 80% **bonus** AD) physical damage to nearby enemies and slow them by 99% decaying over 1 second.
    - *Drakehound's Step’s* dash may be buffered during the lockout or initiated within $0.275$ seconds of the lockout ending; in either case, she will spin a second time at the end of the dash to apply the same effects.
  - **COST:** 70 energy.
  - **COOLDOWN:** 13 / 12 / 11 / 10 / 9 seconds.
  - **CAST TIME:** None, **EFFECT RADIUS:** er 325.
- Public Execution - R
  - **PASSIVE:** **Ambessa** gains 10 / 20 / 30% armor penetration and heals herself for 10 / 12.5 / 15% (+ 50% life steal) of the post-mitigation damage (Damage calculated after modifiers) she deals to enemies with her active abilities, reduced to 25% against minions and 40% versus monsters.
  - **ACTIVE:** **Ambessa** prepares a strike in a line in the target direction, then blinks behind the farthest enemy champion within the area and seizes them. If she successfully seizes the target, **Ambessa** attaches them to herself and suppresses them for $0.75$ seconds, during which she reveals them and picks them up off the ground to afterwards crash them back down, dealing 150 / 200 / 250 / 300 / 350 (+ 80% **bonus** AD) physical damage and stunning them for $0.4$ seconds.
  - ***Ambessa** is displacement immune and unable to act during the cast time and while the target is suppressed, lingering for $0.1$ seconds afterwards. Drakehound's Step’s dash may be buffered during the cast time or lockout, as well as initiated within $0.275$ seconds of the lockout ending.*
  - **COOLDOWN:** 130 / 115 / 100 seconds.
  - **CAST TIME:** $0.55$ seconds, **RANGE:** er 1250, **WIDTH:** er 65.

## Trivia

- Ambessa is the second champion to be revealed for League of Legends, Wild Rift, and Legends of Runeterra around the same time, after Akshan.
  - To date, Ambessa is the only champion that has been revealed to be announced in all games within the League of Legends IP (Intellectual Property), including Teamfight Tactics.
- Ambessa is the first Ionia champion to use energy.
- With Mel's release, Ambessa is the first female champion to be a mother of another champion.
  - There are playable champions which are mothers, but not of other playable champions, such as Rek'Sai.
  - There are also champions which have known mothers, such as Smolder and Nunu, but their mothers are not playable champions themselves.
- Her buff flavortext reads:
  - *Medarda Maxim* (Drakehound's Step stacks): *"Don't pity the meek" — Medarda Family Motto*
  - *D I S R E S P E C T* (Sundering Slam ready): *"Find out what it means to me." — Ambessa Medarda*
  - *Antici—* (Repudiation steadying): *—pation*
  - *Unfazed* (Repudiation shield): *"Some say the best defense is a good offense. Why pick when I have both." — Ambessa Medarda*
  - *Pathetic* (Lacerate slow): *"You're a third rate fighter with fourth rate technique." — Ambessa Medarda* [sic]
    - This is a reference to Seto Kaiba's insult to Joey Wheeler in the *Yu-Gi-Oh* anime.
  - *Always Win Your Battles* (Public Execution successful cast): *"Defeat earns you nothing, I simply win." — Ambessa Medarda*
  - *Some Advice* (Public Execution suppression): *"Don't be sorry, be better." — Ambessa Medarda*
    - This is similar to Kratos' quote in *God of War (2018 video game)* video game. (*Do not be sorry, be better.*)
  - Killing an enemy champion grants her a cosmetic buff that reads: *"For you, the day the Medardas graced your village was the most important day of your life. But for me, it was Tuesday."*
    - This quotes M. Bison in *Street Fighter (1994)*, referring to himself instead.
  - Being killed by an enemy Mel grants her the *Took Long Enough* cosmetic buff that reads: *Mel has killed Ambessa. 'This is a new feeling. Pride in someone else. Finally.' - Ambessa Medarda*
    - This is a reference to DragonBall Z Abridged's Vegeta's Unyielding Rage.

---
*This page was automatically generated from League of Legends Wiki data.*