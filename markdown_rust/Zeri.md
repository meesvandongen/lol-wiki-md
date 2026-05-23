# Zeri

## Overview

- **Title:** Zeri
- **Resource:** Mana

## Stats

| Stat | Base | Growth |
|------|------|--------|
| Armor | 24 | 4.2 |
| Attack Damage | 56 | 2 |
| Attack Speed | 0.658 | 2 |
| HP | 600 | 110 |
| HP Regen | 3.25 | 0.7 |
| MP | 250 | 45 |
| MP Regen | 6 | 0.8 |
| Magic Resist | 30 | 1.3 |
| Move Speed | 330 | 0 |
| Range | 500 | 0 |

## Advanced Stats

| Metric | Value |
|--------|-------|
| Acquisition Radius | 800 |
| Attack Delay Offset (s) | -0.1437 |
| Attack Speed Ratio | 0.625 |
| Missile Speed | 0 |
| Pathing Radius | 40 |
| Selection Height | 122.2222 |
| Selection Radius | 100 |

## Abilities

### Passive – false

| Attribute | Value |
|-----------|------:|
| **Attack Range** | Zeri |
| **Speed** | N/A (Non-projectile) |

**INNATE:** **Zeri** generates 1 charge for every 40 units she travels by any means and 10 charge every time she casts _Zeri_, up to a maximum of 100 charge. Her basic attacks consume charge to deal **modified** damage.
**Zeri** gains maximum charge when the game starts and upon [respawn](./respawn.md)ing.

**BASIC ATTACK:** **Zeri** zaps the target, applying spell effects as spell damage, and triggering on-cast effects. This cannot critically strike nor trigger on-hit and on-attack effects.

At full charge, **Zeri's** next attack is empowered to consume all charge to deal 75 to 160 (+ 110% AP) (+ 1 to 11 / key= of target's **maximum** health) magic damage. The damage based on the target's health ratio is capped at 300 against monsters.

While not at full charge, **Zeri's** attacks deal 10+(15/17)*(x-1)*(0.7025+0.0175*(x-1)) (+ 3% AP) magic damage, and  targets below 6*10 to 6*25 / color=hp (+ 18% AP) health. Each attack consumes 10 charge if she has enough already.

| Detail | Value |
|--------|------:|
| **Targeting** | Unit |
| **Affects** | Self |
| **Damage Type** | magic |
| **Spell Effects** | Spell |
| **Spell Shield** | special |
| **Parry** | false |
| **Projectile** | false |

**Notes:**

- Charged attacks only deal the **base** damage to structures.
- Uncharged attacks do not execute enemies that are shielded or invulnerable while below the health threshold.
- Spell shield will only block a fully charged attack. Uncharged attacks are not blocked.
- The attack's range is **not** increased from attack range increases (Rapid Firecannon). Instead, Zeri reach is.
- Uncharged and charged attacks trigger Tear of the Goddess Mana Charge.
- The empowered attack will trigger but not be consumed against wards or [jungle plants](./jungle_plants.md).

### Q – Zeri Burst Fire

| Attribute | Value |
|-----------|------:|
| **Range** | 750 (Active spray missile reaches) + 100% **bonus** attack range |
| **Cast Time** | 100% of **Zeri's** windup time ((+(1/Zeri)*Zeri/round=3% AP) at **base** attack speed) |
| **Angle** | 5ÃÂ° (Angle at which the outermost missiles fan out from one another, 2.5 degree from the center) / 2ÃÂ° (Decreased spread angle during ult) |
| **Width** | 80 (Active spray missiles) |
| **Speed** | 2600 (Active spray missile speed) / 3400 (Empowered spray missile speed while Overcharged) |
| **Cost** | None |
| **Static** | 1**Zeri's** attack speed |
| **Queue Time** | 0.25 (Regular cast) / 0.05 (Empowered cast while Overcharged) |

**ACTIVE:** **Zeri** fires a burst of 7 rounds in the target direction that each deal physical damage to the first enemy hit.

_Burst Fire's_ projectile is treated as a basic attack: it hits any enemy unit a typical basic attack can; deals basic damage; can critically strike for total critical damage damage; applies on-hit effects to the first enemy hit; and triggers on-attack effects once. _Burst Fire's_ cooldown and cast time are reduced with attack speed, with the maximum of 1.50 attacks per second. 70% of attack speed in excess of the cap is converted into **bonus** attack damage.

| Detail | Value |
|--------|------:|
| **Targeting** | Direction <!-- internally Location --> |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical |
| **Spell Effects** | basic |
| **Spell Shield** | false |
| **Parry** | special |
| **Projectile** | true |
| **Call For Help** | true |

**Notes:**

- _Burst Fire_ is automatically learned with her first skill point upon spawning.
- Any of _Burst Fire's_ rounds hitting a champion will draw minion aggro, regardless of whether or not other targets were hit first by _Burst Fire's_ rounds.
  - A lightning chain from a Zeri empowered _Burst Fire_ will not draw minion aggro.
- Casting _Burst Fire_ does not cancel previous attack (Zeri) and movement orders.
- _Burst Fire_ does not trigger Tear of the Goddess Mana Charge.
- _Burst Fire's_ cooldown is not reduced by Navori Flickerblade Transcendence.
- Amumu, Fizz, and Leona flat damage reduction apply to **each** of the 7 instances of damage from _Burst Fire_.
  - Warden's Mail flat damage reduction and cap only apply to the first instance of damage.
  - Guardian's Horn flat damage reduction only applies to the first instance of damage, but handles it as damage-over-time (reduced to 25% effectiveness).
- _Burst Fire_ still counts as an ability activation for the purposes of on-cast effects such as triggering Kassadin passive.
  - It does **not** count as an ability activation for Spellblade.
- _Burst Fire_ can hit **all** enemy units that a basic attack would be able to target. This does include [jungle plants](./jungle_plants.md), [structure](./structure.md)s, and [ward](./ward.md)s.
  - Wards will only be hit once by _Burst Fire_. [Stealthed wards and traps](./Stealth.md#Stealthed_traps_and_wards) will not be hit.
  - Champion-summoned units behave differently depending on the unit.
    - _Gangplank's_ Gangplank and _YorickÃ¢ÂÂs_ Yorick are hit only once by _Burst Fire_, and any subsequent rounds not empowered by Zeri will stop upon reaching the same object.
    - _Kalista's_ Kalista is hit only once by _Burst Fire_, but any subsequent rounds will continue to travel through the _Sentinel_.
- If _Burst Fire_ hits an enemy while **Zeri** is not visible to enemies, the area around her (400 units) will be revealed for 4.50 seconds.
- _Burst Fire_ rolls critical strike for all rounds as well as the additional physical damage dealt when **Zeri** is Zeri.
- _Burst Fire_ is parried by dodge and block.
  - Blind causes _Burst Fire_ to cast in a random direction. The rounds will still deal damage to enemies hit.
- _Burst Fire's_ rounds are each fired in the target direction from where **Zeri** is at the time.
- **Only** attack speed granted by being Zeri can exceed the cap of 1.50.
  - Hail of Blades does not allow **Zeri** to exceed her attack speed cap.
- Items and runes that trigger off of attacking [eg Fleet Footwork, Kraken Slayer] will only trigger if _Burst Fire_ hits a unit.
- If Zeri would get full Energized stacks from a _Burst Fire_, the rest of the rounds will trigger Energized.
- _Burst Fire's_ cast-indicator does not show range increases; modified range VFX are still visible around the champion model.
- _Burst Fire_ can apply the effect of Horizon Focus when the enemy hit is within the last 50 units of this ability.
- _Burst Fire_ uses a modified icon when empowered by Zeri () and Zeri () as well as both ().
- Destroying a ward that is targetable but not visible to **Zeri's** team (e.g. a Control Ward in [Fog of War](./Fog_of_War.md)) via _Burst Fire_ uniquely allows her to remove its accompanying [ward timer](./Ward.md#Ward_Timers) (if it exists) without seeing the ward being destroyed.

### W – Zeri Ultrashock Laser

| Attribute | Value |
|-----------|------:|
| **Cast Time** | type=**bonus** attack speed / round=3 / 0.55;0.528;0.495;0.462;0.429;0.396;0.363;0.33;0.3 / key1= / 0;24.4444;61.1111;97.7778;134.444;171.111;207.778;244.444;277.778 / formula=0.55-0.09 per 100% **bonus** attack speed)). _This is capped at 45.5% reduction at 277.8% **bonus** attack speed._
|cost         = (+50 to 90% AP)
|costtype     = Mana
|range        =  1200 (Initial missile) /  -50 Ã¢ÂÂ 1550 (1600 units total empowered length)
|width        =  80 (Initial missile) /  200 (Laser beam width)
|speed        = 2500 (Initial missile)
|targeting    = Direction
|damagetype   = Magic
|projectile   = special
|affects      = Enemies
|spelleffects = special
|onhiteffects = 
|spellshield  = true
|parry        = 
|callforhelp  = 
|notes        = 
* Applies spell damage for the pulse and area damage for the laser.
* The pulse missile is blocked by projectile-interception effects but not the laser.
* _Ultrashock Laser_ interacts with [player-generated terrain](./Terrain.md#Player-Generated). |
| **Cooldown** | (+12 to 8% AP) |

**ACTIVE:** **Zeri** fires an electric pulse in the target direction that deals physical damage to the first enemy hit and slows them for 2 seconds.

If the pulse hits [terrain](./terrain.md), it transforms into a laser in a line that grants sight of the area for 1.75 seconds and fires after 0.85 seconds, applying the same effects to enemies hit and critically striking for total critical damage damage against champions and monsters.

### E – Zeri Spark Surge

| Attribute | Value |
|-----------|------:|
| **Range** | 300 (Standard dash distance) |
| **Cast Time** | None |
| **Speed** | 600 + 100% movement speed |
| **Cost** | (+90 to 70% AP) |
| **Cost Type** | Mana |
| **Cooldown** | (+22 to 18% AP) |
| **Cooldown Start** | on-cast |

**ACTIVE:** **Zeri** dashes in the target direction. She will dash farther across [terrain](./terrain.md) if _Spark Surge_ was cast within 50 units of any, gaining unobstructed vision of the surrounding 800 units and revealing herself while there are enemy champions within 1500 units of her.

Afterwards, she gains _Lightning Rounds_ for 5 seconds, empowering _Zeri_ to deal **bonus** magic damage to the first enemy hit, increased by 0 to 85 for 11 / key= / 0 to 100 / key1= / type=critical strike chance, and pierce through enemies. Targets after the first take modified damage which does not apply on-hit effects or life steal nor trigger on-attack effects, but is affected by critical strike modifiers.

_Spark Surge's_ **current** cooldown is reduced by 0.50 seconds for every champion **Zeri** hits with _Zeri_ basic attacks or abilities, increased to 1.50 seconds if she does so with a cast of _Zeri_ or _Zeri_ that critically strikes.

_Spark Surge resets **Zeri's** basic attack timer and Zeri cooldown. Zeri and Zeri can be cast during the dash._

| Detail | Value |
|--------|------:|
| **Targeting** | Direction |
| **Affects** | Enemies, Self |
| **Damage Type** | Magic |
| **Spell Effects** | proc |
| **Parry** | true |
| **Grounded** | true |
| **Knockdown** | true |

**Notes:**

- _Spark Surge's_ extended dash travels based on the thickness of the terrain, up to a maximum distance.
- The following table refers for interactions while **Zeri** is dashing: (attack=false)

### R – Zeri Lightning Crash

| Attribute | Value |
|-----------|------:|
| **Cast Time** | 0.25 |
| **Effect Radius** | 825 (Nova radius) |
| **Cost** | 100 |
| **Cost Type** | Mana |
| **Cooldown** | (+80 to 70% AP) |
| **Cooldown Start** | post-effect |

**ACTIVE:** **Zeri** discharges an electric nova that deals magic damage to nearby enemies. If this hits at least one enemy champion, she enters _Overcharged_ for 5 seconds. Hitting an enemy champion with an ability or _Zeri_ attack will extend the duration by 1.50 seconds, up to its original duration.

**OVERCHARGED:** **Zeri** gains 10% **bonus** movement speed and 30% **bonus** attack speed that is allowed to exceed her attack speed cap (normally 1.5 attacks per second) by the amount gained. _Zeri_ is empowered to have a 20% shorter cast time and instead fire 3 rounds that travel with increased speed and chain to the nearest visible enemy within 650 units of the target, up to 4 subsequent targets, to deal 40% AD physical damage. This damage is affected by critical strike modifiers.

During _Overcharged_, **Zeri** can generate stacks of _Hypercharged_ from enemy champions that last 1.50 seconds. She generates 1 stack for each one she hits with an ability or _Zeri_ attack, increased to 3 on abilities that critically strike. Subsequent hits refresh the duration of _Hypercharged_. _Zeri_ grants stacks only against the first target hit.

**HYPERCHARGED:** For each stack, **Zeri** gains 0.50% **bonus** movement speed.

| Detail | Value |
|--------|------:|
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical Magic |
| **Spell Effects** | special |
| **Spell Shield** | special |
| **Projectile** | false |

**Notes:**

- **Zeri** can move while casting _Lightning Crash_, and it does not interrupt her previous orders.
- **Zeri** can stack _Overcharge_ up to 100,000 times.
- Applies area damage for the nova and proc damage for the chain lightning.
- Spell shield blocks _Lightning Crash's_ initial nova but does not prevent **Zeri** from becoming _Overcharged_.
  - It does not block _Zeri_ electricity chain.
- Clones count for granting _Overcharge_.
- _Lightning Crash's_ and _Zeri_ empowerments to _Zeri_ can combine together.
- _Zeri_ electricity chain will not chain across structures.
- If **Zeri** enters resurrection during _Overcharge_, she will lose all stacks of the effect but _Overcharge_ will not end prematurely.
  - She cannot gain stacks again after reviving while the buff is active.

## Trivia

- When Zeri and Ekko are on the same team, they gain a buff named  _Zaunite Ingenuity_, which reads _"Kids from the undercity stick together."_
- Zeri's splash was different upon release. After a few days it was edited to show a darker skintone, smaller and more detailed eyes, among other changes. [https://i.redd.it/sdrsel7taea81.gif Gif comparison].
- She is the only champion with 0 point at a skill by the client classification.
  - In this case, her Utility rating is 0.
- Her Q can destroy Control wards without having vision on them.

<!-- Raw excerpt (first 20 lines) -->

<details><summary>Raw excerpt</summary>

```wikitext

== Abilities ==

==Champion skins==
:_This article section only contains champion skins. For all associated collection items, see [Zeri (Collection)](./Zeri_Cosmetics.md)._

==Patch history==

==Trivia==
* When Zeri and Ekko are on the same team, they gain a buff named  _Zaunite Ingenuity_, which reads _"Kids from the undercity stick together."_
```
</details>
