use yew::prelude::*;


use crate::util::timeout::{set_timeout, TimeoutHandle};
use crate::components::{library::LibraryComponent as Library, pack::PackComponent as Pack, folder::FolderComponent as Folder};
use crate::agents::global_msg::{GlobalMsgBus, Request as GlobalReq};


#[derive(PartialEq, Eq, Clone)]
pub enum Tabs {
    Library,
    Pack,
    Folder,
    GroupFolder(String),
}

impl std::fmt::Display for Tabs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tabs::Library => write!(f, "Library"),
            Tabs::Folder => write!(f, "Folder"),
            Tabs::Pack => write!(f, "Pack"),
            Tabs::GroupFolder(name) => write!(f, "{}'s Folder", name),
        }
    }
}

impl Tabs {
    pub fn shorten_string(&self) -> String {
        match self {
            Tabs::Library => { "Lib".to_string()}
            Tabs::Pack => {"Pck".to_string()}
            Tabs::Folder => {"Fldr".to_string()}
            Tabs::GroupFolder(grp_fldr) => {
                format!("{}...", &grp_fldr[..=4])
            }
        }
    }
}

impl PartialEq<str> for Tabs {
    fn eq(&self, other: &str) -> bool {
        match self {
            Tabs::Library => {"Library" == other}
            Tabs::Pack => {"Pack" == other}
            Tabs::Folder => {"Folder" == other}
            Tabs::GroupFolder(grp_fldr) => {
                grp_fldr == other
            }
        }
    }
}

#[derive(Clone)]
pub enum TopLevelMsg {
    ChangeTab(Tabs),
    SetMsg(String),
    JoinGroup,
    EraseData,
    ImportData,
    CancelModal,
    ModalOk,
}

pub enum ModalStatus {
    JoinGroup,
    EraseData,
    ImportData,
    Closed,
}

/// Root component
pub struct App
{
    active_tab: Tabs,
    link: ComponentLink<Self>,
    message_txt: String,
    message_clear_timeout_handle: Option<TimeoutHandle>,
    _producer: Box<dyn Bridge<GlobalMsgBus>>,
    modal_status: ModalStatus,
}

impl App {

    /// change the active tab, returns true if the new tab is different
    fn change_tab(&mut self, tab: Tabs) -> bool {

        if self.active_tab == tab {
            return false;
        }
        self.active_tab = tab;
        true
    }

    /// change the current message, returns true for consistency with change_tab
    fn set_message(&mut self, message: String) -> bool {

        if message == "" {
            let handle = self.message_clear_timeout_handle.take();
            drop(handle);
        } else {
            self.set_message_clear_timeout();
        }

        self.message_txt = message;
        true
    }

    fn set_message_clear_timeout(&mut self) {

        //ensure that previous timeout is cancelled
        let old_timeout = self.message_clear_timeout_handle.take();
        drop(old_timeout);

        let callback = self.link.callback_once(|_: ()| TopLevelMsg::SetMsg("".to_owned()));
        
        self.message_clear_timeout_handle = Some(set_timeout(15000, move || callback.emit(())).unwrap());
        
    }

    fn gen_nav_tabs(&self) -> Html {

        return match self.active_tab {

            Tabs::Library => {
                html! {
                    <div class="btn-group" role="tabs" style="padding-left: 125px; transform: translate(0px,6px)">
                        <button class="btn inactiveNavTab" onclick=self.link.callback(|_| TopLevelMsg::ChangeTab(Tabs::Folder))>{"Folder"}</button>
                        <button class="btn inactiveNavTab" onclick=self.link.callback(|_| TopLevelMsg::ChangeTab(Tabs::Pack))>{"Pack"}</button>
                        <button class="btn activeNavTab">{"Library"}</button>
                    </div>
                }
            }
            Tabs::Pack => {
                html! {
                    <div class="btn-group debug" role="tabs" style="padding-left: 20px; transform: translate(0px,6px)">
                        <button class="btn inactiveNavTab" onclick=self.link.callback(|_| TopLevelMsg::ChangeTab(Tabs::Folder))>{"Folder"}</button>
                        <button class="btn activeNavTab">{"Pack"}</button>
                        <button class="btn inactiveNavTab" onclick=self.link.callback(|_| TopLevelMsg::ChangeTab(Tabs::Library))>{"Library"}</button>
                    </div>
                }
            }
            Tabs::Folder => {
                html! {
                    <div class="btn-group debug" role="tabs" style="padding-left: 20px; transform: translate(0px,6px)">
                        <button class="btn activeNavTab">{"Folder"}</button>
                        <button class="btn inactiveNavTab" onclick=self.link.callback(|_| TopLevelMsg::ChangeTab(Tabs::Pack))>{"Pack"}</button>
                        <button class="btn inactiveNavTab" onclick=self.link.callback(|_| TopLevelMsg::ChangeTab(Tabs::Library))>{"Library"}</button>
                    </div>
                }
            }
            _ => { 
                html! {
                    <div/>
                }
            }
        };
        
    }

    fn build_modal(&self) -> Html {
        match self.modal_status {
            ModalStatus::JoinGroup => {
                self.join_group_modal()
            }
            ModalStatus::EraseData => {
                self.erase_data_modal()
            }
            ModalStatus::ImportData => {
                self.import_data_modal()
            }
            
            //closed, display nothing
            ModalStatus::Closed => {
                html!{
                    <></>
                }
            }
        }
    }

    fn join_group_modal(&self) -> Html {
        todo!();
    }

    fn erase_data_modal(&self) -> Html {
        todo!();
    }

    fn import_data_modal(&self) -> Html {
        todo!();
    }
}

impl Component for App {
    type Message = TopLevelMsg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|e| {
            match e {
                GlobalReq::SetHeaderMsg(msg) => {
                    TopLevelMsg::SetMsg(msg)
                }
                GlobalReq::JoinGroup => {
                    TopLevelMsg::JoinGroup
                }
                GlobalReq::EraseData => {
                    TopLevelMsg::EraseData
                }
                GlobalReq::ImportData => {
                    TopLevelMsg::ImportData
                }
            }
        });
        let _producer = GlobalMsgBus::bridge(callback);
        App {
            active_tab: Tabs::Library,
            message_txt: "".to_owned(),
            message_clear_timeout_handle: None,
            link,
            _producer,
            modal_status: ModalStatus::Closed,
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        return match msg {
            TopLevelMsg::ChangeTab(tab) => self.change_tab(tab),
            TopLevelMsg::SetMsg(message) => self.set_message(message),
            TopLevelMsg::JoinGroup => {false}
            TopLevelMsg::EraseData => {false}
            TopLevelMsg::ImportData => {false}
            TopLevelMsg::CancelModal => {
                self.modal_status = ModalStatus::Closed;
                true
            }
            TopLevelMsg::ModalOk => {
                self.modal_status = ModalStatus::Closed;
                true
            }
        }
    }

    fn view(&self) -> Html {

        let set_msg_callback = self.link.callback(|msg: String| TopLevelMsg::SetMsg(msg));

        html! {
            <>
            <div class="container-fluid" style="background-color: #00637b; padding: 5px; max-width: 720px">
                <div style="background-color: #ffbd18; font-family: Lucida Console; margin: 5px; color: #FFFFFF; font-weight: bold">
                    <span style="padding-left: 5px">{&self.active_tab}</span><span style="float: right; color: red">{&self.message_txt}</span>
                </div>
                <div style="background-color: #4abdb5; padding: 10px">
                    {self.gen_nav_tabs()}
                    <Library active={self.active_tab == Tabs::Library}/>
                    <Pack active={self.active_tab == Tabs::Pack} />
                    <Folder active={self.active_tab == Tabs::Folder} set_msg_callback={set_msg_callback.clone()}/>
                </div>
            </div>
            {self.build_modal()}
            </>
        }

        //let library: RwLockReadGuard<ChipLibrary> = get_instance().get().unwrap().read().unwrap();
        
        /*
        html! {
            <>
            {
                library.library.iter().map(|chip| html!{
                    <div>
                        {&chip.1.name}{" "}{chip.1.kind}
                    </div>
                }).collect::<Html>()
            }
            </>
        }
        */



        /*
        html! {
            <>
                <Nav />
                <Router<AppRoute, ()>
                    render = Router::render(|switch: AppRoute | {
                        match switch {
                            AppRoute::Home => html!{ <Home /> },
                            AppRoute::About => html!{ <About /> },
                            AppRoute::PageNotFound(Permissive(None)) => html!{"Page not found"},
                            AppRoute::PageNotFound(Permissive(Some(missed_route))) => html!{format!("Page '{}' not found", missed_route)}
                        }
                    } )
                    redirect = Router::redirect(|route: Route<()>| {
                        AppRoute::PageNotFound(Permissive(Some(route.route)))
                    })
                />
            </>
        }
        */
    }
}
