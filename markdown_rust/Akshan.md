# Akshan

## Overview

- **Title:** Akshan
- **Resource:** Mana

## Stats

| Stat | Base | Growth |
|------|------|--------|
| Armor | 26 | 4.7 |
| Attack Damage | 52 | 3 |
| Attack Speed | 0.638 | 4 |
| HP | 630 | 107 |
| HP Regen | 3.75 | 0.65 |
| MP | 350 | 40 |
| MP Regen | 8.2 | 0.7 |
| Magic Resist | 30 | 1.3 |
| Move Speed | 330 | 0 |
| Range | 500 | 0 |

## Advanced Stats

| Metric | Value |
|--------|-------|
| Acquisition Radius | 750 |
| Attack Delay Offset (s) | -0.1667 |
| Attack Speed Ratio | 0.4 |
| Missile Speed | 2000 |
| Pathing Radius | 35 |
| Selection Height | 100 |
| Selection Radius | 100 |

## Abilities

### Passive – Dirty Fighting

| Attribute | Value |
|-----------|------:|
| **Speed** | Akshan (Missile speed of basic attacks and crits) / 5000 (Additional shot missile speed) |
| **Static Cooldown (Unaffected by ability haste, starts upon the shield being destroyed or expiring)** | 16 to 4 for 4 / 1;6;11;16 |

**INNATE:** Whenever **Akshan** uses a basic attack, he fires an additional shot after a delay that deals 50% AD physical damage, increased to 100% AD against minions. Issuing an attack order (Default right-click/MB2) on a different target before the additional shot has been launched causes **Akshan** to fire it at the new target. If the second shot is cancelled instead, he gains 20 to 75 x (1 + 100% **bonus** attack speed) **bonus** movement speed decaying over 1 second.

The additional shot applies on-hit effects, triggers on-attack effects, and can critically strikeÃ£ÂÂ for 30% total critical damage bonus damage. Ã¢ÂÂ· 100% base damage + 30% **bonus** critical damage. Ã£ÂÂ

**INNATE:** **Akshan's** basic attacks on-hit and ability hits apply a stack of _Dirty Fighting_ to enemies for 5 seconds, refreshing on subsequent applications and stacking up to 3 times. The third stack against a target consumes them all to deal them 15;40;80;150 / 1;6;11;16 (+ 60% AP) **bonus** magic damage; if the target is a champion, **Akshan** will also gain a 40+(240/17)*(x-1)*(0.7025+0.0175*(x-1)) (+ 35% **bonus** AD) shield for 2 seconds. The shield may be gained only once every few seconds.

| Detail | Value |
|--------|------:|
| **Targeting** | Passive |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical Magic |
| **Spell Effects** | special |
| **Spell Shield** | false |
| **Parry** | true |
| **Projectile** | true |

**Notes:**

- If the first shot has killed its target, **Akshan** will automatically acquire another enemy within 200 units beyond his basic attack range, else he can do so by issuing an attack order (Default right-click/MB2) to a different target.
- Applies basic damage for the second shot and proc damage for the bonus damage.
- The second shot:
  - Is treated as a basic attack.
  - Critically strikes independently from the first shot.
  - Can be cancelled by inputting a different command right after using the first shot.
  - Counts as a separate hit for effects such as Electrocute, Muramana Shock, and Eclipse Ever Rising Moon.
  - Starts the attack windup's cooldown after it is used, rather than when the first shot is.
- The attack speed scaling on the movement speed buff includes the bonus attack speed gained from **Akshan's** innate attack speed growth.
  - At level 18, at minimum it grants 0.00 decaying movement speed.
- Changing targets for the second shot will also [acquire](./Basic_attack.md#Acquisition) the new target.
- The second shot, if the first shot's target was killed, will prioritize visible enemy champions, then minions on low health.
- This ability's damage is calculated based on the caster's current stats and changes dynamically. <!-- Blurb -->

### Q – Avengerang

| Attribute | Value |
|-----------|------:|
| **Range** | 850 + 500 per enemy hit |
| **Cast Time** | 0.25 |
| **Effect Radius** | 400 (Missiles' own sight range) |
| **Width** | 120 |
| **Speed** | 1500 / 2400 |
| **Cost** | (+60 to 80% AP) |
| **Cost Type** | Mana |
| **Cooldown** | (+8 to 5% AP) (Starts after the boomerang returns) |
| **Cooldown Start** | post-effect |

**ACTIVE:** **Akshan** throws a boomerang in the target direction that briefly grants sight around its trajectory and deals physical damage to enemies hit, revealing them for 1 second and extending its range each time it hits a target. If this hits an enemy champion, **Akshan** gains 20% (+ 5% per 100 AP) **bonus** movement speed that decays over 1 second.

Once the boomerang has passed its original range and has not hit a target in the last 500 units of travelling, it homes back to **Akshan** and applies the same effects to enemies hit.

_Avengerang_ deals reduced damage against non-champions.

_Enemies can be hit only once per pass._

| Detail | Value |
|--------|------:|
| **Targeting** | Direction |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical |
| **Spell Effects** | spellaoe |
| **Spell Shield** | special |
| **Projectile** | true |

**Notes:**

- Spell shield will only block a single hit. They do not prevent the initial throw from extending its range.
- This ability's damage is calculated based on the caster's current stats and changes dynamically. <!-- Blurb -->

### W – Going Rogue

| Attribute | Value |
|-----------|------:|
| **Cast Time** | 0.50 |
| **Cost** | (+40 to 0% AP) |
| **Cost Type** | Mana |
| **Cooldown** | (+18 to 2% AP) |
| **Cooldown Start** | post-effect |
| **Detection Radius** | 800 |

**PASSIVE:** **Akshan** marks enemy champions that kill allied champions as _Scoundrels_ for 60 seconds. If **Akshan** scores a takedown against a _Scoundrel_ while alive and within 3 seconds of damaging them, he receives an additional 100 and resurrects all dead allied champions that they have slain after 1 second.

When **Akshan** claims a _Scoundrel's_ bounty he removes the marks of all other enemies. _Scoundrels_ refresh their mark duration on subsequent kills, and will have their mark removed when they die by any means. **Akshan** cannot mark enemies as _Scoundrels_ while they are dead. Allies are resurrected at their [summoning platform](./Spawn.md).

**ACTIVE:** **Akshan** enters camouflage, which lasts indefinitely while he is near [terrain](./terrain.md) or inside brush, and for 2 seconds otherwise. During this time. he can see trails leading toward _Scoundrels_, and while facing them if they are within 5000 (Pending for test) units, he gains **bonus** mana regeneration equal to 12% of his **missing** mana as well as **bonus** movement speed.

_Going Rogue_ can be recast after 1 second, and does so automatically after its duration. Attacking or casting abilities ends _Going Rogue_ immediately.

**_Akshan** can move during Going Rogue's cast time._

**RECAST:** **Akshan** ends _Going Rogue_.

| Detail | Value |
|--------|------:|
| **Targeting** | Auto |
| **Affects** | Self, Enemies |

**Notes:**

- The mark displays the duration and slay count.
  - Enemy champions that kill **Akshan** will not be marked as _Scoundrels_ nor increase their slay count.
- Clones do not count for triggering _Going Rogue's_ passive.
- _Going Rogue's_ buff refreshes to last indefinitely upon moving near terrain or into brush, and will refresh to 2 seconds after leaving near terrain or brush.
- Allied champions within a zombie state are resurrected only after the state ends.
- **Akshan** can still trigger _Going Rogue's_ passive while he is within a zombie state.
- Using a basic attack breaks the stealth at the start of the attack windup.

### E – Heroic Swing

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 800 |
| **Width** | 80 (Hook missile width) |
| **Speed** | 2500 (Hook missile speed) / 1200 (Dash orbital speed) / 3000 (Automatic attacks missile speed) |
| **Cost** | 70 |
| **Cost Type** | Mana |
| **Cooldown** | (+18 to 12% AP) |
| **Cooldown Start** | post-effect |

**ACTIVE:** **Akshan** can activate _Heroic Swing_ three times before the ability goes on cooldown, and can use the third cast only after 0.50 seconds of the second cast. **Akshan** may input an attack or movement command to use the second and third casts.

**FIRST CAST:** **Akshan** fires a hook in the target direction that embeds in the first [terrain](./terrain.md) hit, and enters preparation (See notes) for up to 2.12 seconds. _Heroic Swing's_ second cast can be used while the hook is attached. If the hook fails to attach or **Akshan** is moved (See notes) or becomes immobilized, grounded, or polymorphed within the duration, the preparation will end prematurely, cancelling _Heroic Swing_ in the process.

**SECOND CAST:** **Akshan** swings around the terrain in either a clockwise or counterclockwise direction based on the position of the cursor relative to his facing direction (See notes), stopping upon colliding with an enemy champion or terrain. While swinging, he fires at the nearest visible enemy every  to deal them physical damage and apply on-hit effects for each shot, with on-hit damage reduced to 25% effectiveness.

**_Akshan** will be knocked down by any immobilizing or polymorphing crowd control during the dash._

**THIRD CAST:** **Akshan** ends the swing by jumping to the target location and fires one last shot at a nearby visible enemy.

Scoring an enemy champion takedown reduces _Heroic Swing's_ **current** cooldown to 0.50 seconds. The shots can critically strike forÃ£ÂÂ 90% total critical damage damage Ã¢ÂÂ· 90% **total** critical damage Ã£ÂÂand apply life steal at 100% effectiveness.

_Akshan and Akshan can be cast during the third cast's dash. Akshan can be cast at all points during Heroic Swing, though **Akshan** cannot fire during the swing while it is active. **Akshan** prioritizes firing at enemy champions with stacks of Akshan, then those damaged by his targeted spells in the last 4 seconds, then the nearest enemy. Heroic Swing can be cast during **Akshan's** other abilities._

| Detail | Value |
|--------|------:|
| **Targeting** | [Auto](./Auto-targeted.md) / [Location](./Location-targeted.md) |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical |
| **Spell Effects** | basic |
| **Spell Shield** | special |
| **Parry** | true |
| **Projectile** | true |
| **Call For Help** | true |
| **Grounded** | special |
| **Knockdown** | true |

**Notes:**

- The second cast's dash will displace **Akshan** around the terrain in either a clockwise or counterclockwise direction, with the specific direction being determined by the position of the player's cursor relative to his facing direction.
  - The dash will move clockwise if the player's cursor is anywhere to the left of **Akshan** from his facing direction (or perspective), and counterclockwise if the cursor is anywhere to his right.
- **Akshan** will not stop swinging until his dash is stopped.
  - The swing will end prematurely if the terrain the hook was attached to no longer exists, such as [player-generated terrain](./Terrain.md#Player-Generated).
- **Akshan** will prioritize firing at enemies he damaged with _any_ unit-targeted ability or spell within the last 4 seconds, such as Akshan or Ignite.
- _Heroic Swing_ grants a buff to **Akshan** for 2 seconds that indicates and determines the first cast's duration.
  - This buff starts as soon as the ability is cast, and lingers for 0.12 seconds after it expires.
    - This means the hook can be attached to terrain for longer if it hits it earlier in the duration, and vice versa.
  - If **Akshan** dashes (excluding second cast's dash) or blinks or becomes affected by immobilization, ground, or polymorph in the duration, the buff is removed _immediately_, causing the first cast to be lost and cancelling _Heroic Swing_ entirely.
- **Akshan** immediately fires one shot at the beginning of his swing and a final shot while dismounting from the swing.
- **Akshan** can fire at any targetable enemy unit excluding structures and [jungle plants](./jungle_plants.md).
- Only _Heroic Swing's_ first cast is disabled while grounded or rooted. The third cast is still usable during those effects.
- _Heroic Swing's_ second and third cast can both be used while silenced.
- Takedowns against clones do not count for resetting _Heroic Swing's_ cooldown.
- If a takedown is scored while _Heroic Swing_ is active, the cooldown afterwards will be 0.50 seconds.
- **Akshan** will attempt to basic attack the target he fired at with the last shot after the third cast's dash ends, if there is no other input given.
- Each shot generates a stack of Conqueror.
- Spell shield will only block a single shot.
- The attack speed scaling on the damage of the attacks includes the bonus attack speed gained from **Akshan's** innate attack speed growth.
  - At level 18, each shot at minimum deals 0.00 (+ 0.00% AD) physical damage.
- Despite this ability not applying on-attack effects, it does apply Navori Flickerblade Transcendence.
- Under certain circumstances, upon attaching the hook to terrain, **Akshan** will create a particle on the location permanently.
  - This particle can be attacked by minions.
- This ability's damage is calculated based on the caster's current stats and changes dynamically.
- Hook range including lollipop against walls, automated attacks range while swinging, final dash distance and speed.
- The following table refers for interactions while the hook is attached or in flight: (attack=Automatically initiates the second cast.)
- The following table refers for interactions while **Akshan** is swinging: (attack=Automatically initiates the third cast.) <!-- Blurb -->

### R – Comeuppance

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 2500 |
| **Width** | 120 |
| **Speed** | 3200 |
| **Cost** | 100 |
| **Cost Type** | Mana |
| **Cooldown** | (+100 to 70% AP) |
| **Cooldown Start** | post-effect |

**ACTIVE:** **Akshan** locks onto the target enemy champion and begins channeling for 2.50 seconds, revealing them as well as revealing himself. He gradually stores bullets into his weapon over the duration.

_Comeuppance_ can be recast after 0.50 seconds during the channel, and does so automatically afterwards. _Comeuppance_ is placed on a 5-second cooldown if the channel is cancelled.

**RECAST:** **Akshan** fires all stored bullets at the target, each briefly granting sight around their trajectory and dealing physical damage to the first enemy hit, increased by 50% of critical chance and critical damage bonuses as well as by key= / key1= / type=target's **missing** health / 0 to 200 for 11 / 0 to 100 / formula=2% per 1% of target's **missing** health, capped at 100% **missing** health. The shots can hit structures.

Each bullet's damage applies life steal at 100% effectiveness and executes minions.

**_Akshan** can move while channeling Comeuppance._

| Detail | Value |
|--------|------:|
| **Targeting** | Unit |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical |
| **Spell Effects** | spell |
| **Spell Shield** | special |
| **Projectile** | true |
| **Call For Help** | false |
| **Silence** | true |

**Notes:**

- _Comeuppance_ refreshes the duration of _Akshan_ stacks on the target every 0.25 seconds.
- A kill threshold will appear on the [health bar](./health_bar.md) of the target locked onto during the channel, which increases each time **Akshan** stores a bullet. If the target's **current** health falls below the threshold, their health bar will be framed in red.
  - The indicator factors the bonus damage applied by Akshan as well as [damage modifiers](./damage_modifier.md) and the target's resistances.
  - It also factors the projected increase in the target's **missing** health, from which each consecutive bullet scales more.
  - It updates dynamically over the channel's duration, reacting to fluctuations of the target's health and damage mitigations.
- The reveal will linger for 2 seconds after the channel has ended.
- Spell shield will not prevent the lock-on and will only block a single bullet.
- _Comeuppance_ will not go on a reduced cooldown if **Akshan** dies during the channel.
- _Comeuppance_ will cancel if the target becomes untargetable or dies during the channel.
- Damage to structures **does** scale with their **missing** health.
- This ability's damage is calculated based on the caster's current stats and changes dynamically.
- The following table refers for interactions while **Akshan** is channeling: (move=true)

## Trivia

- His dance references Indian musical video [https://www.youtube.com/watch?v=l_MyUGq7pgs Malhari] from album [https://en.wikipedia.org/wiki/Bajirao_Mastani_(soundtrack) Bajirao Mastani].
  - A side-by-side comparison can be seen [https://www.youtube.com/watch?v=BbUvuaDd2WU here.]
- Akshan is the first champion to be revealed for League of Legends, Wild Rift and Legends of Runeterra around the same time, and with the next one being Ambessa.
- Akshan is the second of three champions released in 2021 tied to Viego return and a continuation of the Ruined King's story.<ref>[https://www.youtube.com/watch?v=bo0K25R2fpY Champions in Season 2021]</ref>
- Akshan's Series 2 [Eternals](./Eternals.md) make the following references:
  - _Plot Armor_ is a reference to the type of narrative where the character remains alive despite adversity in order to sustain the story.

<!-- Raw excerpt (first 20 lines) -->

<details><summary>Raw excerpt</summary>

```wikitext

== Abilities ==

==Champion skins==
:_This article section only contains champion skins. For all associated collection items, see [Akshan (Collection)](./Akshan_Cosmetics.md)._

==Patch history==

==Trivia==
```
</details>
