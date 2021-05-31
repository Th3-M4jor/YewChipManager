use yew::prelude::*;

use crate::components::ChipSortOptions;

#[derive(Properties, Clone, PartialEq)]
pub(crate) struct ChipSortBoxProps {
    pub sort_by: ChipSortOptions,
    pub include_owned: bool,
    pub sort_changed: Callback<ChangeData>,
}

pub(crate) struct ChipSortBox {
    pub props: ChipSortBoxProps,
}

impl Component for ChipSortBox {
    type Properties = ChipSortBoxProps;
    type Message = ();

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            props,
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            return true;
        }
        false
    }

    fn view(&self) -> Html {
        html!{
            <>
            <span unselectable="on" class="Chip noselect">{"Sort By"}</span>
            <select value={self.props.sort_by.as_str()} class="chip-sort-select" onchange=self.props.sort_changed.clone()>
                <option value="Name">{"Name"}</option>
                <option value="Element">{"Element"}</option>
                <option value="MaxDamage">{"MaxDamage"}</option>
                <option value="AverageDamage">{"AverageDamage"}</option>
                <option value="Skill">{"Skill"}</option>
                <option value="Range">{"Range"}</option>
                {
                    if self.props.include_owned {
                        html!{
                            <option value="Owned">{"Owned"}</option>
                        }
                    } else {
                        html!{}
                    }
                }
            </select>
            </>
        }
    }
}