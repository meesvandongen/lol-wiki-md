# Sylas

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
| **Champion** | Sylas |
| **Title** | the Unshackled |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2019-01-25 |
| **Release Patch** | V9.2 |
| **Roles** | Burst, Skirmisher |
| **Riot Positions** | Middle |
| **External Positions** | Top, Middle |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $600.0$ | $+122.0$ | $2674.0$ |
| **Mana** | $400.0$ | $+70.0$ | $1590.0$ |
| **Health Regen** | $9.0$ | $+0.9$ | $24.3$ |
| **Mana Regen** | $8.0$ | $+0.8$ | $21.6$ |
| **Armor** | $29.0$ | $+5.2$ | $117.4$ |
| **Magic Resist** | $32.0$ | $+2.55$ | $75.3$ |
| **Attack Damage** | $61.0$ | $+3.0$ | $112.0$ |
| **Attack Speed** | $0.645$ | $+3.5\%$ | $1.029$ |
| **Movement Speed** | $340.0$ | $+0.0$ | $340.0$ |
| **Attack Range** | $175.0$ | $+0.0$ | $175.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.645$ |
| **Attack Speed Ratio** | $0.645$ |
| **Bonus AS per Level** | $3.5\%$ |
| **Attack Windup** | $16.8\%$ |
| **Acquisition Radius** | $525 units$ |
| **Pathing Radius** | $35 units$ |
| **Selection Radius** | $100 units$ |
| **Selection Height** | $135 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Petricite Burst

**Innate:** **Sylas**’s ability casts generate stacks of *Unshackled* that stack up to a cap. While he has stacks, he gains tremendous .

**Innate:** Whenever **Sylas** casts an ability, he generates a stack of *Unshackled* for 4 seconds, refreshing on subsequent casts and stacking up to 3 times. **Unshackled:** **Sylas**' next basic attack gains and is empowered to consume a stack to whirl his chains around him, which has an uncancellable windup and deals 130% AD (+ 30% AP) magic damage to the primary target and 40% AD (+ 20% AP) magic damage to nearby enemies. *Unshackled* deals 130% damage to monsters, and executes minions that are secondary targets and would be left below *25 health*. *Petricite Burst* can critical strike for critical damage only against the primary target.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Self |
| **Damage Type** | Magic |
| **Effect Radius** | 300 units |
| **Spell Shield** | False |
| **Spell Effects** | Special |

**Notes:**

- Spellblade damage does not get converted to magic damage, and will deal its damage only to the primary target.
- Applies proc damage to the primary target and area damage to secondary targets.
- 'Petricite Burst's damage cannot critically strike against secondary targets.
- *Petricite Burst* only applies on-hit effects to the primary target.
- *Petricite Burst* attacks benefit from .
- *Petricite Burst* can be dodge, block, or missed while **Sylas** is blind as the primary target but not as a secondary target.

---

### Q: Chain Lash

**Active:** **Sylas** lashes out two chains that converge at the target location, dealing magic damage to enemies hit and slow them for a brief moment.

*After a brief delay, the intersection explodes to deal magic damage to enemies within.*

**Active:** **Sylas** lashes out two chains that converge to the target location and extend beyond it up to a maximum range, dealing magic damage to enemies hit and slow them for $1.5$ seconds. After a $0.6$-second delay, the chains' intersection explodes to deal magic damage to enemies within, reduced to 40% against minions.

| Attribute | Value |
|-----------|-------|
| **Range** | 50 / 775 units |
| **Cooldown** | $10-6$ seconds |
| **Cast Time** | $0.4$ seconds |
| **Cost** | 55 Mana |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | 180 / 200 units |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | False |

**Scaling:**
- **Magic Damage:** $40-120$ (+ 40% AP)
- **Slow:** $15-35$%
- **Magic Damage:* $60-280$ (+ 80% AP)*Minion Damage:** $60×0.4-280×0.4$ (+ $800.4$% AP)0.4)-120+(280×0.4)$ (+ $40+(80×0.4)$% AP)

**Notes:**

- The detonation's effect radius center is placed at the intersection between the chains from the initial cast, meaning a max range cast can create a detonation that reaches further than the chains do.

---

### W: Kingslayer

**Active:** **Sylas** dashes to the target enemy and strikes them to deal magic damage.

*If this hits a champion, **Sylas** is also heal, increased based on his **missing** health.*

**Active:** **Sylas** dashes to the front of the target enemy's location then strikes them to deal magic damage. If this damages a champion, **Sylas** is also heal, increased by type=his **missing** health.

| Attribute | Value |
|-----------|-------|
| **Range** | 400 units |
| **Cooldown** | $12-6$ seconds |
| **Cast Time** | none |
| **Cost** | $50-90$ Mana |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Single target |

**Scaling:**
- **Magic Damage:** $75-215$ (+ 60% AP)
- **Minimum Heal:** $20-100$ (+ 20% AP) (+ 5% of his **bonus* health)2-100×2$ (+ $20×2$% AP) (+ $5×2$% of his
- **bonus** health)

**Notes:**

- The damage will be dealt if the dash completed uninterrupted, and regardless of how far the target's location is at the end of the dash.
  - **Sylas** will heal even if the target champion dies before the dash is completed.
  - The only exception is if the target becomes untargetable, in which case *Kingslayer* will have no effect.

---

### E: Abduct

**Active:** **Sylas** whips his chains in the target direction that deal magic damage and briefly stun the first enemy hit, standard sight them. He will then dash to the target and briefly airborne upon arrival.

**Active:** **Sylas** whips out his chains in the target direction that deal magic damage to the first enemy hit and standard sight and stun them for $0.5$ seconds. Upon hitting the target, **Sylas** dash to their location and airborne for $0.5$ seconds upon arrival. **Sylas is lockout to cast **Chain Lash* while the chains are in flight.*

| Attribute | Value |
|-----------|-------|
| **Cast Time** | $0.25$ seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Speed** | 2500 - 400 / 1800 units/second |
| **Spell Shield** | True |
| **Spell Effects** | Single target |
| **Projectile** | True |

**Scaling:**
- **Magic Damage:** $80-280$ (+ 80% AP)

**Notes:**

- **Sylas** will not dash to his target if the chains are blocked by spell shield.
- The target will be revealed for 2 seconds if the dash is interrupted.
- *Abduct* will still apply the airborne if the target is untargetable by the end of the dash. Effect at cast time start
- **Sylas** is unable to perform actions while dashing to the target, and becomes able to again after $0.05$ seconds of the dash ending.
- **Sylas** will not dash to the target hit if he is channel or dash from a *Hijacked* ability.
  - *Abduct* will not apply the airborne when this happens.
  - If the chains connect before a pre-channel cast time is complete then Sylas will dash to the target and the ability will cancel.

---

### E: Abscond

**Active:** **Sylas** dashes to the target location. He can then cast **Abduct** within a short time.

**Active:** **Sylas** dashes to the target location. Within $3.5$ seconds, he can cast **Abduct** after a $0.2$-second delay from casting *Abscond*. **Kingslayer* can be cast during the dash.*

| Attribute | Value |
|-----------|-------|
| **Range** | 400 units |
| **Cooldown** | $13-9$ seconds |
| **Cast Time** | none |
| **Cost** | 65 Mana |
| **Targeting** | Location |
| **Affects** | Self |
| **Speed** | 1450 units/second |

**Notes:**

- No additional details.

---

### R: Hijack

**Active:** **Sylas** launches his chains at the target enemy champion, gaining a copy of their ultimate ability and briefly true sight them. He cannot steal from the same champion again for a while, and can hold the *hijacked* ability for some time, during which he can recast *Hijack*.

**Recast:** **Sylas** casts his *hijacked* ability at no cost.

**Active:** **Sylas** launches his chains at the target enemy champion, gaining a copy of their ultimate ability and true sight them for $0.825$ seconds. **Sylas** cannot select the same champion again for a set duration, and can hold the *hijacked* ultimate for up to 90 seconds, during which he can recast *Hijack*. **Recast:** **Sylas** casts his hijacked ultimate ability at no cost, scaling based on 'Hijack's rank and his own statistics. *Hijacked* ultimates and abilities that do not scale with have their ratios converted to ability power ratios, scaling with $0.6$% AP per 1% **total** AD, and $0.4$% AP per 1% *bonus AD respectively.

| Attribute | Value |
|-----------|-------|
| **Range** | 950 units |
| **Cooldown** | $80-30$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | 75 Mana |
| **Targeting** | Unit |
| **Affects** | Self, Enemies |
| **Speed** | 2200 units/second |
| **Spell Shield** | True |
| **Projectile** | True |

**Notes:**

- '*Petricite Burst*'s* stack from casting *Hijack' is given after **Sylas** receives the Hijacked ability.
- *Hijack**s per target cooldown is indicated by a under recently targetted enemy champions, and is only visible to **Sylas*' and each individual enemy.
- 'Hijack's base chain missile will only be destroyed and not replicated against Rebuttal. **Sylas** will not receive a Hijacked *Golden Eclipse*.
- If **Sylas** casts *Hijack* on a clone, no ultimate is stolen and the ability will be placed on full cooldown.
  - The same does not occur for champions within a zombie state.
- clone do not copy 'Hijack's target cooldown indicator, potentially revealing them to **Sylas**.
  - Inherent Glamour does not hide her personal cooldown indicator, nor does she copy her allies'. ** If the same happens when casting an on-target stolen ability, the ability will instead cancel and go on cooldown.
- If *Hijack* is intercepted by Unbreakable, **Sylas** will gain a copy of *R* instead and an on-target cooldown will be applied to him.
  - If **Braum** has an on-target cooldown on him (by being previously *Hijacked*) upon interception, no ultimate will be stolen.
- Since the copied ability scales based on 'Hijack's rank, abilities can be copied regardless of their rank.
- *Hijack* will copy all passives, actives and recast effects of the stolen ability. *Hijack* will not trigger the effects of other abilities unless absolutely necessary.
- Shape-shifting ultimates will transform **Sylas** into the target's alternate form, replacing his basic abilities and **base** stats. Namely: *Cougar Form*, *Spider Form*, *Cannon Form* and *Dragon Form*.
  - 'Dragon's Descent' transforms **Sylas**' mana, into 'Shyvana's fury, being able to replenishing it by basic attacking. At the end of the effect, **Sylas** gains his previous mana.
  - 'Dragon's Descent' does not refresh ability cooldowns upon transforming.
  - *Spider Form* freezes Sylas' original cooldowns on their current values, causing them to not come off cooldown during Spider Form.
  - Upon transforming back from *Cannon Form*, Sylas' next basic attack deals *Transform Mercury Hammer*'s bonus magic damage and uses his critical strike animation.
  - Transforming into *Cannon Form* with more than 2 stacks of Unshackled and then casting *Hypercharge* will limit its effect until the currently held Unshackled stacks are spent.
  - Transforming into *Cougar Form* with more than 2 stacks of Unshackled and then casting *Takedown* will disable Takedown's effect until the buff timer runs out.
  - Permanent transformations last for ~5 minutes or until toggled off.
  - The alternate abilities of transforming champions will cost mana and scale with the ranks as appropriate for the ability (e.g. ranks in R for Nidalee and ranks in Q/W/E for Jayce/Elise).
  - Upon transforming, a *Petricite Burst* stack will be deducted.
  - *Petricite Burst* stacks will still be gained from abilities cast in alternate forms, but the stacks are gained independently from the normal ones. ** If Sylas*** has both normal and transformed stacks, *Petricite Burst* will prioritize consuming **Sylas** stacks before the transformed ones.
- Ultimates comply with One for All rules if an ally is playing the hijacked champion. For example, an allied Hemorrhage stacks will increase the damage of the hijacked *Noxian Guillotine*; or an allied Dark Sphere will add extra damage to hijacked *Unleashed Power* as well as producing 3 spheres that she can use. ;Ultimate Interactions
- Spirit Rush - Gains a charge upon scoring a champion takedown within 3 seconds of damaging them.
- Comeuppance - **Sylas** can cast *Abscond*, but doing so ends the channel prematurely. If **Comeuppance** is cast after *Abduct* but before the chains hit a target, **Sylas** isn't pulled to the target and the channel continues as usual.
- Curse of the Sad Mummy - Does not apply *Cursed Touch*.
- Moonlight Vigil - Based on the *main weapon* at the moment of steal.
  - *Crescendum* empowers his basic attacks temporarily.
  - *Calibrum* does not apply marks.
  - *Severum* grants a shield if healed while at full health.
- Enchanted Crystal Arrow - Does not apply *Frost Shot* to secondary targets.
- Pyroclasm - Does not apply stacks of *Blaze*.
- Feast - Gains permanent *Feast* stacks on-kill.
- Missile Barrage - Gains max charges.
- Noxian Guillotine - Does not apply a stack of *Hemorrhage*. Upon gaining a reset from a Rank 3 *Noxian Guillotine*, **Sylas** can hold onto the ability for up to 300 seconds.
- Grand Challenge - Gains movement speed in the effect and deals bonus *Vitals* true damage and can trigger the area healing.
- Hero's Entrance - Gives **Sylas** and allies *Shield of Durand's* magic shield, based on *Kingslayer* rank.
- Cannon Barrage - Does not steal cannon upgrades.
- GNAR! - Gains a usable ultimate regardless of cast on **Gnar** or **Mega Gnar**.
- Needlework - Does not apply *A Thousand Cuts*.
- UPGRADE!!! - ''Sylas's* basic abilities are replaced with *Heimerdinger's empowered abilities. The upgrade buff lasts until the ultimate can be stolen again.
- Leap of Faith - Does not reduce *cooldown* of *Kingslayer*.
- Vanguard's Edge - Applies marks of *Bladesurge*.
- Grandmaster-at-Arms - Applies magic damage on-hit while holding onto **Grandmaster-at-Arms** and while it is active.
- Riftwalk - Only provides one use.
- Killer Instinct - **Sylas**' basic attacks, *Abduct* and nearby allies' immobilize apply stacks of *Plasma*, up to 4. **Sylas** is unable to apply the 5th stack necessary to expunge the stacks.
- Fate's Call - Only castable with a *Black Spear* (purchasable in the Shop for 0 gold gold).
- Mantra - **Sylas**' basic abilities are replaced with 'Karma's empowered abilities, according to **Sylas**' own ability ranks (including *Hijack* for Mantra bonuses).
- Death Lotus - Can be cast without a target.
- Umbral Trespass - Based on current form (**Shadow Assassin** / **Rhaast**).
- Void Assault - Can steal the ability's *evolution* bonus effects.
- Slicing Maelstrom - Applies *Mark of the Storm*.
- Chaaaaaaaarge!!! - Does not need *mount* to cast.
- Living Artillery - Only provides one use.
- Mimic - Uses the empowered version of **LeBlanc** last cast basic ability. *Mimic Sigil of Malice* does not apply its mark. If she has not cast any abilities, *Mimic Sigil of Malice* is copied.
- Solar Flare - Does not apply *Sunlight*.
- Lilting Lullaby - **Sylas**' abilities apply *Dream-Laden Bough*’s debuff, which does not deal its damage over time.
- The Culling - Can cast *Abscond* and *Abduct* freely during the channel.
- Final Spark - Does not apply *Illumination*.
- Highlander - takedown reduce basic ability cooldowns while holding onto *Highlander* and while it is active.
- Golden Eclipse - **Sylas**' basic attacks and abilities apply stacks of *Overwhelm*, which will execute targets below the threshold.
- Realm of Death - Can join the *dimension* of outgoing realms if cast in the proper range *(does not extend their duration)*.
- Fury of the Sands - Does not reduce *cooldown* of *Chain Lash*.
- Inherent Glamour - Always steals *Pop Blossom*, regardless of the champion Neeko is disguised as.
- Command: Shockwave - Cast on self without a *Ball*.
- Grand Starfall - The spear deals damage based on *Chain Lash* rank and scales with 46% AP.
- Behind Enemy Lines - Does not cast *Skystrike* upon attacking an enemy champion. It can be casted manually however.
- Thrill of the Hunt - **Sylas** will gain *Unseen Predator* on his next attack.
- Dominus - Grants **Sylas** 20 mana on cast and then 5 mana per second (up to 75 total mana over the duration).
- Void Rush - **Sylas**' attacks against enemy champions will apply a *mark* which can be used to cast the ability.
- Blade of the Exile - Grants AD based on **Sylas**' AP on cast and grants him 75 **bonus** attack range for his auto attacks aswell for *Chain Lash* and *Kingslayer*.
- Inferno Trigger - Can use without a *Grade*.
- Encore - Allies do not gain *Notes*.
- Hallucinate - Creates a **Sylas** Clone that also copies **Sylas**' stolen ultimate indicator. The clone behaves like a **Shaco** clone, being able to *Backstab*, apply *Two-Shiv Poison*'s slow scaling with *Abscond* rank, and summoning *mini-boxes* that deal damage based on *Hijack* rank.
- Dragon's Descent - Has a 200 second base cooldown.
- Impale - Can freely cast abilities during the suppression, but cannot basic attack or Flash.
- Hijack - Steals the ability that the enemy **Sylas** has stolen. Cannot be targeted if no ability has been stolen.
- Unleashed Power - Throws a minimum of 3 *spheres*, which are left on the ground afterwards and vanish after 6 seconds.
- Shadow Assault - Does not apply *Blade's End*.
- Devour - *Sylas*' basic attacks apply *An Acquired Taste*, allowing him to cast *Devour* on enemies.
- Noxious Trap - Gains max charges.
- Buster Shot - Always has 525 cast range as **Sylas** does not benefit from *Draw a Bead*’s per-level cast range increase.
- Spray and Pray - Gains 300 attack range and his attacks produce missiles that travel 850 range. These attacks are classified as melee.
- **Udyr** - Steals *Wingborne Storm*, and can recast to *Awaken* it. Effects are based on 'Hijack's rank.
- Fear Beyond Death - Can cast *Kingslayer* during the recast.
- Chain of Corruption - Applies stacks of *Blight* to targets hit that can be detonated by **Sylas**' other abilities, dealing damage based on *Kingslayer* rank.
- Final Hour - Gains the AD and MS bonuses. Additionally, **Sylas** gains invisibility when he casts *Chain Lash*.
- Life Form Disintegration Ray - Does not apply *Organic Deconstruction*.
- Sovereign's Domination - Always steals *Heartbreaker*, regardless of the champion Viego is possessing.
- Arcane Storm - If Viktor has *augmented* the ability, ''Sylas'' hijacked version will also have the augment's effects.
- Featherstorm - *Feathers* do not interact with an ally or enemy **Xayah**.
- Last Breath - Critical strike ignore 60% of the target's **bonus armor** after activation.
- R - Grants *Q*’s active effects, but no other bonuses.
- Final Chapter - Can freely cast abilities during the channel.
- Let's Bounce! - Cannot declare attacks nor use any abilities nor while bouncing. **Sylas** does not create *chunks*.
- Lightning Crash - Can gain 'Overcharge's effects and refresh it on basic attacks (even without being charged) and abilities.
- Death Mark - *Shadow* does not mimic abilities but can be reactivated to *swap* places.

---

## Patch History

### V25.18
- *Petricite Burst*
  - Monster damage reduced to 130% from 150%.
- *Petricite Burst*
  - **Bug Fixes:** Critical strikes no longer factor the spell's damage again after the attack's base damage, causing them to deal much more damage than intended.

### V25.16
- *Petricite Burst*
  - Monster damage increased to 150% from 100%.

### V25.13
- *Hijack*
  - **Bug Fixes:** Initiating a Rift Herald Rodeo channel while incurring the cast time of a stolen ultimate now properly consumes the slot.
  - **Bug Fixes:** Initiating a Teleport / Unleashed Teleport channel while incurring the cast animation of a stolen ultimate no longer unintentionally casts the associated spell a second time.
  - **Bug Fixes:** After initiating a Teleport / Unleashed Teleport channel while his model is transformed during all relevant stolen ultimate abilities, **Sylas** will now properly return to his actual model instead of keeping the transformed model until his death.

### V25.12
- *Abduct*
  - **Bug Fixes:** Fixed a bug that caused it to apply *Kingslayer*’s damage and heal multiple times.

### V25.10
- *Hijack*
  - **Bug Fixes:** Now properly converts copied *Ragnarok*’s AD ratios into AP ratios.

### V25.07
- *Hijack*
  - **Bug Fixes:** Now correctly incurs its slot cooldown upon copying Lightning Crash.
- Sylas
  - **Bug Fixes:** Restored wings and props during the Recall animation.

### V14.22
- Stats
  - Health growth reduced to 122 from 129.

### V14.17
- *Kingslayer*
  - Damage AP ratio reduced to 60% AP from 70% AP.

### V14.16
- Stats
  - Base health increased to 600 from 575.
- *Petricite Burst*
  - Primary target AP ratio increased to 30% AP from 25% AP.
- *Kingslayer*
  - Base damage increased to $75-215$ from $65-205$.
- *Hijack*
  - **Bug Fixes:** Now properly keeps the passive effects of Grandmaster-at-Arms and Highlander for the duration of the active spells.

### V14.15
- Stats
  - Base armor increased to 29 from 27.
- *Chain Lash*
  - Explosion base damage reduced to $60-280$ from $70-290$.
  - Explosion AP ratio reduced to 80% AP from 90% AP.
  - Cooldown reduced to $10-6$ seconds from $11-7$.
- *Kingslayer*
  - Damage AP ratio reduced to 70% AP from 80% AP.
  - Heal AP ratio reduced to 20% AP from 35% AP.
  - **New Effect:** Heal now scales with 5% of his **bonus** health.
- *Abduct*
  - AP ratio reduced to 80% AP from 100% AP.

## Trivia

- 
  - In Sylas's case, *Hijack* infinitely stacks his health if he steals Feast.
- Sylas is the first champion to have an animation when he cancels his Recall.
- Sylas was the first champion released in 2019.

---
*This page was automatically generated from League of Legends Wiki data.*