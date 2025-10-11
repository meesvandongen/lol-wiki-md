# Template checklist — expanded

Generated: 2025-10-10T15:54:19.224Z

Batch 21 of 33 — items 401..420

## Portal/League/News

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3APortal%2FLeague%2FNews%2Fdoc.txt
- Decoded name: Template:Portal/League/News/doc
- Namespace: Template
- Remainder: Portal/League/News/doc
- Path parts: Template:Portal / League / News / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: Uses content from [[Template:Portal/League/Updates]] and [[Template:Portal/League/Events]] to display [[League of Legends]] news in a standardized format on the Wiki's main page. Update those two templates instead.

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Portal templates|{{PAGENAME}}]]
</includeonly>
```

### Notes

_No notes specified._

---

## Portal/League/Updates

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3APortal%2FLeague%2FUpdates%2Fdoc.txt
- Decoded name: Template:Portal/League/Updates/doc
- Namespace: Template
- Remainder: Portal/League/Updates/doc
- Path parts: Template:Portal / League / Updates / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page -->
;Description
: This template lists recent news and updates on the [[Main Page]].
: Updates are displayed with the help of {{t|Portal/News/Row}}.

;Syntax
<pre>{{Portal/News/Row
|image = (file name)        --- Image file name + extension on the wiki.
|y = #px                    --- Replace # with an integer. Move the image downward (positive) or upward (negative) compared to the center. By default the image is centered.
|scale = (decimal number)   --- Zoom the image in (number bigger than 1) or out (number smaller than 1).
|header = (text)            --- Title
|subheader = (text)         --- Sub-title shown to the right of the title. Usually the date: use abbreviated month + day for the date format (e.g. "Jul 17")
|link = (wiki link)         --- Input is automatically placed between [[ ]], creating a link to an article on the wiki for the entire tile.
|external= (link)           --- Input is automatically placed between [ ], creating an external link for the entire tile.
|text= (text)               --- The description of the tile.
}}</pre>

;Notes
* The ideal file for <code>image</code> should have an aspect ratio of roughly {{fd|1.5}}:1 and should not have any transparency. If no suitable image exists on the wiki, a screenshot taken specifically for this template may be uploaded. For example, if an update adds a boss monster (whose file would require transparency), an appropriate substitute would be a screenshot of a player fighting the boss in its environment. Once that update "falls off" the Main Page, however, the screenshot should be deleted.
```

### Notes

_No notes specified._

---

## Portal/LOR/Events

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3APortal%2FLOR%2FEvents%2Fdoc.txt
- Decoded name: Template:Portal/LOR/Events/doc
- Namespace: Template
- Remainder: Portal/LOR/Events/doc
- Path parts: Template:Portal / LOR / Events / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

;Description
:This template is used by the [[Template:Portal/LOR/News|News Portal Template]] and should not be used directly.
;Syntax
<pre>
{{Portal/LOR/Events}}
</pre>

;Output
<div class="mainpage-body" style="max-width:620px;">{{Portal/LOR/Events}}</div>
;See Also
: [[Template:Portal/LOR/Updates]]
: [[Template:Portal/LOR/News]]

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Navigation templates]]
[[Category:PoC Templates]]
```

### Notes

_No notes specified._

---

## Portal/LOR/News

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3APortal%2FLOR%2FNews%2Fdoc.txt
- Decoded name: Template:Portal/LOR/News/doc
- Namespace: Template
- Remainder: Portal/LOR/News/doc
- Path parts: Template:Portal / LOR / News / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

;Description
:This template is used on Main Pages, [[LoR:Legends of Runeterra|Legends of Runeterra]] and [[LoR:The Path of Champions|The Path of Champions]] and should not be used elsewhere.
;Editting
: [[Template:Portal/LOR/Updates]] for the top part of the portal
: [[Template:Portal/LOR/Events]] for the bottom part of the portal

;Syntax
<pre>
{{Portal/LOR/News}}
</pre>

;Output
<div style="max-width:50%;">{{Portal/LOR/News}}</div>

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Navigation templates]]
```

### Notes

_No notes specified._

---

## Portal/LOR/Updates

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3APortal%2FLOR%2FUpdates%2Fdoc.txt
- Decoded name: Template:Portal/LOR/Updates/doc
- Namespace: Template
- Remainder: Portal/LOR/Updates/doc
- Path parts: Template:Portal / LOR / Updates / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

;Description
:This template is used by the [[Template:Portal/LOR/News|News Portal Template]] and should not be used directly.
;Syntax
<pre>
{{Portal/LOR/Updates}}
</pre>

;Output

<div class="mainpage-body" style="max-width:620px;">{{Portal/LOR/Updates}}</div>


;See Also
: [[Template:Portal/LOR/Events]]
: [[Template:Portal/LOR/News]]

<includeonly>
```

### Notes

_No notes specified._

---

## Portal/News/Row

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3APortal%2FNews%2FRow%2Fdoc.txt
- Decoded name: Template:Portal/News/Row/doc
- Namespace: Template
- Remainder: Portal/News/Row/doc
- Path parts: Template:Portal / News / Row / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: Creates the layout for an "Updates" tile in the [[Main Page]] (automatically re-sizes when added there).
: See [[Template:Portal/League/Updates]] for details.

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Portal templates|{{PAGENAME}}]]
</includeonly>
```

### Notes

_No notes specified._

---

## Portal/Updates/Tile2

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3APortal%2FUpdates%2FTile2%2Fdoc.txt
- Decoded name: Template:Portal/Updates/Tile2/doc
- Namespace: Template
- Remainder: Portal/Updates/Tile2/doc
- Path parts: Template:Portal / Updates / Tile2 / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: Creates the layout for an "Events" tile in the [[Main Page]] (automatically re-sizes when added there).
: See [[Template:Portal/League/Events]] for details.

;Example
<pre>
{{Portal/Updates/Tile2
|image = Arena Promo 03.jpg
|y-offset = 50px
|scale = 1.4
|date = July 17 (11:00) – August 19 (23:59) [PST]
|title = Arena
|link = Arena
|description = Face off as pairs in a tournament across the Rings of Wrath, dueling out until one team stands victorious. Choose random Augments that make every match fun and intense in its own unique way!
<br/><center>• New items • New augments</center>
}}
</pre>
{{Portal/Updates/Tile2
```

### Notes

_No notes specified._

---

## Portal/WR/Updates

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3APortal%2FWR%2FUpdates%2Fdoc.txt
- Decoded name: Template:Portal/WR/Updates/doc
- Namespace: Template
- Remainder: Portal/WR/Updates/doc
- Path parts: Template:Portal / WR / Updates / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page -->
;Description
: This template lists recent news and updates on the [[WR:Wild Rift|Wild Rift landing page]].
: Updates are displayed with the help of {{t|Portal/News/Row}}.

;Syntax
<pre>{{Portal/News/Row
|image = (file name)        --- Image file name + extension on the wiki.
|y = #px                    --- Replace # with an integer. Move the image downward (positive) or upward (negative) compared to the center. By default the image is centered.
|scale = (decimal number)   --- Zoom the image in (number bigger than 1) or out (number smaller than 1).
|header = (text)            --- Title
|subheader = (text)         --- Sub-title shown to the right of the title. Usually the date: use abbreviated month + day for the date format (e.g. "Jul 17")
|link = (wiki link)         --- Input is automatically placed between [[ ]], creating a link to an article on the wiki for the entire tile.
|external= (link)           --- Input is automatically placed between [ ], creating an external link for the entire tile.
|text= (text)               --- The description of the tile.
}}</pre>

;Notes
* The ideal file for <code>image</code> should have an aspect ratio of roughly {{fd|1.5}}:1 and should not have any transparency. If no suitable image exists on the wiki, a screenshot taken specifically for this template may be uploaded. For example, if an update adds a boss monster (whose file would require transparency), an appropriate substitute would be a screenshot of a player fighting the boss in its environment. Once that update "falls off" the Main Page, however, the screenshot should be deleted.
```

### Notes

_No notes specified._

---

## Position category

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3APosition%20category%2Fdoc.txt
- Decoded name: Template:Position category/doc
- Namespace: Template
- Remainder: Position category/doc
- Path parts: Template:Position category / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

This template generates a list of positions for a given champion from the following data templates:
* [[Template:Position category/Official]]
* [[Template:Position category/Stats]]
* [[Template:Position category/Editor]]

;Usage
<pre>{{Position category|Aatrox}}</pre>
{{Position category|Aatrox}}

This template is not currently used. However, its subpages are used to automatically generated categories on [[Champion]] articles as well as [[List of champions/Position]].
<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Deprecated templates]]
[[de:Template:Position category]]
</includeonly>
```

### Notes

_No notes specified._

---

## Postponed

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3APostponed%2Fdoc.txt
- Decoded name: Template:Postponed/doc
- Namespace: Template
- Remainder: Postponed/doc
- Path parts: Template:Postponed / doc

```
{{Documentation subpage}}
;Description
:This template is intended to provide quicker reference to the stance various participants in a discussion have in the matter at hand. It is in a separate capacity also a general template used in voting.
;Syntax
:Type <code><nowiki>{{postponed}}</nowiki></code> on a discussion page.

<includeonly>[[Category:Community templates|{{PAGENAME}}]]</includeonly>
<noinclude></noinclude>
```

### Notes

_No notes specified._

---

## Pre-Arcane

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3APre-Arcane%2Fdoc.txt
- Decoded name: Template:Pre-Arcane/doc
- Namespace: Template
- Remainder: Pre-Arcane/doc
- Path parts: Template:Pre-Arcane / doc

```
#REDIRECT [[Template:Ambox documentation]][[Category:Template documentation]]
```

### Notes

_No notes specified._

---

## Prestige

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3APrestige%2Fdoc.txt
- Decoded name: Template:Prestige/doc
- Namespace: Template
- Remainder: Prestige/doc
- Path parts: Template:Prestige / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

;Description
:This template will produce a Prestige token icon and the link to [[Champion skin]] page.

:This is a specific application of [[Module:ImageLink]], so other parameters can apply to depart from the purpose of this template. Below is a table of the relevant parameters.

{| class="article-table"
! Name
! Accepted parameters
! Notes
|- 
| <code>{{{1}}}</code><br/><code>text</code>
| custom integer
| Indicates the amount of currency.
|-
| <code>token</code>
| custom string
| Indicates the filename of the icon to display. The filename must have a "Token" suffix (e.g. Hextech_Crafting_Prestige_Token.png).
```

### Notes

_No notes specified._

---

## PrevNext

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3APrevNext%2Fdoc.txt
- Decoded name: Template:PrevNext/doc
- Namespace: Template
- Remainder: PrevNext/doc
- Path parts: Template:PrevNext / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
This template can be used to add quick top-navigation for moving between articles that ascend or descend in a sequence. This template should not be used on articles that provide an alternative navigation method, such as an infobox. 

;Syntax
 {{t|PrevNext|title<nowiki>=</nowiki>Title|prev<nowiki>=</nowiki>Previous|next<nowiki>=</nowiki>Next}}
: All parameters are required.

;Example
:<code><nowiki>{{PrevNext|title=[[TFT:List of champions|List of champions]] in {{portal link|Teamfight Tactics}}|prev=[[TFT:Galaxies: Return to the Stars| Return to the Stars]]|next=[[TFT:Fates|Fates]]}}</nowiki></code>
::{{PrevNext|title=[[TFT:List of champions|List of champions]] in {{portal link|Teamfight Tactics}}|prev=[[TFT:Galaxies: Return to the Stars| Return to the Stars]]|next=[[TFT:Fates|Fates]]}}

;Default
{{PrevNext}}

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Deprecated templates]]
<!--[[Category:Navigation templates]]-->
```

### Notes

_No notes specified._

---

## Protect

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AProtect%2Fdoc.txt
- Decoded name: Template:Protect/doc
- Namespace: Template
- Remainder: Protect/doc
- Path parts: Template:Protect / doc

```
#REDIRECT [[Template:Ambox documentation]][[Category:Template documentation]]
```

### Notes

_No notes specified._

---

## Pst1

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3APst1%2Fdoc.txt
- Decoded name: Template:Pst1/doc
- Namespace: Template
- Remainder: Pst1/doc
- Path parts: Template:Pst1 / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

;Description
: This template's content is:
<pre style="font-size:150%;margin-left:1em;">
<nowiki>{{{{{{1}}}}}}</nowiki>
</pre>
* This is a parameter-selection template, which is only meant to be invoked through a [[:Category:Data templates|data template]]. It will select a parameter, thus returning its value.
* '''pst1''' selects the parameter named <code><nowiki>{{{1}}}</nowiki></code>

;Syntax
<div style="font-family:monospace;font-size:150%;margin-left:1em;background-color:var(--league-blue);padding:1px 1em;border:var(--league-grey-3) 1px solid">
<nowiki>{{<Title of data template>|pst1}}</nowiki>
</div>

;Usage
* The data template that '''pst1''' is used in must start with a <span style="color:#23CF29;">template invocation</span>, and have a <span style="color:orange;">variable parameter</span> declared at the beginning:
<div style="font-family:monospace;font-size:150%;margin-left:1em;background-color:var(--league-blue);padding:1px 1em;border:var(--league-grey-3) 1px solid">
<span style="color:#23cf29;"><nowiki>{{</nowiki></span><span style="color:orange;"><nowiki>{{{1}}}</nowiki></span>|example text|<nowiki>{{{2|}}}</nowiki>|…<br>
```

### Notes

_No notes specified._

---

## Pst2

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3APst2%2Fdoc.txt
- Decoded name: Template:Pst2/doc
- Namespace: Template
- Remainder: Pst2/doc
- Path parts: Template:Pst2 / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

;Description
: This template's content is:
<pre style="font-size:150%;margin-left:1em;">
<nowiki>{{{{{{2}}}}}}</nowiki>
</pre>
* This is a parameter-selection template, which is only meant to be invoked through a [[:Category:Data templates|data template]]. It will select a parameter, thus returning its value.
* '''pst2''' selects the parameter named <code><nowiki>{{{2}}}</nowiki></code>

;Syntax
<div style="font-family:monospace;font-size:150%;margin-left:1em;background-color:var(--league-blue);padding:1px 1em;border:var(--league-grey-3) 1px solid">
<nowiki>{{<Title of data template>|pst2|<Parameter from data template>}}</nowiki>
</div>

;Usage
* The data template that '''pst2''' is used in must start with a <span style="color:#23CF29;">template invocation</span>, and have <span style="color:orange;">variable parameter 1</span> declared at the beginning and then <span style="color:orange;">variable parameter 2</span> at the second position.
<div style="font-family:monospace;font-size:150%;margin-left:1em;background-color:var(--league-blue);padding:1px 1em;border:var(--league-grey-3) 1px solid">
<span style="color:#23cf29;"><nowiki>{{</nowiki></span><span style="color:orange;"><nowiki>{{{1}}}</nowiki></span>|example text|<span style="color:orange;"><nowiki>{{{2|}}}</nowiki></span>|…<br>
```

### Notes

_No notes specified._

---

## Pst2d

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3APst2d%2Fdoc.txt
- Decoded name: Template:Pst2d/doc
- Namespace: Template
- Remainder: Pst2d/doc
- Path parts: Template:Pst2d / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: This template's content is:
<pre style="font-size:150%;margin-left:1em;">
<nowiki>{{{{{{2}}}|}}}</nowiki>
</pre>
* It is a version of [[Template:Pst2]] that simply does not output anything if:
** the given data template parameter's name is invalid, OR
** the given data template parameter's value is empty.
* It otherwise has the same functionality as pst2. '''See documentation on pst2''' for all details.

;Syntax
<div style="font-family:monospace;font-size:150%;margin-left:1em;background-color:var(--league-blue);padding:1px 1em;border:var(--league-grey-3) 1px solid">
<nowiki>{{<Title of data template>|pst2d|<Parameter from data template>}}</nowiki>
</div>

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Parameter selection templates]]
```

### Notes

_No notes specified._

---

## Purge

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3APurge%2Fdoc.txt
- Decoded name: Template:Purge/doc
- Namespace: Template
- Remainder: Purge/doc
- Path parts: Template:Purge / doc

```
{{Documentation subpage}}
<code>{{tl|Purge}}</code> produces a link that will clear the cache and rebuild page from the wikitext.

==Purpose==
For efficiency in service, [[wikipedia:MediaWiki|MediaWiki]] [[wikipedia:Cache (computing)|cache]]s pages. Depending on the exact sequence of actions editors follow, one may view an out-of-date version of a given page. This happens most often on heavily edited pages and on pages which contain an instance of [[wikipedia:Wikipedia:Transclusion costs and benefits|transclusion]]. 

<code>{{Tl|Purge}}</code> adds to any page a link that, when clicked, not only reloads the page, but clears the server cache, forcing the page to be completely rendered "from scratch". This is not a one-time solution; generally, editing a page is itself sufficient to purge cache. Rather, it is a measure taken in anticipation of the frequent need to purge.

==Examples==
{| class="wikitable"
|-
! Code
! Yields
! Result
|-
| <tt><nowiki>{{Purge}}</nowiki></tt>
| {{Purge}}
| Creates a purge link for the page it is used on.
|-
| <tt><nowiki>{{Purge|Purge this page's server cache.}}</nowiki></tt>
```

### Notes

_No notes specified._

---

## Question

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AQuestion%2Fdoc.txt
- Decoded name: Template:Question/doc
- Namespace: Template
- Remainder: Question/doc
- Path parts: Template:Question / doc

```
{{Documentation subpage}}
;Description
:This template is intended to provide quicker reference to the stance various participants in a discussion have in the matter at hand. It is in a separate capacity also a general template used in voting.
;Syntax
:Type <code><nowiki>{{question}}</nowiki></code> on a discussion page.
<noinclude></noinclude>
```

### Notes

_No notes specified._

---

## Quicksilver info

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AQuicksilver%20info%2Fdoc.txt
- Decoded name: Template:Quicksilver info/doc
- Namespace: Template
- Remainder: Quicksilver info/doc
- Path parts: Template:Quicksilver info / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: Used as a means to display notes on the articles of {{sbc|{{nie|Quicksilver}}}} items.

;Syntax
: <code>{{t|Quicksilver info}}</code>
{{Quicksilver info}}

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Data templates]]
</includeonly>
```

### Notes

_No notes specified._

---
