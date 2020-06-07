use yew::prelude::*;
use crate::util::generate_element_images;
use crate::chip_library::ChipLibrary;
use web_sys::MouseEvent;

#[derive(Properties, Clone)]
pub struct FolderChipProps {
    pub index: usize,
    pub set_msg_callback: Callback<String>,
}

pub enum FldrChipMsg {
    ReturnToPack,
    ChangeUsed,
    DoNothing,
}

pub struct FolderChip {
    props: FolderChipProps,
    link: ComponentLink<Self>,
}

impl Component for FolderChip {
    type Message = FldrChipMsg;
    
    type Properties = FolderChipProps;
    
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            FldrChipMsg::ReturnToPack => {
                //todo
                true
            },
            FldrChipMsg::ChangeUsed => {
                let mut folder = ChipLibrary::get_instance().folder.write().unwrap();
                folder[self.props.index].used = !folder[self.props.index].used;
                true
            },
            FldrChipMsg::DoNothing => {
                false
            },
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let folder = ChipLibrary::get_instance().folder.read().unwrap();
        let chip = folder.get(self.props.index).unwrap();
        let chip_css = if chip.used {
            "UsedChip"
        } else {
            chip.chip.kind.to_css_class()
        };

        html! {
            <div class=("row justify-content-center noselect chipHover", chip_css) ondoubleclick={self.link.callback(|_| FldrChipMsg::ReturnToPack)} id={format!("F1_{}", self.props.index)}>
                <div class="col-1 nopadding debug">
                    {self.props.index + 1}
                </div>
                <div class="col-3 nopadding debug" style="white-space: nowrap">
                    {&chip.chip.name}
                </div>
                <div class="col-2 nopadding debug">
                    {chip.chip.skill()}
                </div>
                <div class="col-1 nopadding debug">
                    {&chip.chip.damage}
                </div>
                <div class="col-1 nopadding debug centercontent">
                    {&chip.chip.range}
                </div>
                <div class="col-1 nopadding debug centercontent" style="white-space: nowrap">
                    {&chip.chip.hits}
                </div>
                <div class="col-1 nopadding debug centercontent">
                    {generate_element_images(&chip.chip.element)}
                </div>
                <div class="col-1 nopadding centercontent" ondoubleclick={self.link.callback(|e:MouseEvent| {e.stop_propagation(); FldrChipMsg::DoNothing})}>
                    <input name="chipUsed" type="checkbox" checked={chip.used} onchange={self.link.callback(|_| FldrChipMsg::ChangeUsed)}/>
                </div>
            </div>
        }
        
    }
    
}

