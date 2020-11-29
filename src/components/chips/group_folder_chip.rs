use yew::prelude::*;
use std::rc::Rc;
use crate::chip_library::BattleChip;
use crate::util::generate_element_images;

#[derive(Properties, Clone)]
pub(crate) struct GroupFolderChipProps {
    pub used: bool,
    pub chip: Rc<BattleChip>,
    pub idx: usize,
    pub on_mouse_enter: Callback<MouseEvent>,
}

impl PartialEq for GroupFolderChipProps {
    fn eq(&self, other: &Self) -> bool {
        self.used == other.used 
        && self.idx == other.idx 
        && Rc::ptr_eq(&self.chip, &other.chip)
        && self.on_mouse_enter == other.on_mouse_enter
    }
}

pub(crate) struct GroupFolderChipComponent {
    props: GroupFolderChipProps,
}

impl Component for GroupFolderChipComponent {
    type Properties = GroupFolderChipProps;
    type Message = ();

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self{
            props,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        
        if self.props != props {
            self.props = props;
            return true;
        }
        false
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let chip_css = if self.props.used {
            "UsedChip"
        } else {
            self.props.chip.kind.to_css_class()
        };

        let outer_class = classes!("chip-row", "noselect", "chipHover", chip_css);
        
        html!{
            <div
                class=outer_class
                id={&self.props.chip.name}
                onmouseover=self.props.on_mouse_enter.clone()
            >
                <div class="chip-col-1 nopadding">
                    {self.props.idx + 1}
                </div>
                <div class="chip-col-3 nopadding">
                    {&self.props.chip.name}
                </div>
                <div class="chip-col-1-5 nopadding">
                    {self.props.chip.skill().as_str()}
                </div>
                <div class="chip-col-1-5 nopadding">
                    {self.props.chip.damage.as_str()}
                </div>
                <div class="chip-col-2 nopadding">
                    {generate_element_images(&self.props.chip.element)}
                </div>
                <div class="chip-col-1 nopadding centercontent">
                    <input
                        name="chipUsed"
                        type="checkbox"
                        class="centerInputBox"
                        checked=self.props.used
                        disabled=true
                    />
                </div>
            </div>
        }

    }
}

