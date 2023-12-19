use std::marker::PhantomData;

use rustorio_prototype::{
    entity::LabPrototype,
    technology::TechnologyPrototype,
    HasPrototypes,
    Id,
    InheritsBase,
    PrototypeBase,
    Prototypes,
};
use yew::{
    classes,
    function_component,
    html,
    use_context,
    use_state,
    AttrValue,
    Callback,
    Html,
    Properties,
};

use crate::{
    data::{
        AppData,
        GameData,
    },
    ui::icon::FactorioIcon,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DefaultSelected<T> {
    None,
    First,
    Some(T),
}

impl<T> From<Option<T>> for DefaultSelected<T> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(x) => Self::Some(x),
            None => Self::None,
        }
    }
}

impl<T> From<T> for DefaultSelected<T> {
    fn from(value: T) -> Self {
        Self::Some(value)
    }
}

impl<T> Default for DefaultSelected<T> {
    fn default() -> Self {
        Self::None
    }
}

pub trait SelectableItem: Sized + 'static {
    type Id: Clone + PartialEq;
    fn id(&self) -> Self::Id;
    fn icon(&self) -> Html;
    fn tooltip(&self) -> Option<Html>;
}

pub trait Selectable: PartialEq {
    type Item: SelectableItem;
    fn options<'a>(&'a self, game_data: &'a GameData) -> impl Iterator<Item = &'a Self::Item>;
}

#[derive(Properties)]
struct SelectProps<S: Selectable> {
    pub selectable: S,
    #[prop_or_default]
    pub default: DefaultSelected<<S::Item as SelectableItem>::Id>,
    pub on_select: Option<Callback<<S::Item as SelectableItem>::Id>>,
}

impl<S: Selectable> PartialEq for SelectProps<S> {
    fn eq(&self, other: &Self) -> bool {
        self.selectable == other.selectable
            && self.default == other.default
            && self.on_select == other.on_select
    }
}

#[function_component]
fn Select<S: Selectable>(props: &SelectProps<S>) -> Html {
    let app_data = use_context::<AppData>().unwrap();
    let game_data = app_data.game_data();

    let mut options = props.selectable.options(&game_data).peekable();

    let default = match &props.default {
        DefaultSelected::None => None,
        DefaultSelected::First => options.peek().map(|x| x.id()),
        DefaultSelected::Some(x) => Some(x.clone()),
    };
    let selected = use_state(move || default);

    let mut option_buttons = vec![];
    let mut current_icon = html! {
        <yew_feather::MoreHorizontal />
    };
    for option in options {
        let icon = option.icon();
        let active = selected
            .as_ref()
            .map(|id| id == &option.id())
            .unwrap_or_default();
        if active {
            current_icon = icon.clone();
        }
        option_buttons.push(html! {
            <button type="button" class={classes!("btn", "btn-outline-secondary", "dropdown-item", active.then_some("active"))}>
                {icon}
            </button>
        });
    }

    html! {
        <div class="dropdown">
            <button type="button" class="btn btn-outline-primary dropdown-toggle" data-bs-toggle="dropdown" aria-expanded="false">
                // todo: show current selection
                {current_icon}
            </button>
            <div class="dropdown-menu dropdown-menu-dark">
                {option_buttons}
            </div>
        </div>
    }
}

#[derive(Copy, Clone, Debug)]
struct All<P>(PhantomData<P>);

impl<P> PartialEq for All<P> {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

impl<P> Default for All<P> {
    fn default() -> Self {
        All(PhantomData)
    }
}

impl<P> Selectable for All<P>
where
    P: SelectableItem + InheritsBase<Base = PrototypeBase>,
    Prototypes: HasPrototypes<P>,
{
    type Item = P;

    fn options<'a>(&'a self, game_data: &'a GameData) -> impl Iterator<Item = &'a Self::Item> {
        let mut options = HasPrototypes::<P>::iter(&game_data.prototypes).collect::<Vec<_>>();
        options.sort_by_cached_key(|option| &option.base().order);
        options.into_iter()
    }
}

impl SelectableItem for LabPrototype {
    type Id = Id<Self>;

    fn id(&self) -> Self::Id {
        self.base().name.clone().into()
    }

    fn icon(&self) -> Html {
        self.parent
            .parent
            .parent
            .icon_spec
            .file_name()
            .map(|file_name| html! { <FactorioIcon file_name={file_name.to_string()} size="32" /> })
            .unwrap_or_default()
    }

    fn tooltip(&self) -> Option<Html> {
        let name = &self.base().name;
        Some(html! { {name} })
    }
}

#[derive(Properties, PartialEq)]
pub struct LabSelectProps {
    #[prop_or_default]
    pub default: DefaultSelected<Id<LabPrototype>>,
    pub on_select: Option<Callback<Id<LabPrototype>>>,
}

#[function_component]
pub fn LabSelect(props: &LabSelectProps) -> Html {
    html! {
        <Select<All<LabPrototype>> selectable={All::default()} default={props.default.clone()} on_select={props.on_select.clone()} />
    }
}

impl SelectableItem for TechnologyPrototype {
    type Id = Id<Self>;

    fn id(&self) -> Self::Id {
        self.base().name.clone().into()
    }

    fn icon(&self) -> Html {
        let icon = self.icon_spec.file_name().map(
            |file_name| html! { <FactorioIcon file_name={file_name.to_string()} size="32" /> },
        );
        html! {
            <>
                { icon.unwrap_or_default() }
                //<span class="ml-3">{ &self.base().name }</span>
            </>
        }
    }

    fn tooltip(&self) -> Option<Html> {
        let name = &self.base().name;
        Some(html! { {name} })
    }
}

#[derive(Properties, PartialEq)]
pub struct TechnologySelectProps {
    #[prop_or_default]
    pub default: Option<Id<TechnologyPrototype>>,
    pub on_select: Option<Callback<Id<TechnologyPrototype>>>,
}

#[function_component]
pub fn TechnologySelect(props: &TechnologySelectProps) -> Html {
    // todo: blocklist technologies (i.e. the ones that are already in the research
    // list)
    html! {
        <Select<All<TechnologyPrototype>> selectable={All::default()} default={DefaultSelected::First} on_select={props.on_select.clone()} />
    }
}
