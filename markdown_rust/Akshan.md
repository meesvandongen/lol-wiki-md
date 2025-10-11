# Akshan

## Stats

| Stat | Base | Growth |
|------|------|--------|

## Abilities

### Passive – Dirty Fighting

| Attribute | Value |
|-----------|------:|
| **Speed** | Akshan (Missile speed of basic attacks and crits) / 5000 (Additional shot missile speed) |
| **Static Cooldown (Unaffected by ability haste, starts upon the shield being destroyed or expiring)** | 16 to 4 for 4 / 1;6;11;16 |

**INNATE:** Whenever **Akshan** uses a basic attack, he fires an additional shot after a delay that deals 50% AD physical damage, increased to 100% AD against minions. Issuing an attack order (Default right-click/MB2) on a different target before the additional shot has been launched causes **Akshan** to fire it at the new target. If the second shot is cancelled instead, he gains 20 to 75 x (1 + 100% **bonus** attack speed) **bonus** movement speed decaying over 1 second.

The additional shot applies on-hit effects, triggers on-attack effects, and can critically strike.egamad sunob }}egamad lacitirc|}}3.0=dom|001|57|egamad lacitirc{{|sa{{ rof

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

### Q – Avengerang

| Attribute | Value |
|-----------|------:|
| **Range** |  850 + 500 per enemy hit |
| **Cast Time** | 0.25 |
| **Effect Radius** |  400 (Missiles' own sight range) |
| **Width** |  120 |
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

When **Akshan** claims a _Scoundrel's_ bounty he removes the marks of all other enemies. _Scoundrels_ refresh their mark duration on subsequent kills, and will have their mark removed when they die by any means. **Akshan** cannot mark enemies as _Scoundrels_ while they are dead. Allies are resurrected at their summoning_platform.

**ACTIVE:** **Akshan** enters camouflage, which lasts indefinitely while he is near terrain or inside brush, and for 2 seconds otherwise. During this time. he can see trails leading toward _Scoundrels_, and while facing them if they are within 5000 (Pending for test) units, he gains **bonus** mana regeneration equal to 12% of his **missing** mana as well as **bonus** movement speed.

_Going Rogue_ can be recast after 1 second, and does so automatically after its duration. Attacking or casting abilities ends _Going Rogue_ immediately.

**_Akshan** can move during Going Rogue's cast time._

**RECAST:** **Akshan** ends _Going Rogue_.

| Detail | Value |
|--------|------:|
| **Targeting** | Auto |
| **Affects** | Self, Enemies |

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

**FIRST CAST:** **Akshan** fires a hook in the target direction that embeds in the first terrain hit, and enters preparation (See notes) for up to 2.12 seconds. _Heroic Swing's_ second cast can be used while the hook is attached. If the hook fails to attach or **Akshan** is moved (See notes) or becomes immobilized, grounded, or polymorphed within the duration, the preparation will end prematurely, cancelling _Heroic Swing_ in the process.

**SECOND CAST:** **Akshan** swings around the terrain in either a clockwise or counterclockwise direction based on the position of the cursor relative to his facing direction (See notes), stopping upon colliding with an enemy champion or terrain. While swinging, he fires at the nearest visible enemy every  to deal them physical damage and apply on-hit effects for each shot, with on-hit damage reduced to 25% effectiveness.<br><br>**_Akshan** will be knocked down by any immobilizing or polymorphing crowd control during the dash._

**THIRD CAST:** **Akshan** ends the swing by jumping to the target location and fires one last shot at a nearby visible enemy.

Scoring an enemy champion takedown reduces _Heroic Swing's_ **current** cooldown to 0.50 seconds. The shots can critically strike foregamad }}egamad lacitirc|}}9.0=dom|001|571|egamad lacitirc{{|sa{{and apply {{sti|life steal}} at 100% effectiveness.

_Akshan and Akshan can be cast during the third cast's dash. Akshan can be cast at all points during Heroic Swing, though **Akshan** cannot fire during the swing while it is active. **Akshan** prioritizes firing at enemy champions with stacks of Akshan, then those damaged by his targeted spells in the last 4 seconds, then the nearest enemy. Heroic Swing can be cast during **Akshan's** other abilities._

| Detail | Value |
|--------|------:|
| **Targeting** | Auto / Location |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical |
| **Spell Effects** | basic |
| **Spell Shield** | special |
| **Parry** | true |
| **Projectile** | true |
| **Call For Help** | true |
| **Grounded** | special |
| **Knockdown** | true |

### R – Comeuppance

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** |  2500 |
| **Width** |  120 |
| **Speed** | 3200 |
| **Cost** | 100 |
| **Cost Type** | Mana |
| **Cooldown** | (+100 to 70% AP) |
| **Cooldown Start** | post-effect |

**ACTIVE:** **Akshan** locks onto the target enemy champion and begins channeling for 2.50 seconds, revealing them as well as revealing himself. He gradually stores bullets into his weapon over the duration.

_Comeuppance_ can be recast after 0.50 seconds during the channel, and does so automatically afterwards. _Comeuppance_ is placed on a 5-second cooldown if the channel is cancelled.

**RECAST:** **Akshan** fires all stored bullets at the target, each briefly granting sight around their trajectory and dealing physical damage to the first enemy hit, increased by *100% of critical chance and critical damage bonuses as well as by key= / key1= / type=target's **missing** health / 0 to 200 for 11 / 0 to 100 / formula=2% per 1% of target's **missing** health, capped at 100% **missing** health. The shots can hit structures.

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

<!-- Raw excerpt (first 20 lines) -->

<details><summary>Raw excerpt</summary>

```wikitext

== Abilities ==

==Champion skins==
:_This article section only contains champion skins. For all associated collection items, see Akshan_(Collection)._

==Patch history==

==Trivia==
```
</details>
