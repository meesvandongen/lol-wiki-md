# Rek'Sai

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
| **Champion** | Rek'Sai |
| **Title** | the Void Burrower |
| **Resource** | Rage |
| **Range Type** | Melee |
| **Release Date** | 2014-12-11 |
| **Release Patch** | V4.21 |
| **Latest Changes** | V25.17 |
| **Roles** | Diver |
| **Riot Positions** | Jungle |
| **External Positions** | Jungle |
| **Blue Essence** | 2400 |
| **Riot Points** | 880 |
| **Difficulty** | 2 |
| **Hero Type** | Fighter |
| **Alt Type** | Tank |
| **Adaptive Type** | Physical |
| **Damage** | 2 |
| **Toughness** | 2 |
| **Control** | 2 |
| **Mobility** | 2 |
| **Utility** | 2 |
| **Style** | 30 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $600.0$ | $+99.0$ |
| **Mana** | $100.0$ | $+0.0$ |
| **Health Regen** | $2.5$ | $+0.5$ |
| **Armor** | $35.0$ | $+4.5$ |
| **Magic Resist** | $32.0$ | $+2.05$ |
| **Attack Damage** | $62.0$ | $+3.0$ |
| **Attack Speed** | $0.667$ | |
| **Movement Speed** | $340.0$ | $+0.0$ |
| **Attack Range** | $175.0$ | $+0.0$ |
| **Base Attack Speed** | $0.667$ | |
| **Attack Speed Ratio** | $0.667$ | |
| **Bonus AS per Level** | $2.0\%$ | |
| **Attack Windup** | $26.7\%$ | |
| **Acquisition Radius** | $600$ units | |
| **Pathing Radius** | $30$ units | |
| **Selection Radius** | $100$ units | |
| **Selection Height** | $122$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $105.0\%$ |
| **Damage Taken** | $90.0\%$ |

#### URF

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $107.0\%$ |
| **Damage Taken** | $93.0\%$ |
| **Healing** | $120.0\%$ |

## Abilities

### Passive: Fury of the Xer'Sai

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Self |

**INNATE:** **Rek'Sai** basic attacks and ability hits generate 25 Fury. After 8 seconds of being out of combat, she loses 20 Fury per second. Fury gain is reduced「 by 80% ⟷ to 5 per hit 」against minions, wards, and plants. *Prey Seeker* will not generate Fury against non-champions.

When **Rek'Sai** becomes **BURROWED**, she consumes her current Fury over 3 seconds to heal for up to 10%–20%@1–16 **maximum** health at 100 Fury, stopping once fully consumed or she reaches (health) full health. If she starts generating Fury while **BURROWED**, the consumption occurs after 5 seconds.

**Notes:**

- Fury bar changes from to when full.
- When **Rek'Sai** automatically consumes Fury while **BURROWED** (no transformation), the healing buff will refresh every second while she has/maintains over 25 Fury. This causes the healing to last 6 seconds at 100 Fury.

---

### Q: Prey Seeker

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.1$ seconds |
| **Target Range** | 1500 (Missile range, standard end behaviour) units |
| **Effect Radius** | 150 (Around collision point) units |
| **Width** | 130 (Missile width) units |
| **Speed** | 1950 units/second |
| **Cooldown** | 10 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |
| **Projectile** | True |

**ACTIVE:** **Rek'Sai** fires a void bolt in the target direction that detonates on the first enemy hit, dealing magic damage to all nearby enemies and revealing them for 5 seconds.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 50 / 80 / 110 / 140 / 170 (+ 25% **bonus** AD) (+ 70% AP) |

**Notes:**

Effect at cast time end
- *Prey Seeker*’s effect radius is centered around the location of the missile as it collides.

---

### Q: Queen's Wrath

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 325 units |
| **Cooldown** | 4 / 3.5 / 3 / 2.5 / 2 seconds |
| **Cooldown Start** | post-effect |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | False |
| **Spell Effects** | Special |
| **Parry** | Special |

**ACTIVE:** **Rek'Sai** gains 35% **bonus** attack speed for 3 seconds. Her next basic attack within the duration will have an uncancellable windup and deal **bonus** physical damage to the target and surrounding enemies. *Queen's Wrath*’s damage to the primary target is affected by critical strike modifiers.

| Attribute | Value |
|-----------|------:|
| **Bonus Physical Damage** | 30 / 35 / 40 / 45 / 50% AD |
| **Total Bonus Physical Damage** | 90 / 105 / 120 / 135 / 150% AD |

If **Rek'Sai** completes an attack, the duration is refreshed, for up to 3 total empowered attacks.

*Queen's Wrath resets **Rek'Sai**’s basic attack timer.*

**Notes:**

- Damage to the primary target applies spell damage and damage to secondary targets applies default damage.
- *Queen's Wrath*’s bonus damage applies against structures.
- When dodged or blocked, or if **Rek'Sai** is blinded, the damage dealt to the primary target is nullified, but the damage around her will still occur.

---

### W: Burrow

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 1500 units |
| **Cooldown** | 4 (Starts after casting Unburrow) seconds |
| **Cooldown Start** | Special |
| **Targeting** | Auto |
| **Affects** | Self |

**ACTIVE:** **Rek'Sai** becomes **BURROWED**, reducing her attack range by 50 and her sight radius to 385 units, while gaining access to her **BURROWED** abilities. She also becomes ghosted and gains **bonus** movement speed, as well as *Tremor Sense*.

| Attribute | Value |
|-----------|------:|
| **Bonus Movement Speed** | 5 / 10 / 15 / 20 / 25 |

**TREMOR SENSE:** **Rek'Sai** and her allies gain obscured vision of enemies that are near her and are moving, at a rate of once every second.

Once *Burrow* has been learned, **Rek'Sai** automatically becomes **BURROWED** upon completing a Recall channel or respawning.

**Notes:**

**Burrow* does not count as an ability activation for the purposes of on-cast effects such as Spellblade and triggering Force Pulse’s passive.
- Enemies detected by *Tremor Sense* are displayed as rings on the ground. Champions display larger rings and **Rek'Sai** will react to this detection.
- Visible enemies will not be detected by Tremor Sense. Enemies standing still while **Rek'Sai** is nearby are insensible by *Tremor Sense*.

---

### W: Unburrow

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 220 (Knockup AOE radius, center from Rek'Sai to edge of target) / cr 250 (Secondary targets are knocked or pulled to this radius around Rek'Sai) / er 75 (Rek'Sai's typical attack range for the unborrow via right click) units |
| **Cooldown Start** | Special |
| **Static Cooldown** | 1 (Starts after casting Burrow) |
| **On-target CD Static** | 10 / 9 / 8 / 7 / 6 (Begins after the target is displaced) |
| **Targeting** | Auto |
| **Affects** | Self / Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |

**ACTIVE:** **Rek'Sai** becomes **UN-BURROWED**, gaining access to her **UN-BURROWED** abilities and emerging from the ground to deal magic damage to nearby enemies, knocking them up for 1 second. Minions and small monsters are instead knocked back a short distance. Targets hit cannot be displaced by *Unburrow* again for some time.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 30 / 55 / 80 / 105 / 130 (+ 80% AP) |

**Rek'Sai** can also trigger *Unburrow* by issuing an attack command, casting the ability once she is in attack range of the target and applying the knock up against them.

**Notes:**

- *Unburrow* can apply the knock up to minions and small monsters if the ability is cast against them with an attack command.
- *Unburrow* does not count as an ability activation for the purposes of on-cast effects such as Spellblade and triggering Force Pulse’s passive.
- *Unburrow* is still considered an ability and is thus stopped by silence. She can basic attack normally during this time, without leaving *unburrow*.
- For a brief moment during the beginning animation of Tunnel, **Rek'Sai** can trigger *Unburrow* by issuing an attack command.

---

### E: Tunnel

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 850 units |
| **Speed** | 500 + 100% movement speed |
| **Cooldown** | 18 / 17 / 16 / 15 / 14 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Direction |
| **Affects** | Self |
| **Terrain Grace** | True |
| **Grounded** | True |
| **Knockdown** | True |
| **On-Tunnel Cooldown (Unaffected by ability haste)** | 6 / 5 / 4 / 3 / 2 (Starts after dash ends) |

**ACTIVE:** **Rek'Sai** dashes in the target direction, leaving behind a *Tunnel* with two interconnected entrances that lasts up to 10 minutes. The *Tunnel* can be traversed again when **Rek'Sai** targets either entrance, shutting down the *Tunnel* for a few seconds.

**Rek'Sai** can have up to 8 *Tunnels* active on the map at once, but deploying beyond the maximum destroys the oldest one. Enemy champions can destroy a *Tunnel* by standing on either entrance for $1.5$ seconds, granting them 5 gold upon successfully destroying the *Tunnel*.

If **Rek'Sai** attempts to enter a *Tunnel* while **UN-BURROWED**, she will automatically *Burrow* and dash to the *Tunnel*’s selected entrance over $0.6$ seconds before entering it, and automatically *Unburrow* after exiting the *Tunnel*. This does not place *Burrow* on cooldown nor trigger *Fury of the Xer'Sai*, but does trigger other *Burrow* effects and *Unburrow*. If *Burrow* is on cooldown at the time of interacting with the *Tunnel*, the cooldown will be reset.

***Rek'Sai** cannot enter a tunnel while immobilized, grounded, or silenced.*

**Notes:**

- Creating a *Tunnel* grants sight of the area at the other end for 2 seconds.
- *Tunnel* entrances can be targeted by allied movement abilities (ex: *Leap Strike*, *Shunpo*, *Safeguard*, *Teleport*).
- *Tunnel* entrances are visible on allied champions' minimaps, while enemies must first gain sight of one before it becomes visible on theirs (the opposite entrance is not revealed).
- Destroying a *Tunnel* while **Rek'Sai** is traversing it will not prevent her from reaching the opposite entrance.
- **Rek'Sai** can *Tunnel* through terrain if she dashes close to them and the wall is thin enough to allow the opposite entrance to spawn on the other side.
- If **Rek'Sai** issues an attack command while the dash passes by an enemy, the target will be knocked up but **Rek'Sai** will still complete the movement.

---

### E: Furious Bite

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 225 units |
| **Cooldown** | 6 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | spell |
| **Call For Help** | True |

**ACTIVE:** **Rek'Sai** bites the target enemy, dealing physical damage.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 80 / 108 / 136 / 164 / 192 (+ 64% **bonus** AD) |

At maximum Fury, *Furious Bite* deals 125% damage and is converted to true damage.

| Attribute | Value |
|-----------|------:|
| **True Damage** | 100 / 135 / 170 / 205 / 240 (+ 80% **bonus** AD) |

**Notes:**

No additional notes.

---

### R: Void Rush

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 1500 units |
| **Speed** | 1400 (Dash speed) units/second |
| **Cooldown** | 120 / 110 / 100 / 90 / 80 seconds |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | spell |
| **Call For Help** | True |
| **Grounded** | True |
| **Knockdown** | False |

**PASSIVE:** **Rek'Sai** applies *Marked as Prey* to enemy champions she damages, lasting for 5 seconds.

**ACTIVE:** **Rek'Sai** singles out the target enemy champion *Marked as Prey*, becoming displacement immune and unable to act and vanishing into the ground, as well as revealing the target for 2 seconds (From the start of the cast time). After $0.9$ seconds (Estimated, from end of cast time), she blinks to a location near the target (Relative to her casting position), becoming targetable again and emerging from the ground to pounce at them. Upon arrival, she slashes at the target with her claws, dealing physical damage, and dashes 125 units through them.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 150 / 200 / 250 / 300 / 350 (+ 100% **bonus** AD) (+ 15 / 17.5 / 20 / 22.5 / 25% of target's **maximum** health) |

*If cast while in **BURROWED** form, **Rek'Sai** will automatically Unburrow at the end of Void Rush's cast time, placing Burrow on a 1-second cooldown without triggering Unburrow's effects. If Burrow is already on cooldown, its remaining cooldown is set to 1 second.*

**Notes:**

- **Rek'Sai** will track the target if they change locations while dashing to them. Once she has started to dash through the target however, she will no longer track them.
  - She will dash through the target's previous location without dealing damage if the target moves beyond 1750 units (Estimated).
- If the target moves 1000 (Estimated) or more units away right before being struck, they will not be dealt the damage.
  - In this case, the target is displaced in the moment when **Rek'Sai** starts to dash through and prepares to strike them.
- *Void Rush* will be cancelled and placed on a (cd) 5-second cooldown if the target dies or moves too far away during the cast time or while **Rek'Sai** is vanished, or if they are no longer in sight during the cast time.
  - If the ability is cancelled while **Rek'Sai** is vanished, she will reappear and emerge from the ground without performing the dash once the $0.9$-second delay has elapsed.
  - If the ability is cancelled from losing sight of the target, **Rek'Sai** will not vanish into the ground after the cast time (but still becoming untargetable) and will remain in place while unable to act for $0.5$ seconds.
  - The ability will **not** be cancelled if the target becomes untargetable during the cast time or while **Rek'Sai** is vanished.
    - She will still dash to the target even if they are untargetable.
- All sources of damage by **Rek'Sai** count for applying the mark.
- **Rek'Sai** gains the displacement immunity from the start of the cast time, and loses it once her dash ends.
- **Rek'Sai** can activate *Void Rush* either while Burrowed or Unburrowed.
  - She will always emerge as *Unburrowed*.
- If **Rek'Sai** enters resurrection during the cast time, *Void Rush* is canceled and goes on a 5 second cooldown.
  - If the *resurrection* occurs while vanished, she will emerge and pounce the target immediately.
- *Void Rush* will not deal damage if the target is untargetable by the time **Rek'Sai** reaches them.
- After vanishing and then emerging, **Rek'Sai** will leave behind a temporary, cosmetic Tunnel. She cannot interact with this *Tunnel*, and each entrance is destroyed $2.25$ seconds after being spawned from when she vanished or emerged.
- A screeching noise made by **Rek'Sai** can be heard globally after *Void Rush* is cast (*will be heard by enemies even if they don't have sight of **Rek'Sai** and/or her destination*).
- The following table refers for interactions while **Rek'Sai** is performing *Void Rush*:

#### Channel Behavior (cast)

| Aspect | State / Notes |
|--------|---------------|
| **Summoner Spells** | Disabled |

---

## Patch History

### V25.17
- Tunnel
  - **Bug Fixes:** No longer unintentionally causes her to enter combat upon casting the ability.

### V25.16
- Stats
  - Base attack damage increased to 62 from 58.
- Queen's Wrath
  - **New Effect:** If the buff would expire after starting a *Queen's Wrath* attack windup, now refreshes the buff's duration to the duration of the windup.
- Tunnel
  - **Bug Fixes:** No longer sometimes fails to exit at the designated location if cast beyond thick terrain due to incorrectly assuming there is not enough space.
    - Now dashes to a virtual spawned unit instead of the cast location, which is killed right after the dash successfully begins.
      - Due to the implementation, **Rek'Sai** is attributed the kill, which unintentionally causes her to enter combat.

### V25.15
- Stats
  - Base armor reduced to 35 from 36.
  - Armor growth reduced to $4.5$ from $4.95$.
- Unburrow
  - Target displace immunity reduced to 10 / 9 / 8 / 7 / 6 seconds from 10 at all ranks.
- Furious Bite
  - Cooldown reduced to 6 seconds from 10.
  - **New Effect:** Damage at maximum Fury is now converted to true damage.
  - Damage at maximum Fury:
    - **New:*** Deals 100 / 135 / 170 / 205 / 240 (+ 80% **bonus** AD).
    - **OLD:** Deals 100% AD (+ 8 / 9.5 / 11 / 12.5 / 14% of target's **maximum** health), capped at 75 to 400 against monsters.
  - While below maximum Fury, damage is now 80% of the maximum-Fury damage instead of explicit.
    - **New:*** Deals 80 / 108 / 136 / 164 / 192 (+ 64% **bonus** AD).
    - **OLD:** Deals 100% **total** AD.
- Void Rush
  - Base damage reduced to 150 / 250 / 350 from 150 / 300 / 450.
  - Target health ratio changed to 15 / 20 / 25% of target's **maximum** health from 25 / 30 / 35% of target's **missing** health.
  - Cooldown increased to 120 / 100 / 80 seconds from 100 / 90 / 80.
  - Cast fail cooldown reduced to 5 seconds from 10.
  - Now places Burrow on a 1-second cooldown after cast, rather than on full cooldown.

### V25.S1.1#January 9th Hotfix|V25.S1.1
- Void Rush
  - **Bug Fixes:** No longer becomes permanently invisible if she dies during the cast time while False Life is active.

### V14.12
- Queen's Wrath
  - Bonus attack speed reduced to 35% from 45%.
- Unburrow
  - Base damage reduced to 30 / 55 / 80 / 105 / 130 from 50 / 75 / 100 / 125 / 150.

### V14.7
- Fury of the Xer'Sai
  - Fury generation modifier against minions, wards, and jungle plants reduced to 20% from 50%.
    - Fury generated against minions, wards, and jungle plants reduced to 5 from $12.5$.
  - Maximum heal reduced to 10%–20%@1–16 **maximum** health from 12%–20%@1–16.
- Queen's Wrath
  - **Bug Fixes:** Now properly generates rune and item stacks (such as Conqueror, Eclipse, etc).

### V14.6
- Void Rush
  - Rescripted to no longer utilize a cast wrapper that would potentially be interruptible.
    - **Bug Fixes:** Casting no longer fails if Rek'Sai was stunned on the very next frame after casting *Void Rush*.

### V14.5#March 6th Hotfix|V14.5
- Burrow
  - **UNDOCUMENTED / NEW EFFECT:** Now reduces her attack range by 50 while burrowed.
- Unburrow
  - **Bug Fixes:** Knockup no longer fails if the target moves outside of effect range fast enough when she issues an attack.

### V14.5
- Stats
  - Attack windup reduced to 20% from $26.6667$%.
  - Critical strike attack windup reduced to 20% from $20.8333$%.
  - Attack windup modifier changed to $0.6$ from 1.
- Queen's Wrath
  - **New Effect:** Empowered attacks now have an uncancellable windup.
  - **Bug Fixes:** Now applies spell effects to its primary target.
- Unburrow
  - **New Effect:** Can now damage (but not knock up) targets who were recently knocked up and immune to *Unburrow*.
  - **Bug Fixes:** Now properly knocks up large monsters if they were not the primary target.
  - **Bug Fixes:** No longer sometimes locks her out of declaring basic attacks for some time after unburrowing.
- Furious Bite
  - Health ratio increased to 8 / 9.5 / 11 / 12.5 / 14% of target's **maximum** health from 6 / 8 / 10 / 12 / 14%.
  - Monster damage cap increased to 75 to 400 from 60 to 400.
- Tunnel
  - **Bug Fixes:** Potentially fixed movement through terrain failing upon entering a *Tunnel* created by casting this ability.
- Void Rush
  - Base damage increased to 150 / 300 / 450 from 100 / 250 / 400.
  - Health ratio increased to 25 / 30 / 35% of target's **missing** health from 20 / 25 / 30%.
  - **Undocumented:** The initial movement spell now has a 1-frame / -second cast time in order to be properly buffered with other attacks and spells, and not result in unexpectedly canceling the movement and damaging effects upon emerging.

### V14.4
- Stats
  - Base health regeneration reduced to $2.5$ from $7.5$.
  - Health regeneration growth reduced to $0.5$ from $0.65$.
  - Base movement speed increased to 340 from 335.
- Fury of the Xer'Sai
  - Maximum heal health ratio increased to 12%–20%@1–16 **maximum** health from 2%–10%@1–16.
  - **Removed:*** No longer has a maximum base heal of 10.
  - **New Effect:** Fury generation is now reduced by 50% against minions, wards, and jungle plants.
- Queen's Wrath
  - Now functions as a basic attack modifier instead of an ability. References the user's basic attack cast time.
    - **Removed:*** No longer has a $0.25$-second cast time for each empowered attack.
  - **New Effect:** Now grants 45% **bonus** attack speed while active.
  - Made cast animations smoother at high attack speeds.
- Prey Seeker
  - Cooldown reduced to 10 seconds at all ranks from 12 / 11.5 / 11 / 10.5 / 10.
  - Damage type changed to magic from physical.
  - Base damage reduced to 50 / 80 / 110 / 140 / 170 from 60 / 95 / 130 / 165 / 200.
  - Bonus AD ratio reduced to 25% **bonus** AD from 50%.
- Burrow
  - Bonus movement speed changed to 5 / 10 / 15 / 20 / 25 from 5–15@1–11.
  - Vision radius increased to 385 from 250.
  - **Removed:*** No longer reduces her attack range to 75.
- Unburrow
  - Damage type changed to magic from physical.
  - Base damage reduced to 50 / 75 / 100 / 125 / 150 from 50 / 85 / 120 / 155 / 190.
  - **Removed:*** Damage no longer scales with 80% **bonus** AD.
  - **New Effect:** Damage now scales with 80% AP.
  - **New Effect:** Now knocks up all targets hit, instead of only knocking up the primary target and knocking back all secondary targets.
    - The 10-second per-target immunity will apply to all targets hit.
  - **Removed:*** No longer slows targets by 40% for $0.5$ seconds after knocking them up.
- Furious Bite
  - Cooldown reduced to 10 seconds from 12.
  - **Removed:*** No longer has a base damage of 5 / 10 / 15 / 20 / 25.
  - AD ratio increased to 100% AD from 70% AD.
  - **Removed:*** At maximum Fury, no longer deals double damage and converts it to true damage.
  - **New Effect:** At maximum Fury, now deals 6 / 8 / 10 / 12 / 14% of target's **maximum** health **bonus** physical damage, capped at 60 to 400 against monsters.
- Tunnel
  - Cooldown reduced to 18 / 17 / 16 / 15 / 14 seconds from 26 / 24 / 22 / 20 / 18 seconds.
  - Tunnel cooldown reduced to 6 / 5 / 4 / 3 / 2 seconds from 10 / 8 / 6 / 4 / 2 seconds.
  - Manual cast dash speed reduced to 500 from 550.
  - **New Effect:** Manual cast dash speed now scales with total movement speed instead of movement speed from boots.
  - Movement speed while burrowing and unburrowing slightly increased.
  - **Bug Fixes:** Dash speed while in the *Tunnel* is now properly 850 (matching the movement command/re-use speed) instead of 550, if the *Tunnel* is created by casting this ability.
  - **Bug Fixes:** No longer has a $0.05$-second delay when entering a *Tunnel* after its creation by casting this ability.
- Void Rush
  - Bonus AD ratio reduced to 100% **bonus** AD from 175%.

## Trivia

- Rek'Sai is the first female Voidborn released.
- Rek'Sai is the first fighter champion released without a secondary role.
- Rek'Sai's design was greatly influenced by the canceled champion Omen.
  - Just like with Cho'Gath, Malphite, Nautilus, and Skarner, Rek'Sai's in-game is smaller than in-universe (she is estimated to be as large as a 'small Tyrannosaurus, mammoth, whale shark, large but not huge').
- Rek'Sai had her game assets reused for several featured game modes.
  - She was given a retexture and features as a monster called the "Grumpy Burrow Monster" (along with its huge variant) in Invasion.
  - Her animations were reused for the Rek'Sai, Rek'Sai, and Rek'Sai monsters in Odyssey: Extraction.
  - Rek'Sai was used as a boss enemy in Swarm's Warehouse District map.
- Rek'Sai's Series 2 Eternals make the following references:
  - *Jaws* is a reference to the Jaws (franchise) franchise.
  - *The Predator* is a reference to the Predator (franchise) franchise.
  - *Tremors* is a reference to the Tremors (franchise) franchise.

---
*This page was automatically generated from League of Legends Wiki data.*