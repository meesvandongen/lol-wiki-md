# Template checklist — expanded

Generated: 2025-10-10T15:54:19.180Z

Batch 12 of 33 — items 221..240

## Forumheader

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AForumheader%2Fdoc.txt
- Decoded name: Template:Forumheader/doc
- Namespace: Template
- Remainder: Forumheader/doc
- Path parts: Template:Forumheader / doc

```
{{Documentation subpage}}
;Description
:This template is used at the top of certain forum pages. See [[Help:Wiki-style forums]] for more information.

;Usage
:This template gets automatically preloaded when creating a new forum thread. The actual page that will be preloaded is a subpage of this template, named after the forum category. The current available preloads are:
:*[[Template:Forumheader/Game discussion]]
:*[[Template:Forumheader/Help desk]]
:*[[Template:Forumheader/League of Legends Wiki Issues]]
:*[[Template:Forumheader/Manual of Style]]
:When adding a new forum category, be sure to create the appropriate preload.

;Syntax
:<code>{{t|Forumheader|Name of forum}}</code>.

;See also
:*[[Template:SubForumheader]]

<includeonly>
[[Category:Community templates|{{PAGENAME}}]]
```

### Notes

_No notes specified._

---

## From Wikimedia

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AFrom%20Wikimedia%2Fdoc.txt
- Decoded name: Template:From Wikimedia/doc
- Namespace: Template
- Remainder: From Wikimedia/doc
- Path parts: Template:From Wikimedia / doc

```
{{Documentation subpage}}
;Description
:This template is used to mark images as having been uploaded on [[wikipedia:|Wikipedia]] or another [[wikimedia:|Wikimedia]] project.
;Syntax
:Type <code>{{t|From Wikimedia}}</code> on the image information page.

<includeonly>[[Category:Image wiki templates|{{PAGENAME}}]]</includeonly><noinclude></noinclude>
```

### Notes

_No notes specified._

---

## GalleryHelper

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AGalleryHelper%2Fdoc.txt
- Decoded name: Template:GalleryHelper/doc
- Namespace: Template
- Remainder: GalleryHelper/doc
- Path parts: Template:GalleryHelper / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: This template helps add images and create proper image galleries within most [[:Category:Infobox templates|infobox templates]]. It utilizes [[Module:GalleryHelper]].
: This template should '''only''' be used inside the appropriate template parameters. It has no function otherwise. Refer to each template's documentation for how to use GalleryHelper in a specific template.

;Usage
: In an infobox's image parameter (usually something like <code>image=</code> or <code>gallery=</code>, among others) type the following. This adds one image to the infobox, and is equivalent to entering the file name in the parameter without using this template.
<pre>
|(parameter name)= {{GalleryHelper
    |file 1
    }}
</pre>
: More images can be added, which should be separated by a '''vertical bar''' <code>|</code> character. This creates a tabber where each image is placed under its own tab.
<pre>
|(parameter name)= {{GalleryHelper
    |file 1
    |file 2
    |file 3
    |etc.
```

### Notes

_No notes specified._

---

## Game

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AGame%2Fdoc.txt
- Decoded name: Template:Game/doc
- Namespace: Template
- Remainder: Game/doc
- Path parts: Template:Game / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: Quick hacky wikilink template to shorten page links after moving game pages.

;Syntax
: {{t|game|game name}}

;Example
: {{t|game|Teamfight Tactics}}

;See also
* 

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Formatting templates]]
</includeonly>
```

### Notes

_No notes specified._

---

## Game modes

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AGame%20modes%2Fdoc.txt
- Decoded name: Template:Game modes/doc
- Namespace: Template
- Remainder: Game modes/doc
- Path parts: Template:Game modes / doc

```
{{Documentation subpage}}
;Description
:This navbox is used for Game Modes Articles, to link them to the other modes.
;Syntax
:Type <code>{{t|game modes}}</code> at the end of the article.
<includeonly>
[[Category:Navigation templates]]

[[de:Vorlage:Navigation Spielmodus]]
</includeonly>
```

### Notes

_No notes specified._

---

## Gameplay elements

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AGameplay%20elements%2Fdoc.txt
- Decoded name: Template:Gameplay elements/doc
- Namespace: Template
- Remainder: Gameplay elements/doc
- Path parts: Template:Gameplay elements / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

;Description
: Navigation footer for articles of champion gameplay mechanics.

;Syntax
: {{tl|Gameplay elements}}

;See also
* [[:Category:Gameplay elements]]

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Navigation templates]]
[[de:Vorlage:Navigation Spielelement]]
</includeonly>
```

### Notes

_No notes specified._

---

## Games

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AGames%2Fdoc.txt
- Decoded name: Template:Games/doc
- Namespace: Template
- Remainder: Games/doc
- Path parts: Template:Games / doc

```
{{Documentation subpage}}
;Description
:This navbox is used for Game Articles, to link them with the rest of Riot Games created games.
;Syntax
:Type <code>{{t|Games}}</code> at the end of the article.

<includeonly>
[[Category:Navigation templates]]

[[de:Vorlage:Navigation Spiel]]
</includeonly>
```

### Notes

_No notes specified._

---

## Gems

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AGems%2Fdoc.txt
- Decoded name: Template:Gems/doc
- Namespace: Template
- Remainder: Gems/doc
- Path parts: Template:Gems / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

;Description
:This template will produce a [[Gemstone]] icon and the link to its page. You can also adjust the size of the icon and add its cost.

;Syntax
:Type <code>{{t|gems|cost|3=size=icon size}}</code> at any part of the article.

;Example
:<code>{{tl|gems}}</code>
:* {{gems}}

:<code>{{tl|gems|100}}</code>
:* {{gems|100}}

:<code>{{tl|gems|100|3=size=30}}</code>
:* {{gems|100|size=30}}

<includeonly>
```

### Notes

_No notes specified._

---

## Gold

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AGold%2Fdoc.txt
- Decoded name: Template:Gold/doc
- Namespace: Template
- Remainder: Gold/doc
- Path parts: Template:Gold / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

;Description
:This template will produce an [[Gold]] icon and the link to its page. The amount of gold can be a decimal number, the template <code>{{t|format decimal}}</code> will automatically be used.

;Syntax
:<code>{{t|g|value|3=size=icon size}}</code>

;Example
:<code>{{tl|g}}</code>
:* {{g}}

:<code>{{tl|g|2=text=*none*}}</code>
:* {{g|text=*none*}}

:<code>{{tl|g|2560}}</code>
:* {{g|2560}}

:<code>{{tl|g|2560.33}}</code>
```

### Notes

_No notes specified._

---

## Gold efficiency

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AGold%20efficiency%2Fdoc.txt
- Decoded name: Template:Gold efficiency/doc
- Namespace: Template
- Remainder: Gold efficiency/doc
- Path parts: Template:Gold efficiency / doc

```
{{Documentation subpage}}
__NOTOC__
;Description
:This template is a modified [[Template:Texttip|text tip]] meant to be used in gold efficiency sections for items. Both the visible text and the tooltip text are standardized and cannot be changed.

;Syntax
:<code>{{t|gold efficiency}}</code>

;Example
<pre>
{{gold efficiency}}
</pre>

:...yields:
{{gold efficiency}}

;See also
* [[Template:Cost analysis]]
* [[Template:Gold value]]

```

### Notes

_No notes specified._

---

## Gold efficiency calculation

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AGold%20efficiency%20calculation%2Fdoc.txt
- Decoded name: Template:Gold efficiency calculation/doc
- Namespace: Template
- Remainder: Gold efficiency calculation/doc
- Path parts: Template:Gold efficiency calculation / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
:This template will automatically calculate the gold efficiency of the inputted Item in percentage form (rounded to 2 decimal points), calculated from the items Buy value and the inputted Gold value.
:Additionally adds a [[Template:Texttip|Texttip]], to show the difference in gold values from the item's Buy value and the inputted Gold value.

;Usage
: <code>{{t|gec|Item name|Gold value}}</code>
;Example
: <code>{{tl|gec|Pickaxe|550}}</code>
: {{gec|Pickaxe|550}}
: <code>{{tl|gec|Pickaxe|1200}}</code>
: {{gec|Pickaxe|1200}}

If a 'plus' symbol is inputted before the number for 'gold value', this template interprets the value as an additional increase of gold efficiency instead (hover to show difference).
;Example
: <code>{{tl|gec|Pickaxe|+550}}</code>
: {{gec|Pickaxe|+550}}
: <code>{{tl|gec|Pickaxe|+1200}}</code>
: {{gec|Pickaxe|+1200}}
```

### Notes

_No notes specified._

---

## Gold value

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AGold%20value%2Fdoc.txt
- Decoded name: Template:Gold value/doc
- Namespace: Template
- Remainder: Gold value/doc
- Path parts: Template:Gold value / doc

```
{{Documentation subpage}}
__NOTOC__
;Description
:This template is a modified [[Template:Texttip|text tip]] meant to be used in gold efficiency sections for items. The tooltip text is standardized and cannot be changed. The visible text is italicized, customizable (defaults to "Gold Value") and links to the [[Gold efficiency]] page.

;Syntax
:<code>{{t|gold value|text}}</code>
:<code>{{t|gv|text}}</code>

;Parameters
* '''1/text''' = Text to show.
* '''nolink=''' - Remove link (no value needed).

;Examples
<pre>{{gold value}}</pre>
{{gold value}}

<pre>{{gold value|foo}}</pre>
{{gold value|foo}}

```

### Notes

_No notes specified._

---

## Gradient bar

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AGradient%20bar%2Fdoc.txt
- Decoded name: Template:Gradient bar/doc
- Namespace: Template
- Remainder: Gradient bar/doc
- Path parts: Template:Gradient bar / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

== Usage ==
'''Gradient bar''' is used in {{t|custom champion info}}.

;Usage:
*<code><nowiki>{{</nowiki>{{PAGENAME}}|attack|100}}</code>
*<code><nowiki>{{</nowiki>{{PAGENAME}}|health|20}}</code>
*<code><nowiki>{{</nowiki>{{PAGENAME}}|spells|50}}</code>
*<code><nowiki>{{</nowiki>{{PAGENAME}}|difficulty|100}}</code>

;Technicalities
:Inline tags are styled with "display:inline-block" and used instead of block tags to support the usage of this gradient bar ''inline'' with other elements, such as an image.

==CSS==
 .gradbar.attack .light
 .gradbar.attack .dark
 .gradbar.health .light
 .gradbar.health .dark
```

### Notes

_No notes specified._

---

## H:title

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AH%3Atitle%2Fdoc.txt
- Decoded name: Template:H:title/doc
- Namespace: Template
- Remainder: H:title/doc
- Path parts: Template:H:title / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

{{Wikipedia}}
This template assigns an HTML title attribute to a given block of text. This causes a [[wikipedia:tooltip|tooltip]] to be displayed when the user hovers over the text with their cursor.

===Usage===
<pre>{{H:title
|spantitle
|label
|link=yes/no (defaults to "no")
|dotted=yes/no (defaults to "yes")
}}</pre>

===Examples===
* <code><nowiki>{{H:title|Hello|Ezreal|link=yes}}</nowiki></code> produces {{H:title|Hello|Ezreal|link=yes}}.
* <code><nowiki>{{H:title|Hello|Ezreal|dotted=no}}</nowiki></code> produces {{H:title|Hello|Ezreal|dotted=no}}.
* <code><nowiki>{{H:title|Hello|Ezreal|link=yes|dotted=no}}</nowiki></code> produces {{H:title|Hello|Ezreal|link=yes|dotted=no}}.

<includeonly>
```

### Notes

_No notes specified._

---

## Helptip

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AHelptip%2Fdoc.txt
- Decoded name: Template:Helptip/doc
- Namespace: Template
- Remainder: Helptip/doc
- Path parts: Template:Helptip / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: Adds an "info" icon that can be hovered to view help text in a tooltip.

;Syntax
: {{t|Helptip|Tooltip text}}

;Example
: {{Helptip|Example tooltip text}}

;See also
* [[Template:Double Dagger]]

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:General wiki templates]]
</includeonly>
```

### Notes

_No notes specified._

---

## Hidden

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AHidden%2Fdoc.txt
- Decoded name: Template:Hidden/doc
- Namespace: Template
- Remainder: Hidden/doc
- Path parts: Template:Hidden / doc

```
{{Documentation subpage}}__NOTOC__
;Usage
: This template creates a simple and dynamic navigation box. It displays the first parameter (the heading) with a [show] link on the right. If [show] is clicked by the user, the second parameter (the body) is displayed below the heading and the [show] link becomes [hide]. Clicking on [hide] then hides the body again and the [hide] link becomes [show] again.

;Syntax
<pre>{{Hidden
|header=
|content=
}}
</pre>

Only two parameters are required for this template:
: '''header'''
:text for header (or title); alternative to using unnamed parameter {{{1}}}
: '''content'''
:text for content (or body); alternative to using unnamed parameter {{{2}}}

; Example
<pre>{{Hidden
|Title text here
```

### Notes

_No notes specified._

---

## Hidden Box

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AHidden%20Box%2Fdoc.txt
- Decoded name: Template:Hidden Box/doc
- Namespace: Template
- Remainder: Hidden Box/doc
- Path parts: Template:Hidden Box / doc

```
{{Documentation subpage}}__NOTOC__
==Usage==
{{HiddenB
|This is a version of [[Template:Hidden]] but with top-bottom borders.
|This template creates a dynamic navigation box. It displays the first parameter (the heading) between two borders, one at the top and another at the bottom, with a [show] link on the right. If [show] is clicked by the user, the second parameter (the body) is displayed below the heading, between the borders, and the [show] link becomes [hide]. Clicking on [hide] then hides the body again and the [hide] link becomes [show] again.
}}

== Syntax ==
{| class="article-table"
! Name
! Accepted parameters
! Notes
|- 
| width
| custom input
| Sets the width of the box.
|-
| bordercolor
| custom hexcode
| Sets the hexcode color of the borders, no # required. Defaults to #BBB
```

### Notes

_No notes specified._

---

## Hidden sort key

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AHidden%20sort%20key%2Fdoc.txt
- Decoded name: Template:Hidden sort key/doc
- Namespace: Template
- Remainder: Hidden sort key/doc
- Path parts: Template:Hidden sort key / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: This template adds a hidden <code>sortkey</code>-class element, meant for sorting sortable tables.

;Syntax
: {{t|Hidden sort key|key title}}

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:General wiki templates]]
</includeonly>
```

### Notes

_No notes specified._

---

## Highest lowest stats

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AHighest%20lowest%20stats%2Fdoc.txt
- Decoded name: Template:Highest lowest stats/doc
- Namespace: Template
- Remainder: Highest lowest stats/doc
- Path parts: Template:Highest lowest stats / doc

```
{{Documentation subpage}}

== Paramaters ==
<code><nowiki>{{{1}}}</nowiki></code> --> stat name. If not specified it returns a table based on the page name if found in the data<br>
<code>sortby=top/bot</code> --> top (best) or bot (worst) champions by size. If not specified it returns both<br>
<code>rangetype=melee/ranged</code> --> melee or ranged only champions. If not specified it returns both<br>
<code>size=</code> --> the amount of rows. Defaulted to 5<br>
<code>lvl=</code> --> only for the specified lvl<br>
If <code>size=1</code> and <code>sortby</code> and <code>lvl</code> are used, which basically reduces the amount of shown champions to 1, it will remove the table and simply return that stat with the champion. This also allows the use of:<br>
<code>show=</code> --> which allows to show champions that are not bottom or top in the list<br>
<code>get=</code> --> to only show the stat / champion with the paramaters <code>stat</code> and <code>champ</code>.

== Examples ==
<pre>{{Highest lowest stats|armor}}</pre>creates: {{Highest lowest stats|armor}}

----

<pre>{{Highest lowest stats|attack speed}}</pre>creates: {{Highest lowest stats|attack speed}}

----
```

### Notes

_No notes specified._

---

## Hwei tabber

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AHwei%20tabber%2Fdoc.txt
- Decoded name: Template:Hwei tabber/doc
- Namespace: Template
- Remainder: Hwei tabber/doc
- Path parts: Template:Hwei tabber / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
This is a variant of [[Template:Image tabber]] specifically designed for Hwei's four distinct kits. 

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Section formatting templates]]
</includeonly>
```

### Notes

_No notes specified._

---
