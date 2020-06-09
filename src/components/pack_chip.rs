use yew::prelude::*;
use std::sync::Arc;
use crate::chip_library::battle_chip::BattleChip;
use crate::util::generate_element_images;

#[derive(Properties, Clone)]
pub struct PackChipProps {
    pub used: u8,
    pub owned: u8,
    pub add_to_folder: Callback<MouseEvent>,
    pub chip: Arc<BattleChip>,
}

impl PartialEq for PackChipProps {
    fn eq(&self, other: &Self) -> bool {
        self.used == other.used
        && self.owned == other.owned
        && self.add_to_folder == other.add_to_folder
        && Arc::ptr_eq(&self.chip, &other.chip)
    }
}

pub struct PackChipComponent {
    props: PackChipProps,
}

impl Component for PackChipComponent {
    type Properties = PackChipProps;
    type Message = ();
    
    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            props
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let should_update = self.props == props;
        self.props = props;
        should_update
    }

    fn view(&self) -> Html {
        let chip_css = if self.props.owned <= self.props.used {
            "UsedChip"
        } else {
            self.props.chip.kind.to_css_class()
        };

        html!{
            <div class=("row justify-content-center noselect chipHover", chip_css) ondoubleclick={self.props.add_to_folder.clone()} id={format!("P_{}", self.props.chip.name)}>
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
                <div class="col-1 nopadding">
                    {self.props.owned}
                </div>
                <div class="col-1 nopadding">
                    {self.props.used}
                </div>
            </div>
        }

    }
}