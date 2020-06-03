use yew::prelude::*;
use yew::html::ChangeData;

use crate::components::library_chip::LibraryChip;

use crate::chip_library::get_instance;
use crate::chip_library::battle_chip::BattleChip;

#[derive(Properties, Clone)]
pub struct LibraryProps {
    pub active: bool,
    pub set_msg_callback: Callback<String>,
}

#[derive(Eq, PartialEq)]
pub enum LibrarySortOptions {
    Name,
    Element,
    MaxDamage,
    AverageDamage,
    Skill,
    Range,
}

impl std::fmt::Display for LibrarySortOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            LibrarySortOptions::Name => write!(f,"Name"),
            LibrarySortOptions::Element => write!(f, "Element"),
            LibrarySortOptions::MaxDamage => write!(f, "MaxDamage"),
            LibrarySortOptions::AverageDamage => write!(f, "AverageDamage"),
            LibrarySortOptions::Skill => write!(f, "Skill"),
            LibrarySortOptions::Range => write!(f, "Range"),
        }
    }   
}

impl From<&str> for LibrarySortOptions {
    fn from(val: &str) -> Self {
        match val {
            "Name" => LibrarySortOptions::Name,
            "Element" => LibrarySortOptions::Element,
            "MaxDamage" => LibrarySortOptions::MaxDamage,
            "AverageDamage" => LibrarySortOptions::AverageDamage,
            "Skill" => LibrarySortOptions::Skill,
            "Range" => LibrarySortOptions::Range,
            _ => unreachable!(),
        }
    }
}

pub enum LibraryMessage {
    ChangeSort(LibrarySortOptions),
    DoNothing,
}

pub struct LibraryComponent{
    props: LibraryProps,
    link: ComponentLink<Self>,
    sort_by: LibrarySortOptions,
}

impl Component for LibraryComponent {
    type Message = LibraryMessage;
    type Properties = LibraryProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            sort_by: LibrarySortOptions::Name,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            LibraryMessage::ChangeSort(opt) => {
                self.sort_by = opt;
                true
            }
            LibraryMessage::DoNothing => false
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.active == props.active {
            self.props = props;
            false
        } else {
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {

        let (library_containter_class, outer_container_class) = if self.props.active {("container-fluid Folder activeFolder", "container-fluid")} else {("container-fluid Folder", "inactiveTab")};
        let select_changed = self.link.callback(|e: ChangeData| {
            if let ChangeData::Select(val) = e {
                LibraryMessage::ChangeSort(LibrarySortOptions::from(val.value().as_ref()))
            } else {
                LibraryMessage::DoNothing
            }
        });
        html! {
            <div class={outer_container_class}>
                <div class="row nopadding">
                    <div class="col-10 nopadding">
                        <div class={library_containter_class}>
                                {self.build_top_row()}
                                {self.build_library_chips()}
                        </div>
                    </div>
                    <div class="col-2 nopadding">
                        <span unselectable="on" class="Chip">{"Sort By"}</span>
                        <select value={&self.sort_by} style="width: 100%" class="custom-select" onchange={select_changed}>
                            <option value="Name">{"Name"}</option>
                            <option value="Element">{"Element"}</option>
                            <option value="MaxDamage">{"MaxDamage"}</option>
                            <option value="AverageDamage">{"AverageDamage"}</option>
                            <option value="Skill">{"Skill"}</option>
                            <option value="Range">{"Range"}</option>
                        </select>
                    </div>
                </div>
            </div>
        }
    }

}

impl LibraryComponent {
    fn build_top_row(&self) -> Html {
        html! {
            <div class="row sticky-top justify-content-center debug" style="background-color: gray">
                <div class="col-3 Chip nopadding debug" style="white-space: nowrap">
                    {"NAME"}
                </div>
                <div class="col-2 Chip nopadding debug">
                    {"SKILL"}
                </div>
                <div class="col-1 Chip nopadding debug">
                    {"DMG"}
                </div>
                <div class="col-1 Chip nopadding debug">
                    {"RANGE"}
                </div>
                <div class="col-1 Chip nopadding debug">
                    {"HITS"}
                </div>
                <div class="col-1 Chip nopadding debug"/>
            </div>
        }
    }

    fn build_library_chips(&self) -> Html {
        let chip_lib = self.fetch_chips();

        html!{
            <>
            {chip_lib.iter().map(|chip| {
                html!{
                    <>
                    <LibraryChip name={&chip.name} set_msg_callback={self.props.set_msg_callback.clone()}/>
                    </>
                }
            }).collect::<Html>()}
            </>
        }
    }

    fn fetch_chips(&self) -> Vec<&BattleChip> {
        let mut chip_lib = get_instance().get().unwrap().library.values().collect::<Vec<&BattleChip>>();
        match self.sort_by {
            LibrarySortOptions::Name => {
                chip_lib.sort_unstable_by(|a, b| {
                    a.kind.cmp(&b.kind).then_with(||a.name.cmp(&b.name))
                });
            }
            LibrarySortOptions::Element => {
                chip_lib.sort_unstable_by(|a, b| {
                    a.element.cmp(&b.element).then_with(||a.name.cmp(&b.name))
                });
            }
            LibrarySortOptions::MaxDamage => {
                chip_lib.sort_unstable_by(|a, b| {
                    a.max_dmg().partial_cmp(&b.max_dmg()).unwrap().reverse().then_with(||a.name.cmp(&b.name))
                });
            }
            LibrarySortOptions::AverageDamage => {
                chip_lib.sort_unstable_by(|a, b| {
                    a.avg_dmg().partial_cmp(&b.avg_dmg()).unwrap().reverse().then_with(||a.name.cmp(&b.name))
                });
            }
            LibrarySortOptions::Skill => {
                chip_lib.sort_unstable_by(|a, b| {
                    a.skill().cmp(&b.skill()).then_with(||a.name.cmp(&b.name))
                });
            }
            LibrarySortOptions::Range => {
                chip_lib.sort_unstable_by(|a, b| {
                    a.range.cmp(&b.range).then_with(||a.name.cmp(&b.name))
                });
            }
        }
        chip_lib
    }

}