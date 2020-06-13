use yew::prelude::*;
use std::rc::Rc;
use crate::chip_library::BattleChip;
use crate::util::generate_element_images;

#[derive(Properties, Clone)]
pub(crate) struct FolderChipProps {
    pub used: bool,
    pub chip: Rc<BattleChip>,
    pub idx: usize,
    pub swap_used: Callback<MouseEvent>,
    pub return_to_pack_callback: Callback<MouseEvent>,
    pub on_mouse_enter: Callback<MouseEvent>,
}

impl PartialEq for FolderChipProps {
    fn eq(&self, other: &Self) -> bool {
        self.used == other.used 
        && self.idx == other.idx 
        && Rc::ptr_eq(&self.chip, &other.chip)
        && self.swap_used == other.swap_used
        && self.return_to_pack_callback == other.return_to_pack_callback
        && self.on_mouse_enter == other.on_mouse_enter
    }
}

pub(crate) struct FolderChipComponent {
    props: FolderChipProps,
    link: ComponentLink<Self>,
    id_1: String,
    id_2: String,
}

impl Component for FolderChipComponent {
    type Properties = FolderChipProps;
    type Message = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut id_1 = String::from("F1_");
            let mut id_2 = String::from("F2_");
            let idx_str = props.idx.to_string();
            id_1.push_str(&idx_str);
            id_2.push_str(&idx_str);

        Self{
            props,
            link,
            id_1,
            id_2,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props.idx != self.props.idx {
            let mut id_1 = String::from("F1_");
            let mut id_2 = String::from("F2_");
            let idx_str = self.props.idx.to_string();
            id_1.push_str(&idx_str);
            id_2.push_str(&idx_str);
            self.id_1 = id_1;
            self.id_2 = id_2;
        }

        
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
        
        html!{
            <div
                class=("row justify-content-center noselect chipHover", chip_css)
                ondoubleclick={self.props.return_to_pack_callback.clone()}
                id={&self.id_1}
                onmouseover={self.props.on_mouse_enter.clone()}
            >
                <div class="col-1 nopadding">
                    {self.props.idx + 1}
                </div>
                <div class="col-3 nopadding">
                    {&self.props.chip.name}
                </div>
                <div class="col-3 nopadding">
                    {self.props.chip.skill().as_str()}
                </div>
                <div class="col-2 nopadding">
                    {generate_element_images(&self.props.chip.element)}
                </div>
                <div class="col-1 nopadding centercontent" ondoubleclick={self.link.callback(|e:MouseEvent| e.stop_propagation())}>
                    <input
                        name="chipUsed"
                        type="checkbox"
                        class="centerInputBox"
                        checked={self.props.used}
                        onclick={self.props.swap_used.clone()}
                        id={&self.id_2}
                    />
                </div>
            </div>
        }

    }
}

