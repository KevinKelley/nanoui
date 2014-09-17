/*
// primitive
Image
Glyph, Font
Color, Gradient
Shape (path, svg-style string of path commands, plus stroke/fill colors)
Box (? html box model w/ border/gap/etc ?)
Canvas (owner-draw)

// composite
Token, Text (atomically-painted unicode, but could decompose into seq of Glyph)
EditableText (before, and after, cursor)
StyledText
Transform
Clip (scissor)
Row,Col,Table
Layer, Glass, Overlay, HUD

// presentation
Style, Class(css), Theme
Stroke, Fill, Pen, Paint


// widgets
Button, Checkbox, Radio
Label, Paragraph, Editbox,
List, Dropdown, Combo
Tree, Treetable
Toolbar, Menu
Slider, Scrollbar, Spinner, Colorwheel
Graph, Chart, Table
Tabpane(deck)
Pane, Panel, Canvas, ScrollPane, Window
*/
