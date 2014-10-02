///! Basic CSS block layout.

use robinson::style::{StyledNode, Inline, Block, DisplayNone};
use robinson::css::{Keyword, Length, Px};
use std::default::Default;
use std::iter::AdditiveIterator; // for `sum`

// CSS box model. All sizes are in px.

#[deriving(Default, Show)]
pub struct Dimensions {
    // Top left corner of the content area, relative to the document origin:
    pub x: f32,
    pub y: f32,

    // Content area size:
    pub width: f32,
    pub height: f32,

    // Surrounding edges:
    pub padding: EdgeSizes,
    pub border: EdgeSizes,
    pub margin: EdgeSizes,
}
impl Dimensions {
    pub fn sized(w:uint, h:uint) -> Dimensions {
        Dimensions {
        x: 0.0,
        y: 0.0,
        width: w as f32,
        height: h as f32,
        padding: Default::default(),
        border: Default::default(),
        margin: Default::default(),
        }
    }
    /// Total height of a box including its margins, border, and padding.
    fn margin_box_height(&self) -> f32 {
        self.height + self.padding.top + self.padding.bottom
                    + self.border.top + self.border.bottom
                    + self.margin.top + self.margin.bottom
    }
    /// Total width of a box including its margins, border, and padding.
    fn margin_box_width(&self) -> f32 {
        self.width + self.padding.left + self.padding.right
                   + self.border.left + self.border.right
                   + self.margin.left + self.margin.right
    }
}

#[deriving(Default, Show)]
pub struct EdgeSizes {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}


/// A node in the layout tree.
#[deriving(Show)]
pub struct LayoutBox<'a> {
    pub dimensions: Dimensions,
    pub box_type: BoxType<'a>,
    pub children: Vec<LayoutBox<'a>>,
}

#[deriving(Show)]
pub enum BoxType<'a> {
    BlockNode(&'a StyledNode<'a>),
    InlineNode(&'a StyledNode<'a>),
    AnonymousBlock(&'a StyledNode<'a>),
}

impl<'a> LayoutBox<'a> {
    fn new(box_type: BoxType) -> LayoutBox {
        LayoutBox {
            box_type: box_type,
            dimensions: Default::default(),
            children: Vec::new(),
        }
    }

    pub fn get_style_node(&self) -> &'a StyledNode<'a> {
        match self.box_type {
            BlockNode(node) => node,
            InlineNode(node) => node,
            AnonymousBlock(node) => node //fail!("Anonymous block box has no style node")
        }
    }
}

/// Transform a style tree into a layout tree.
pub fn layout_tree<'a>(node: &'a StyledNode<'a>, containing_block: Dimensions) -> LayoutBox<'a> {
    let mut root_box = build_layout_tree(node);
    root_box.layout(containing_block);
    return root_box;
}

/// Build the tree of LayoutBoxes, but don't perform any layout calculations yet.
fn build_layout_tree<'a>(style_node: &'a StyledNode<'a>) -> LayoutBox<'a> {
    // Create the root box.
    let mut root = LayoutBox::new(match style_node.display() {
        Block => BlockNode(style_node),
        Inline => InlineNode(style_node),
        DisplayNone => fail!("Root node has display: none.")
    });

    // Create the descendant boxes.
    for child in style_node.children.iter() {
        match child.display() {
            Block => root.children.push(build_layout_tree(child)),
            Inline => root.get_inline_container().children.push(build_layout_tree(child)),
            DisplayNone => {} // Don't lay out nodes with `display: none;`
        }
    }
    return root;
}

impl<'a> LayoutBox<'a> {
    /// Lay out a box and its descendants.
    fn layout(&mut self, containing_block: Dimensions) {
        match self.box_type {
            BlockNode(_) => self.layout_block(containing_block),
            InlineNode(_) => self.layout_inline(containing_block),
            AnonymousBlock(_) => {
                // anon blocks can be anon-block or anon-inline...
                // so far we're just creating anon block when a block contains
                // both inline and block subs.  therefore the anon block
                // will do inline formatting on its children... i think
                self.layout_inline(containing_block);
            } // TODO
        }
    }

//////////////////////////////////////////////////////////////////////
// inline

    /// layout an inline node and its descendants.
    /// I'm taking this to be just a mirror (horizontal replacing vertical)
    /// of the logic for block-layout
    fn layout_inline(&mut self, containing_block: Dimensions) {

        self.measure_content();

        self.layout_inline_children();

        self.calculate_inline_position(containing_block);
    }

    /// ideal desired size for this block's content (if any)
    fn measure_content(&mut self) {
        //let style = self.get_style_node();
        let d = &mut self.dimensions;

        //let em = Length(16.0, Px);
        //let em5 = Length(16.0 * 5.0, Px); // avg word length in pix

        d.width = 80.0;  //em5.to_px();
        d.height = 16.0; //em.to_px();
    }

    /// Lay out the inline's children within its content area.
    ///
    /// Sets `self.dimensions.width` to the total content width.
    fn layout_inline_children(&mut self) {
        let d = &mut self.dimensions;
        for child in self.children.mut_iter() {
            child.layout(*d);
            // Increment width so each child is laid out next to the previous one (no wrap)
            // TODO wrap
            d.width = d.width + child.dimensions.margin_box_width();
        }
    }

    fn calculate_inline_position(&mut self, containing_block: Dimensions) {
//        let style = self.get_style_node();
        let d = &mut self.dimensions;

//        // margin, border, and padding have initial value 0.
//        let zero = Length(0.0, Px);
//
//        // If margin-left or margin-right is `auto`, the used value is zero.
//        d.margin.left = style.lookup("margin-left", "margin", &zero).to_px();
//        d.margin.right = style.lookup("margin-right", "margin", &zero).to_px();
//
//        d.border.left = style.lookup("border-left-width", "border-width", &zero).to_px();
//        d.border.right = style.lookup("border-right-width", "border-width", &zero).to_px();
//
//        d.padding.left = style.lookup("padding-left", "padding", &zero).to_px();
//        d.padding.right = style.lookup("padding-right", "padding", &zero).to_px();

        // Position the box to right of all previous boxes in the container.
        d.x = containing_block.x + containing_block.width +
              d.margin.left + d.border.left + d.padding.left;
        d.y = containing_block.y +
              d.margin.top + d.border.top + d.padding.top;
    }


////////////////////////////////////////////////////////////////////
// block

    fn is_leaf(&self) -> bool { self.children.len() == 0 }

    /// Lay out a block-level element and its descendants.
    fn layout_block(&mut self, mut containing_block: Dimensions) {

        let given_height = containing_block.height;
        containing_block.height = 0.0;

        // Child width can depend on parent width, so we need to calculate this box's width before
        // laying out its children.
        self.calculate_block_width(containing_block);

        // Determine where the box is located within its container.
        self.calculate_block_position(containing_block);

        // either we have children, in which case lay them out, accumulating height...
        // ...or we don't, and we're a leaf-node; in which case use intrinsic height.
        if self.is_leaf() {
            self.dimensions.height = 16.0;
        }
        else {
            // Recursively lay out the children of this box.
            self.layout_block_children();
        }

        // after all that, it's possible that height is specifically set in css,
        // so in that case, override it.
        self.check_override_block_height();
    }

    /// Calculate the width of a block-level non-replaced element in normal flow.
    ///
    /// http://www.w3.org/TR/CSS2/visudet.html#blockwidth
    ///
    /// Sets the horizontal margin/padding/border dimensions, and the `width`.
    fn calculate_block_width(&mut self, containing_block: Dimensions) {
        let style = self.get_style_node();

        // `width` has initial value `auto`.
        let auto = Keyword("auto".to_string());
        let mut width = style.value("width").unwrap_or(auto.clone());

        // margin, border, and padding have initial value 0.
        let zero = Length(0.0, Px);

        let mut margin_left = style.lookup("margin-left", "margin", &zero);
        let mut margin_right = style.lookup("margin-right", "margin", &zero);

        let border_left = style.lookup("border-left-width", "border-width", &zero);
        let border_right = style.lookup("border-right-width", "border-width", &zero);

        let padding_left = style.lookup("padding-left", "padding", &zero);
        let padding_right = style.lookup("padding-right", "padding", &zero);

        let total = [&margin_left, &margin_right, &border_left, &border_right,
                     &padding_left, &padding_right, &width].iter().map(|v| v.to_px()).sum();

        // If width is not auto and the total is wider than the container, treat auto margins as 0.
        if width != auto && total > containing_block.width {
            if margin_left == auto {
                margin_left = Length(0.0, Px);
            }
            if margin_right == auto {
                margin_right = Length(0.0, Px);
            }
        }

        // Adjust used values so that the above sum equals `containing_block.width`.
        // Each arm of the `match` should increase the total width by exactly `underflow`,
        // and afterward all values should be absolute lengths in px.
        let underflow = containing_block.width - total;

        match (width == auto, margin_left == auto, margin_right == auto) {
            // If the values are overconstrained, calculate margin_right.
            (false, false, false) => {
                margin_right = Length(margin_right.to_px() + underflow, Px);
            }

            // If exactly one size is auto, its used value follows from the equality.
            (false, false, true) => { margin_right = Length(underflow, Px); }
            (false, true, false) => { margin_left  = Length(underflow, Px); }

            // If width is set to auto, any other auto values become 0.
            (true, _, _) => {
                if margin_left == auto { margin_left = Length(0.0, Px); }
                if margin_right == auto { margin_right = Length(0.0, Px); }

                if underflow >= 0.0 {
                    // Expand width to fill the underflow.
                    width = Length(underflow, Px);
                } else {
                    // Width can't be negative. Adjust the right margin instead.
                    width = Length(0.0, Px);
                    margin_right = Length(margin_right.to_px() + underflow, Px);
                }
            }

            // If margin-left and margin-right are both auto, their used values are equal.
            (false, true, true) => {
                margin_left = Length(underflow / 2.0, Px);
                margin_right = Length(underflow / 2.0, Px);
            }
        }

        let d = &mut self.dimensions;
        d.width = width.to_px();

        d.padding.left = padding_left.to_px();
        d.padding.right = padding_right.to_px();

        d.border.left = border_left.to_px();
        d.border.right = border_right.to_px();

        d.margin.left = margin_left.to_px();
        d.margin.right = margin_right.to_px();
    }

    /// Finish calculating the block's edge sizes, and position it within its containing block.
    ///
    /// http://www.w3.org/TR/CSS2/visudet.html#normal-block
    ///
    /// Sets the vertical margin/padding/border dimensions, and the `x`, `y` values.
    fn calculate_block_position(&mut self, containing_block: Dimensions) {
        let style = self.get_style_node();
        let d = &mut self.dimensions;

        // margin, border, and padding have initial value 0.
        let zero = Length(0.0, Px);

        // If margin-top or margin-bottom is `auto`, the used value is zero.
        d.margin.top = style.lookup("margin-top", "margin", &zero).to_px();
        d.margin.bottom = style.lookup("margin-bottom", "margin", &zero).to_px();

        d.border.top = style.lookup("border-top-width", "border-width", &zero).to_px();
        d.border.bottom = style.lookup("border-bottom-width", "border-width", &zero).to_px();

        d.padding.top = style.lookup("padding-top", "padding", &zero).to_px();
        d.padding.bottom = style.lookup("padding-bottom", "padding", &zero).to_px();

        // Position the box below all the previous boxes in the container.
        d.x = containing_block.x +
              d.margin.left + d.border.left + d.padding.left;
        d.y = containing_block.y + containing_block.height +
              d.margin.top + d.border.top + d.padding.top;
    }

    /// Lay out the block's children within its content area.
    ///
    /// Sets `self.dimensions.height` to the total content height.
    fn layout_block_children(&mut self) {
        let d = &mut self.dimensions;
        //assert!(d.height == 0.0); // testing assumptions; self-block hasn't been placed yet
        for child in self.children.mut_iter() {
            child.layout(*d);
            // Increment the height so each child is laid out below the previous one.
            d.height = d.height + child.dimensions.margin_box_height();
        }
    }

    /// Height of a block-level non-replaced element in normal flow with overflow visible.
    fn check_override_block_height(&mut self) {
        // If the height is set to an explicit length, use that exact length.
        // Otherwise, just keep the value set by `layout_block_children`.
        match self.get_style_node().value("height") {
            Some(Length(h, Px)) => { self.dimensions.height = h; }
            _ => {}
        }
    }

    /// Where a new inline child should go.
    fn get_inline_container(&mut self) -> &mut LayoutBox<'a> {
        match self.box_type {
            InlineNode(_) | AnonymousBlock(_) => self,
            BlockNode(node) => {
                // If we've just generated an anonymous block box, keep using it.
                // Otherwise, create a new one.
                match self.children.last() {
                    Some(&LayoutBox { box_type: AnonymousBlock(_),..}) => {}

                    // WORRY: capture parent's styled node reference,
                    // probably not right, but where to get a node-ref when there's no node?
                    _ => self.children.push(LayoutBox::new(AnonymousBlock(node)))
                }
                self.children.mut_last().unwrap()
            }
        }
    }

}
