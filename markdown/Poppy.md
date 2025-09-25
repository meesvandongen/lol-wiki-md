# Poppy

## Table of Contents
- [Basic Information](#basic-information)
- [Statistics](#statistics)
- [Abilities](#abilities)
- [Patch History](#patch-history)
- [Trivia](#trivia)

## Basic Information

| Attribute | Value |
|-----------|------:|
| **Champion** | Poppy |
| **Title** | Keeper of the Hammer |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2010-01-13 |
| **Release Patch** | V1.0.0.70 |
| **Latest Changes** | V25.15 |
| **Roles** | Warden |
| **Riot Positions** | Top, Jungle |
| **External Positions** | Top, Jungle |
| **Blue Essence** | 225 |
| **Riot Points** | 260 |
| **Difficulty** | 2 |
| **Hero Type** | Tank |
| **Alt Type** | Fighter |
| **Adaptive Type** | Physical |
| **Damage** | 2 |
| **Toughness** | 3 |
| **Control** | 3 |
| **Mobility** | 2 |
| **Utility** | 1 |
| **Style** | 75 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $610.0$ | $+110.0$ |
| **Mana** | $280.0$ | $+40.0$ |
| **Health Regen** | $8.0$ | $+0.8$ |
| **Mana Regen** | $7.0$ | $+0.7$ |
| **Armor** | $35.0$ | $+5.0$ |
| **Magic Resist** | $32.0$ | $+2.05$ |
| **Attack Damage** | $60.0$ | $+4.0$ |
| **Attack Speed** | $0.658$ | |
| **Movement Speed** | $345.0$ | $+0.0$ |
| **Attack Range** | $125.0$ | $+0.0$ |
| **Base Attack Speed** | $0.658$ | |
| **Attack Speed Ratio** | $0.625$ | |
| **Bonus AS per Level** | $2.5\%$ | |
| **Attack Windup** | $23.4\%$ | |
| **Acquisition Radius** | $400$ units | |
| **Gameplay Radius** | $55$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $100$ units | |
| **Selection Height** | $115$ units | |
| **Critical Damage** | $175.0\%$ | |

## Abilities

### Passive: Iron Ambassador

| Attribute | Value |
|-----------|------:|
| **Speed** | 1600 (Both attack missile and on-kill return missile speeds) units/second |
| **Static Cooldown** | 16–8@1–13 |
| **Targeting** | Passive |
| **Affects** | Self |
| **Damage Type** | Magic |
| **Spell Shield** | False |
| **Spell Effects** | Proc |
| **Projectile** | True |
| **Parry** | True |

**INNATE:** Periodically, **Poppy**’s next basic attack is empowered to throw her buckler, gaining 350 **bonus** range and dealing 20 to 180 **bonus** magic damage. After it hits, the buckler then falls to a location near the target over 1 second, landing and remaining there for up to 4 seconds.

**Poppy** can move over the buckler to retrieve it, gaining a shield for (health) 11 to 20 of her **maximum** health for 3 seconds. If the buckler kills the target, or if the target is already dead when the buckler hits it, the buckler will bounce back to **Poppy** instead of falling. Enemy champions can move over the buckler to destroy it.

**Notes:**

- *Iron Ambassador*’s buckler attack counts as melee.

---

### Q: Hammer Shock

| Attribute | Value |
|-----------|------:|
| **Range** | 460 / -120 (Edge range, but behaves very weirdly. These values are for targets of 65 radius) units |
| **Cast Time** | $0.3325$ seconds |
| **Width** | 160 units |
| **Cost** | 35 / 40 / 45 / 50 / 55 Mana |
| **Cooldown** | 8 / 7 / 6 / 5 / 4 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | Special |
| **Spell Effects** | spellaoe |
| **Projectile** | False |

**ACTIVE:** **Poppy** smashes the area in the target direction, dealing physical damage to enemies hit. Against minions and monsters, the damage based on their health ratio is capped.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 30 / 55 / 80 / 105 / 130 (+ 100% **bonus** AD) (+ 9% of target's **maximum** health) |

| Attribute | Value |
|-----------|------:|
| **Capped Minion/Monster Health Damage** | 75 / 105 / 135 / 165 / 195 |
| **Maximum Minion/Monster Damage** | 105 / 160 / 215 / 270 / 325 (+ 100% **bonus** AD) |

The impact creates a field for 1 second that slows enemies within, which then ruptures to deal the same physical damage.

| Attribute | Value |
|-----------|------:|
| **Slow** | 20 / 25 / 30 / 35 / 40% |

| Attribute | Value |
|-----------|------:|
| **Total Physical Damage** | 60 / 110 / 160 / 210 / 260 (+ 200% **bonus** AD) (+ 18% of target's **maximum** health) |
| **Total Maximum Minion/Monster Damage** | 210 / 320 / 430 / 540 / 650 (+ 200% **bonus** AD) |

**Notes:**

- Spell shields will block only a single instance of damage. Effect at cast time end

---

### W: Steadfast Presence

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 400 units |
| **Cost** | 50 Mana |
| **Cooldown** | 20 / 18 / 16 / 14 / 12 seconds |
| **Targeting** | Auto |
| **Affects** | Self |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | aoe |

**PASSIVE - STUBBORN TO A FAULT:** **Poppy** increases her (armor) **total** armor and (mr) **total** magic resistance by 12%, doubled to 24% while she is below 40% **maximum** health.

**ACTIVE:** **Poppy** gains (ms) 40% **bonus** movement speed and creates an aura around herself for 2 seconds, causing all enemies who attempt to dash into or within it to be dealt magic damage and knocked up for $0.5$ seconds. If a target was successfully interrupted, they become grounded and slowed by 25% for 2 seconds.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 70 / 110 / 150 / 190 / 230 (+ 70% AP) |

*Steadfast Presence* can only block a single dash per enemy per cast.

**Notes:**

- *Stubborn to a Fault*’s armor amplification stacks with an instance of recursion.
- *Steadfast Presence* does not stop dashes if the unit is crowd control immune, displacement immune, untargetable, or protected by spell shield in the process (e.g. Death Mark, Stormbringer, Unstoppable Force).
  - Dashes when the unit is crowd control immune or displacement immune still trigger the aura and take the damage, but do not get knocked up and thus also never become grounded and slowed.
  - Spell shields deny all the effects if *Steadfast Presence* is triggered by their holder, and are consumed in the process.
  - Being untargetable prevents *Steadfast Presence* from triggering, but this does not prevent it from triggering against another, targetable dash.
- *Steadfast Presence* does not trigger against lunges.

---

### E: Heroic Charge

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 475 units |
| **Speed** | 1800 units/second |
| **Cost** | 70 Mana |
| **Cooldown** | 14 / 13 / 12 / 11 / 10 seconds |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | spell |
| **Call For Help** | True |
| **Grounded** | True |
| **Knockdown** | True |

**ACTIVE:** **Poppy** dashes to the target enemy's location. If they are in range upon arrival, she deals physical damage and carries them along with her for up to 400 units.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 40 / 60 / 80 / 100 / 120 (+ 60% **bonus** AD) |

If the target hits terrain, she stops to deal the same physical damage again and stuns them for a duration.

| Attribute | Value |
|-----------|------:|
| **Stun Duration** | 1.6 / 1.7 / 1.8 / 1.9 / 2 seconds |

| Attribute | Value |
|-----------|------:|
| **Total Physical Damage** | 80 / 120 / 160 / 200 / 240 (+ 120% **bonus** AD) |

**Notes:**

- **Poppy** will be ordered to basic attack the target.
- *Heroic Charge* can apply spell effects twice (once when **Poppy** hits her target and once she stuns them against a wall).
  - Both instances of damage and crowd control are all the same cast instance. Effects that only trigger once per spell cast will not trigger twice.
- **Poppy** dashes slightly less far than the distance she pushes the target.
- *Heroic Charge* can interact with player-generated terrain.
- The dash does not follow targets. The target's position at the time of *Heroic Charge*’s cast is the direction **Poppy** will dash towards.
  - **Poppy** does not carry nor deal damage to the target if they have left a certain radius before she collides with them.

---

### R: Keeper's Verdict

| Attribute | Value |
|-----------|------:|
| **Range** | 500 (Uncharged range) / cr 850 / 1025 / 1200 units |
| **Cast Time** | $0.25$ (Uncharged release) / $0.35$ (Charged release) seconds |
| **Effect Radius** | 180 (Uncharged radius around Poppy) / 225 (Eruption radius upon missile collision) units |
| **Width** | 180 (Both uncharged area and shockwave missile width) units |
| **Speed** | 2500 (Shockwave missile speed) units/second |
| **Cost** | 100 Mana |
| **Cooldown** | 140 / 130 / 120 / 110 / 100 seconds |
| **Cooldown Start** | post-effect |
| **Targeting** | Auto / Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |
| **Projectile** | True |
| **Silence** | True |

**ACTIVE:** **Poppy** charges while being slowed by 15% for up to 4 seconds to increase *Keeper's Verdict*’s range and knock back distance over $0.5$ seconds after the first $0.5$ seconds of the channel. *Keeper's Verdict* can be recast within the duration.

**RECAST:** **Poppy** releases the charge to launch her hammer upwards, dealing physical damage to enemies nearby and in an area in front of her and knocking them up for 1 second.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 100 / 150 / 200 (+ 45% **bonus** AD) |

If *Keeper's Verdict* was charged for at least $0.5$ seconds, it deals 100% increased damage and enemies hit are knocked back up-to 3400 units (Scales up to this value over the first 1 second of the channel) toward the enemy team's fountain, during which they are revealed and rendered untargetable. Additionally, if no enemies are hit in front (Same as uncharged radius) of **Poppy**, she sends a shockwave that travels in the target direction until it collides with an enemy champion, which causes a hammer to erupt. Targets hit by the shockwave or the eruption receive the charged effects.

| Attribute | Value |
|-----------|------:|
| **Increased Damage** | 200 / 300 / 400 (+ 90% **bonus** AD) |

If the charge is interrupted or completes without reactivation, *Keeper's Verdict* is cancelled and the ability is put on a (cd) 15-second cooldown.

**Notes:**

- During the Recast's cast time, **Poppy**’s Flash is sealed.
- The erupting hammer of the charged cast that sends out a missile will be centered on the first champion struck, not at the location the missile collided with them.
- Enemies within the enemy team's fountain (at least 400 units away from the center of the fountain (the respawning point)) that are hit by a *Keeper's Verdict* that would knock them back are instead knocked up for $0.75$ seconds and not rendered untargetable, like the uncharged effect. They still receive the full damage.
- *Keeper's Verdict* does not destroy in-flight projectiles for enemies hit if they are rendered untargetable.
- The knock back will only displace enemies as far as they can go (e.g. to the furthest current spot available of Realm of Death’s and The Hextech Ultimatum’s borders and to the corner of the fountain).
  - The displacement's duration will remain unchanged regardless of if the knock back cannot achieve its full distance due to the affected target being restricted by boundaries they are unable to cross. In order for this to be possible, the speed of the knock back is decreased (based on where the actual landing point is) to match the amount of distance covered to time spent displaced.
- *Keeper's Verdict*’s airborne debuff is non-dispellable and thus cannot be removed by cleansing effects.
  - The debuff will remove itself automatically when the forced movement ends.
- The following table refers for interactions while **Poppy** is channeling:

#### Channel Behavior (charge)

| Aspect | State / Notes |
|--------|---------------|
| **Attacking** | Disabled |
| **Movement** | Allowed |
| **Abilities** | Disabled |
| **Items** | Interrupts / Allowed / Disabled |
| **Summoner Spells** | Allowed / Interrupts / Recasts |

#### Channel Behavior (cast)

| Aspect | State / Notes |
|--------|---------------|
| **Summoner Spells** | Allowed / Disabled |

---

## Patch History

### V25.15
- Hammer Shock
  - **UNDOCUMENTED / BUG FIX:** Tooltip now correctly mentions that the health ratio damage cap also applies to minions and not only monsters.

### V25.09
- Steadfast Presence
  - **Bug Fixes:** No longer is able to activate the blocking/grounding effect on enemy targets landing via Teleport / Unleashed Teleport.

### V25.06
- Hammer Shock
  - Health ratio damage cap per hit against minions and monsters increased to 75 / 105 / 135 / 165 / 195 from 50 / 80 / 110 / 140 / 170.

### V25.05
- Stats
  - Health growth increased to 110 from 104.
- Iron Ambassador
  - Shield health ratio changed to 11 to 20 **maximum** health from 13%–18%@1–13. *Now scales with each level.*
- Heroic Charge
  - Base damage per hit reduced to 40 / 60 / 80 / 100 / 120 from 50 / 70 / 90 / 110 / 130.
  - Bonus AD ratio per hit increased to 60% **bonus** AD from 50%.
- Keeper's Verdict
  - **Bug Fixes:** If Aftershock is equipped and ready, no longer fails to cast and consumes the cooldown when buffering the ability during Heroic Charge.

### V14.24
- Stats
  - Base armor reduced to 35 from 38.
  - Armor growth increased to 5 from $4.7$
- Hammer Shock
  - Base damage changed to 30 / 55 / 80 / 105 / 130 from 40 / 60 / 80 / 100 / 120.
  - Bonus AD ratio increased to 100% **bonus** AD from 90%.

### V14.22
- Stats
  - Base attack damage reduced to 60 from 64.
  - Base attack speed increased to $0.658$ from $0.625$.

### V14.20
- Iron Ambassador
  - Cooldown increased to 16–8@1–13 from 13–7@1–13.
- Hammer Shock
  - Health ratio damage cap per hit against minions and monsters increased to 50 / 80 / 110 / 140 / 170 from 30 / 60 / 90 / 120 / 150.
- Heroic Charge
  - Base damage reduced to 50 / 70 / 90 / 110 / 130 from 60 / 80 / 100 / 120 / 140.

### V14.9
- Stats
  - Selection radius increased to 100 units from 95.

### V14.2
- Iron Ambassador
  - **New Effect:** Buckler now bounces back to her even if the target dies while the missile is in flight.

### V13.8
- Hammer Shock
  - Health ratio increased to 9% of target's **maximum** health from 8%.
- Steadfast Presence
  - Increased resistances increased to 12% from 10%.

## Trivia

- Poppy was the first champion released in 2010.
- Poppy is the first and so far only post-launch champion to be priced at on release (all other champions with the same price were already present when the game officially launched in October 2009).
- Poppy is one of two champions that went into the weekly champion rotation on the day of their release. The other one is Udyr.
- Of all champion updates in the game, Poppy has gone the longest without an update to her base splash art prior to her VGU in 2015.
- Poppy is the first champion with full facial animation.
- Poppy is the first champion to receive updated splash artwork for all of her skins.
  - Future champion reworks/updates have since followed this tradition.
- The splash art features the titular Poppy.
- Poppy's theme resembles Demacia Rising, (instruments used include a Brass instrument and a flute).
- Poppy's dance references the dance from *Futurama*.
  - A side-by-side comparison can be seen here.
- Poppy was initially going to wear a red scarf as a 'heroic thing', it competed heavily with her pigtails and was deemed way too busy on her model, the scarf was trimmed down to a muffler.
- Poppy's favorite food / candy is the lollipop.
- Poppy finds Yordles 'too fluffy' and disorganized, herself preferring the orderliness found in Demacia.
- Poppy is the first champion to have a form of crowd control in all of her abilities.
- Poppy is the first and only champion to have two skins released for the Snowdown Showdown event.

---
*This page was automatically generated from League of Legends Wiki data.*