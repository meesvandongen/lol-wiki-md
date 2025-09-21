# Fizz

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
| **Champion** | Fizz |
| **Title** | the Tidal Trickster |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2011-11-15 |
| **Release Patch** | V1.0.0.129 |
| **Roles** | Assassin |
| **Riot Positions** | Middle |
| **External Positions** | Middle |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $640.0$ | $+106.0$ | $2442.0$ |
| **Mana** | $317.0$ | $+52.0$ | $1201.0$ |
| **Health Regen** | $8.0$ | $+0.7$ | $19.9$ |
| **Mana Regen** | $6.0$ | $+0.8$ | $19.6$ |
| **Armor** | $22.0$ | $+4.6$ | $100.2$ |
| **Magic Resist** | $32.0$ | $+2.05$ | $66.8$ |
| **Attack Damage** | $58.0$ | $+3.0$ | $109.0$ |
| **Attack Speed** | $0.658$ | $+3.1\%$ | $1.005$ |
| **Movement Speed** | $335.0$ | $+0.0$ | $335.0$ |
| **Attack Range** | $175.0$ | $+0.0$ | $175.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.658$ |
| **Attack Speed Ratio** | $0.658$ |
| **Bonus AS per Level** | $3.1\%$ |
| **Acquisition Radius** | $400 units$ |
| **Gameplay Radius** | $55 units$ |
| **Pathing Radius** | $30 units$ |
| **Selection Radius** | $108.333 units$ |
| **Selection Height** | $138.889 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Nimble Fighter

**Innate:** **Fizz** becomes permanently ghosted and takes slightly reduced damage from all hits.

**Innate:** **Fizz** is permanently ghosted and reduces every instance of incoming damage by 4 (+ 1% AP), up to a maximum of 50% reduction.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Self |

**Notes:**

- No additional notes.

---

### Q: Urchin Strike

**Active:** **Fizz** dash through the target enemy, dealing magic damage, physical damage, and applying on-hit effects.

**Active:** **Fizz** dashes a fixed distance in the direction of the target enemy. If they are in range upon arrival, he deals magic damage plus 100% AD physical damage and applies on-hit effects. **Seastone Trident* and *Chum the Waters* can be cast during the dash.*

| Attribute | Value |
|-----------|-------|
| **Range** | 550 units |
| **Cooldown** | $8-6$ seconds |
| **Cast Time** | none |
| **Cost** | 50 Mana |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Physical Magic |
| **Spell Shield** | True |
| **Spell Effects** | spell |

**Scaling:**
- **Magic Damage:** $10-70$ (+ 55% AP)

**Notes:**

- *Urchin Strike* can only go through walls if the target is on the other side.
- *Urchin Strike* only damages targets if within 200 units of their original location upon completion of the dash.
- **Fizz** will be automatically ordered to basic attack the target after *Urchin Strike*.
- *Urchin Strike* does not deal damage if the target is untargetable.
- applies only on the physical damage component. Spell vamp applies to both the physical damage and bonus magic damage.

---

### W: Seastone Trident

**Passive:** **Fizz**’s basic attacks bleed his enemies, continually dealing magic damage.

**Active:** **Fizz** empowers his next basic attack to deal **bonus** magic damage, and has an additional effect based on fatality. * If this attack kills the target, the cooldown is ah and the *mana cost* is refunded. * If this attack does not kill the target, ''Fizz's** basic attacks deal **bonus'' magic damage for a few seconds.

**Passive:** ''Fizz's' basic attacks rend enemies on-hit, dealing magic damage every $0.5$ seconds over 3 seconds, refreshing on subsequent hits. **Active:** **Fizz** empowers his next basic attack within 4 seconds to have an uncancellable windup, gain range*bonus** range*, and deal **bonus** magic damage. If *Seastone Trident* kills its target, the *cooldown* is reduced to 1 second and **Fizz** restores *mana*. Otherwise, if the target is not killed, ''Fizz's** basic attacks deal **bonus'' magic damage on-hit for the next 5 seconds. *Seastone Trident* deals 50% damage to structures. *Seastone Trident basic attack reset *'Fizz's* basic attack timer.*

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $7-3$ seconds |
| **Cast Time** | none |
| **Cost** | $30-70$ Mana |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Special |

**Scaling:**
- **Total Magic Damage:** $20-60$ (+ 30% AP) Magic Damage per Tick $20/6-60/6 round=2$ (+ $30/6
- **Bonus Magic Damage:** $50-130$ (+ 45% AP)
- **Mana Restored:** $30-70$
- **Bonus Magic Damage On-Hit:** $10-30$ (+ 35% AP)

**Notes:**

- Applies spell damage on the empowered attack, persistent damage on the damage over time effect, and proc damage on the bonus damage on-hit.
- 'Seastone Trident's ' interactions with dodge, block, and blind effects.
  - The passive bonus damage gets negated by dodge or block the attack, or if **Fizz** is blind.
  - The active: ** If the attacked enemy is dodge or if Fizz*** is blind, the empowered attack won't deal any damage, but the empowered attack won't be consumed. *** If the attacked enemy is block, the empowered attack won't deal any damage despite being consumed. But the attack will apply the passive bonus damage of *Seastone Trident*.
- The empowered attack will trigger against structures.
- The damage over time effect is not applied to inhibitors.

---

### E: Playful

**Active:** **Fizz** dash in the target direction and balances onto his spear, becoming untargetable. From this position, Fizz can either slam the ground or choose to jump again with **Trickster**.

*Slamming the ground deals magic damage and slow nearby enemies.*

**Active:** **Fizz** dash to the target location while becoming untargetable, balancing on his trident for $0.75$ seconds, during which he can cast **Trickster** after $0.15$ seconds into the duration. He then begins to hop off of his trident over $0.5$ seconds, afterwards becoming targetable and landing to create a splash that deals magic damage to nearby enemies and slow them for 2 seconds. *While hopping off of the trident, **Fizz** may input movement commands to direct his landing.*

| Attribute | Value |
|-----------|-------|
| **Range** | 400 units |
| **Cooldown** | $16-8$ seconds |
| **Cast Time** | none |
| **Cost** | $75-95$ Mana |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | 375 units |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |

**Scaling:**
- **Magic Damage:** $80-280$ (+ 95% AP)
- **Slow:** $40-60$%

**Notes:**

- *Playful* cannot be used for approximately $0.5$ seconds after using *Urchin Strike*.
- Flash can be used while hopping off the trident, but not while balanced on it.
- **Fizz** can receive while aggro hopping off the trident, but not while balanced on it
- If the first dash is interrupted, **Fizz** initiates the hop immediately, and if the second one is, the splash will not occur.
  - In either cases, **Fizz** loses the untargetability.
- If *Playful* is cast while under the effect of a move block, **Fizz** will not become untargetable and will initiate the hop instantly.
  - If *Trickster* is cast, the splash will not occur.
- There is a minimum dash distance of 25 units for *Playful*.
  - If **Fizz** casts *Playful* on top of himself after being teleported (e.g. Flash, Recall), the minimum dash distance will not apply.
- The distance covered during 'Playful's* directed splash is equal to half of *'Fizz's' movement speed.
- If **Fizz** enters stasis (buff) during either *Playful* or *Trickster*, the splash occurs after the stasis ends.

---

### E: Trickster

**Active:** **Fizz** dash in the target direction and slams down, dealing magic damage to nearby enemies. This area is smaller than **Playful**.

**Active:** **Fizz** dash to the target location, hopping down and splashing onto the ground prematurely upon arrival to deal the same magic damage in a smaller radius but not applying the slow.

| Attribute | Value |
|-----------|-------|
| **Range** | 300 units |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | 225 units |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |

**Scaling:**
- **Magic Damage:** $80-280$ (+ 95% AP)

**Notes:**

- *Trickster* does not count as an ability activation.
- Both *Playful*’s and 'Trickster's dash may be directed separately.
- *Trickster* has a minimum dash distance of 75 units.
- Flash cannot be used during the dash.
- The splash occurs immediately if **Fizz** encounters terrain that cannot be dashed through (he has to be particularly close to the wall).

---

### R: Chum the Waters

**Active:** **Fizz** tosses a lure in the target direction that lands on the ground, attracting a shark that erupts after a short delay, dealing magic damage to nearby enemies, airborne and slow them. The farther the lure flies, the larger the shark that is attracted, increasing the effects.

*The lure attaches to the first enemy champion that touches it, slow them. The shark will airborne the target instead of knocking them back.*

**Active:** **Fizz** throws down a lure at the target location that attracts a shark, granting sight of the area before it emerges to chomp at the lure after 2 seconds, dealing magic damage to nearby enemies, airborne, though not through terrain, and slow them for 2 seconds. The further the lure travels in its initial flight, the larger the shark that is attracted; increasing 'Chum the Waters' damage, slow, eruption radius and knock back distance. **Guppy (<455):** 40% slow, 200 eruption radius and 150 unit knock back distance. **Chomper (455-910):** 60% slow, 325 eruption radius and 250 unit knock back distance. **Gigalodon (>910):** 80% slow, 450 eruption radius and 350 unit knock back distance. Enemy champion can intercept the lure while it is in flight, which attaches to them upon contact and causes the shark to emerge at their position after the same delay. The lure's holder is slow and true sight for the duration and afterwards is impacted by the eruption but is airborne for 1 second instead of knocked back.

| Attribute | Value |
|-----------|-------|
| **Range** | 1300 units |
| **Cooldown** | $100-70$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | 100 Mana |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Speed** | 1300 units/second |
| **Spell Shield** | Special |
| **Spell Effects** | spellaoe |
| **Projectile** | Special |

**Scaling:**
- **Guppy Damage:** $150-350$ (+ 80% AP)
- **Chomper Damage:** $225-425$ (+ 100% AP)

**Notes:**

- *Chum the Waters* will not attach the lure to an enemy champion if they have a spell shield as well as against an enemy **Malzahar** with his *Void Shift* active (mitigation effect is consumed in both cases).
  - The area of effect for the eruption will be at the target's location at the time of the attachment being blocked; the target will intercept the lure without getting attached to it.
- 'Chum the Waters' lure missile will stop its movement upon colliding with Blade Whirl or Wind Wall specifically, creating the area of effect for the eruption prematurely without destroying the missile. Collision with Rebuttal will destroy it altogether, however. Effect at cast time end
  - The targeted area does not change unless the ability was cast beyond maximum range.
- If the lure attaches to an enemy champion, a countdown will appear at their position to signify when the shark is about to emerge. The countdown begins from 4, and ticks down by one every $0.5$ seconds.

---

## Patch History

### V25.14
- *Seastone Trident*
  - Cooldown reduced to $7-3$ seconds from $7-5$.
- *Playful* / *Trickster*
  - AP ratio increased to 95% AP from 90% AP.

### V14.22
- *Urchin Strike*
  - AP ratio increased to 55% AP from 45% AP.
- *Seastone Trident*
  - Active AP ratio increased to 45% AP from 40% AP.

### V14.2
- *Urchin Strike*
  - AP ratio reduced to 45% AP from 50% AP.
- *Seastone Trident*
  - Damage over time AP ratio per tick reduced to 5% AP from $6.67$% AP.
    - Total damage over time AP ratio reduced to 30% AP from 40% AP.
  - **New Effect:** Damage is now applied to structures at 50% damage.
    - Includes empowered attack, damage over time, and on-hit effects.
      - Damage over time is not applied to inhibitors.
- *Urchin Strike*
  - AP ratio reduced to 50% AP from 55% AP.
- *Seastone Trident*
  - Active AP ratio reduced to 40% AP from 50% AP.

### V14.1
- *Chum the Waters*
  - **Bug Fixes:** Water VFX now shows for the player.

### V13.5
- *Seastone Trident*
  - Mana restore increased to $30-70$ from $20-52$.
- *Playful*
  - Base damage increased to $80-280$ from $70-270$.
  - Mana cost reduced to $75-95$ from $90-110$.

### V13.4
- *Urchin Strike*
  - **Bug Fixes:** Now properly applies spell effects even while *Seastone Trident* wasn't learned yet.

### V12.19
- *Playful*
  - AP ratio increased to 90% AP from 75% AP.

### V12.17
- *Playful*
  - **Bug Fixes:** Range indicator is now properly shown.

### V12.16
- General
  - Updated visual effects.
- Fizz
  - Complete overhaul.
- Fizz, Fizz, Fizz
  - Same as new base, but with new effects for *Seastone Trident* and *Chum the Waters*.
- Fizz
  - Complete overhaul.
- Fizz
  - Complete overhaul.
- Fizz, Fizz, Fizz, Fizz, Fizz
  - New passive marker for *Seastone Trident*.

### V12.11
- Stats
  - Health growth reduced to 106 from 112.
  - Mana growth increased to 52 from 37.

## Trivia

- If *Chum the Waters* deals the killing blow to a champion with a small model (like or a Yordle) their corpse is eaten and not visible on the map (doesn't work on for some reason)
- Seastone Trident is seen alongside other champion weapons in the game's Mac Version trailer.

---
*This page was automatically generated from League of Legends Wiki data.*