# Template checklist — expanded

Generated: 2025-10-10T15:54:19.190Z

Batch 14 of 33 — items 261..280

## Items

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AItems%2Fdoc.txt
- Decoded name: Template:Items/doc
- Namespace: Template
- Remainder: Items/doc
- Path parts: Template:Items / doc

```
{{Documentation subpage}}
;Description
: Formats the two lists of items: [[Template:Items/List]] for League of Legends and [[Template:Items/WR list]] for Wild Rift.

__NOTOC__
<includeonly>[[Category:Section formatting templates]]
[[de:Vorlage:Navigation Gegenstand]]</includeonly>
```

### Notes

_No notes specified._

---

## Items/List

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AItems%2FList%2Fdoc.txt
- Decoded name: Template:Items/List/doc
- Namespace: Template
- Remainder: Items/List/doc
- Path parts: Template:Items / List / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
: ''For the {{tip|Wild Rift}} version, see [[Template:Items/WR list]].''

;Description
: List of items for League of Legends.
: The contents are generated automatically using getRoster from [[Module:ItemData]] which automatically categorizes items by type.

;Syntax
: <code><nowiki>{{Items/List}}</nowiki></code>

;Usage
: Meant to be used in main navigation pages only.

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:General wiki templates]]
</includeonly>
```

### Notes

_No notes specified._

---

## JoJ Banner

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AJoJ%20Banner%2Fdoc.txt
- Decoded name: Template:JoJ Banner/doc
- Namespace: Template
- Remainder: JoJ Banner/doc
- Path parts: Template:JoJ Banner / doc

```
{{Documentation subpage}}

;Description
Article header banner on the [[Journal of Justice]].

;Example
<pre>{{JoJ Banner
|file  = Ralston FarnsleySquare.png
|text1 = The Journal of Justice
|text2 = An open letter from the Editor-in-Chief
|text3 = Senior [[Summoner]] Ralston Farnsley reporting from the [[Institute of War]]
}}</pre>


{{JoJ Banner
|file  = Ralston FarnsleySquare.png
|text1 = The Journal of Justice
|text2 = An open letter from the Editor-in-Chief
|text3 = Senior [[Summoner]] Ralston Farnsley reporting from the [[Institute of War]]
}}
```

### Notes

_No notes specified._

---

## Jungle monster stat

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AJungle%20monster%20stat%2Fdoc.txt
- Decoded name: Template:Jungle monster stat/doc
- Namespace: Template
- Remainder: Jungle monster stat/doc
- Path parts: Template:Jungle monster stat / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: Automatically generates leveled stats for jungle monsters.

;Syntax
: <code>{{t|Jungle monster stat|value|stat}}</code>

;Example
: <code>{{tl|Jungle monster stat|30|ad}}</code>
: {{Jungle monster stat|30|ad}}

: <code>{{tl|Jungle monster stat|120|exp}}</code>
: {{Jungle monster stat|120|exp}}

: <code>{{tl|Jungle monster stat|2050|hp}}</code>
: {{Jungle monster stat|2050|hp}}

;See also
* 
```

### Notes

_No notes specified._

---

## Jungle pet info

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AJungle%20pet%20info%2Fdoc.txt
- Decoded name: Template:Jungle pet info/doc
- Namespace: Template
- Remainder: Jungle pet info/doc
- Path parts: Template:Jungle pet info / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: Show jungle pet information and details.

;Syntax
: <code>{{t|Jungle pet info}}</code>
{{Jungle pet info}}

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Data templates]]
</includeonly>
```

### Notes

_No notes specified._

---

## Keyword color

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AKeyword%20color%2Fdoc.txt
- Decoded name: Template:Keyword color/doc
- Namespace: Template
- Remainder: Keyword color/doc
- Path parts: Template:Keyword color / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
''This template uses the Lua Module [[Module:Color|Color]].''

A list of colors associated with keywords. If any keyword, in the following order, is found in <code><nowiki>{{{1}}}</nowiki></code>, it returns the associated hex color. If no keyword is found, "inherit" is returned (when used in CSS, this means the color will be unchanged from its surrounding text).

Note that keywords' capitalization does not matter.

The current list: 
{| class="wikitable"
|+ League of Legends
! Keyword !! Color
|-
| <code>heal</code><br /> || <span style="color:{{Keyword color|heal}}">{{#explode:{{Keyword color|heal}}|# |1}}</span>
|-
| <code>heal and shield power</code><br /><code>HSP</code> || <span style="color:{{Keyword color|heal and shield power}}">{{#explode:{{Keyword color|heal and shield power}}|# |1}}</span>
|-
| <code>health</code><br /><code>HP</code> || <span style="color:{{Keyword color|hp}}">{{#explode:{{Keyword color|hp}}|# |1}}</span>
|-
| <code>ability haste</code><br /><code>AH</code><br /><code>haste</code> || <span style="color:{{Keyword color|ah}}">{{#explode:{{Keyword color|ah}}|# |1}}</span>
```

### Notes

_No notes specified._

---

## Krakens

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AKrakens%2Fdoc.txt
- Decoded name: Template:Krakens/doc
- Namespace: Template
- Remainder: Krakens/doc
- Path parts: Template:Krakens / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

;Description
:Used to display Kraken costs for the temporary game mode [[Black Market Brawlers]]; It will produce an [[Krakens]] icon and the link to its page.

;Syntax
:Type <code>{{t|krakens|cost|3=size=icon size}}</code> at any part of the article.

;Example
:<code>{{tl|krakens}}</code>
:* {{krakens}}

:<code>{{tl|krakens|100}}</code>
:* {{krakens|100}}

:<code>{{tl|krakens|100|3=size=30}}</code>
:* {{krakens|100|size=30}}

<includeonly>
```

### Notes

_No notes specified._

---

## L

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AL%2Fdoc.txt
- Decoded name: Template:L/doc
- Namespace: Template
- Remainder: L/doc
- Path parts: Template:L / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: Links to a page in the LoR namespace and removes the namespace from the displayed text.

;Syntax
: {{t|L|link|alt text}}
*'''1''': Link to the LoR page without its namespace. Deafult: ''Legends of Runeterra''
*'''2''': Alternative text. Otherwise displays '''1'''.

;Usage
:{{tl|L|The Path of Champions}}
{{l|The Path of Champions}}

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Templates]]
</includeonly>
```

### Notes

_No notes specified._

---

## Lc

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ALc%2Fdoc.txt
- Decoded name: Template:Lc/doc
- Namespace: Template
- Remainder: Lc/doc
- Path parts: Template:Lc / doc

```
This template takes in an input string and displays it in bold format and small-character capitals. By default, it displays the string as a list item, but it can be prevented from doing so by adding "nolist=" as a field.

===Syntax===
<pre>{{lc|Attack Speed}}</pre>
<pre>{{lc|Attack Speed|nolist=}}</pre>

===Result===
{{lc|Attack Speed}}
{{lc|Attack Speed|nolist=}}

[[Category:Template documentation]]
```

### Notes

_No notes specified._

---

## LegendPoints

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ALegendPoints%2Fdoc.txt
- Decoded name: Template:LegendPoints/doc
- Namespace: Template
- Remainder: LegendPoints/doc
- Path parts: Template:LegendPoints / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

;Description
:This template will produce a [[LoR:Progression#Legend Levels|Legend Points]] icon, a mechanic of [[LoR:The Path of Champions|The Path of Champions]] and the link to its page. You can also adjust the size of the icon and change its text. For more options, see the [[Module:ImageLink]].

;Syntax
:Type <code>{{t|LegendPoints|text|3=size=icon size}}</code> at any part of the article.

;Example
:<code>{{tl|LegendPoints}}</code>
:* {{LegendPoints}}

:<code>{{tl|LegendPoints|2000}}</code>
:* {{LegendPoints|20}}

:<code>{{tl|LegendPoints|8000|3=size=30}}</code>
:* {{LegendPoints|8000|size=30}}

<includeonly>
```

### Notes

_No notes specified._

---

## Legends of Runeterra

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ALegends%20of%20Runeterra%2Fdoc.txt
- Decoded name: Template:Legends of Runeterra/doc
- Namespace: Template
- Remainder: Legends of Runeterra/doc
- Path parts: Template:Legends of Runeterra / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
:Navigation template for most/all {{tip|Legends of Runeterra}} content.

;Syntax
:{{t|Legends of Runeterra|2=hide=true/false}}
: hide defaults to true


{{Legends of Runeterra}}

;See also
* [[Template:Legends of Runeterra/Game Mechanics]]
* [[Template:Legends of Runeterra/Keywords]]
* [[Template:Legends of Runeterra/List of cards]]
* [[Template:Legends of Runeterra/Sets]]

<includeonly>
<!-- Categories and interwikis go here: -->
```

### Notes

_No notes specified._

---

## Legends of Runeterra/Game Mechanics

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ALegends%20of%20Runeterra%2FGame%20Mechanics%2Fdoc.txt
- Decoded name: Template:Legends of Runeterra/Game Mechanics/doc
- Namespace: Template
- Remainder: Legends of Runeterra/Game Mechanics/doc
- Path parts: Template:Legends of Runeterra / Game Mechanics / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: Navigation template for all Legends of Runeterra gameplay mechanics.

;Syntax
: {{t|Legends of Runeterra/Game Mechanics|2=hide=true/false}}
: hide defaults to true

{{Legends of Runeterra/Game Mechanics}}

;See also
* [[Template:Legends of Runeterra]]
* [[Template:Legends of Runeterra/List of cards]]
* [[Template:Legends of Runeterra/Keywords]]
* [[Template:Legends of Runeterra/Sets]]

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Navigation templates]]
```

### Notes

_No notes specified._

---

## Legends of Runeterra/Keywords

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ALegends%20of%20Runeterra%2FKeywords%2Fdoc.txt
- Decoded name: Template:Legends of Runeterra/Keywords/doc
- Namespace: Template
- Remainder: Legends of Runeterra/Keywords/doc
- Path parts: Template:Legends of Runeterra / Keywords / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: Navigation template for all Legends of Runeterra keywords.

;Syntax
: {{t|Legends of Runeterra/Keywords}}


{{Legends of Runeterra/Keywords}}

;See also
* [[Template:Legends of Runeterra]]
* [[Template:Legends of Runeterra/Game Mechanics]]
* [[Template:Legends of Runeterra/List of cards]]
* [[Template:Legends of Runeterra/Sets]]

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Navigation templates]]
```

### Notes

_No notes specified._

---

## Legends of Runeterra/List of cards

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ALegends%20of%20Runeterra%2FList%20of%20cards%2Fdoc.txt
- Decoded name: Template:Legends of Runeterra/List of cards/doc
- Namespace: Template
- Remainder: Legends of Runeterra/List of cards/doc
- Path parts: Template:Legends of Runeterra / List of cards / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: Navigation template for all lists of cards in Legends of Runeterra.

;Syntax
: {{t|Legends of Runeterra/List of cards|2=hide=true/false}}
: hide defaults to false

{{Legends of Runeterra/List of cards}}

;See also
* [[Template:Legends of Runeterra]]
* [[Template:Legends of Runeterra/Game Mechanics]]
* [[Template:Legends of Runeterra/Keywords]]
* [[Template:Legends of Runeterra/Sets]]

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Navigation templates]]
```

### Notes

_No notes specified._

---

## Legends of Runeterra/Sets

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ALegends%20of%20Runeterra%2FSets%2Fdoc.txt
- Decoded name: Template:Legends of Runeterra/Sets/doc
- Namespace: Template
- Remainder: Legends of Runeterra/Sets/doc
- Path parts: Template:Legends of Runeterra / Sets / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: Navigation template for all Legends of Runeterra expansions/sets.

;Syntax
: {{t|Legends of Runeterra/Sets|2=hide=true/false}}
: hide defaults to false

{{Legends of Runeterra/Sets}}

;See also
* [[Template:Legends of Runeterra]]
* [[Template:Legends of Runeterra/Game Mechanics]]
* [[Template:Legends of Runeterra/List of cards]]
* [[Template:Legends of Runeterra/Keywords]]

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Navigation templates]]
```

### Notes

_No notes specified._

---

## LessOrEqual

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ALessOrEqual%2Fdoc.txt
- Decoded name: Template:LessOrEqual/doc
- Namespace: Template
- Remainder: LessOrEqual/doc
- Path parts: Template:LessOrEqual / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
Use in formulas to have proper non-breaking space placement; linebreaks will only occur after an operator, and not before.

{{Math symbols}}

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Character formatting templates]]
[[de:Vorlage:Grad]]
[[pt-br:Predefinição:MenorOuIgual]]
</includeonly>
```

### Notes

_No notes specified._

---

## Lethality

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ALethality%2Fdoc.txt
- Decoded name: Template:Lethality/doc
- Namespace: Template
- Remainder: Lethality/doc
- Path parts: Template:Lethality / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

== Usage ==
Can display the formula for [[Lethality]] into Armor penetration. Can also display a specified amount of lethality in argument 1.

Note that this formula is obsolete as of [[V14.1|patch 14.1]]. The formula used here is the most recent version before that patch. ''For changes to the formula itself, refer to [[V6.22]] and [[V7.14]].''
;Syntax
<code>{{t|Lethality|Formula}}</code><br>
<code>{{t|Lethality|Number}}</code>

;Example
* <code>{{tl|Lethality|formula}}</code>
: {{Lethality|formula}}

* <code>{{tl|Lethality|10}}</code>
: {{Lethality|10}}

* <code>{{tl|Lethality|18}}</code>
: {{Lethality|18}}
```

### Notes

_No notes specified._

---

## Level up

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ALevel%20up%2Fdoc.txt
- Decoded name: Template:Level up/doc
- Namespace: Template
- Remainder: Level up/doc
- Path parts: Template:Level up / doc

```
{{Documentation subpage}}
;Description
:This template is used to create an ability level up feature into the {{t|abilities}} template.
;Syntax
:Type <code>{{t|level up|Characteristic|Units|1st Level|2nd Level|3rd Level|4th Level|5th Level|6th Level}}</code> in the ability "level" tag.
;Tag Description
*'''Characteristic:''' Cooldown, Cost, Damage, Magic Damage, Multiplier, Attack Speed, etc.
*'''Units:''' seconds, mana, health, energy, etc.
*'''1st, 2nd, 3rd, 4th, 5th, 6th Levels:''' Any number (include percent signs when applicable).
;Example
<code><nowiki>{{level up|Cost|Health.|95|105|115|125|135|145}}</nowiki></code>
*{{level up|Cost|Health.|95|105|115|125|135|145}}
<code><nowiki>{{level up|Bonus Damage||5%|10%|15%|20%|25%}}</nowiki></code>
*{{level up|Bonus Damage||5%|10%|15%|20%|25%}}

<includeonly>[[Category:Deprecated templates|{{PAGENAME}}]]</includeonly>
<noinclude></noinclude>
```

### Notes

_No notes specified._

---

## Leveled Stat

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ALeveled%20Stat%2Fdoc.txt
- Decoded name: Template:Leveled Stat/doc
- Namespace: Template
- Remainder: Leveled Stat/doc
- Path parts: Template:Leveled Stat / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

;Description
:This template will return the value of a champion's stat at a certain level.
;Syntax
:<code>{{tl|Leveled Stat|''name''|''level''|''stat''}}</code>
{| class="wikitable"
! Stat
! Meaning
|-
| <code>dps</code> || Damage * Attack speed
|-
| <code>dam</code> || Damage
|-
| <code>as</code> || Attack Speed
|-
| <code>hp</code> || Health
|-
| <code>hp5</code> || Health regen
```

### Notes

_No notes specified._

---

## Limited

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ALimited%2Fdoc.txt
- Decoded name: Template:Limited/doc
- Namespace: Template
- Remainder: Limited/doc
- Path parts: Template:Limited / doc

```
{{Documentation subpage}}
;Description
:This template is used to mark articles that have information about limited edition subjects.
;Syntax
:<code>{{t|limited}}</code> or <code>{{t|limited|type}}</code>
*'''type''' should direct to a valid Category:Removed_<nowiki><type></nowiki>, currently: ''content'' (default), ''items'', ''masteries'', or ''monsters''.
<includeonly>[[Category:Article management templates|{{PAGENAME}}]]</includeonly>
```

### Notes

_No notes specified._

---
