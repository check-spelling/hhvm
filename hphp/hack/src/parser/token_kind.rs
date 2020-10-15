/**
 * Copyright (c) 2016, Facebook, Inc.
 * All rights reserved.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the "hack" directory of this source tree. An additional
 * directory.
 *
 **
 *
 * THIS FILE IS @generated; DO NOT EDIT IT
 * To regenerate this file, run
 *
 *   buck run //hphp/hack/src:generate_full_fidelity
 *
 **
 *
 */

use ocamlrep_derive::{FromOcamlRep, ToOcamlRep};

#[allow(non_camel_case_types)] // allow Include_once and Require_once
#[derive(Debug, Copy, Clone, PartialEq, Ord, Eq, PartialOrd, FromOcamlRep, ToOcamlRep)]
pub enum TokenKind {
    // No text tokens
    EndOfFile,
    // Given text tokens
    Abstract,
    Arraykey,
    As,
    Async,
    Attribute,
    Await,
    Backslash,
    Binary,
    Bool,
    Boolean,
    Break,
    Case,
    Catch,
    Category,
    Children,
    Class,
    Classname,
    Clone,
    Const,
    Construct,
    Continue,
    Darray,
    Default,
    Define,
    Dict,
    Do,
    Double,
    Echo,
    Else,
    Elseif,
    Empty,
    Endfor,
    Endforeach,
    Endif,
    Endswitch,
    Endwhile,
    Enum,
    Eval,
    Extends,
    Fallthrough,
    Float,
    File,
    Final,
    Finally,
    For,
    Foreach,
    From,
    Function,
    Global,
    Concurrent,
    Goto,
    If,
    Implements,
    Include,
    Includes,
    Include_once,
    Inout,
    Instanceof,
    Insteadof,
    Int,
    Integer,
    Interface,
    Is,
    Isset,
    Keyset,
    List,
    Mixed,
    Namespace,
    New,
    Newtype,
    Noreturn,
    Num,
    Object,
    Parent,
    Print,
    Private,
    Protected,
    Public,
    Real,
    Reify,
    Record,
    RecordDec,
    Require,
    Require_once,
    Required,
    Lateinit,
    Resource,
    Return,
    SelfToken,
    Shape,
    Static,
    String,
    Super,
    Suspend,
    Switch,
    This,
    Throw,
    Trait,
    Try,
    Tuple,
    Type,
    Unset,
    Use,
    Using,
    Var,
    Varray,
    Vec,
    Void,
    Where,
    While,
    Yield,
    NullLiteral,
    LeftBracket,
    RightBracket,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Dot,
    MinusGreaterThan,
    PlusPlus,
    MinusMinus,
    StarStar,
    Star,
    Plus,
    Minus,
    Tilde,
    Exclamation,
    Dollar,
    Slash,
    Percent,
    LessThanEqualGreaterThan,
    LessThanLessThan,
    GreaterThanGreaterThan,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
    EqualEqual,
    EqualEqualEqual,
    ExclamationEqual,
    ExclamationEqualEqual,
    Carat,
    Bar,
    Ampersand,
    AmpersandAmpersand,
    BarBar,
    Question,
    QuestionAs,
    QuestionColon,
    QuestionQuestion,
    QuestionQuestionEqual,
    Colon,
    Semicolon,
    Equal,
    StarStarEqual,
    StarEqual,
    SlashEqual,
    PercentEqual,
    PlusEqual,
    MinusEqual,
    DotEqual,
    LessThanLessThanEqual,
    GreaterThanGreaterThanEqual,
    AmpersandEqual,
    CaratEqual,
    BarEqual,
    Comma,
    At,
    ColonColon,
    EqualGreaterThan,
    EqualEqualGreaterThan,
    QuestionMinusGreaterThan,
    DotDotDot,
    DollarDollar,
    BarGreaterThan,
    SlashGreaterThan,
    LessThanSlash,
    LessThanQuestion,
    ColonAt,
    Backtick,
    XHP,
    // Variable text tokens
    ErrorToken,
    Name,
    Variable,
    DecimalLiteral,
    OctalLiteral,
    HexadecimalLiteral,
    BinaryLiteral,
    FloatingLiteral,
    SingleQuotedStringLiteral,
    DoubleQuotedStringLiteral,
    DoubleQuotedStringLiteralHead,
    StringLiteralBody,
    DoubleQuotedStringLiteralTail,
    HeredocStringLiteral,
    HeredocStringLiteralHead,
    HeredocStringLiteralTail,
    NowdocStringLiteral,
    BooleanLiteral,
    XHPCategoryName,
    XHPElementName,
    XHPClassName,
    XHPStringLiteral,
    XHPBody,
    XHPComment,
    Hashbang,
}

impl TokenKind {
    pub fn to_string(&self) -> &str {
        match self {
            // No text tokens
            TokenKind::EndOfFile => "end_of_file",
            // Given text tokens
            TokenKind::Abstract => "abstract",
            TokenKind::Arraykey => "arraykey",
            TokenKind::As => "as",
            TokenKind::Async => "async",
            TokenKind::Attribute => "attribute",
            TokenKind::Await => "await",
            TokenKind::Backslash => "\\",
            TokenKind::Binary => "binary",
            TokenKind::Bool => "bool",
            TokenKind::Boolean => "boolean",
            TokenKind::Break => "break",
            TokenKind::Case => "case",
            TokenKind::Catch => "catch",
            TokenKind::Category => "category",
            TokenKind::Children => "children",
            TokenKind::Class => "class",
            TokenKind::Classname => "classname",
            TokenKind::Clone => "clone",
            TokenKind::Const => "const",
            TokenKind::Construct => "__construct",
            TokenKind::Continue => "continue",
            TokenKind::Darray => "darray",
            TokenKind::Default => "default",
            TokenKind::Define => "define",
            TokenKind::Dict => "dict",
            TokenKind::Do => "do",
            TokenKind::Double => "double",
            TokenKind::Echo => "echo",
            TokenKind::Else => "else",
            TokenKind::Elseif => "elseif",
            TokenKind::Empty => "empty",
            TokenKind::Endfor => "endfor",
            TokenKind::Endforeach => "endforeach",
            TokenKind::Endif => "endif",
            TokenKind::Endswitch => "endswitch",
            TokenKind::Endwhile => "endwhile",
            TokenKind::Enum => "enum",
            TokenKind::Eval => "eval",
            TokenKind::Extends => "extends",
            TokenKind::Fallthrough => "fallthrough",
            TokenKind::Float => "float",
            TokenKind::File => "file",
            TokenKind::Final => "final",
            TokenKind::Finally => "finally",
            TokenKind::For => "for",
            TokenKind::Foreach => "foreach",
            TokenKind::From => "from",
            TokenKind::Function => "function",
            TokenKind::Global => "global",
            TokenKind::Concurrent => "concurrent",
            TokenKind::Goto => "goto",
            TokenKind::If => "if",
            TokenKind::Implements => "implements",
            TokenKind::Include => "include",
            TokenKind::Includes => "includes",
            TokenKind::Include_once => "include_once",
            TokenKind::Inout => "inout",
            TokenKind::Instanceof => "instanceof",
            TokenKind::Insteadof => "insteadof",
            TokenKind::Int => "int",
            TokenKind::Integer => "integer",
            TokenKind::Interface => "interface",
            TokenKind::Is => "is",
            TokenKind::Isset => "isset",
            TokenKind::Keyset => "keyset",
            TokenKind::List => "list",
            TokenKind::Mixed => "mixed",
            TokenKind::Namespace => "namespace",
            TokenKind::New => "new",
            TokenKind::Newtype => "newtype",
            TokenKind::Noreturn => "noreturn",
            TokenKind::Num => "num",
            TokenKind::Object => "object",
            TokenKind::Parent => "parent",
            TokenKind::Print => "print",
            TokenKind::Private => "private",
            TokenKind::Protected => "protected",
            TokenKind::Public => "public",
            TokenKind::Real => "real",
            TokenKind::Reify => "reify",
            TokenKind::Record => "recordname",
            TokenKind::RecordDec => "record",
            TokenKind::Require => "require",
            TokenKind::Require_once => "require_once",
            TokenKind::Required => "required",
            TokenKind::Lateinit => "lateinit",
            TokenKind::Resource => "resource",
            TokenKind::Return => "return",
            TokenKind::SelfToken => "self",
            TokenKind::Shape => "shape",
            TokenKind::Static => "static",
            TokenKind::String => "string",
            TokenKind::Super => "super",
            TokenKind::Suspend => "suspend",
            TokenKind::Switch => "switch",
            TokenKind::This => "this",
            TokenKind::Throw => "throw",
            TokenKind::Trait => "trait",
            TokenKind::Try => "try",
            TokenKind::Tuple => "tuple",
            TokenKind::Type => "type",
            TokenKind::Unset => "unset",
            TokenKind::Use => "use",
            TokenKind::Using => "using",
            TokenKind::Var => "var",
            TokenKind::Varray => "varray",
            TokenKind::Vec => "vec",
            TokenKind::Void => "void",
            TokenKind::Where => "where",
            TokenKind::While => "while",
            TokenKind::Yield => "yield",
            TokenKind::NullLiteral => "null",
            TokenKind::LeftBracket => "[",
            TokenKind::RightBracket => "]",
            TokenKind::LeftParen => "(",
            TokenKind::RightParen => ")",
            TokenKind::LeftBrace => "{",
            TokenKind::RightBrace => "}",
            TokenKind::Dot => ".",
            TokenKind::MinusGreaterThan => "->",
            TokenKind::PlusPlus => "++",
            TokenKind::MinusMinus => "--",
            TokenKind::StarStar => "**",
            TokenKind::Star => "*",
            TokenKind::Plus => "+",
            TokenKind::Minus => "-",
            TokenKind::Tilde => "~",
            TokenKind::Exclamation => "!",
            TokenKind::Dollar => "$",
            TokenKind::Slash => "/",
            TokenKind::Percent => "%",
            TokenKind::LessThanEqualGreaterThan => "<=>",
            TokenKind::LessThanLessThan => "<<",
            TokenKind::GreaterThanGreaterThan => ">>",
            TokenKind::LessThan => "<",
            TokenKind::GreaterThan => ">",
            TokenKind::LessThanEqual => "<=",
            TokenKind::GreaterThanEqual => ">=",
            TokenKind::EqualEqual => "==",
            TokenKind::EqualEqualEqual => "===",
            TokenKind::ExclamationEqual => "!=",
            TokenKind::ExclamationEqualEqual => "!==",
            TokenKind::Carat => "^",
            TokenKind::Bar => "|",
            TokenKind::Ampersand => "&",
            TokenKind::AmpersandAmpersand => "&&",
            TokenKind::BarBar => "||",
            TokenKind::Question => "?",
            TokenKind::QuestionAs => "?as",
            TokenKind::QuestionColon => "?:",
            TokenKind::QuestionQuestion => "??",
            TokenKind::QuestionQuestionEqual => "??=",
            TokenKind::Colon => ":",
            TokenKind::Semicolon => ";",
            TokenKind::Equal => "=",
            TokenKind::StarStarEqual => "**=",
            TokenKind::StarEqual => "*=",
            TokenKind::SlashEqual => "/=",
            TokenKind::PercentEqual => "%=",
            TokenKind::PlusEqual => "+=",
            TokenKind::MinusEqual => "-=",
            TokenKind::DotEqual => ".=",
            TokenKind::LessThanLessThanEqual => "<<=",
            TokenKind::GreaterThanGreaterThanEqual => ">>=",
            TokenKind::AmpersandEqual => "&=",
            TokenKind::CaratEqual => "^=",
            TokenKind::BarEqual => "|=",
            TokenKind::Comma => ",",
            TokenKind::At => "@",
            TokenKind::ColonColon => "::",
            TokenKind::EqualGreaterThan => "=>",
            TokenKind::EqualEqualGreaterThan => "==>",
            TokenKind::QuestionMinusGreaterThan => "?->",
            TokenKind::DotDotDot => "...",
            TokenKind::DollarDollar => "$$",
            TokenKind::BarGreaterThan => "|>",
            TokenKind::SlashGreaterThan => "/>",
            TokenKind::LessThanSlash => "</",
            TokenKind::LessThanQuestion => "<?",
            TokenKind::ColonAt => ":@",
            TokenKind::Backtick => "`",
            TokenKind::XHP => "xhp",
            // Variable text tokes
            TokenKind::ErrorToken => "error_token",
            TokenKind::Name => "name",
            TokenKind::Variable => "variable",
            TokenKind::DecimalLiteral => "decimal_literal",
            TokenKind::OctalLiteral => "octal_literal",
            TokenKind::HexadecimalLiteral => "hexadecimal_literal",
            TokenKind::BinaryLiteral => "binary_literal",
            TokenKind::FloatingLiteral => "floating_literal",
            TokenKind::SingleQuotedStringLiteral => "single_quoted_string_literal",
            TokenKind::DoubleQuotedStringLiteral => "double_quoted_string_literal",
            TokenKind::DoubleQuotedStringLiteralHead => "double_quoted_string_literal_head",
            TokenKind::StringLiteralBody => "string_literal_body",
            TokenKind::DoubleQuotedStringLiteralTail => "double_quoted_string_literal_tail",
            TokenKind::HeredocStringLiteral => "heredoc_string_literal",
            TokenKind::HeredocStringLiteralHead => "heredoc_string_literal_head",
            TokenKind::HeredocStringLiteralTail => "heredoc_string_literal_tail",
            TokenKind::NowdocStringLiteral => "nowdoc_string_literal",
            TokenKind::BooleanLiteral => "boolean_literal",
            TokenKind::XHPCategoryName => "XHP_category_name",
            TokenKind::XHPElementName => "XHP_element_name",
            TokenKind::XHPClassName => "XHP_class_name",
            TokenKind::XHPStringLiteral => "XHP_string_literal",
            TokenKind::XHPBody => "XHP_body",
            TokenKind::XHPComment => "XHP_comment",
            TokenKind::Hashbang => "hashbang",
        }
    }

    pub fn from_string(
        keyword: &[u8],
        only_reserved: bool,
    ) -> Option<Self> {
        let keyword = unsafe { std::str::from_utf8_unchecked(keyword) };
        match keyword {
            "true" if !only_reserved => Some(TokenKind::BooleanLiteral),
            "false" if !only_reserved => Some(TokenKind::BooleanLiteral),
            "abstract" => Some(TokenKind::Abstract),
            "arraykey" if !only_reserved => Some(TokenKind::Arraykey),
            "as" => Some(TokenKind::As),
            "async" => Some(TokenKind::Async),
            "attribute" if !only_reserved => Some(TokenKind::Attribute),
            "await" => Some(TokenKind::Await),
            "\\" => Some(TokenKind::Backslash),
            "binary" if !only_reserved => Some(TokenKind::Binary),
            "bool" if !only_reserved => Some(TokenKind::Bool),
            "boolean" if !only_reserved => Some(TokenKind::Boolean),
            "break" => Some(TokenKind::Break),
            "case" => Some(TokenKind::Case),
            "catch" => Some(TokenKind::Catch),
            "category" if !only_reserved => Some(TokenKind::Category),
            "children" if !only_reserved => Some(TokenKind::Children),
            "class" => Some(TokenKind::Class),
            "classname" if !only_reserved => Some(TokenKind::Classname),
            "clone" => Some(TokenKind::Clone),
            "const" => Some(TokenKind::Const),
            "__construct" => Some(TokenKind::Construct),
            "continue" => Some(TokenKind::Continue),
            "darray" if !only_reserved => Some(TokenKind::Darray),
            "default" => Some(TokenKind::Default),
            "define" if !only_reserved => Some(TokenKind::Define),
            "dict" if !only_reserved => Some(TokenKind::Dict),
            "do" => Some(TokenKind::Do),
            "double" if !only_reserved => Some(TokenKind::Double),
            "echo" => Some(TokenKind::Echo),
            "else" => Some(TokenKind::Else),
            "elseif" => Some(TokenKind::Elseif),
            "empty" => Some(TokenKind::Empty),
            "endfor" => Some(TokenKind::Endfor),
            "endforeach" => Some(TokenKind::Endforeach),
            "endif" => Some(TokenKind::Endif),
            "endswitch" => Some(TokenKind::Endswitch),
            "endwhile" => Some(TokenKind::Endwhile),
            "enum" if !only_reserved => Some(TokenKind::Enum),
            "eval" => Some(TokenKind::Eval),
            "extends" => Some(TokenKind::Extends),
            "fallthrough" if !only_reserved => Some(TokenKind::Fallthrough),
            "float" if !only_reserved => Some(TokenKind::Float),
            "file" if !only_reserved => Some(TokenKind::File),
            "final" => Some(TokenKind::Final),
            "finally" => Some(TokenKind::Finally),
            "for" => Some(TokenKind::For),
            "foreach" => Some(TokenKind::Foreach),
            "from" if !only_reserved => Some(TokenKind::From),
            "function" => Some(TokenKind::Function),
            "global" => Some(TokenKind::Global),
            "concurrent" => Some(TokenKind::Concurrent),
            "goto" => Some(TokenKind::Goto),
            "if" => Some(TokenKind::If),
            "implements" => Some(TokenKind::Implements),
            "include" => Some(TokenKind::Include),
            "includes" => Some(TokenKind::Includes),
            "include_once" => Some(TokenKind::Include_once),
            "inout" => Some(TokenKind::Inout),
            "instanceof" => Some(TokenKind::Instanceof),
            "insteadof" => Some(TokenKind::Insteadof),
            "int" if !only_reserved => Some(TokenKind::Int),
            "integer" if !only_reserved => Some(TokenKind::Integer),
            "interface" => Some(TokenKind::Interface),
            "is" if !only_reserved => Some(TokenKind::Is),
            "isset" => Some(TokenKind::Isset),
            "keyset" if !only_reserved => Some(TokenKind::Keyset),
            "list" => Some(TokenKind::List),
            "mixed" if !only_reserved => Some(TokenKind::Mixed),
            "namespace" => Some(TokenKind::Namespace),
            "new" => Some(TokenKind::New),
            "newtype" if !only_reserved => Some(TokenKind::Newtype),
            "noreturn" if !only_reserved => Some(TokenKind::Noreturn),
            "num" if !only_reserved => Some(TokenKind::Num),
            "object" if !only_reserved => Some(TokenKind::Object),
            "parent" if !only_reserved => Some(TokenKind::Parent),
            "print" => Some(TokenKind::Print),
            "private" => Some(TokenKind::Private),
            "protected" => Some(TokenKind::Protected),
            "public" => Some(TokenKind::Public),
            "real" if !only_reserved => Some(TokenKind::Real),
            "reify" if !only_reserved => Some(TokenKind::Reify),
            "recordname" => Some(TokenKind::Record),
            "record" => Some(TokenKind::RecordDec),
            "require" => Some(TokenKind::Require),
            "require_once" => Some(TokenKind::Require_once),
            "required" => Some(TokenKind::Required),
            "lateinit" => Some(TokenKind::Lateinit),
            "resource" if !only_reserved => Some(TokenKind::Resource),
            "return" => Some(TokenKind::Return),
            "self" if !only_reserved => Some(TokenKind::SelfToken),
            "shape" => Some(TokenKind::Shape),
            "static" => Some(TokenKind::Static),
            "string" if !only_reserved => Some(TokenKind::String),
            "super" if !only_reserved => Some(TokenKind::Super),
            "suspend" if !only_reserved => Some(TokenKind::Suspend),
            "switch" => Some(TokenKind::Switch),
            "this" if !only_reserved => Some(TokenKind::This),
            "throw" => Some(TokenKind::Throw),
            "trait" => Some(TokenKind::Trait),
            "try" => Some(TokenKind::Try),
            "tuple" => Some(TokenKind::Tuple),
            "type" if !only_reserved => Some(TokenKind::Type),
            "unset" => Some(TokenKind::Unset),
            "use" => Some(TokenKind::Use),
            "using" => Some(TokenKind::Using),
            "var" => Some(TokenKind::Var),
            "varray" if !only_reserved => Some(TokenKind::Varray),
            "vec" if !only_reserved => Some(TokenKind::Vec),
            "void" if !only_reserved => Some(TokenKind::Void),
            "where" if !only_reserved => Some(TokenKind::Where),
            "while" => Some(TokenKind::While),
            "yield" => Some(TokenKind::Yield),
            "null" if !only_reserved => Some(TokenKind::NullLiteral),
            "[" => Some(TokenKind::LeftBracket),
            "]" => Some(TokenKind::RightBracket),
            "(" => Some(TokenKind::LeftParen),
            ")" => Some(TokenKind::RightParen),
            "{" => Some(TokenKind::LeftBrace),
            "}" => Some(TokenKind::RightBrace),
            "." => Some(TokenKind::Dot),
            "->" => Some(TokenKind::MinusGreaterThan),
            "++" => Some(TokenKind::PlusPlus),
            "--" => Some(TokenKind::MinusMinus),
            "**" => Some(TokenKind::StarStar),
            "*" => Some(TokenKind::Star),
            "+" => Some(TokenKind::Plus),
            "-" => Some(TokenKind::Minus),
            "~" => Some(TokenKind::Tilde),
            "!" => Some(TokenKind::Exclamation),
            "$" => Some(TokenKind::Dollar),
            "/" => Some(TokenKind::Slash),
            "%" => Some(TokenKind::Percent),
            "<=>" => Some(TokenKind::LessThanEqualGreaterThan),
            "<<" => Some(TokenKind::LessThanLessThan),
            ">>" => Some(TokenKind::GreaterThanGreaterThan),
            "<" => Some(TokenKind::LessThan),
            ">" => Some(TokenKind::GreaterThan),
            "<=" => Some(TokenKind::LessThanEqual),
            ">=" => Some(TokenKind::GreaterThanEqual),
            "==" => Some(TokenKind::EqualEqual),
            "===" => Some(TokenKind::EqualEqualEqual),
            "!=" => Some(TokenKind::ExclamationEqual),
            "!==" => Some(TokenKind::ExclamationEqualEqual),
            "^" => Some(TokenKind::Carat),
            "|" => Some(TokenKind::Bar),
            "&" => Some(TokenKind::Ampersand),
            "&&" => Some(TokenKind::AmpersandAmpersand),
            "||" => Some(TokenKind::BarBar),
            "?" => Some(TokenKind::Question),
            "?as" => Some(TokenKind::QuestionAs),
            "?:" => Some(TokenKind::QuestionColon),
            "??" => Some(TokenKind::QuestionQuestion),
            "??=" => Some(TokenKind::QuestionQuestionEqual),
            ":" => Some(TokenKind::Colon),
            ";" => Some(TokenKind::Semicolon),
            "=" => Some(TokenKind::Equal),
            "**=" => Some(TokenKind::StarStarEqual),
            "*=" => Some(TokenKind::StarEqual),
            "/=" => Some(TokenKind::SlashEqual),
            "%=" => Some(TokenKind::PercentEqual),
            "+=" => Some(TokenKind::PlusEqual),
            "-=" => Some(TokenKind::MinusEqual),
            ".=" => Some(TokenKind::DotEqual),
            "<<=" => Some(TokenKind::LessThanLessThanEqual),
            ">>=" => Some(TokenKind::GreaterThanGreaterThanEqual),
            "&=" => Some(TokenKind::AmpersandEqual),
            "^=" => Some(TokenKind::CaratEqual),
            "|=" => Some(TokenKind::BarEqual),
            "," => Some(TokenKind::Comma),
            "@" => Some(TokenKind::At),
            "::" => Some(TokenKind::ColonColon),
            "=>" => Some(TokenKind::EqualGreaterThan),
            "==>" => Some(TokenKind::EqualEqualGreaterThan),
            "?->" => Some(TokenKind::QuestionMinusGreaterThan),
            "..." => Some(TokenKind::DotDotDot),
            "$$" => Some(TokenKind::DollarDollar),
            "|>" => Some(TokenKind::BarGreaterThan),
            "/>" => Some(TokenKind::SlashGreaterThan),
            "</" => Some(TokenKind::LessThanSlash),
            "<?" => Some(TokenKind::LessThanQuestion),
            ":@" => Some(TokenKind::ColonAt),
            "`" => Some(TokenKind::Backtick),
            "xhp" if !only_reserved => Some(TokenKind::XHP),
            _ => None,
        }
    }

    pub fn ocaml_tag(self) -> u8 {
        match self {
            TokenKind::EndOfFile => 0,
            TokenKind::Abstract => 1,
            TokenKind::Arraykey => 2,
            TokenKind::As => 3,
            TokenKind::Async => 4,
            TokenKind::Attribute => 5,
            TokenKind::Await => 6,
            TokenKind::Backslash => 7,
            TokenKind::Binary => 8,
            TokenKind::Bool => 9,
            TokenKind::Boolean => 10,
            TokenKind::Break => 11,
            TokenKind::Case => 12,
            TokenKind::Catch => 13,
            TokenKind::Category => 14,
            TokenKind::Children => 15,
            TokenKind::Class => 16,
            TokenKind::Classname => 17,
            TokenKind::Clone => 18,
            TokenKind::Const => 19,
            TokenKind::Construct => 20,
            TokenKind::Continue => 21,
            TokenKind::Darray => 22,
            TokenKind::Default => 23,
            TokenKind::Define => 24,
            TokenKind::Dict => 25,
            TokenKind::Do => 26,
            TokenKind::Double => 27,
            TokenKind::Echo => 28,
            TokenKind::Else => 29,
            TokenKind::Elseif => 30,
            TokenKind::Empty => 31,
            TokenKind::Endfor => 32,
            TokenKind::Endforeach => 33,
            TokenKind::Endif => 34,
            TokenKind::Endswitch => 35,
            TokenKind::Endwhile => 36,
            TokenKind::Enum => 37,
            TokenKind::Eval => 38,
            TokenKind::Extends => 39,
            TokenKind::Fallthrough => 40,
            TokenKind::Float => 41,
            TokenKind::File => 42,
            TokenKind::Final => 43,
            TokenKind::Finally => 44,
            TokenKind::For => 45,
            TokenKind::Foreach => 46,
            TokenKind::From => 47,
            TokenKind::Function => 48,
            TokenKind::Global => 49,
            TokenKind::Concurrent => 50,
            TokenKind::Goto => 51,
            TokenKind::If => 52,
            TokenKind::Implements => 53,
            TokenKind::Include => 54,
            TokenKind::Includes => 55,
            TokenKind::Include_once => 56,
            TokenKind::Inout => 57,
            TokenKind::Instanceof => 58,
            TokenKind::Insteadof => 59,
            TokenKind::Int => 60,
            TokenKind::Integer => 61,
            TokenKind::Interface => 62,
            TokenKind::Is => 63,
            TokenKind::Isset => 64,
            TokenKind::Keyset => 65,
            TokenKind::List => 66,
            TokenKind::Mixed => 67,
            TokenKind::Namespace => 68,
            TokenKind::New => 69,
            TokenKind::Newtype => 70,
            TokenKind::Noreturn => 71,
            TokenKind::Num => 72,
            TokenKind::Object => 73,
            TokenKind::Parent => 74,
            TokenKind::Print => 75,
            TokenKind::Private => 76,
            TokenKind::Protected => 77,
            TokenKind::Public => 78,
            TokenKind::Real => 79,
            TokenKind::Reify => 80,
            TokenKind::Record => 81,
            TokenKind::RecordDec => 82,
            TokenKind::Require => 83,
            TokenKind::Require_once => 84,
            TokenKind::Required => 85,
            TokenKind::Lateinit => 86,
            TokenKind::Resource => 87,
            TokenKind::Return => 88,
            TokenKind::SelfToken => 89,
            TokenKind::Shape => 90,
            TokenKind::Static => 91,
            TokenKind::String => 92,
            TokenKind::Super => 93,
            TokenKind::Suspend => 94,
            TokenKind::Switch => 95,
            TokenKind::This => 96,
            TokenKind::Throw => 97,
            TokenKind::Trait => 98,
            TokenKind::Try => 99,
            TokenKind::Tuple => 100,
            TokenKind::Type => 101,
            TokenKind::Unset => 102,
            TokenKind::Use => 103,
            TokenKind::Using => 104,
            TokenKind::Var => 105,
            TokenKind::Varray => 106,
            TokenKind::Vec => 107,
            TokenKind::Void => 108,
            TokenKind::Where => 109,
            TokenKind::While => 110,
            TokenKind::Yield => 111,
            TokenKind::NullLiteral => 112,
            TokenKind::LeftBracket => 113,
            TokenKind::RightBracket => 114,
            TokenKind::LeftParen => 115,
            TokenKind::RightParen => 116,
            TokenKind::LeftBrace => 117,
            TokenKind::RightBrace => 118,
            TokenKind::Dot => 119,
            TokenKind::MinusGreaterThan => 120,
            TokenKind::PlusPlus => 121,
            TokenKind::MinusMinus => 122,
            TokenKind::StarStar => 123,
            TokenKind::Star => 124,
            TokenKind::Plus => 125,
            TokenKind::Minus => 126,
            TokenKind::Tilde => 127,
            TokenKind::Exclamation => 128,
            TokenKind::Dollar => 129,
            TokenKind::Slash => 130,
            TokenKind::Percent => 131,
            TokenKind::LessThanEqualGreaterThan => 132,
            TokenKind::LessThanLessThan => 133,
            TokenKind::GreaterThanGreaterThan => 134,
            TokenKind::LessThan => 135,
            TokenKind::GreaterThan => 136,
            TokenKind::LessThanEqual => 137,
            TokenKind::GreaterThanEqual => 138,
            TokenKind::EqualEqual => 139,
            TokenKind::EqualEqualEqual => 140,
            TokenKind::ExclamationEqual => 141,
            TokenKind::ExclamationEqualEqual => 142,
            TokenKind::Carat => 143,
            TokenKind::Bar => 144,
            TokenKind::Ampersand => 145,
            TokenKind::AmpersandAmpersand => 146,
            TokenKind::BarBar => 147,
            TokenKind::Question => 148,
            TokenKind::QuestionAs => 149,
            TokenKind::QuestionColon => 150,
            TokenKind::QuestionQuestion => 151,
            TokenKind::QuestionQuestionEqual => 152,
            TokenKind::Colon => 153,
            TokenKind::Semicolon => 154,
            TokenKind::Equal => 155,
            TokenKind::StarStarEqual => 156,
            TokenKind::StarEqual => 157,
            TokenKind::SlashEqual => 158,
            TokenKind::PercentEqual => 159,
            TokenKind::PlusEqual => 160,
            TokenKind::MinusEqual => 161,
            TokenKind::DotEqual => 162,
            TokenKind::LessThanLessThanEqual => 163,
            TokenKind::GreaterThanGreaterThanEqual => 164,
            TokenKind::AmpersandEqual => 165,
            TokenKind::CaratEqual => 166,
            TokenKind::BarEqual => 167,
            TokenKind::Comma => 168,
            TokenKind::At => 169,
            TokenKind::ColonColon => 170,
            TokenKind::EqualGreaterThan => 171,
            TokenKind::EqualEqualGreaterThan => 172,
            TokenKind::QuestionMinusGreaterThan => 173,
            TokenKind::DotDotDot => 174,
            TokenKind::DollarDollar => 175,
            TokenKind::BarGreaterThan => 176,
            TokenKind::SlashGreaterThan => 177,
            TokenKind::LessThanSlash => 178,
            TokenKind::LessThanQuestion => 179,
            TokenKind::ColonAt => 180,
            TokenKind::Backtick => 181,
            TokenKind::XHP => 182,
            TokenKind::ErrorToken => 183,
            TokenKind::Name => 184,
            TokenKind::Variable => 185,
            TokenKind::DecimalLiteral => 186,
            TokenKind::OctalLiteral => 187,
            TokenKind::HexadecimalLiteral => 188,
            TokenKind::BinaryLiteral => 189,
            TokenKind::FloatingLiteral => 190,
            TokenKind::SingleQuotedStringLiteral => 191,
            TokenKind::DoubleQuotedStringLiteral => 192,
            TokenKind::DoubleQuotedStringLiteralHead => 193,
            TokenKind::StringLiteralBody => 194,
            TokenKind::DoubleQuotedStringLiteralTail => 195,
            TokenKind::HeredocStringLiteral => 196,
            TokenKind::HeredocStringLiteralHead => 197,
            TokenKind::HeredocStringLiteralTail => 198,
            TokenKind::NowdocStringLiteral => 199,
            TokenKind::BooleanLiteral => 200,
            TokenKind::XHPCategoryName => 201,
            TokenKind::XHPElementName => 202,
            TokenKind::XHPClassName => 203,
            TokenKind::XHPStringLiteral => 204,
            TokenKind::XHPBody => 205,
            TokenKind::XHPComment => 206,
            TokenKind::Hashbang => 207,
        }
    }
}
