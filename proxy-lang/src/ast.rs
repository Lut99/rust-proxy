//  AST.rs
//    by Lut99
// 
//  Created:
//    07 Oct 2022, 21:50:45
//  Last edited:
//    13 Oct 2022, 10:41:37
//  Auto updated?
//    Yes
// 
//  Description:
//!   Defines the abstract syntax tree for the compiled proxy language.
// 

use crate::spec::{Node, TextRange};


/***** TOPLEVEL *****/
/// Defines a complete configuration (the root node).
#[derive(Clone, Debug)]
pub struct Config {
    /// The proxy's settings/configuration options
    pub config   : Vec<SettingsArea>,
    /// The proxy's rules to proxy
    pub patterns : Vec<RulesArea>,

    /// The range in the source text of the entire config.
    pub range : TextRange,
}
impl Node for Config {
    #[inline]
    fn range(&self) -> TextRange { self.range }
}





/***** SETTINGS *****/
/// Defines a single Settings area.
#[derive(Clone, Debug)]
pub struct SettingsArea {
    /// The settings in the settings area.
    pub settings : Vec<Setting>,

    /// The range of this area.
    pub range : TextRange,
}
impl Node for SettingsArea {
    #[inline]
    fn range(&self) -> TextRange { self.range }
}

/// Defines a single setting within the settings area.
#[derive(Clone, Debug)]
pub struct Setting {
    /// The key of the settings
    pub key   : SettingKey,
    /// The value of the settings
    pub value : SettingValue,

    /// The text range of the setting
    pub range : TextRange,
}
impl Node for Setting {
    #[inline]
    fn range(&self) -> TextRange { self.range }
}

/// Defines a key in the setting area.
#[derive(Clone, Debug)]
pub struct SettingKey {
    /// The name of the key.
    pub value : String,
    /// The location of the key in the source text.
    pub range : TextRange,
}
impl Node for SettingKey {
    #[inline]
    fn range(&self) -> TextRange{ self.range }
}

/// Defines a value in the setting area.
#[derive(Clone, Debug)]
pub enum SettingValue {
    /// It's a simple string value.
    String(String, TextRange),
    /// It's a simple non-negative numerical value.
    UInt(u64, TextRange),
    /// It's a simple numerical value,
    SInt(i64, TextRange),
    /// It's a boolean value.
    Bool(bool, TextRange),

    /// It's a list of setting values.
    List(Vec<Self>, TextRange),
    /// It's a struct of setting values.
    Dict(Vec<Setting>, TextRange),
}
impl Node for SettingValue {
    fn range(&self) -> TextRange {
        use SettingValue::*;
        match self {
            String(_, range) => *range,
            UInt(_, range)   => *range,
            SInt(_, range)   => *range,
            Bool(_, range)   => *range,

            List(_, range) => *range,
            Dict(_, range) => *range,
        }
    }
}




/***** RULES *****/
/// Defines an area that may contain rules.
#[derive(Clone, Debug)]
pub struct RulesArea {
    /// The rules within this area, if any.
    pub rules : Vec<Rule>,

    /// The range of this area.
    pub range : TextRange,
}
impl Node for RulesArea {
    #[inline]
    fn range(&self) -> TextRange { self.range }
}

/// Defines a single pattern in the list of them.
#[derive(Clone, Debug)]
pub struct Rule {
    /// The lefthand-side of the pattern (i.e., the matcher). They are syntactically (almost) identical but semantically different.
    pub lhs : Pattern,
    /// The righthand-side of the pattern (i.e., the rewriter). They are syntactically (almost) identical but semantically different.
    pub rhs : Action,

    /// The range of the entire rule.
    pub range : TextRange,
}
impl Node for Rule {
    #[inline]
    fn range(&self) -> TextRange { self.range }
}



/// Defines what to match in a pattern.
#[derive(Clone, Debug)]
pub struct Pattern {
    /// The protocol-part of the pattern (i.e., that before the `://`).
    pub protocol : Protocol,
    /// The base endpoint of the pattern (i.e., that after the `://` and before any `:` for ports or `/` for child paths).
    pub base     : Endpoint,
    /// The path of the pattern (i.e., that after the endpoint _or_ the port).
    pub path     : Path,
    /// The port specified by the pattern.
    pub port     : Port,

    /// The range of the entire pattern.
    pub range : TextRange,
}
impl Node for Pattern {
    #[inline]
    fn range(&self) -> TextRange { self.range }
}

/// Defines what protocol the user specified in a Pattern.
#[derive(Clone, Debug)]
pub enum Protocol {
    /// It's a named one.
    Specific(String, TextRange),
    /// It's any / all.
    Wildcard,
}
impl Node for Protocol {
    #[inline]
    fn range(&self) -> TextRange { if let Self::Specific(_, range) = self { *range } else { TextRange::None } }
}

/// Defines what endpoint the user specified in a Pattern.
#[derive(Clone, Debug)]
pub enum Endpoint {
    /// It's a named one.
    Specific(String, TextRange),
    /// It's any / all.
    Wildcard,
}
impl Node for Endpoint {
    #[inline]
    fn range(&self) -> TextRange { if let Self::Specific(_, range) = self { *range } else { TextRange::None } }
}

/// Defines what path(s) the user specified in a Pattern.
#[derive(Clone, Debug)]
pub enum Path {
    /// It's a named one.
    Specific(Vec<String>, TextRange),
    /// It's any / all.
    Wildcard,
}
impl Node for Path {
    #[inline]
    fn range(&self) -> TextRange { if let Self::Specific(_, range) = self { *range } else { TextRange::None } }
}

/// Defines what port the user specified in a Pattern.
#[derive(Clone, Copy, Debug)]
pub enum Port {
    /// It's a named one.
    Specific(u16, TextRange),
    /// It's any / all.
    Wildcard,
}
impl Node for Port {
    #[inline]
    fn range(&self) -> TextRange { if let Self::Specific(_, range) = self { *range } else { TextRange::None } }
}



/// Defines possible actions the user may take.
#[derive(Clone, Debug)]
pub enum Action {
    /// Accept the given rule as-is (i.e., don't proxy but simple re-send as the original).
    Accept(TextRange),
    /// Rewrite the incoming rule to a (potentially) different one, as specified by the given pattern.
    Rewrite(Pattern),
    /// Drop the incoming pattern with the given HTTP status code and message.
    Drop(u16, Option<String>, TextRange),
}
impl Node for Action {
    #[inline]
    fn range(&self) -> TextRange {
        match self {
            Action::Accept(range)     => *range,
            Action::Rewrite(pattern)  => pattern.range(),
            Action::Drop(_, _, range) => *range,
        }
    }
}
