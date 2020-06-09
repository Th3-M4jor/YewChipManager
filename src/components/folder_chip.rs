use yew::prelude::*;
use std::sync::Arc;
use crate::chip_library::battle_chip::BattleChip;
use crate::util::generate_element_images;

#[derive(Properties, Clone)]
pub struct FolderChipProps {
    pub used: bool,
    pub chip: Arc<BattleChip>,
    pub idx: usize,
    pub swap_used: Callback<MouseEvent>,
    pub return_to_pack_callback: Callback<MouseEvent>, 
}

impl PartialEq for FolderChipProps {
    fn eq(&self, other: &Self) -> bool {
        self.used == other.used 
        && self.idx == other.idx 
        && Arc::ptr_eq(&self.chip, &other.chip)
        && self.swap_used == other.swap_used
        && self.return_to_pack_callback == other.return_to_pack_callback
    }
}

pub struct FolderChipComponent {
    props: FolderChipProps,
    link: ComponentLink<Self>,
}

impl Component for FolderChipComponent {
    type Properties = FolderChipProps;
    type Message = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self{
            props,
            link,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let should_update = self.props != props;
        self.props = props;
        should_update
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

        web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!("render called on {} index {}", self.props.chip.name, self.props.idx)));

        html! {
            <div class=("row justify-content-center noselect chipHover", chip_css) ondoubleclick={self.props.return_to_pack_callback.clone()} id={format!("F1_{}", self.props.idx)}>
                    <div class="col-1 nopadding debug">
                        {self.props.idx + 1}
                    </div>
                    <div class="col-3 nopadding debug" style="white-space: nowrap">
                        {&self.props.chip.name}
                    </div>
                    <div class="col-2 nopadding debug">
                        {self.props.chip.skill()}
                    </div>
                    <div class="col-1 nopadding debug">
                        {&self.props.chip.damage}
                    </div>
                    <div class="col-1 nopadding debug centercontent">
                        {&self.props.chip.range}
                    </div>
                    <div class="col-1 nopadding debug centercontent" style="white-space: nowrap">
                        {&self.props.chip.hits}
                    </div>
                    <div class="col-1 nopadding debug centercontent">
                        {generate_element_images(&self.props.chip.element)}
                    </div>
                    <div class="col-1 nopadding centercontent" ondoubleclick={self.link.callback(|e:MouseEvent| e.stop_propagation())}>
                        <input name="chipUsed" type="checkbox" class="centerInputBox" checked={self.props.used} onclick={self.props.swap_used.clone()} id={format!("F1_{}", self.props.idx)}/>
                    </div>
            </div>
        }

    }
}

