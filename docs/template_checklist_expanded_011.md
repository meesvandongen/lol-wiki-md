# Template checklist — expanded

Generated: 2025-10-10T15:54:19.175Z

Batch 11 of 33 — items 201..220

## Essence

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AEssence%2Fdoc.txt
- Decoded name: Template:Essence/doc
- Namespace: Template
- Remainder: Essence/doc
- Path parts: Template:Essence / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

;Description
:This template will produce an [[LoR:Essence|Essence]] icon, a currency of {{tip|Legends of Runeterra}} game and the link to its page. You can also adjust the size of the icon and add its cost.

;Syntax
:Type <code>{{t|Essence|cost|3=size=icon size}}</code> at any part of the article.

;Example
:<code>{{tl|Essence}}</code>
:* {{Essence}}

:<code>{{tl|Shard|100}}</code>
:* {{Essence|100}}

:<code>{{tl|Essence|100|3=size=30}}</code>
:* {{Essence|100|size=30}}

<includeonly>
```

### Notes

_No notes specified._

---

## Events by occasion

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AEvents%20by%20occasion%2Fdoc.txt
- Decoded name: Template:Events by occasion/doc
- Namespace: Template
- Remainder: Events by occasion/doc
- Path parts: Template:Events by occasion / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: This template is intended to be placed at the bottom of every event article corresponding to the specified event. To be paired with [[Template:Events by year]].

;Syntax
:{{t|Events by occasion}}
{{Events by occasion}}

;Advanced Syntax
:{{t|Events by occasion|Harrowing}}
{{Events by occasion|Harrowing}}

:{{t|Events by occasion|Battle Pass}}
{{Events by occasion|Battle Pass}}

;Example
An event, such as [[Lunar Beasts (2021)]], the 2021st occurrence of the Lunar Revel event, should have:
:{{t|Events by occasion|Lunar Revel}}{{t|Events by year|2021}}
{{Events by occasion|Lunar Revel}}
```

### Notes

_No notes specified._

---

## Events by year

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AEvents%20by%20year%2Fdoc.txt
- Decoded name: Template:Events by year/doc
- Namespace: Template
- Remainder: Events by year/doc
- Path parts: Template:Events by year / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: This template is intended to be placed at the bottom of every event article corresponding to the current year. To be paired with [[Template:Events by occasion]].

;Syntax
:{{t|Events by year}}
{{Events by year}}

;Advanced Syntax
:{{t|Events by year|2015}}
{{Events by year|2015}}

;Example
An event, such as [[Lunar Beasts (2021)]], the 2021st occurrence of the Lunar Revel event, should have:
:{{t|Events by occasion|Lunar Revel}}{{t|Events by year|2021}}
{{Events by occasion|Lunar Revel}}
{{Events by year|2021}}

<includeonly>
```

### Notes

_No notes specified._

---

## Exhibition

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AExhibition%2Fdoc.txt
- Decoded name: Template:Exhibition/doc
- Namespace: Template
- Remainder: Exhibition/doc
- Path parts: Template:Exhibition / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

;Description
: This template creates a gallery of character squares and adds captions of the character's name to each.
: The square and caption both link to the character's article in the Universe namespace.
: It uses [[Module:Exhibition]].

;Syntax
: <code><nowiki>{{Exhibition|Name 1, Name 2, Name 3, ...}}</nowiki></code>
: Each name must be separated by a comma plus a space.

;Usage
: Primarily meant for exhibiting characters of a certain region, species, group, clan, etc. within location or faction articles.

;Example
: {{tl|Exhibition|K'Sante, Lissandra, Nunu, Olaf, Sejuani, Tahm Kench}}
{{Exhibition|K'Sante, Lissandra, Nunu, Olaf, Sejuani, Tahm Kench}}

<includeonly>
```

### Notes

_No notes specified._

---

## Exists

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AExists%2Fdoc.txt
- Decoded name: Template:Exists/doc
- Namespace: Template
- Remainder: Exists/doc
- Path parts: Template:Exists / doc

```
{{Documentation subpage}}
This template tests whether a given page exists. It can be useful in view of the maximum number of #ifexist calls that can be used on a page. (However, in the case of frivolous use of #ifexist which was the reason for the limit, do not replace this use with frivolous use of this template!)

The full syntax for using it is:

 <nowiki>{{</nowiki>exists | page=''page'' | then=''result if page exists'' | else=''result if page does not exist'' }}

The parameters <code>then</code> and <code>else</code> are optional; if not given, the default values for <code>then</code> and <code>else</code> are ''1'' and ''0'' respectively.  The page name may alternatively be given as the first unnamed parameter, as in this short version:

 <nowiki>{{</nowiki>exists | ''page'' }}

The method is based on transclusion of the possibly existing page. If the page does not exist the code for page transclusion produces a (red) link. Even if this result is used for comparison only, and not for final output, the page counts as being transcluded. Therefore the page appears as red link in the list of transcluded pages.

If the page to be transcluded does not exist, then link brackets are put around the full pagename, with the following conversions (visible with [[Special:ExpandTemplates]]):
*a colon is put in front if it is not there yet
*the namespace name (if applicable) and the pagename are capitalized (on most wikis)
*underscores are replaced by spaces

Thus <code><nowiki>{{help:qq w_x}}</nowiki></code> and <code><nowiki>{{:help:qq w_x}}</nowiki></code> both give <code><nowiki>[[:Help:Qq w x]]</nowiki></code>.

```

### Notes

_No notes specified._

---

## Expedition Token

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AExpedition%20Token%2Fdoc.txt
- Decoded name: Template:Expedition Token/doc
- Namespace: Template
- Remainder: Expedition Token/doc
- Path parts: Template:Expedition Token / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

;Description
:This template will produce an [[Expedition Token (Legends of Runeterra)|Expedition Token]] icon, a currency of {{tip|Legends of Runeterra}} game and the link to its page. You can also adjust the size of the icon and add its cost.

;Syntax
:Type <code>{{t|ExpTk|cost|3=size=icon size}}</code> at any part of the article.

;Example
:<code>{{tl|ExpTk}}</code>
:* {{ExpTk}}

:<code>{{tl|ExpTk|100}}</code>
:* {{ExpTk|100}}

:<code>{{tl|ExpTk|100|3=size=30}}</code>
:* {{ExpTk|100|size=30}}

<includeonly>
```

### Notes

_No notes specified._

---

## Experience granted section

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AExperience%20granted%20section%2Fdoc.txt
- Decoded name: Template:Experience granted section/doc
- Namespace: Template
- Remainder: Experience granted section/doc
- Path parts: Template:Experience granted section / doc

```
{{Documentation subpage}}
{{Removed}}
== Usage ==
<pre>{{Experience granted section|125|large|firstclear=50%|altname=Ancient Krug}}</pre>
{{Experience granted section|125|large|firstclear=50%|altname=Ancient Krug}}

----

<pre>{{Experience granted section|{{pp|115;129;143;158;172;186;201;215;230|1;2;3;4;5;6;7;8;9}}|xpmin=115|xpmax=230|large|altname=Rift Scuttler}}</pre>
{{Experience granted section|{{pp|115;129;143;158;172;186;201;215;230|1;2;3;4;5;6;7;8;9}}|xpmin=115|xpmax=230|large|altname=Rift Scuttler}}

----

<pre>{{Experience granted section|11|altname=Mini Krug}}</pre>
{{Experience granted section|11|altname=Mini Krug}}
__NOTOC__
<includeonly>
[[Category:Deprecated templates]]
</includeonly>
```

### Notes

_No notes specified._

---

## Experimental

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AExperimental%2Fdoc.txt
- Decoded name: Template:Experimental/doc
- Namespace: Template
- Remainder: Experimental/doc
- Path parts: Template:Experimental / doc

```
#REDIRECT [[Template:Ambox documentation]][[Category:Template documentation]]
```

### Notes

_No notes specified._

---

## Experimental Hexplate ultimate interactions

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AExperimental%20Hexplate%20ultimate%20interactions%2Fdoc.txt
- Decoded name: Template:Experimental Hexplate ultimate interactions/doc
- Namespace: Template
- Remainder: Experimental Hexplate ultimate interactions/doc
- Path parts: Template:Experimental Hexplate ultimate interactions / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

;Description
: Displays a list of champion ultimate abilities that have special-cased interactions with {{iis|Experimental Hexplate}} Overdrive.

;Syntax
: <code>{{t|Experimental Hexplate ultimate interactions}}</code>
{{Experimental Hexplate ultimate interactions}}

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Data templates]]
</includeonly>
```

### Notes

_No notes specified._

---

## ExplodeCount

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AExplodeCount%2Fdoc.txt
- Decoded name: Template:ExplodeCount/doc
- Namespace: Template
- Remainder: ExplodeCount/doc
- Path parts: Template:ExplodeCount / doc

```
{{Documentation subpage}}
== Usage ==
<nowiki>{{ExplodeCount|<string>|<delimiter>}}</nowiki>

;Example
* <nowiki>{{ExplodeCount|Bananas, Apples, Oranges, Pears, Peaches|,}}</nowiki>
* Result: {{ExplodeCount|Bananas, Apples, Oranges, Pears, Peaches|,}}

Currently only supports up to 11 variables.

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Logic templates]]
</includeonly>
```

### Notes

_No notes specified._

---

## Faction icon

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AFaction%20icon%2Fdoc.txt
- Decoded name: Template:Faction icon/doc
- Namespace: Template
- Remainder: Faction icon/doc
- Path parts: Template:Faction icon / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

{{t|fi|faction name|(optional) display text}}
;Description
: This template defines the names of "factions" or regions across Runeterra that can then be added into a page as an icon-link.
: It links to the Universe namespace. Cases where an article of the same name does not exist are handled by redirects.
:: ''Note that the input for this template is not case-sensitive, but the links it creates are. It is highly advised to always use the correct capitalization in the input, in order to avoid linking to the wrong redirect page or having to create new redirects for different capitalizations (the first letter is excluded, i.e., "the Freljord" and "The Freljord" are both valid; "The void" is invalid.)

;Usage
: {{t|fi|name|link&#61;}}
* '''name''' (Required): The name of the faction or region.
* '''link=''' (Optional): Override the target link.
{| class="article-table"
! Input
! Aliases
! Output
|-
| Arctic || || {{fi|Arctic}}
|-
```

### Notes

_No notes specified._

---

## Factions

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AFactions%2Fdoc.txt
- Decoded name: Template:Factions/doc
- Namespace: Template
- Remainder: Factions/doc
- Path parts: Template:Factions / doc

```
{{Documentation subpage}}
;Description
This template is primarily used for faction lore pages.
;Usage
{{t|Factions}}

;Customization
* <code>|hide=false</code> expand the box, which is otherwise collapsed by default.

;Example
{{Factions}}

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Lore templates]]
[[Category:Navigation templates]]
[[de:Vorlage:Navigation Fraktion]]
</includeonly>
```

### Notes

_No notes specified._

---

## Factorial

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AFactorial%2Fdoc.txt
- Decoded name: Template:Factorial/doc
- Namespace: Template
- Remainder: Factorial/doc
- Path parts: Template:Factorial / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
This template can be used to output the factorial of 0-10.
;Syntax
*<code>{{t|Factorial|integer}}</code>
;Example
*<code><nowiki>{{Factorial|</nowiki>2<nowiki>}}</nowiki></code> results in: {{Factorial|2}}
*<code><nowiki>{{Factorial|</nowiki>8<nowiki>}}</nowiki></code> results in: {{Factorial|8}}
*<code><nowiki>{{Factorial|</nowiki>One<nowiki>}}</nowiki></code> results in: {{Factorial|One}}

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Logic templates]]
[[de:Vorlage:Fakultät]]
</includeonly>
```

### Notes

_No notes specified._

---

## Fairuse

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AFairuse%2Fdoc.txt
- Decoded name: Template:Fairuse/doc
- Namespace: Template
- Remainder: Fairuse/doc
- Path parts: Template:Fairuse / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

;Description
: This template is used to mark [[Riot Games Inc.]] assets as fair use, and categorize those assets under the [[:Category:Assets copyrighted to Riot Games]].

;Syntax
: Type <code>{{t|fairuse}}</code> on the image information page.

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Image wiki templates|{{PAGENAME}}]]
[[Category:License]]
[[de:Vorlage:Fairuse]]
[[ru:Шаблон:Fairuse]]
</includeonly>
```

### Notes

_No notes specified._

---

## Fake

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AFake%2Fdoc.txt
- Decoded name: Template:Fake/doc
- Namespace: Template
- Remainder: Fake/doc
- Path parts: Template:Fake / doc

```
#REDIRECT [[Template:Ambox documentation]][[Category:Template documentation]]
```

### Notes

_No notes specified._

---

## Fandom

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AFandom%2Fdoc.txt
- Decoded name: Template:Fandom/doc
- Namespace: Template
- Remainder: Fandom/doc
- Path parts: Template:Fandom / doc

```
{{Documentation subpage}}
;Usage
This is a shorthand template for linking to articles on another {{w|Fandom (website)|fandom}} (aka {{w|wiki}}) beyond of the League of Legends Wiki. Full URLs affect web tracking in a way that FANDOM doesn't want and so you're discouraged from using full URLs to link between Fandoms.

;Syntax
 You may use either {{t|fandom}} or {{t|f}} to access this template.

To link to a fandom, simply use:
 {{t|f|fandom name}}

{{t|f|worm}} -> {{f|Worm}}

The word "Wiki" is automatically added.

To link to an article, the template syntax is:
 {{t|f|fandom name|article title}}

{{t|f|Marvel|Hulk}} -> {{f|Marvel|Hulk}}

To link to an article by name but showing different inline text while linking to it, the template syntax is:
```

### Notes

_No notes specified._

---

## File info

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AFile%20info%2Fdoc.txt
- Decoded name: Template:File info/doc
- Namespace: Template
- Remainder: File info/doc
- Path parts: Template:File info / doc

```
{{Documentation subpage}}
== Usage ==
Draws a table to contain information of the source of a file.

== Syntax ==
;Variables
{| class="article-table"
! Name
! Accepted parameters
! Notes
|- 
| desc
| custom string
| Writes in the Description cell. The title of a post or work should be ''italicized''.
|-
| source
| custom string
| Writes in the Source cell. The date of the source can be written here in parenthesis () after the source.
|-
| author
```

### Notes

_No notes specified._

---

## Flag

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AFlag%2Fdoc.txt
- Decoded name: Template:Flag/doc
- Namespace: Template
- Remainder: Flag/doc
- Path parts: Template:Flag / doc

```
{{Documentation subpage}}
A fork of [[Template:A]]. That template should probably be split up into its composite parts, but that would be a huge undertaking.

{| class="article-table"
! Usage
! Example
|-
| {{t|flag|active}} || {{flag|active}}
|-
| {{t|flag|inactive}} || {{flag|inactive}}
|-
| {{t|flag|new}} || {{flag|new}}
|-
| {{t|flag|removed}} || {{flag|removed}}
|-
| {{t|flag|available}} || {{flag|available}}
|-
| {{t|flag|unavailable}} || {{flag|unavailable}}
|-
| {{t|flag|prestige shop}} || {{flag|prestige shop}}
```

### Notes

_No notes specified._

---

## FlipText

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AFlipText%2Fdoc.txt
- Decoded name: Template:FlipText/doc
- Namespace: Template
- Remainder: FlipText/doc
- Path parts: Template:FlipText / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: This template displays one of two pieces of text, switching to the other upon clicking on the text. It can accept all strings and other templates.

;Syntax
: {{t|ft|text1|text2}}
: text1 is visible first.
: Both parameters are required.

;Usage
* Within champion abilities, please only use FlipText in the following circumstances:
** Two ways of presenting the same information
*** Applied Damage Over Times - e.g. {{ai|Noxious Blast|Cassiopeia}}: {{ft|Per Instance|Total Damage}} or {{ai|Ranger's Focus|Ashe}}: {{ft|Per Attack|Total Damage}}. 
*** Complicated scalings/formulas - e.g. {{as|{{ft|(+1% per X AP)|(+X% per 100 AP)}}}}.
*** Shortening/expanding scalings/formulas - e.g. {{ai|Arcane Mastery|Ryze}}: {{ft|20{{plus}}(''5{{times}}'''Ryze's''' level'')|{{pp|25 to 110}}}}
** Self-Modifiers - e.g. {{cis|Fizz}} abilities with and without {{ai|Chum the Waters|Fizz}}
** Hide information irrelevant to gameplay - e.g. {{cai|Rampant Growth|Zyra}} cooldown is affected by its own passive reduction ({{ft|16.7 / 15.4 / 14.1 / 12.9 / 11.7 second cooldown|17 / 16 / 15 / 14 / 13}}).
* Do '''not''' use FlipText for the following:
** Different damage to secondary targets - e.g. {{ais|Enchanted Crystal Arrow|Ashe}} damage to main target and secondary targets. 
```

### Notes

_No notes specified._

---

## Format decimal

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AFormat%20decimal%2Fdoc.txt
- Decoded name: Template:Format decimal/doc
- Namespace: Template
- Remainder: Format decimal/doc
- Path parts: Template:Format decimal / doc

```
{{Documentation subpage}}
This template takes a decimal number and formats the visual presentation of all the numbers after the decimal point to be reduced in font size. This only affects numbers and any characters inputted after the affected numbers will be displayed at full size.

Examples:
*<code>{{tl|fd|12.34}}</code>
**{{fd|12.34}}
*<code>{{tl|fd|56.789%}}</code>
**{{fd|56.789%}}

The function <code><nowiki>{{#invoke:fd|getmulti}}</nowiki></code> allows for the insertion of multiple decimals numbers.

Example:
<code><nowiki>{{#invoke:fd|getmulti|1.23% to 4.56%}}</nowiki></code> creates: {{#invoke:fd|getmulti|1.23% to 4.56%}}

== See also ==
*Module basis: [[Module:Fd|Fd]] 
*Template: [[Template:Passive progression|Passive progression]]

<includeonly>
[[Category:Formatting templates|{{PAGENAME}}]]
```

### Notes

_No notes specified._

---
