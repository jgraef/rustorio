use nom::{
    branch::alt,
    bytes::complete::{
        escaped_transform,
        is_not,
        tag,
        take,
        take_until,
        take_while1,
    },
    character::complete::{
        char,
        digit1,
        line_ending,
        multispace0,
        multispace1,
        newline,
        none_of,
        not_line_ending,
        one_of,
    },
    combinator::{
        all_consuming,
        cut,
        map,
        not,
        opt,
        peek,
        value,
    },
    error::{
        context,
        ErrorKind,
        FromExternalError,
        ParseError,
        VerboseError,
    },
    multi::{
        many0,
        many0_count,
        separated_list0,
        separated_list1,
    },
    sequence::{
        delimited,
        pair,
        preceded,
        separated_pair,
        terminated,
        tuple,
    },
    IResult,
    Parser,
};

use super::{
    Category,
    Locale,
    Localization,
    LocalizationElement,
    PluralizationPattern,
    PluralizationRule,
};

type Res<'a, U> = IResult<&'a str, U, VerboseError<&'a str>>;

fn comment(input: &str) -> Res<()> {
    value((), tuple((alt((char('#'), char(';'))), is_not("\r\n"))))(input)
}

fn consume_junk(input: &str) -> Res<()> {
    value((), many0(alt((value((), line_ending), value((), comment)))))(input)
}

fn consume_junk_before<'a, U>(
    parser: impl FnMut(&'a str) -> Res<'a, U>,
) -> impl FnMut(&'a str) -> Res<'a, U> {
    preceded(consume_junk, parser)
}

pub(super) fn parse_locale<'a>(input: &'a str) -> Res<'a, Locale> {
    all_consuming(map(
        tuple((parse_category_body, many0(parse_category))),
        |(global, categories)| {
            Locale {
                global,
                categories: categories.into_iter().collect(),
            }
        },
    ))(input)
}

fn parse_category<'a>(input: &'a str) -> Res<'a, (String, Category)> {
    map(
        tuple((
            map(
                delimited(char('['), is_not("\r\n"), char(']')),
                |name: &str| name.to_owned(),
            ),
            parse_category_body,
        )),
        |(name, body)| (name.to_owned(), body),
    )(input)
}

fn parse_category_body<'a>(input: &'a str) -> Res<'a, Category> {
    map(
        many0(consume_junk_before(separated_pair(
            map(is_not("="), |s: &str| s.to_owned()),
            char('='),
            parse_localization,
        ))),
        |values| {
            Category {
                values: values.into_iter().collect(),
            }
        },
    )(input)
}

fn parse_localization<'a>(input: &'a str) -> Res<'a, Localization> {
    map(
        many0(alt((
            delimited(tag("__"), parse_parameter, tag("__")),
            map(take_until("__"), |text: &str| {
                LocalizationElement::Text(text.to_owned())
            }),
        ))),
        |elements| Localization { elements },
    )(input)
}

fn parse_parameter<'a>(input: &'a str) -> Res<'a, LocalizationElement> {
    alt((
        map(nom::character::complete::u32, |param: u32| {
            LocalizationElement::Parameter(param)
        }),
        map(
            tuple((
                tag("__plural_for_parameter_"),
                nom::character::complete::u32,
                tag("_{"),
                separated_list1(
                    char('|'),
                    separated_pair(parse_pluralization_pattern, char('='), is_not("|}")),
                ),
                tag("}"),
            )),
            |(_, parameter, _, rules, _)| {
                LocalizationElement::Pluralization {
                    parameter,
                    rules: rules
                        .into_iter()
                        .map(|(pattern, text)| {
                            PluralizationRule {
                                pattern,
                                text: text.to_owned(),
                            }
                        })
                        .collect(),
                }
            },
        ),
        map(
            preceded(tag("ENTITY__"), take_until("__")),
            |param: &str| LocalizationElement::Entity(param.to_owned()),
        ),
        map(preceded(tag("ITEM__"), take_until("__")), |param: &str| {
            LocalizationElement::Item(param.to_owned())
        }),
        map(preceded(tag("TILE__"), take_until("__")), |param: &str| {
            LocalizationElement::Tile(param.to_owned())
        }),
        map(preceded(tag("FLUID__"), take_until("__")), |param: &str| {
            LocalizationElement::Fluid(param.to_owned())
        }),
        map(take_until("__"), |_| LocalizationElement::Todo),
    ))(input)
}

fn parse_pluralization_pattern<'a>(input: &'a str) -> Res<'a, PluralizationPattern> {
    alt((
        map(tag("rest"), |_| PluralizationPattern::Rest),
        map(
            separated_list1(char(','), nom::character::complete::u32),
            |values| PluralizationPattern::Numbers(values),
        ),
        map(
            separated_list1(
                char(','),
                preceded(tag("ends in "), nom::character::complete::u32),
            ),
            |values| PluralizationPattern::EndsWith(values),
        ),
    ))(input)
}
