# Template checklist — expanded

Generated: 2025-10-10T15:54:19.124Z

Batch 1 of 33 — items 1..20

## Ability bar

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3AAbility%20bar%2Fdoc.txt
- Decoded name: Template:Ability bar/doc
- Namespace: Template
- Remainder: Ability bar/doc
- Path parts: Template:Ability bar / doc

```
;Usage
:This template creates a replica of the in-game ability bar with tooltips for abilities, champion, health, resources, items and summoner spells. For a more customizable version, see [[Template:Custom Ability bar]].

;Syntax
:{{t|Ability bar|Champion}}
:Additional parameters:
* skin = A skin for the given champion ("Original" by default)
* variant = Variant of the champion circle
* level = Champion level
* experience = % of the experience bar to be filled up ("50%" by default)
* currenthealth = Remaining health, either a number or a %
* currentresource = Remaining resource, either a number or a %
* qrank = Number of dots to light up under the q ability
* wrank = Number of dots to light up under the w ability
* erank = Number of dots to light up under the e ability
* rrank = Number of dots to light up under the r ability
* passive = Passive icon to use, if multiple exist for the champion ("0" by default)
* q = Q icon to use, if multiple exist for the champion ("0" by default)
* w = W icon to use, if multiple exist for the champion ("0" by default)
* e = E icon to use, if multiple exist for the champion ("0" by default)
```

### Notes

Visual UI element; layout-only—exclude from markdown output.

---

## Ability data

- [x] Status: done
- [x] Include decision: include

### Source (first 20 lines)

- Source filename: Template%3AAbility%20data%2Fdoc.txt
- Decoded name: Template:Ability data/doc
- Namespace: Template
- Remainder: Ability data/doc
- Path parts: Template:Ability data / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
:''For the {{tip|Wild Rift}} version, see [[Template:WR Ability data]].

;Description
* {{t|Ability data}} defines the format of [[:Category:Ability data templates|Ability data templates]] for champion abilities in {{tip|League of Legends}} and generates their documentation.
* ''Data templates'' store data centrally, which can then be retrieved for any purpose on the Wiki. '''Ability data''' templates are most commonly used in {{t|ai}} (an icon-link template + a tooltip) and in champion articles.
* The documentation produces a table on the data template's page that lists all possible parameters and their given value. Instructions for retrieving and formatting values are then provided.

;Creation of Ability data templates
* Create new '''Ability data''' templates using the following title scheme. Ability names should be typed <u>as shown in-game</u>.
<code><nowiki>Template:Data <Name of champion>/<Name of ability></nowiki></code>
* All '''Ability data''' templates begin with:
<pre><nowiki>{{{{{1<noinclude>|Ability data</noinclude>}}}|
}}
</nowiki></pre>
: The <nowiki><noinclude> </noinclude></nowiki> tags are necessary to prevent transclusion of the entire documentation page whenever the data template is invoked.
* Data is then stored in the data template as parameter inputs. Inputs are separated from each other by a <code>|</code> vertical bar.
** The first input must be the ability's name.
** The second input must be the text <code><nowiki>{{{2|}}}</nowiki></code>
```

### Notes

Data template storing ability values — include and implement reader.

---

## Ability details

- [x] Status: done
- [x] Include decision: include

### Source (first 20 lines)

- Source filename: Template%3AAbility%20details%2Fdoc.txt
- Decoded name: Template:Ability details/doc
- Namespace: Template
- Remainder: Ability details/doc
- Path parts: Template:Ability details / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

;Core features
<pre>{{Ability details
|champion	=
|skill		= 
|targeting	= <!-- Passive | Auto | Direction | Unit | Proximity | Location | Vector | custom -->
|outofrange	= <!-- (See below) -->
|terraingrace	= <!-- True | False | custom -->
|affects	= <!-- custom -->
|damagetype	= <!-- Magic | Physical | True | Physical Magic | Physical True | Magic True | MPT | custom --> (MPT = all three)
|spelleffects	= <!-- Raw | Default | Proc | Reactive | Attack | Single | AOE | DOT | Pet | Special | Mixed | False | Unknown -->
|spellshield	= <!-- True | Special | False | na | Unknown --> 
|parry		= <!-- True | Special | False | Unknown -->
|projectile	= <!-- True | Yasuo | Special | Bypass | Unknown -->
|grounded	= <!-- True | False | Special | na | Unknown -->
|knockdown	= <!-- True | False | Special | na | Unknown -->
|silence	= <!-- True | False | Special | Unknown -->
|callforhelp	= <!-- True | False | Special | Unknown -->
```

### Notes

Provides ability metadata (targeting, effects) — include.

---

## Ability details/New

- [x] Status: done
- [x] Include decision: include

### Source (first 20 lines)

- Source filename: Template%3AAbility%20details%2FNew%2Fdoc.txt
- Decoded name: Template:Ability details/New/doc
- Namespace: Template
- Remainder: Ability details/New/doc
- Path parts: Template:Ability details / New / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

== Usage ==
Formatting for [[Template:Ability details/Infobox]] when placed within [[Template:Ability]].

This is a template to be used on all Ability Details pages to provide consistent format. See [[Template:Ability]] for practical usage.

== Syntax ==
;Variables
{| class="article-table"
! Name
! Accepted parameters
! Notes
|-
|show
|true <br/>false
| Sets whether ability details are NOT collapsed by default.
|-
|targeting
```

### Notes

New-format details template — include.

---

## Ability frame

- [x] Status: done
- [x] Include decision: include

### Source (first 20 lines)

- Source filename: Template%3AAbility%20frame%2Fdoc.txt
- Decoded name: Template:Ability frame/doc
- Namespace: Template
- Remainder: Ability frame/doc
- Path parts: Template:Ability frame / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
{{Ability frame|Q|Ability info placeholder|Ability details placeholder}}

== Usage ==
<code><nowiki>{{Ability frame|[I|Q|W|E|R]|{{Ability info|<params>}}|{{Ability details|<params>}}}}</nowiki></code>


Example:

<code><nowiki>{{Ability frame</nowiki><br/>
<nowiki>|E</nowiki><br/>
<nowiki>|{{desktop|{{Edit|Template:Data Ashe/Hawkshot|title=Click here to edit this ability.|style=position:absolute; right:13px; z-index:5}}}} {{Ability info</nowiki><br/>
<nowiki>|disp_name=Hawkshot</nowiki><br/>
<nowiki>|icon         = Hawkshot.png</nowiki><br/>
<nowiki>|description  = {{sbc|Active:}} '''Ashe''' sends a hawk spirit in the target direction.</nowiki><br/>
<nowiki>}}}}</nowiki></code>

{{Ability frame
|E
```

### Notes

Aggregates ability info/details; convert to rendered sections.

---

## Ability icon

- [x] Status: done
- [x] Include decision: include

### Source (first 20 lines)

- Source filename: Template%3AAbility%20icon%2Fdoc.txt
- Decoded name: Template:Ability icon/doc
- Namespace: Template
- Remainder: Ability icon/doc
- Path parts: Template:Ability icon / doc

```
{{Documentation subpage}}
== Description ==
This template can make a link to an ability out of its image and name, or it may show the proper image for the ability. It also needs the champion's name in order to function properly. There's also an optional third parameter that lets you specify an alternative name for the ability while specifying the link to the current ability (used for old ability names to provide icons). If an ability icon does not exist, the template will substitute a temporary image.

== Syntax ==
*<code>{{t|ai|''Ability''|''Champion''}}</code>
*<code>{{t|ai|''Ability''|''Champion''|''Alternative name''}}</code>

== Example ==
*<code>{{tl|ai|Demacian Justice|Garen}}</code>
::{{ai|Demacian Justice|Garen}}

*<code>{{tl|ai|Fox-Fire|Ahri}}</code>
::{{ai|Fox-Fire|Ahri}}

*<code>{{tl|ai|Demacian Justice|Garen|Justice Strike!}}</code>
::{{ai|Demacian Justice|Garen|Justice Strike!}}

*<code>{{tl|ai|R|Garen}}</code>
::{{ai|R|Garen}}
```

### Notes

_No notes specified._

---

## Ability info

- [x] Status: done
- [x] Include decision: include

### Source (first 20 lines)

- Source filename: Template%3AAbility%20info%2Fdoc.txt
- Decoded name: Template:Ability info/doc
- Namespace: Template
- Remainder: Ability info/doc
- Path parts: Template:Ability info / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

== Usage ==
This is the formatting for everything that is printed over [[Template:Ability frame]]. See [[Template:Ability]] for practical usage.

== Syntax ==
;Variables
Most parameters are meant to contain numbers.
* Ranges/dimensions are measured and presented in game units.
* Time is measured and presented in seconds.
{| class="article-table"
! Name
! Accepted parameters
! Notes
|- 
| skill
| custom input
| Indicates the skill order (Q, W, E, R).
|-
```

### Notes

Main ability text (name, description, cost) — include.

---

## Ability info/old

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3AAbility%20info%2Fold%2Fdoc.txt
- Decoded name: Template:Ability info/old/doc
- Namespace: Template
- Remainder: Ability info/old/doc
- Path parts: Template:Ability info / old / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

== Usage ==
This is the formatting for everything that is printed over [[Template:Ability frame]]. See [[Template:Ability]] for practical usage. This version is archived because it is the last version before the header was changed to use Fandom's infoboxes. This version also uses tables instead of a grid layout.

== Syntax ==
;Variables
Most parameters are meant to contain numbers. Characteristics that involve distance are measured in [[Range|units]].

{| class="article-table"
! Name
! Accepted parameters
! Notes
|- 
| skill
| custom input
| Indicates the skill order (Q, W, E, R).
|-
| name<br/>disp_name
```

### Notes

Deprecated format; exclude (archive only).

---

## Ability info/stats

- [x] Status: done
- [x] Include decision: include

### Source (first 20 lines)

- Source filename: Template%3AAbility%20info%2Fstats%2Fdoc.txt
- Decoded name: Template:Ability info/stats/doc
- Namespace: Template
- Remainder: Ability info/stats/doc
- Path parts: Template:Ability info / stats / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: Briefly describe what the template does here

;Syntax
:

;Example
:
<pre>
{{Ability info/stats|Cool Ability Name
|cost=Test
|costtype=Test
|cooldown=Test|cdstart=on-cast
|cast time=Test
|static=Test
|ontargetcdstatic=Test
|ontargetcd=Test
|recharge=Test
```

### Notes

Numeric stats helper — include.

---

## Ability pet

- [x] Status: done
- [x] Include decision: include

### Source (first 20 lines)

- Source filename: Template%3AAbility%20pet%2Fdoc.txt
- Decoded name: Template:Ability pet/doc
- Namespace: Template
- Remainder: Ability pet/doc
- Path parts: Template:Ability pet / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

== Usage ==
<pre>{{Ability pet}}</pre>
{{Ability pet}}

{{clr}}
==Example==
<pre>{{Ability pet
|name={{ai|Jack in the Box|Shaco}}
|image=[[File:Jack In The Box Render.png|x172px]]
|description=Jack in the Box is a static, [[Champion summoned units|autonomous minion]] that attacks nearby units. 
* Apply [[spell effects]] as a [[damage over time]] ability.
** [[File:Hextech Revolver item.png|20px|link=Spell vamp]] [[Spell vamp]] is applied.
** {{ii|Rylai's Crystal Scepter}} will apply a 15% slow.
* Does not apply [[on-hit effect]]s.
* Does not affect/is not affected by {{ai|Counter Strike|Jax}}, {{ai|Spirit's Refuge|Shen}} and [[Blind]], because the basic attacks count as spells.
* Classified as a [[minion]] for targeting purposes.
* Boxes will prioritize the nearest enemy champion that has damaged Shaco or that Shaco has damaged, preferring to attack that champion whenever it is in range, even if they have moved too far away before walking back into range.
```

### Notes

Pet-specific ability formatting — include (lower priority).

---

## Ability presentation

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3AAbility%20presentation%2Fdoc.txt
- Decoded name: Template:Ability presentation/doc
- Namespace: Template
- Remainder: Ability presentation/doc
- Path parts: Template:Ability presentation / doc

```
{{Documentation subpage}}

== Usage ==
Displays the champion name, ability name and ability description.

;Syntax
<code>{{t|abp|Champion|Ability}}</code>

;Example
* <code>{{tl|abp|Rebirth|Anivia}}</code>
: {{abp|Rebirth|Anivia}}

* <code>{{tl|abp|Flash Frost|Anivia}}</code>
: {{abp|Flash Frost|Anivia}}

* <code>{{tl|abp|Crystallize|Anivia}}</code>
: {{abp|Crystallize|Anivia}}

* <code>{{tl|abp|Frostbite|Anivia}}</code>
: {{abp|Frostbite|Anivia}}
```

### Notes

Presentation/layout-only; exclude.

---

## Ability progression

- [x] Status: done
- [x] Include decision: include

### Source (first 20 lines)

- Source filename: Template%3AAbility%20progression%2Fdoc.txt
- Decoded name: Template:Ability progression/doc
- Namespace: Template
- Remainder: Ability progression/doc
- Path parts: Template:Ability progression / doc

```
{{Documentation subpage}}__NOTOC__
;Description
: This template is used to show the progression of a value with an [[Champion ability|ability's level]] (though this template may also be used for other progressions).
* Up to 6 values can be displayed, if this limit is exceeded an error is displayed.
* In addition, line wrapping within the displayed progression is prevented.
* The [[Template:Format decimal|default decimal formatting]] is applied to the output.
* All values may be generated using mathematical operations.
* For uniformity the short version [[Template:Ap|ap]] should be used.

;Usage
: The template can be integrated anywhere with the following code: <code><nowiki>{{ap|</nowiki>{{pht|parameters}}<nowiki>}}</nowiki></code>

;Manual examples
* <code><nowiki>{{ap|50|70|90}}</nowiki></code> creates: {{ap|50|70|90}}
* <code><nowiki>{{ap|1|2|3|4|5|6}}</nowiki></code> creates: {{ap|1|2|3|4|5|6}}
* <code><nowiki>{{ap|1.5|1.7|2.1}}</nowiki></code> creates: {{ap|1.5|1.7|2.1}}

;Autocompletion
: In addition to manually entering individual values, the template can also automatically generate them with a formula that includes at least one <code>x</code>, displaying 5 values by default:
*<code><nowiki>{{ap|2*x}}</nowiki></code> creates: {{ap|2*x}} 
```

### Notes

Leveling tables and values — include.

---

## Ability scaling

- [x] Status: done
- [x] Include decision: include

### Source (first 20 lines)

- Source filename: Template%3AAbility%20scaling%2Fdoc.txt
- Decoded name: Template:Ability scaling/doc
- Namespace: Template
- Remainder: Ability scaling/doc
- Path parts: Template:Ability scaling / doc

```
{{Documentation subpage}}

;Description
: This template color-codes text. If a ''keyword'' is detected, its predefined color is applied to the text.

;Usage
: For any ability and effect scalings. See [[Template:Keyword color]] for a list of all valid keywords and their color.

;Syntax
: {{t|as|text|keyword}}
: '''text''' (Required): The text to display and detect keywords from.
: '''keyword''' (Optional): If the detected keyword is invalid or its color is not the one desired, it can be overridden by a different keyword in order to change the text's color.
: Note that keywords' capitalization (whether in the 'text' or the 'keyword' parameter) does not matter.

;Examples
<code>{{tl|as|(+ 10% maximum mana)}}</code>
:{{as|(+ 10% maximum mana)}}
<code>{{tl|as|'''additional''' Mana|Mana}}</code>
:{{as|'''additional''' Mana|Mana}}
<code>{{tl|as|(+ 100% AP)}}</code>
```

### Notes

Shows scaling (e.g. "+35% AP") — include.

---

## AS

- [x] Status: done
- [x] Include decision: include

### Source (first 20 lines)

- Source filename: Template%3AAS%2Fdoc.txt
- Decoded name: Template:AS/doc
- Namespace: Template
- Remainder: AS/doc
- Path parts: Template:AS / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

;Description
:This template will produce an [[Ancient Sparks]] icon and the link to its page. You can also adjust the size of the icon and add its cost.

;Syntax
:Type <code>{{t|AS|cost|3=size=icon size}}</code> at any part of the article.

;Example
:<code>{{tl|AS}}</code>
:* {{AS|Ancient Sparks}}

:<code>{{tl|AS|100}}</code>
:* {{AS|100}}

:<code>{{tl|AS|100|3=size=30}}</code>
:* {{AS|100|size=30}}

<includeonly>
```

### Notes

_No notes specified._

---

## Critical damage

- [x] Status: done
- [x] Include decision: include

### Source (first 20 lines)

- Source filename: Template%3ACritical%20damage%2Fdoc.txt
- Decoded name: Template:Critical damage/doc
- Namespace: Template
- Remainder: Critical damage/doc
- Path parts: Template:Critical damage / doc

```
{{Documentation subpage}}

;Usage
:<code><nowiki>{{critical damage|main crit damage|Infinity Edge crit ratio|mod=value|base=value|ie=critical damage stat}}</nowiki></code>

The intent of this template is to easily update all [[:Category:Champions with critical strike ratios|applicable]] articles when there's a general change in {{ii|Infinity Edge}} or {{tip|critical strike}} interactions with a simple change in the template, while also allowing U.R.F. scalings to be added autonomously while the game mode is available.

;Example
* Case: IE damage will factor off the main crit damage bonus.
** <code><nowiki>{{critical damage|100}}</nowiki></code> --> {{critical damage|100}}
** <code><nowiki>{{critical damage|50}}</nowiki></code> --> {{critical damage|50}}

* Case: IE damage will factor off another defined crit damage bonus. (e.g. champion critical modifier, special cased abilities)
** <code><nowiki>{{critical damage|50|150}}</nowiki></code> --> {{critical damage|50|150}}

* Case: Both IE and the base crit bonus are affected by the same modifier. This paramater is to avoid manually multiplying both ratios. 
** <code><nowiki>{{critical damage|175|100|mod=0.9}}</nowiki></code> --> {{critical damage|175|100|mod=0.9}}

* Case: You need to output the '''bonus''' damage, but the mod is calculated from '''total'''. Again, this is to avoid having to manually perform the calculations.
** <code><nowiki>{{critical damage|175|100|mod=0.9|base=100}}</nowiki></code> --> {{critical damage|175|100|mod=0.9|base=100}}
```

### Notes

_No notes specified._

---

## Item availability

- [x] Status: done
- [x] Include decision: include

### Source (first 20 lines)

- Source filename: Template%3AItem%20availability%2Fdoc.txt
- Decoded name: Template:Item availability/doc
- Namespace: Template
- Remainder: Item availability/doc
- Path parts: Template:Item availability / doc

```
{{Documentation subpage}}

== Usage ==
<pre>{{Item availability|Common}}</pre>
{{Item availability|Common}}

<pre>{{Item availability|S}}</pre>
{{Item availability|S}}

<pre>{{Item availability|TC}}</pre>
{{Item availability|TC}}

<pre>{{Item availability|STCH}}</pre>
{{Item availability|STCH}}

== See also ==

<includeonly>
[[Category:Deprecated templates]]

```

### Notes

Item availability text/data — include.

---

## Item availability compact

- [x] Status: done
- [x] Include decision: include

### Source (first 20 lines)

- Source filename: Template%3AItem%20availability%20compact%2Fdoc.txt
- Decoded name: Template:Item availability compact/doc
- Namespace: Template
- Remainder: Item availability compact/doc
- Path parts: Template:Item availability compact / doc

```
{{Documentation subpage}}

== Usage ==
<pre>{{Item availability compact|S}}</pre>
{{Item availability compact|S}}

<pre>{{Item availability compact|STHN}}</pre>
{{Item availability compact|STHN}}

<pre>{{Item availability compact|STCH}}</pre>
{{Item availability compact|STCH}}

== See also ==
[[Template:Item availability]]

<includeonly>
[[Category:Deprecated templates]]
</includeonly>
```

### Notes

Compact variant — include (lower priority).

---

## Item group

- [x] Status: done
- [x] Include decision: include

### Source (first 20 lines)

- Source filename: Template%3AItem%20group%2Fdoc.txt
- Decoded name: Template:Item group/doc
- Namespace: Template
- Remainder: Item group/doc
- Path parts: Template:Item group / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: Puts a link to the [[item group]] page with the corresponding item group.

;Syntax
: <code>{{t|ig|item group|alt}}</code>

;Example
: <code>{{tl|ig|Boots}}</code>
:: {{ig|Boots}}

: <code>{{tl|ig|Anti-CC|Quicksilver}}</code>
:: {{ig|Anti-CC|Quicksilver}}

;See also
* [[Template:Named item effect]]

<includeonly>
<!-- Categories and interwikis go here: -->
```

### Notes

Grouping for item lists — include.

---

## Item haste table

- [x] Status: done
- [x] Include decision: include

### Source (first 20 lines)

- Source filename: Template%3AItem%20haste%20table%2Fdoc.txt
- Decoded name: Template:Item haste table/doc
- Namespace: Template
- Remainder: Item haste table/doc
- Path parts: Template:Item haste table / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: This template is used to show a list of all effects that grant '''item haste'''.

;Syntax
: <code>{{t|Item haste table}}</code>
{{Item haste table}}

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Data templates]]
</includeonly>
```

### Notes

Haste table data — include.

---

## Item header

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3AItem%20header%2Fdoc.txt
- Decoded name: Template:Item header/doc
- Namespace: Template
- Remainder: Item header/doc
- Path parts: Template:Item header / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

== Usage ==
This template populates the beginning of item pages using item data stored in item data templates, including the sections:
* Infobox (Template:Item infobox)
* Introductory paragraph
* Item recipe section

The template also detects whether the item is removed or champion-specific.

== Syntax ==
{{t|Item header|item name}}

== See also ==
* [[Template:Infobox item]]
* [[Template:Item data]]
* [[Template:Recipe]]

<includeonly>
```

### Notes

Layout/header for item infobox — exclude.

---
