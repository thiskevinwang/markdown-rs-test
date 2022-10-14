use markdown::mdast::*;

use serde::{
  // import SerializeStruct to fix:
  // > no method named `serialize_field` found for associated type `<S as serde::Serializer>::SerializeStruct` in the current scope
  // https://github.com/serde-rs/serde/issues/1687
  ser::SerializeStruct,
  Serialize,
  Serializer,
};

// This creates a structs/enum that replicate the internal `markdown` structs/enums,
// and makes them serializable via `serde`

#[derive(Debug)]
struct MyPoint(markdown::unist::Point);
impl From<markdown::unist::Point> for MyPoint {
  fn from(p: markdown::unist::Point) -> MyPoint {
    MyPoint(p)
  }
}
impl Serialize for MyPoint {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut state = serializer.serialize_struct("Point", 3)?;
    state.serialize_field("line", &self.0.line)?;
    state.serialize_field("column", &self.0.column)?;
    state.serialize_field("offset", &self.0.offset)?;
    state.end()
  }
}

#[derive(Debug)]
struct MyPosition(markdown::unist::Position);
impl From<markdown::unist::Position> for MyPosition {
  fn from(p: markdown::unist::Position) -> MyPosition {
    MyPosition(p)
  }
}
impl Serialize for MyPosition {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut state = serializer.serialize_struct("Position", 2)?;
    let pos = MyPosition::from(self.0.clone());
    let start: MyPoint = MyPoint::from(pos.0.start);
    state.serialize_field("start", &start)?;

    let end: MyPoint = MyPoint::from(pos.0.end);
    state.serialize_field("end", &end)?;
    state.end()
  }
}

#[derive(Debug, Serialize)]
enum MyReferenceKind {
  Collapsed,
  Full,
  Shortcut,
}
impl From<markdown::mdast::ReferenceKind> for MyReferenceKind {
  fn from(r: markdown::mdast::ReferenceKind) -> MyReferenceKind {
    match r {
      markdown::mdast::ReferenceKind::Collapsed => MyReferenceKind::Collapsed,
      markdown::mdast::ReferenceKind::Full => MyReferenceKind::Full,
      markdown::mdast::ReferenceKind::Shortcut => MyReferenceKind::Shortcut,
    }
  }
}

#[derive(Debug)]
enum MyAttributeContent {
  // AttributeContent(markdown::mdast::AttributeContent),
  Expression(String, Vec<markdown::mdast::Stop>),
  Property(markdown::mdast::MdxJsxAttribute),
}
impl From<markdown::mdast::AttributeContent> for MyAttributeContent {
  fn from(a: markdown::mdast::AttributeContent) -> MyAttributeContent {
    match a {
      markdown::mdast::AttributeContent::Expression(e, v) => MyAttributeContent::Expression(e, v),
      markdown::mdast::AttributeContent::Property(p) => MyAttributeContent::Property(p),
    }
  }
}
impl Serialize for MyAttributeContent {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    match self {
      MyAttributeContent::Expression(e, v) => {
        let mut state = serializer.serialize_struct("Expression", 2)?;
        state.serialize_field("type", "Expression")?;
        state.serialize_field("value", e)?;
        state.serialize_field("stops", v)?;
        state.end()
      }
      MyAttributeContent::Property(ref p) => {
        let mut state = serializer.serialize_struct("Property", 2)?;
        state.serialize_field("type", "Property")?;
        if let Some(v) = p.value.clone() {
          let prop: MyAttributeValue = v.into();
          state.serialize_field("value", &prop)?;
        }
        state.end()
      } // _ => {
        //     let mut state = serializer.serialize_struct("MyAttributeContent", 1)?;
        //     state.serialize_field("type", "AttributeContent")?;
        //     state.end()
        // }
    }
  }
}

#[derive(Debug, Serialize)]
enum MyAlignKind {
  Left,
  Center,
  Right,
  None,
}
impl From<markdown::mdast::AlignKind> for MyAlignKind {
  fn from(r: markdown::mdast::AlignKind) -> MyAlignKind {
    match r {
      markdown::mdast::AlignKind::Left => MyAlignKind::Left,
      markdown::mdast::AlignKind::Center => MyAlignKind::Center,
      markdown::mdast::AlignKind::Right => MyAlignKind::Right,
      markdown::mdast::AlignKind::None => MyAlignKind::None,
    }
  }
}

#[derive(Debug)]
enum MyAttributeValue {
  Expression(String, Vec<markdown::mdast::Stop>),
  Literal(String),
}
impl From<markdown::mdast::AttributeValue> for MyAttributeValue {
  fn from(av: markdown::mdast::AttributeValue) -> MyAttributeValue {
    match av {
      markdown::mdast::AttributeValue::Expression(e, v) => MyAttributeValue::Expression(e, v),
      markdown::mdast::AttributeValue::Literal(s) => MyAttributeValue::Literal(s),
    }
  }
}
impl Serialize for MyAttributeValue {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    match self {
      MyAttributeValue::Expression(ref e, ref v) => {
        let mut state = serializer.serialize_struct("Expression", 2)?;
        state.serialize_field("expression", e)?;
        state.serialize_field("stops", v)?;
        state.end()
      }
      MyAttributeValue::Literal(ref l) => l.serialize(serializer),
      // _ => self.serialize(serializer),
    }
  }
}

#[derive(Debug)]
struct MyMdxJsxAttribute(markdown::mdast::MdxJsxAttribute);
impl From<markdown::mdast::MdxJsxAttribute> for MyMdxJsxAttribute {
  fn from(a: markdown::mdast::MdxJsxAttribute) -> MyMdxJsxAttribute {
    MyMdxJsxAttribute(a)
  }
}
impl Serialize for MyMdxJsxAttribute {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut state = serializer.serialize_struct("MdxJsxAttribute", 3)?;
    state.serialize_field("type", "MdxJsxAttribute")?;
    state.serialize_field("name", &self.0.name)?;
    if let Some(v) = &self.0.value {
      let my_value: MyAttributeValue = MyAttributeValue::from(v.clone());
      state.serialize_field("value", &my_value)?;
    }
    state.end()
  }
}

/// This enum mirrors`markdown::mdast::Node`
#[derive(Debug)]
pub enum MyNode {
  // Node(Node),
  Root(Root),
  BlockQuote(BlockQuote),
  FootnoteDefinition(FootnoteDefinition),
  MdxJsxFlowElement(MdxJsxFlowElement),
  List(List),
  MdxjsEsm(MdxjsEsm),
  Toml(Toml),
  Yaml(Yaml),
  Break(Break),
  InlineCode(InlineCode),
  InlineMath(InlineMath),
  Delete(Delete),
  Emphasis(Emphasis),
  MdxTextExpression(MdxTextExpression),
  FootnoteReference(FootnoteReference),
  Html(Html),
  Image(Image),
  ImageReference(ImageReference),
  MdxJsxTextElement(MdxJsxTextElement),
  Link(Link),
  LinkReference(LinkReference),
  Strong(Strong),
  Text(Text),
  Code(Code),
  Math(Math),
  MdxFlowExpression(MdxFlowExpression),
  Heading(Heading),
  Table(Table),
  ThematicBreak(ThematicBreak),
  TableRow(TableRow),
  TableCell(TableCell),
  ListItem(ListItem),
  Definition(Definition),
  Paragraph(Paragraph),
}

/// Create a `MyNode` Enum from `markdown::mdast::Node` Enum.
impl From<Node> for MyNode {
  fn from(n: Node) -> MyNode {
    match n {
      Node::Root(r) => MyNode::Root(r),
      Node::BlockQuote(r) => MyNode::BlockQuote(r),
      Node::FootnoteDefinition(r) => MyNode::FootnoteDefinition(r),
      Node::MdxJsxFlowElement(r) => MyNode::MdxJsxFlowElement(r),
      Node::List(r) => MyNode::List(r),
      Node::MdxjsEsm(r) => MyNode::MdxjsEsm(r),
      Node::Toml(r) => MyNode::Toml(r),
      Node::Yaml(r) => MyNode::Yaml(r),
      Node::Break(r) => MyNode::Break(r),
      Node::InlineCode(r) => MyNode::InlineCode(r),
      Node::InlineMath(r) => MyNode::InlineMath(r),
      Node::Delete(r) => MyNode::Delete(r),
      Node::Emphasis(r) => MyNode::Emphasis(r),
      Node::MdxTextExpression(r) => MyNode::MdxTextExpression(r),
      Node::FootnoteReference(r) => MyNode::FootnoteReference(r),
      Node::Html(r) => MyNode::Html(r),
      Node::Image(r) => MyNode::Image(r),
      Node::ImageReference(r) => MyNode::ImageReference(r),
      Node::MdxJsxTextElement(r) => MyNode::MdxJsxTextElement(r),
      Node::Link(r) => MyNode::Link(r),
      Node::LinkReference(r) => MyNode::LinkReference(r),
      Node::Strong(r) => MyNode::Strong(r),
      Node::Text(r) => MyNode::Text(r),
      Node::Code(r) => MyNode::Code(r),
      Node::Math(r) => MyNode::Math(r),
      Node::MdxFlowExpression(r) => MyNode::MdxFlowExpression(r),
      Node::Heading(r) => MyNode::Heading(r),
      Node::Table(r) => MyNode::Table(r),
      Node::ThematicBreak(r) => MyNode::ThematicBreak(r),
      Node::TableRow(r) => MyNode::TableRow(r),
      Node::TableCell(r) => MyNode::TableCell(r),
      Node::ListItem(r) => MyNode::ListItem(r),
      Node::Definition(r) => MyNode::Definition(r),
      Node::Paragraph(r) => MyNode::Paragraph(r),
    }
  }
}

impl Serialize for MyNode {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    match self {
      // MyNode::Node(ref node) => {
      //     // let mut state = serializer.serialize_struct_variant("MyNode", 0, "Node", 3)?;
      //     let mut state = serializer.serialize_struct("Node", 3)?;
      //     if let Some(children) = node.children() {
      //         let new_children = children
      //             .iter()
      //             .map(|c| MyNode::from(c.clone()))
      //             .collect::<Vec<MyNode>>();
      //         state.serialize_field("children", &new_children)?;
      //     }
      //     if let Some(position) = node.position() {
      //         let new_position: MyPosition = position.clone().into();
      //         state.serialize_field("position", &new_position)?;
      //     }
      //     state.end()
      // }
      MyNode::Root(ref node) => {
        let mut state = serializer.serialize_struct("Root", 2)?;
        state.serialize_field("type", "Root")?;
        let new_children = node
          .children
          .iter()
          .map(|c| MyNode::from(c.clone()))
          .collect::<Vec<MyNode>>();
        state.serialize_field("children", &new_children)?;

        if let Some(p) = node.position.clone() {
          let pos: MyPosition = MyPosition::from(p);
          state.serialize_field("position", &pos)?;
        }

        state.end()
      }
      MyNode::BlockQuote(ref node) => {
        let mut state = serializer.serialize_struct("BlockQuote", 2)?;
        state.serialize_field("type", "BlockQuote")?;
        if let Some(pos) = node.position.clone() {
          let new_pos: MyPosition = MyPosition::from(pos);
          state.serialize_field("position", &new_pos)?;
        }
        state.serialize_field(
          "children",
          &node
            .children
            .iter()
            .map(|c| MyNode::from(c.clone()))
            .collect::<Vec<MyNode>>(),
        )?;
        state.end()
      }
      MyNode::FootnoteDefinition(ref node) => {
        let mut state = serializer.serialize_struct("FootnoteDefinition", 2)?;
        state.serialize_field("type", "FootnoteDefinition")?;
        if let Some(pos) = node.position.clone() {
          let new_pos: MyPosition = MyPosition::from(pos);
          state.serialize_field("position", &new_pos)?;
        }
        state.serialize_field(
          "children",
          &node
            .children
            .iter()
            .map(|c| MyNode::from(c.clone()))
            .collect::<Vec<MyNode>>(),
        )?;
        state.serialize_field("identifier", &node.identifier)?;
        state.serialize_field("label", &node.label)?;
        state.end()
      }
      MyNode::MdxJsxFlowElement(ref node) => {
        let mut state = serializer.serialize_struct("MdxJsxFlowElement", 2)?;
        state.serialize_field("type", "MdxJsxFlowElement")?;
        if let Some(name) = &node.name {
          state.serialize_field("name", &name)?;
        }
        if let Some(pos) = node.position.clone() {
          let new_pos: MyPosition = MyPosition::from(pos);
          state.serialize_field("position", &new_pos)?;
        }
        state.serialize_field(
          "children",
          &node
            .children
            .iter()
            .map(|c| MyNode::from(c.clone()))
            .collect::<Vec<MyNode>>(),
        )?;
        state.serialize_field(
          "attributes",
          &node
            .attributes
            .iter()
            .map(|a| MyAttributeContent::from(a.clone()))
            .collect::<Vec<MyAttributeContent>>(),
        )?;
        state.end()
      }
      MyNode::List(ref node) => {
        let mut state = serializer.serialize_struct("List", 2)?;
        state.serialize_field("type", "List")?;
        if let Some(pos) = node.position.clone() {
          let new_pos: MyPosition = MyPosition::from(pos);
          state.serialize_field("position", &new_pos)?;
        }
        state.serialize_field("ordered", &node.ordered)?;
        state.serialize_field("start", &node.start)?;
        state.serialize_field("spread", &node.spread)?;
        state.serialize_field(
          "children",
          &node
            .children
            .iter()
            .map(|c| MyNode::from(c.clone()))
            .collect::<Vec<MyNode>>(),
        )?;
        state.end()
      }
      MyNode::MdxjsEsm(ref node) => {
        let mut state = serializer.serialize_struct("MdxjsEsm", 2)?;
        state.serialize_field("type", "MdxjsEsm")?;
        if let Some(pos) = node.position.clone() {
          let new_pos: MyPosition = MyPosition::from(pos);
          state.serialize_field("position", &new_pos)?;
        }
        state.serialize_field("value", &node.value)?;
        state.end()
      }
      MyNode::Toml(ref node) => {
        let mut state = serializer.serialize_struct("Toml", 2)?;
        state.serialize_field("type", "Toml")?;
        if let Some(pos) = node.position.clone() {
          let new_pos: MyPosition = MyPosition::from(pos);
          state.serialize_field("position", &new_pos)?;
        }
        state.serialize_field("value", &node.value)?;
        state.end()
      }
      MyNode::Yaml(ref node) => {
        let mut state = serializer.serialize_struct("Yaml", 2)?;
        state.serialize_field("type", "Yaml")?;
        if let Some(pos) = node.position.clone() {
          let new_pos: MyPosition = MyPosition::from(pos);
          state.serialize_field("position", &new_pos)?;
        }
        state.serialize_field("value", &node.value)?;
        state.end()
      }
      MyNode::Break(ref node) => {
        let mut state = serializer.serialize_struct("Break", 2)?;
        state.serialize_field("type", "Break")?;
        if let Some(pos) = node.position.clone() {
          let new_pos: MyPosition = MyPosition::from(pos);
          state.serialize_field("position", &new_pos)?;
        }
        state.end()
      }
      MyNode::InlineCode(ref node) => {
        let mut state = serializer.serialize_struct("InlineCode", 2)?;
        state.serialize_field("type", "InlineCode")?;
        if let Some(pos) = node.position.clone() {
          let new_pos: MyPosition = MyPosition::from(pos);
          state.serialize_field("position", &new_pos)?;
        }
        state.serialize_field("value", &node.value)?;
        state.end()
      }
      MyNode::InlineMath(ref node) => {
        let mut state = serializer.serialize_struct("InlineMath", 2)?;
        state.serialize_field("type", "InlineMath")?;
        if let Some(pos) = node.position.clone() {
          let new_pos: MyPosition = MyPosition::from(pos);
          state.serialize_field("position", &new_pos)?;
        }
        state.serialize_field("value", &node.value)?;
        state.end()
      }
      MyNode::Delete(ref node) => {
        let mut state = serializer.serialize_struct("Delete", 2)?;
        state.serialize_field("type", "Delete")?;
        if let Some(pos) = node.position.clone() {
          let new_pos: MyPosition = MyPosition::from(pos);
          state.serialize_field("position", &new_pos)?;
        }
        state.serialize_field(
          "children",
          &node
            .children
            .iter()
            .map(|c| MyNode::from(c.clone()))
            .collect::<Vec<MyNode>>(),
        )?;
        state.end()
      }
      MyNode::Emphasis(ref node) => {
        let mut state = serializer.serialize_struct("Emphasis", 2)?;
        state.serialize_field("type", "Emphasis")?;
        if let Some(pos) = node.position.clone() {
          let new_pos: MyPosition = MyPosition::from(pos);
          state.serialize_field("position", &new_pos)?;
        }
        state.serialize_field(
          "children",
          &node
            .children
            .iter()
            .map(|c| MyNode::from(c.clone()))
            .collect::<Vec<MyNode>>(),
        )?;
        state.end()
      }
      MyNode::MdxTextExpression(ref node) => {
        let mut state = serializer.serialize_struct("MdxTextExpression", 2)?;
        state.serialize_field("type", "MdxTextExpression")?;
        if let Some(pos) = node.position.clone() {
          let new_pos: MyPosition = MyPosition::from(pos);
          state.serialize_field("position", &new_pos)?;
        }
        state.serialize_field("value", &node.value)?;
        state.end()
      }
      MyNode::FootnoteReference(ref node) => {
        let mut state = serializer.serialize_struct("FootnoteReference", 2)?;
        state.serialize_field("type", "FootnoteReference")?;
        if let Some(pos) = node.position.clone() {
          let new_pos: MyPosition = MyPosition::from(pos);
          state.serialize_field("position", &new_pos)?;
        }
        state.serialize_field("identifier", &node.identifier)?;
        state.end()
      }
      MyNode::Html(ref node) => {
        let mut state = serializer.serialize_struct("Html", 2)?;
        state.serialize_field("type", "Html")?;
        if let Some(pos) = node.position.clone() {
          let new_pos: MyPosition = MyPosition::from(pos);
          state.serialize_field("position", &new_pos)?;
        }
        state.serialize_field("value", &node.value)?;
        state.end()
      }
      MyNode::Image(ref node) => {
        let mut state = serializer.serialize_struct("Image", 2)?;
        state.serialize_field("type", "Image")?;
        if let Some(pos) = node.position.clone() {
          let new_pos: MyPosition = MyPosition::from(pos);
          state.serialize_field("position", &new_pos)?;
        }
        state.serialize_field("title", &node.title)?;
        state.serialize_field("alt", &node.alt)?;
        state.serialize_field("url", &node.url)?;
        state.end()
      }
      MyNode::ImageReference(ref node) => {
        let mut state = serializer.serialize_struct("ImageReference", 2)?;
        state.serialize_field("type", "ImageReference")?;
        if let Some(pos) = node.position.clone() {
          let new_pos: MyPosition = MyPosition::from(pos);
          state.serialize_field("position", &new_pos)?;
        }
        state.serialize_field("label", &node.label)?;
        state.serialize_field("alt", &node.alt)?;
        state.serialize_field("identifier", &node.identifier)?;
        state.serialize_field(
          "reference_kind",
          &MyReferenceKind::from(node.reference_kind),
        )?;
        state.end()
      }
      MyNode::MdxJsxTextElement(ref node) => {
        let mut state = serializer.serialize_struct("MdxJsxTextElement", 2)?;
        state.serialize_field("type", "MdxJsxTextElement")?;
        if let Some(pos) = node.position.clone() {
          let new_pos: MyPosition = MyPosition::from(pos);
          state.serialize_field("position", &new_pos)?;
        }
        state.serialize_field("name", &node.name)?;
        state.serialize_field(
          "children",
          &node
            .children
            .iter()
            .map(|c| MyNode::from(c.clone()))
            .collect::<Vec<MyNode>>(),
        )?;
        state.serialize_field(
          "attributes",
          &node
            .attributes
            .iter()
            .map(|a| MyAttributeContent::from(a.clone()))
            .collect::<Vec<MyAttributeContent>>(),
        )?;
        state.end()
      }
      MyNode::Link(ref node) => {
        let mut state = serializer.serialize_struct("Link", 2)?;
        state.serialize_field("type", "Link")?;
        if let Some(pos) = node.position.clone() {
          let new_pos: MyPosition = MyPosition::from(pos);
          state.serialize_field("position", &new_pos)?;
        }
        state.serialize_field(
          "children",
          &node
            .children
            .iter()
            .map(|c| MyNode::from(c.clone()))
            .collect::<Vec<MyNode>>(),
        )?;
        if let Some(title) = node.title.clone() {
          state.serialize_field("title", &title)?;
        }
        state.serialize_field("url", &node.url)?;
        state.end()
      }
      MyNode::LinkReference(ref node) => {
        let mut state = serializer.serialize_struct("LinkReference", 2)?;
        state.serialize_field("type", "LinkReference")?;
        if let Some(pos) = node.position.clone() {
          let new_pos: MyPosition = MyPosition::from(pos);
          state.serialize_field("position", &new_pos)?;
        }
        state.serialize_field("label", &node.label)?;
        state.serialize_field("identifier", &node.identifier)?;
        state.serialize_field(
          "children",
          &node
            .children
            .iter()
            .map(|c| MyNode::from(c.clone()))
            .collect::<Vec<MyNode>>(),
        )?;
        state.serialize_field(
          "reference_kind",
          &MyReferenceKind::from(node.reference_kind),
        )?;
        state.end()
      }
      MyNode::Strong(ref node) => {
        let mut state = serializer.serialize_struct("Strong", 2)?;
        state.serialize_field("type", "Strong")?;
        if let Some(pos) = node.position.clone() {
          let new_pos: MyPosition = MyPosition::from(pos);
          state.serialize_field("position", &new_pos)?;
        }
        state.serialize_field(
          "children",
          &node
            .children
            .iter()
            .map(|c| MyNode::from(c.clone()))
            .collect::<Vec<MyNode>>(),
        )?;
        state.end()
      }

      MyNode::Text(ref node) => {
        let mut state = serializer.serialize_struct("Text", 2)?;
        state.serialize_field("type", "Text")?;
        state.serialize_field("value", &node.value)?;
        if let Some(p) = node.position.clone() {
          let pos: MyPosition = MyPosition::from(p);
          state.serialize_field("position", &pos)?;
        }
        state.end()
      }
      MyNode::Code(ref node) => {
        let mut state = serializer.serialize_struct("Code", 2)?;
        state.serialize_field("type", "Code")?;
        state.serialize_field("value", &node.value)?;
        if let Some(p) = node.position.clone() {
          let pos: MyPosition = MyPosition::from(p);
          state.serialize_field("position", &pos)?;
        }
        state.serialize_field("lang", &node.lang.clone().unwrap_or("".to_owned()))?;
        state.serialize_field("meta", &node.meta.clone().unwrap_or("".to_owned()))?;
        state.end()
      }
      // MyNode::Math(ref node) => {}
      MyNode::MdxFlowExpression(ref node) => {
        let mut state = serializer.serialize_struct("MdxFlowExpression", 2)?;
        state.serialize_field("type", "MdxFlowExpression")?;
        state.serialize_field("value", &node.value)?;
        if let Some(p) = node.position.clone() {
          let pos: MyPosition = MyPosition::from(p);
          state.serialize_field("position", &pos)?;
        }
        state.serialize_field("stops", &node.stops)?;
        state.end()
      }
      MyNode::Heading(ref node) => {
        // let mut state = serializer.serialize_struct_variant("MyNode", 0, "Heading", 2)?;
        let mut state = serializer.serialize_struct("MyNode", 3)?;
        // state.serialize_field("type", std::any::type_name_of_val(&node))?;
        state.serialize_field("type", "Heading")?;
        state.serialize_field("depth", &node.depth)?;
        let new_children = node
          .children
          .iter()
          .map(|c| MyNode::from(c.clone()))
          .collect::<Vec<MyNode>>();
        state.serialize_field("children", &new_children)?;
        if let Some(p) = node.position.clone() {
          let pos: MyPosition = MyPosition::from(p);
          state.serialize_field("position", &pos)?;
        }

        state.end()
      }
      MyNode::Table(ref node) => {
        let mut state = serializer.serialize_struct("Table", 2)?;
        state.serialize_field("type", "Table")?;
        if let Some(p) = node.position.clone() {
          let pos: MyPosition = MyPosition::from(p);
          state.serialize_field("position", &pos)?;
        }
        state.serialize_field(
          "children",
          &node
            .children
            .iter()
            .map(|c| MyNode::from(c.clone()))
            .collect::<Vec<MyNode>>(),
        )?;
        state.serialize_field(
          "align",
          &node
            .align
            .iter()
            .map(|ak| MyAlignKind::from(ak.clone()))
            .collect::<Vec<MyAlignKind>>(),
        )?;
        state.end()
      }
      MyNode::ThematicBreak(ref node) => {
        let mut state = serializer.serialize_struct("ThematicBreak", 2)?;
        state.serialize_field("type", "ThematicBreak")?;
        if let Some(p) = node.position.clone() {
          let pos: MyPosition = MyPosition::from(p);
          state.serialize_field("position", &pos)?;
        }
        state.end()
      }
      MyNode::TableRow(ref node) => {
        let mut state = serializer.serialize_struct("TableRow", 2)?;
        state.serialize_field("type", "TableRow")?;
        if let Some(p) = node.position.clone() {
          let pos: MyPosition = MyPosition::from(p);
          state.serialize_field("position", &pos)?;
        }
        state.serialize_field(
          "children",
          &node
            .children
            .iter()
            .map(|c| MyNode::from(c.clone()))
            .collect::<Vec<MyNode>>(),
        )?;
        state.end()
      }
      MyNode::TableCell(ref node) => {
        let mut state = serializer.serialize_struct("TableCell", 2)?;
        state.serialize_field("type", "TableCell")?;
        if let Some(p) = node.position.clone() {
          let pos: MyPosition = MyPosition::from(p);
          state.serialize_field("position", &pos)?;
        }
        state.serialize_field(
          "children",
          &node
            .children
            .iter()
            .map(|c| MyNode::from(c.clone()))
            .collect::<Vec<MyNode>>(),
        )?;
        state.end()
      }
      MyNode::ListItem(ref node) => {
        let mut state = serializer.serialize_struct("ListItem", 2)?;
        state.serialize_field("type", "ListItem")?;
        if let Some(p) = node.position.clone() {
          let pos: MyPosition = MyPosition::from(p);
          state.serialize_field("position", &pos)?;
        }
        state.serialize_field("checked", &node.checked.clone().unwrap_or(false))?;
        state.serialize_field(
          "children",
          &node
            .children
            .iter()
            .map(|c| MyNode::from(c.clone()))
            .collect::<Vec<MyNode>>(),
        )?;
        state.serialize_field("spread", &node.spread)?;
        state.end()
      }
      MyNode::Definition(ref node) => {
        let mut state = serializer.serialize_struct("Definition", 2)?;
        state.serialize_field("type", "Definition")?;
        if let Some(p) = node.position.clone() {
          let pos: MyPosition = MyPosition::from(p);
          state.serialize_field("position", &pos)?;
        }
        state.serialize_field("identifier", &node.identifier)?;
        state.serialize_field("url", &node.url)?;
        state.serialize_field("title", &node.title.clone().unwrap_or("".to_owned()))?;
        state.serialize_field("label", &node.label.clone().unwrap_or("".to_owned()))?;
        state.end()
      }
      MyNode::Paragraph(ref node) => {
        let mut state = serializer.serialize_struct("Paragraph", 2)?;
        state.serialize_field("type", "Paragraph")?;
        if let Some(p) = node.position.clone() {
          let pos: MyPosition = MyPosition::from(p);
          state.serialize_field("position", &pos)?;
        }
        let new_children = node
          .children
          .iter()
          .map(|c| MyNode::from(c.clone()))
          .collect::<Vec<MyNode>>();
        state.serialize_field("children", &new_children)?;
        state.end()
      }
      // default
      _ => {
        let mut state = serializer.serialize_struct("Default", 1)?;
        state.serialize_field("type", "_TODO")?;
        state.end()
      }
    }
  }
}
