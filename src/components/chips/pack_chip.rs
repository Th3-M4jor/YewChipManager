use yew::prelude::*;
use std::rc::Rc;
use crate::chip_library::BattleChip;
use crate::util::generate_element_images;

#[derive(Properties, Clone)]
pub(crate) struct PackChipProps {
    pub used: u32,
    pub owned: u32,
    pub add_to_folder: Callback<MouseEvent>,
    pub on_mouse_enter: Callback<MouseEvent>,
    pub chip: Rc<BattleChip>,
}

impl PartialEq for PackChipProps {
    fn eq(&self, other: &Self) -> bool {
        self.used == other.used
        && self.owned == other.owned
        && self.add_to_folder == other.add_to_folder
        && Rc::ptr_eq(&self.chip, &other.chip)
    }
}

pub(crate) struct PackChipComponent {
    props: PackChipProps,
    id_str: String,
}

impl Component for PackChipComponent {
    type Properties = PackChipProps;
    type Message = ();
    
    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        let id_str = String::from("P_") + &props.chip.name;
        Self {
            props, id_str
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            self.id_str = String::from("P_") + &self.props.chip.name;
            return true;
        }
        false
    }

    fn view(&self) -> Html {

        let chip_css = if self.props.owned <= self.props.used {
            "UsedChip"
        } else {
            self.props.chip.class.to_css_class()
        };

        let outer_class = classes!("chip-row", "noselect", "chipHover", chip_css);

        html!{
            <div class=outer_class 
                ondblclick=self.props.add_to_folder.clone() 
                id=self.id_str.clone() 
                onmouseover=self.props.on_mouse_enter.clone()
                >
                <div class="chip-col-3 nopadding" style="white-space: nowrap">
                    {&self.props.chip.name}
                </div>
                <div class="chip-col-1-5 nopadding">
                    {self.props.chip.skill().as_str()}
                </div>
                <div class="chip-col-1-5 nopadding">
                    {self.props.chip.damage.as_str()}
                </div>
                <div class="chip-col-2 nopadding centercontent">
                    {generate_element_images(&self.props.chip.element)}
                </div>
                <div class="chip-col-1 nopadding">
                    {self.props.owned}
                </div>
                <div class="chip-col-1 nopadding">
                    {self.props.used}
                </div>
            </div>
        }

    }
}