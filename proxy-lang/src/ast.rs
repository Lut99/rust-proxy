//  AST.rs
//    by Lut99
// 
//  Created:
//    07 Oct 2022, 21:50:45
//  Last edited:
//    08 Oct 2022, 20:30:58
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
    /// The ports the proxy listens on (if given in this area).
    pub ports : Option<Vec<PortSetting>>,
    /// Whether the proxy uses TLS or not.
    pub tls   : Option<(TextRange, bool)>,

    /// The range of this area.
    pub range : TextRange,
}
impl Node for SettingsArea {
    #[inline]
    fn range(&self) -> TextRange { self.range }
}



/// Defines the setting for the ports to listen on.
#[derive(Clone, Debug)]
pub struct PortSetting {
    /// The list of ports to listen on.
    pub ports : Vec<PortSettingPort>,

    /// The range for this entire port setting.
    pub range : TextRange,
}
impl Node for PortSetting {
    #[inline]
    fn range(&self) -> TextRange { self.range }
}

/// Defines a single port definition in the PortSetting.
#[derive(Clone, Copy, Debug)]
pub struct PortSettingPort {
    /// The port to listen on.
    pub port  : usize,
    /// The range of this setting in the source config.
    pub range : TextRange,
}
impl Node for PortSettingPort {
    #[inline]
    fn range(&self) -> TextRange { self.range }
}



/// Defines the setting for the TLS.
#[derive(Clone, Copy, Debug)]
pub struct TlsSetting {
    /// The range for this entire setting.
    pub range : TextRange,
}
impl Node for TlsSetting {
    #[inline]
    fn range(&self) -> TextRange { self.range }
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
