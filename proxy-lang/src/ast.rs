//  AST.rs
//    by Lut99
// 
//  Created:
//    07 Oct 2022, 21:50:45
//  Last edited:
//    07 Oct 2022, 22:35:02
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
    pub config   : Settings,
    /// The proxy's rules to proxy
    pub patterns : Vec<Rule>,

    /// The range in the source text of the entire config.
    pub range : TextRange,
}

impl Node for Config {
    #[inline]
    fn range(&self) -> TextRange { self.range }
}





/***** SETTINGS *****/
/// Defines the settings that may be set by the config.
#[derive(Clone, Debug)]
pub struct Settings {
    /// The ports the proxy listens on.
    pub ports : Vec<(u16, TextRange)>,
    /// Whether the proxy uses TLS or not.
    pub tls   : (TextRange, bool),

    /// The range in the source text of the settings part.
    pub range : TextRange,
}

impl Node for Settings {
    #[inline]
    fn range(&self) -> TextRange { self.range }
}



/// Defines the setting for the ports to listen on.
#[derive(Clone, Debug)]
pub struct PortSetting {
    /// The list of ports to listen on.
    pub ports : Vec<PortSettingPort>,
}

/// Defines a single port definition in the PortSetting.
#[derive(Clone, Copy, Debug)]
pub struct PortSettingPort {
    /// The port to listen on.
    pub port  : usize,
    /// The range of this setting in the source config.
    pub range : TextRange,
}



/// Defines the setting for the TLS.
#[derive(Clone, Copy, Debug)]
pub struct TlsSetting {
    
}





/***** RULES *****/
/// Defines a single pattern in the list of them.
#[derive(Clone, Debug)]
pub struct Rule {
    /// The lefthand-side of the pattern (i.e., the matcher). They are syntactically (almost) identical but semantically different.
    pub lhs : Pattern,
    /// The righthand-side of the pattern (i.e., the rewriter). They are syntactically (almost) identical but semantically different.
    pub rhs : Action,
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
}



/// Defines what protocol the user specified in a Pattern.
#[derive(Clone, Debug)]
pub enum Protocol {
    /// It's a named one.
    Specific(String),
    /// It's any / all.
    Wildcard,
}

/// Defines what endpoint the user specified in a Pattern.
#[derive(Clone, Debug)]
pub enum Endpoint {
    /// It's a named one.
    Specific(String),
    /// It's any / all.
    Wildcard,
}

/// Defines what path(s) the user specified in a Pattern.
#[derive(Clone, Debug)]
pub enum Path {
    /// It's a named one.
    Specific(Vec<String>),
    /// It's any / all.
    Wildcard,
}

/// Defines what port the user specified in a Pattern.
#[derive(Clone, Copy, Debug)]
pub enum Port {
    /// It's a named one.
    Specific(u16),
    /// It's any / all.
    Wildcard,
}



/// Defines possible actions the user may take.
#[derive(Clone, Debug)]
pub enum Action {
    /// Accept the given rule as-is (i.e., don't proxy but simple re-send as the original).
    Accept,
    /// Rewrite the incoming rule to a (potentially) different one, as specified by the given pattern.
    Rewrite(Pattern),
    /// Drop the incoming pattern with the given HTTP status code and message.
    Drop(u16, Option<String>),
}
