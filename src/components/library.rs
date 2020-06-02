use yew::prelude::*;
use crate::components::library_chip::LibraryChip as BattleChip;


#[derive(Properties, Clone)]
pub struct LibraryProps {
    pub active: bool,
    pub set_msg_callback: Callback<String>,
}

pub struct LibraryComponent(LibraryProps);

impl Component for LibraryComponent {
    type Message = ();
    type Properties = LibraryProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self(props)
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.0.active == props.active {
            self.0 = props;
            false
        } else {
            self.0 = props;
            true
        }
    }

    fn view(&self) -> Html {

        let library_containter_class = if self.0.active {"container-fluid Folder activeFolder"} else {"container-fluid Folder"};

        html! {
            <div class="container-fluid">
                <div class="row">
                    <div class="col-10">
                        <div class={library_containter_class}>
                                {self.build_top_row()}
                                <BattleChip name="AirHoc" set_msg_callback={self.0.set_msg_callback.clone()}/>
                        </div>
                    </div>
                    <div class="col-2"/>
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
}