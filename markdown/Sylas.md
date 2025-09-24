# Sylas

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
| **Champion** | Sylas |
| **Title** | the Unshackled |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2019-01-25 |
| **Release Patch** | V9.2 |
| **Latest Changes** | V25.18 |
| **Roles** | Burst, Skirmisher |
| **Riot Positions** | Middle |
| **External Positions** | Top, Middle |
| **Blue Essence** | 1575 |
| **Riot Points** | 790 |
| **Difficulty** | 3 |
| **Hero Type** | Mage |
| **Alt Type** | Assassin |
| **Adaptive Type** | Magic |
| **Damage** | 2 |
| **Toughness** | 2 |
| **Control** | 1 |
| **Mobility** | 3 |
| **Utility** | 1 |
| **Style** | 70 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $600.0$ | $+122.0$ |
| **Mana** | $400.0$ | $+70.0$ |
| **Health Regen** | $9.0$ | $+0.9$ |
| **Mana Regen** | $8.0$ | $+0.8$ |
| **Armor** | $29.0$ | $+5.2$ |
| **Magic Resist** | $32.0$ | $+2.55$ |
| **Attack Damage** | $61.0$ | $+3.0$ |
| **Attack Speed** | $0.645$ | |
| **Movement Speed** | $340.0$ | $+0.0$ |
| **Attack Range** | $175.0$ | $+0.0$ |
| **Base Attack Speed** | $0.645$ | |
| **Attack Speed Ratio** | $0.645$ | |
| **Bonus AS per Level** | $3.5\%$ | |
| **Attack Windup** | $16.8\%$ | |
| **Acquisition Radius** | $525$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $100$ units | |
| **Selection Height** | $135$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $105.0\%$ |
| **Damage Taken** | $100.0\%$ |

#### URF

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $95.0\%$ |
| **Damage Taken** | $100.0\%$ |

## Abilities

### Passive: Petricite Burst

| Attribute | Value |
|-----------|------:|
| **Effect Radius** | 300 units |
| **Targeting** | Passive |
| **Affects** | Self |
| **Damage Type** | Magic |
| **Spell Shield** | False |
| **Spell Effects** | Special |
| **Parry** | Special |

**INNATE:** Whenever **Sylas** casts an ability, he generates a stack of *Unshackled* for 4 seconds, refreshing on subsequent casts and stacking up to 3 times.

**UNSHACKLED:** **Sylas**' next basic attack gains 125% **bonus** attack speed and is empowered to consume a stack to whirl his chains around him, which has an uncancellable windup and deals 130% AD (+ 30% AP) magic damage to the primary target and 40% AD (+ 20% AP) magic damage to nearby enemies.

*Unshackled* deals 130% damage to monsters, and executes minions that are secondary targets and would be left below 25 health.

*Petricite Burst* can critically strike for damage only against the primary target.

**Notes:**

- Spellblade damage does not get converted to magic damage, and will deal its damage only to the primary target.
- Applies proc damage to the primary target and area damage to secondary targets.
- *Petricite Burst*’s damage cannot critically strike against secondary targets.
- *Petricite Burst* only applies on-hit effects to the primary target.
- *Petricite Burst* attacks benefit from life steal.
- *Petricite Burst* can be dodged, blocked, or missed while **Sylas** is blinded as the primary target but not as a secondary target.

---

### Q: Chain Lash

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.4$ seconds |
| **Target Range** | 50 (Minimum convergence range) / 775 (Maximum convergence range) units |
| **Effect Radius** | 180 (Detonation radius against champions and minions) / 200 (Detonation radius against monsters) |
| **Cost** | 55 Mana |
| **Cooldown** | 10 / 9 / 8 / 7 / 6 seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | False |
| **Out of Range Behavior** | cast at max |

**ACTIVE:** **Sylas** lashes out two chains that converge to the target location and extend beyond it up to a maximum range, dealing magic damage to enemies hit and slowing them for $1.5$ seconds.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 40 / 60 / 80 / 100 / 120 (+ 40% AP) |

| Attribute | Value |
|-----------|------:|
| **Slow** | 15 / 20 / 25 / 30 / 35% |

After a $0.6$-second delay, the chains' intersection explodes to deal magic damage to enemies within, reduced to 40% against minions.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 60 / 115 / 170 / 225 / 280 (+ 80% AP) |
| **Total Magic Damage** | 100 / 175 / 250 / 325 / 400 (+ 120% AP) |

| Attribute | Value |
|-----------|------:|
| **Minion Damage** | 24 / 46 / 68 / 90 / 112 (+ 32% AP) |
| **Total Minion Damage** | 64 / 106 / 148 / 190 / 232 (+ 72% AP) |

**Notes:**

- The detonation's effect radius center is placed at the intersection between the chains from the initial cast, meaning a max range cast can create a detonation that reaches further than the chains do.

---

### W: Kingslayer

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 400 units |
| **Cost** | 50 / 60 / 70 / 80 / 90 Mana |
| **Cooldown** | 12 / 10.5 / 9 / 7.5 / 6 seconds |
| **Queue Time** | $0.5$ seconds |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Single target |
| **Call For Help** | True |
| **Grounded** | True |
| **Knockdown** | True |

**ACTIVE:** **Sylas** dashes to the front of the target enemy's location then strikes them to deal magic damage.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 75 / 110 / 145 / 180 / 215 (+ 60% AP) |

If this damages a champion, **Sylas** is also healed, increased by 0%–100%@0–60 (@=his **missing** health).

| Attribute | Value |
|-----------|------:|
| **Minimum Heal** | 20 / 40 / 60 / 80 / 100 (+ 20% AP) (+ 5% of his **bonus** health) |
| **Maximum Heal** | 40 / 80 / 120 / 160 / 200 (+ 40% AP) (+ 10% of his **bonus** health) |

**Notes:**

- The damage will be dealt if the dash completed uninterrupted, and regardless of how far the target's location is at the end of the dash.
  - **Sylas** will heal even if the target champion dies before the dash is completed.
  - The only exception is if the target becomes untargetable, in which case *Kingslayer* will have no effect.

---

### E: Abduct

| Attribute | Value |
|-----------|------:|
| **Range** | 950 (Maximum range at center with lollipop) / er 790 (Maximum edge range from missile collision) units |
| **Cast Time** | $0.25$ seconds |
| **Width** | 120 (Missile collision width) units |
| **Speed** | 2500 - 400 (Chain missile, deceleration 3200/s) / 1800 (Dash speed) units/second |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Single target |
| **Projectile** | True |
| **Grounded** | True |
| **Knockdown** | True |

**ACTIVE:** **Sylas** whips out his chains in the target direction that deal magic damage to the first enemy hit and reveal and stun them for $0.5$ seconds. Upon hitting the target, **Sylas** dashes to their location and knocks them up for $0.5$ seconds upon arrival.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 80 / 130 / 180 / 230 / 280 (+ 80% AP) |

***Sylas** is unable to cast Chain Lash while the chains are in flight.*

**Notes:**

- **Sylas** will not dash to his target if the chains are blocked by spell shield.
- The target will be revealed for 2 seconds if the dash is interrupted.
- *Abduct* will still apply the knock up if the target is untargetable by the end of the dash. Effect at cast time start
- **Sylas** is unable to perform actions while dashing to the target, and becomes able to again after $0.05$ seconds of the dash ending.
- **Sylas** will not dash to the target hit if he is channeling or dashing from a Hijacked ability.
  - *Abduct* will not apply the knock up when this happens.
  - If the chains connect before a pre-channel cast time is complete then Sylas will dash to the target and the ability will cancel.

---

### E: Abscond

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 400 (Maximum dash range) units |
| **Speed** | 1450 (Dash speed) units/second |
| **Cost** | 65 Mana |
| **Cooldown** | 13 / 12 / 11 / 10 / 9 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Location |
| **Affects** | Self |
| **Out of Range Behavior** | cast at max |
| **Grounded** | True |
| **Knockdown** | True |

**ACTIVE:** **Sylas** dashes to the target location. Within $3.5$ seconds, he can cast *Abduct* after a $0.2$-second delay (Starts on-cast, not after completing the dash) from casting *Abscond*.

*Kingslayer can be cast during the dash.*

**Notes:**

- No additional details.

---

### R: Hijack

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 950 units |
| **Speed** | 2200 units/second |
| **Cost** | 75 Mana |
| **Cooldown** | 80 / 67.5 / 55 / 42.5 / 30 (Starts upon recasting or when the hold duration ends) seconds |
| **Cooldown Start** | Special |
| **Targeting** | Unit |
| **Affects** | Self, Enemies |
| **Spell Shield** | True |
| **Projectile** | True |
| **On-Target Cooldown (Reduced by Sylas' ability haste, including ultimate haste. The target's haste is not considered.)** | 200% of target's base ultimate cooldown (A minimum of 40 second cooldown. Starts on steal, rather than on casting the stolen ability.) |

**ACTIVE:** **Sylas** launches his chains at the target enemy champion, gaining a copy of their ultimate ability and revealing them for $0.825$ seconds (Estimated, from start of cast time). **Sylas** cannot select the same champion again for a set duration, and can hold the *hijacked* ultimate for up to 90 seconds, during which he can recast *Hijack*.

**RECAST:** **Sylas** casts his hijacked ultimate ability at no cost, scaling based on *Hijack*’s rank and his own statistics.

*Hijacked* ultimates and abilities that do not scale with ability power have their attack damage ratios converted to ability power ratios, scaling with $0.6$% AP per 1% **total** AD, and $0.4$% AP per 1% **bonus** AD respectively.

**Notes:**

- *Petricite Burst*’s stack from casting *Hijack* is given after **Sylas** receives the Hijacked ability.
- *Hijack*’s per target cooldown is indicated by a under recently targetted enemy champions, and is only visible to **Sylas** and each individual enemy.
- *Hijack*’s base chain missile will only be destroyed and not replicated against Mel’s Rebuttal. **Sylas** will not receive a Hijacked Golden Eclipse.
- If **Sylas** casts *Hijack* on a clone, no ultimate is stolen and the ability will be placed on full cooldown.
  - The same does not occur for champions within a zombie state.
- Clones do not copy *Hijack*’s target cooldown indicator, potentially revealing them to **Sylas**.
  - Neeko’s Inherent Glamour does not hide her personal cooldown indicator, nor does she copy her allies'. ** If the same happens when casting an on-target stolen ability, the ability will instead cancel and go on cooldown.
- If *Hijack* is intercepted by Braum’s Unbreakable, **Sylas** will gain a copy of R instead and an on-target cooldown will be applied to him.
  - If Braum has an on-target cooldown on him (by being previously *Hijacked*) upon interception, no ultimate will be stolen.
- Since the copied ability scales based on *Hijack*’s rank, abilities can be copied regardless of their rank.
- *Hijack* will copy all passives, actives and recast effects of the stolen ability. *Hijack* will not trigger the effects of other abilities unless absolutely necessary.
- Shape-shifting ultimates will transform **Sylas** into the target's alternate form, replacing his basic abilities and **base** stats. Namely: Cougar Form, Spider Form, Cannon Form and Dragon Form.
  - *Dragon's Descent* transforms **Sylas**' mana, into *Shyvana*’s fury, being able to replenishing it by basic attacking. At the end of the effect, **Sylas** gains his previous mana.
  - *Dragon's Descent* does not refresh ability cooldowns upon transforming.
  - *Spider Form* freezes Sylas' original cooldowns on their current values, causing them to not come off cooldown during Spider Form.
  - Upon transforming back from Cannon Form, Sylas' next basic attack deals Transform Mercury Hammer's bonus magic damage and uses his critical strike animation.
  - Transforming into Cannon Form with more than 2 stacks of Unshackled and then casting Hypercharge will limit its effect until the currently held Unshackled stacks are spent.
  - Transforming into *Cougar Form* with more than 2 stacks of Unshackled and then casting Takedown will disable Takedown's effect until the buff timer runs out.
  - Permanent transformations last for ~5 minutes or until toggled off.
  - The alternate abilities of transforming champions will cost mana and scale with the ranks as appropriate for the ability (e.g. ranks in R for Nidalee and ranks in Q/W/E for Jayce/Elise).
  - Upon transforming, a Petricite Burst stack will be deducted.
  - Petricite Burst stacks will still be gained from abilities cast in alternate forms, but the stacks are gained independently from the normal ones.
    - If **Sylas** has both normal and transformed stacks, *Petricite Burst* will prioritize consuming **Sylas** stacks before the transformed ones.
- Ultimates comply with One for All rules if an ally is playing the hijacked champion. For example, an allied Darius’ Hemorrhage stacks will increase the damage of the hijacked Noxian Guillotine; or an allied Syndra’s Dark Sphere will add extra damage to hijacked Unleashed Power as well as producing 3 spheres that she can use. ;Ultimate Interactions
- Ahri’s Spirit Rush - Gains a charge upon scoring a champion takedown within 3 seconds of damaging them.
- Akshan’s Comeuppance - **Sylas** can cast Abscond, but doing so ends the channel prematurely. If **Comeuppance** is cast after Abduct but before the chains hit a target, **Sylas** isn't pulled to the target and the channel continues as usual.
- Amumu’s Curse of the Sad Mummy - Does not apply Cursed Touch.
- Aphelios’ Moonlight Vigil - Based on the main weapon at the moment of steal.
  - Crescendum empowers his basic attacks temporarily.
  - Calibrum does not apply marks.
  - Severum grants a shield if healed while at full health.
- Ashe’s Enchanted Crystal Arrow - Does not apply Frost Shot to secondary targets.
- Brand’s Pyroclasm - Does not apply stacks of Blaze.
- Cho'Gath’s Feast - Gains permanent *Feast* stacks on-kill.
- Corki’s Missile Barrage - Gains max charges.
- Darius’ Noxian Guillotine - Does not apply a stack of Hemorrhage. Upon gaining a reset from a Rank 3 *Noxian Guillotine*, **Sylas** can hold onto the ability for up to 300 seconds.
- Fiora’s Grand Challenge - Gains movement speed in the effect and deals bonus Vitals true damage and can trigger the area healing.
- Galio’s Hero's Entrance - Gives **Sylas** and allies Shield of Durand's magic shield, based on Kingslayer rank.
- Gangplank’s Cannon Barrage - Does not steal cannon upgrades.
- Gnar’s GNAR! - Gains a usable ultimate regardless of cast on Gnar or Mega Gnar.
- Gwen’s Needlework - Does not apply A Thousand Cuts.
- Heimerdinger’s UPGRADE!!! - **Sylas**’s basic abilities are replaced with *Heimerdinger*’s empowered abilities. The upgrade buff lasts until the ultimate can be stolen again.
- Illaoi’s Leap of Faith - Does not reduce cooldown of Kingslayer.
- Irelia’s Vanguard's Edge - Applies marks of Bladesurge.
- Jax’s Grandmaster-at-Arms - Applies magic damage on-hit while holding onto **Grandmaster-at-Arms** and while it is active.
- Kassadin’s Riftwalk - Only provides one use.
- Kai'Sa’s Killer Instinct - **Sylas**' basic attacks, Abduct and nearby allies' immobilizations apply stacks of Plasma, up to 4. **Sylas** is unable to apply the 5th stack necessary to expunge the stacks.
- Kalista’s Fate's Call - Only castable with a Black Spear (purchasable in the Shop for 0 gold gold).
- Karma’s Mantra - **Sylas**' basic abilities are replaced with *Karma*’s empowered abilities, according to **Sylas**' own ability ranks (including *Hijack* for Mantra bonuses).
- Katarina’s Death Lotus - Can be cast without a target.
- Kayn’s Umbral Trespass - Based on current form (Shadow Assassin / Rhaast).
- Kha'Zix’s Void Assault - Can steal the ability's evolution bonus effects.
- Kennen’s Slicing Maelstrom - Applies Mark of the Storm.
- Kled’s Chaaaaaaaarge!!! - Does not need mount to cast.
- Kog'Maw’s Living Artillery - Only provides one use.
- LeBlanc’s Mimic - Uses the empowered version of LeBlanc last cast basic ability. Mimic Sigil of Malice does not apply its mark. If she has not cast any abilities, Mimic Sigil of Malice is copied.
- Leona’s Solar Flare - Does not apply Sunlight.
- Lillia’s Lilting Lullaby - **Sylas**' abilities apply Dream-Laden Bough’s debuff, which does not deal its damage over time.
- Lucian’s The Culling - Can cast Abscond and Abduct freely during the channel.
- Lux’s Final Spark - Does not apply Illumination.
- Master Yi’s Highlander - Takedowns reduce basic ability cooldowns while holding onto *Highlander* and while it is active.
- Mel’s Golden Eclipse - **Sylas**' basic attacks and abilities apply stacks of Overwhelm, which will execute targets below the threshold.
- Mordekaiser’s Realm of Death - Can join the *dimension* of outgoing realms if cast in the proper range *(does not extend their duration)*.
- Nasus’ Fury of the Sands - Does not reduce cooldown of Chain Lash.
- Neeko’s Inherent Glamour - Always steals Pop Blossom, regardless of the champion Neeko is disguised as.
- Orianna’s Command: Shockwave - Cast on self without a Ball.
- Pantheon’s Grand Starfall - The spear deals damage based on Chain Lash rank and scales with 46% AP.
- Quinn’s Behind Enemy Lines - Does not cast Skystrike upon attacking an enemy champion. It can be casted manually however.
- Rengar’s Thrill of the Hunt - **Sylas** will gain Unseen Predator on his next attack.
- Renekton’s Dominus - Grants **Sylas** 20 mana on cast and then 5 mana per second (up to 75 total mana over the duration).
- Rek'Sai’s Void Rush - **Sylas**' attacks against enemy champions will apply a mark which can be used to cast the ability.
- Riven’s Blade of the Exile - Grants AD based on **Sylas**' AP on cast and grants him 75 **bonus** attack range for his auto attacks aswell for Chain Lash and Kingslayer.
- Samira’s Inferno Trigger - Can use without a Grade.
- Seraphine’s Encore - Allies do not gain Notes.
- Shaco’s Hallucinate - Creates a **Sylas** Clone that also copies **Sylas**' stolen ultimate indicator. The clone behaves like a **Shaco** clone, being able to Backstab, apply Two-Shiv Poison's slow scaling with Abscond rank, and summoning mini-boxes that deal damage based on Hijack rank.
- Shyvana’s Dragon's Descent - Has a 200 second base cooldown.
- Skarner’s Impale - Can freely cast abilities during the suppression, but cannot basic attack or Flash.
- Sylas’ Hijack - Steals the ability that the enemy **Sylas** has stolen. Cannot be targeted if no ability has been stolen.
- Syndra’s Unleashed Power - Throws a minimum of 3 spheres, which are left on the ground afterwards and vanish after 6 seconds.
- Talon’s Shadow Assault - Does not apply Blade's End.
- Tahm Kench’s Devour - *Sylas** basic attacks apply An Acquired Taste, allowing him to cast *Devour* on enemies.
- Teemo’s Noxious Trap - Gains max charges.
- Tristana’s Buster Shot - Always has 525 cast range as **Sylas** does not benefit from Draw a Bead’s per-level cast range increase.
- Twitch’s Spray and Pray - Gains 300 attack range and his attacks produce missiles that travel 850 range. These attacks are classified as melee.
- Udyr - Steals Wingborne Storm, and can recast to Awaken it. Effects are based on *Hijack*’s rank.
- Urgot’s Fear Beyond Death - Can cast Kingslayer during the recast.
- Varus’ Chain of Corruption - Applies stacks of Blight to targets hit that can be detonated by **Sylas**' other abilities, dealing damage based on Kingslayer rank.
- Vayne’s Final Hour - Gains the AD and MS bonuses. Additionally, **Sylas** gains invisibility when he casts Chain Lash.
- Vel'Koz’s Life Form Disintegration Ray - Does not apply Organic Deconstruction.
- Viego’s Sovereign's Domination - Always steals Heartbreaker, regardless of the champion Viego is possessing.
- Viktor’s Arcane Storm - If Viktor has augmented the ability, **Sylas' ** hijacked version will also have the augment's effects.
- Xayah’s Featherstorm - Feathers do not interact with an ally or enemy Xayah.
- Yasuo’s Last Breath - Critical strikes ignore 60% of the target's **bonus** armor after activation.
- Yunara’s R - Grants Q’s active effects, but no other bonuses.
- Yuumi’s Final Chapter - Can freely cast abilities during the channel.
- Zac’s Let's Bounce! - Cannot declare attacks nor use any abilities nor while bouncing. **Sylas** does not create chunks.
- Zeri’s Lightning Crash - Can gain *Overcharge*’s effects and refresh it on basic attacks (even without being charged) and abilities.
- Zed’s Death Mark - *Shadow* does not mimic abilities but can be reactivated to swap places.

---

## Patch History

### V25.18
- Petricite Burst
  - Monster damage reduced to 130% from 150%.

### V25.16#August 14th Hotfix|V25.16
- Petricite Burst
  - **Bug Fixes:** Critical strikes no longer factor the spell's damage again after the attack's base damage, causing them to deal much more damage than intended.

### V25.16
- Petricite Burst
  - Monster damage increased to 150% from 100%.

### V25.13
- Hijack
  - **Bug Fixes:** Initiating a

### Rift Herald#Summoned form|Rift Herald Rodeo
  - **Bug Fixes:** Initiating a Teleport / Unleashed Teleport channel while incurring the cast animation of a stolen ultimate no longer unintentionally casts the associated spell a second time.
  - **Bug Fixes:** After initiating a Teleport / Unleashed Teleport channel while his model is transformed during all relevant stolen ultimate abilities, **Sylas** will now properly return to his actual model instead of keeping the transformed model until his death.

### V25.12
- Abduct
  - **Bug Fixes:** Fixed a bug that caused it to apply Kingslayer’s damage and heal multiple times.

### V25.10
- Hijack
  - **Bug Fixes:** Now properly converts copied Ragnarok’s AD ratios into AP ratios.

### V25.07
- Hijack
  - **Bug Fixes:** Now correctly incurs its slot cooldown upon copying Zeri’s Lightning Crash.

### V25.S1.1
- Sylas
  - **Bug Fixes:** Restored wings and props during the Recall animation.

### V14.22
- Stats
  - Health growth reduced to 122 from 129.

## Trivia

- 
- Sylas is the first champion to have an animation when he cancels his Recall.
- Sylas was the first champion released in 2019.

---
*This page was automatically generated from League of Legends Wiki data.*