# Poppy

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
| **Champion** | Poppy |
| **Title** | Keeper of the Hammer |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2010-01-13 |
| **Release Patch** | V1.0.0.70 |
| **Roles** | Warden |
| **Riot Positions** | Top, Jungle |
| **External Positions** | Top, Jungle |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $610.0$ | $+110.0$ | $2480.0$ |
| **Mana** | $280.0$ | $+40.0$ | $960.0$ |
| **Health Regen** | $8.0$ | $+0.8$ | $21.6$ |
| **Mana Regen** | $7.0$ | $+0.7$ | $18.9$ |
| **Armor** | $35.0$ | $+5.0$ | $120.0$ |
| **Magic Resist** | $32.0$ | $+2.05$ | $66.8$ |
| **Attack Damage** | $60.0$ | $+4.0$ | $128.0$ |
| **Attack Speed** | $0.658$ | $+2.5\%$ | $0.938$ |
| **Movement Speed** | $345.0$ | $+0.0$ | $345.0$ |
| **Attack Range** | $125.0$ | $+0.0$ | $125.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.658$ |
| **Attack Speed Ratio** | $0.625$ |
| **Bonus AS per Level** | $2.5\%$ |
| **Attack Windup** | $23.4\%$ |
| **Acquisition Radius** | $400 units$ |
| **Gameplay Radius** | $55 units$ |
| **Pathing Radius** | $35 units$ |
| **Selection Radius** | $100 units$ |
| **Selection Height** | $115 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Iron Ambassador

**Innate:** Periodically, **Poppy** projectile her buckler as her next basic attack, which has **bonus range** and deals **bonus** magic damage. The buckler then falls nearby and remains for a few seconds, which she and enemy champions can step over to either retrieve or destroy it.

*If this attack kills the target, the buckler bounces back to Poppy instead of falling.*

**Innate:** Periodically, ''Poppy's** next basic attack is empowered to throw her buckler, gaining *350 **bonus** range* and dealing 20 to 180 **bonus'' magic damage. After it hits, the buckler then falls to a location near the target over 1 second, landing and remaining there for up to 4 seconds. **Poppy** can move over the buckler to retrieve it, gaining a shield for health*maximum** health* for 3 seconds. If the buckler kills the target, or if the target is already dead when the buckler hits it, the buckler will bounce back to **Poppy** instead of falling. Enemy champion can move over the buckler to destroy it.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Self |
| **Damage Type** | Magic |
| **Speed** | 1600 units/second |
| **Spell Shield** | False |
| **Spell Effects** | Proc |
| **Projectile** | True |

**Notes:**

- 'Iron Ambassador's buckler attack counts as melee.

---

### Q: Hammer Shock

**Active:** **Poppy** smashes the area in the target direction, dealing physical damage to enemies hit based on their **maximum** health.

*The impact creates a brief field that slow enemies within, then ruptures to deal the same physical damage again.*

**Active:** **Poppy** smashes the area in the target direction, dealing physical damage to enemies hit. Against minions and monsters, the damage based on their health ratio is capped. The impact creates a field for 1 second that slow enemies within, which then ruptures to deal the same physical damage.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $8-4$ seconds |
| **Cast Time** | $0.3325$ seconds |
| **Cost** | $35-55$ Mana |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | Special |
| **Spell Effects** | spellaoe |
| **Projectile** | False |

**Scaling:**
- **Physical Damage:** $30-130$
- *bonus AD) (+ 9% of target's
- **maximum** health) **Capped Minion/Monster Health Damage:* $75-195$bonus AD)
- **Slow:** $20-40$%
- **Total Physical Damage:** $30×2-130×2$ (+ $100×2$%
- *bonus AD) (+ 18% of target's **maximum* health)2-(130+195)*2$ (+ $100×2$% bonus AD)

**Notes:**

- Spell shield will block only a single instance of damage. Effect at cast time end

---

### W: Steadfast Presence

**Passive:** **Poppy** increases her *armor **total** armor* and *mr **total** magic resist*. This effect is doubled while she is at low health.

**Active:** **Poppy** gains *ms **bonus** movement speed* for a short time and creates an aura that stops enemy dashes. Enemies that dash within the aura are dealt magic damage and briefly airborne, they are then grounded and slow for a short time.

**Passive - Stubborn to a Fault:** **Poppy** increases her *armor **total** armor* and *mr **total** magic resistance* by 12%, doubled to 24% while she is below 40% **maximum** health. **Active:** **Poppy** gains ms*bonus** movement speed* and creates an aura around herself for 2 seconds, causing all enemies who attempt to dash into or within it to be dealt magic damage and airborne for $0.5$ seconds. If a target was successfully interrupted, they become grounded and slow by 25% for 2 seconds. *Steadfast Presence* can only block a single dash per enemy per cast.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $20-12$ seconds |
| **Cast Time** | none |
| **Cost** | 50 Mana |
| **Targeting** | Auto |
| **Affects** | Self |
| **Damage Type** | Magic |
| **Effect Radius** | 400 units |
| **Spell Shield** | True |
| **Spell Effects** | aoe |

**Scaling:**
- **Magic Damage:** $70-230$ (+ 70% AP)

**Notes:**

- 'Stubborn to a Fault's armor amplification stacks with an instance of recursion.
- *Steadfast Presence* does not stop dash if the unit is cc-immune, displacement immune, untargetable, or protected by spell shield in the process (e.g. *Death Mark*, *Stormbringer*, *Unstoppable Force*).
  - Dashes when the unit is cc-immune or displacement immune still trigger the aura and take the damage, but do not get airborne and thus also never become ground and slow.
  - spell shield deny all the effects if *Steadfast Presence* is triggered by their holder, and are consumed in the process.
  - Being untargetable prevents *Steadfast Presence* from triggering, but this does not prevent it from triggering against another, targetable dash.
- *Steadfast Presence* does not trigger against lunge.

---

### E: Heroic Charge

**Active:** **Poppy** dashes to the target enemy, dealing physical damage and airborne them forward. If the target hits terrain, **Poppy** deals the same physical damage again and briefly stun them.

**Active:** **Poppy** dashes to the target enemy's location. If they are in range upon arrival, she deals physical damage and airborne them along with her for up to 400 units. If the target hits terrain, she stops to deal the same physical damage again and stun them for a duration.

| Attribute | Value |
|-----------|-------|
| **Range** | 475 units |
| **Cooldown** | $14-10$ seconds |
| **Cast Time** | none |
| **Cost** | 70 Mana |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Speed** | 1800 units/second |
| **Spell Shield** | True |
| **Spell Effects** | spell |

**Scaling:**
- **Physical Damage:** $40-120$ bonus AD)
- **Stun Duration:** $1.6-2$ seconds
- **Total Physical Damage:** $40×2-120×2$ (+ 120% bonus AD)

**Notes:**

- **Poppy** will be ordered to basic attack the target.
- *Heroic Charge* can apply spell effects twice (once when **Poppy** hits her target and once she stuns them against a wall).
  - Both instances of damage and crowd control are all the same cast instance. Effects that only trigger once per spell cast will not trigger twice.
- **Poppy** dashes slightly less far than the distance she pushes the target.
- *Heroic Charge* can interact with player-generated terrain.
- The dash does not follow targets. The target's position at the time of 'Heroic Charge's cast is the direction **Poppy** will dash towards.
  - **Poppy** does not airborne nor deal damage to the target if they have left a certain radius before she collides with them.

---

### R: Keeper's Verdict

**Active:** **Poppy** channels up to a few seconds, increasing the *range* and airborne distance of her hammer. After a brief period, it becomes empowered with a new effect.

**Recast:** **Poppy** swings her hammer, dealing physical damage to enemies nearby and in the area in front of her, briefly airborne.

**Active:** **Poppy** channels while being slow by 15% for up to 4 seconds to increase 'Keeper's Verdict's* *range* and airborne distance over $0.5$ seconds after the first $0.5$ seconds of the channel. *Keeper's Verdict' can be recast within the duration. **Recast:** **Poppy** releases the charge to launch her hammer upwards, dealing physical damage to enemies nearby and in an area in front of her and airborne for 1 second. If 'Keeper's Verdict' was charged for at least $0.5$ seconds, it deals 100% increased damage and enemies hit are airborne up-to 3400 units toward the enemy team's fountain, during which they are standard sight and rendered untargetable. Additionally, if no enemies are hit in front of **Poppy**, she sends a shockwave that travels in the target direction until it collides with an enemy champion, which causes a hammer to erupt. Targets hit by the shockwave or the eruption receive the charged effects. If the charge is interrupt or completes without reactivation, 'Keeper's Verdict' is cancelled and the ability is put on a cd cooldown.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $140-100$ seconds |
| **Cast Time** | $0.25$ / $0.35$ seconds |
| **Cost** | 100 Mana |
| **Targeting** | Auto / Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Speed** | 2500 units/second |
| **Effect Radius** | 180 / 225 units |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |
| **Projectile** | True |

**Scaling:**
- **Physical Damage:* $100bonus AD)
- **Increased Damage:* $200bonus AD)

**Notes:**

- During the Recast's cast time, ''Poppy's' Flash is lockout.
- The erupting hammer of the charged cast that sends out a missile will be centered on the first champion struck, not at the location the missile collided with them.
- Enemies within the enemy team's fountain (at least 400 units away from the center of the fountain) that are hit by a 'Keeper's Verdict' that would knock them back are instead airborne for $0.75$ seconds and not rendered untargetable, like the uncharged effect. They still receive the full damage.
- 'Keeper's Verdict' does not destroy in-flight projectile for enemies hit if they are rendered untargetable.
- The airborne will only displace enemies as far as they can go (e.g. to the furthest current spot available of *Realm of Death*’s and *The Hextech Ultimatum*’s borders and to the corner of the fountain).
  - The displacement's duration will remain unchanged regardless of if the knock back cannot achieve its full distance due to the affected target being restricted by boundaries they are unable to cross. In order for this to be possible, the speed of the knock back is decreased (based on where the actual landing point is) to match the amount of distance covered to time spent displaced.
- 'Keeper's Verdict's airborne debuff is non-dispellable and thus cannot be removed by cleanse effects.
  - The debuff will remove itself automatically when the forced movement ends.
- The following table refers for interactions while **Poppy** is channel:

---

## Patch History

### V25.15
- *Hammer Shock*
  - **Undocumented / Bug Fix:** Tooltip now correctly mentions that the health ratio damage cap also applies to minions and not only monsters.

### V25.09
- *Steadfast Presence*
  - **Bug Fixes:** No longer is able to activate the blocking/grounding effect on enemy targets landing via Teleport / Unleashed Teleport.

### V25.06
- *Hammer Shock*
  - Health ratio damage cap per hit against minions and monsters increased to $75-195$ from $50-170$.

### V25.05
- Stats
  - Health growth increased to 110 from 104.
- *Iron Ambassador*
  - Shield health ratio changed to key=% **maximum** health from key=%. *Now scales with each level.*
- *Heroic Charge*
  - Base damage per hit reduced to $40-120$ from $50-130$.
  - Bonus AD ratio per hit increased to 60% *bonus AD from 50%.
- *Keeper's Verdict*
  - **Bug Fixes:** If *Aftershock* is equipped and ready, no longer fails to cast and consumes the cooldown when buffering the ability during *Heroic Charge*.

### V14.24
- Stats
  - Base armor reduced to 35 from 38.
  - Armor growth increased to 5 from $4.7$
- *Hammer Shock*
  - Base damage changed to $30-130$ from $40-120$.
  - Bonus AD ratio increased to 100% *bonus AD from 90%.

### V14.22
- Stats
  - Base attack damage reduced to 60 from 64.
  - Base attack speed increased to $0.658$ from $0.625$.

### V14.20
- *Iron Ambassador*
  - Cooldown increased to 16–8@1–13 from 13–7@1–13.
- *Hammer Shock*
  - Health ratio damage cap per hit against minions and monsters increased to $50-170$ from $30-150$.
- *Heroic Charge*
  - Base damage reduced to $50-130$ from $60-140$.

### V14.9
- Stats
  - Selection radius increased to 100 units from 95.

### V14.2
- *Iron Ambassador*
  - **New Effect:** Buckler now bounces back to her even if the target dies while the missile is in flight.

### V13.8
- *Hammer Shock*
  - Health ratio increased to 9% of target's **maximum** health from 8%.
- *Steadfast Presence*
  - Increased resistances increased to 12% from 10%.

## Trivia

- Poppy was the first champion released in 2010.
- Poppy is the first and so far only post-launch champion to be priced at on release (all other champions with the same price were already present when the game officially launched in October 2009).
- Poppy is one of two champions that went into the weekly champion rotation on the day of their release. The other one is **Udyr**.
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